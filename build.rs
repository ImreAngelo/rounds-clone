use std::env;
use std::path::PathBuf;

fn main() {
    // Only copy DLL when steamworks feature is enabled
    if !cfg!(feature = "steamworks") {
        return;
    }

    // // TODO: Only copy for Windows targets
    // if !cfg!(target_os = "windows") {
    //     return;
    // }

    // Get the Steamworks SDK path from environment variable
    let sdk_path = env::var("STEAMWORKS_SDK_PATH")
        .expect("STEAMWORKS_SDK_PATH environment variable not set");

    let dll_src = PathBuf::from(&sdk_path)
        .join("redistributable_bin/win64/steam_api64.dll");

    if !dll_src.exists() {
        panic!(
            "steam_api64.dll not found at: {}. \
             Make sure STEAMWORKS_SDK_PATH is set correctly.",
            dll_src.display()
        );
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let dll_dest = PathBuf::from(&out_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("steam_api64.dll");

    std::fs::copy(&dll_src, &dll_dest).expect("Failed to copy steam_api64.dll");

    println!("cargo:warning=Copied steam_api64.dll to {:?}", dll_dest);
}
