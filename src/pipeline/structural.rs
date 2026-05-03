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

use std::io::Write;

use lopdf::{Document, SaveOptions};

use crate::pipeline::error::PipelineError;

pub fn save_with_structural_compression<W: Write>(
    doc: &mut Document,
    writer: &mut W,
) -> Result<(), PipelineError> {
    let options = SaveOptions {
        use_object_streams: false,
        use_xref_streams: false,
        linearize: false,
        ..SaveOptions::default()
    };

    doc.save_with_options(writer, options)
        .map_err(|e| PipelineError::WriteFailed(e.to_string()))
}
