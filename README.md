<div align="center">

```
╔════════════════════════════════════════════════════════════╗
║                                                            ║
║   ████████╗██╗   ██╗██╗██████╗ ██████╗ ███████╗           ║
║      ██╔══╝██║   ██║██║██╔══██╗██╔══██╗██╔════╝           ║
║      ██║   ██║   ██║██║██████╔╝██║  ██║█████╗             ║
║      ██║   ██║   ██║██║██╔═══╝ ██║  ██║██╔══╝             ║
║      ██║   ╚██████╔╝██║██║     ██████╔╝██║                ║
║      ╚═╝    ╚═════╝ ╚═╝╚═╝     ╚═════╝ ╚═╝                ║
║                                                            ║
║          ⚡ blazing-fast  🔒 privacy-first  🦀 rust         ║
║                                                            ║
╚════════════════════════════════════════════════════════════╝
```

> **A terminal-native PDF toolkit — built in Rust, runs offline, leaves no trace.**

<br/>

[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen?style=for-the-badge)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-Welcome-blue?style=for-the-badge)](CONTRIBUTING.md)
[![Maintained](https://img.shields.io/badge/Maintained-Yes-success?style=for-the-badge)]()

<br/>

[Features](#-features) · [Installation](#-installation) · [Usage](#-usage) · [Roadmap](#-roadmap) · [Contributing](#-contributing)

</div>

---

## Overview

**tuipdf** is a terminal-native PDF toolkit built entirely in Rust. Inspired by the convenience of online document tools, it brings essential PDF operations — compression, merging, splitting, and conversion — directly into your terminal through a rich, keyboard-driven TUI experience.

**No uploads. No cloud. No tracking.** Your files never leave your machine.

Built for developers, power users, and open-source contributors who believe productivity tools should be **fast**, **local**, and **transparent**.

---

## ✨ Features

| Feature | Description |
|---------|-------------|
| 📦 **Compress** | Reduce PDF file size without sacrificing quality |
| 🔗 **Merge** | Combine multiple PDFs into a single document |
| ✂️ **Split** | Extract specific pages or ranges from any PDF |
| 🔄 **Convert** | Convert PDFs to and from other formats |
| 🖥️ **Interactive TUI** | Keyboard-driven, distraction-free terminal interface |
| 🔒 **Privacy-first** | 100% local processing — nothing leaves your machine |
| ⚡ **Blazing fast** | Rust-native performance with zero bloat |
| 🌐 **Offline-ready** | No internet connection required, ever |

---

## 📦 Installation

### Prerequisites

- Rust `1.75+` — [Install via rustup](https://rustup.rs/)

### From Source

```bash
git clone https://github.com/KnightShadows/tuipdf.git
cd tuipdf
cargo build --release
./target/release/tuipdf
```

### Using Cargo

```bash
cargo install tuipdf
```

### Verify Installation

```bash
tuipdf --version
```

---

## 🚀 Usage

Launch the interactive TUI:

```bash
tuipdf
```

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `↑` / `↓` | Navigate menu |
| `Enter` | Select option |
| `Tab` | Switch panels |
| `q` | Quit |
| `?` | Show help |
| `Esc` | Go back |

### CLI Flags

```bash
tuipdf --help          # Show all options
tuipdf --version       # Show version info
tuipdf open file.pdf   # Open a PDF directly
```

---

## 🆚 tuipdf vs Online Tools

| | tuipdf | Online Tools |
|---|:---:|:---:|
| 🔒 Privacy | ✅ Files stay local | ❌ Uploaded to servers |
| ⚡ Speed | ✅ Instant, native | ❌ Depends on internet |
| 🌐 Offline Use | ✅ Always works | ❌ Requires connection |
| 💸 Cost | ✅ Free forever | ❌ Often paywalled |
| 🖥️ Workflow | ✅ Stay in terminal | ❌ Switch to browser |
| 🛡️ Security | ✅ Fully auditable | ❌ Closed source |

---

## 🛠️ Tech Stack

| Crate | Purpose |
|-------|---------|
| [ratatui](https://github.com/ratatui-org/ratatui) | TUI framework |
| [crossterm](https://github.com/crossterm-rs/crossterm) | Cross-platform terminal backend |
| [pdfium-render](https://github.com/ajrcarey/pdfium-render) | PDF rendering & processing |
| [clap](https://github.com/clap-rs/clap) | CLI argument parsing |
| [tokio](https://tokio.rs/) | Async runtime |

---

## 🗺️ Roadmap

- [x] Project setup & repository structure
- [x] Interactive TUI welcome screen
- [ ] PDF viewer (read & navigate PDFs)
- [ ] PDF compression
- [ ] PDF merging
- [ ] PDF splitting
- [ ] PDF conversion
- [ ] File picker integration
- [ ] Configuration file support (`~/.config/tuipdf/config.toml`)
- [ ] Cross-platform builds (Linux, macOS, Windows)
- [ ] Homebrew & AUR packages

---

## 🤝 Contributing

Contributions are what make open source great. All contributions are welcome — bug fixes, features, docs, or suggestions.

```bash
# 1. Fork the repository
# 2. Clone your fork
git clone https://github.com/YOUR_USERNAME/tuipdf.git

# 3. Create a feature branch
git checkout -b feature/your-feature-name

# 4. Make your changes and commit
git commit -m "feat: add your feature"

# 5. Push and open a Pull Request
git push origin feature/your-feature-name
```

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

## 📄 License

Copyright 2026 KnightShadows Organization & Aditya Anand

Licensed under the [Mozilla Public License 2.0](LICENSE).

---

## 🙏 Acknowledgements

- [ratatui](https://github.com/ratatui-org/ratatui) — for the excellent TUI framework
- The Rust community — for the incredible ecosystem
- All contributors and users of tuipdf

---

<div align="center">

**Made with 🦀 by [Aditya Anand](https://github.com/adityaanand05) under [KnightShadows](https://github.com/KnightShadows)**

*Fast. Local. Open. — tuipdf*

</div>
