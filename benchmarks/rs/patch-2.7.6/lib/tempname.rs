use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn abort() -> !;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __lxstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
}
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uint64_t = __uint64_t;
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
unsafe extern "C" fn lstat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __lxstat(1 as libc::c_int, __path, __statbuf);
}
static mut letters: [libc::c_char; 63] = unsafe {
    *::std::mem::transmute::<
        &[u8; 63],
        &[libc::c_char; 63],
    >(b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789\0")
};
pub unsafe extern "C" fn try_tempname(
    mut tmpl: *mut libc::c_char,
    mut suffixlen: libc::c_int,
    mut args: *mut libc::c_void,
    mut tryfunc: Option::<
        unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_void) -> libc::c_int,
    >,
) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut XXXXXX: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut value: uint64_t = 0;
    let mut random_time_bits: uint64_t = 0;
    let mut count: libc::c_uint = 0;
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut save_errno: libc::c_int = *__errno_location();
    let mut attempts: libc::c_uint = (62 as libc::c_int * 62 as libc::c_int
        * 62 as libc::c_int) as libc::c_uint;
    len = strlen(tmpl) as libc::c_int;
    if len < 6 as libc::c_int + suffixlen
        || memcmp(
            &mut *tmpl.offset((len - 6 as libc::c_int - suffixlen) as isize)
                as *mut libc::c_char as *const libc::c_void,
            b"XXXXXX\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            6 as libc::c_int as libc::c_ulong,
        ) != 0
    {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    XXXXXX = &mut *tmpl.offset((len - 6 as libc::c_int - suffixlen) as isize)
        as *mut libc::c_char;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    random_time_bits = (tv.tv_usec as uint64_t) << 16 as libc::c_int
        ^ tv.tv_sec as libc::c_ulong;
    value = (value as libc::c_ulong)
        .wrapping_add(random_time_bits ^ getpid() as libc::c_ulong) as uint64_t
        as uint64_t;
    count = 0 as libc::c_int as libc::c_uint;
    while count < attempts {
        let mut v: uint64_t = value;
        *XXXXXX
            .offset(
                0 as libc::c_int as isize,
            ) = letters[v.wrapping_rem(62 as libc::c_int as libc::c_ulong) as usize];
        v = (v as libc::c_ulong).wrapping_div(62 as libc::c_int as libc::c_ulong)
            as uint64_t as uint64_t;
        *XXXXXX
            .offset(
                1 as libc::c_int as isize,
            ) = letters[v.wrapping_rem(62 as libc::c_int as libc::c_ulong) as usize];
        v = (v as libc::c_ulong).wrapping_div(62 as libc::c_int as libc::c_ulong)
            as uint64_t as uint64_t;
        *XXXXXX
            .offset(
                2 as libc::c_int as isize,
            ) = letters[v.wrapping_rem(62 as libc::c_int as libc::c_ulong) as usize];
        v = (v as libc::c_ulong).wrapping_div(62 as libc::c_int as libc::c_ulong)
            as uint64_t as uint64_t;
        *XXXXXX
            .offset(
                3 as libc::c_int as isize,
            ) = letters[v.wrapping_rem(62 as libc::c_int as libc::c_ulong) as usize];
        v = (v as libc::c_ulong).wrapping_div(62 as libc::c_int as libc::c_ulong)
            as uint64_t as uint64_t;
        *XXXXXX
            .offset(
                4 as libc::c_int as isize,
            ) = letters[v.wrapping_rem(62 as libc::c_int as libc::c_ulong) as usize];
        v = (v as libc::c_ulong).wrapping_div(62 as libc::c_int as libc::c_ulong)
            as uint64_t as uint64_t;
        *XXXXXX
            .offset(
                5 as libc::c_int as isize,
            ) = letters[v.wrapping_rem(62 as libc::c_int as libc::c_ulong) as usize];
        fd = tryfunc.unwrap()(tmpl, args);
        if fd >= 0 as libc::c_int {
            *__errno_location() = save_errno;
            return fd;
        } else if *__errno_location() != 17 as libc::c_int {
            return -(1 as libc::c_int)
        }
        value = (value as libc::c_ulong)
            .wrapping_add(7777 as libc::c_int as libc::c_ulong) as uint64_t as uint64_t;
        count = count.wrapping_add(1);
        count;
    }
    *__errno_location() = 17 as libc::c_int;
    return -(1 as libc::c_int);
}
unsafe extern "C" fn try_file(
    mut tmpl: *mut libc::c_char,
    mut flags: *mut libc::c_void,
) -> libc::c_int {
    let mut openflags: *mut libc::c_int = flags as *mut libc::c_int;
    return open(
        tmpl,
        *openflags & !(0o3 as libc::c_int) | 0o2 as libc::c_int | 0o100 as libc::c_int
            | 0o200 as libc::c_int,
        0o400 as libc::c_int | 0o200 as libc::c_int,
    );
}
unsafe extern "C" fn try_dir(
    mut tmpl: *mut libc::c_char,
    mut flags: *mut libc::c_void,
) -> libc::c_int {
    return mkdir(
        tmpl,
        (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int) as __mode_t,
    );
}
unsafe extern "C" fn try_nocreate(
    mut tmpl: *mut libc::c_char,
    mut flags: *mut libc::c_void,
) -> libc::c_int {
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
    if lstat(tmpl, &mut st) == 0 as libc::c_int
        || *__errno_location() == 75 as libc::c_int
    {
        *__errno_location() = 17 as libc::c_int;
    }
    return if *__errno_location() == 2 as libc::c_int {
        0 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
}
pub unsafe extern "C" fn gen_tempname(
    mut tmpl: *mut libc::c_char,
    mut suffixlen: libc::c_int,
    mut flags: libc::c_int,
    mut kind: libc::c_int,
) -> libc::c_int {
    let mut tryfunc: Option::<
        unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_void) -> libc::c_int,
    > = None;
    match kind {
        0 => {
            tryfunc = Some(
                try_file
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            );
        }
        1 => {
            tryfunc = Some(
                try_dir
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            );
        }
        2 => {
            tryfunc = Some(
                try_nocreate
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            );
        }
        _ => {
            if (b"invalid KIND in __gen_tempname\0" as *const u8 as *const libc::c_char)
                .is_null()
            {} else {
                __assert_fail(
                    b"! \"invalid KIND in __gen_tempname\"\0" as *const u8
                        as *const libc::c_char,
                    b"tempname.c\0" as *const u8 as *const libc::c_char,
                    319 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 40],
                        &[libc::c_char; 40],
                    >(b"int gen_tempname(char *, int, int, int)\0"))
                        .as_ptr(),
                );
            }
            'c_1866: {
                if (b"invalid KIND in __gen_tempname\0" as *const u8
                    as *const libc::c_char)
                    .is_null()
                {} else {
                    __assert_fail(
                        b"! \"invalid KIND in __gen_tempname\"\0" as *const u8
                            as *const libc::c_char,
                        b"tempname.c\0" as *const u8 as *const libc::c_char,
                        319 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 40],
                            &[libc::c_char; 40],
                        >(b"int gen_tempname(char *, int, int, int)\0"))
                            .as_ptr(),
                    );
                }
            };
            abort();
        }
    }
    return try_tempname(
        tmpl,
        suffixlen,
        &mut flags as *mut libc::c_int as *mut libc::c_void,
        tryfunc,
    );
}
