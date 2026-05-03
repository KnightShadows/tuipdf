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

use crate::pipeline::error::PipelineError;

const LOW_MEMORY_THRESHOLD_BYTES: u64 = 200 * 1024 * 1024;

#[cfg(target_os = "windows")]
fn available_memory_bytes() -> Option<u64> {
    use std::mem::MaybeUninit;

    #[repr(C)]
    #[allow(non_snake_case, non_camel_case_types, clippy::upper_case_acronyms)]
    struct MEMORYSTATUSEX {
        dwLength: u32,
        dwMemoryLoad: u32,
        ullTotalPhys: u64,
        ullAvailPhys: u64,
        ullTotalPageFile: u64,
        ullAvailPageFile: u64,
        ullTotalVirtual: u64,
        ullAvailVirtual: u64,
        ullAvailExtendedVirtual: u64,
    }

    unsafe extern "system" {
        fn GlobalMemoryStatusEx(lpBuffer: *mut MEMORYSTATUSEX) -> i32;
    }

    let mut status = MaybeUninit::<MEMORYSTATUSEX>::zeroed();
    unsafe {
        let ptr = status.as_mut_ptr();
        (*ptr).dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;
        if GlobalMemoryStatusEx(ptr) == 0 {
            return None;
        }
        Some((*ptr).ullAvailPhys)
    }
}

#[cfg(target_os = "linux")]
fn available_memory_bytes() -> Option<u64> {
    let contents = std::fs::read_to_string("/proc/meminfo").ok()?;
    for line in contents.lines() {
        if let Some(rest) = line.strip_prefix("MemAvailable:") {
            let kb_str = rest.trim().strip_suffix("kB")?.trim();
            let kb: u64 = kb_str.parse().ok()?;
            return Some(kb * 1024);
        }
    }
    None
}

#[cfg(target_os = "macos")]
fn available_memory_bytes() -> Option<u64> {
    use std::process::Command;
    let output = Command::new("sysctl")
        .args(["-n", "hw.memsize"])
        .output()
        .ok()?;
    let total_str = String::from_utf8_lossy(&output.stdout);
    let total: u64 = total_str.trim().parse().ok()?;
    Some(total / 4)
}

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
fn available_memory_bytes() -> Option<u64> {
    None
}

pub fn has_sufficient_memory() -> bool {
    match available_memory_bytes() {
        Some(avail) => avail >= LOW_MEMORY_THRESHOLD_BYTES,
        None => {
            log::warn!("Could not detect available memory; defaulting to parallel processing");
            true
        }
    }
}

pub fn check_memory_pressure() -> Result<bool, PipelineError> {
    let sufficient = has_sufficient_memory();
    if !sufficient {
        log::warn!(
            "Available system memory below {}MB — switching to sequential processing",
            LOW_MEMORY_THRESHOLD_BYTES / (1024 * 1024)
        );
    }
    Ok(sufficient)
}
