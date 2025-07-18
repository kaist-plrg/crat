use ::libc;
extern "C" {
    pub type thpool_;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn exit(_: libc::c_int) -> !;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn thpool_init(num_threads: libc::c_int) -> threadpool;
    fn thpool_add_work(
        _: threadpool,
        function_p: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        arg_p: *mut libc::c_void,
    ) -> libc::c_int;
    fn thpool_wait(_: threadpool);
}
pub type threadpool = *mut thpool_;
pub unsafe extern "C" fn sleep_1(mut secs: *mut libc::c_int) {
    sleep(*secs as libc::c_uint);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if argc < 3 as libc::c_int {
        puts(
            b"This testfile needs at least two arguments\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    let mut num_jobs: libc::c_int = strtol(
        *argv.offset(1 as libc::c_int as isize),
        &mut p,
        10 as libc::c_int,
    ) as libc::c_int;
    let mut num_threads: libc::c_int = strtol(
        *argv.offset(2 as libc::c_int as isize),
        &mut p,
        10 as libc::c_int,
    ) as libc::c_int;
    let mut wait_each_job: libc::c_int = (if !(*argv.offset(3 as libc::c_int as isize))
        .is_null()
    {
        strtol(*argv.offset(3 as libc::c_int as isize), &mut p, 10 as libc::c_int)
    } else {
        0 as libc::c_int as libc::c_long
    }) as libc::c_int;
    let mut sleep_per_thread: libc::c_int = (if !(*argv
        .offset(4 as libc::c_int as isize))
        .is_null()
    {
        strtol(*argv.offset(4 as libc::c_int as isize), &mut p, 10 as libc::c_int)
    } else {
        1 as libc::c_int as libc::c_long
    }) as libc::c_int;
    let mut thpool: threadpool = thpool_init(num_threads);
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    while n < num_jobs {
        thpool_add_work(
            thpool,
            ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
            >(
                ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut libc::c_int) -> ()>,
                    *mut libc::c_void,
                >(Some(sleep_1 as unsafe extern "C" fn(*mut libc::c_int) -> ())),
            ),
            &mut sleep_per_thread as *mut libc::c_int as *mut libc::c_void,
        );
        if wait_each_job != 0 {
            thpool_wait(thpool);
        }
        n += 1;
        n;
    }
    if wait_each_job == 0 {
        thpool_wait(thpool);
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
