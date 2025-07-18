#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    // add unix dependencies below
    // println!("cargo:rustc-flags=-l readline");
println!("cargo:rustc-link-arg=ksw2_extz2_sse41.o");
println!("cargo:rustc-link-arg=ksw2_exts2_sse41.o");
println!("cargo:rustc-link-arg=ksw2_extd2_sse41.o");
println!("cargo:rustc-link-arg=ksw2_extz2_sse2.o");
println!("cargo:rustc-link-arg=ksw2_exts2_sse2.o");
println!("cargo:rustc-link-arg=ksw2_extd2_sse2.o");
println!("cargo:rustc-link-arg=ksw2_ll_sse.o");
println!("cargo:rustc-link-arg=-lz");
}

#[cfg(target_os = "macos")]
fn main() {
    // add macos dependencies below
    // println!("cargo:rustc-flags=-l edit");
}
