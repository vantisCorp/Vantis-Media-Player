//! Build script for Vantis Media Player

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    // Set up build-time configuration
    println!("cargo:rustc-env=VANTIS_VERSION={}", env!("CARGO_PKG_VERSION"));
    println!("cargo:rustc-env=VANTIS_BUILD_TIME={}", chrono::Utc::now().to_rfc3339());
    
    // Enable nightly features
    println!("cargo:rustc-cfg=nightly");
    
    // Detect platform-specific features
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-cfg=windows");
        println!("cargo:rustc-cfg=directx");
    }
    
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-cfg=linux");
        println!("cargo:rustc-cfg=vulkan");
    }
    
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-cfg=macos");
        println!("cargo:rustc-cfg=metal");
    }
}