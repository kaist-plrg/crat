use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn getrandom(
        __buffer: *mut libc::c_void,
        __length: size_t,
        __flags: libc::c_uint,
    ) -> ssize_t;
    fn clock() -> clock_t;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __lxstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type clock_t = __clock_t;
pub type clockid_t = __clockid_t;
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
pub type random_value = uint_fast64_t;
pub type uint_fast64_t = libc::c_ulong;
#[inline]
unsafe extern "C" fn lstat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __lxstat(1 as libc::c_int, __path, __statbuf);
}
unsafe extern "C" fn mix_random_values(
    mut r: random_value,
    mut s: random_value,
) -> random_value {
    return (2862933555777941757 as libc::c_long as libc::c_ulong)
        .wrapping_mul(r)
        .wrapping_add(3037000493 as libc::c_long as libc::c_ulong) ^ s;
}
unsafe extern "C" fn random_bits(mut r: *mut random_value, mut s: random_value) -> bool {
    if getrandom(
        r as *mut libc::c_void,
        ::std::mem::size_of::<random_value>() as libc::c_ulong,
        0x1 as libc::c_int as libc::c_uint,
    ) as libc::c_ulong == ::std::mem::size_of::<random_value>() as libc::c_ulong
    {
        return 1 as libc::c_int != 0;
    }
    let mut v: random_value = s;
    let mut tv: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(0 as libc::c_int, &mut tv);
    v = mix_random_values(v, tv.tv_sec as random_value);
    v = mix_random_values(v, tv.tv_nsec as random_value);
    *r = mix_random_values(v, clock() as random_value);
    return 0 as libc::c_int != 0;
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
static mut letters: [libc::c_char; 63] = unsafe {
    *::std::mem::transmute::<
        &[u8; 63],
        &[libc::c_char; 63],
    >(b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789\0")
};
pub unsafe extern "C" fn gen_tempname_len(
    mut tmpl: *mut libc::c_char,
    mut suffixlen: libc::c_int,
    mut flags: libc::c_int,
    mut kind: libc::c_int,
    mut x_suffix_len: size_t,
) -> libc::c_int {
    static mut tryfunc: [Option::<
        unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_void) -> libc::c_int,
    >; 3] = unsafe {
        [
            Some(
                try_file
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            Some(
                try_dir
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            Some(
                try_nocreate
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ]
    };
    return try_tempname_len(
        tmpl,
        suffixlen,
        &mut flags as *mut libc::c_int as *mut libc::c_void,
        tryfunc[kind as usize],
        x_suffix_len,
    );
}
pub unsafe extern "C" fn try_tempname_len(
    mut tmpl: *mut libc::c_char,
    mut suffixlen: libc::c_int,
    mut args: *mut libc::c_void,
    mut tryfunc: Option::<
        unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_void) -> libc::c_int,
    >,
    mut x_suffix_len: size_t,
) -> libc::c_int {
    let mut len: size_t = 0;
    let mut XXXXXX: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut count: libc::c_uint = 0;
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut save_errno: libc::c_int = *__errno_location();
    let mut attempts: libc::c_uint = (62 as libc::c_int * 62 as libc::c_int
        * 62 as libc::c_int) as libc::c_uint;
    let mut v: random_value = 0 as libc::c_int as random_value;
    let mut vdigbuf: random_value = 0;
    let mut vdigits: libc::c_int = 0 as libc::c_int;
    let biased_min: random_value = (18446744073709551615 as libc::c_ulong
        as libc::c_ulonglong)
        .wrapping_sub(
            (18446744073709551615 as libc::c_ulong as libc::c_ulonglong)
                .wrapping_rem(
                    (62 as libc::c_longlong * 62 as libc::c_int as libc::c_longlong
                        * 62 as libc::c_int as libc::c_longlong
                        * 62 as libc::c_int as libc::c_longlong
                        * 62 as libc::c_int as libc::c_longlong
                        * 62 as libc::c_int as libc::c_longlong
                        * 62 as libc::c_int as libc::c_longlong
                        * 62 as libc::c_int as libc::c_longlong
                        * 62 as libc::c_int as libc::c_longlong
                        * 62 as libc::c_int as libc::c_longlong) as libc::c_ulonglong,
                ),
        ) as random_value;
    len = strlen(tmpl);
    if len < x_suffix_len.wrapping_add(suffixlen as libc::c_ulong)
        || strspn(
            &mut *tmpl
                .offset(
                    len
                        .wrapping_sub(x_suffix_len)
                        .wrapping_sub(suffixlen as libc::c_ulong) as isize,
                ),
            b"X\0" as *const u8 as *const libc::c_char,
        ) < x_suffix_len
    {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    XXXXXX = &mut *tmpl
        .offset(
            len.wrapping_sub(x_suffix_len).wrapping_sub(suffixlen as libc::c_ulong)
                as isize,
        ) as *mut libc::c_char;
    count = 0 as libc::c_int as libc::c_uint;
    while count < attempts {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < x_suffix_len {
            if vdigits == 0 as libc::c_int {
                while random_bits(&mut v, v) as libc::c_int != 0 && biased_min <= v {}
                vdigbuf = v;
                vdigits = 10 as libc::c_int;
            }
            *XXXXXX
                .offset(
                    i as isize,
                ) = letters[vdigbuf.wrapping_rem(62 as libc::c_int as libc::c_ulong)
                as usize];
            vdigbuf = (vdigbuf as libc::c_ulong)
                .wrapping_div(62 as libc::c_int as libc::c_ulong) as random_value
                as random_value;
            vdigits -= 1;
            vdigits;
            i = i.wrapping_add(1);
            i;
        }
        fd = tryfunc.unwrap()(tmpl, args);
        if fd >= 0 as libc::c_int {
            *__errno_location() = save_errno;
            return fd;
        } else if *__errno_location() != 17 as libc::c_int {
            return -(1 as libc::c_int)
        }
        count = count.wrapping_add(1);
        count;
    }
    *__errno_location() = 17 as libc::c_int;
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn gen_tempname(
    mut tmpl: *mut libc::c_char,
    mut suffixlen: libc::c_int,
    mut flags: libc::c_int,
    mut kind: libc::c_int,
) -> libc::c_int {
    return gen_tempname_len(tmpl, suffixlen, flags, kind, 6 as libc::c_int as size_t);
}
pub unsafe extern "C" fn try_tempname(
    mut tmpl: *mut libc::c_char,
    mut suffixlen: libc::c_int,
    mut args: *mut libc::c_void,
    mut tryfunc: Option::<
        unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_void) -> libc::c_int,
    >,
) -> libc::c_int {
    return try_tempname_len(tmpl, suffixlen, args, tryfunc, 6 as libc::c_int as size_t);
}
