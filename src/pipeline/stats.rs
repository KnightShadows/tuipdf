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

use std::sync::atomic::{AtomicU32, Ordering};

pub struct StatsCollector {
    images_compressed: AtomicU32,
    images_failed: AtomicU32,
    objects_removed: AtomicU32,
}

impl StatsCollector {
    pub fn new() -> Self {
        Self {
            images_compressed: AtomicU32::new(0),
            images_failed: AtomicU32::new(0),
            objects_removed: AtomicU32::new(0),
        }
    }

    pub fn record_image_compressed(&self) {
        self.images_compressed.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_image_failed(&self) {
        self.images_failed.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_objects_removed(&self, count: u32) {
        self.objects_removed.fetch_add(count, Ordering::Relaxed);
    }

    pub fn images_compressed(&self) -> u32 {
        self.images_compressed.load(Ordering::Relaxed)
    }

    pub fn images_failed(&self) -> u32 {
        self.images_failed.load(Ordering::Relaxed)
    }

    pub fn objects_removed(&self) -> u32 {
        self.objects_removed.load(Ordering::Relaxed)
    }
}
