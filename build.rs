use swift_rs::SwiftLinker;

fn main() {
    // swift-rs has a minimum of macOS 10.13
    // Ensure the same minimum supported macOS version is specified as in your `Package.swift` file.
    println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=15.0");


    SwiftLinker::new("15.0")
        .with_package("SwiftRawInput", "SwiftRawInput")
        .link();
    // Other build steps
}

