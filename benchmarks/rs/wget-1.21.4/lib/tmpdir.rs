use ::libc;
extern "C" {
    fn secure_getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
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
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __time_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __gid_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __ino_t = libc::c_ulong;
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
unsafe extern "C" fn direxists(mut dir: *const libc::c_char) -> bool {
    let mut buf: stat = stat {
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
    return stat(dir, &mut buf) == 0 as libc::c_int
        && buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn path_search(
    mut tmpl: *mut libc::c_char,
    mut tmpl_len: size_t,
    mut dir: *const libc::c_char,
    mut pfx: *const libc::c_char,
    mut try_tmpdir: bool,
) -> libc::c_int {
    let mut d: *const libc::c_char = 0 as *const libc::c_char;
    let mut dlen: size_t = 0;
    let mut plen: size_t = 0;
    let mut add_slash: bool = false;
    if pfx.is_null() || *pfx.offset(0 as libc::c_int as isize) == 0 {
        pfx = b"file\0" as *const u8 as *const libc::c_char;
        plen = 4 as libc::c_int as size_t;
    } else {
        plen = strlen(pfx);
        if plen > 5 as libc::c_int as libc::c_ulong {
            plen = 5 as libc::c_int as size_t;
        }
    }
    if try_tmpdir {
        d = secure_getenv(b"TMPDIR\0" as *const u8 as *const libc::c_char);
        if !d.is_null() && direxists(d) as libc::c_int != 0 {
            dir = d;
        } else if !(!dir.is_null() && direxists(dir) as libc::c_int != 0) {
            dir = 0 as *const libc::c_char;
        }
    }
    if dir.is_null() {
        if direxists(b"/tmp\0" as *const u8 as *const libc::c_char) {
            dir = b"/tmp\0" as *const u8 as *const libc::c_char;
        } else if strcmp(
            b"/tmp\0" as *const u8 as *const libc::c_char,
            b"/tmp\0" as *const u8 as *const libc::c_char,
        ) != 0 as libc::c_int
            && direxists(b"/tmp\0" as *const u8 as *const libc::c_char) as libc::c_int
                != 0
        {
            dir = b"/tmp\0" as *const u8 as *const libc::c_char;
        } else {
            *__errno_location() = 2 as libc::c_int;
            return -(1 as libc::c_int);
        }
    }
    dlen = strlen(dir);
    add_slash = dlen != 0 as libc::c_int as libc::c_ulong
        && !(*dir.offset(dlen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == '/' as i32);
    if tmpl_len
        < dlen
            .wrapping_add(add_slash as libc::c_ulong)
            .wrapping_add(plen)
            .wrapping_add(6 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    memcpy(tmpl as *mut libc::c_void, dir as *const libc::c_void, dlen);
    sprintf(
        tmpl.offset(dlen as isize),
        &*(b"/%.*sXXXXXX\0" as *const u8 as *const libc::c_char)
            .offset(!add_slash as libc::c_int as isize) as *const libc::c_char,
        plen as libc::c_int,
        pfx,
    );
    return 0 as libc::c_int;
}
