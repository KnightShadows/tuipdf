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

use std::io::Write;
use std::path::Path;

use lopdf::dictionary;
use lopdf::{Document, Object, Stream};

fn create_text_only_pdf(path: &Path) {
    let mut doc = Document::with_version("1.7");

    let content = b"BT /F1 12 Tf 100 700 Td (Hello World from tuipdf test!) Tj ET".to_vec();
    let content_stream = Stream::new(dictionary! {}, content.clone());
    let content_id = doc.add_object(Object::Stream(content_stream));

    let font_dict = dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
    };
    let font_id = doc.add_object(font_dict);

    let resources = dictionary! {
        "Font" => dictionary! {
            "F1" => Object::Reference(font_id),
        },
    };

    let page = dictionary! {
        "Type" => "Page",
        "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
        "Contents" => Object::Reference(content_id),
        "Resources" => resources,
    };
    let page_id = doc.add_object(page);

    let pages = dictionary! {
        "Type" => "Pages",
        "Kids" => vec![Object::Reference(page_id)],
        "Count" => 1,
    };
    let pages_id = doc.add_object(pages);

    if let Ok(Object::Dictionary(page_dict)) = doc.get_object_mut(page_id) {
        page_dict.set("Parent", Object::Reference(pages_id));
    }

    let catalog = dictionary! {
        "Type" => "Catalog",
        "Pages" => Object::Reference(pages_id),
    };
    let catalog_id = doc.add_object(catalog);

    doc.trailer.set("Root", Object::Reference(catalog_id));

    doc.save(path).ok();
}

fn create_image_heavy_pdf(path: &Path) {
    let mut doc = Document::with_version("1.7");

    let width: u32 = 200;
    let height: u32 = 200;
    let mut pixel_data = Vec::with_capacity((width * height * 3) as usize);
    for y in 0..height {
        for x in 0..width {
            pixel_data.push((x % 256) as u8);
            pixel_data.push((y % 256) as u8);
            pixel_data.push(((x + y) % 256) as u8);
        }
    }

    let image_stream = Stream::new(
        dictionary! {
            "Type" => "XObject",
            "Subtype" => "Image",
            "Width" => width as i64,
            "Height" => height as i64,
            "ColorSpace" => "DeviceRGB",
            "BitsPerComponent" => 8,
        },
        pixel_data,
    );
    let image_id = doc.add_object(Object::Stream(image_stream));

    let mut pixel_data2 = Vec::with_capacity((width * height * 3) as usize);
    for y in 0..height {
        for x in 0..width {
            pixel_data2.push(((x * 2) % 256) as u8);
            pixel_data2.push(((y * 2) % 256) as u8);
            pixel_data2.push(((x * y) % 256) as u8);
        }
    }
    let image_stream2 = Stream::new(
        dictionary! {
            "Type" => "XObject",
            "Subtype" => "Image",
            "Width" => width as i64,
            "Height" => height as i64,
            "ColorSpace" => "DeviceRGB",
            "BitsPerComponent" => 8,
        },
        pixel_data2,
    );
    let image_id2 = doc.add_object(Object::Stream(image_stream2));

    let content = format!(
        "q {} 0 0 {} 50 500 cm /Im1 Do Q q {} 0 0 {} 50 200 cm /Im2 Do Q",
        width, height, width, height
    );
    let content_stream = Stream::new(dictionary! {}, content.into_bytes());
    let content_id = doc.add_object(Object::Stream(content_stream));

    let resources = dictionary! {
        "XObject" => dictionary! {
            "Im1" => Object::Reference(image_id),
            "Im2" => Object::Reference(image_id2),
        },
    };

    let page = dictionary! {
        "Type" => "Page",
        "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
        "Contents" => Object::Reference(content_id),
        "Resources" => resources,
    };
    let page_id = doc.add_object(page);

    let pages = dictionary! {
        "Type" => "Pages",
        "Kids" => vec![Object::Reference(page_id)],
        "Count" => 1,
    };
    let pages_id = doc.add_object(pages);

    if let Ok(Object::Dictionary(page_dict)) = doc.get_object_mut(page_id) {
        page_dict.set("Parent", Object::Reference(pages_id));
    }

    let catalog = dictionary! {
        "Type" => "Catalog",
        "Pages" => Object::Reference(pages_id),
    };
    let catalog_id = doc.add_object(catalog);

    doc.trailer.set("Root", Object::Reference(catalog_id));
    doc.save(path).ok();
}

