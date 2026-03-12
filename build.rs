use std::{env, fs, path::{Path, PathBuf}, process::Command};

fn main() {
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let profile = env::var("PROFILE").unwrap_or_default();

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // OUT_DIR = target/<triple>/<profile>/build/<pkg>-<hash>/out
    let build_dir = out_dir.parent().unwrap().parent().unwrap(); // .../build/
    let exe_dir = build_dir.parent().unwrap(); // .../debug/ or .../release/

    match os.as_str() {
        "linux" => setup_linux(build_dir, exe_dir),
        "windows" if profile == "debug" => setup_windows_debug(build_dir, exe_dir),
        _ => {}
    }
}

/// Copies libsteam_api.so next to the binary and sets rpath=$ORIGIN so it's
/// found when the binary is run directly (not just via `cargo run`).
fn setup_linux(build_dir: &Path, exe_dir: &Path) {
    if let Some(src) = find_in_build_output(build_dir, "steamworks-sys-", "libsteam_api.so") {
        fs::copy(&src, exe_dir.join("libsteam_api.so")).expect("Failed to copy libsteam_api.so");
        println!("cargo:rerun-if-changed={}", src.display());
    } else {
        println!("cargo:warning=libsteam_api.so not found in steamworks-sys build output");
    }
    // $ORIGIN tells the dynamic linker to search the binary's own directory.
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
}

/// Copies all DLLs needed for dynamic linking (fast_compile) to the exe directory:
///   - steam_api64.dll  from steamworks-sys bundled SDK (matches pre-built FFI bindings)
///   - bevy_dylib-<hash>.dll  from deps/ (compiled before this build script runs)
///   - std-<hash>.dll  from the Rust toolchain's Windows target libdir
///   - libgcc_s_seh-1.dll / libwinpthread-1.dll  from the MinGW installation
fn setup_windows_debug(build_dir: &Path, exe_dir: &Path) {
    // steam_api64.dll
    if let Some(src) = find_in_build_output(build_dir, "steamworks-sys-", "steam_api64.dll") {
        fs::copy(&src, exe_dir.join("steam_api64.dll")).expect("Failed to copy steam_api64.dll");
        println!("cargo:rerun-if-changed={}", src.display());
    } else {
        println!("cargo:warning=steam_api64.dll not found in steamworks-sys build output");
    }

    // bevy_dylib-<hash>.dll and any other dylib deps
    let deps_dir = exe_dir.join("deps");
    if let Ok(entries) = fs::read_dir(&deps_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            if name.to_string_lossy().ends_with(".dll") {
                fs::copy(entry.path(), exe_dir.join(&name)).ok();
            }
        }
    }

    // std-<hash>.dll from the active Rust toolchain
    let target = env::var("TARGET").unwrap();
    let rustc = env::var("RUSTC").unwrap_or_else(|_| "rustc".into());
    if let Ok(out) = Command::new(rustc)
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

    // MinGW runtime DLLs — search /usr/lib/gcc/x86_64-w64-mingw32/<version>/,
    // sorted descending so the posix variant is preferred over win32.
    let mingw_base = PathBuf::from("/usr/lib/gcc/x86_64-w64-mingw32");
    if let Ok(versions) = fs::read_dir(&mingw_base) {
        let mut dirs: Vec<_> = versions.flatten().collect();
        dirs.sort_by_key(|e| e.file_name());
        dirs.reverse();
        for dll_name in &["libgcc_s_seh-1.dll", "libwinpthread-1.dll"] {
            // Also try /usr/x86_64-w64-mingw32/lib/ as a fallback
            let candidates = dirs
                .iter()
                .map(|d| d.path().join(dll_name))
                .chain([PathBuf::from("/usr/x86_64-w64-mingw32/lib").join(dll_name)]);
            for src in candidates {
                if src.exists() {
                    fs::copy(&src, exe_dir.join(dll_name)).ok();
                    println!("cargo:rerun-if-changed={}", src.display());
                    break;
                }
            }
        }
    }
}

/// Finds a file produced by a cargo dependency's build script.
/// Searches `build_dir` for a subdirectory starting with `prefix` and returns
/// the path `<subdir>/out/<filename>` if it exists.
fn find_in_build_output(build_dir: &Path, prefix: &str, filename: &str) -> Option<PathBuf> {
    fs::read_dir(build_dir).ok()?.flatten().find_map(|entry| {
        if entry.file_name().to_string_lossy().starts_with(prefix) {
            let path = entry.path().join("out").join(filename);
            path.exists().then_some(path)
        } else {
            None
        }
    })
}