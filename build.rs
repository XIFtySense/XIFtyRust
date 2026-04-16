use std::env;
use std::path::PathBuf;

fn main() {
    let core_dir = env::var_os("XIFTY_CORE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("../XIFty"));

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
}