fn create_mixed_content_pdf(path: &Path) {
    let mut doc = Document::with_version("1.7");

    let width: u32 = 100;
    let height: u32 = 100;
    let mut pixel_data = Vec::with_capacity((width * height * 3) as usize);
    for y in 0..height {
        for x in 0..width {
            pixel_data.push((x % 256) as u8);
            pixel_data.push((y % 256) as u8);
            pixel_data.push(128u8);
        }
    }

    let image_stream = Stream::new(
        dictionary! {
            "Type" => "XObject",
            "Subtype" => "Image",
            "Width" => width as i64,
            "Height" => height as i64,
            "ColorSpace" => "DeviceRGB",
            "BitsPerComponent" => 8,
        },
        pixel_data,
    );
    let image_id = doc.add_object(Object::Stream(image_stream));

    let content = b"BT /F1 12 Tf 100 700 Td (Mixed content PDF test!) Tj ET q 100 0 0 100 50 400 cm /Im1 Do Q".to_vec();
    let content_stream = Stream::new(dictionary! {}, content);
    let content_id = doc.add_object(Object::Stream(content_stream));

    let font_dict = dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
    };
    let font_id = doc.add_object(font_dict);

    let resources = dictionary! {
        "Font" => dictionary! {
            "F1" => Object::Reference(font_id),
        },
        "XObject" => dictionary! {
            "Im1" => Object::Reference(image_id),
        },
    };

    let page = dictionary! {
        "Type" => "Page",
        "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
        "Contents" => Object::Reference(content_id),
        "Resources" => resources,
    };
    let page_id = doc.add_object(page);

    let pages = dictionary! {
        "Type" => "Pages",
        "Kids" => vec![Object::Reference(page_id)],
        "Count" => 1,
    };
    let pages_id = doc.add_object(pages);

    if let Ok(Object::Dictionary(page_dict)) = doc.get_object_mut(page_id) {
        page_dict.set("Parent", Object::Reference(pages_id));
    }

    let catalog = dictionary! {
        "Type" => "Catalog",
        "Pages" => Object::Reference(pages_id),
    };
    let catalog_id = doc.add_object(catalog);

    doc.trailer.set("Root", Object::Reference(catalog_id));
    doc.save(path).ok();
}

fn create_already_compressed_pdf(path: &Path) {
    let mut doc = Document::with_version("1.7");

    let content = b"BT /F1 10 Tf 72 720 Td (Tiny) Tj ET".to_vec();
    let content_stream = Stream::new(dictionary! {}, content);
    let content_id = doc.add_object(Object::Stream(content_stream));

    let font_dict = dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
    };
    let font_id = doc.add_object(font_dict);

    let resources = dictionary! {
        "Font" => dictionary! {
            "F1" => Object::Reference(font_id),
        },
    };

    let page = dictionary! {
        "Type" => "Page",
        "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
        "Contents" => Object::Reference(content_id),
        "Resources" => resources,
    };
    let page_id = doc.add_object(page);

    let pages = dictionary! {
        "Type" => "Pages",
        "Kids" => vec![Object::Reference(page_id)],
        "Count" => 1,
    };
    let pages_id = doc.add_object(pages);

    if let Ok(Object::Dictionary(page_dict)) = doc.get_object_mut(page_id) {
        page_dict.set("Parent", Object::Reference(pages_id));
    }

    let catalog = dictionary! {
        "Type" => "Catalog",
        "Pages" => Object::Reference(pages_id),
    };
    let catalog_id = doc.add_object(catalog);

    doc.trailer.set("Root", Object::Reference(catalog_id));
    doc.save(path).ok();
}

