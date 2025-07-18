use ::libc;
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
unsafe extern "C" fn ftypelet(mut bits: mode_t) -> libc::c_char {
    if bits & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
    {
        return '-' as i32 as libc::c_char;
    }
    if bits & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint
    {
        return 'd' as i32 as libc::c_char;
    }
    if bits & 0o170000 as libc::c_int as libc::c_uint
        == 0o60000 as libc::c_int as libc::c_uint
    {
        return 'b' as i32 as libc::c_char;
    }
    if bits & 0o170000 as libc::c_int as libc::c_uint
        == 0o20000 as libc::c_int as libc::c_uint
    {
        return 'c' as i32 as libc::c_char;
    }
    if bits & 0o170000 as libc::c_int as libc::c_uint
        == 0o120000 as libc::c_int as libc::c_uint
    {
        return 'l' as i32 as libc::c_char;
    }
    if bits & 0o170000 as libc::c_int as libc::c_uint
        == 0o10000 as libc::c_int as libc::c_uint
    {
        return 'p' as i32 as libc::c_char;
    }
    if bits & 0o170000 as libc::c_int as libc::c_uint
        == 0o140000 as libc::c_int as libc::c_uint
    {
        return 's' as i32 as libc::c_char;
    }
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return 'm' as i32 as libc::c_char;
    }
    return '?' as i32 as libc::c_char;
}
pub unsafe extern "C" fn strmode(mut mode: mode_t, mut str: *mut libc::c_char) {
    *str.offset(0 as libc::c_int as isize) = ftypelet(mode);
    *str
        .offset(
            1 as libc::c_int as isize,
        ) = (if mode & 0o400 as libc::c_int as libc::c_uint != 0 {
        'r' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    *str
        .offset(
            2 as libc::c_int as isize,
        ) = (if mode & 0o200 as libc::c_int as libc::c_uint != 0 {
        'w' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    *str
        .offset(
            3 as libc::c_int as isize,
        ) = (if mode & 0o4000 as libc::c_int as libc::c_uint != 0 {
        if mode & 0o100 as libc::c_int as libc::c_uint != 0 {
            's' as i32
        } else {
            'S' as i32
        }
    } else if mode & 0o100 as libc::c_int as libc::c_uint != 0 {
        'x' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    *str
        .offset(
            4 as libc::c_int as isize,
        ) = (if mode & (0o400 as libc::c_int >> 3 as libc::c_int) as libc::c_uint != 0 {
        'r' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    *str
        .offset(
            5 as libc::c_int as isize,
        ) = (if mode & (0o200 as libc::c_int >> 3 as libc::c_int) as libc::c_uint != 0 {
        'w' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    *str
        .offset(
            6 as libc::c_int as isize,
        ) = (if mode & 0o2000 as libc::c_int as libc::c_uint != 0 {
        if mode & (0o100 as libc::c_int >> 3 as libc::c_int) as libc::c_uint != 0 {
            's' as i32
        } else {
            'S' as i32
        }
    } else if mode & (0o100 as libc::c_int >> 3 as libc::c_int) as libc::c_uint != 0 {
        'x' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    *str
        .offset(
            7 as libc::c_int as isize,
        ) = (if mode
        & (0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as libc::c_uint
        != 0
    {
        'r' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    *str
        .offset(
            8 as libc::c_int as isize,
        ) = (if mode
        & (0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as libc::c_uint
        != 0
    {
        'w' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    *str
        .offset(
            9 as libc::c_int as isize,
        ) = (if mode & 0o1000 as libc::c_int as libc::c_uint != 0 {
        if mode
            & (0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                as libc::c_uint != 0
        {
            't' as i32
        } else {
            'T' as i32
        }
    } else if mode
        & (0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as libc::c_uint
        != 0
    {
        'x' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    *str.offset(10 as libc::c_int as isize) = ' ' as i32 as libc::c_char;
    *str.offset(11 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn filemodestring(
    mut statp: *const stat,
    mut str: *mut libc::c_char,
) {
    strmode((*statp).st_mode, str);
    if ((*statp).st_mode).wrapping_sub((*statp).st_mode) != 0 {
        *str.offset(0 as libc::c_int as isize) = 'F' as i32 as libc::c_char;
    } else if ((*statp).st_mode).wrapping_sub((*statp).st_mode) != 0 {
        *str.offset(0 as libc::c_int as isize) = 'Q' as i32 as libc::c_char;
    } else if ((*statp).st_mode).wrapping_sub((*statp).st_mode) != 0 {
        *str.offset(0 as libc::c_int as isize) = 'S' as i32 as libc::c_char;
    }
}
