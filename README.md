<div align="center">

# TUIPDF

> **A beautifully crafted, terminal-native PDF compressor — built in Rust, powered by MuPDF.**

<br/>

[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen?style=for-the-badge)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-Welcome-blue?style=for-the-badge)](CONTRIBUTING.md)

<br/>

[Features](#-features) · [Installation](#-installation) · [Usage](#-usage) · [Roadmap](#-roadmap) · [Contributing](#-contributing)

</div>

<div align="center">
  <img src="assets/image.jpg" alt="tuipdf terminal preview" width="700"/>
</div>

---

## Overview

**tuipdf** is a terminal-native PDF tool built entirely in Rust. Inspired by the convenience of professional online document compressors (like iLovePDF and SmallPDF), it brings industry-grade PDF optimization directly into your terminal through a rich, keyboard-driven TUI experience.

**No uploads. No cloud. No tracking.** Your files never leave your machine.

Built for developers, power users, and open-source contributors who believe productivity tools should be **fast**, **local**, and **beautiful**.

---

## ✨ Features

| Feature | Description |
|---------|-------------|
| 📦 **Industry-Grade Compression** | Utilizes MuPDF's advanced API for optimal PDF size reduction (DPI downsampling, font subsetting, stream optimization). |
| 🎚️ **Compression Tiers** | Three tailored profiles (Low/Quality, Medium/Balanced, High/Extreme) to fit your needs. |
| 🖥️ **Interactive TUI** | Keyboard-driven, distraction-free terminal interface with modern aesthetics and smooth animations. |
| 🔒 **Privacy-first** | 100% local processing — nothing leaves your machine. |
| ⚡ **Blazing fast** | Rust-native performance combined with MuPDF's C backend. |
| 🌐 **Offline-ready** | No internet connection required, ever. |

---

## 🛠️ Prerequisites

`tuipdf` relies on the **MuPDF** library for its core compression engine. 

- **Linux (Ubuntu/Debian):** `sudo apt install libmupdf-dev mupdf-tools`
- **macOS:** `brew install mupdf-tools`
- **Windows:** MSYS2 with `mingw-w64-x86_64-mupdf` (see setup script)

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

## 🆚 tuipdf vs Online Tools

| | tuipdf | Online Tools |
|---|:---:|:---:|
| 🔒 Privacy | ✅ Files stay local | ❌ Uploaded to servers |
| ⚡ Speed | ✅ Instant, native | ❌ Depends on internet |
| 🌐 Offline Use | ✅ Always works | ❌ Requires connection |
| 💸 Cost | ✅ Free forever | ❌ Often paywalled/limited |
| 🖥️ Workflow | ✅ Stay in terminal | ❌ Switch to browser |
| 🛡️ Quality | ✅ MuPDF backend | ✅ Proprietary backends |

---

## 🛠️ Tech Stack

| Component | Purpose |
|-------|---------|
| [ratatui](https://github.com/ratatui-org/ratatui) | Rich TUI framework |
| [crossterm](https://github.com/crossterm-rs/crossterm) | Cross-platform terminal backend |
| [MuPDF](https://mupdf.com/) | High-performance C PDF engine |
| [clap](https://github.com/clap-rs/clap) | CLI argument parsing |

---

## 🗺️ Roadmap

**Phase 1: Core Compressor — ✅ Current**
- [x] High-performance C FFI compression engine (MuPDF integration)
- [x] Premium TUI with dynamic gradients, active animations, and stats
- [x] Robust cross-platform build scripts (Windows, Linux, macOS)
- [x] Tailored image rewriting and compression tiers (Low, Medium, High)

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
