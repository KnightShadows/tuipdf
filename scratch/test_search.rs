fn main() {
    let prog_files = std::env::var("ProgramFiles").unwrap_or_else(|_| "C:\\Program Files".to_string());
    println!("Searching in: {}", prog_files);
    if let Ok(entries) = std::fs::read_dir(&prog_files) {
        for entry in entries.flatten() {
            if let Ok(name) = entry.file_name().into_string() {
                println!("Found folder: {}", name);
                if name.to_lowercase().starts_with("qpdf") {
                    let bin_path = entry.path().join("bin").join("qpdf.exe");
                    println!("Checking bin path: {:?}", bin_path);
                    if bin_path.exists() {
                        println!("SUCCESS: Found qpdf at {:?}", bin_path);
                    }
                }
            }
        }
    }
}