#[test]
fn test_text_only_pdf() {
    let dir = tempfile::tempdir().expect("Failed to create temp dir");
    let input_path = dir.path().join("text_only.pdf");
    let output_dir = dir.path().to_path_buf();

    create_text_only_pdf(&input_path);
    assert!(input_path.exists(), "Input PDF should exist");

    let original_size = std::fs::metadata(&input_path).expect("metadata").len();
    assert!(original_size > 0, "Input PDF should not be empty");

    let config = crate::pipeline::CompressionConfig {
        quality: 75,
        dpi_threshold: 150,
        output_dir: output_dir.clone(),
        remove_metadata: false,
    };

    let stats = crate::pipeline::compress_pdf(&input_path, &config)
        .expect("compress_pdf should succeed for text-only PDF");

    assert!(
        stats.compressed_bytes > 0,
        "Compressed file should not be empty"
    );

    let output_path = output_dir.join("text_only_compressed.pdf");
    assert!(output_path.exists(), "Output file should exist");

    let reloaded = Document::load(&output_path);
    assert!(
        reloaded.is_ok(),
        "Compressed PDF should be loadable by lopdf"
    );
}

#[test]
fn test_image_heavy_pdf() {
    let dir = tempfile::tempdir().expect("Failed to create temp dir");
    let input_path = dir.path().join("image_heavy.pdf");
    let output_dir = dir.path().to_path_buf();

    create_image_heavy_pdf(&input_path);
    assert!(input_path.exists(), "Input PDF should exist");

    let original_size = std::fs::metadata(&input_path).expect("metadata").len();
    assert!(original_size > 0, "Input PDF should not be empty");

    let config = crate::pipeline::CompressionConfig {
        quality: 75,
        dpi_threshold: 150,
        output_dir: output_dir.clone(),
        remove_metadata: false,
    };

    let stats = crate::pipeline::compress_pdf(&input_path, &config)
        .expect("compress_pdf should succeed for image-heavy PDF");

    assert!(
        stats.compressed_bytes < stats.original_bytes,
        "Compressed file ({}) should be smaller than original ({})",
        stats.compressed_bytes,
        stats.original_bytes
    );

    let output_path = output_dir.join("image_heavy_compressed.pdf");
    let reloaded = Document::load(&output_path);
    assert!(reloaded.is_ok(), "Compressed PDF should be loadable");
}

#[test]
fn test_mixed_content_pdf() {
    let dir = tempfile::tempdir().expect("Failed to create temp dir");
    let input_path = dir.path().join("mixed_content.pdf");
    let output_dir = dir.path().to_path_buf();

    create_mixed_content_pdf(&input_path);

    let _original_size = std::fs::metadata(&input_path).expect("metadata").len();

    let config = crate::pipeline::CompressionConfig {
        quality: 75,
        dpi_threshold: 150,
        output_dir: output_dir.clone(),
        remove_metadata: false,
    };

    let stats = crate::pipeline::compress_pdf(&input_path, &config)
        .expect("compress_pdf should succeed for mixed-content PDF");

    assert!(
        stats.compressed_bytes <= stats.original_bytes,
        "Compressed should not be larger than original"
    );

    let output_path = output_dir.join("mixed_content_compressed.pdf");
    let reloaded = Document::load(&output_path);
    assert!(reloaded.is_ok(), "Compressed PDF should be loadable");
}

#[test]
fn test_already_compressed_pdf() {
    let dir = tempfile::tempdir().expect("Failed to create temp dir");
    let input_path = dir.path().join("already_small.pdf");
    let output_dir = dir.path().to_path_buf();

    create_already_compressed_pdf(&input_path);

    let original_size = std::fs::metadata(&input_path).expect("metadata").len();

    let config = crate::pipeline::CompressionConfig {
        quality: 75,
        dpi_threshold: 150,
        output_dir: output_dir.clone(),
        remove_metadata: false,
    };

    let stats = crate::pipeline::compress_pdf(&input_path, &config)
        .expect("compress_pdf should succeed even for already-compressed PDF");

    let max_allowed = original_size + (original_size / 4) + 512;
    assert!(
        stats.compressed_bytes <= max_allowed,
        "Pipeline should not make file much larger: compressed {} vs original {} (max allowed {})",
        stats.compressed_bytes,
        original_size,
        max_allowed
    );
}

