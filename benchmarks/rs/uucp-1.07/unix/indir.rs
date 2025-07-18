use ::libc;
extern "C" {
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn fsuser_access(
        _: *const stat,
        imode: libc::c_int,
        zuser: *const libc::c_char,
    ) -> boolean;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
}
pub type size_t = libc::c_ulong;
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
pub unsafe extern "C" fn fsysdep_in_directory(
    mut zfile: *const libc::c_char,
    mut zdir: *const libc::c_char,
    mut fcheck: boolean,
    mut freadable: boolean,
    mut zuser: *const libc::c_char,
) -> boolean {
    let mut c: size_t = 0;
    let mut zcopy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zslash: *mut libc::c_char = 0 as *mut libc::c_char;
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
    if *zfile as libc::c_int != '/' as i32 {
        return 0 as libc::c_int;
    }
    c = strlen(zdir);
    if c > 0 as libc::c_int as libc::c_ulong
        && *zdir.offset(c.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == '/' as i32
    {
        c = c.wrapping_sub(1);
        c;
    }
    if strncmp(zfile, zdir, c) != 0 as libc::c_int
        || *zfile.offset(c as isize) as libc::c_int != '/' as i32
            && *zfile.offset(c as isize) as libc::c_int != '\0' as i32
    {
        return 0 as libc::c_int;
    }
    if !(strstr(zfile.offset(c as isize), b"/../\0" as *const u8 as *const libc::c_char))
        .is_null()
    {
        return 0 as libc::c_int;
    }
    if fcheck == 0 {
        return 1 as libc::c_int;
    }
    zcopy = zbufcpy(zfile);
    zslash = zcopy.offset(c as isize);
    loop {
        let mut b: libc::c_char = 0;
        let mut shold: stat = stat {
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
        b = *zslash;
        *zslash = '\0' as i32 as libc::c_char;
        shold = s;
        if stat(zcopy, &mut s) != 0 as libc::c_int {
            if *__errno_location() != 2 as libc::c_int {
                ulog(
                    LOG_ERROR,
                    b"stat (%s): %s\0" as *const u8 as *const libc::c_char,
                    zcopy,
                    strerror(*__errno_location()),
                );
                ubuffree(zcopy);
                return 0 as libc::c_int;
            }
            if zslash == zcopy.offset(c as isize) {
                ubuffree(zcopy);
                return 1 as libc::c_int;
            }
            s = shold;
            break;
        } else {
            if !(s.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint)
            {
                break;
            }
            if fsuser_access(&mut s, 1 as libc::c_int, zuser) == 0 {
                ulog(
                    LOG_ERROR,
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    zcopy,
                    strerror(13 as libc::c_int),
                );
                ubuffree(zcopy);
                return 0 as libc::c_int;
            }
            if b as libc::c_int == '\0' as i32 {
                break;
            }
            *zslash = b;
            zslash = strchr(zslash.offset(1 as libc::c_int as isize), '/' as i32);
            if zslash.is_null() {
                break;
            }
        }
    }
    if fsuser_access(
        &mut s,
        if freadable != 0 { 4 as libc::c_int } else { 2 as libc::c_int },
        zuser,
    ) == 0
    {
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
    return 1 as libc::c_int;
}
