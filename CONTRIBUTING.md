<div align="center">

# Contributing to TUIPDF

**A comprehensive guide for engineers and designers contributing to the `tuipdf` ecosystem.**

[Architecture](#️-architecture--internals) • [Environment Setup](#-development-environment) • [Coding Standards](#-engineering-standards) • [Pull Requests](#-pull-request-lifecycle)

</div>

---

Thank you for your interest in contributing to **tuipdf**. We are building a production-grade, terminal-native PDF processing suite, and we rely on the open-source community to help drive innovation, stability, and aesthetic excellence.

## 🏗️ Architecture & Internals

Before contributing, it is crucial to understand the structural paradigm of `tuipdf`. The application is strictly bifurcated into frontend UI and backend processing.

| Subsystem | Stack | Responsibility |
| :--- | :--- | :--- |
| **Frontend UI** | `ratatui`, `crossterm` | Terminal rendering, layout calculation, keyboard event polling, and state management. Lives primarily in `src/ui.rs` and `src/app.rs`. |
| **Processing Pipeline** | `lopdf`, `mozjpeg`, `rayon` | The core backend logic. Handles PDF parsing, color-space detection, parallel image extraction, SIMD-accelerated re-encoding, and dictionary mutation. Located in `src/pipeline/`. |

---

## 🛠️ Development Environment

To compile and execute the project locally, your environment must meet the requirements outlined in our **[Installation Guide](INSTALL.md#️-system-requirements)** (Rust, CMake, NASM).

### Quick Start
```bash
# 1. Clone your fork
git clone https://github.com/YOUR_USERNAME/tuipdf.git
cd tuipdf

# 2. Build the project
cargo build

# 3. Execute in development mode
cargo run --
```

---

## 📐 Engineering Standards

We hold the `tuipdf` codebase to an exceptionally high standard. To ensure your Pull Request is accepted rapidly, please adhere strictly to the following guidelines:

### 1. Zero-Warning Policy
Your code must compile with absolutely zero warnings. Prior to committing, you must run:
```bash
cargo fmt
cargo clippy -- -D warnings
```

### 2. Documentation & Commenting
We practice "Clean Code" principles.
- **No Inline Comments**: Do not use inline comments (`//`) to explain *what* code is doing. The code should be self-documenting through pristine variable naming and structural clarity.
- **Architectural Comments**: Docstrings (`///`) are permitted *only* for high-level architectural explanations of complex pipelines or algorithms.
- **File Attribution**: Every `.rs` file must begin with the standard `tuipdf` attribution block (refer to `src/main.rs`).

### 3. UI/UX Excellence
`tuipdf` is a *premium* terminal application.
- If you are implementing new UI components, they must conform to the established color palette defined in `src/ui.rs`.
- Ensure elements are properly padded, responsive to terminal resizing, and incorporate smooth visual transitions where applicable.

---

## 🗺️ High-Priority Initiatives

We are currently tracking towards **Phase 2** of our roadmap. High-impact areas for contribution include:

- **Format Conversion**: `PDF ↔ Word`, `PDF ↔ Image (JPEG/PNG)`.
- **Structural Mutation**: PDF Splitting, Merging, and Page Extraction.
- **Cryptography**: Implementing PDF Password Protection and Decryption.

> *Tip: Please open a dedicated Issue outlining your proposed implementation before beginning work on a major feature. This prevents duplicated effort.*

---

## 🚀 Pull Request Lifecycle

1. **Fork & Branch**: Fork the repository and create a feature branch (`git checkout -b feat/your-feature-name` or `fix/issue-description`).
2. **Implement**: Write your code adhering to the [Engineering Standards](#-engineering-standards).
3. **Verify**: Ensure the test suite passes and `cargo clippy` is clean.
4. **Commit**: Write semantic, descriptive commit messages.
5. **Submit**: Open a Pull Request against the `main` branch of the `KnightShadows/tuipdf` repository. Provide a clear summary of your changes, the rationale, and any visual screenshots if UI modifications were made.

---

## 🐛 Bug Triage

When filing a bug report, precision is key. Please provide:
1. **Environment Specifications**: OS, Architecture, and Terminal Emulator.
2. **Reproduction Steps**: The exact CLI arguments or keystrokes utilized.
3. **Artifacts**: Stack traces (`RUST_BACKTRACE=1`), crash logs, or visual screenshots of the graphical anomaly.

---

<div align="center">
<i>We appreciate your commitment to building high-quality open source software.</i>
</div>
