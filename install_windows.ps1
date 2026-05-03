# tuipdf
# ------
# A beautifully crafted, terminal-native PDF tool built in Rust.
# It aims to make compressing PDF files as fast, efficient and flexible
# as possible directly from your terminal.
#
# Authors: KnightShadows Team and individual contributors (see CONTRIBUTORS file)
#          Aditya Anand <aditya19study@gmail.com> (c) 2026
# Website: https://github.com/KnightShadows/tuipdf
# License: MPL-2.0 (see LICENSE file)

$ErrorActionPreference = "Stop"

$REPO = "https://github.com/KnightShadows/tuipdf.git"

function Write-Info($msg) { Write-Host "[tuipdf] $msg" -ForegroundColor Cyan }
function Write-Ok($msg)   { Write-Host "[tuipdf] $msg" -ForegroundColor Green }
function Write-Warn($msg) { Write-Host "[tuipdf] $msg" -ForegroundColor Yellow }
function Write-Err($msg)  { Write-Host "[tuipdf] $msg" -ForegroundColor Red }

# ── 1. Check for Rust ────────────────────────────────────────────────────────
if (!(Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Err "Rust not found. Please install Rust from https://rustup.rs/ and restart your terminal."
    exit 1
} else {
    Write-Info "Rust found: $(rustc --version)"
}

# ── 2. Check for CMake (needed by mozjpeg-sys) ──────────────────────────────
if (!(Get-Command cmake -ErrorAction SilentlyContinue)) {
    Write-Warn "CMake not found. Attempting to install via winget..."
    winget install -e --id Kitware.CMake
    Write-Warn "Please restart your terminal and re-run this script after CMake is installed."
    exit 1
} else {
    Write-Info "CMake found: $(cmake --version | Select-Object -First 1)"
}

# ── 3. Install tuipdf ───────────────────────────────────────────────────────
Write-Info "Installing tuipdf..."
if (Test-Path ".\Cargo.toml") {
    # If running locally in the repo
    cargo install --path . --force
} else {
    # If running via invoke-webrequest
    cargo install --git $REPO --force
}

if (Get-Command tuipdf -ErrorAction SilentlyContinue) {
    Write-Ok "Installation complete! Run 'tuipdf' to start."
    tuipdf --version
} else {
    Write-Err "tuipdf binary not found after install. You may need to add '$HOME\.cargo\bin' to your PATH."
}
