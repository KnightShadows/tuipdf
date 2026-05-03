// tuipdf
// ------
// A beautifully crafted, terminal-native PDF tool built in Rust.
// It aims to make compressing PDF files as fast, efficient and flexible
// as possible directly from your terminal.
//
// Authors: KnightShadows Team and individual contributors (see CONTRIBUTORS file)
//          Aditya Anand <aditya19study@gmail.com> (c) 2025
// Website: https://github.com/KnightShadows/tuipdf
// License: MPL-2.0 (see LICENSE file)

use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use std::fmt;

use crate::pipeline;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompressionLevel {
    Low,
    Medium,
    High,
}

impl CompressionLevel {
    pub fn next(&self) -> Self {
        match self {
            Self::Low => Self::Medium,
            Self::Medium => Self::High,
            Self::High => Self::Low,
        }
    }
}

impl fmt::Display for CompressionLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Low => write!(f, "Low"),
            Self::Medium => write!(f, "Medium"),
            Self::High => write!(f, "High"),
        }
    }
}

#[derive(Debug)]
pub enum CompressionMsg {
    Progress(f32),
    Done { original: u64, compressed: u64 },
    Error(String),
}

fn level_to_config(level: CompressionLevel, output_dir: PathBuf) -> pipeline::CompressionConfig {
    match level {
        CompressionLevel::Low => pipeline::CompressionConfig {
            quality: 85,
            dpi_threshold: 200,
            output_dir,
            remove_metadata: false,
        },
        CompressionLevel::Medium => pipeline::CompressionConfig {
            quality: 75,
            dpi_threshold: 150,
            output_dir,
            remove_metadata: false,
        },
        CompressionLevel::High => pipeline::CompressionConfig {
            quality: 50,
            dpi_threshold: 100,
            output_dir,
            remove_metadata: true,
        },
    }
}

pub fn start(
    input: String,
    output: String,
    level: CompressionLevel,
) -> (JoinHandle<()>, mpsc::Receiver<CompressionMsg>) {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let result = (|| -> Result<(u64, u64), String> {
            let input_path = Path::new(&input);

            let _ = tx.send(CompressionMsg::Progress(0.1));

            let output_path = Path::new(&output);
            let output_dir = output_path
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .to_path_buf();

            let config = level_to_config(level, output_dir);

            let _ = tx.send(CompressionMsg::Progress(0.2));

            let stats = pipeline::compress_pdf(input_path, &config).map_err(|e| e.to_string())?;

            let _ = tx.send(CompressionMsg::Progress(0.9));

            let _expected_name = output_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("compressed.pdf");
            let pipeline_output = config.output_dir.join(format!(
                "{}_compressed.pdf",
                input_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("output")
            ));

            if pipeline_output != output_path && pipeline_output.exists() {
                let _ = std::fs::rename(&pipeline_output, output_path);
            }

            Ok((stats.original_bytes, stats.compressed_bytes))
        })();

        match result {
            Ok((original, compressed)) => {
                let _ = tx.send(CompressionMsg::Progress(1.0));
                thread::sleep(Duration::from_millis(200));
                let _ = tx.send(CompressionMsg::Done {
                    original,
                    compressed,
                });
            }
            Err(e) => {
                let _ = tx.send(CompressionMsg::Error(e));
            }
        }
    });

    (handle, rx)
}
