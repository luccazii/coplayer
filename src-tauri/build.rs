fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos") {
        cc::Build::new()
            .file("src/platform/macos.m")
            .flag("-fobjc-arc")
            .flag("-fblocks")
            .compile("coplayer_platform");
        for framework in ["AppKit", "CoreGraphics", "ScreenCaptureKit"] {
            println!("cargo:rustc-link-lib=framework={framework}");
        }
        println!("cargo:rerun-if-changed=src/platform/macos.m");
    }
    tauri_build::build()
}
