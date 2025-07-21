use std::env;
use std::path::PathBuf;
#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    // add unix dependencies below
    // println!("cargo:rustc-flags=-l readline");
let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
println!("cargo:rustc-link-arg=-L.");
println!("cargo:rustc-link-arg=-Wl,-rpath,{}", root.display());
println!("cargo:rustc-link-arg=-lraylib");
}

#[cfg(target_os = "macos")]
fn main() {
    // add macos dependencies below
    // println!("cargo:rustc-flags=-l edit");
}
