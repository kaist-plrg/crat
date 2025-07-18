use ::libc;
extern "C" {
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
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
pub unsafe extern "C" fn file_type(mut st: *const stat) -> *const libc::c_char {
    if (*st).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
    {
        return if (*st).st_size == 0 as libc::c_int as libc::c_long {
            dcgettext(
                0 as *const libc::c_char,
                b"regular empty file\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            )
        } else {
            dcgettext(
                0 as *const libc::c_char,
                b"regular file\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            )
        };
    }
    if (*st).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint
    {
        return dcgettext(
            0 as *const libc::c_char,
            b"directory\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
    }
    if (*st).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o120000 as libc::c_int as libc::c_uint
    {
        return dcgettext(
            0 as *const libc::c_char,
            b"symbolic link\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
    }
    if ((*st).st_mode).wrapping_sub((*st).st_mode) != 0 {
        return dcgettext(
            0 as *const libc::c_char,
            b"message queue\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
    }
    if ((*st).st_mode).wrapping_sub((*st).st_mode) != 0 {
        return dcgettext(
            0 as *const libc::c_char,
            b"semaphore\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
    }
    if ((*st).st_mode).wrapping_sub((*st).st_mode) != 0 {
        return dcgettext(
            0 as *const libc::c_char,
            b"shared memory object\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
    }
    if (*st).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o60000 as libc::c_int as libc::c_uint
    {
        return dcgettext(
            0 as *const libc::c_char,
            b"block special file\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
    }
    if (*st).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o20000 as libc::c_int as libc::c_uint
    {
        return dcgettext(
            0 as *const libc::c_char,
            b"character special file\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
    }
    if (*st).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o10000 as libc::c_int as libc::c_uint
    {
        return dcgettext(
            0 as *const libc::c_char,
            b"fifo\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
    }
    if (*st).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o140000 as libc::c_int as libc::c_uint
    {
        return dcgettext(
            0 as *const libc::c_char,
            b"socket\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
    }
    return dcgettext(
        0 as *const libc::c_char,
        b"weird file\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int,
    );
}
