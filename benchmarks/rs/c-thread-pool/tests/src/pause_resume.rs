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
    fn thpool_add_work(
        _: threadpool,
        function_p: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        arg_p: *mut libc::c_void,
    ) -> libc::c_int;
    fn thpool_pause(_: threadpool);
    fn thpool_resume(_: threadpool);
    fn thpool_destroy(_: threadpool);
}
pub type threadpool = *mut thpool_;
pub unsafe extern "C" fn sleep_4_secs() {
    sleep(4 as libc::c_int as libc::c_uint);
    puts(b"SLEPT\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if argc != 2 as libc::c_int {
        puts(
            b"This testfile needs excactly one arguments\0" as *const u8
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
    thpool_pause(thpool);
    thpool_add_work(
        thpool,
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                *mut libc::c_void,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> (),
                        unsafe extern "C" fn() -> (),
                    >(sleep_4_secs),
                ),
            ),
        ),
        0 as *mut libc::c_void,
    );
    thpool_add_work(
        thpool,
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                *mut libc::c_void,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> (),
                        unsafe extern "C" fn() -> (),
                    >(sleep_4_secs),
                ),
            ),
        ),
        0 as *mut libc::c_void,
    );
    sleep(3 as libc::c_int as libc::c_uint);
    thpool_resume(thpool);
    sleep(2 as libc::c_int as libc::c_uint);
    thpool_destroy(thpool);
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
