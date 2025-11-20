// build.rs
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out_dir.display());

    // Put the linker script somewhere the linker can find it.
    fs::write(out_dir.join("memory.x"), include_bytes!("memory.x")).unwrap();
    println!("cargo::rustc-link-arg=-Tmemory.x");
    println!("cargo:rerun-if-changed=memory.x");

    fs::write(out_dir.join("link.x"), include_bytes!("link.x")).unwrap();
    println!("cargo::rustc-link-arg=-Tlink.x");
    println!("cargo:rerun-if-changed=link.x");

    println!("cargo:rerun-if-changed=build.rs");
}
