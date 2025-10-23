use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Use PyO3's helper function to set the correct linker arguments for extension modules
    #[cfg(target_os = "macos")]
    pyo3_build_config::add_extension_module_link_args();

    // Special handling for macOS
    if env::var("CARGO_CFG_TARGET_OS").is_ok_and(|os| os == "macos") {
        // Check if we're cross-compiling to macOS ARM64
        let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
        let is_cross_compiling = target_arch == "aarch64" && cfg!(not(target_arch = "aarch64"));
        // Try to find Python framework
        if let Ok(python_lib) = env::var("PYTHON_LIB") {
            // Use the provided library flags
            for flag in python_lib.split_whitespace() {
                if let Some(lib_path) = flag.strip_prefix("-L") {
                    println!("cargo:rustc-link-search=native={}", lib_path);
                } else if let Some(lib_name) = flag.strip_prefix("-l") {
                    println!("cargo:rustc-link-lib={}", lib_name);
                } else if let Some(framework_name) = flag.strip_prefix("-framework ") {
                    println!("cargo:rustc-link-lib=framework={}", framework_name);
                }
            }
        } else if env::var("PYO3_PYTHON").is_ok() && !is_cross_compiling {
            // Fallback to framework linking (only for native builds)
            println!("cargo:rustc-link-lib=framework=Python");
            println!("cargo:rustc-link-search=framework=/opt/homebrew/Frameworks");
            println!("cargo:rustc-link-search=framework=/usr/local/Frameworks");
        } else if is_cross_compiling {
            // For cross-compilation to macOS ARM64, we might need special handling
            println!("cargo:warning=Cross-compiling to macOS ARM64 may require additional configuration");
            println!("cargo:rustc-link-lib=framework=Python");
        } else {
            // Native build on macOS (Intel or Apple Silicon)
            println!("cargo:rustc-link-lib=framework=Python");
            println!("cargo:rustc-link-search=framework=/opt/homebrew/Frameworks");
            println!("cargo:rustc-link-search=framework=/usr/local/Frameworks");
        }
    }

    // Enable PyO3 auto-initialize feature
    println!("cargo:rustc-cfg=pyo3_auto_initialize");

    // === Copy built extension module to Python directory ===
    // Only run copy logic when building this crate directly
    if env::var("CARGO_CFG_TARGET_OS").is_err() {
        return;
    }

    let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "../target".to_string());
    let profile = if cfg!(debug_assertions) { "debug" } else { "release" };

    // Determine expected library name based on platform
    let lib_stem = "minigu_python";
    let src_ext = get_library_extension();
    let dest_ext = if cfg!(target_os = "windows") { "pyd" } else { "so" };

    let src_path = Path::new(&target_dir).join(profile).join(format!("{}{}", lib_stem, src_ext));
    let dest_path = Path::new("minigu_python.").with_extension(dest_ext);

    // Try to copy the file
    if src_path.exists() {
        match fs::copy(&src_path, &dest_path) {
            Ok(_) => println!("Successfully copied Python extension to {}", dest_path.display()),
            Err(e) => eprintln!("Warning: Failed to copy extension module: {}", e),
        }
    } else {
        // Try alternative extensions (e.g., .so on Linux even if not default)
        for ext in [".so", ".dll", ".dylib"].iter() {
            let alt_path = Path::new(&target_dir).join(profile).join(format!("{}{}", lib_stem, ext));
            if alt_path.exists() {
                match fs::copy(&alt_path, &dest_path) {
                    Ok(_) => {
                        println!("Successfully copied Python extension to {}", dest_path.display());
                        break;
                    }
                    Err(e) => eprintln!("Warning: Failed to copy extension module: {}", e),
                }
            }
        }
    }

    // Ensure cargo reruns build script when needed
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=build.rs");
}

// Helper function to get correct extension for current platform
#[cfg(target_os = "windows")]
fn get_library_extension() -> &'static str {
    ".dll"
}

#[cfg(target_os = "macos")]
fn get_library_extension() -> &'static str {
    ".dylib"
}

#[cfg(all(unix, not(target_os = "macos")))]
fn get_library_extension() -> &'static str {
    ".so"
}
