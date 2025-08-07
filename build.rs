use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    // Skip library detection if we're building docs
    if env::var("DOCS_RS").is_ok() {
        return;
    }

    println!("cargo:rerun-if-changed=build.rs");
    
    // Try to find espeak-ng library
    if let Some(lib_path) = find_espeak_library() {
        println!("cargo:rustc-link-search=native={}", lib_path);
        println!("cargo:rustc-link-lib=dylib=espeak-ng");
        
        // Set rpath for macOS
        if cfg!(target_os = "macos") {
            println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_path);
        }
        
        // Set rpath for Linux
        if cfg!(target_os = "linux") {
            println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_path);
        }
    } else {
        eprintln!("Warning: espeak-ng library not found. Please install espeak-ng:");
        eprintln!("  macOS: brew install espeak-ng");
        eprintln!("  Ubuntu/Debian: sudo apt-get install espeak-ng libespeak-ng-dev");
        eprintln!("  Windows: Download from https://github.com/espeak-ng/espeak-ng/releases");
    }
}

fn find_espeak_library() -> Option<String> {
    // Common library paths to check
    let common_paths = vec![
        // macOS Homebrew paths
        "/opt/homebrew/lib",
        "/usr/local/lib",
        "/opt/local/lib",
        
        // Linux paths
        "/usr/lib",
        "/usr/local/lib",
        "/usr/lib/x86_64-linux-gnu",
        "/usr/lib/aarch64-linux-gnu",
        "/lib",
        "/lib64",
        
        // Windows paths (when using MSYS2/MinGW)
        "/mingw64/lib",
        "/mingw32/lib",
        "C:/msys64/mingw64/lib",
        "C:/msys64/mingw32/lib",
    ];

    // Check environment variables first
    if let Ok(lib_path) = env::var("ESPEAK_LIB_DIR") {
        if check_library_exists(&lib_path) {
            return Some(lib_path);
        }
    }

    // Try pkg-config first
    if let Some(path) = try_pkg_config() {
        return Some(path);
    }

    // Check common paths
    for path in &common_paths {
        if check_library_exists(path) {
            return Some(path.to_string());
        }
    }

    // Try to find using system commands
    if let Some(path) = find_with_system_commands() {
        return Some(path);
    }

    None
}

fn check_library_exists(lib_dir: &str) -> bool {
    let lib_path = Path::new(lib_dir);
    if !lib_path.exists() {
        return false;
    }

    // Check for different library file patterns
    let lib_patterns = if cfg!(target_os = "windows") {
        vec!["espeak-ng.dll", "libespeak-ng.dll"]
    } else if cfg!(target_os = "macos") {
        vec!["libespeak-ng.dylib", "libespeak-ng.1.dylib"]
    } else {
        vec!["libespeak-ng.so", "libespeak-ng.so.1"]
    };

    for pattern in lib_patterns {
        let full_path = lib_path.join(pattern);
        if full_path.exists() {
            return true;
        }
    }

    false
}

fn try_pkg_config() -> Option<String> {
    match Command::new("pkg-config")
        .args(&["--libs-only-L", "espeak-ng"])
        .output()
    {
        Ok(output) if output.status.success() => {
            let output_str = String::from_utf8_lossy(&output.stdout);
            // Parse -L/path/to/lib format
            for part in output_str.split_whitespace() {
                if part.starts_with("-L") {
                    let path = &part[2..];
                    if check_library_exists(path) {
                        return Some(path.to_string());
                    }
                }
            }
        }
        _ => {}
    }
    None
}

fn find_with_system_commands() -> Option<String> {
    // Try different approaches based on OS
    if cfg!(target_os = "macos") {
        // Try to find via Homebrew
        if let Ok(output) = Command::new("brew").args(&["--prefix", "espeak-ng"]).output() {
            if output.status.success() {
                let prefix = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let lib_path = format!("{}/lib", prefix);
                if check_library_exists(&lib_path) {
                    return Some(lib_path);
                }
            }
        }
        
        // Try locate command
        if let Ok(output) = Command::new("locate").args(&["libespeak-ng.dylib"]).output() {
            if output.status.success() {
                let paths = String::from_utf8_lossy(&output.stdout);
                for line in paths.lines() {
                    if let Some(parent) = Path::new(line.trim()).parent() {
                        let parent_str = parent.to_string_lossy().to_string();
                        if check_library_exists(&parent_str) {
                            return Some(parent_str);
                        }
                    }
                }
            }
        }
    } else if cfg!(target_os = "linux") {
        // Try ldconfig
        if let Ok(output) = Command::new("ldconfig").args(&["-p"]).output() {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines() {
                    if line.contains("libespeak-ng.so") {
                        if let Some(path_start) = line.rfind(" => ") {
                            let lib_file = &line[path_start + 4..].trim();
                            if let Some(parent) = Path::new(lib_file).parent() {
                                let parent_str = parent.to_string_lossy().to_string();
                                if check_library_exists(&parent_str) {
                                    return Some(parent_str);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    None
}