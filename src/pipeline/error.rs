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

#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error("Failed to load PDF: {0}")]
    LoadFailed(String),

    #[error("Object {object_id} extraction failed: {reason}")]
    ExtractionFailed {
        object_id: u32,
        reason: String,
    },

    #[error("Failed to rebuild PDF: {0}")]
    RebuildFailed(String),

    #[error("Failed to write output: {0}")]
    WriteFailed(String),

    #[error("Unsupported PDF feature: {0}")]
    UnsupportedFeature(String),

}