#[test]
fn test_malformed_pdf() {
    let dir = tempfile::tempdir().expect("Failed to create temp dir");
    let malformed_path = dir.path().join("malformed.pdf");

    let mut f = std::fs::File::create(&malformed_path).expect("create file");
    f.write_all(b"THIS IS NOT A PDF FILE AT ALL")
        .expect("write");
    drop(f);

    let config = crate::pipeline::CompressionConfig {
        quality: 75,
        dpi_threshold: 150,
        output_dir: dir.path().to_path_buf(),
        remove_metadata: false,
    };

    let result = crate::pipeline::compress_pdf(&malformed_path, &config);
    assert!(result.is_err(), "Malformed PDF should return an error");

    let err = result.unwrap_err();
    let err_msg = err.to_string();
    assert!(
        err_msg.contains("Failed to load PDF") || err_msg.contains("PDF structure is invalid"),
        "Error should indicate load or structure failure, got: {err_msg}"
    );
}

#[test]
fn test_scanned_grayscale_pdf() {
    let dir = tempfile::tempdir().expect("Failed to create temp dir");
    let input_path = dir.path().join("scanned_gray.pdf");
    let output_dir = dir.path().to_path_buf();

    create_scanned_grayscale_pdf(&input_path);
    assert!(input_path.exists(), "Input PDF should exist");

    let original_size = std::fs::metadata(&input_path).expect("metadata").len();
    assert!(original_size > 0, "Input PDF should not be empty");

    let config = crate::pipeline::CompressionConfig {
        quality: 75,
        dpi_threshold: 150,
        output_dir: output_dir.clone(),
        remove_metadata: false,
    };

    let stats = crate::pipeline::compress_pdf(&input_path, &config)
        .expect("compress_pdf should succeed for scanned grayscale PDF");

    assert!(
        stats.compressed_bytes > 0,
        "Compressed file should not be empty"
    );

    let output_path = output_dir.join("scanned_gray_compressed.pdf");
    assert!(output_path.exists(), "Output file should exist");

    let reloaded =
        Document::load(&output_path).expect("Compressed scanned PDF should be loadable");

    let mut found_content_stream = false;
    for (_, obj) in &reloaded.objects {
        if let Object::Stream(stream) = obj {
            let has_image_subtype = stream
                .dict
                .get(b"Subtype")
                .ok()
                .and_then(|o| {
                    if let Object::Name(n) = o {
                        Some(n.as_slice() == b"Image")
                    } else {
                        None
                    }
                })
                .unwrap_or(false);

            if !has_image_subtype && !stream.content.is_empty() {
                found_content_stream = true;
            }
        }
    }

    assert!(
        found_content_stream,
        "Output PDF must have a non-empty content stream (otherwise pages are blank)"
    );
}

fn create_scanned_grayscale_pdf(path: &Path) {
    let mut doc = Document::with_version("1.7");

    let width: u32 = 100;
    let height: u32 = 100;
    let mut pixel_data = Vec::with_capacity((width * height) as usize);
    for y in 0..height {
        for x in 0..width {
            pixel_data.push(((x + y) % 256) as u8);
        }
    }

    let image_stream = Stream::new(
        dictionary! {
            "Type" => "XObject",
            "Subtype" => "Image",
            "Width" => width as i64,
            "Height" => height as i64,
            "ColorSpace" => "DeviceGray",
            "BitsPerComponent" => 8,
        },
        pixel_data,
    );
    let image_id = doc.add_object(Object::Stream(image_stream));

    let content = format!("q {} 0 0 {} 0 0 cm /Im1 Do Q", width, height);
    let content_stream = Stream::new(dictionary! {}, content.into_bytes());
    let content_id = doc.add_object(Object::Stream(content_stream));

    let resources = dictionary! {
        "XObject" => dictionary! {
            "Im1" => Object::Reference(image_id),
        },
    };

    let page = dictionary! {
        "Type" => "Page",
        "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
        "Contents" => Object::Reference(content_id),
        "Resources" => resources,
    };
    let page_id = doc.add_object(page);

    let pages = dictionary! {
        "Type" => "Pages",
        "Kids" => vec![Object::Reference(page_id)],
        "Count" => 1,
    };
    let pages_id = doc.add_object(pages);

    if let Ok(Object::Dictionary(page_dict)) = doc.get_object_mut(page_id) {
        page_dict.set("Parent", Object::Reference(pages_id));
    }

    let catalog = dictionary! {
        "Type" => "Catalog",
        "Pages" => Object::Reference(pages_id),
    };
    let catalog_id = doc.add_object(catalog);

    doc.trailer.set("Root", Object::Reference(catalog_id));
    doc.save(path).ok();
}

