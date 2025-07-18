use ::libc;
extern "C" {
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn zsysdep_in_dir(
        zdir: *const libc::c_char,
        zfile: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn fsysdep_directory(zpath: *const libc::c_char) -> boolean;
}
pub type size_t = libc::c_ulong;
pub type boolean = libc::c_int;
pub type tlog = libc::c_uint;
pub const LOG_DEBUG_END: tlog = 6;
pub const LOG_DEBUG_CONTINUE: tlog = 5;
pub const LOG_DEBUG_START: tlog = 4;
pub const LOG_DEBUG: tlog = 3;
pub const LOG_FATAL: tlog = 2;
pub const LOG_ERROR: tlog = 1;
pub const LOG_NORMAL: tlog = 0;
pub unsafe extern "C" fn zsysdep_add_base(
    mut zfile: *const libc::c_char,
    mut zname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut clen: size_t = 0;
    let mut zlook: *const libc::c_char = 0 as *const libc::c_char;
    let mut zfree: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zret: *mut libc::c_char = 0 as *mut libc::c_char;
    if *zfile as libc::c_int != '/' as i32 {
        ulog(
            LOG_FATAL,
            b"zsysdep_add_base: %s: Can't happen\0" as *const u8 as *const libc::c_char,
            zfile,
        );
    }
    clen = strlen(zfile);
    if *zfile.offset(clen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        as libc::c_int != '/' as i32
    {
        if fsysdep_directory(zfile) == 0 {
            return zbufcpy(zfile);
        }
        zfree = 0 as *mut libc::c_char;
    } else {
        zfree = zbufcpy(zfile);
        *zfree
            .offset(
                clen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = '\0' as i32 as libc::c_char;
        zfile = zfree;
    }
    zlook = strrchr(zname, '/' as i32);
    if !zlook.is_null() {
        zname = zlook.offset(1 as libc::c_int as isize);
    }
    zret = zsysdep_in_dir(zfile, zname);
    ubuffree(zfree);
    return zret;
}
