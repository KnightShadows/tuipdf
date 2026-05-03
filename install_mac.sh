#!/bin/bash
# tuipdf
# ------
# A beautifully crafted, terminal-native PDF tool built in Rust.
# It aims to make compressing PDF files as fast, efficient and flexible
# as possible directly from your terminal.
#
# Authors: KnightShadows Team and individual contributors (see CONTRIBUTORS file)
#          Aditya Anand <aditya19study@gmail.com> (c) 2025
# Website: https://github.com/KnightShadows/tuipdf
# License: MPL-2.0 (see LICENSE file)

set -e

echo "Installing build dependencies via Homebrew for macOS..."
if ! command -v brew &> /dev/null; then
    echo "Homebrew not found. Please install it first from https://brew.sh"
    exit 1
fi

brew install cmake nasm

echo "Building tuipdf in release mode..."
cargo build --release

echo "Done! Binary is at target/release/tuipdf"
