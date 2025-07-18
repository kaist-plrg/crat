use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn zsysdep_in_dir(
        zdir: *const libc::c_char,
        zfile: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
}
pub type size_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type boolean = libc::c_int;
pub type tlog = libc::c_uint;
pub const LOG_DEBUG_END: tlog = 6;
pub const LOG_DEBUG_CONTINUE: tlog = 5;
pub const LOG_DEBUG_START: tlog = 4;
pub const LOG_DEBUG: tlog = 3;
pub const LOG_FATAL: tlog = 2;
pub const LOG_ERROR: tlog = 1;
pub const LOG_NORMAL: tlog = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
pub unsafe extern "C" fn zsysdep_local_file(
    mut zfile: *const libc::c_char,
    mut zpubdir: *const libc::c_char,
    mut pfbadname: *mut boolean,
) -> *mut libc::c_char {
    let mut zdir: *const libc::c_char = 0 as *const libc::c_char;
    if !pfbadname.is_null() {
        *pfbadname = 0 as libc::c_int;
    }
    if *zfile as libc::c_int == '/' as i32 {
        return zbufcpy(zfile);
    }
    if *zfile as libc::c_int != '~' as i32 {
        zdir = zpubdir;
    } else {
        if *zfile.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
            return zbufcpy(zpubdir);
        }
        if *zfile.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32 {
            zdir = zpubdir;
            zfile = zfile.offset(2 as libc::c_int as isize);
        } else {
            let mut cuserlen: size_t = 0;
            let mut zcopy: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut q: *mut passwd = 0 as *mut passwd;
            zfile = zfile.offset(1);
            zfile;
            cuserlen = strcspn(
                zfile as *mut libc::c_char,
                b"/\0" as *const u8 as *const libc::c_char,
            );
            zcopy = zbufalc(cuserlen.wrapping_add(1 as libc::c_int as libc::c_ulong));
            memcpy(zcopy as *mut libc::c_void, zfile as *const libc::c_void, cuserlen);
            *zcopy.offset(cuserlen as isize) = '\0' as i32 as libc::c_char;
            q = getpwnam(zcopy);
            if q.is_null() {
                ulog(
                    LOG_ERROR,
                    b"User %s not found\0" as *const u8 as *const libc::c_char,
                    zcopy,
                );
                ubuffree(zcopy);
                if !pfbadname.is_null() {
                    *pfbadname = 1 as libc::c_int;
                }
                return 0 as *mut libc::c_char;
            }
            ubuffree(zcopy);
            if *zfile.offset(cuserlen as isize) as libc::c_int == '\0' as i32 {
                return zbufcpy((*q).pw_dir);
            }
            zdir = (*q).pw_dir;
            zfile = zfile
                .offset(
                    cuserlen.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
        }
    }
    return zsysdep_in_dir(zdir, zfile);
}
