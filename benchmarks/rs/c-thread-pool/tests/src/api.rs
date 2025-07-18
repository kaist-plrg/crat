use ::libc;
extern "C" {
    pub type thpool_;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn thpool_init(num_threads: libc::c_int) -> threadpool;
    fn thpool_add_work(
        _: threadpool,
        function_p: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        arg_p: *mut libc::c_void,
    ) -> libc::c_int;
    fn thpool_num_threads_working(_: threadpool) -> libc::c_int;
}
pub type threadpool = *mut thpool_;
pub unsafe extern "C" fn sleep_2_secs() {
    sleep(2 as libc::c_int as libc::c_uint);
    puts(b"SLEPT\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut num: libc::c_int = 0 as libc::c_int;
    let mut thpool: threadpool = 0 as *mut thpool_;
    thpool = thpool_init(10 as libc::c_int);
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
                    >(sleep_2_secs),
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
                    >(sleep_2_secs),
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
                    >(sleep_2_secs),
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
                    >(sleep_2_secs),
                ),
            ),
        ),
        0 as *mut libc::c_void,
    );
    sleep(1 as libc::c_int as libc::c_uint);
    num = thpool_num_threads_working(thpool);
    if thpool_num_threads_working(thpool) != 4 as libc::c_int {
        printf(
            b"Expected 4 threads working, got %d\0" as *const u8 as *const libc::c_char,
            num,
        );
        return -(1 as libc::c_int);
    }
    thpool = thpool_init(5 as libc::c_int);
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
                    >(sleep_2_secs),
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
                    >(sleep_2_secs),
                ),
            ),
        ),
        0 as *mut libc::c_void,
    );
    sleep(1 as libc::c_int as libc::c_uint);
    num = thpool_num_threads_working(thpool);
    if num != 2 as libc::c_int {
        printf(
            b"Expected 2 threads working, got %d\0" as *const u8 as *const libc::c_char,
            num,
        );
        return -(1 as libc::c_int);
    }
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
