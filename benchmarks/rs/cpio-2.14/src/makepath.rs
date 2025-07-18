use ::libc;
extern "C" {
    fn stat_error(_: *const libc::c_char);
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn getuid() -> __uid_t;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut newdir_umask: mode_t;
    fn delay_set_stat(
        file_name: *const libc::c_char,
        st: *mut stat,
        invert_permissions: mode_t,
    );
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
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub unsafe extern "C" fn make_path(
    mut argpath: *const libc::c_char,
    mut owner: uid_t,
    mut group: gid_t,
    mut verbose_fmt_string: *const libc::c_char,
) -> libc::c_int {
    let mut dirpath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stats: stat = stat {
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
    let mut retval: libc::c_int = 0 as libc::c_int;
    let mut tmpmode: mode_t = 0;
    let mut invert_permissions: mode_t = 0;
    let mut we_are_root: libc::c_int = (getuid() == 0 as libc::c_int as libc::c_uint)
        as libc::c_int;
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (strlen(argpath)).wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
    );
    dirpath = fresh0.leak().as_mut_ptr() as *mut libc::c_char;
    strcpy(dirpath, argpath);
    if stat(dirpath, &mut stats) != 0 {
        tmpmode = (0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
            | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            | (0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                | (0o400 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
                    | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)))
            as libc::c_uint & !newdir_umask;
        invert_permissions = if we_are_root != 0 {
            0 as libc::c_int as libc::c_uint
        } else {
            (0o200 as libc::c_int | 0o100 as libc::c_int) as libc::c_uint & !tmpmode
        };
        let mut slash: *mut libc::c_char = dirpath;
        while *slash as libc::c_int == '/' as i32 {
            slash = slash.offset(1);
            slash;
        }
        loop {
            slash = strchr(slash, '/' as i32);
            if slash.is_null() {
                break;
            }
            *slash = '\0' as i32 as libc::c_char;
            if stat(dirpath, &mut stats) != 0 {
                if mkdir(dirpath, tmpmode ^ invert_permissions) != 0 {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"cannot make directory `%s'\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        dirpath,
                    );
                    return 1 as libc::c_int;
                } else {
                    if !verbose_fmt_string.is_null() {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            verbose_fmt_string,
                            dirpath,
                        );
                    }
                    if stat(dirpath, &mut stats) != 0 {
                        stat_error(dirpath);
                    } else {
                        if owner != -(1 as libc::c_int) as libc::c_uint {
                            stats.st_uid = owner;
                        }
                        if group != -(1 as libc::c_int) as libc::c_uint {
                            stats.st_gid = group;
                        }
                        delay_set_stat(dirpath, &mut stats, invert_permissions);
                    }
                }
            } else if !(stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint)
            {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"`%s' exists but is not a directory\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    dirpath,
                );
                return 1 as libc::c_int;
            }
            let fresh1 = slash;
            slash = slash.offset(1);
            *fresh1 = '/' as i32 as libc::c_char;
            while *slash as libc::c_int == '/' as i32 {
                slash = slash.offset(1);
                slash;
            }
        }
        if mkdir(dirpath, tmpmode ^ invert_permissions) != 0 {
            if *__errno_location() != 17 as libc::c_int
                || stat(dirpath, &mut stats) != 0 as libc::c_int
                || !(stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o40000 as libc::c_int as libc::c_uint)
            {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot make directory `%s'\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    dirpath,
                );
                return 1 as libc::c_int;
            }
        } else if stat(dirpath, &mut stats) != 0 {
            stat_error(dirpath);
        } else {
            if owner != -(1 as libc::c_int) as libc::c_uint {
                stats.st_uid = owner;
            }
            if group != -(1 as libc::c_int) as libc::c_uint {
                stats.st_gid = group;
            }
            delay_set_stat(dirpath, &mut stats, invert_permissions);
        }
        if !verbose_fmt_string.is_null() {
            error(0 as libc::c_int, 0 as libc::c_int, verbose_fmt_string, dirpath);
        }
    } else if !(stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint)
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"`%s' exists but is not a directory\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            dirpath,
        );
        return 1 as libc::c_int;
    }
    return retval;
}
