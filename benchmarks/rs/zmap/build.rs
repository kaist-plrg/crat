#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    // add unix dependencies below
    // println!("cargo:rustc-flags=-l readline");
println!("cargo:rustc-link-arg=-ljson-c");
println!("cargo:rustc-link-arg=-lunistring");
println!("cargo:rustc-link-arg=-lgmp");
println!("cargo:rustc-link-arg=-lpcap");
}

#[cfg(target_os = "macos")]
fn main() {
    // add macos dependencies below
    // println!("cargo:rustc-flags=-l edit");
}
