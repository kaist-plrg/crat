use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn eaccess(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn concatenated_filename(
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
static mut suffixes: [*const libc::c_char; 1] = [
    b"\0" as *const u8 as *const libc::c_char,
];
pub unsafe extern "C" fn find_in_given_path(
    mut progname: *const libc::c_char,
    mut path: *const libc::c_char,
    mut directory: *const libc::c_char,
    mut optimize_for_exec: bool,
) -> *const libc::c_char {
    let mut has_slash: bool = 0 as libc::c_int != 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = progname;
    while *p as libc::c_int != '\0' as i32 {
        if *p as libc::c_int == '/' as i32 {
            has_slash = 1 as libc::c_int != 0;
            break;
        } else {
            p = p.offset(1);
            p;
        }
    }
    if has_slash {
        if optimize_for_exec {
            return progname
        } else {
            let mut failure_errno: libc::c_int = 0;
            let mut i: size_t = 0;
            let mut directory_as_prefix: *const libc::c_char = if !directory.is_null()
                && !(*progname.offset(0 as libc::c_int as isize) as libc::c_int
                    == '/' as i32)
            {
                directory
            } else {
                b"\0" as *const u8 as *const libc::c_char
            };
            failure_errno = 2 as libc::c_int;
            i = 0 as libc::c_int as size_t;
            while i
                < (::std::mem::size_of::<[*const libc::c_char; 1]>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                    )
            {
                let mut suffix: *const libc::c_char = suffixes[i as usize];
                let mut progpathname: *mut libc::c_char = concatenated_filename(
                    directory_as_prefix,
                    progname,
                    suffix,
                );
                if progpathname.is_null() {
                    return 0 as *const libc::c_char;
                }
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
                    if stat(progpathname, &mut statbuf) >= 0 as libc::c_int {
                        if !(statbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                            == 0o40000 as libc::c_int as libc::c_uint)
                        {
                            if strcmp(progpathname, progname) == 0 as libc::c_int {
                                free(progpathname as *mut libc::c_void);
                                return progname;
                            } else {
                                return progpathname
                            }
                        }
                        *__errno_location() = 13 as libc::c_int;
                    }
                }
                if *__errno_location() != 2 as libc::c_int {
                    failure_errno = *__errno_location();
                }
                free(progpathname as *mut libc::c_void);
                i = i.wrapping_add(1);
                i;
            }
            *__errno_location() = failure_errno;
            return 0 as *const libc::c_char;
        }
    }
    if path.is_null() {
        path = b"\0" as *const u8 as *const libc::c_char;
    }
    let mut path_copy: *mut libc::c_char = strdup(path);
    if path_copy.is_null() {
        return 0 as *const libc::c_char;
    }
    let mut failure_errno_0: libc::c_int = 0;
    let mut path_rest: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    failure_errno_0 = 2 as libc::c_int;
    path_rest = path_copy;
    's_188: loop {
        let mut dir: *const libc::c_char = 0 as *const libc::c_char;
        let mut last: bool = false;
        let mut dir_as_prefix_to_free: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut dir_as_prefix: *const libc::c_char = 0 as *const libc::c_char;
        let mut i_0: size_t = 0;
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
        if !directory.is_null()
            && !(*dir.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32)
        {
            dir_as_prefix_to_free = concatenated_filename(
                directory,
                dir,
                0 as *const libc::c_char,
            );
            if dir_as_prefix_to_free.is_null() {
                failure_errno_0 = *__errno_location();
                break;
            } else {
                dir_as_prefix = dir_as_prefix_to_free;
            }
        } else {
            dir_as_prefix_to_free = 0 as *mut libc::c_char;
            dir_as_prefix = dir;
        }
        i_0 = 0 as libc::c_int as size_t;
        while i_0
            < (::std::mem::size_of::<[*const libc::c_char; 1]>() as libc::c_ulong)
                .wrapping_div(
                    ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                )
        {
            let mut suffix_0: *const libc::c_char = suffixes[i_0 as usize];
            let mut progpathname_0: *mut libc::c_char = concatenated_filename(
                dir_as_prefix,
                progname,
                suffix_0,
            );
            if progpathname_0.is_null() {
                failure_errno_0 = *__errno_location();
                free(dir_as_prefix_to_free as *mut libc::c_void);
                break 's_188;
            } else {
                if eaccess(progpathname_0, 1 as libc::c_int) == 0 as libc::c_int {
                    let mut statbuf_0: stat = stat {
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
                    if stat(progpathname_0, &mut statbuf_0) >= 0 as libc::c_int {
                        if !(statbuf_0.st_mode & 0o170000 as libc::c_int as libc::c_uint
                            == 0o40000 as libc::c_int as libc::c_uint)
                        {
                            if strcmp(progpathname_0, progname) == 0 as libc::c_int {
                                free(progpathname_0 as *mut libc::c_void);
                                progpathname_0 = malloc(
                                    (2 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(strlen(progname))
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ) as *mut libc::c_char;
                                if progpathname_0.is_null() {
                                    failure_errno_0 = *__errno_location();
                                    free(dir_as_prefix_to_free as *mut libc::c_void);
                                    break 's_188;
                                } else {
                                    *progpathname_0
                                        .offset(
                                            0 as libc::c_int as isize,
                                        ) = '.' as i32 as libc::c_char;
                                    *progpathname_0
                                        .offset(
                                            1 as libc::c_int as isize,
                                        ) = '/' as i32 as libc::c_char;
                                    memcpy(
                                        progpathname_0.offset(2 as libc::c_int as isize)
                                            as *mut libc::c_void,
                                        progname as *const libc::c_void,
                                        (strlen(progname))
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                    );
                                }
                            }
                            free(dir_as_prefix_to_free as *mut libc::c_void);
                            free(path_copy as *mut libc::c_void);
                            return progpathname_0;
                        } else {
                            *__errno_location() = 13 as libc::c_int;
                        }
                    }
                }
                if *__errno_location() != 2 as libc::c_int {
                    failure_errno_0 = *__errno_location();
                }
                free(progpathname_0 as *mut libc::c_void);
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        }
        free(dir_as_prefix_to_free as *mut libc::c_void);
        if last {
            break;
        }
        path_rest = cp.offset(1 as libc::c_int as isize);
    }
    free(path_copy as *mut libc::c_void);
    *__errno_location() = failure_errno_0;
    return 0 as *const libc::c_char;
}
