use ::libc;
extern "C" {
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn link(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;
    fn fsysdep_make_dirs(zfile: *const libc::c_char, fpublic: boolean) -> boolean;
    fn __errno_location() -> *mut libc::c_int;
}
pub type boolean = libc::c_int;
pub type tlog = libc::c_uint;
pub const LOG_DEBUG_END: tlog = 6;
pub const LOG_DEBUG_CONTINUE: tlog = 5;
pub const LOG_DEBUG_START: tlog = 4;
pub const LOG_DEBUG: tlog = 3;
pub const LOG_FATAL: tlog = 2;
pub const LOG_ERROR: tlog = 1;
pub const LOG_NORMAL: tlog = 0;
pub unsafe extern "C" fn fsysdep_link(
    mut zfrom: *const libc::c_char,
    mut zto: *const libc::c_char,
    mut pfworked: *mut boolean,
) -> boolean {
    *pfworked = 0 as libc::c_int;
    if link(zfrom, zto) == 0 as libc::c_int {
        *pfworked = 1 as libc::c_int;
        return 1 as libc::c_int;
    }
    if *__errno_location() == 2 as libc::c_int {
        if fsysdep_make_dirs(zto, 1 as libc::c_int) == 0 {
            return 0 as libc::c_int;
        }
        if link(zfrom, zto) == 0 as libc::c_int {
            *pfworked = 1 as libc::c_int;
            return 1 as libc::c_int;
        }
    }
    if *__errno_location() == 18 as libc::c_int {
        return 1 as libc::c_int;
    }
    ulog(
        LOG_ERROR,
        b"link (%s, %s): %s\0" as *const u8 as *const libc::c_char,
        zfrom,
        zto,
        strerror(*__errno_location()),
    );
    return 0 as libc::c_int;
}
