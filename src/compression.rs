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

use std::fs;
use std::os::raw::{c_char, c_int, c_uchar};
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::ffi::CStr;

use anyhow::{anyhow, Result};

use std::fmt;

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


#[repr(C)]
#[derive(Debug, Copy, Clone)]
enum CCompressionLevel {
    Low = 0,
    Medium = 1,
    High = 2,
}

#[repr(C)]
struct CCompressResult {
    data: *mut c_uchar,
    size: usize,
    error_code: c_int,
    error_msg: [c_char; 256],
}

extern "C" {
    fn compress_pdf(
        input: *const c_uchar,
        input_size: usize,
        level: CCompressionLevel,
    ) -> CCompressResult;
    fn free_compress_result(result: *mut CCompressResult);
}


pub fn start(
    input: String,
    output: String,
    level: CompressionLevel,
) -> (JoinHandle<()>, mpsc::Receiver<CompressionMsg>) {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let result = (|| -> Result<(u64, u64)> {
            let original_data = fs::read(&input)?;
            let original_size = original_data.len() as u64;

            let _ = tx.send(CompressionMsg::Progress(0.2));
            
            let compressed_data = compress_pdf_ffi(&original_data, level)?;
            
            let _ = tx.send(CompressionMsg::Progress(0.8));
            fs::write(&output, &compressed_data)?;
            let compressed_size = compressed_data.len() as u64;

            Ok((original_size, compressed_size))
        })();

        match result {
            Ok((original, compressed)) => {
                let _ = tx.send(CompressionMsg::Progress(1.0));
                thread::sleep(Duration::from_millis(200));
                let _ = tx.send(CompressionMsg::Done { original, compressed });
            }
            Err(e) => {
                let _ = tx.send(CompressionMsg::Error(e.to_string()));
            }
        }
    });

    (handle, rx)
}

pub fn compress_pdf_ffi(input: &[u8], level: CompressionLevel) -> Result<Vec<u8>> {
    let c_level = match level {
        CompressionLevel::Low => CCompressionLevel::Low,
        CompressionLevel::Medium => CCompressionLevel::Medium,
        CompressionLevel::High => CCompressionLevel::High,
    };

    unsafe {
        let mut res = compress_pdf(input.as_ptr(), input.len(), c_level);

        if res.error_code != 0 {
            let msg = CStr::from_ptr(res.error_msg.as_ptr())
                .to_string_lossy()
                .into_owned();
            free_compress_result(&mut res);
            return Err(anyhow!("MuPDF Error: {}", msg));
        }

        if res.data.is_null() {
            return Err(anyhow!("MuPDF returned null data without error code"));
        }

        let slice = std::slice::from_raw_parts(res.data, res.size);
        let out = slice.to_vec();

        free_compress_result(&mut res);
        
        Ok(out)
    }
}