#[test]
fn test_colored_rgb_pdf() {
    let dir = tempfile::tempdir().expect("Failed to create temp dir");
    let input_path = dir.path().join("colored_rgb.pdf");
    let output_dir = dir.path().to_path_buf();

    create_colored_rgb_pdf(&input_path);
    assert!(input_path.exists(), "Input PDF should exist");

    let config = crate::pipeline::CompressionConfig {
        quality: 75,
        dpi_threshold: 150,
        output_dir: output_dir.clone(),
        remove_metadata: false,
    };

    let stats = crate::pipeline::compress_pdf(&input_path, &config)
        .expect("compress_pdf should succeed for colored RGB PDF");

    assert!(
        stats.compressed_bytes > 0,
        "Compressed file should not be empty"
    );

    let output_path = output_dir.join("colored_rgb_compressed.pdf");
    assert!(output_path.exists(), "Output file should exist");

    let reloaded = Document::load(&output_path).expect("Compressed RGB PDF should be loadable");

    let mut found_content_stream = false;
    for (_, obj) in &reloaded.objects {
        if let Object::Stream(stream) = obj {
            let has_image_subtype = stream
                .dict
                .get(b"Subtype")
                .ok()
                .and_then(|o| {
                    if let Object::Name(n) = o {
                        Some(n.as_slice() == b"Image")
                    } else {
                        None
                    }
                })
                .unwrap_or(false);

            if !has_image_subtype && !stream.content.is_empty() {
                found_content_stream = true;
            }
        }
    }
    assert!(
        found_content_stream,
        "Output PDF must have a non-empty content stream"
    );

    let mut found_image = false;
    for (_, obj) in &reloaded.objects {
        if let Object::Stream(stream) = obj {
            let is_image = stream
                .dict
                .get(b"Subtype")
                .ok()
                .and_then(|o| {
                    if let Object::Name(n) = o {
                        Some(n.as_slice() == b"Image")
                    } else {
                        None
                    }
                })
                .unwrap_or(false);

            if is_image && !stream.content.is_empty() {
                found_image = true;
            }
        }
    }
    assert!(found_image, "Output PDF must still contain image data");
}

fn create_colored_rgb_pdf(path: &Path) {
    let mut doc = Document::with_version("1.7");

    let width: u32 = 150;
    let height: u32 = 150;
    let mut pixel_data = Vec::with_capacity((width * height * 3) as usize);
    for y in 0..height {
        for x in 0..width {
            pixel_data.push(((x * 255 / width) % 256) as u8);
            pixel_data.push(((y * 255 / height) % 256) as u8);
            pixel_data.push((128u32 + (x + y) % 128) as u8);
        }
    }

    let image_stream = Stream::new(
        dictionary! {
            "Type" => "XObject",
            "Subtype" => "Image",
            "Width" => width as i64,
            "Height" => height as i64,
            "ColorSpace" => "DeviceRGB",
            "BitsPerComponent" => 8,
        },
        pixel_data,
    );
    let image_id = doc.add_object(Object::Stream(image_stream));

    let content = format!(
        "BT /F1 12 Tf 50 750 Td (Textbook with colored image) Tj ET q {} 0 0 {} 50 400 cm /Im1 Do Q",
        width, height
    );
    let content_stream = Stream::new(dictionary! {}, content.into_bytes());
    let content_id = doc.add_object(Object::Stream(content_stream));

    let font_dict = dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
    };
    let font_id = doc.add_object(font_dict);

    let resources = dictionary! {
        "Font" => dictionary! {
            "F1" => Object::Reference(font_id),
        },
        "XObject" => dictionary! {
            "Im1" => Object::Reference(image_id),
        },
    };

    let page = dictionary! {
        "Type" => "Page",
        "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
        "Contents" => Object::Reference(content_id),
        "Resources" => resources,
    };
    let page_id = doc.add_object(page);

    let pages = dictionary! {
        "Type" => "Pages",
        "Kids" => vec![Object::Reference(page_id)],
        "Count" => 1,
    };
    let pages_id = doc.add_object(pages);

    if let Ok(Object::Dictionary(page_dict)) = doc.get_object_mut(page_id) {
        page_dict.set("Parent", Object::Reference(pages_id));
    }

    let catalog = dictionary! {
        "Type" => "Catalog",
        "Pages" => Object::Reference(pages_id),
    };
    let catalog_id = doc.add_object(catalog);

    doc.trailer.set("Root", Object::Reference(catalog_id));
    doc.save(path).ok();
}

