use std::env;

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
            // This is a simplified approach - in practice, you'd need to specify
            // the correct paths to the macOS SDK and Python libraries
            println!(
                "cargo:warning=Cross-compiling to macOS ARM64 may require additional configuration"
            );
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
}
