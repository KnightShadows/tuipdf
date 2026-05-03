// tuipdf
// ------
// A beautifully crafted, terminal-native PDF tool built in Rust.
// It aims to make compressing PDF files as fast, efficient and flexible
// as possible directly from your terminal.
//
// Authors: KnightShadows Team and individual contributors (see CONTRIBUTORS file)
//          Aditya Anand <aditya19study@gmail.com> (c) 2026
// Website: https://github.com/KnightShadows/tuipdf
// License: MPL-2.0 (see LICENSE file)

pub mod compressor;
pub mod error;
pub mod extractor;
pub mod memory;
pub mod parser;
pub mod rebuilder;
pub mod stats;
pub mod structural;

#[cfg(test)]
mod integration_tests;

use std::fs;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::time::Instant;

use rayon::prelude::*;

pub use error::PipelineError;
pub use parser::ContentType;
pub use stats::StatsCollector;

pub struct CompressionConfig {
    pub quality: u8,
    pub dpi_threshold: u32,
    pub output_dir: PathBuf,
    pub remove_metadata: bool,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            quality: 75,
            dpi_threshold: 150,
            output_dir: PathBuf::from("."),
            remove_metadata: false,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct CompressionStats {
    pub original_bytes: u64,
    pub compressed_bytes: u64,
    pub ratio: f32,
    pub duration_ms: u128,
    pub images_compressed: u32,
    pub images_failed: u32,
    pub objects_removed: u32,
    pub processing_mode: ProcessingMode,
}

#[derive(Debug)]
pub enum ProcessingMode {
    Parallel,
    Sequential,
}

pub fn compress_pdf(
    input: &Path,
    config: &CompressionConfig,
) -> Result<CompressionStats, PipelineError> {
    let start = Instant::now();

    let original_bytes = fs::metadata(input)
        .map_err(|e| PipelineError::LoadFailed(format!("Cannot read file metadata: {e}")))?
        .len();

    let mut doc = parser::load_pdf(input)?;

    let classified = parser::classify_objects(&doc);

    let collector = StatsCollector::new();
    let removed = rebuilder::remove_unused_objects(&mut doc);
    collector.record_objects_removed(removed);

    let use_parallel = memory::check_memory_pressure()?;
    let processing_mode = if use_parallel {
        ProcessingMode::Parallel
    } else {
        ProcessingMode::Sequential
    };

    let compressible: Vec<_> = classified
        .iter()
        .filter(|(_, ct)| is_compressible(ct))
        .collect();

    let results: Vec<_> = if use_parallel {
        compressible
            .par_iter()
            .filter_map(|(obj_id, content_type)| {
                compress_single_object(&doc, *obj_id, content_type, config, &collector)
            })
            .collect()
    } else {
        compressible
            .iter()
            .filter_map(|(obj_id, content_type)| {
                compress_single_object(&doc, *obj_id, content_type, config, &collector)
            })
            .collect()
    };

    for (obj_id, compressed) in results {
        rebuilder::reinsert_compressed_stream(&mut doc, obj_id, compressed)
            .map_err(|e| PipelineError::RebuildFailed(e.to_string()))?;
    }

    if config.remove_metadata {
        rebuilder::strip_metadata(&mut doc);
    }

    let file_name = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let output_path = config
        .output_dir
        .join(format!("{file_name}_compressed.pdf"));
    let temp_path = config
        .output_dir
        .join(format!(".{file_name}_compressed.pdf.tmp"));

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| PipelineError::WriteFailed(format!("Cannot create output dir: {e}")))?;
    }

    {
        let temp_file = fs::File::create(&temp_path)
            .map_err(|e| PipelineError::WriteFailed(format!("Cannot create temp file: {e}")))?;
        let mut writer = BufWriter::new(temp_file);

        structural::save_with_structural_compression(&mut doc, &mut writer)?;
    }

    fs::rename(&temp_path, &output_path).map_err(|e| {
        let _ = fs::remove_file(&temp_path);
        PipelineError::WriteFailed(format!("Cannot rename temp file to output: {e}"))
    })?;

    let compressed_bytes = fs::metadata(&output_path)
        .map_err(|e| PipelineError::WriteFailed(format!("Cannot read output metadata: {e}")))?
        .len();

    let ratio = if original_bytes > 0 {
        compressed_bytes as f32 / original_bytes as f32
    } else {
        1.0
    };

    let duration_ms = start.elapsed().as_millis();

    Ok(CompressionStats {
        original_bytes,
        compressed_bytes,
        ratio,
        duration_ms,
        images_compressed: collector.images_compressed(),
        images_failed: collector.images_failed(),
        objects_removed: collector.objects_removed(),
        processing_mode,
    })
}

fn is_compressible(ct: &ContentType) -> bool {
    matches!(
        ct,
        ContentType::JpegImage { .. }
            | ContentType::PngImage { .. }
            | ContentType::RawBitmap { .. }
    )
}

fn compress_single_object(
    doc: &lopdf::Document,
    obj_id: lopdf::ObjectId,
    content_type: &ContentType,
    config: &CompressionConfig,
    collector: &StatsCollector,
) -> Option<(lopdf::ObjectId, compressor::CompressedData)> {
    let extracted = match extractor::extract_stream(doc, obj_id) {
        Ok(e) => e,
        Err(e) => {
            log::warn!("Extraction failed for object {:?}: {}", obj_id, e);
            if is_image_type(content_type) {
                collector.record_image_failed();
            }
            return None;
        }
    };

    let original_size = extractor::raw_stream_size(doc, obj_id).unwrap_or(extracted.data.len());

    let result = compressor::compress_stream(
        &extracted.data,
        content_type,
        original_size,
        config.quality,
        config.dpi_threshold,
    );

    match result {
        Some(Ok(compressed)) => {
            if is_image_type(content_type) && compressed.data.len() < original_size {
                collector.record_image_compressed();
            }
            Some((obj_id, compressed))
        }
        Some(Err(reason)) => {
            log::warn!("Compression failed for object {:?}: {}", obj_id, reason);
            if is_image_type(content_type) {
                collector.record_image_failed();
            }
            None
        }
        None => None,
    }
}

fn is_image_type(ct: &ContentType) -> bool {
    matches!(
        ct,
        ContentType::JpegImage { .. }
            | ContentType::PngImage { .. }
            | ContentType::RawBitmap { .. }
    )
}
