#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    // add unix dependencies below
    // println!("cargo:rustc-flags=-l readline");
println!("cargo:rustc-link-arg=-lasound");
println!("cargo:rustc-link-arg=-lpulse");
println!("cargo:rustc-link-arg=-lpulse-simple");
println!("cargo:rustc-link-arg=-lao");
println!("cargo:rustc-link-arg=-lcrypto");
println!("cargo:rustc-link-arg=-lssl");
}

#[cfg(target_os = "macos")]
fn main() {
    // add macos dependencies below
    // println!("cargo:rustc-flags=-l edit");
}
