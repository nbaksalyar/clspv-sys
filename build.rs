use cmake::Config;
use std::env;
use std::process::Command;

fn main() {
    if env::var("RUSTC_WRAPPER").unwrap_or(String::default()) == "rust-analyzer" {
        // Do not build anything if it was requested by rust-analyzer.
        return;
    }

    fetch_clspv_dependencies();
    build_clspv();
    emit_std_cpp_link();
}

// Runs the python script to fetch dependencies required for building clspv.
fn fetch_clspv_dependencies() {
    // python3 utils/fetch_sources.py
    Command::new("python3")
        .current_dir("clspv-ffi/clspv")
        .args(["utils/fetch_sources.py", "--shallow"])
        .status()
        .expect("failed to fetch clspv dependencies");
}

fn build_clspv() {
    let dst = Config::new("clspv-ffi")
        // CMake options
        .define("CMAKE_POSITION_INDEPENDENT_CODE", "ON")
        .define("CMAKE_INSTALL_LIBDIR", "lib")
        // LLVM build options - disable unnecessary dependencies.
        .define("LLVM_ENABLE_TERMINFO", "OFF")
        .define("LLVM_ENABLE_ZSTD", "OFF")
        .define("LLVM_ENABLE_ZLIB", "OFF")
        // Always build in the release mode - LLVM requires inordinate amounts of memory
        // if it is built in the debug mode.
        .profile("Release")
        // It takes more than a minute for CMake to configure clspv, so let's prevent
        // it from excessive reconfigurations.
        .always_configure(false)
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=clspv_combined");
    println!("cargo:rustc-link-lib=static=clspv_ffi");
}

// taken from `shaderc-rs`, licensed under the Apache 2.0 license.
fn emit_std_cpp_link() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();

    match (target_os.as_str(), target_env.as_str()) {
        ("linux", _) | ("windows", "gnu") => println!("cargo:rustc-link-lib=dylib=stdc++"),
        ("macos", _) => println!("cargo:rustc-link-lib=dylib=c++"),
        _ => {}
    }
}
