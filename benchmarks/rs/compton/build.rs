#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    // add unix dependencies below
    // println!("cargo:rustc-flags=-l readline");
println!("cargo:rustc-link-arg=-lXrandr");
println!("cargo:rustc-link-arg=-lXext");
println!("cargo:rustc-link-arg=-lX11");
println!("cargo:rustc-link-arg=-lXrender");
println!("cargo:rustc-link-arg=-lXfixes");
println!("cargo:rustc-link-arg=-lXdamage");
println!("cargo:rustc-link-arg=-lXcomposite");
}

#[cfg(target_os = "macos")]
fn main() {
    // add macos dependencies below
    // println!("cargo:rustc-flags=-l edit");
}
