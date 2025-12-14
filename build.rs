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
    }
}