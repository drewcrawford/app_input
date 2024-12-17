use std::env;
use swift_rs::SwiftLinker;

fn main() {
    // Detect the target OS explicitly using environment variables
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    if target_os == "macos" {
        println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=15.0");

        SwiftLinker::new("15.0")
            .with_package("SwiftRawInput", "SwiftRawInput")
            .link();
    }

    // Other build steps
}

