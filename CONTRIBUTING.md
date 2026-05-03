# Contributing to tuipdf

First off, thank you for considering contributing to `tuipdf`! It's people like you that make open source such a great community to learn, inspire, and create.

This document serves as a guide for anyone looking to contribute to the project, whether it's through code, documentation, or issue reporting.

## 🏗️ Project Architecture

Before diving in, it's helpful to understand how `tuipdf` is structured:

1.  **Frontend (Rust & Ratatui):** The entire user interface, state management, and input handling is written in pure Rust using `ratatui` and `crossterm`.
2.  **Backend (Pure Rust Pipeline):** The PDF compression pipeline is implemented entirely in Rust using `lopdf` for PDF parsing, `mozjpeg` for JPEG re-encoding, `flate2` for DEFLATE compression, and `rayon` for parallel processing. The pipeline lives in `src/pipeline/`.

## 🛠️ Development Setup

To build and run `tuipdf` locally, you need the Rust toolchain and CMake (for building the bundled MozJPEG C library).

### Windows
Install CMake:
```powershell
winget install Kitware.CMake
```

### Linux (Debian/Ubuntu)
```bash
sudo apt install build-essential cmake nasm
```

### macOS
```bash
brew install cmake nasm
```

Once dependencies are installed, you can build the project:
```bash
cargo build
cargo run --
```

## 📝 Coding Guidelines

To maintain a clean and professional codebase, please adhere to the following rules:

1.  **File Headers Only:** We keep our code extremely clean. **Do not use inline comments or docstrings (`//` or `///`) in the codebase unless absolutely necessary for complex logic.** Every file *must* start with the standard `tuipdf` attribution block (you can copy it from `src/main.rs`).
2.  **Rust Conventions:** Run `cargo fmt` and `cargo clippy` before submitting any code. Ensure there are zero warnings.
3.  **UI Aesthetics:** `tuipdf` prides itself on being a *premium* terminal app. If you are adding UI components, ensure they use the established color palette (found in `src/ui.rs`), smooth transitions, and proper alignment.

## 🗺️ What to Work On

Check out our **Roadmap** in the `README.md`! We are actively looking for help building out Phase 2 features:
*   PDF to Word / Word to PDF
*   PDF to Image / Image to PDF
*   Merge / Split PDFs
*   Security Features (Password Protection)

Feel free to open an issue stating which feature you'd like to tackle so we can assign it to you and avoid duplicate work.

## 🚀 Submitting a Pull Request

1.  Fork the repository under your own GitHub account.
2.  Clone the repository to your local machine.
3.  Create a new branch for your feature or bug fix: `git checkout -b feature/my-awesome-feature`
4.  Make your changes, ensuring you follow the coding guidelines.
5.  Test your changes thoroughly.
6.  Commit your changes with a clear and descriptive commit message.
7.  Push your branch to your fork: `git push origin feature/my-awesome-feature`
8.  Open a Pull Request against the `main` branch of the `KnightShadows/tuipdf` repository.

## 🐛 Reporting Bugs

If you find a bug, please open an issue! Include:
- Your OS and terminal emulator.
- The command you ran.
- The expected behavior vs. the actual behavior.
- Any crash logs or screenshots.

Thank you for contributing to `tuipdf`!
