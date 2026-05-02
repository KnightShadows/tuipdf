// tuipdf
// ------
// A beautifully crafted, terminal-native PDF tool built in Rust.
// It aims to make compressing PDF files as fast, efficient and flexible
// as possible directly from your terminal.
//
// Authors: KnightShadows Team and individual contributors (see CONTRIBUTORS file)
//          Aditya Anand <aditya19study@gmail.com> (c) 2025
// Website: https://github.com/KnightShadows/tuipdf
// License: MPL-2.0 (see LICENSE file)

fn main() {
    println!("cargo:rerun-if-changed=c/compress.c");
    println!("cargo:rerun-if-changed=c/compress.h");

    let mut build = cc::Build::new();
    build.file("c/compress.c");

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    match target_os.as_str() {
        "linux" => {
            build.include("/usr/include/mupdf");
            println!("cargo:rustc-link-lib=mupdf");
            println!("cargo:rustc-link-lib=mupdf-third");
            println!("cargo:rustc-link-lib=m");
            println!("cargo:rustc-link-lib=pthread");
        }
        "macos" => {
            build.include("/usr/local/include");
            build.include("/opt/homebrew/include");
            println!("cargo:rustc-link-search=native=/usr/local/lib");
            println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
            println!("cargo:rustc-link-lib=mupdf");
            println!("cargo:rustc-link-lib=mupdf-third");
            println!("cargo:rustc-link-lib=pthread");
        }
        "windows" => {
            build.include("C:/msys64/mingw64/include/mupdf");
            build.include("C:/msys64/ucrt64/include/mupdf");
            
            build.include("C:/vcpkg/installed/x64-windows/include");
            
            println!("cargo:rustc-link-search=native=C:/msys64/mingw64/lib");
            println!("cargo:rustc-link-search=native=C:/msys64/ucrt64/lib");
            println!("cargo:rustc-link-search=native=C:/vcpkg/installed/x64-windows/lib");
            
            println!("cargo:rustc-link-arg=-lmupdf");
            println!("cargo:rustc-link-arg=-lfreetype");
            println!("cargo:rustc-link-arg=-lharfbuzz");
            println!("cargo:rustc-link-arg=-lopenjp2");
            println!("cargo:rustc-link-arg=-ljbig2dec");
            println!("cargo:rustc-link-arg=-ljpeg");
            println!("cargo:rustc-link-arg=-lz");
            println!("cargo:rustc-link-arg=-lgumbo");
            println!("cargo:rustc-link-arg=-llcms2");
            println!("cargo:rustc-link-arg=-lWs2_32");
            println!("cargo:rustc-link-arg=-lAdvapi32");
            println!("cargo:rustc-link-arg=-lShell32");
            println!("cargo:rustc-link-arg=-lUser32");
            println!("cargo:rustc-link-arg=-lGdi32");
            println!("cargo:rustc-link-arg=-lComdlg32");
        }
        _ => {}
    }

    build.compile("compress");
}