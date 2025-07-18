#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    // add unix dependencies below
    // println!("cargo:rustc-flags=-l readline");
println!("cargo:rustc-link-arg=-lpcre");
println!("cargo:rustc-link-arg=-lz");
println!("cargo:rustc-link-arg=-lnettle");
println!("cargo:rustc-link-arg=-lidn2");
println!("cargo:rustc-link-arg=-luuid");
}

#[cfg(target_os = "macos")]
fn main() {
    // add macos dependencies below
    // println!("cargo:rustc-flags=-l edit");
}
