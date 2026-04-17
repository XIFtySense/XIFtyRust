use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;

fn main() {
    let repo_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest dir missing"));
    let library_name = library_name();
    let runtime_dir = resolve_runtime_dir(&repo_root)
        .or_else(|| build_from_source_override(&repo_root))
        .unwrap_or_else(|| {
            panic!(
                "no XIFty runtime available; set XIFTY_RUNTIME_DIR or XIFTY_CORE_DIR, or run scripts/prepare-runtime.sh"
            )
        });

    let manifest_path = runtime_dir.join("manifest.json");
    if manifest_path.exists() {
        validate_manifest(&manifest_path, &library_name);
        println!("cargo:rerun-if-changed={}", manifest_path.display());
    }

    let lib_dir = runtime_dir.join("lib");
    let library_path = lib_dir.join(&library_name);
    assert!(
        library_path.exists(),
        "missing xifty ffi runtime library at {}",
        library_path.display()
    );

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib=xifty_ffi");
    println!("cargo:rerun-if-env-changed=XIFTY_RUNTIME_DIR");
    println!("cargo:rerun-if-env-changed=XIFTY_RUNTIME_CACHE_DIR");
    println!("cargo:rerun-if-env-changed=XIFTY_RUNTIME_VERSION");
    println!("cargo:rerun-if-env-changed=XIFTY_CORE_DIR");
    println!("cargo:rerun-if-env-changed=CARGO_TARGET_DIR");
    println!("cargo:rerun-if-changed=runtime-version.txt");

    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir.display());
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir.display());
}

fn resolve_runtime_dir(repo_root: &Path) -> Option<PathBuf> {
    let bundled_runtime = repo_root.join("runtime");
    if bundled_runtime.join("manifest.json").exists() {
        return Some(bundled_runtime);
    }

    if let Some(runtime_override) = env::var_os("XIFTY_RUNTIME_DIR") {
        let runtime_dir = PathBuf::from(runtime_override);
        if runtime_dir.join("manifest.json").exists() {
            return Some(runtime_dir);
        }
        panic!(
            "XIFTY_RUNTIME_DIR is set but manifest.json is missing at {}",
            runtime_dir.display()
        );
    }

    let runtime_version = runtime_version(repo_root);
    let cache_root = env::var_os("XIFTY_RUNTIME_CACHE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo_root.join(".xifty-runtime"));
    let runtime_dir = cache_root.join(format!(
        "xifty-runtime-{}-v{}",
        runtime_target(),
        runtime_version
    ));
    if runtime_dir.join("manifest.json").exists() {
        return Some(runtime_dir);
    }

    None
}

fn build_from_source_override(repo_root: &Path) -> Option<PathBuf> {
    let core_dir = env::var_os("XIFTY_CORE_DIR").map(PathBuf::from)?;
    let manifest = core_dir.join("Cargo.toml");
    assert!(
        manifest.exists(),
        "XIFTY_CORE_DIR does not look like a XIFty source tree: {}",
        core_dir.display()
    );

    let status = Command::new("cargo")
        .args(["build", "-p", "xifty-ffi", "--manifest-path"])
        .arg(&manifest)
        .args(["--release"])
        .status()
        .expect("failed to build xifty-ffi from XIFTY_CORE_DIR");
    assert!(
        status.success(),
        "cargo build -p xifty-ffi --release failed for {}",
        core_dir.display()
    );

    let target_dir = env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| core_dir.join("target"));
    let runtime_dir = repo_root.join(".xifty-core-build");
    let lib_dir = runtime_dir.join("lib");
    fs::create_dir_all(&lib_dir).expect("failed to create source override runtime dir");
    fs::copy(
        target_dir.join("release").join(library_name()),
        lib_dir.join(library_name()),
    )
    .expect("failed to stage source override runtime library");

    Some(runtime_dir)
}

fn validate_manifest(manifest_path: &Path, expected_library: &str) {
    let manifest: Value = serde_json::from_str(
        &fs::read_to_string(manifest_path).expect("failed to read runtime manifest"),
    )
    .expect("invalid runtime manifest json");

    let library_file = manifest["library_file"]
        .as_str()
        .expect("runtime manifest missing library_file");
    assert_eq!(
        library_file, expected_library,
        "runtime manifest library_file did not match host library name"
    );
}

fn runtime_version(repo_root: &Path) -> String {
    if let Ok(override_version) = env::var("XIFTY_RUNTIME_VERSION") {
        return override_version;
    }

    fs::read_to_string(repo_root.join("runtime-version.txt"))
        .expect("failed to read runtime-version.txt")
        .trim()
        .to_string()
}

fn runtime_target() -> &'static str {
    match (env::consts::OS, env::consts::ARCH) {
        ("macos", "aarch64") => "macos-arm64",
        ("linux", "x86_64") => "linux-x64",
        (os, arch) => panic!("unsupported runtime host: {os}/{arch}"),
    }
}

fn library_name() -> &'static str {
    match env::consts::OS {
        "macos" => "libxifty_ffi.dylib",
        "linux" => "libxifty_ffi.so",
        os => panic!("unsupported runtime host operating system: {os}"),
    }
}
