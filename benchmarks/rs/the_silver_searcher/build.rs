#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    // add unix dependencies below
    // println!("cargo:rustc-flags=-l readline");
println!("cargo:rustc-link-arg=-lz");
println!("cargo:rustc-link-arg=-llzma");
println!("cargo:rustc-link-arg=-lpcre");
}

#[cfg(target_os = "macos")]
fn main() {
    // add macos dependencies below
    // println!("cargo:rustc-flags=-l edit");
}
