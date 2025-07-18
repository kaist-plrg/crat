use ::libc;
extern "C" {
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    static mut zScwd: *mut libc::c_char;
    fn zsysdep_local_file(
        zname: *const libc::c_char,
        zpubdir: *const libc::c_char,
        pfbadname: *mut boolean,
    ) -> *mut libc::c_char;
    fn zsysdep_in_dir(
        zdir: *const libc::c_char,
        zfile: *const libc::c_char,
    ) -> *mut libc::c_char;
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
pub unsafe extern "C" fn fsysdep_needs_cwd(mut zfile: *const libc::c_char) -> boolean {
    return (*zfile as libc::c_int != '/' as i32 && *zfile as libc::c_int != '~' as i32)
        as libc::c_int;
}
pub unsafe extern "C" fn zsysdep_local_file_cwd(
    mut zfile: *const libc::c_char,
    mut zpubdir: *const libc::c_char,
    mut pfbadname: *mut boolean,
) -> *mut libc::c_char {
    if !pfbadname.is_null() {
        *pfbadname = 0 as libc::c_int;
    }
    if *zfile as libc::c_int == '/' as i32 {
        return zbufcpy(zfile)
    } else if *zfile as libc::c_int == '~' as i32 {
        return zsysdep_local_file(zfile, zpubdir, pfbadname)
    } else {
        return zsysdep_add_cwd(zfile)
    };
}
pub unsafe extern "C" fn zsysdep_add_cwd(
    mut zfile: *const libc::c_char,
) -> *mut libc::c_char {
    if *zfile as libc::c_int == '/' as i32 || *zfile as libc::c_int == '~' as i32 {
        return zbufcpy(zfile);
    }
    if zScwd.is_null() {
        ulog(
            LOG_ERROR,
            b"Can't determine current directory\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut libc::c_char;
    }
    return zsysdep_in_dir(zScwd, zfile);
}
