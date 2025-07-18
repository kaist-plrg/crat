use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn execv(
        __path: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn fork() -> __pid_t;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
}
pub type __pid_t = libc::c_int;
pub type pid_t = __pid_t;
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    if argc < 2 as libc::c_int {
        printf(
            b"Pass command to run as an argument: \"./space parent\" /cmd/to/run [args]\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    let mut pid: pid_t = fork();
    if pid == -(1 as libc::c_int) {
        printf(b"fork() failed\n\0" as *const u8 as *const libc::c_char);
        exit(2 as libc::c_int);
    }
    if pid == 0 as libc::c_int {
        execv(
            *argv.offset(1 as libc::c_int as isize),
            &mut *argv.offset(1 as libc::c_int as isize) as *mut *mut libc::c_char
                as *const *mut libc::c_char,
        );
        printf(b"execv() failed\n\0" as *const u8 as *const libc::c_char);
        exit(3 as libc::c_int);
    } else {
        let mut status: libc::c_int = 0;
        waitpid(pid, &mut status, 0 as libc::c_int);
        if status != 0 as libc::c_int {
            printf(
                b"Child regurned with a non-zero status: %d\n\0" as *const u8
                    as *const libc::c_char,
                status,
            );
            exit(4 as libc::c_int);
        }
    }
    return 0;
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
