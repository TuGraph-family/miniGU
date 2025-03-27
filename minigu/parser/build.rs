fn main() {
    #[cfg(feature = "bench-antlr4")]
    {
        if cfg!(target_os = "macos") {
            println!("cargo:rustc-link-lib=dylib=c++");
        } else if cfg!(target_os = "linux") {
            println!("cargo:rustc-link-lib=dylib=stdc++");
        }
        let dst = cmake::Config::new("benches/antlr4")
            .build_target("gql_antlr4_bundled")
            .build();
        println!("cargo:rustc-link-search=native={}/build", dst.display());
        println!("cargo:rustc-link-lib=static=gql_antlr4_bundled");
        println!("cargo:rerun-if-changed=benches/antlr4");
    }
}
