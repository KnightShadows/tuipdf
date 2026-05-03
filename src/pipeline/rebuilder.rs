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

use lopdf::{Document, Object, ObjectId};

use crate::pipeline::compressor::CompressedData;
use crate::pipeline::error::PipelineError;

pub fn reinsert_compressed_stream(
    doc: &mut Document,
    object_id: ObjectId,
    compressed: CompressedData,
) -> Result<(), PipelineError> {
    let obj = doc
        .objects
        .get_mut(&object_id)
        .ok_or_else(|| PipelineError::RebuildFailed(format!("Object {:?} not found", object_id)))?;

    match obj {
        Object::Stream(stream) => {
            let new_len = compressed.data.len();
            stream.content = compressed.data;

            stream.dict.set("Length", Object::Integer(new_len as i64));

            if let Some(w) = compressed.width {
                stream.dict.set("Width", Object::Integer(w as i64));
            }
            if let Some(h) = compressed.height {
                stream.dict.set("Height", Object::Integer(h as i64));
            }
            if let Some(cs) = compressed.color_space {
                stream.dict.set("ColorSpace", Object::Name(cs.as_bytes().to_vec()));
            }
            if let Some(bpc) = compressed.bits_per_component {
                stream.dict.set("BitsPerComponent", Object::Integer(bpc as i64));
            }

            if compressed.is_jpeg {
                stream
                    .dict
                    .set("Filter", Object::Name(b"DCTDecode".to_vec()));
            } else if compressed.is_flate {
                stream
                    .dict
                    .set("Filter", Object::Name(b"FlateDecode".to_vec()));
            } else {
                stream.dict.remove(b"Filter");
            }

            stream.dict.remove(b"DecodeParms");

            Ok(())
        }
        _ => Err(PipelineError::RebuildFailed(format!(
            "Object {:?} is not a stream",
            object_id
        ))),
    }
}

pub fn remove_unused_objects(doc: &mut Document) -> u32 {
    let _before = doc.objects.len();

    let mut referenced = std::collections::HashSet::new();
    let trailer_obj = Object::Dictionary(doc.trailer.clone());
    collect_references(&trailer_obj, doc, &mut referenced);

    let to_remove: Vec<ObjectId> = doc
        .objects
        .keys()
        .filter(|id| !referenced.contains(id))
        .copied()
        .collect();

    for id in &to_remove {
        doc.objects.remove(id);
    }

    to_remove.len() as u32
}

fn collect_references(
    obj: &Object,
    doc: &Document,
    referenced: &mut std::collections::HashSet<ObjectId>,
) {
    match obj {
        Object::Reference(id) => {
            if referenced.insert(*id)
                && let Some(target) = doc.objects.get(id) {
                    collect_references(target, doc, referenced);
                }
        }
        Object::Array(arr) => {
            for item in arr {
                collect_references(item, doc, referenced);
            }
        }
        Object::Dictionary(dict) => {
            for (_, value) in dict.iter() {
                collect_references(value, doc, referenced);
            }
        }
        Object::Stream(stream) => {
            for (_, value) in stream.dict.iter() {
                collect_references(value, doc, referenced);
            }
        }
        _ => {}
    }
}

pub fn strip_metadata(doc: &mut Document) {
    doc.trailer.remove(b"Info");

    let xmp_ids: Vec<ObjectId> = doc
        .objects
        .iter()
        .filter_map(|(&id, obj)| {
            if let Object::Stream(stream) = obj {
                let is_metadata = stream
                    .dict
                    .get(b"Type")
                    .ok()
                    .and_then(|o| {
                        if let Object::Name(n) = o {
                            Some(n.as_slice() == b"Metadata")
                        } else {
                            None
                        }
                    })
                    .unwrap_or(false);

                let is_xml = stream
                    .dict
                    .get(b"Subtype")
                    .ok()
                    .and_then(|o| {
                        if let Object::Name(n) = o {
                            Some(n.as_slice() == b"XML")
                        } else {
                            None
                        }
                    })
                    .unwrap_or(false);

                if is_metadata && is_xml {
                    return Some(id);
                }
            }
            None
        })
        .collect();

    for id in xmp_ids {
        doc.objects.remove(&id);
    }
}
