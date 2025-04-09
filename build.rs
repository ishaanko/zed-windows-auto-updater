use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Tell Cargo to re-run this build script if the manifest changes
    println!("cargo:rerun-if-changed=resources/app.manifest");

    // Copy the manifest file to the output directory
    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_path = Path::new("resources/app.manifest");
    let dest_path = Path::new(&out_dir).join("app.manifest");
    
    fs::copy(manifest_path, dest_path).expect("Failed to copy manifest");
    
    // On Windows, set up resource compilation for the manifest
    #[cfg(target_os = "windows")]
    {
        // Link to required Windows libraries
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=gdi32");
        println!("cargo:rustc-link-lib=shell32");
    }
}