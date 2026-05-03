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

use std::sync::mpsc;
use std::thread::JoinHandle;

use crate::compression::{self, CompressionMsg};
use crate::input::InputField;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    InputPath,
    CompressionLevel,
    ActionButton,
}

pub use crate::compression::CompressionLevel;

#[derive(Debug)]
pub enum Status {
    Idle,
    Compressing,
    Done { original: u64, compressed: u64 },
    Error(String),
}

pub struct App {
    pub input_field: InputField,
    pub output_path: String,
    pub status: Status,
    pub progress: f32,
    pub focus: Focus,
    pub compression_handle: Option<JoinHandle<()>>,
    pub compression_rx: Option<mpsc::Receiver<CompressionMsg>>,
    pub should_quit: bool,
    pub tick: usize,
    pub prompt: Option<String>,
    pub compression_level: CompressionLevel,
}

impl App {
    pub fn new() -> Self {
        Self {
            input_field: InputField::new(),
            output_path: String::new(),
            status: Status::Idle,
            progress: 0.0,
            focus: Focus::InputPath,
            compression_handle: None,
            compression_rx: None,
            should_quit: false,
            tick: 0,
            prompt: None,
            compression_level: CompressionLevel::Medium,
        }
    }

    pub fn cycle_focus(&mut self) {
        self.focus = match self.focus {
            Focus::InputPath => Focus::CompressionLevel,
            Focus::CompressionLevel => Focus::ActionButton,
            Focus::ActionButton => Focus::InputPath,
        };
    }

    pub fn derive_output_path(&mut self) {
        let input = self.input_field.value();
        if input.is_empty() {
            self.output_path.clear();
            return;
        }
        if let Some(stem) = input
            .strip_suffix(".pdf")
            .or_else(|| input.strip_suffix(".PDF"))
        {
            self.output_path = format!("{}_compressed.pdf", stem);
        } else {
            self.output_path = format!("{}_compressed.pdf", input);
        }
    }

    pub fn try_compress(&mut self) -> Option<String> {
        if matches!(self.status, Status::Compressing) {
            return Some("Compression already in progress.".to_string());
        }

        let input = self.input_field.value().to_string();
        if input.is_empty() {
            return Some("Please enter an input PDF path.".to_string());
        }

        if !std::path::Path::new(&input).exists() {
            return Some(format!("File not found: {}", input));
        }

        self.derive_output_path();
        let output = self.output_path.clone();

        self.progress = 0.0;
        self.status = Status::Compressing;

        let (handle, rx) = compression::start(input, output, self.compression_level);
        self.compression_handle = Some(handle);
        self.compression_rx = Some(rx);

        None
    }

    pub fn poll_compression(&mut self) {
        let rx = match self.compression_rx.as_ref() {
            Some(rx) => rx,
            None => return,
        };

        loop {
            match rx.try_recv() {
                Ok(CompressionMsg::Progress(p)) => {
                    self.progress = p;
                }
                Ok(CompressionMsg::Done {
                    original,
                    compressed,
                }) => {
                    self.progress = 1.0;
                    self.status = Status::Done {
                        original,
                        compressed,
                    };
                    self.cleanup_thread();
                    break;
                }
                Ok(CompressionMsg::Error(msg)) => {
                    self.status = Status::Error(msg);
                    self.cleanup_thread();
                    break;
                }
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => {
                    if matches!(self.status, Status::Compressing) {
                        self.status = Status::Error(
                            "Compression thread disconnected unexpectedly.".to_string(),
                        );
                    }
                    self.cleanup_thread();
                    break;
                }
            }
        }
    }

    fn cleanup_thread(&mut self) {
        self.compression_rx = None;
        self.compression_handle.take();
    }

    pub fn tick(&mut self) {
        self.tick = self.tick.wrapping_add(1);
    }

    pub fn input_file_size(&self) -> Option<u64> {
        let path = self.input_field.value();
        if path.is_empty() {
            return None;
        }
        std::fs::metadata(path).ok().map(|m| m.len())
    }
}
