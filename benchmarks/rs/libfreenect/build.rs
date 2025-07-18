#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    // add unix dependencies below
    // println!("cargo:rustc-flags=-l readline");
println!("cargo:rustc-link-arg=-lusb-1.0");
println!("cargo:rustc-link-arg=-lglut");
println!("cargo:rustc-link-arg=-lGLU");
println!("cargo:rustc-link-arg=-lGL");
}

#[cfg(target_os = "macos")]
fn main() {
    // add macos dependencies below
    // println!("cargo:rustc-flags=-l edit");
}
