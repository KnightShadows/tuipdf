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

use std::io::Cursor;

use flate2::Compression;
use flate2::write::ZlibEncoder;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, ImageReader};
use std::io::Write;

use crate::pipeline::parser::{ColorInfo, ContentType};

pub struct CompressedData {
    pub data: Vec<u8>,
    pub is_jpeg: bool,
    pub is_flate: bool,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub color_space: Option<&'static str>,
    pub bits_per_component: Option<u8>,
}

pub fn compress_stream(
    data: &[u8],
    content_type: &ContentType,
    original_encoded_size: usize,
    quality: u8,
    dpi_threshold: u32,
) -> Option<Result<CompressedData, String>> {
    match content_type {
        ContentType::JpegImage {
            width,
            height,
            dpi,
            color,
        } => Some(compress_jpeg_image(
            data,
            *width,
            *height,
            *dpi,
            color,
            original_encoded_size,
            quality,
            dpi_threshold,
        )),
        ContentType::PngImage {
            width,
            height,
            dpi,
            bits_per_component,
            color,
        } => Some(compress_png_image(
            data,
            *width,
            *height,
            *dpi,
            *bits_per_component,
            color,
            original_encoded_size,
            quality,
            dpi_threshold,
        )),
        ContentType::RawBitmap {
            width,
            height,
            bits_per_component,
            color,
        } => Some(compress_raw_bitmap(
            data,
            *width,
            *height,
            *bits_per_component,
            color,
            original_encoded_size,
            quality,
            dpi_threshold,
        )),

        // Content streams (page operators) must NEVER be recompressed.
        // They are tiny deflated text and recompressing them with the wrong
        // format (raw DEFLATE vs zlib) corrupts the PDF, producing blank pages.
        ContentType::TextStream => None,

        ContentType::EmbeddedFont => None,
        ContentType::FormXObject => None,
        ContentType::Unknown => None,
    }
}

#[allow(clippy::too_many_arguments)]
fn compress_jpeg_image(
    data: &[u8],
    _width: u32,
    _height: u32,
    dpi: Option<u32>,
    color: &ColorInfo,
    original_encoded_size: usize,
    quality: u8,
    dpi_threshold: u32,
) -> Result<CompressedData, String> {
    // For CMYK JPEGs, the `image` crate may decode incorrectly (treating
    // CMYK channels as RGBA). Skip compression to avoid discoloration.
    if color.is_cmyk {
        return Err("CMYK JPEG — skipping to preserve colors".to_string());
    }

    let img = load_image_from_bytes(data)?;
    let img = maybe_downsample(img, dpi, dpi_threshold);
    let (new_w, new_h) = img.dimensions();

    // Detect if the source is grayscale — encode as grayscale JPEG to
    // preserve color space and avoid bloating single-channel images to 3x.
    let is_gray = color.is_grayscale
        || matches!(
            img,
            DynamicImage::ImageLuma8(_) | DynamicImage::ImageLuma16(_)
        );

    let jpeg_bytes = if is_gray {
        encode_jpeg_gray(&img, quality)?
    } else {
        encode_jpeg(&img, quality)?
    };

    if jpeg_bytes.len() >= original_encoded_size {
        return Err("Compressed size larger than original".to_string());
    }

    let (cs, bpc) = if is_gray {
        ("DeviceGray", 8)
    } else {
        ("DeviceRGB", 8)
    };

    Ok(CompressedData {
        data: jpeg_bytes,
        is_jpeg: true,
        is_flate: false,
        width: Some(new_w),
        height: Some(new_h),
        color_space: Some(cs),
        bits_per_component: Some(bpc),
    })
}

#[allow(clippy::too_many_arguments)]
fn compress_png_image(
    data: &[u8],
    width: u32,
    height: u32,
    dpi: Option<u32>,
    bits_per_component: u8,
    color: &ColorInfo,
    original_encoded_size: usize,
    quality: u8,
    dpi_threshold: u32,
) -> Result<CompressedData, String> {
    let img = load_image_from_raw_pixels(data, width, height, bits_per_component, color)?;
    let img = maybe_downsample(img, dpi, dpi_threshold);
    let (new_w, new_h) = img.dimensions();

    let is_gray = color.is_grayscale
        || matches!(
            img,
            DynamicImage::ImageLuma8(_) | DynamicImage::ImageLuma16(_)
        );

    let jpeg_bytes = if is_gray {
        encode_jpeg_gray(&img, quality)?
    } else {
        encode_jpeg(&img, quality)?
    };

    if jpeg_bytes.len() >= original_encoded_size {
        return Err("Compressed size larger than original".to_string());
    }

    let (cs, bpc) = if is_gray {
        ("DeviceGray", 8)
    } else {
        ("DeviceRGB", 8)
    };

    Ok(CompressedData {
        data: jpeg_bytes,
        is_jpeg: true,
        is_flate: false,
        width: Some(new_w),
        height: Some(new_h),
        color_space: Some(cs),
        bits_per_component: Some(bpc),
    })
}

