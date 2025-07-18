use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn free(__ptr: *mut libc::c_void);
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn xcalloc(count: size_t, size: size_t) -> *mut libc::c_void;
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
    fn geteuid() -> __uid_t;
    fn setuid(__uid: __uid_t) -> libc::c_int;
    fn pthread_setaffinity_np(
        __th: pthread_t,
        __cpusetsize: size_t,
        __cpuset: *const cpu_set_t,
    ) -> libc::c_int;
    fn pthread_self() -> pthread_t;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn log_fatal(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> !;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type uint32_t = __uint32_t;
pub type macaddr_t = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpu_set_t {
    pub __bits: [__cpu_mask; 16],
}
pub type __cpu_mask = libc::c_ulong;
pub type pthread_t = libc::c_ulong;
pub unsafe extern "C" fn max_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    if a >= b {
        return a;
    }
    return b;
}
pub unsafe extern "C" fn enforce_range(
    mut name: *const libc::c_char,
    mut v: libc::c_int,
    mut min: libc::c_int,
    mut max: libc::c_int,
) {
    if check_range(v, min, max) == 1 as libc::c_int {
        log_fatal(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"argument `%s' must be between %d and %d\n\0" as *const u8
                as *const libc::c_char,
            name,
            min,
            max,
        );
    }
}
pub unsafe extern "C" fn split_string(
    mut in_0: *const libc::c_char,
    mut len: *mut libc::c_int,
    mut results: *mut *mut *const libc::c_char,
) {
    let mut fields: *mut *const libc::c_char = xcalloc(
        128 as libc::c_int as size_t,
        ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
    ) as *mut *const libc::c_char;
    let mut retvlen: libc::c_int = 0 as libc::c_int;
    let mut currloc: *const libc::c_char = in_0;
    loop {
        if retvlen < 128 as libc::c_int {} else {
            __assert_fail(
                b"retvlen < MAX_SPLITS\0" as *const u8 as *const libc::c_char,
                b"util.c\0" as *const u8 as *const libc::c_char,
                62 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"void split_string(const char *, int *, const char ***)\0"))
                    .as_ptr(),
            );
        }
        'c_1463: {
            if retvlen < 128 as libc::c_int {} else {
                __assert_fail(
                    b"retvlen < MAX_SPLITS\0" as *const u8 as *const libc::c_char,
                    b"util.c\0" as *const u8 as *const libc::c_char,
                    62 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 55],
                        &[libc::c_char; 55],
                    >(b"void split_string(const char *, int *, const char ***)\0"))
                        .as_ptr(),
                );
            }
        };
        let mut len_0: size_t = strcspn(
            currloc,
            b", \0" as *const u8 as *const libc::c_char,
        );
        if len_0 == 0 as libc::c_int as libc::c_ulong {
            currloc = currloc.offset(1);
            currloc;
        } else {
            let mut new: *mut libc::c_char = xmalloc(
                len_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            strncpy(new, currloc, len_0);
            *new.offset(len_0 as isize) = '\0' as i32 as libc::c_char;
            let fresh0 = retvlen;
            retvlen = retvlen + 1;
            let ref mut fresh1 = *fields.offset(fresh0 as isize);
            *fresh1 = new;
            if !(*fields.offset((retvlen - 1 as libc::c_int) as isize)).is_null()
            {} else {
                __assert_fail(
                    b"fields[retvlen - 1]\0" as *const u8 as *const libc::c_char,
                    b"util.c\0" as *const u8 as *const libc::c_char,
                    71 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 55],
                        &[libc::c_char; 55],
                    >(b"void split_string(const char *, int *, const char ***)\0"))
                        .as_ptr(),
                );
            }
            'c_1355: {
                if !(*fields.offset((retvlen - 1 as libc::c_int) as isize)).is_null()
                {} else {
                    __assert_fail(
                        b"fields[retvlen - 1]\0" as *const u8 as *const libc::c_char,
                        b"util.c\0" as *const u8 as *const libc::c_char,
                        71 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 55],
                            &[libc::c_char; 55],
                        >(b"void split_string(const char *, int *, const char ***)\0"))
                            .as_ptr(),
                    );
                }
            };
        }
        if len_0 == strlen(currloc) {
            break;
        }
        currloc = currloc.offset(len_0 as isize);
    }
    *results = fields;
    *len = retvlen;
}
pub unsafe extern "C" fn fprintw(
    mut f: *mut FILE,
    mut s: *const libc::c_char,
    mut w: size_t,
) {
    if strlen(s) <= w {
        fprintf(f, b"%s\0" as *const u8 as *const libc::c_char, s);
        return;
    }
    let mut news: *mut libc::c_char = strdup(s);
    let mut pch: *mut libc::c_char = strtok(
        news,
        b"\n\0" as *const u8 as *const libc::c_char,
    );
    while !pch.is_null() {
        if strlen(pch) <= w {
            printf(b"%s\n\0" as *const u8 as *const libc::c_char, pch);
            pch = strtok(
                0 as *mut libc::c_char,
                b"\n\0" as *const u8 as *const libc::c_char,
            );
        } else {
            let mut t: *mut libc::c_char = pch;
            while strlen(t) != 0 {
                let mut numchars: size_t = 0 as libc::c_int as size_t;
                let mut tmp: *mut libc::c_char = t;
                loop {
                    let mut new: size_t = (strcspn(
                        tmp,
                        b" \0" as *const u8 as *const libc::c_char,
                    ))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong);
                    if new == strlen(tmp) || new > w {
                        numchars = (numchars as libc::c_ulong).wrapping_add(new)
                            as size_t as size_t;
                        break;
                    } else {
                        if numchars.wrapping_add(new) > w {
                            break;
                        }
                        tmp = tmp.offset(new as isize);
                        numchars = (numchars as libc::c_ulong).wrapping_add(new)
                            as size_t as size_t;
                    }
                }
                fprintf(
                    f,
                    b"%.*s\n\0" as *const u8 as *const libc::c_char,
                    numchars as libc::c_int,
                    t,
                );
                t = t.offset(numchars as isize);
                if t > pch.offset(strlen(pch) as isize) {
                    break;
                }
            }
            pch = strtok(
                0 as *mut libc::c_char,
                b"\n\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    free(news as *mut libc::c_void);
}
pub unsafe extern "C" fn parse_max_hosts(
    mut max_targets: *mut libc::c_char,
) -> uint32_t {
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    *__errno_location() = 0 as libc::c_int;
    let mut v: libc::c_double = strtod(max_targets, &mut end);
    if end == max_targets || *__errno_location() != 0 as libc::c_int {
        log_fatal(
            b"argparse\0" as *const u8 as *const libc::c_char,
            b"can't convert max-targets to a number\0" as *const u8
                as *const libc::c_char,
        );
    }
    if *end.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32
        && *end.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        v = v
            * ((1 as libc::c_int as libc::c_ulonglong) << 32 as libc::c_int)
                as libc::c_double / 100.0f64;
    } else if *end.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
        log_fatal(
            b"eargparse\0" as *const u8 as *const libc::c_char,
            b"extra characters after max-targets\0" as *const u8 as *const libc::c_char,
        );
    }
    if v <= 0 as libc::c_int as libc::c_double {
        return 0 as libc::c_int as uint32_t
    } else if v
        >= ((1 as libc::c_int as libc::c_ulonglong) << 32 as libc::c_int)
            as libc::c_double
    {
        return 0xffffffff as libc::c_uint
    } else {
        return v as uint32_t
    };
}
pub unsafe extern "C" fn time_string(
    mut time: uint32_t,
    mut est: libc::c_int,
    mut buf: *mut libc::c_char,
    mut len: size_t,
) {
    let mut y: libc::c_int = time.wrapping_div(31556736 as libc::c_int as libc::c_uint)
        as libc::c_int;
    let mut d: libc::c_int = time
        .wrapping_rem(31556736 as libc::c_int as libc::c_uint)
        .wrapping_div(86400 as libc::c_int as libc::c_uint) as libc::c_int;
    let mut h: libc::c_int = time
        .wrapping_rem(86400 as libc::c_int as libc::c_uint)
        .wrapping_div(3600 as libc::c_int as libc::c_uint) as libc::c_int;
    let mut m: libc::c_int = time
        .wrapping_rem(3600 as libc::c_int as libc::c_uint)
        .wrapping_div(60 as libc::c_int as libc::c_uint) as libc::c_int;
    let mut s: libc::c_int = time.wrapping_rem(60 as libc::c_int as libc::c_uint)
        as libc::c_int;
    if est != 0 {
        if y > 0 as libc::c_int {
            snprintf(buf, len, b"%d years\0" as *const u8 as *const libc::c_char, y);
        } else if d > 9 as libc::c_int {
            snprintf(buf, len, b"%dd\0" as *const u8 as *const libc::c_char, d);
        } else if d > 0 as libc::c_int {
            snprintf(buf, len, b"%dd%02dh\0" as *const u8 as *const libc::c_char, d, h);
        } else if h > 9 as libc::c_int {
            snprintf(buf, len, b"%dh\0" as *const u8 as *const libc::c_char, h);
        } else if h > 0 as libc::c_int {
            snprintf(buf, len, b"%dh%02dm\0" as *const u8 as *const libc::c_char, h, m);
        } else if m > 9 as libc::c_int {
            snprintf(buf, len, b"%dm\0" as *const u8 as *const libc::c_char, m);
        } else if m > 0 as libc::c_int {
            snprintf(buf, len, b"%dm%02ds\0" as *const u8 as *const libc::c_char, m, s);
        } else {
            snprintf(buf, len, b"%ds\0" as *const u8 as *const libc::c_char, s);
        }
    } else if d > 0 as libc::c_int {
        snprintf(
            buf,
            len,
            b"%dd%d:%02d:%02d\0" as *const u8 as *const libc::c_char,
            d,
            h,
            m,
            s,
        );
    } else if h > 0 as libc::c_int {
        snprintf(
            buf,
            len,
            b"%d:%02d:%02d\0" as *const u8 as *const libc::c_char,
            h,
            m,
            s,
        );
    } else {
        snprintf(buf, len, b"%d:%02d\0" as *const u8 as *const libc::c_char, m, s);
    };
}
pub unsafe extern "C" fn number_string(
    mut n: uint32_t,
    mut buf: *mut libc::c_char,
    mut len: size_t,
) {
    let mut figs: libc::c_int = 0 as libc::c_int;
    if n < 1000 as libc::c_int as libc::c_uint {
        snprintf(buf, len, b"%u \0" as *const u8 as *const libc::c_char, n);
    } else if n < 1000000 as libc::c_int as libc::c_uint {
        if n < 10000 as libc::c_int as libc::c_uint {
            figs = 2 as libc::c_int;
        } else if n < 100000 as libc::c_int as libc::c_uint {
            figs = 1 as libc::c_int;
        }
        snprintf(
            buf,
            len,
            b"%0.*f K\0" as *const u8 as *const libc::c_char,
            figs,
            n as libc::c_float as libc::c_double / 1000.0f64,
        );
    } else {
        if figs < 10000000 as libc::c_int {
            figs = 2 as libc::c_int;
        } else if figs < 100000000 as libc::c_int {
            figs = 1 as libc::c_int;
        }
        snprintf(
            buf,
            len,
            b"%0.*f M\0" as *const u8 as *const libc::c_char,
            figs,
            n as libc::c_float as libc::c_double / 1000000.0f64,
        );
    };
}
pub unsafe extern "C" fn parse_mac(
    mut out: *mut macaddr_t,
    mut in_0: *mut libc::c_char,
) -> libc::c_int {
    if strlen(in_0)
        < (6 as libc::c_int * 3 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    let mut octet: [libc::c_char; 4] = [0; 4];
    octet[2 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        if i < 6 as libc::c_int - 1 as libc::c_int
            && *in_0.offset((i * 3 as libc::c_int + 2 as libc::c_int) as isize)
                as libc::c_int != ':' as i32
        {
            return 0 as libc::c_int;
        }
        strncpy(
            octet.as_mut_ptr(),
            &mut *in_0.offset((i * 3 as libc::c_int) as isize),
            2 as libc::c_int as libc::c_ulong,
        );
        let mut err: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut b: libc::c_long = strtol(
            octet.as_mut_ptr(),
            &mut err,
            16 as libc::c_int,
        );
        if !err.is_null() && *err as libc::c_int != '\0' as i32 {
            return 0 as libc::c_int;
        }
        *out.offset(i as isize) = (b & 0xff as libc::c_int as libc::c_long) as macaddr_t;
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn check_range(
    mut v: libc::c_int,
    mut min: libc::c_int,
    mut max: libc::c_int,
) -> libc::c_int {
    if v < min || v > max {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn file_exists(mut name: *mut libc::c_char) -> libc::c_int {
    let mut file: *mut FILE = fopen(name, b"r\0" as *const u8 as *const libc::c_char);
    if file.is_null() {
        return 0 as libc::c_int;
    }
    fclose(file);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn drop_privs() -> libc::c_int {
    let mut pw: *mut passwd = 0 as *mut passwd;
    if geteuid() != 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    pw = getpwnam(b"nobody\0" as *const u8 as *const libc::c_char);
    if !pw.is_null() {
        if setuid((*pw).pw_uid) == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn set_cpu(mut core: uint32_t) -> libc::c_int {
    let mut cpuset: cpu_set_t = cpu_set_t { __bits: [0; 16] };
    libc::memset(
        &mut cpuset as *mut cpu_set_t as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong as libc::size_t,
    );
    let mut __cpu: size_t = core as size_t;
    if __cpu.wrapping_div(8 as libc::c_int as libc::c_ulong)
        < ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong
    {
        let ref mut fresh2 = *(cpuset.__bits)
            .as_mut_ptr()
            .offset(
                __cpu
                    .wrapping_div(
                        (8 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                            ),
                    ) as isize,
            );
        *fresh2
            |= (1 as libc::c_int as __cpu_mask)
                << __cpu
                    .wrapping_rem(
                        (8 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                            ),
                    );
    } else {};
    if pthread_setaffinity_np(
        pthread_self(),
        ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong,
        &mut cpuset,
    ) != 0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