#[test]
fn test_cmyk_image_pdf() {
    let dir = tempfile::tempdir().expect("Failed to create temp dir");
    let input_path = dir.path().join("cmyk_image.pdf");
    let output_dir = dir.path().to_path_buf();

    create_cmyk_image_pdf(&input_path);
    assert!(input_path.exists(), "Input PDF should exist");

    let config = crate::pipeline::CompressionConfig {
        quality: 75,
        dpi_threshold: 150,
        output_dir: output_dir.clone(),
        remove_metadata: false,
    };

    let stats = crate::pipeline::compress_pdf(&input_path, &config)
        .expect("compress_pdf should succeed for CMYK image PDF");

    assert!(
        stats.compressed_bytes > 0,
        "Compressed file should not be empty"
    );

    let output_path = output_dir.join("cmyk_image_compressed.pdf");
    let reloaded = Document::load(&output_path).expect("Compressed CMYK PDF should be loadable");

    let mut found_cmyk_image = false;
    for (_, obj) in &reloaded.objects {
        if let Object::Stream(stream) = obj {
            let is_image = stream
                .dict
                .get(b"Subtype")
                .ok()
                .and_then(|o| {
                    if let Object::Name(n) = o {
                        Some(n.as_slice() == b"Image")
                    } else {
                        None
                    }
                })
                .unwrap_or(false);

            if is_image && !stream.content.is_empty() {
                found_cmyk_image = true;
            }
        }
    }
    assert!(
        found_cmyk_image,
        "CMYK image must be preserved in the output PDF"
    );
}

fn create_cmyk_image_pdf(path: &Path) {
    let mut doc = Document::with_version("1.7");

    let width: u32 = 80;
    let height: u32 = 80;
    let mut pixel_data = Vec::with_capacity((width * height * 4) as usize);
    for y in 0..height {
        for x in 0..width {
            pixel_data.push(((x * 3) % 256) as u8);
            pixel_data.push(((y * 3) % 256) as u8);
            pixel_data.push(100u8);
            pixel_data.push(20u8);
        }
    }

    let image_stream = Stream::new(
        dictionary! {
            "Type" => "XObject",
            "Subtype" => "Image",
            "Width" => width as i64,
            "Height" => height as i64,
            "ColorSpace" => "DeviceCMYK",
            "BitsPerComponent" => 8,
        },
        pixel_data,
    );
    let image_id = doc.add_object(Object::Stream(image_stream));

    let content = format!("q {} 0 0 {} 50 500 cm /Im1 Do Q", width, height);
    let content_stream = Stream::new(dictionary! {}, content.into_bytes());
    let content_id = doc.add_object(Object::Stream(content_stream));

    let resources = dictionary! {
        "XObject" => dictionary! {
            "Im1" => Object::Reference(image_id),
        },
    };

    let page = dictionary! {
        "Type" => "Page",
        "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
        "Contents" => Object::Reference(content_id),
        "Resources" => resources,
    };
    let page_id = doc.add_object(page);

    let pages = dictionary! {
        "Type" => "Pages",
        "Kids" => vec![Object::Reference(page_id)],
        "Count" => 1,
    };
    let pages_id = doc.add_object(pages);

    if let Ok(Object::Dictionary(page_dict)) = doc.get_object_mut(page_id) {
        page_dict.set("Parent", Object::Reference(pages_id));
    }

    let catalog = dictionary! {
        "Type" => "Catalog",
        "Pages" => Object::Reference(pages_id),
    };
    let catalog_id = doc.add_object(catalog);

    doc.trailer.set("Root", Object::Reference(catalog_id));
    doc.save(path).ok();
}
