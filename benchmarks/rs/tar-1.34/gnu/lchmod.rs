use ::libc;
extern "C" {
    fn __fxstatat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
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
pub type mode_t = __mode_t;
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
pub unsafe extern "C" fn lchmod(
    mut file: *const libc::c_char,
    mut mode: mode_t,
) -> libc::c_int {
    let mut fd: libc::c_int = open(
        file,
        0o10000000 as libc::c_int | 0o400000 as libc::c_int | 0o2000000 as libc::c_int,
    );
    if fd < 0 as libc::c_int {
        return fd;
    }
    let mut st: stat = stat {
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
    if fstatat(
        fd,
        b"\0" as *const u8 as *const libc::c_char,
        &mut st,
        0x1000 as libc::c_int,
    ) != 0 as libc::c_int
    {
        let mut stat_errno: libc::c_int = *__errno_location();
        close(fd);
        *__errno_location() = stat_errno;
        return -(1 as libc::c_int);
    }
    if st.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o120000 as libc::c_int as libc::c_uint
    {
        close(fd);
        *__errno_location() = 95 as libc::c_int;
        return -(1 as libc::c_int);
    }
    static mut fmt: [libc::c_char; 17] = unsafe {
        *::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"/proc/self/fd/%d\0")
    };
    let mut buf: [libc::c_char; 26] = [0; 26];
    sprintf(buf.as_mut_ptr(), fmt.as_ptr(), fd);
    let mut chmod_result: libc::c_int = chmod(buf.as_mut_ptr(), mode);
    let mut chmod_errno: libc::c_int = *__errno_location();
    close(fd);
    if chmod_result == 0 as libc::c_int {
        return chmod_result;
    }
    if chmod_errno != 2 as libc::c_int {
        *__errno_location() = chmod_errno;
        return chmod_result;
    }
    return chmod(file, mode);
}
