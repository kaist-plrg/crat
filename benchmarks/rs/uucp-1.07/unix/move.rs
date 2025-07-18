use ::libc;
extern "C" {
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    static mut iDebug: libc::c_int;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn fcopy_file(
        zfrom: *const libc::c_char,
        zto: *const libc::c_char,
        fpublic: boolean,
        fmkdirs: boolean,
        fsignals: boolean,
    ) -> boolean;
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn fsuser_access(
        _: *const stat,
        imode: libc::c_int,
        zuser: *const libc::c_char,
    ) -> boolean;
    fn fsysdep_make_dirs(zfile: *const libc::c_char, fpublic: boolean) -> boolean;
    fn __errno_location() -> *mut libc::c_int;
    fn creat(__file: *const libc::c_char, __mode: mode_t) -> libc::c_int;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type mode_t = __mode_t;
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
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
pub unsafe extern "C" fn fsysdep_move_file(
    mut zorig: *const libc::c_char,
    mut zto: *const libc::c_char,
    mut fmkdirs: boolean,
    mut fpublic: boolean,
    mut fcheck: boolean,
    mut zuser: *const libc::c_char,
) -> boolean {
    let mut s: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut o: libc::c_int = 0;
    if iDebug & 0o200 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"fsysdep_move_file: Moving %s to %s\0" as *const u8 as *const libc::c_char,
            zorig,
            zto,
        );
    }
    if fcheck != 0 {
        let mut zcopy: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut zslash: *mut libc::c_char = 0 as *mut libc::c_char;
        zcopy = zbufcpy(zto);
        zslash = strrchr(zcopy, '/' as i32);
        if zslash == zcopy {
            *zslash.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        } else {
            *zslash = '\0' as i32 as libc::c_char;
        }
        if stat(zcopy, &mut s) != 0 as libc::c_int {
            ulog(
                LOG_ERROR,
                b"stat (%s): %s\0" as *const u8 as *const libc::c_char,
                zcopy,
                strerror(*__errno_location()),
            );
            ubuffree(zcopy);
            return 0 as libc::c_int;
        }
        if fsuser_access(&mut s, 2 as libc::c_int, zuser) == 0 {
            ulog(
                LOG_ERROR,
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                zcopy,
                strerror(13 as libc::c_int),
            );
            ubuffree(zcopy);
            return 0 as libc::c_int;
        }
        ubuffree(zcopy);
    }
    if rename(zorig, zto) == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if fmkdirs != 0 && *__errno_location() == 2 as libc::c_int {
        if fsysdep_make_dirs(zto, fpublic) == 0 {
            return 0 as libc::c_int;
        }
        if rename(zorig, zto) == 0 as libc::c_int {
            return 1 as libc::c_int;
        }
    }
    *__errno_location() = 18 as libc::c_int;
    if *__errno_location() != 18 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"rename (%s, %s): %s\0" as *const u8 as *const libc::c_char,
            zorig,
            zto,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    if stat(zorig as *mut libc::c_char, &mut s) < 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"stat (%s): %s\0" as *const u8 as *const libc::c_char,
            zorig,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    remove(zto);
    o = creat(zto as *mut libc::c_char, s.st_mode);
    if o < 0 as libc::c_int {
        if fmkdirs != 0 && *__errno_location() == 2 as libc::c_int {
            if fsysdep_make_dirs(zto, fpublic) == 0 {
                return 0 as libc::c_int;
            }
            o = creat(zto as *mut libc::c_char, s.st_mode);
        }
        if o < 0 as libc::c_int {
            ulog(
                LOG_ERROR,
                b"creat (%s): %s\0" as *const u8 as *const libc::c_char,
                zto,
                strerror(*__errno_location()),
            );
            return 0 as libc::c_int;
        }
    }
    close(o);
    if fcopy_file(zorig, zto, fpublic, fmkdirs, 0 as libc::c_int) == 0 {
        return 0 as libc::c_int;
    }
    if remove(zorig) != 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"remove (%s): %s\0" as *const u8 as *const libc::c_char,
            zorig,
            strerror(*__errno_location()),
        );
    }
    return 1 as libc::c_int;
}
