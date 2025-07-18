#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    // add unix dependencies below
    // println!("cargo:rustc-flags=-l readline");
println!("cargo:rustc-link-arg=-L/usr/local/lib");
println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/local/lib");
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
