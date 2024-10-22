use std::env;
use std::fs;
use std::path::Path;

fn main() {
    #[cfg(feature = "gen-ffi")]
    gen_ffi();

    println!("cargo:rustc-link-search=lib");
    println!("cargo:rustc-link-lib=dylib={}", "SDL3");

    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");

    copy_dylib();
}

#[cfg(target_os = "windows")]
fn copy_dylib() {
    copy_sdl("lib/SDL3.dll");
}

#[cfg(target_os = "linux")]
fn copy_dylib() {
    copy_sdl("lib/libSDL3.so");
}

fn copy_sdl(sdl_src: &str) {
    let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or("target".to_string());
    let profile = env::var("PROFILE").unwrap();
    let file_name = if cfg!(target_os = "linux") {
        sdl_src.split("/").last().unwrap().to_string() + ".0"
    } else {
        sdl_src.split("/").last().unwrap().to_string()
    };

    let sdl_dest = Path::new(&target_dir)
        .join(profile)
        .join(file_name);

    match fs::copy(sdl_src, sdl_dest) {
        Ok(_) => println!("cargo:rerun-if-changed={}", sdl_src),
        Err(e) => panic!("Failed to copy SDL3 dylib: {}", e),
    }
}

#[cfg(feature = "gen-ffi")]
fn gen_ffi() {
    #[cfg(target_os = "linux")]
    let ffi_src = "src/ffi/linux.rs";

    #[cfg(target_os = "windows")]
    let ffi_src = "src/ffi/windows.rs";

    bindgen::Builder::default()
        .use_core()
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .ctypes_prefix("libc")
        .clang_arg("-Iext/SDL3/include")
        .header("src/ffi.h")
        .layout_tests(false)
        .clang_macro_fallback()
        .clang_macro_fallback_build_dir(env::var("OUT_DIR").unwrap())
        .blocklist_type("FP_NAN")
        .blocklist_type("FP_INFINITE")
        .blocklist_type("FP_ZERO")
        .blocklist_type("FP_SUBNORMAL")
        .blocklist_type("FP_NORMAL")
        .opaque_type("_IMAGE_TLS_DIRECTORY64")
        .derive_debug(true)
        .impl_debug(true)
        .generate()
        .unwrap()
        .write_to_file(ffi_src)
        .unwrap();
}
