use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let core_dir = env::var_os("XIFTY_CORE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(".xifty-core"));

    let core_repo = env::var("XIFTY_CORE_REPO")
        .unwrap_or_else(|_| "https://github.com/XIFtySense/XIFty.git".to_string());
    let core_ref = env::var("XIFTY_CORE_REF").unwrap_or_else(|_| "main".to_string());

    if !core_dir.join(".git").exists() {
        if let Some(parent) = core_dir.parent() {
            fs::create_dir_all(parent).expect("failed to create core cache parent");
        }
        let status = Command::new("git")
            .args(["clone", "--depth", "1", "--branch", &core_ref, &core_repo])
            .arg(&core_dir)
            .status()
            .expect("failed to clone xifty core");
        assert!(status.success(), "git clone xifty core failed");
    } else {
        let status = Command::new("git")
            .args(["-C"])
            .arg(&core_dir)
            .args(["fetch", "--depth", "1", "origin", &core_ref])
            .status()
            .expect("failed to fetch xifty core");
        assert!(status.success(), "git fetch xifty core failed");
        let status = Command::new("git")
            .args(["-C"])
            .arg(&core_dir)
            .args(["checkout", "--force", "FETCH_HEAD"])
            .status()
            .expect("failed to checkout xifty core");
        assert!(status.success(), "git checkout xifty core failed");
    }

    let manifest = core_dir.join("Cargo.toml");
    let status = std::process::Command::new("cargo")
        .args(["build", "-p", "xifty-ffi", "--manifest-path"])
        .arg(&manifest)
        .status()
        .expect("failed to build xifty-ffi");
    assert!(status.success(), "cargo build -p xifty-ffi failed");

    println!(
        "cargo:rustc-link-search=native={}",
        core_dir.join("target/debug").display()
    );
    println!("cargo:rustc-link-lib=static=xifty_ffi");
    println!("cargo:rerun-if-env-changed=XIFTY_CORE_DIR");
    println!("cargo:rerun-if-env-changed=XIFTY_CORE_REPO");
    println!("cargo:rerun-if-env-changed=XIFTY_CORE_REF");
}
