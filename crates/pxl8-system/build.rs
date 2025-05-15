fn main() {
    println!("cargo:rerun-if-changed=ext/log.c/src/log.h");
    println!("cargo:rerun-if-changed=ext/log.c/src/log.c");

    println!("cargo:rerun-if-changed=ext/microui/src/microui.h");
    println!("cargo:rerun-if-changed=ext/microui/src/microui.c");

    cc::Build::new()
        .define("LOG_USE_COLOR", None)
        .file("./ext/log.c/src/log.c")
        .flag_if_supported("-Wno-unused-function")
        .file("./src/stbi/stb_image.c")
        .compile("ffi");

    println!("cargo:rustc-link-lib=c");

    // TODO: alloc::format macro brings this dependency along for the ride
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=gcc_s");
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=System");
}
