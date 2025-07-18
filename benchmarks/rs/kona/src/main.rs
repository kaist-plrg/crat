use ::libc;
extern "C" {
    fn attend() -> I;
    fn kinit() -> I;
    fn args(n: libc::c_int, v: *mut S) -> I;
}
pub type I = libc::c_longlong;
pub type C = libc::c_char;
pub type S = *mut C;
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut S) -> libc::c_int {
    kinit();
    args(argc, argv);
    attend();
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0((args.len() - 1) as libc::c_int, args.as_mut_ptr() as *mut S) as i32,
        )
    }
}