#[allow(clippy::too_many_arguments)]
fn compress_raw_bitmap(
    data: &[u8],
    width: u32,
    height: u32,
    bits_per_component: u8,
    color: &ColorInfo,
    original_encoded_size: usize,
    quality: u8,
    dpi_threshold: u32,
) -> Result<CompressedData, String> {
    compress_png_image(
        data,
        width,
        height,
        None,
        bits_per_component,
        color,
        original_encoded_size,
        quality,
        dpi_threshold,
    )
}

fn load_image_from_bytes(data: &[u8]) -> Result<DynamicImage, String> {
    let reader = ImageReader::new(Cursor::new(data))
        .with_guessed_format()
        .map_err(|e| format!("Failed to guess image format: {e}"))?;

    reader
        .decode()
        .map_err(|e| format!("Failed to decode image: {e}"))
}

/// Construct a `DynamicImage` from raw (decompressed) pixel bytes using the
/// PDF dictionary metadata (`Width`, `Height`, `BitsPerComponent`, `ColorSpace`).
///
/// The `color` parameter tells us exactly how many bytes per pixel and what
/// the channels represent:
///   - components=1  →  grayscale  →  1 byte/pixel (at 8bpc)
///   - components=3  →  RGB       →  3 bytes/pixel
///   - components=4  →  CMYK      →  4 bytes/pixel  → converted to RGB
///
/// Sub-byte bit depths (1, 2, 4 bpc) are unpacked to 8-bit before image
/// construction.
fn load_image_from_raw_pixels(
    data: &[u8],
    width: u32,
    height: u32,
    bits_per_component: u8,
    color: &ColorInfo,
) -> Result<DynamicImage, String> {
    if width == 0 || height == 0 {
        return Err("Image has zero dimensions".to_string());
    }

    let components = color.components as usize;

    // ── Sub-byte bit depths (1, 2, 4 bpc) ──────────────────────────────
    // These are almost always grayscale in practice. Unpack to 8-bit gray.
    if bits_per_component < 8 {
        return unpack_subbyte_image(data, width, height, bits_per_component, color);
    }

    // ── 8-bit (and 16-bit) standard paths ───────────────────────────────
    let pixels = (width as usize) * (height as usize);
    let expected = pixels * components;

    // Verify data length matches what we expect from the color space.
    if data.len() < expected {
        return Err(format!(
            "Image data too short for {}x{} @ {} components: got {} bytes, need {}",
            width,
            height,
            components,
            data.len(),
            expected
        ));
    }

    match components {
        1 => {
            // Grayscale
            image::GrayImage::from_raw(width, height, data[..expected].to_vec())
                .map(DynamicImage::ImageLuma8)
                .ok_or_else(|| "Failed to construct grayscale image".to_string())
        }
        3 => {
            // RGB
            image::RgbImage::from_raw(width, height, data[..expected].to_vec())
                .map(DynamicImage::ImageRgb8)
                .ok_or_else(|| "Failed to construct RGB image".to_string())
        }
        4 => {
            // CMYK → convert to RGB
            let rgb_data = cmyk_to_rgb(data, pixels);
            image::RgbImage::from_raw(width, height, rgb_data)
                .map(DynamicImage::ImageRgb8)
                .ok_or_else(|| "Failed to construct RGB image from CMYK data".to_string())
        }
        _ => Err(format!(
            "Unsupported component count: {}",
            components
        )),
    }
}

