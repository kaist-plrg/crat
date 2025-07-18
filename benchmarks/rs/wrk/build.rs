#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    // add unix dependencies below
    // println!("cargo:rustc-flags=-l readline");
println!("cargo:rustc-link-arg=-L.");
println!("cargo:rustc-link-arg=-Wl,-rpath,.");
println!("cargo:rustc-link-arg=-lluajit-5.1");
println!("cargo:rustc-link-arg=-lssl");
println!("cargo:rustc-link-arg=-lcrypto");
}

#[cfg(target_os = "macos")]
fn main() {
    // add macos dependencies below
    // println!("cargo:rustc-flags=-l edit");
}
