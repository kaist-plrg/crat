use ::libc;
extern "C" {
    fn ttyname_r(
        __fd: libc::c_int,
        __buf: *mut libc::c_char,
        __buflen: size_t,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
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
pub type size_t = libc::c_ulong;
pub type uid_t = __uid_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
pub unsafe extern "C" fn snoopy_datasource_tty__get_tty_uid(
    mut ttyUid: *mut uid_t,
    result: *mut libc::c_char,
) -> libc::c_int {
    let mut ttyPath: [libc::c_char; 4097] = [0; 4097];
    let mut ttyPathLen: size_t = 4096 as libc::c_int as size_t;
    let mut retVal: libc::c_int = 0;
    let mut statbuffer: stat = stat {
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
    retVal = ttyname_r(0 as libc::c_int, ttyPath.as_mut_ptr(), ttyPathLen);
    if 0 as libc::c_int != retVal {
        if 9 as libc::c_int == retVal {
            return snprintf(
                result,
                2048 as libc::c_int as libc::c_ulong,
                b"ERROR(ttyname_r->EBADF)\0" as *const u8 as *const libc::c_char,
            );
        }
        if 34 as libc::c_int == retVal {
            return snprintf(
                result,
                2048 as libc::c_int as libc::c_ulong,
                b"ERROR(ttyname_r->ERANGE)\0" as *const u8 as *const libc::c_char,
            );
        }
        if 25 as libc::c_int == retVal {
            return snprintf(
                result,
                2048 as libc::c_int as libc::c_ulong,
                b"(none)\0" as *const u8 as *const libc::c_char,
            );
        }
        return snprintf(
            result,
            2048 as libc::c_int as libc::c_ulong,
            b"(unknown)\0" as *const u8 as *const libc::c_char,
        );
    }
    if -(1 as libc::c_int) == stat(ttyPath.as_mut_ptr(), &mut statbuffer) {
        return snprintf(
            result,
            2048 as libc::c_int as libc::c_ulong,
            b"ERROR(unable to stat() %s)\0" as *const u8 as *const libc::c_char,
            ttyPath.as_mut_ptr(),
        );
    }
    *ttyUid = statbuffer.st_uid;
    return 0 as libc::c_int;
}
