use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Tell Cargo to rerun this script if the settings file changes
    println!("cargo:rerun-if-changed=settings.toml");

    // Get the output directory (target/debug or target/release)
    let out_dir = env::var("OUT_DIR").unwrap();
    println!(" out dir {out_dir}");
    let dest_path = Path::new(&out_dir).join("../../.."); // Go up to reach the executable folder
    println!("dest path {}", dest_path.to_str().unwrap());
    // Copy your file
    fs::copy("settings_term1.toml", dest_path.join("settings_term1.toml"))
        .expect("Failed to copy settings file");
    fs::copy("settings_term2.toml", dest_path.join("settings_term2.toml"))
        .expect("Failed to copy settings file");
}
