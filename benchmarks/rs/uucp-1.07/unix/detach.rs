use ::libc;
extern "C" {
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    static mut iDebug: libc::c_int;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn ulog_id(iid: libc::c_int);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn _exit(_: libc::c_int) -> !;
    fn getpid() -> __pid_t;
    fn getppid() -> __pid_t;
    fn getpgrp() -> __pid_t;
    fn setsid() -> __pid_t;
    fn ussignal(isig: libc::c_int);
    fn usset_signal(
        isig: libc::c_int,
        pfn: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        fforce: boolean,
        pfignored: *mut boolean,
    );
    fn ixsfork() -> pid_t;
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
}
pub type __pid_t = libc::c_int;
pub type pid_t = __pid_t;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type boolean = libc::c_int;
pub type tlog = libc::c_uint;
pub const LOG_DEBUG_END: tlog = 6;
pub const LOG_DEBUG_CONTINUE: tlog = 5;
pub const LOG_DEBUG_START: tlog = 4;
pub const LOG_DEBUG: tlog = 3;
pub const LOG_FATAL: tlog = 2;
pub const LOG_ERROR: tlog = 1;
pub const LOG_NORMAL: tlog = 0;
pub unsafe extern "C" fn usysdep_detach() {
    let mut igrp: pid_t = 0;
    ulog(LOG_NORMAL, 0 as *mut libc::c_void as *const libc::c_char);
    igrp = getpgrp();
    if igrp == getpid() {
        let mut fignored: boolean = 0;
        let mut ipid: pid_t = 0;
        usset_signal(
            1 as libc::c_int,
            ::std::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
            0 as libc::c_int,
            &mut fignored,
        );
        ipid = ixsfork();
        if ipid < 0 as libc::c_int {
            ulog(
                LOG_FATAL,
                b"fork: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        if ipid != 0 as libc::c_int {
            _exit(0 as libc::c_int);
        }
        while getppid() != 1 as libc::c_int {
            sleep(1 as libc::c_int as libc::c_uint);
        }
        ipid = getpid();
        ulog_id(ipid);
        if fignored == 0 {
            usset_signal(
                1 as libc::c_int,
                Some(ussignal as unsafe extern "C" fn(libc::c_int) -> ()),
                1 as libc::c_int,
                0 as *mut libc::c_void as *mut boolean,
            );
        }
        if iDebug & 0o40 as libc::c_int != 0 as libc::c_int {
            ulog(
                LOG_DEBUG,
                b"usysdep_detach: Forked; old PID %ld, new pid %ld\0" as *const u8
                    as *const libc::c_char,
                igrp as libc::c_long,
                ipid as libc::c_long,
            );
        }
    }
    close(0 as libc::c_int);
    close(1 as libc::c_int);
    close(2 as libc::c_int);
    if open(
        b"/dev/null\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    ) != 0 as libc::c_int
        || open(
            b"/dev/null\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0o1 as libc::c_int,
        ) != 1 as libc::c_int
        || open(
            b"/dev/null\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0o1 as libc::c_int,
        ) != 2 as libc::c_int
    {
        ulog(
            LOG_FATAL,
            b"open (/dev/null): %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
    if setsid() < 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"setsid: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
}
