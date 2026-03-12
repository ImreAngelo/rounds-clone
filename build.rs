use std::{env, fs, path::PathBuf, process::Command};

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() != "windows" {
        return;
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // OUT_DIR = target/<triple>/<profile>/build/<pkg>-<hash>/out
    let build_dir = out_dir.parent().unwrap().parent().unwrap(); // .../build/
    let exe_dir = build_dir.parent().unwrap(); // .../debug/ or .../release/

    // steamworks-sys copies steam_api64.dll from its bundled SDK into its own OUT_DIR.
    // Using the crate's bundled DLL ensures it matches the pre-built FFI bindings (v008).
    let mut steam_found = false;
    if let Ok(entries) = fs::read_dir(build_dir) {
        for entry in entries.flatten() {
            if entry.file_name().to_string_lossy().starts_with("steamworks-sys-") {
                let src = entry.path().join("out/steam_api64.dll");
                if src.exists() {
                    fs::copy(&src, exe_dir.join("steam_api64.dll"))
                        .expect("Failed to copy steam_api64.dll");
                    println!("cargo:rerun-if-changed={}", src.display());
                    steam_found = true;
                    break;
                }
            }
        }
    }
    if !steam_found {
        println!("cargo:warning=steam_api64.dll not found in steamworks-sys build output");
    }

    // Copy all DLLs from deps/ to exe dir (bevy_dylib-<hash>.dll and any future dylib deps).
    // Cargo compiles dependencies before running the main crate's build.rs, so these exist.
    let deps_dir = exe_dir.join("deps");
    if let Ok(entries) = fs::read_dir(&deps_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            if name.to_string_lossy().ends_with(".dll") {
                fs::copy(entry.path(), exe_dir.join(&name)).ok();
            }
        }
    }

    // Copy std-<hash>.dll from the Rust toolchain's Windows target libdir.
    // Uses the RUSTC env var (set by cargo) so it always matches the active toolchain.
    let target = env::var("TARGET").unwrap();
    let rustc = env::var("RUSTC").unwrap_or_else(|_| "rustc".into());
    if let Ok(out) = Command::new(&rustc)
        .args(["--print", "target-libdir", "--target", &target])
        .output()
    {
        let libdir = PathBuf::from(String::from_utf8_lossy(&out.stdout).trim());
        if let Ok(entries) = fs::read_dir(&libdir) {
            for entry in entries.flatten() {
                let name = entry.file_name();
                if name.to_string_lossy().ends_with(".dll") {
                    fs::copy(entry.path(), exe_dir.join(&name)).ok();
                    println!("cargo:rerun-if-changed={}", entry.path().display());
                }
            }
        }
    }

    // Copy MinGW runtime DLLs (libgcc_s_seh-1.dll, libwinpthread-1.dll).
    // Search /usr/lib/gcc/x86_64-w64-mingw32/<version>/, preferring posix over win32.
    let mingw_base = PathBuf::from("/usr/lib/gcc/x86_64-w64-mingw32");
    let dll_names = ["libgcc_s_seh-1.dll", "libwinpthread-1.dll"];
    if let Ok(versions) = fs::read_dir(&mingw_base) {
        let mut dirs: Vec<_> = versions.flatten().collect();
        dirs.sort_by_key(|e| e.file_name());
        dirs.reverse(); // posix sorts after win32 alphabetically, reverse gives posix first
        for dll_name in &dll_names {
            for version_dir in &dirs {
                let src = version_dir.path().join(dll_name);
                if src.exists() {
                    fs::copy(&src, exe_dir.join(dll_name)).ok();
                    println!("cargo:rerun-if-changed={}", src.display());
                    break;
                }
            }
        }
    }
    // Fallback location for libwinpthread-1.dll
    let wp = PathBuf::from("/usr/x86_64-w64-mingw32/lib/libwinpthread-1.dll");
    if wp.exists() && !exe_dir.join("libwinpthread-1.dll").exists() {
        fs::copy(&wp, exe_dir.join("libwinpthread-1.dll")).ok();
    }
}
