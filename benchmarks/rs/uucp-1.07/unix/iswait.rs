use ::libc;
extern "C" {
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    static mut iDebug: libc::c_int;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn __errno_location() -> *mut libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
}
pub type __pid_t = libc::c_int;
pub type pid_t = __pid_t;
pub type pointer = *mut libc::c_void;
pub type tlog = libc::c_uint;
pub const LOG_DEBUG_END: tlog = 6;
pub const LOG_DEBUG_CONTINUE: tlog = 5;
pub const LOG_DEBUG_START: tlog = 4;
pub const LOG_DEBUG: tlog = 3;
pub const LOG_FATAL: tlog = 2;
pub const LOG_ERROR: tlog = 1;
pub const LOG_NORMAL: tlog = 0;
pub type wait_status = libc::c_int;
pub unsafe extern "C" fn ixswait(
    mut ipid: libc::c_ulong,
    mut zreport: *const libc::c_char,
) -> libc::c_int {
    let mut istat: wait_status = 0;
    while waitpid(
        ipid as pid_t,
        &mut istat as *mut wait_status as pointer as *mut libc::c_int,
        0 as libc::c_int,
    ) < 0 as libc::c_int
    {
        if *__errno_location() != 4 as libc::c_int {
            if !zreport.is_null() {
                ulog(
                    LOG_ERROR,
                    b"waitpid: %s\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
            }
            return -(1 as libc::c_int);
        }
        ulog(LOG_ERROR, 0 as *mut libc::c_void as *const libc::c_char);
    }
    if iDebug & 0o400 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"%s %d\0" as *const u8 as *const libc::c_char,
            if istat & 0x7f as libc::c_int == 0 as libc::c_int {
                b"Exit status\0" as *const u8 as *const libc::c_char
            } else {
                b"Signal\0" as *const u8 as *const libc::c_char
            },
            if istat & 0x7f as libc::c_int == 0 as libc::c_int {
                (istat & 0xff00 as libc::c_int) >> 8 as libc::c_int
            } else {
                istat & 0x7f as libc::c_int
            },
        );
    }
    if istat & 0x7f as libc::c_int == 0 as libc::c_int
        && (istat & 0xff00 as libc::c_int) >> 8 as libc::c_int == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if !zreport.is_null() {
        if !(istat & 0x7f as libc::c_int == 0 as libc::c_int) {
            ulog(
                LOG_ERROR,
                b"%s: Got signal %d\0" as *const u8 as *const libc::c_char,
                zreport,
                istat & 0x7f as libc::c_int,
            );
        } else {
            ulog(
                LOG_ERROR,
                b"%s: Exit status %d\0" as *const u8 as *const libc::c_char,
                zreport,
                (istat & 0xff00 as libc::c_int) >> 8 as libc::c_int,
            );
        }
    }
    if istat & 0x7f as libc::c_int == 0 as libc::c_int {
        return (istat & 0xff00 as libc::c_int) >> 8 as libc::c_int
    } else {
        return -(1 as libc::c_int)
    };
}
