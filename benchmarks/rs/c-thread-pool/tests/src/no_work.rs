use ::libc;
extern "C" {
    pub type thpool_;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn exit(_: libc::c_int) -> !;
    fn thpool_init(num_threads: libc::c_int) -> threadpool;
    fn thpool_destroy(_: threadpool);
}
pub type threadpool = *mut thpool_;
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if argc != 2 as libc::c_int {
        puts(
            b"This testfile needs exactly one arguments\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    let mut num_threads: libc::c_int = strtol(
        *argv.offset(1 as libc::c_int as isize),
        &mut p,
        10 as libc::c_int,
    ) as libc::c_int;
    let mut thpool: threadpool = thpool_init(num_threads);
    thpool_destroy(thpool);
    sleep(1 as libc::c_int as libc::c_uint);
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
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
