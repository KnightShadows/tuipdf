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

# ── 2. Check Build Environment / Dependencies ───────────────────────────────
$hasGcc = (Get-Command gcc -ErrorAction SilentlyContinue)

if ($hasGcc) {
    Write-Info "Existing GCC environment found. We will attempt to build using your current setup."
    Write-Warn "(Ensure you have MuPDF C-headers installed via your package manager.)"
} else {
    Write-Warn "No GCC environment found. Checking for MSYS2 to set up MuPDF dependencies..."
    if (!(Test-Path C:\msys64\usr\bin\pacman.exe)) {
        Write-Info "MSYS2 not found at C:\msys64. Installing via winget..."
        winget install -e --id MSYS2.MSYS2
        Write-Err "Please restart your terminal and run this script again after MSYS2 is installed."
        exit 1
    }

    Write-Info "MSYS2 found. Installing GCC and MuPDF toolchain via pacman..."
    & C:\msys64\usr\bin\bash.exe -lc "pacman -S --noconfirm mingw-w64-x86_64-gcc mingw-w64-x86_64-mupdf mingw-w64-ucrt-x86_64-mupdf"
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
