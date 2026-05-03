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

use lopdf::{Document, Object, ObjectId};

use crate::pipeline::error::PipelineError;

pub struct ExtractedStream {
    pub data: Vec<u8>,
}

pub fn extract_stream(
    doc: &Document,
    object_id: ObjectId,
) -> Result<ExtractedStream, PipelineError> {
    let object = doc
        .objects
        .get(&object_id)
        .ok_or_else(|| PipelineError::ExtractionFailed {
            object_id: object_id.0,
            reason: "Object not found in document".to_string(),
        })?;

    match object {
        Object::Stream(stream) => {
            let data = match stream.decompressed_content() {
                Ok(decoded) => decoded,
                Err(_) => {
                    log::warn!(
                        "Could not decompress stream for object {}; using raw content",
                        object_id.0
                    );
                    stream.content.clone()
                }
            };

            Ok(ExtractedStream { data })
        }
        _ => Err(PipelineError::ExtractionFailed {
            object_id: object_id.0,
            reason: "Object is not a stream".to_string(),
        }),
    }
}

pub fn raw_stream_size(doc: &Document, object_id: ObjectId) -> Option<usize> {
    doc.objects.get(&object_id).and_then(|obj| {
        if let Object::Stream(stream) = obj {
            Some(stream.content.len())
        } else {
            None
        }
    })
}
