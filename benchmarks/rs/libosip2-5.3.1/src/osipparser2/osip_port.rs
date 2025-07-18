use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn lrand48() -> libc::c_long;
    fn srand48(__seedval: libc::c_long);
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn openlog(
        __ident: *const libc::c_char,
        __option: libc::c_int,
        __facility: libc::c_int,
    );
    fn syslog(__pri: libc::c_int, __fmt: *const libc::c_char, _: ...);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type osip_malloc_func_t = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
pub type osip_free_func_t = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type osip_realloc_func_t = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
) -> *mut libc::c_void;
pub type _trace_level = libc::c_uint;
pub const END_TRACE_LEVEL: _trace_level = 8;
pub const TRACE_LEVEL7: _trace_level = 7;
pub const TRACE_LEVEL6: _trace_level = 6;
pub const TRACE_LEVEL5: _trace_level = 5;
pub const TRACE_LEVEL4: _trace_level = 4;
pub const TRACE_LEVEL3: _trace_level = 3;
pub const TRACE_LEVEL2: _trace_level = 2;
pub const TRACE_LEVEL1: _trace_level = 1;
pub const TRACE_LEVEL0: _trace_level = 0;
pub type osip_trace_level_t = _trace_level;
pub type osip_trace_func_t = unsafe extern "C" fn(
    *const libc::c_char,
    libc::c_int,
    osip_trace_level_t,
    *const libc::c_char,
    ::std::ffi::VaList,
) -> ();
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
pub static mut logfile: *mut FILE = 0 as *const FILE as *mut FILE;
pub static mut tracing_table: [libc::c_int; 8] = [0; 8];
static mut use_syslog: libc::c_int = 0 as libc::c_int;
static mut trace_func: Option::<osip_trace_func_t> = None;
static mut random_seed_set: libc::c_uint = 0 as libc::c_int as libc::c_uint;
pub static mut osip_malloc_func: Option::<osip_malloc_func_t> = None;
pub static mut osip_realloc_func: Option::<osip_realloc_func_t> = None;
pub static mut osip_free_func: Option::<osip_free_func_t> = None;
pub static mut osip_error_table: [*const libc::c_char; 61] = [
    b"success\0" as *const u8 as *const libc::c_char,
    b"undefined error\0" as *const u8 as *const libc::c_char,
    b"bad parameter\0" as *const u8 as *const libc::c_char,
    b"wrong state\0" as *const u8 as *const libc::c_char,
    b"allocation failure\0" as *const u8 as *const libc::c_char,
    b"syntax error\0" as *const u8 as *const libc::c_char,
    b"not found\0" as *const u8 as *const libc::c_char,
    b"api not initialized\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"no network\0" as *const u8 as *const libc::c_char,
    b"busy port\0" as *const u8 as *const libc::c_char,
    b"unknown host\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"disk full\0" as *const u8 as *const libc::c_char,
    b"no rights\0" as *const u8 as *const libc::c_char,
    b"file not found\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"time out\0" as *const u8 as *const libc::c_char,
    b"too much call\0" as *const u8 as *const libc::c_char,
    b"wrong format\0" as *const u8 as *const libc::c_char,
    b"no common codec\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
    b"undefined\0" as *const u8 as *const libc::c_char,
];
pub unsafe extern "C" fn osip_strerror(mut err: libc::c_int) -> *const libc::c_char {
    if err > 0 as libc::c_int {
        return osip_error_table[0 as libc::c_int as usize];
    }
    if err > -(60 as libc::c_int) {
        return osip_error_table[-err as usize];
    }
    return osip_error_table[59 as libc::c_int as usize];
}
pub unsafe extern "C" fn osip_build_random_number() -> libc::c_uint {
    if random_seed_set == 0 {
        let mut ticks: libc::c_uint = 0;
        let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
        let mut fd: libc::c_int = 0;
        gettimeofday(&mut tv, 0 as *mut libc::c_void);
        ticks = (tv.tv_sec + tv.tv_usec) as libc::c_uint;
        fd = open(
            b"/dev/urandom\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        if fd > 0 as libc::c_int {
            let mut r: libc::c_uint = 0;
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < 512 as libc::c_int {
                read(
                    fd,
                    &mut r as *mut libc::c_uint as *mut libc::c_void,
                    ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                );
                ticks = ticks.wrapping_add(r);
                i += 1;
                i;
            }
            close(fd);
        }
        srand48(ticks as libc::c_long);
        random_seed_set = 1 as libc::c_int as libc::c_uint;
    }
    let mut val: libc::c_int = lrand48() as libc::c_int;
    if val == 0 as libc::c_int {
        let mut ticks_0: libc::c_uint = 0;
        let mut tv_0: timeval = timeval { tv_sec: 0, tv_usec: 0 };
        gettimeofday(&mut tv_0, 0 as *mut libc::c_void);
        ticks_0 = (tv_0.tv_sec + tv_0.tv_usec) as libc::c_uint;
        srand48(ticks_0 as libc::c_long);
        return lrand48() as libc::c_uint;
    }
    return val as libc::c_uint;
}
pub unsafe extern "C" fn osip_strncpy(
    mut dest: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut length: size_t,
) -> *mut libc::c_char {
    strncpy(dest, src, length);
    *dest.offset(length as isize) = '\0' as i32 as libc::c_char;
    return dest;
}
pub unsafe extern "C" fn osip_strdup(mut ch: *const libc::c_char) -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut length: size_t = 0;
    if ch.is_null() {
        return 0 as *mut libc::c_char;
    }
    length = strlen(ch);
    copy = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(length.wrapping_add(1 as libc::c_int as libc::c_ulong))
    } else {
        malloc(length.wrapping_add(1 as libc::c_int as libc::c_ulong))
    }) as *mut libc::c_char;
    if copy.is_null() {
        return 0 as *mut libc::c_char;
    }
    osip_strncpy(copy, ch, length);
    return copy;
}
pub unsafe extern "C" fn osip_atoi(mut number: *const libc::c_char) -> libc::c_int {
    let mut i: libc::c_long = 0;
    if number.is_null() {
        return -(1 as libc::c_int);
    }
    i = strtol(
        number,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    if i == -(9223372036854775807 as libc::c_long) - 1 as libc::c_long
        || i == 9223372036854775807 as libc::c_long
    {
        return -(1 as libc::c_int);
    }
    return i as libc::c_int;
}
pub unsafe extern "C" fn osip_usleep(mut useconds: libc::c_int) {
    let mut delay: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut sec: libc::c_int = 0;
    sec = useconds / 1000000 as libc::c_int;
    if sec > 0 as libc::c_int {
        delay.tv_sec = sec as __time_t;
        delay.tv_usec = 0 as libc::c_int as __suseconds_t;
    } else {
        delay.tv_sec = 0 as libc::c_int as __time_t;
        delay.tv_usec = useconds as __suseconds_t;
    }
    select(
        0 as libc::c_int,
        0 as *mut fd_set,
        0 as *mut fd_set,
        0 as *mut fd_set,
        &mut delay,
    );
}
pub unsafe extern "C" fn osip_strdup_without_quote(
    mut ch: *const libc::c_char,
) -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()((strlen(ch)).wrapping_add(1 as libc::c_int as libc::c_ulong))
    } else {
        malloc((strlen(ch)).wrapping_add(1 as libc::c_int as libc::c_ulong))
    }) as *mut libc::c_char;
    if copy.is_null() {
        return 0 as *mut libc::c_char;
    }
    if *ch as libc::c_int == '"' as i32 {
        osip_strncpy(
            copy,
            ch.offset(1 as libc::c_int as isize),
            strlen(ch.offset(1 as libc::c_int as isize)),
        );
        osip_strncpy(
            copy.offset(strlen(copy) as isize).offset(-(1 as libc::c_int as isize)),
            b"\0\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
    } else {
        osip_strncpy(copy, ch, strlen(ch));
    }
    return copy;
}
pub unsafe extern "C" fn osip_tolower(mut word: *mut libc::c_char) -> libc::c_int {
    while *word != 0 {
        *word = ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *word as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    };
                } else {
                    __res = tolower(*word as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc()).offset(*word as libc::c_int as isize);
            }
            __res
        }) as libc::c_char;
        word = word.offset(1);
        word;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_strcasecmp(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
) -> libc::c_int {
    return strcasecmp(s1, s2);
}
pub unsafe extern "C" fn osip_strncasecmp(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    return strncasecmp(s1, s2, len);
}
pub unsafe extern "C" fn osip_strcasestr(
    mut haystack: *const libc::c_char,
    mut needle: *const libc::c_char,
) -> *mut libc::c_char {
    let mut c: libc::c_char = 0;
    let mut sc: libc::c_char = 0;
    let mut len: size_t = 0;
    let fresh0 = needle;
    needle = needle.offset(1);
    c = *fresh0;
    if c as libc::c_int != 0 as libc::c_int {
        c = ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = c as libc::c_uchar as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    };
                } else {
                    __res = tolower(c as libc::c_uchar as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(c as libc::c_uchar as libc::c_int as isize);
            }
            __res
        }) as libc::c_char;
        len = strlen(needle);
        loop {
            loop {
                let fresh1 = haystack;
                haystack = haystack.offset(1);
                sc = *fresh1;
                if sc as libc::c_int == 0 as libc::c_int {
                    return 0 as *mut libc::c_char;
                }
                if !(({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = sc as libc::c_uchar
                                as libc::c_int;
                            __res = (if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            });
                        } else {
                            __res = tolower(sc as libc::c_uchar as libc::c_int);
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc())
                            .offset(sc as libc::c_uchar as libc::c_int as isize);
                    }
                    __res
                }) as libc::c_char as libc::c_int != c as libc::c_int)
                {
                    break;
                }
            }
            if !(osip_strncasecmp(haystack, needle, len) != 0 as libc::c_int) {
                break;
            }
        }
        haystack = haystack.offset(-1);
        haystack;
    }
    return haystack as *mut libc::c_char;
}
pub unsafe extern "C" fn osip_clrspace(mut word: *mut libc::c_char) -> libc::c_int {
    let mut pbeg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pend: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    if word.is_null() {
        return -(1 as libc::c_int);
    }
    if *word as libc::c_int == '\0' as i32 {
        return 0 as libc::c_int;
    }
    len = strlen(word);
    pbeg = word;
    pbeg = pbeg
        .offset(strspn(pbeg, b" \r\n\t\0" as *const u8 as *const libc::c_char) as isize);
    pend = word.offset(len as isize).offset(-(1 as libc::c_int as isize));
    while ' ' as i32 == *pend as libc::c_int || '\r' as i32 == *pend as libc::c_int
        || '\n' as i32 == *pend as libc::c_int || '\t' as i32 == *pend as libc::c_int
    {
        pend = pend.offset(-1);
        pend;
        if pend < pbeg {
            *word = '\0' as i32 as libc::c_char;
            return 0 as libc::c_int;
        }
    }
    if pend.offset(1 as libc::c_int as isize)
        <= word.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
    {
        *pend.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    }
    if pbeg != word {
        memmove(
            word as *mut libc::c_void,
            pbeg as *const libc::c_void,
            (pend.offset_from(pbeg) as libc::c_long + 2 as libc::c_int as libc::c_long)
                as libc::c_ulong,
        );
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn __osip_set_next_token(
    mut dest: *mut *mut libc::c_char,
    mut buf: *mut libc::c_char,
    mut end_separator: libc::c_int,
    mut next: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut sep: *mut libc::c_char = 0 as *mut libc::c_char;
    *next = 0 as *mut libc::c_char;
    sep = buf;
    while *sep as libc::c_int != end_separator && *sep as libc::c_int != '\0' as i32
        && *sep as libc::c_int != '\r' as i32 && *sep as libc::c_int != '\n' as i32
    {
        sep = sep.offset(1);
        sep;
    }
    if *sep as libc::c_int == '\r' as i32 || *sep as libc::c_int == '\n' as i32 {
        if *sep as libc::c_int != end_separator {
            return -(1 as libc::c_int);
        }
    }
    if *sep as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if sep == buf {
        return -(1 as libc::c_int);
    }
    *dest = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(
            (sep.offset_from(buf) as libc::c_long + 1 as libc::c_int as libc::c_long)
                as size_t,
        )
    } else {
        malloc(
            (sep.offset_from(buf) as libc::c_long + 1 as libc::c_int as libc::c_long)
                as libc::c_ulong,
        )
    }) as *mut libc::c_char;
    if (*dest).is_null() {
        return -(4 as libc::c_int);
    }
    osip_strncpy(*dest, buf, sep.offset_from(buf) as libc::c_long as size_t);
    *next = sep.offset(1 as libc::c_int as isize);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn __osip_quote_find(
    mut qstring: *const libc::c_char,
) -> *const libc::c_char {
    let mut quote: *const libc::c_char = 0 as *const libc::c_char;
    quote = strchr(qstring, '"' as i32);
    if quote == qstring {
        return quote;
    }
    if quote.is_null() {
        return 0 as *const libc::c_char;
    }
    let mut i: libc::c_int = 1 as libc::c_int;
    loop {
        if 0 as libc::c_int
            == strncmp(
                quote.offset(-(i as isize)),
                b"\\\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as libc::c_ulong,
            )
        {
            i += 1;
            i;
        } else {
            if i % 2 as libc::c_int == 1 as libc::c_int {
                return quote;
            }
            quote = strchr(quote.offset(1 as libc::c_int as isize), '"' as i32);
            if quote.is_null() {
                return 0 as *const libc::c_char;
            }
            i = 1 as libc::c_int;
        }
        if quote.offset(-(i as isize)) == qstring.offset(-(1 as libc::c_int as isize)) {
            if *qstring as libc::c_int == '\\' as i32 {
                i += 1;
                i;
            }
            if i % 2 as libc::c_int == 0 as libc::c_int {
                return quote
            } else {
                qstring = quote.offset(1 as libc::c_int as isize);
                quote = strchr(quote.offset(1 as libc::c_int as isize), '"' as i32);
                if quote.is_null() {
                    return 0 as *const libc::c_char;
                }
                i = 1 as libc::c_int;
            }
        }
    };
}
pub unsafe extern "C" fn osip_enquote(mut s: *const libc::c_char) -> *mut libc::c_char {
    let mut rtn: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    rtn = (if osip_malloc_func.is_some() {
        osip_malloc_func
            .unwrap()(
            (strlen(s))
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_add(3 as libc::c_int as libc::c_ulong),
        )
    } else {
        malloc(
            (strlen(s))
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_add(3 as libc::c_int as libc::c_ulong),
        )
    }) as *mut libc::c_char;
    t = rtn;
    if rtn.is_null() {
        return 0 as *mut libc::c_char;
    }
    let fresh2 = t;
    t = t.offset(1);
    *fresh2 = '"' as i32 as libc::c_char;
    while *s as libc::c_int != '\0' as i32 {
        match *s as libc::c_int {
            34 | 92 | 127 => {
                let fresh3 = t;
                t = t.offset(1);
                *fresh3 = '\\' as i32 as libc::c_char;
                let fresh4 = t;
                t = t.offset(1);
                *fresh4 = *s;
            }
            10 | 13 => {
                let fresh5 = t;
                t = t.offset(1);
                *fresh5 = ' ' as i32 as libc::c_char;
            }
            _ => {
                let fresh6 = t;
                t = t.offset(1);
                *fresh6 = *s;
            }
        }
        s = s.offset(1);
        s;
    }
    let fresh7 = t;
    t = t.offset(1);
    *fresh7 = '"' as i32 as libc::c_char;
    let fresh8 = t;
    t = t.offset(1);
    *fresh8 = '\0' as i32 as libc::c_char;
    return rtn;
}
pub unsafe extern "C" fn osip_dequote(mut s: *mut libc::c_char) {
    let mut len: size_t = 0;
    if *s as libc::c_int == '\0' as i32 {
        return;
    }
    if *s as libc::c_int != '"' as i32 {
        return;
    }
    len = strlen(s);
    let fresh9 = len;
    len = len.wrapping_sub(1);
    memmove(
        s as *mut libc::c_void,
        s.offset(1 as libc::c_int as isize) as *const libc::c_void,
        fresh9,
    );
    if len > 0 as libc::c_int as libc::c_ulong
        && *s.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == '"' as i32
    {
        len = len.wrapping_sub(1);
        *s.offset(len as isize) = '\0' as i32 as libc::c_char;
    }
    while *s as libc::c_int != '\0' as i32 {
        if *s as libc::c_int == '\\' as i32 {
            let fresh10 = len;
            len = len.wrapping_sub(1);
            memmove(
                s as *mut libc::c_void,
                s.offset(1 as libc::c_int as isize) as *const libc::c_void,
                fresh10,
            );
        }
        s = s.offset(1);
        s;
        len = len.wrapping_sub(1);
        len;
    }
}
pub unsafe extern "C" fn osip_trace_initialize(
    mut level: osip_trace_level_t,
    mut file: *mut FILE,
) -> libc::c_int {
    let mut i: osip_trace_level_t = TRACE_LEVEL0;
    logfile = 0 as *mut FILE;
    if !file.is_null() {
        logfile = file;
    } else {
        logfile = stdout;
    }
    while (i as libc::c_uint) < END_TRACE_LEVEL as libc::c_int as libc::c_uint {
        if (i as libc::c_uint) < level as libc::c_uint {
            tracing_table[i as usize] = 1 as libc::c_int;
        } else {
            tracing_table[i as usize] = 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_trace_initialize_syslog(
    mut level: osip_trace_level_t,
    mut ident: *mut libc::c_char,
) {
    let mut i: osip_trace_level_t = TRACE_LEVEL0;
    openlog(
        ident,
        0x2 as libc::c_int | 0x1 as libc::c_int,
        (3 as libc::c_int) << 3 as libc::c_int,
    );
    use_syslog = 1 as libc::c_int;
    while (i as libc::c_uint) < END_TRACE_LEVEL as libc::c_int as libc::c_uint {
        if (i as libc::c_uint) < level as libc::c_uint {
            tracing_table[i as usize] = 1 as libc::c_int;
        } else {
            tracing_table[i as usize] = 0 as libc::c_int;
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn osip_trace_enable_until_level(mut level: osip_trace_level_t) {
    let mut i: osip_trace_level_t = TRACE_LEVEL0;
    while (i as libc::c_uint) < END_TRACE_LEVEL as libc::c_int as libc::c_uint {
        if (i as libc::c_uint) < level as libc::c_uint {
            tracing_table[i as usize] = 1 as libc::c_int;
        } else {
            tracing_table[i as usize] = 0 as libc::c_int;
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn osip_trace_initialize_func(
    mut level: osip_trace_level_t,
    mut func: Option::<osip_trace_func_t>,
) {
    let mut i: osip_trace_level_t = TRACE_LEVEL0;
    trace_func = func;
    while (i as libc::c_uint) < END_TRACE_LEVEL as libc::c_int as libc::c_uint {
        if (i as libc::c_uint) < level as libc::c_uint {
            tracing_table[i as usize] = 1 as libc::c_int;
        } else {
            tracing_table[i as usize] = 0 as libc::c_int;
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn osip_trace_enable_level(mut level: osip_trace_level_t) {
    tracing_table[level as usize] = 1 as libc::c_int;
}
pub unsafe extern "C" fn osip_trace_disable_level(mut level: osip_trace_level_t) {
    tracing_table[level as usize] = 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_is_trace_level_activate(
    mut level: osip_trace_level_t,
) -> libc::c_int {
    return tracing_table[level as usize];
}
pub unsafe extern "C" fn osip_trace(
    mut filename_long: *const libc::c_char,
    mut li: libc::c_int,
    mut level: osip_trace_level_t,
    mut f: *mut FILE,
    mut chfr: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    let mut time_buffer: [libc::c_char; 80] = [
        '\0' as i32 as libc::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut fi: *const libc::c_char = 0 as *const libc::c_char;
    if logfile.is_null() && use_syslog == 0 as libc::c_int && trace_func.is_none()
        && f.is_null()
    {
        osip_trace_initialize(TRACE_LEVEL3, stdout);
    }
    if tracing_table[level as usize] == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    let mut timestamp: time_t = 0;
    let mut now: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut ptm: *mut tm = 0 as *mut tm;
    let mut local_tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut tenths_ms: libc::c_int = 0;
    gettimeofday(&mut now, 0 as *mut libc::c_void);
    timestamp = now.tv_sec;
    tenths_ms = (now.tv_usec / 100 as libc::c_long) as libc::c_int;
    ptm = localtime_r(&mut timestamp, &mut local_tm);
    snprintf(
        time_buffer.as_mut_ptr(),
        80 as libc::c_int as libc::c_ulong,
        b"%04d-%02d-%02d %02d:%02d:%02d.%04d\0" as *const u8 as *const libc::c_char,
        1900 as libc::c_int + (*ptm).tm_year,
        (*ptm).tm_mon + 1 as libc::c_int,
        (*ptm).tm_mday,
        (*ptm).tm_hour,
        (*ptm).tm_min,
        (*ptm).tm_sec,
        tenths_ms,
    );
    if !filename_long.is_null() {
        fi = strrchr(filename_long, '/' as i32);
        if fi.is_null() {
            fi = strrchr(filename_long, '\\' as i32);
        }
        if !fi.is_null() {
            fi = fi.offset(1);
            fi;
        }
        if fi.is_null() {
            fi = filename_long;
        }
    }
    if f.is_null() {
        f = logfile;
    }
    ap = args.clone();
    if trace_func.is_some() {
        trace_func.unwrap()(fi, li, level, chfr, ap.as_va_list());
        return 0 as libc::c_int;
    }
    let mut buffer: [libc::c_char; 2024] = [0; 2024];
    let mut in_0: libc::c_int = 0 as libc::c_int;
    memset(
        buffer.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_char; 2024]>() as libc::c_ulong,
    );
    if level as libc::c_uint == TRACE_LEVEL0 as libc::c_int as libc::c_uint {
        in_0 = snprintf(
            buffer.as_mut_ptr(),
            (2024 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
            b"| FATAL | %s <%10.10s:%5i> \0" as *const u8 as *const libc::c_char,
            time_buffer.as_mut_ptr(),
            fi,
            li,
        );
    } else if level as libc::c_uint == TRACE_LEVEL1 as libc::c_int as libc::c_uint {
        in_0 = snprintf(
            buffer.as_mut_ptr(),
            (2024 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
            b"|  BUG  | %s <%10.10s:%5i> \0" as *const u8 as *const libc::c_char,
            time_buffer.as_mut_ptr(),
            fi,
            li,
        );
    } else if level as libc::c_uint == TRACE_LEVEL2 as libc::c_int as libc::c_uint {
        in_0 = snprintf(
            buffer.as_mut_ptr(),
            (2024 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
            b"| ERROR | %s <%10.10s:%5i> \0" as *const u8 as *const libc::c_char,
            time_buffer.as_mut_ptr(),
            fi,
            li,
        );
    } else if level as libc::c_uint == TRACE_LEVEL3 as libc::c_int as libc::c_uint {
        in_0 = snprintf(
            buffer.as_mut_ptr(),
            (2024 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
            b"|WARNING| %s <%10.10s:%5i> \0" as *const u8 as *const libc::c_char,
            time_buffer.as_mut_ptr(),
            fi,
            li,
        );
    } else if level as libc::c_uint == TRACE_LEVEL4 as libc::c_int as libc::c_uint {
        in_0 = snprintf(
            buffer.as_mut_ptr(),
            (2024 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
            b"| INFO1 | %s <%10.10s:%5i> \0" as *const u8 as *const libc::c_char,
            time_buffer.as_mut_ptr(),
            fi,
            li,
        );
    } else if level as libc::c_uint == TRACE_LEVEL5 as libc::c_int as libc::c_uint {
        in_0 = snprintf(
            buffer.as_mut_ptr(),
            (2024 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
            b"| INFO2 | %s <%10.10s:%5i> \0" as *const u8 as *const libc::c_char,
            time_buffer.as_mut_ptr(),
            fi,
            li,
        );
    } else if level as libc::c_uint == TRACE_LEVEL6 as libc::c_int as libc::c_uint {
        in_0 = snprintf(
            buffer.as_mut_ptr(),
            (2024 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
            b"| INFO3 | %s <%10.10s:%5i> \0" as *const u8 as *const libc::c_char,
            time_buffer.as_mut_ptr(),
            fi,
            li,
        );
    } else if level as libc::c_uint == TRACE_LEVEL7 as libc::c_int as libc::c_uint {
        in_0 = snprintf(
            buffer.as_mut_ptr(),
            (2024 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
            b"| INFO4 | %s <%10.10s:%5i> \0" as *const u8 as *const libc::c_char,
            time_buffer.as_mut_ptr(),
            fi,
            li,
        );
    }
    vsnprintf(
        buffer.as_mut_ptr().offset(in_0 as isize),
        (2024 as libc::c_int - 1 as libc::c_int - in_0) as libc::c_ulong,
        chfr,
        ap.as_va_list(),
    );
    if use_syslog == 1 as libc::c_int {
        if level as libc::c_uint == TRACE_LEVEL0 as libc::c_int as libc::c_uint {
            syslog(
                3 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                buffer.as_mut_ptr(),
            );
        } else if level as libc::c_uint == TRACE_LEVEL1 as libc::c_int as libc::c_uint {
            syslog(
                3 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                buffer.as_mut_ptr(),
            );
        } else if level as libc::c_uint == TRACE_LEVEL2 as libc::c_int as libc::c_uint {
            syslog(
                3 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                buffer.as_mut_ptr(),
            );
        } else if level as libc::c_uint == TRACE_LEVEL3 as libc::c_int as libc::c_uint {
            syslog(
                4 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                buffer.as_mut_ptr(),
            );
        } else if level as libc::c_uint == TRACE_LEVEL4 as libc::c_int as libc::c_uint {
            syslog(
                6 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                buffer.as_mut_ptr(),
            );
        } else if level as libc::c_uint == TRACE_LEVEL5 as libc::c_int as libc::c_uint {
            syslog(
                6 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                buffer.as_mut_ptr(),
            );
        } else if level as libc::c_uint == TRACE_LEVEL6 as libc::c_int as libc::c_uint {
            syslog(
                7 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                buffer.as_mut_ptr(),
            );
        } else if level as libc::c_uint == TRACE_LEVEL7 as libc::c_int as libc::c_uint {
            syslog(
                7 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                buffer.as_mut_ptr(),
            );
        }
        return 0 as libc::c_int;
    }
    if !f.is_null() {
        fprintf(f, b"%s\0" as *const u8 as *const libc::c_char, buffer.as_mut_ptr());
        fflush(f);
        return 0 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_set_allocators(
    mut malloc_func: Option::<osip_malloc_func_t>,
    mut realloc_func: Option::<osip_realloc_func_t>,
    mut free_func: Option::<osip_free_func_t>,
) {
    osip_malloc_func = malloc_func;
    osip_realloc_func = realloc_func;
    osip_free_func = free_func;
}
pub unsafe extern "C" fn osip_hash(mut str: *const libc::c_char) -> libc::c_ulong {
    let mut hash: libc::c_uint = 5381 as libc::c_int as libc::c_uint;
    let mut c: libc::c_int = 0;
    loop {
        let fresh11 = str;
        str = str.offset(1);
        c = *fresh11 as libc::c_int;
        if !(c != 0) {
            break;
        }
        hash = (hash << 5 as libc::c_int)
            .wrapping_add(hash)
            .wrapping_add(c as libc::c_uint);
    }
    return (hash & 0xffffffff as libc::c_uint) as libc::c_ulong;
}
pub unsafe extern "C" fn osip_str_append(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
) -> *mut libc::c_char {
    while *src as libc::c_int != '\0' as i32 {
        *dst = *src;
        src = src.offset(1);
        src;
        dst = dst.offset(1);
        dst;
    }
    *dst = '\0' as i32 as libc::c_char;
    return dst;
}
pub unsafe extern "C" fn osip_strn_append(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut len: size_t,
) -> *mut libc::c_char {
    memmove(dst as *mut libc::c_void, src as *mut libc::c_void, len);
    dst = dst.offset(len as isize);
    *dst = '\0' as i32 as libc::c_char;
    return dst;
}
pub unsafe extern "C" fn osip_clrncpy(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut len: size_t,
) -> *mut libc::c_char {
    let mut pbeg: *const libc::c_char = 0 as *const libc::c_char;
    let mut pend: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut spaceless_length: size_t = 0;
    if src.is_null() || len == 0 as libc::c_int as libc::c_ulong {
        *dst = '\0' as i32 as libc::c_char;
        return 0 as *mut libc::c_char;
    }
    pbeg = src;
    pbeg = pbeg
        .offset(strspn(pbeg, b" \r\n\t\0" as *const u8 as *const libc::c_char) as isize);
    pend = src.offset(len as isize).offset(-(1 as libc::c_int as isize));
    while ' ' as i32 == *pend as libc::c_int || '\r' as i32 == *pend as libc::c_int
        || '\n' as i32 == *pend as libc::c_int || '\t' as i32 == *pend as libc::c_int
    {
        pend = pend.offset(-1);
        pend;
        if pend < pbeg {
            *dst = '\0' as i32 as libc::c_char;
            return dst;
        }
    }
    spaceless_length = (pend.offset_from(pbeg) as libc::c_long
        + 1 as libc::c_int as libc::c_long) as size_t;
    memmove(dst as *mut libc::c_void, pbeg as *const libc::c_void, spaceless_length);
    p = dst.offset(spaceless_length as isize);
    *p = '\0' as i32 as libc::c_char;
    spaceless_length = spaceless_length.wrapping_add(1);
    if (spaceless_length < len) as libc::c_int as libc::c_long != 0 {
        loop {
            p = p.offset(1);
            *p = '\0' as i32 as libc::c_char;
            spaceless_length = spaceless_length.wrapping_add(1);
            if !(spaceless_length < len) {
                break;
            }
        }
    }
    return dst;
}
