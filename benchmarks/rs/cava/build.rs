use std::env;
use std::path::PathBuf;
#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    // add unix dependencies below
    // println!("cargo:rustc-flags=-l readline");
let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
println!("cargo:rustc-link-arg=-L.");
println!("cargo:rustc-link-arg=-Wl,-rpath,{}", root.display());
println!("cargo:rustc-link-arg=-liniparser");
println!("cargo:rustc-link-arg=-ltinfo");
println!("cargo:rustc-link-arg=-lncursesw");
println!("cargo:rustc-link-arg=-lfftw3");
println!("cargo:rustc-link-arg=-lportaudio");
println!("cargo:rustc-link-arg=-lpulse");
println!("cargo:rustc-link-arg=-lpulse-simple");
println!("cargo:rustc-link-arg=-lasound");
println!("cargo:rustc-link-arg=-lrt");
}

#[cfg(target_os = "macos")]
fn main() {
    // add macos dependencies below
    // println!("cargo:rustc-flags=-l edit");
}
