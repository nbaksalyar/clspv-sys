use cmake::Config;
use std::env;
use std::process::Command;

fn main() {
    if env::var("RUSTC_WRAPPER").unwrap_or(String::default()) == "rust-analyzer" {
        // Do not build anything if it was requested by rust-analyzer.
        return;
    }

    let build_from_source = env::var("CARGO_FEATURE_BUILD_FROM_SOURCE").is_ok();

    // Check if pre-built clspv libs are used.
    let mut search_dir = if let Ok(lib_dir) = env::var("CLSPV_LIB_DIR") {
        println!("cargo:warning=clspv-sys: searching native clspv libraries in '{lib_dir}'");
        Some(lib_dir)
    } else {
        None
    };

    // Try to build with the static library if a path was explicit set.
    if let Some(search_dir) = search_dir {
        println!("cargo:rustc-link-search=native={search_dir}");
        println!("cargo:rustc-link-lib=static=clspv_combined");
        println!("cargo:rustc-link-lib=static=clspv_ffi");
        emit_std_cpp_link();
        return;
    }

    if build_from_source {
        fetch_clspv_dependencies();
        build_clspv();
        emit_std_cpp_link();
    } else {
        println!("cargo:warning=clspv-sys: clspv libraries not found - either use the build-from-source feature or provide a path to pre-built binary.");
    }
}

fn build_clspv() {
    println!("cargo:warning=clspv-sys: building clspv from source. it may take a while");
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();

    if target_env == "msvc" {
        // TODO: support GCC builds on Windows.
        build_clspv_msvc();
    } else {
        build_clspv_unix();
    }
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

fn build_clspv_msvc() {
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
        // Windows-specific configuration - set max. path and disable warnings for VS2019.
        .define("CMAKE_OBJECT_PATH_MAX", "512")
        .define("CMAKE_C_FLAGS", "/Wv:18")
        .define("CMAKE_CXX_FLAGS", "/Wv:18")
        .generator("Ninja")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=clspv_combined");
    println!("cargo:rustc-link-lib=static=clspv_ffi");
}

fn build_clspv_unix() {
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
        .generator("Ninja")
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
