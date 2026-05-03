<div align="center">

# TUIPDF — Installation & Deployment Guide

**Comprehensive setup instructions for compiling and deploying `tuipdf` across Windows, macOS, and Linux.**

[System Requirements](#️-system-requirements) • [Automated Install](#-automated-installation-recommended) • [Manual Compilation](#-manual-compilation-from-source) • [Troubleshooting](#-troubleshooting)

</div>

---

## 🖥️ System Requirements

`tuipdf` is engineered in pure Rust, ensuring minimal system dependencies. However, because it incorporates SIMD-accelerated JPEG processing, your build environment requires standard C/C++ build tools.

| Component | Minimum Version | Purpose |
| :--- | :--- | :--- |
| **Rust Toolchain** | `1.70.0+` | Compiling the core application. |
| **CMake** | `3.10+` | Orchestrating C library builds (e.g., `mozjpeg`). |
| **NASM / YASM** | `2.14+` | Compiling high-performance SIMD assembly optimizations. |
| **C Compiler** | Modern | `gcc`, `clang`, or MSVC for building the bundled JPEG library. |

> **Note:** These dependencies are **only required for compiling** `tuipdf`. Once compiled, the resulting binary is completely self-contained.

---

## 🚀 Automated Installation (Recommended)

For immediate deployment, we maintain official installation scripts that automatically resolve dependencies, clone the source, and compile the binary.

### Windows (PowerShell)

The Windows deployment script automatically detects Rust and CMake (installing CMake via `winget` if missing), and provisions the binary into your Cargo path.

```powershell
irm https://raw.githubusercontent.com/KnightShadows/tuipdf/main/install_windows.ps1 | iex
```
*Ensure that your Cargo binary directory (`$HOME\.cargo\bin`) is in your system `%PATH%`.*

### macOS (Homebrew)

The macOS script leverages [Homebrew](https://brew.sh/) to fetch build dependencies prior to compilation.

```bash
git clone https://github.com/KnightShadows/tuipdf.git
cd tuipdf
./install_mac.sh
```

### Linux (Debian / Ubuntu)

The Linux script provisions the `build-essential` toolchain alongside `cmake` and `nasm` via `apt`.

```bash
git clone https://github.com/KnightShadows/tuipdf.git
cd tuipdf
./install.sh
```

---

## 🛠️ Manual Compilation (From Source)

For enterprise environments, custom architectures, or users on non-Debian distributions (Arch, Fedora, etc.), you can manually compile `tuipdf`.

### 1. Provision Build Dependencies

<details>
<summary><b>Debian / Ubuntu</b></summary>

```bash
sudo apt-get update && sudo apt-get install -y build-essential cmake nasm
```
</details>

<details>
<summary><b>Arch Linux</b></summary>

```bash
sudo pacman -S base-devel cmake nasm
```
</details>

<details>
<summary><b>macOS</b></summary>

```bash
brew install cmake nasm
```
</details>

<details>
<summary><b>Windows</b></summary>

1. Install **[CMake](https://cmake.org/download/)** and ensure it is added to your system `PATH`.
2. Install **[NASM](https://www.nasm.us/)** (Highly recommended for maximum SIMD performance).
</details>

### 2. Clone & Compile

Once your environment is provisioned, clone the repository and trigger a release build using Cargo:

```bash
git clone https://github.com/KnightShadows/tuipdf.git
cd tuipdf
cargo install --path .
```

This compiles the project with maximum optimization (`--release`) and places the executable in your `~/.cargo/bin` directory.

---

## ✅ Verification

To verify a successful deployment, run the following command to check the binary version and access the help menu:

```bash
tuipdf --version
tuipdf --help
```

---

## 🗑️ Uninstallation

To cleanly remove `tuipdf` from your system, utilize Cargo's native uninstall command:

```bash
cargo uninstall tuipdf
```

---

## 🆘 Troubleshooting

**1. CMake error during build (mozjpeg-sys)**
The build pipeline cannot locate `cmake`. Verify that CMake is installed and accessible via your system `PATH`. On Windows, a terminal restart is often required after installing CMake.

**2. Missing NASM/YASM warning**
The `mozjpeg` compression engine will fall back to slower, non-SIMD routines if NASM is not found. For optimal performance, install `nasm` via your system's package manager (`apt`, `brew`, `winget`).

**3. "Command not found: tuipdf" post-installation**
Cargo places compiled binaries in `~/.cargo/bin` (or `%USERPROFILE%\.cargo\bin` on Windows). You must append this directory to your system's `PATH` environment variable.
