use std::{env, fs, path::PathBuf};

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() != "windows" {
        println!("cargo:warning=Not building for windows...");
        return;
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // OUT_DIR = target/<triple>/<profile>/build/<pkg>-<hash>/out
    let build_dir = out_dir.parent().unwrap().parent().unwrap(); // .../build/
    let exe_dir = build_dir.parent().unwrap(); // .../debug/ or .../release/

    // steamworks-sys copies steam_api64.dll from its bundled SDK into its own OUT_DIR.
    // We copy it here to the exe directory so Windows can find it at runtime.
    // Using the crate's bundled DLL ensures it matches the pre-built FFI bindings.
    let dst = exe_dir.join("steam_api64.dll");
    if let Ok(entries) = fs::read_dir(build_dir) {
        for entry in entries.flatten() {
            if entry.file_name().to_string_lossy().starts_with("steamworks-sys-") {
                let src = entry.path().join("out/steam_api64.dll");
                if src.exists() {
                    fs::copy(&src, &dst).expect("Failed to copy steam_api64.dll");
                    println!("cargo:rerun-if-changed={}", src.display());
                    return;
                }
            }
        }
    }

    println!("cargo:warning=steam_api64.dll not found in steamworks-sys build output");
}
