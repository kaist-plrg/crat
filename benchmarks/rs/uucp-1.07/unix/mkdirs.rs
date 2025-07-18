use ::libc;
extern "C" {
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn fsysdep_directory(zpath: *const libc::c_char) -> boolean;
    fn __errno_location() -> *mut libc::c_int;
}
pub type __mode_t = libc::c_uint;
pub type boolean = libc::c_int;
pub type tlog = libc::c_uint;
pub const LOG_DEBUG_END: tlog = 6;
pub const LOG_DEBUG_CONTINUE: tlog = 5;
pub const LOG_DEBUG_START: tlog = 4;
pub const LOG_DEBUG: tlog = 3;
pub const LOG_FATAL: tlog = 2;
pub const LOG_ERROR: tlog = 1;
pub const LOG_NORMAL: tlog = 0;
pub unsafe extern "C" fn fsysdep_make_dirs(
    mut zfile: *const libc::c_char,
    mut fpublic: boolean,
) -> boolean {
    let mut zcopy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut imode: libc::c_int = 0;
    zcopy = zbufcpy(zfile);
    if fpublic != 0 {
        imode = 0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int >> 3 as libc::c_int;
    } else {
        imode = 0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int
            | 0o100 as libc::c_int >> 3 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int;
    }
    z = zcopy;
    while *z as libc::c_int != '\0' as i32 {
        if *z as libc::c_int == '/' as i32 && z != zcopy {
            if !(*z.offset(-(1 as libc::c_int) as isize) as libc::c_int == '/' as i32) {
                *z = '\0' as i32 as libc::c_char;
                if mkdir(zcopy, imode as __mode_t) != 0 as libc::c_int {
                    let mut ierr: libc::c_int = 0;
                    ierr = *__errno_location();
                    if ierr != 17 as libc::c_int && ierr != 21 as libc::c_int
                        && ierr != 30 as libc::c_int
                        && (ierr != 13 as libc::c_int || fsysdep_directory(zcopy) == 0)
                    {
                        ulog(
                            LOG_ERROR,
                            b"mkdir (%s): %s\0" as *const u8 as *const libc::c_char,
                            zcopy,
                            strerror(ierr),
                        );
                        ubuffree(zcopy);
                        return 0 as libc::c_int;
                    }
                }
                *z = '/' as i32 as libc::c_char;
            }
        }
        z = z.offset(1);
        z;
    }
    ubuffree(zcopy);
    return 1 as libc::c_int;
}
