use ::libc;
extern "C" {
    fn renameat(
        __oldfd: libc::c_int,
        __old: *const libc::c_char,
        __newfd: libc::c_int,
        __new: *const libc::c_char,
    ) -> libc::c_int;
    fn renameat2(
        __oldfd: libc::c_int,
        __old: *const libc::c_char,
        __newfd: libc::c_int,
        __new: *const libc::c_char,
        __flags: libc::c_uint,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn __fxstatat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn rpl_free(ptr: *mut libc::c_void);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
unsafe extern "C" fn fstatat(
    mut __fd: libc::c_int,
    mut __filename: *const libc::c_char,
    mut __statbuf: *mut stat,
    mut __flag: libc::c_int,
) -> libc::c_int {
    return __fxstatat(1 as libc::c_int, __fd, __filename, __statbuf, __flag);
}
unsafe extern "C" fn errno_fail(mut e: libc::c_int) -> libc::c_int {
    *__errno_location() = e;
    return -(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn lstatat(
    mut fd: libc::c_int,
    mut name: *const libc::c_char,
    mut st: *mut stat,
) -> libc::c_int {
    return fstatat(fd, name, st, 0x100 as libc::c_int);
}
pub unsafe extern "C" fn renameatu(
    mut fd1: libc::c_int,
    mut src: *const libc::c_char,
    mut fd2: libc::c_int,
    mut dst: *const libc::c_char,
    mut flags: libc::c_uint,
) -> libc::c_int {
    let mut ret_val: libc::c_int = -(1 as libc::c_int);
    let mut err: libc::c_int = 22 as libc::c_int;
    ret_val = renameat2(fd1, src, fd2, dst, flags);
    err = *__errno_location();
    if !(ret_val < 0 as libc::c_int
        && (err == 22 as libc::c_int || err == 38 as libc::c_int
            || err == 95 as libc::c_int))
    {
        return ret_val;
    }
    let mut src_len: size_t = 0;
    let mut dst_len: size_t = 0;
    let mut src_temp: *mut libc::c_char = src as *mut libc::c_char;
    let mut dst_temp: *mut libc::c_char = dst as *mut libc::c_char;
    let mut src_slash: bool = false;
    let mut dst_slash: bool = false;
    let mut rename_errno: libc::c_int = 20 as libc::c_int;
    let mut src_st: stat = stat {
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
    let mut dst_st: stat = stat {
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
    let mut dst_found_nonexistent: bool = 0 as libc::c_int != 0;
    if flags & !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0 {
        return errno_fail(95 as libc::c_int);
    }
    if flags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        if lstatat(fd2, dst, &mut dst_st) == 0 as libc::c_int
            || *__errno_location() == 75 as libc::c_int
        {
            return errno_fail(17 as libc::c_int);
        }
        if *__errno_location() != 2 as libc::c_int {
            return -(1 as libc::c_int);
        }
        dst_found_nonexistent = 1 as libc::c_int != 0;
    }
    src_len = strlen(src);
    dst_len = strlen(dst);
    if src_len == 0 || dst_len == 0 {
        return renameat(fd1, src, fd2, dst);
    }
    src_slash = *src
        .offset(src_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        as libc::c_int == '/' as i32;
    dst_slash = *dst
        .offset(dst_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        as libc::c_int == '/' as i32;
    if !src_slash && !dst_slash {
        return renameat(fd1, src, fd2, dst);
    }
    if lstatat(fd1, src, &mut src_st) != 0 {
        return -(1 as libc::c_int);
    }
    if dst_found_nonexistent {
        if !(src_st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint)
        {
            return errno_fail(2 as libc::c_int);
        }
    } else if lstatat(fd2, dst, &mut dst_st) != 0 {
        if *__errno_location() != 2 as libc::c_int
            || !(src_st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint)
        {
            return -(1 as libc::c_int);
        }
    } else if !(dst_st.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint)
    {
        return errno_fail(20 as libc::c_int)
    } else if !(src_st.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint)
    {
        return errno_fail(21 as libc::c_int)
    }
    ret_val = renameat(fd1, src_temp, fd2, dst_temp);
    rename_errno = *__errno_location();
    if src_temp != src as *mut libc::c_char {
        rpl_free(src_temp as *mut libc::c_void);
    }
    if dst_temp != dst as *mut libc::c_char {
        rpl_free(dst_temp as *mut libc::c_void);
    }
    *__errno_location() = rename_errno;
    return ret_val;
}
