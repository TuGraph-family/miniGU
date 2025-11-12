use std::env;

fn main() {
    // Use PyO3's helper function to set the correct linker arguments for extension modules
    #[cfg(target_os = "macos")]
    pyo3_build_config::add_extension_module_link_args();

    // Special handling for macOS
    if env::var("CARGO_CFG_TARGET_OS").is_ok_and(|os| os == "macos") {
        // Check if we're cross-compiling to macOS ARM64
        let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
        let host_arch = env::var("HOST").unwrap_or_default();
        let _is_cross_compiling = target_arch == "aarch64" && !host_arch.contains("aarch64");
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
        } else {
            // For macOS, we rely on pyo3's automatic linking
            // Do not manually link to Python framework as it may cause conflicts
            // The pyo3/extension-module feature handles this properly
        }
        // Additional macOS-specific linker arguments to avoid issues
        println!("cargo:rustc-link-arg=-undefined");
        println!("cargo:rustc-link-arg=dynamic_lookup");
    }

    // Enable PyO3 auto-initialize feature
    println!("cargo:rustc-cfg=pyo3_auto_initialize");
}
