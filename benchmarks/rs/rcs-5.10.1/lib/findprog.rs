use ::libc;
extern "C" {
    fn rpl_free(ptr: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn eaccess(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn xconcatenated_filename(
        directory: *const libc::c_char,
        filename: *const libc::c_char,
        suffix: *const libc::c_char,
    ) -> *mut libc::c_char;
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
pub unsafe extern "C" fn find_in_path(
    mut progname: *const libc::c_char,
) -> *const libc::c_char {
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path_rest: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(strchr(progname, '/' as i32)).is_null() {
        return progname;
    }
    path = getenv(b"PATH\0" as *const u8 as *const libc::c_char);
    if path.is_null() || *path as libc::c_int == '\0' as i32 {
        return progname;
    }
    path = xstrdup(path);
    path_rest = path;
    loop {
        let mut dir: *const libc::c_char = 0 as *const libc::c_char;
        let mut last: bool = false;
        let mut progpathname: *mut libc::c_char = 0 as *mut libc::c_char;
        dir = path_rest;
        cp = path_rest;
        while *cp as libc::c_int != '\0' as i32 && *cp as libc::c_int != ':' as i32 {
            cp = cp.offset(1);
            cp;
        }
        last = *cp as libc::c_int == '\0' as i32;
        *cp = '\0' as i32 as libc::c_char;
        if dir == cp as *const libc::c_char {
            dir = b".\0" as *const u8 as *const libc::c_char;
        }
        progpathname = xconcatenated_filename(dir, progname, 0 as *const libc::c_char);
        if eaccess(progpathname, 1 as libc::c_int) == 0 as libc::c_int {
            let mut statbuf: stat = stat {
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
            if stat(progpathname, &mut statbuf) >= 0 as libc::c_int
                && !(statbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o40000 as libc::c_int as libc::c_uint)
            {
                if strcmp(progpathname, progname) == 0 as libc::c_int {
                    rpl_free(progpathname as *mut libc::c_void);
                    progpathname = (if ::std::mem::size_of::<libc::c_char>()
                        as libc::c_ulong == 1 as libc::c_int as libc::c_ulong
                    {
                        xmalloc(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(strlen(progname))
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        )
                    } else {
                        xnmalloc(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(strlen(progname))
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        )
                    }) as *mut libc::c_char;
                    *progpathname
                        .offset(0 as libc::c_int as isize) = '.' as i32 as libc::c_char;
                    *progpathname
                        .offset(1 as libc::c_int as isize) = '/' as i32 as libc::c_char;
                    memcpy(
                        progpathname.offset(2 as libc::c_int as isize)
                            as *mut libc::c_void,
                        progname as *const libc::c_void,
                        (strlen(progname))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    );
                }
                rpl_free(path as *mut libc::c_void);
                return progpathname;
            }
        }
        rpl_free(progpathname as *mut libc::c_void);
        if last {
            break;
        }
        path_rest = cp.offset(1 as libc::c_int as isize);
    }
    rpl_free(path as *mut libc::c_void);
    return progname;
}
