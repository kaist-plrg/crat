use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn exit(_: libc::c_int) -> !;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn perror(__s: *const libc::c_char);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn getrusage(__who: __rusage_who_t, __usage: *mut rusage) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm128_t {
    pub x: uint64_t,
    pub y: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rusage {
    pub ru_utime: timeval,
    pub ru_stime: timeval,
    pub c2rust_unnamed: C2RustUnnamed_12,
    pub c2rust_unnamed_0: C2RustUnnamed_11,
    pub c2rust_unnamed_1: C2RustUnnamed_10,
    pub c2rust_unnamed_2: C2RustUnnamed_9,
    pub c2rust_unnamed_3: C2RustUnnamed_8,
    pub c2rust_unnamed_4: C2RustUnnamed_7,
    pub c2rust_unnamed_5: C2RustUnnamed_6,
    pub c2rust_unnamed_6: C2RustUnnamed_5,
    pub c2rust_unnamed_7: C2RustUnnamed_4,
    pub c2rust_unnamed_8: C2RustUnnamed_3,
    pub c2rust_unnamed_9: C2RustUnnamed_2,
    pub c2rust_unnamed_10: C2RustUnnamed_1,
    pub c2rust_unnamed_11: C2RustUnnamed_0,
    pub c2rust_unnamed_12: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ru_nivcsw: libc::c_long,
    pub __ru_nivcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ru_nvcsw: libc::c_long,
    pub __ru_nvcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub ru_nsignals: libc::c_long,
    pub __ru_nsignals_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub ru_msgrcv: libc::c_long,
    pub __ru_msgrcv_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub ru_msgsnd: libc::c_long,
    pub __ru_msgsnd_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub ru_oublock: libc::c_long,
    pub __ru_oublock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub ru_inblock: libc::c_long,
    pub __ru_inblock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub ru_nswap: libc::c_long,
    pub __ru_nswap_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub ru_majflt: libc::c_long,
    pub __ru_majflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub ru_minflt: libc::c_long,
    pub __ru_minflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub ru_isrss: libc::c_long,
    pub __ru_isrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub ru_idrss: libc::c_long,
    pub __ru_idrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub ru_ixrss: libc::c_long,
    pub __ru_ixrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub ru_maxrss: libc::c_long,
    pub __ru_maxrss_word: __syscall_slong_t,
}
pub const RUSAGE_SELF: __rusage_who = 0;
pub type __rusage_who_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsbucket_128x_t {
    pub b: *mut mm128_t,
    pub e: *mut mm128_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsbucket_64_t {
    pub b: *mut uint64_t,
    pub e: *mut uint64_t,
}
pub type __rusage_who = libc::c_int;
pub const RUSAGE_CHILDREN: __rusage_who = -1;
pub static mut mm_verbose: libc::c_int = 1 as libc::c_int;
pub static mut mm_dbg_flag: libc::c_int = 0 as libc::c_int;
pub static mut mm_realtime0: libc::c_double = 0.;
pub unsafe extern "C" fn cputime() -> libc::c_double {
    let mut r: rusage = rusage {
        ru_utime: timeval { tv_sec: 0, tv_usec: 0 },
        ru_stime: timeval { tv_sec: 0, tv_usec: 0 },
        c2rust_unnamed: C2RustUnnamed_12 { ru_maxrss: 0 },
        c2rust_unnamed_0: C2RustUnnamed_11 { ru_ixrss: 0 },
        c2rust_unnamed_1: C2RustUnnamed_10 { ru_idrss: 0 },
        c2rust_unnamed_2: C2RustUnnamed_9 { ru_isrss: 0 },
        c2rust_unnamed_3: C2RustUnnamed_8 { ru_minflt: 0 },
        c2rust_unnamed_4: C2RustUnnamed_7 { ru_majflt: 0 },
        c2rust_unnamed_5: C2RustUnnamed_6 { ru_nswap: 0 },
        c2rust_unnamed_6: C2RustUnnamed_5 { ru_inblock: 0 },
        c2rust_unnamed_7: C2RustUnnamed_4 { ru_oublock: 0 },
        c2rust_unnamed_8: C2RustUnnamed_3 { ru_msgsnd: 0 },
        c2rust_unnamed_9: C2RustUnnamed_2 { ru_msgrcv: 0 },
        c2rust_unnamed_10: C2RustUnnamed_1 { ru_nsignals: 0 },
        c2rust_unnamed_11: C2RustUnnamed_0 { ru_nvcsw: 0 },
        c2rust_unnamed_12: C2RustUnnamed { ru_nivcsw: 0 },
    };
    getrusage(RUSAGE_SELF as libc::c_int, &mut r);
    return (r.ru_utime.tv_sec + r.ru_stime.tv_sec) as libc::c_double
        + 1e-6f64 * (r.ru_utime.tv_usec + r.ru_stime.tv_usec) as libc::c_double;
}
pub unsafe extern "C" fn peakrss() -> libc::c_long {
    let mut r: rusage = rusage {
        ru_utime: timeval { tv_sec: 0, tv_usec: 0 },
        ru_stime: timeval { tv_sec: 0, tv_usec: 0 },
        c2rust_unnamed: C2RustUnnamed_12 { ru_maxrss: 0 },
        c2rust_unnamed_0: C2RustUnnamed_11 { ru_ixrss: 0 },
        c2rust_unnamed_1: C2RustUnnamed_10 { ru_idrss: 0 },
        c2rust_unnamed_2: C2RustUnnamed_9 { ru_isrss: 0 },
        c2rust_unnamed_3: C2RustUnnamed_8 { ru_minflt: 0 },
        c2rust_unnamed_4: C2RustUnnamed_7 { ru_majflt: 0 },
        c2rust_unnamed_5: C2RustUnnamed_6 { ru_nswap: 0 },
        c2rust_unnamed_6: C2RustUnnamed_5 { ru_inblock: 0 },
        c2rust_unnamed_7: C2RustUnnamed_4 { ru_oublock: 0 },
        c2rust_unnamed_8: C2RustUnnamed_3 { ru_msgsnd: 0 },
        c2rust_unnamed_9: C2RustUnnamed_2 { ru_msgrcv: 0 },
        c2rust_unnamed_10: C2RustUnnamed_1 { ru_nsignals: 0 },
        c2rust_unnamed_11: C2RustUnnamed_0 { ru_nvcsw: 0 },
        c2rust_unnamed_12: C2RustUnnamed { ru_nivcsw: 0 },
    };
    getrusage(RUSAGE_SELF as libc::c_int, &mut r);
    return r.c2rust_unnamed.ru_maxrss * 1024 as libc::c_int as libc::c_long;
}
pub unsafe extern "C" fn realtime() -> libc::c_double {
    let mut tp: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut tp, 0 as *mut libc::c_void);
    return tp.tv_sec as libc::c_double + tp.tv_usec as libc::c_double * 1e-6f64;
}
pub unsafe extern "C" fn mm_err_puts(mut str: *const libc::c_char) {
    let mut ret: libc::c_int = 0;
    ret = puts(str);
    if ret == -(1 as libc::c_int) {
        perror(
            b"[ERROR] failed to write the results\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
}
pub unsafe extern "C" fn mm_err_fwrite(
    mut p: *const libc::c_void,
    mut size: size_t,
    mut nitems: size_t,
    mut fp: *mut FILE,
) {
    let mut ret: libc::c_int = 0;
    ret = fwrite(p, size, nitems, fp) as libc::c_int;
    if ret == -(1 as libc::c_int) {
        perror(b"[ERROR] failed to write data\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
}
pub unsafe extern "C" fn mm_err_fread(
    mut p: *mut libc::c_void,
    mut size: size_t,
    mut nitems: size_t,
    mut fp: *mut FILE,
) {
    let mut ret: libc::c_int = 0;
    ret = fread(p, size, nitems, fp) as libc::c_int;
    if ret == -(1 as libc::c_int) {
        perror(b"[ERROR] failed to read data\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
}
pub unsafe extern "C" fn rs_sort_128x(
    mut beg: *mut mm128_t,
    mut end: *mut mm128_t,
    mut n_bits: libc::c_int,
    mut s: libc::c_int,
) {
    let mut i: *mut mm128_t = 0 as *mut mm128_t;
    let mut size: libc::c_int = (1 as libc::c_int) << n_bits;
    let mut m: libc::c_int = size - 1 as libc::c_int;
    let mut k: *mut rsbucket_128x_t = 0 as *mut rsbucket_128x_t;
    let mut b: [rsbucket_128x_t; 256] = [rsbucket_128x_t {
        b: 0 as *mut mm128_t,
        e: 0 as *mut mm128_t,
    }; 256];
    let mut be: *mut rsbucket_128x_t = b.as_mut_ptr().offset(size as isize);
    if n_bits <= 8 as libc::c_int {} else {
        __assert_fail(
            b"n_bits <= RS_MAX_BITS\0" as *const u8 as *const libc::c_char,
            b"misc.c\0" as *const u8 as *const libc::c_char,
            156 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"void rs_sort_128x(mm128_t *, mm128_t *, int, int)\0"))
                .as_ptr(),
        );
    }
    'c_3864: {
        if n_bits <= 8 as libc::c_int {} else {
            __assert_fail(
                b"n_bits <= RS_MAX_BITS\0" as *const u8 as *const libc::c_char,
                b"misc.c\0" as *const u8 as *const libc::c_char,
                156 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"void rs_sort_128x(mm128_t *, mm128_t *, int, int)\0"))
                    .as_ptr(),
            );
        }
    };
    k = b.as_mut_ptr();
    while k != be {
        (*k).e = beg;
        (*k).b = (*k).e;
        k = k.offset(1);
        k;
    }
    i = beg;
    while i != end {
        b[((*i).x >> s & m as libc::c_ulong) as usize]
            .e = (b[((*i).x >> s & m as libc::c_ulong) as usize].e).offset(1);
        b[((*i).x >> s & m as libc::c_ulong) as usize].e;
        i = i.offset(1);
        i;
    }
    k = b.as_mut_ptr().offset(1 as libc::c_int as isize);
    while k != be {
        (*k)
            .e = ((*k).e)
            .offset(
                ((*k.offset(-(1 as libc::c_int as isize))).e).offset_from(beg)
                    as libc::c_long as isize,
            );
        (*k).b = (*k.offset(-(1 as libc::c_int as isize))).e;
        k = k.offset(1);
        k;
    }
    k = b.as_mut_ptr();
    while k != be {
        if (*k).b != (*k).e {
            let mut l: *mut rsbucket_128x_t = 0 as *mut rsbucket_128x_t;
            l = b.as_mut_ptr().offset(((*(*k).b).x >> s & m as libc::c_ulong) as isize);
            if l != k {
                let mut tmp: mm128_t = *(*k).b;
                let mut swap: mm128_t = mm128_t { x: 0, y: 0 };
                loop {
                    swap = tmp;
                    tmp = *(*l).b;
                    let fresh0 = (*l).b;
                    (*l).b = ((*l).b).offset(1);
                    *fresh0 = swap;
                    l = b
                        .as_mut_ptr()
                        .offset((tmp.x >> s & m as libc::c_ulong) as isize);
                    if !(l != k) {
                        break;
                    }
                }
                let fresh1 = (*k).b;
                (*k).b = ((*k).b).offset(1);
                *fresh1 = tmp;
            } else {
                (*k).b = ((*k).b).offset(1);
                (*k).b;
            }
        } else {
            k = k.offset(1);
            k;
        }
    }
    let ref mut fresh2 = (*b.as_mut_ptr()).b;
    *fresh2 = beg;
    k = b.as_mut_ptr().offset(1 as libc::c_int as isize);
    while k != be {
        (*k).b = (*k.offset(-(1 as libc::c_int as isize))).e;
        k = k.offset(1);
        k;
    }
    if s != 0 {
        s = if s > n_bits { s - n_bits } else { 0 as libc::c_int };
        k = b.as_mut_ptr();
        while k != be {
            if ((*k).e).offset_from((*k).b) as libc::c_long
                > 64 as libc::c_int as libc::c_long
            {
                rs_sort_128x((*k).b, (*k).e, n_bits, s);
            } else if ((*k).e).offset_from((*k).b) as libc::c_long
                > 1 as libc::c_int as libc::c_long
            {
                rs_insertsort_128x((*k).b, (*k).e);
            }
            k = k.offset(1);
            k;
        }
    }
}
pub unsafe extern "C" fn radix_sort_128x(mut beg: *mut mm128_t, mut end: *mut mm128_t) {
    if end.offset_from(beg) as libc::c_long <= 64 as libc::c_int as libc::c_long {
        rs_insertsort_128x(beg, end);
    } else {
        rs_sort_128x(
            beg,
            end,
            8 as libc::c_int,
            (8 as libc::c_int - 1 as libc::c_int) * 8 as libc::c_int,
        );
    };
}
pub unsafe extern "C" fn rs_insertsort_128x(
    mut beg: *mut mm128_t,
    mut end: *mut mm128_t,
) {
    let mut i: *mut mm128_t = 0 as *mut mm128_t;
    i = beg.offset(1 as libc::c_int as isize);
    while i < end {
        if (*i).x < (*i.offset(-(1 as libc::c_int as isize))).x {
            let mut j: *mut mm128_t = 0 as *mut mm128_t;
            let mut tmp: mm128_t = *i;
            j = i;
            while j > beg && tmp.x < (*j.offset(-(1 as libc::c_int as isize))).x {
                *j = *j.offset(-(1 as libc::c_int as isize));
                j = j.offset(-1);
                j;
            }
            *j = tmp;
        }
        i = i.offset(1);
        i;
    }
}
pub unsafe extern "C" fn rs_sort_64(
    mut beg: *mut uint64_t,
    mut end: *mut uint64_t,
    mut n_bits: libc::c_int,
    mut s: libc::c_int,
) {
    let mut i: *mut uint64_t = 0 as *mut uint64_t;
    let mut size: libc::c_int = (1 as libc::c_int) << n_bits;
    let mut m: libc::c_int = size - 1 as libc::c_int;
    let mut k: *mut rsbucket_64_t = 0 as *mut rsbucket_64_t;
    let mut b: [rsbucket_64_t; 256] = [rsbucket_64_t {
        b: 0 as *mut uint64_t,
        e: 0 as *mut uint64_t,
    }; 256];
    let mut be: *mut rsbucket_64_t = b.as_mut_ptr().offset(size as isize);
    if n_bits <= 8 as libc::c_int {} else {
        __assert_fail(
            b"n_bits <= RS_MAX_BITS\0" as *const u8 as *const libc::c_char,
            b"misc.c\0" as *const u8 as *const libc::c_char,
            159 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"void rs_sort_64(uint64_t *, uint64_t *, int, int)\0"))
                .as_ptr(),
        );
    }
    'c_4414: {
        if n_bits <= 8 as libc::c_int {} else {
            __assert_fail(
                b"n_bits <= RS_MAX_BITS\0" as *const u8 as *const libc::c_char,
                b"misc.c\0" as *const u8 as *const libc::c_char,
                159 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"void rs_sort_64(uint64_t *, uint64_t *, int, int)\0"))
                    .as_ptr(),
            );
        }
    };
    k = b.as_mut_ptr();
    while k != be {
        (*k).e = beg;
        (*k).b = (*k).e;
        k = k.offset(1);
        k;
    }
    i = beg;
    while i != end {
        b[(*i >> s & m as libc::c_ulong) as usize]
            .e = (b[(*i >> s & m as libc::c_ulong) as usize].e).offset(1);
        b[(*i >> s & m as libc::c_ulong) as usize].e;
        i = i.offset(1);
        i;
    }
    k = b.as_mut_ptr().offset(1 as libc::c_int as isize);
    while k != be {
        (*k)
            .e = ((*k).e)
            .offset(
                ((*k.offset(-(1 as libc::c_int as isize))).e).offset_from(beg)
                    as libc::c_long as isize,
            );
        (*k).b = (*k.offset(-(1 as libc::c_int as isize))).e;
        k = k.offset(1);
        k;
    }
    k = b.as_mut_ptr();
    while k != be {
        if (*k).b != (*k).e {
            let mut l: *mut rsbucket_64_t = 0 as *mut rsbucket_64_t;
            l = b.as_mut_ptr().offset((*(*k).b >> s & m as libc::c_ulong) as isize);
            if l != k {
                let mut tmp: uint64_t = *(*k).b;
                let mut swap: uint64_t = 0;
                loop {
                    swap = tmp;
                    tmp = *(*l).b;
                    let fresh3 = (*l).b;
                    (*l).b = ((*l).b).offset(1);
                    *fresh3 = swap;
                    l = b.as_mut_ptr().offset((tmp >> s & m as libc::c_ulong) as isize);
                    if !(l != k) {
                        break;
                    }
                }
                let fresh4 = (*k).b;
                (*k).b = ((*k).b).offset(1);
                *fresh4 = tmp;
            } else {
                (*k).b = ((*k).b).offset(1);
                (*k).b;
            }
        } else {
            k = k.offset(1);
            k;
        }
    }
    let ref mut fresh5 = (*b.as_mut_ptr()).b;
    *fresh5 = beg;
    k = b.as_mut_ptr().offset(1 as libc::c_int as isize);
    while k != be {
        (*k).b = (*k.offset(-(1 as libc::c_int as isize))).e;
        k = k.offset(1);
        k;
    }
    if s != 0 {
        s = if s > n_bits { s - n_bits } else { 0 as libc::c_int };
        k = b.as_mut_ptr();
        while k != be {
            if ((*k).e).offset_from((*k).b) as libc::c_long
                > 64 as libc::c_int as libc::c_long
            {
                rs_sort_64((*k).b, (*k).e, n_bits, s);
            } else if ((*k).e).offset_from((*k).b) as libc::c_long
                > 1 as libc::c_int as libc::c_long
            {
                rs_insertsort_64((*k).b, (*k).e);
            }
            k = k.offset(1);
            k;
        }
    }
}
pub unsafe extern "C" fn radix_sort_64(mut beg: *mut uint64_t, mut end: *mut uint64_t) {
    if end.offset_from(beg) as libc::c_long <= 64 as libc::c_int as libc::c_long {
        rs_insertsort_64(beg, end);
    } else {
        rs_sort_64(
            beg,
            end,
            8 as libc::c_int,
            (8 as libc::c_int - 1 as libc::c_int) * 8 as libc::c_int,
        );
    };
}
pub unsafe extern "C" fn rs_insertsort_64(
    mut beg: *mut uint64_t,
    mut end: *mut uint64_t,
) {
    let mut i: *mut uint64_t = 0 as *mut uint64_t;
    i = beg.offset(1 as libc::c_int as isize);
    while i < end {
        if *i < *i.offset(-(1 as libc::c_int as isize)) {
            let mut j: *mut uint64_t = 0 as *mut uint64_t;
            let mut tmp: uint64_t = *i;
            j = i;
            while j > beg && tmp < *j.offset(-(1 as libc::c_int as isize)) {
                *j = *j.offset(-(1 as libc::c_int as isize));
                j = j.offset(-1);
                j;
            }
            *j = tmp;
        }
        i = i.offset(1);
        i;
    }
}
pub unsafe extern "C" fn ks_heapdown_uint32_t(
    mut i: size_t,
    mut n: size_t,
    mut l: *mut uint32_t,
) {
    let mut k: size_t = i;
    let mut tmp: uint32_t = *l.offset(i as isize);
    loop {
        k = (k << 1 as libc::c_int).wrapping_add(1 as libc::c_int as libc::c_ulong);
        if !(k < n) {
            break;
        }
        if k != n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && *l.offset(k as isize)
                < *l.offset(k.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
        {
            k = k.wrapping_add(1);
            k;
        }
        if *l.offset(k as isize) < tmp {
            break;
        }
        *l.offset(i as isize) = *l.offset(k as isize);
        i = k;
    }
    *l.offset(i as isize) = tmp;
}
pub unsafe extern "C" fn ks_heapmake_uint32_t(mut lsize: size_t, mut l: *mut uint32_t) {
    let mut i: size_t = 0;
    i = (lsize >> 1 as libc::c_int).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    while i != -(1 as libc::c_int) as size_t {
        ks_heapdown_uint32_t(i, lsize, l);
        i = i.wrapping_sub(1);
        i;
    }
}
pub unsafe extern "C" fn ks_ksmall_uint32_t(
    mut n: size_t,
    mut arr: *mut uint32_t,
    mut kk: size_t,
) -> uint32_t {
    let mut low: *mut uint32_t = 0 as *mut uint32_t;
    let mut high: *mut uint32_t = 0 as *mut uint32_t;
    let mut k: *mut uint32_t = 0 as *mut uint32_t;
    let mut ll: *mut uint32_t = 0 as *mut uint32_t;
    let mut hh: *mut uint32_t = 0 as *mut uint32_t;
    let mut mid: *mut uint32_t = 0 as *mut uint32_t;
    low = arr;
    high = arr.offset(n as isize).offset(-(1 as libc::c_int as isize));
    k = arr.offset(kk as isize);
    loop {
        if high <= low {
            return *k;
        }
        if high == low.offset(1 as libc::c_int as isize) {
            if *high < *low {
                let mut t: uint32_t = *low;
                *low = *high;
                *high = t;
            }
            return *k;
        }
        mid = low
            .offset(
                (high.offset_from(low) as libc::c_long
                    / 2 as libc::c_int as libc::c_long) as isize,
            );
        if *high < *mid {
            let mut t_0: uint32_t = *mid;
            *mid = *high;
            *high = t_0;
        }
        if *high < *low {
            let mut t_1: uint32_t = *low;
            *low = *high;
            *high = t_1;
        }
        if *low < *mid {
            let mut t_2: uint32_t = *mid;
            *mid = *low;
            *low = t_2;
        }
        let mut t_3: uint32_t = *mid;
        *mid = *low.offset(1 as libc::c_int as isize);
        *low.offset(1 as libc::c_int as isize) = t_3;
        ll = low.offset(1 as libc::c_int as isize);
        hh = high;
        loop {
            loop {
                ll = ll.offset(1);
                ll;
                if !(*ll < *low) {
                    break;
                }
            }
            loop {
                hh = hh.offset(-1);
                hh;
                if !(*low < *hh) {
                    break;
                }
            }
            if hh < ll {
                break;
            }
            let mut t_4: uint32_t = *ll;
            *ll = *hh;
            *hh = t_4;
        }
        let mut t_5: uint32_t = *low;
        *low = *hh;
        *hh = t_5;
        if hh <= k {
            low = ll;
        }
        if hh >= k {
            high = hh.offset(-(1 as libc::c_int as isize));
        }
    };
}
pub unsafe extern "C" fn ks_heapdown_uint64_t(
    mut i: size_t,
    mut n: size_t,
    mut l: *mut uint64_t,
) {
    let mut k: size_t = i;
    let mut tmp: uint64_t = *l.offset(i as isize);
    loop {
        k = (k << 1 as libc::c_int).wrapping_add(1 as libc::c_int as libc::c_ulong);
        if !(k < n) {
            break;
        }
        if k != n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && *l.offset(k as isize)
                < *l.offset(k.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
        {
            k = k.wrapping_add(1);
            k;
        }
        if *l.offset(k as isize) < tmp {
            break;
        }
        *l.offset(i as isize) = *l.offset(k as isize);
        i = k;
    }
    *l.offset(i as isize) = tmp;
}
pub unsafe extern "C" fn ks_heapmake_uint64_t(mut lsize: size_t, mut l: *mut uint64_t) {
    let mut i: size_t = 0;
    i = (lsize >> 1 as libc::c_int).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    while i != -(1 as libc::c_int) as size_t {
        ks_heapdown_uint64_t(i, lsize, l);
        i = i.wrapping_sub(1);
        i;
    }
}
pub unsafe extern "C" fn ks_ksmall_uint64_t(
    mut n: size_t,
    mut arr: *mut uint64_t,
    mut kk: size_t,
) -> uint64_t {
    let mut low: *mut uint64_t = 0 as *mut uint64_t;
    let mut high: *mut uint64_t = 0 as *mut uint64_t;
    let mut k: *mut uint64_t = 0 as *mut uint64_t;
    let mut ll: *mut uint64_t = 0 as *mut uint64_t;
    let mut hh: *mut uint64_t = 0 as *mut uint64_t;
    let mut mid: *mut uint64_t = 0 as *mut uint64_t;
    low = arr;
    high = arr.offset(n as isize).offset(-(1 as libc::c_int as isize));
    k = arr.offset(kk as isize);
    loop {
        if high <= low {
            return *k;
        }
        if high == low.offset(1 as libc::c_int as isize) {
            if *high < *low {
                let mut t: uint64_t = *low;
                *low = *high;
                *high = t;
            }
            return *k;
        }
        mid = low
            .offset(
                (high.offset_from(low) as libc::c_long
                    / 2 as libc::c_int as libc::c_long) as isize,
            );
        if *high < *mid {
            let mut t_0: uint64_t = *mid;
            *mid = *high;
            *high = t_0;
        }
        if *high < *low {
            let mut t_1: uint64_t = *low;
            *low = *high;
            *high = t_1;
        }
        if *low < *mid {
            let mut t_2: uint64_t = *mid;
            *mid = *low;
            *low = t_2;
        }
        let mut t_3: uint64_t = *mid;
        *mid = *low.offset(1 as libc::c_int as isize);
        *low.offset(1 as libc::c_int as isize) = t_3;
        ll = low.offset(1 as libc::c_int as isize);
        hh = high;
        loop {
            loop {
                ll = ll.offset(1);
                ll;
                if !(*ll < *low) {
                    break;
                }
            }
            loop {
                hh = hh.offset(-1);
                hh;
                if !(*low < *hh) {
                    break;
                }
            }
            if hh < ll {
                break;
            }
            let mut t_4: uint64_t = *ll;
            *ll = *hh;
            *hh = t_4;
        }
        let mut t_5: uint64_t = *low;
        *low = *hh;
        *hh = t_5;
        if hh <= k {
            low = ll;
        }
        if hh >= k {
            high = hh.offset(-(1 as libc::c_int as isize));
        }
    };
}
