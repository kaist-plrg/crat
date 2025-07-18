#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    // add unix dependencies below
    // println!("cargo:rustc-flags=-l readline");
println!("cargo:rustc-link-arg=-lgcrypt");
println!("cargo:rustc-link-arg=-lao");
println!("cargo:rustc-link-arg=-lavfilter");
println!("cargo:rustc-link-arg=-lavutil");
println!("cargo:rustc-link-arg=-lavformat");
println!("cargo:rustc-link-arg=-lavcodec");
println!("cargo:rustc-link-arg=-ljson-c");
println!("cargo:rustc-link-arg=-lcurl");
}

#[cfg(target_os = "macos")]
fn main() {
    // add macos dependencies below
    // println!("cargo:rustc-flags=-l edit");
}