/// Unpack sub-byte images (1, 2, or 4 bits per component) to 8-bit grayscale.
fn unpack_subbyte_image(
    data: &[u8],
    width: u32,
    height: u32,
    bits_per_component: u8,
    color: &ColorInfo,
) -> Result<DynamicImage, String> {
    let components = color.components as usize;
    let samples_per_row = (width as usize) * components;
    let bits_per_row = samples_per_row * (bits_per_component as usize);
    let row_bytes = bits_per_row.div_ceil(8);
    let expected = row_bytes * (height as usize);

    if data.len() < expected {
        return Err(format!(
            "{}-bit image data too short: got {} bytes, need {}",
            bits_per_component,
            data.len(),
            expected
        ));
    }

    // For grayscale (components=1), unpack to 8-bit grayscale.
    // For multi-component sub-byte, this is rare; unpack all samples
    // and build an RGB image.
    if components == 1 {
        let mask = (1u8 << bits_per_component) - 1;
        let max_val = mask as f32;
        let mut gray_pixels = Vec::with_capacity((width as usize) * (height as usize));

        for y in 0..height as usize {
            for x in 0..width as usize {
                let bit_offset = x * (bits_per_component as usize);
                let byte_idx = y * row_bytes + bit_offset / 8;
                let shift = 8 - (bits_per_component as usize) - (bit_offset % 8);
                let sample = (data[byte_idx] >> shift) & mask;
                gray_pixels.push((sample as f32 / max_val * 255.0).round() as u8);
            }
        }

        image::GrayImage::from_raw(width, height, gray_pixels)
            .map(DynamicImage::ImageLuma8)
            .ok_or_else(|| {
                format!(
                    "Failed to construct {}-bit grayscale image",
                    bits_per_component
                )
            })
    } else {
        // Multi-component sub-byte (very rare; e.g., 4-bit RGB).
        // Skip compression — not worth the complexity risk.
        Err(format!(
            "Sub-byte multi-component images ({}-bit, {} components) are not supported",
            bits_per_component, components
        ))
    }
}

/// Convert CMYK raw pixel data to RGB.
///
/// Uses the standard subtractive color model:
///   R = 255 × (1 - C/255) × (1 - K/255)
///   G = 255 × (1 - M/255) × (1 - K/255)
///   B = 255 × (1 - Y/255) × (1 - K/255)
///
/// This is the same formula used by Adobe products for DeviceCMYK → RGB
/// conversion without an ICC profile.
fn cmyk_to_rgb(data: &[u8], pixel_count: usize) -> Vec<u8> {
    let mut rgb = Vec::with_capacity(pixel_count * 3);
    for i in 0..pixel_count {
        let base = i * 4;
        let c = data[base] as f32 / 255.0;
        let m = data[base + 1] as f32 / 255.0;
        let y = data[base + 2] as f32 / 255.0;
        let k = data[base + 3] as f32 / 255.0;

        let r = 255.0 * (1.0 - c) * (1.0 - k);
        let g = 255.0 * (1.0 - m) * (1.0 - k);
        let b = 255.0 * (1.0 - y) * (1.0 - k);

        rgb.push(r.round().clamp(0.0, 255.0) as u8);
        rgb.push(g.round().clamp(0.0, 255.0) as u8);
        rgb.push(b.round().clamp(0.0, 255.0) as u8);
    }
    rgb
}

fn maybe_downsample(img: DynamicImage, dpi: Option<u32>, dpi_threshold: u32) -> DynamicImage {
    let current_dpi = match dpi {
        Some(d) if d > dpi_threshold => d,
        _ => return img,
    };

    let scale = dpi_threshold as f64 / current_dpi as f64;
    let (w, h) = img.dimensions();
    let new_w = ((w as f64) * scale).round() as u32;
    let new_h = ((h as f64) * scale).round() as u32;

    if new_w == 0 || new_h == 0 {
        return img;
    }

    img.resize(new_w, new_h, FilterType::Lanczos3)
}

fn encode_jpeg(img: &DynamicImage, quality: u8) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, quality);
    let rgb = img.to_rgb8();
    encoder
        .encode_image(&rgb)
        .map_err(|e| format!("JPEG encoding failed: {e}"))?;
    Ok(out)
}

fn encode_jpeg_gray(img: &DynamicImage, quality: u8) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, quality);
    let gray = img.to_luma8();
    encoder
        .encode_image(&gray)
        .map_err(|e| format!("JPEG grayscale encoding failed: {e}"))?;
    Ok(out)
}

/// Compress data using zlib format (DEFLATE with zlib header/checksum).
/// This is what PDF's FlateDecode filter expects — NOT raw DEFLATE.
#[allow(dead_code)]
fn zlib_compress(data: &[u8]) -> Result<Vec<u8>, String> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
    encoder
        .write_all(data)
        .map_err(|e| format!("Zlib write failed: {e}"))?;
    encoder
        .finish()
        .map_err(|e| format!("Zlib finish failed: {e}"))
}
