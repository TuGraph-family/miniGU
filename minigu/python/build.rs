fn main() {
    // Use PyO3's helper function to set correct link arguments for extension modules
    #[cfg(target_os = "macos")]
    pyo3_build_config::add_extension_module_link_args();
}
