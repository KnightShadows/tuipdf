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

use std::path::Path;

use lopdf::{Document, Object, ObjectId};

use crate::pipeline::error::PipelineError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorInfo {
    pub components: u8,
    pub is_grayscale: bool,
    pub is_cmyk: bool,
}

impl ColorInfo {
    fn gray() -> Self {
        Self {
            components: 1,
            is_grayscale: true,
            is_cmyk: false,
        }
    }
    fn rgb() -> Self {
        Self {
            components: 3,
            is_grayscale: false,
            is_cmyk: false,
        }
    }
    fn cmyk() -> Self {
        Self {
            components: 4,
            is_grayscale: false,
            is_cmyk: true,
        }
    }
    fn unknown() -> Self {
        Self {
            components: 0,
            is_grayscale: false,
            is_cmyk: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ContentType {
    JpegImage {
        width: u32,
        height: u32,
        dpi: Option<u32>,
        color: ColorInfo,
    },
    PngImage {
        width: u32,
        height: u32,
        dpi: Option<u32>,
        bits_per_component: u8,
        color: ColorInfo,
    },
    RawBitmap {
        width: u32,
        height: u32,
        bits_per_component: u8,
        color: ColorInfo,
    },
    TextStream,

    EmbeddedFont,
    FormXObject,
    Unknown,
}

pub fn load_pdf(path: &Path) -> Result<Document, PipelineError> {
    let doc = Document::load(path).map_err(|e| PipelineError::LoadFailed(e.to_string()))?;

    if doc.is_encrypted() {
        return Err(PipelineError::UnsupportedFeature(
            "PDF is encrypted — decryption is not supported. Please decrypt the PDF first."
                .to_string(),
        ));
    }

    let version = &doc.version;
    if version.as_str() > "2.0" {
        log::warn!(
            "PDF version {} is above 2.0 — continuing but results may vary",
            version
        );
    }

    Ok(doc)
}

pub fn classify_objects(doc: &Document) -> Vec<(ObjectId, ContentType)> {
    let mut classified = Vec::new();

    for (&obj_id, object) in &doc.objects {
        if let Object::Stream(stream) = object {
            let content_type = classify_stream(stream, doc);
            classified.push((obj_id, content_type));
        }
    }

    classified
}

fn classify_stream(stream: &lopdf::Stream, doc: &Document) -> ContentType {
    let dict = &stream.dict;

    if let Ok(type_name) = dict.get(b"Type").and_then(|o| resolve_name(o, doc)) {
        match type_name.as_slice() {
            b"Font" | b"FontDescriptor" => return ContentType::EmbeddedFont,
            _ => {}
        }
    }

    let subtype = dict.get(b"Subtype").and_then(|o| resolve_name(o, doc)).ok();

    if let Some(st) = &subtype
        && st.as_slice() == b"Form" {
            return ContentType::FormXObject;
        }

    if let Some(st) = &subtype
        && st.as_slice() == b"Image" {
            return classify_image(dict, doc);
        }

    if let Some(st) = &subtype {
        let s = st.as_slice();
        if s == b"Type1C" || s == b"CIDFontType0C" || s == b"OpenType" {
            return ContentType::EmbeddedFont;
        }
    }

    if dict.has(b"Length1") || dict.has(b"Length2") || dict.has(b"Length3") {
        return ContentType::EmbeddedFont;
    }

    ContentType::TextStream
}

fn classify_image(dict: &lopdf::Dictionary, doc: &Document) -> ContentType {
    let width = get_u32(dict, b"Width", doc).unwrap_or(0);
    let height = get_u32(dict, b"Height", doc).unwrap_or(0);

    let bits_per_component = get_u32(dict, b"BitsPerComponent", doc).unwrap_or(8) as u8;
    let color = detect_color_info(dict, doc);

    if color.components == 0 {
        log::info!(
            "Skipping image {}x{}: unsupported or unrecognized color space",
            width,
            height
        );
        return ContentType::Unknown;
    }

    let filter = get_filter_names(dict, doc);

    let dpi: Option<u32> = None;

    for f in &filter {
        match f.as_slice() {
            b"DCTDecode" => {
                return ContentType::JpegImage {
                    width,
                    height,
                    dpi,
                    color,
                };
            }
            b"JPXDecode" => {
                return ContentType::Unknown;
            }
            b"JBIG2Decode" => {
                return ContentType::Unknown;
            }
            b"CCITTFaxDecode" => {
                return ContentType::Unknown;
            }
            _ => {}
        }
    }

    for f in &filter {
        if f.as_slice() == b"FlateDecode" {
            return ContentType::PngImage {
                width,
                height,
                dpi,
                bits_per_component,
                color,
            };
        }
    }

    if width > 0 && height > 0 {
        return ContentType::RawBitmap {
            width,
            height,
            bits_per_component,
            color,
        };
    }

    ContentType::Unknown
}


fn detect_color_info(dict: &lopdf::Dictionary, doc: &Document) -> ColorInfo {
    let cs_obj = match dict.get(b"ColorSpace") {
        Ok(obj) => obj,
        Err(_) => return ColorInfo::unknown(),
    };

    resolve_color_info(cs_obj, doc)
}

fn resolve_color_info(cs_obj: &Object, doc: &Document) -> ColorInfo {
    let resolved = resolve_object(cs_obj, doc);

    match resolved {
        Object::Name(n) => match n.as_slice() {
            b"DeviceGray" | b"CalGray" | b"G" => ColorInfo::gray(),
            b"DeviceRGB" | b"CalRGB" | b"RGB" => ColorInfo::rgb(),
            b"DeviceCMYK" | b"CMYK" => ColorInfo::cmyk(),
            _ => {
                log::warn!("Unknown simple color space: {:?}", String::from_utf8_lossy(n));
                ColorInfo::unknown()
            }
        },
        Object::Array(arr) => {
            if let Some(first) = arr.first() {
                let first_resolved = resolve_object(first, doc);
                if let Object::Name(n) = first_resolved {
                    return match n.as_slice() {
                        b"DeviceGray" | b"CalGray" | b"G" => ColorInfo::gray(),
                        b"DeviceRGB" | b"CalRGB" | b"RGB" => ColorInfo::rgb(),
                        b"DeviceCMYK" | b"CMYK" => ColorInfo::cmyk(),
                        b"Lab" => ColorInfo::rgb(),
                        b"ICCBased" => {

                            if let Some(profile_ref) = arr.get(1) {
                                let profile_obj = resolve_object(profile_ref, doc);
                                if let Object::Stream(profile_stream) = profile_obj
                                    && let Some(n_val) =
                                        get_u32(&profile_stream.dict, b"N", doc)
                                    {
                                        return match n_val {
                                            1 => ColorInfo::gray(),
                                            3 => ColorInfo::rgb(),
                                            4 => ColorInfo::cmyk(),
                                            _ => ColorInfo::unknown(),
                                        };
                                    }
                            }
                            ColorInfo::unknown()
                        }
                        b"Indexed" | b"I" => {
                            log::info!("Skipping Indexed color space image (palette-based)");
                            ColorInfo::unknown()
                        }
                        b"Separation" => {
                            log::info!("Skipping Separation color space image (spot color)");
                            ColorInfo::unknown()
                        }
                        b"DeviceN" => {
                            log::info!("Skipping DeviceN color space image");
                            ColorInfo::unknown()
                        }
                        b"Pattern" => {
                            log::info!("Skipping Pattern color space image");
                            ColorInfo::unknown()
                        }
                        _ => {
                            log::warn!(
                                "Unknown array color space: {:?}",
                                String::from_utf8_lossy(n)
                            );
                            ColorInfo::unknown()
                        }
                    };
                }
            }
            ColorInfo::unknown()
        }
        _ => ColorInfo::unknown(),
    }
}

fn get_filter_names(dict: &lopdf::Dictionary, doc: &Document) -> Vec<Vec<u8>> {
    let mut names = Vec::new();
    let filter_obj = match dict.get(b"Filter") {
        Ok(obj) => obj,
        Err(_) => return names,
    };

    let resolved = resolve_object(filter_obj, doc);

    match resolved {
        Object::Name(n) => {
            names.push(n.clone());
        }
        Object::Array(arr) => {
            for item in arr {
                let resolved_item = resolve_object(item, doc);
                if let Object::Name(n) = resolved_item {
                    names.push(n.clone());
                }
            }
        }
        _ => {}
    }

    names
}

fn resolve_object<'a>(obj: &'a Object, doc: &'a Document) -> &'a Object {
    match *obj {
        Object::Reference(id) => doc.objects.get(&id).map_or(obj, |o| resolve_object(o, doc)),
        _ => obj,
    }
}

fn resolve_name(obj: &Object, doc: &Document) -> Result<Vec<u8>, lopdf::Error> {
    let resolved = resolve_object(obj, doc);
    match resolved {
        Object::Name(n) => Ok(n.clone()),
        _ => Err(lopdf::Error::ObjectNotFound((0, 0))),
    }
}

fn get_u32(dict: &lopdf::Dictionary, key: &[u8], doc: &Document) -> Option<u32> {
    let obj = dict.get(key).ok()?;
    let resolved = resolve_object(obj, doc);
    match *resolved {
        Object::Integer(i) => {
            if i >= 0 {
                Some(i as u32)
            } else {
                None
            }
        }
        Object::Real(f) => {
            if f >= 0.0 {
                Some(f as u32)
            } else {
                None
            }
        }
        _ => None,
    }
}
