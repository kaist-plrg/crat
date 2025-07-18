use ::libc;
extern "C" {
    pub type __dirstream;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    static mut iDebug: libc::c_int;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn zsysdep_in_dir(
        zdir: *const libc::c_char,
        zfile: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
}
pub type __ino_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type boolean = libc::c_int;
pub type tlog = libc::c_uint;
pub const LOG_DEBUG_END: tlog = 6;
pub const LOG_DEBUG_CONTINUE: tlog = 5;
pub const LOG_DEBUG_START: tlog = 4;
pub const LOG_DEBUG: tlog = 3;
pub const LOG_FATAL: tlog = 2;
pub const LOG_ERROR: tlog = 1;
pub const LOG_NORMAL: tlog = 0;
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub static mut xqtfil_rcsid: [libc::c_char; 51] = unsafe {
    *::std::mem::transmute::<
        &[u8; 51],
        &[libc::c_char; 51],
    >(b"$Id: xqtfil.c,v 1.13 2002/03/05 19:10:42 ian Rel $\0")
};
static mut qSxqt_topdir: *mut DIR = 0 as *const DIR as *mut DIR;
static mut fSone_dir: boolean = 0;
static mut zSdir: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut qSxqt_dir: *mut DIR = 0 as *const DIR as *mut DIR;
static mut zSsystem: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub unsafe extern "C" fn fsysdep_get_xqt_init(
    mut zsystem: *const libc::c_char,
) -> boolean {
    usysdep_get_xqt_free(0 as *mut libc::c_void as *const libc::c_char);
    if !zsystem.is_null() {
        zSdir = zsysdep_in_dir(zsystem, b"X.\0" as *const u8 as *const libc::c_char);
        qSxqt_dir = opendir(zSdir);
        if !qSxqt_dir.is_null() {
            qSxqt_topdir = qSxqt_dir;
            fSone_dir = 1 as libc::c_int;
            zSsystem = zbufcpy(zsystem);
            return 1 as libc::c_int;
        }
    }
    fSone_dir = 0 as libc::c_int;
    qSxqt_topdir = opendir(
        b".\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if qSxqt_topdir.is_null() {
        if *__errno_location() == 2 as libc::c_int {
            return 1 as libc::c_int;
        }
        ulog(
            LOG_ERROR,
            b"opendir (%s): %s\0" as *const u8 as *const libc::c_char,
            b".\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn zsysdep_get_xqt(
    mut zsystem: *const libc::c_char,
    mut pzsystem: *mut *mut libc::c_char,
    mut pferr: *mut boolean,
) -> *mut libc::c_char {
    *pferr = 0 as libc::c_int;
    if qSxqt_topdir.is_null() {
        return 0 as *mut libc::c_char;
    }
    loop {
        let mut qdir: *mut DIR = 0 as *mut DIR;
        let mut q: *mut dirent = 0 as *mut dirent;
        while qSxqt_dir.is_null() {
            let mut qtop: *mut dirent = 0 as *mut dirent;
            qtop = readdir(qSxqt_topdir);
            if qtop.is_null() {
                closedir(qSxqt_topdir);
                qSxqt_topdir = 0 as *mut DIR;
                return 0 as *mut libc::c_char;
            }
            if (*qtop).d_name[0 as libc::c_int as usize] as libc::c_int == '.' as i32 {
                continue;
            }
            if iDebug & 0o200 as libc::c_int != 0 as libc::c_int {
                ulog(
                    LOG_DEBUG,
                    b"zsysdep_get_xqt: Found %s in top directory\0" as *const u8
                        as *const libc::c_char,
                    ((*qtop).d_name).as_mut_ptr(),
                );
            }
            ubuffree(zSdir);
            zSdir = zsysdep_in_dir(
                ((*qtop).d_name).as_mut_ptr(),
                b"X.\0" as *const u8 as *const libc::c_char,
            );
            ubuffree(zSsystem);
            zSsystem = zbufcpy(((*qtop).d_name).as_mut_ptr());
            qSxqt_dir = opendir(zSdir);
            if qSxqt_dir.is_null() && *__errno_location() != 20 as libc::c_int
                && *__errno_location() != 2 as libc::c_int
            {
                ulog(
                    LOG_ERROR,
                    b"opendir (%s): %s\0" as *const u8 as *const libc::c_char,
                    zSdir,
                    strerror(*__errno_location()),
                );
            }
        }
        qdir = qSxqt_dir;
        q = readdir(qdir);
        if !q.is_null() {
            if iDebug & 0o200 as libc::c_int != 0 as libc::c_int {
                ulog(
                    LOG_DEBUG,
                    b"zsysdep_get_xqt: Found %s in subdirectory %s\0" as *const u8
                        as *const libc::c_char,
                    ((*q).d_name).as_mut_ptr(),
                    zSdir,
                );
            }
        }
        if !q.is_null()
            && (*q).d_name[0 as libc::c_int as usize] as libc::c_int == 'X' as i32
            && (*q).d_name[1 as libc::c_int as usize] as libc::c_int == '.' as i32
        {
            let mut zret: *mut libc::c_char = 0 as *mut libc::c_char;
            *pzsystem = zbufcpy(zSsystem);
            zret = zsysdep_in_dir(zSdir, ((*q).d_name).as_mut_ptr());
            if iDebug & 0o200 as libc::c_int != 0 as libc::c_int {
                ulog(
                    LOG_DEBUG,
                    b"zsysdep_get_xqt: Returning %s (system %s)\0" as *const u8
                        as *const libc::c_char,
                    zret,
                    *pzsystem,
                );
            }
            return zret;
        }
        if !q.is_null() {
            continue;
        }
        closedir(qdir);
        qSxqt_dir = 0 as *mut DIR;
        if fSone_dir == 0 {
            continue;
        }
        qSxqt_topdir = 0 as *mut DIR;
        return 0 as *mut libc::c_char;
    };
}
pub unsafe extern "C" fn usysdep_get_xqt_free(mut zsystem: *const libc::c_char) {
    if !qSxqt_topdir.is_null() {
        closedir(qSxqt_topdir);
        qSxqt_topdir = 0 as *mut DIR;
    }
    if !qSxqt_dir.is_null() {
        closedir(qSxqt_dir);
        qSxqt_dir = 0 as *mut DIR;
    }
    ubuffree(zSdir);
    zSdir = 0 as *mut libc::c_char;
    ubuffree(zSsystem);
    zSsystem = 0 as *mut libc::c_char;
    fSone_dir = 0 as libc::c_int;
}
