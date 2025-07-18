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
unsafe extern "C" fn ftypelet(mut bits: libc::c_long) -> libc::c_char {
    if bits & 0o170000 as libc::c_int as libc::c_long
        == 0o60000 as libc::c_int as libc::c_long
    {
        return 'b' as i32 as libc::c_char;
    }
    if bits & 0o170000 as libc::c_int as libc::c_long
        == 0o20000 as libc::c_int as libc::c_long
    {
        return 'c' as i32 as libc::c_char;
    }
    if bits & 0o170000 as libc::c_int as libc::c_long
        == 0o40000 as libc::c_int as libc::c_long
    {
        return 'd' as i32 as libc::c_char;
    }
    if bits & 0o170000 as libc::c_int as libc::c_long
        == 0o100000 as libc::c_int as libc::c_long
    {
        return '-' as i32 as libc::c_char;
    }
    if bits & 0o170000 as libc::c_int as libc::c_long
        == 0o10000 as libc::c_int as libc::c_long
    {
        return 'p' as i32 as libc::c_char;
    }
    if bits & 0o170000 as libc::c_int as libc::c_long
        == 0o120000 as libc::c_int as libc::c_long
    {
        return 'l' as i32 as libc::c_char;
    }
    if bits & 0o170000 as libc::c_int as libc::c_long
        == 0o140000 as libc::c_int as libc::c_long
    {
        return 's' as i32 as libc::c_char;
    }
    return '?' as i32 as libc::c_char;
}
unsafe extern "C" fn rwx(mut bits: libc::c_ushort, mut chars: *mut libc::c_char) {
    *chars
        .offset(
            0 as libc::c_int as isize,
        ) = (if bits as libc::c_int & 0o400 as libc::c_int != 0 {
        'r' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    *chars
        .offset(
            1 as libc::c_int as isize,
        ) = (if bits as libc::c_int & 0o200 as libc::c_int != 0 {
        'w' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    *chars
        .offset(
            2 as libc::c_int as isize,
        ) = (if bits as libc::c_int & 0o100 as libc::c_int != 0 {
        'x' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
}
unsafe extern "C" fn setst(mut bits: libc::c_ushort, mut chars: *mut libc::c_char) {
    if bits as libc::c_int & 0o4000 as libc::c_int != 0 {
        if *chars.offset(3 as libc::c_int as isize) as libc::c_int != 'x' as i32 {
            *chars.offset(3 as libc::c_int as isize) = 'S' as i32 as libc::c_char;
        } else {
            *chars.offset(3 as libc::c_int as isize) = 's' as i32 as libc::c_char;
        }
    }
    if bits as libc::c_int & 0o2000 as libc::c_int != 0 {
        if *chars.offset(6 as libc::c_int as isize) as libc::c_int != 'x' as i32 {
            *chars.offset(6 as libc::c_int as isize) = 'S' as i32 as libc::c_char;
        } else {
            *chars.offset(6 as libc::c_int as isize) = 's' as i32 as libc::c_char;
        }
    }
    if bits as libc::c_int & 0o1000 as libc::c_int != 0 {
        if *chars.offset(9 as libc::c_int as isize) as libc::c_int != 'x' as i32 {
            *chars.offset(9 as libc::c_int as isize) = 'T' as i32 as libc::c_char;
        } else {
            *chars.offset(9 as libc::c_int as isize) = 't' as i32 as libc::c_char;
        }
    }
}
pub unsafe extern "C" fn mode_string(
    mut mode: libc::c_uint,
    mut str: *mut libc::c_char,
) {
    *str.offset(0 as libc::c_int as isize) = ftypelet(mode as libc::c_long);
    rwx(
        ((mode & 0o700 as libc::c_int as libc::c_uint) << 0 as libc::c_int)
            as libc::c_ushort,
        &mut *str.offset(1 as libc::c_int as isize),
    );
    rwx(
        ((mode & 0o70 as libc::c_int as libc::c_uint) << 3 as libc::c_int)
            as libc::c_ushort,
        &mut *str.offset(4 as libc::c_int as isize),
    );
    rwx(
        ((mode & 0o7 as libc::c_int as libc::c_uint) << 6 as libc::c_int)
            as libc::c_ushort,
        &mut *str.offset(7 as libc::c_int as isize),
    );
    setst(mode as libc::c_ushort, str);
}
pub unsafe extern "C" fn filemodestring(
    mut statp: *mut stat,
    mut str: *mut libc::c_char,
) {
    mode_string((*statp).st_mode, str);
}
