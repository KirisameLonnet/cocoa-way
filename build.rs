fn main() {
    #[cfg(target_os = "macos")]
    {
        let homebrew_paths = [
            "/opt/homebrew/lib",        
            "/usr/local/lib",           
        ];
        for path in homebrew_paths {
            if std::path::Path::new(path).exists() {
                println!("cargo:rustc-link-search=native={}", path);
            }
        }
        let homebrew_include_paths = [
            "/opt/homebrew/include",    
            "/usr/local/include",       
        ];
        for path in homebrew_include_paths {
            if std::path::Path::new(path).exists() {
                println!("cargo:include={}", path);
            }
        }

        // Check for required dependencies
        check_dependency("xkbcommon", &[
            "/opt/homebrew/lib/libxkbcommon.dylib",
            "/usr/local/lib/libxkbcommon.dylib",
        ]);
        check_dependency("pixman", &[
            "/opt/homebrew/lib/libpixman-1.dylib",
            "/usr/local/lib/libpixman-1.dylib",
        ]);
    }
}

#[cfg(target_os = "macos")]
fn check_dependency(name: &str, paths: &[&str]) {
    let found = paths.iter().any(|p| std::path::Path::new(p).exists());
    if !found {
        eprintln!();
        eprintln!("╔══════════════════════════════════════════════════════════════╗");
        eprintln!("║  ERROR: Missing dependency '{}'", name);
        eprintln!("╠══════════════════════════════════════════════════════════════╣");
        eprintln!("║  Please install via Homebrew:                                ║");
        eprintln!("║                                                              ║");
        eprintln!("║    brew install lib{}                                   ║", name);
        eprintln!("║                                                              ║");
        eprintln!("║  Then rebuild:                                               ║");
        eprintln!("║    cargo clean && cargo build --release                      ║");
        eprintln!("╚══════════════════════════════════════════════════════════════╝");
        eprintln!();
        std::process::exit(1);
    }
}