<div align="center">

# TUIPDF

> **A beautifully crafted, terminal-native PDF compressor — built in pure Rust.**

<br/>

[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen?style=for-the-badge)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-Welcome-blue?style=for-the-badge)](CONTRIBUTING.md)

<br/>

[Features](#-features) · [Installation](#-installation) · [Usage](#-usage) · [How It Works](#-how-it-works) · [Roadmap](#-roadmap) · [Contributing](#-contributing)

</div>

<div align="center">
  <img src="assets/image.jpg" alt="tuipdf terminal preview" width="700"/>
</div>

---

## Overview

**tuipdf** is a terminal-native PDF compressor built entirely in Rust. It brings production-grade PDF optimization directly into your terminal through a rich, keyboard-driven TUI experience — with **zero external C dependencies** and **zero third-party tool licenses** to worry about.

**No uploads. No cloud. No tracking.** Your files never leave your machine.

Built for developers, power users, and open-source contributors who believe productivity tools should be **fast**, **local**, and **beautiful**.

### Compression Engine

The compression pipeline is implemented in **pure Rust** with a color-space-aware, per-object strategy:

- **JPEG images** → decoded via the `image` crate → optional DPI downsampling → re-encoded at target quality
- **FlateDecode images** (PNG-style) → decompressed → color-space-aware reconstruction → re-encoded as JPEG
- **Raw bitmaps** → interpreted using PDF dictionary metadata (Width, Height, BitsPerComponent, ColorSpace) → JPEG
- **Grayscale / 1-bit B&W images** → preserved as grayscale JPEG to avoid unnecessary channel bloat
- **CMYK images** → converted to RGB via standard subtractive color model before JPEG encoding
- **Indexed / Separation / DeviceN** color spaces → safely skipped to prevent discoloration
- **Content streams, fonts, vector graphics, form XObjects** → kept intact (never recompressed)

---

## ✨ Features

| Feature | Description |
|---------|-------------|
| 📦 **Smart Compression** | Color-space-aware, per-object compression — images get JPEG re-encoding with proper DeviceRGB/DeviceGray/CMYK handling; fonts, vectors, and page operators stay untouched. |
| 🎚️ **Compression Tiers** | Three tailored profiles — **Low** (quality-first, 85%), **Medium** (balanced, 75%), **High** (aggressive, 50% + metadata strip). |
| 📄 **Scanned PDF Support** | Handles scanned B&W, grayscale, and color documents. Supports 1-bit, 2-bit, 4-bit, and 8-bit image depths. |
| 🎨 **Color Fidelity** | Full color space detection (DeviceRGB, DeviceGray, DeviceCMYK, ICCBased, CalGray, CalRGB) prevents discoloration. |
| 🖥️ **Interactive TUI** | Keyboard-driven, distraction-free terminal interface with modern aesthetics and smooth animations. |
| 🔒 **Privacy-first** | 100% local processing — nothing leaves your machine. |
| ⚡ **Blazing fast** | Pure Rust with parallel processing via rayon, automatic memory-pressure detection. |
| 🌐 **Offline-ready** | No internet connection required, ever. |
| 🔧 **Zero C deps** | No system libraries, no CMake, no MuPDF — everything builds from source via Cargo. |

---

## 🛠️ Prerequisites

- **Rust** toolchain (install from [rustup.rs](https://rustup.rs/))

That's it. No CMake, no C compiler, no system libraries needed.

---

## 📦 Installation

### Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/KnightShadows/tuipdf/main/install_windows.ps1 | iex
```

### Linux / macOS

```bash
curl -sSL https://raw.githubusercontent.com/KnightShadows/tuipdf/main/install.sh | bash
```

### From Source

```bash
git clone https://github.com/KnightShadows/tuipdf.git
cd tuipdf
cargo build --release
./target/release/tuipdf
```

---

## 🚀 Usage

Launch the interactive TUI:

```bash
tuipdf
```

### Direct Open

Launch with a file path to instantly load the PDF into the app:

```bash
tuipdf path/to/document.pdf
```

### AI-Enhanced Prompt (Experimental)

Use the `--prompt` or `-p` flag to provide specific instructions for the compression run. Currently, this displays in the UI as context for upcoming AI-driven optimization strategies.

```bash
tuipdf file.pdf --prompt "optimize for web viewing"
```

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `Enter` | Cycle compression levels / Start Compression |
| `Tab` | Switch UI Focus |
| `q` | Quit |
| `Esc` | Quit |
| `Ctrl+C`| Force Quit |

---

## ⚙️ How It Works

```
┌──────────┐     ┌───────────┐     ┌──────────────┐     ┌───────────┐     ┌──────────┐
│  Load &  │────▶│ Classify  │────▶│  Extract &   │────▶│ Rebuild & │────▶│  Write   │
│  Parse   │     │  Objects  │     │  Compress    │     │ Reinsert  │     │  Output  │
└──────────┘     └───────────┘     └──────────────┘     └───────────┘     └──────────┘
     │                │                    │                   │
   lopdf         Color space         image crate          lopdf
   loads         detection           decodes &            writes
   PDF           per stream          re-encodes           final PDF
```

1. **Parse** — `lopdf` loads and validates the PDF structure
2. **Classify** — Each stream object is tagged: JPEG, PNG/Flate, RawBitmap, Font, FormXObject, or Content Stream
3. **Color Detect** — The `ColorSpace` dictionary entry is fully resolved (DeviceRGB, DeviceCMYK, ICCBased N=?, Indexed → skip)
4. **Extract** — Streams are decompressed to raw pixel bytes
5. **Compress** — Images are re-encoded as JPEG at target quality; content streams and fonts are untouched
6. **Rebuild** — Compressed data is reinserted with updated dictionary entries (Filter, Width, Height, ColorSpace, BitsPerComponent)
7. **Write** — The document is saved with standard cross-reference tables

---

## 🆚 tuipdf vs Online Tools

| | tuipdf | Online Tools |
|---|:---:|:---:|
| 🔒 Privacy | ✅ Files stay local | ❌ Uploaded to servers |
| ⚡ Speed | ✅ Instant, native | ❌ Depends on internet |
| 🌐 Offline Use | ✅ Always works | ❌ Requires connection |
| 💸 Cost | ✅ Free forever | ❌ Often paywalled/limited |
| 🖥️ Workflow | ✅ Stay in terminal | ❌ Switch to browser |
| 🔧 Setup | ✅ `cargo install` | ✅ No setup needed |
| 📄 Scanned PDFs | ✅ Full support | ⚠️ Often rejected |
| 🎨 Color Fidelity | ✅ CMYK/ICC aware | ⚠️ May discolor |

---

## 🛠️ Tech Stack

| Component | Purpose |
|-------|---------|
| [ratatui](https://github.com/ratatui-org/ratatui) | Rich TUI framework |
| [crossterm](https://github.com/crossterm-rs/crossterm) | Cross-platform terminal backend |
| [lopdf](https://github.com/J-F-Liu/lopdf) | PDF parsing and manipulation |
| [image](https://github.com/image-rs/image) | Image decoding, encoding (JPEG, PNG, grayscale) |
| [flate2](https://github.com/rust-lang/flate2-rs) | Zlib compression for FlateDecode streams |
| [rayon](https://github.com/rayon-rs/rayon) | Data-parallel processing |
| [clap](https://github.com/clap-rs/clap) | CLI argument parsing |

---

## 🗺️ Roadmap

**Phase 1: Core Compressor — ✅ Complete**
- [x] Pure-Rust compression pipeline (lopdf + image + flate2)
- [x] Color-space-aware image handling (DeviceRGB, DeviceGray, DeviceCMYK, ICCBased)
- [x] Scanned PDF support (1-bit B&W, grayscale, and color)
- [x] Premium TUI with dynamic gradients, active animations, and stats
- [x] Robust cross-platform build (Windows, Linux, macOS)
- [x] Tailored compression tiers (Low, Medium, High)
- [x] Parallel processing with memory-pressure detection
- [x] Comprehensive integration test suite

**Phase 2: Conversions & Operations**
- [ ] PDF to Word
- [ ] Word to PDF
- [ ] PDF to Image (JPEG, PNG)
- [ ] Image to PDF
- [ ] Merge PDFs
- [ ] Split PDF
- [ ] Extract or Remove Pages

**Phase 3: Security & UX**
- [ ] Protect PDF (Add Password)
- [ ] Unlock PDF (Remove Password)
- [ ] Batch Processing Mode (Multiple files at once)
- [ ] Native File Picker (Select files visually in the terminal)

---

## 🤝 Contributing

Contributions are what make open source great. All contributions are welcome — bug fixes, features, docs, or suggestions.

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

## 📄 License

Copyright 2026 KnightShadows Organization & Aditya Anand

Licensed under the [Mozilla Public License 2.0](LICENSE).

---

<div align="center">

**Made with 🦀 by [Aditya Anand](https://github.com/adityaanand05) under [KnightShadows](https://github.com/KnightShadows)**

*Fast. Local. Open. — tuipdf*

</div>
