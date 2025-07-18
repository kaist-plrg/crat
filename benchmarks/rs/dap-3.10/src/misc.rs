use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn finite(_: libc::c_double) -> libc::c_int;
    fn rint(_: libc::c_double) -> libc::c_double;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn dap_free(ptr: *mut libc::c_void, mesg: *mut libc::c_char);
    fn dap_malloc(nbytes: libc::c_int, mesg: *mut libc::c_char) -> *mut libc::c_char;
    static mut dap_maxiter: libc::c_int;
    static mut dap_err: *mut FILE;
    static mut dap_log: *mut FILE;
}
pub type size_t = libc::c_ulong;
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
#[inline]
unsafe extern "C" fn getchar() -> libc::c_int {
    return getc(stdin);
}
static mut mdays: [libc::c_int; 13] = [
    0 as libc::c_int,
    31 as libc::c_int,
    28 as libc::c_int,
    31 as libc::c_int,
    30 as libc::c_int,
    31 as libc::c_int,
    30 as libc::c_int,
    31 as libc::c_int,
    31 as libc::c_int,
    30 as libc::c_int,
    31 as libc::c_int,
    30 as libc::c_int,
    31 as libc::c_int,
];
pub unsafe extern "C" fn dap_numdate(mut date: *mut libc::c_char) -> libc::c_int {
    let mut d: libc::c_int = 0;
    let mut dday: libc::c_int = 0;
    let mut dyr: libc::c_int = 0;
    let mut mon: libc::c_int = 0;
    let mut day: libc::c_int = 0;
    let mut yr: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut ndays: libc::c_int = 0;
    d = 0 as libc::c_int;
    mon = 0 as libc::c_int;
    while d < 2 as libc::c_int && '0' as i32 <= *date.offset(d as isize) as libc::c_int
        && *date.offset(d as isize) as libc::c_int <= '9' as i32
    {
        mon = 10 as libc::c_int * mon + *date.offset(d as isize) as libc::c_int
            - '0' as i32;
        d += 1;
        d;
    }
    if *date.offset(d as isize) as libc::c_int == '/' as i32 {
        d += 1;
        d;
    }
    dday = d;
    day = 0 as libc::c_int;
    while d < dday + 2 as libc::c_int
        && '0' as i32 <= *date.offset(d as isize) as libc::c_int
        && *date.offset(d as isize) as libc::c_int <= '9' as i32
    {
        day = 10 as libc::c_int * day + *date.offset(d as isize) as libc::c_int
            - '0' as i32;
        d += 1;
        d;
    }
    if *date.offset(d as isize) as libc::c_int == '/' as i32 {
        d += 1;
        d;
    }
    dyr = d;
    yr = 0 as libc::c_int;
    while d < dyr + 4 as libc::c_int
        && '0' as i32 <= *date.offset(d as isize) as libc::c_int
        && *date.offset(d as isize) as libc::c_int <= '9' as i32
    {
        yr = 10 as libc::c_int * yr + *date.offset(d as isize) as libc::c_int
            - '0' as i32;
        d += 1;
        d;
    }
    if d < dyr + 4 as libc::c_int {
        return -(1 as libc::c_int);
    }
    m = 1 as libc::c_int;
    ndays = day;
    while m < mon {
        ndays += mdays[m as usize];
        m += 1;
        m;
    }
    if mon > 2 as libc::c_int && yr % 4 as libc::c_int == 0
        && yr % 100 as libc::c_int != 0
    {
        ndays += 1;
        ndays;
    }
    if yr < 1752 as libc::c_int {
        return -(1 as libc::c_int);
    }
    y = 1752 as libc::c_int;
    while y < yr {
        ndays += 365 as libc::c_int;
        if y % 4 as libc::c_int == 0
            && (y % 100 as libc::c_int != 0 || y % 400 as libc::c_int == 0)
        {
            ndays += 1;
            ndays;
        }
        y += 1;
        y;
    }
    return ndays;
}
pub unsafe extern "C" fn dap_datenum(mut n: libc::c_int, mut d: *mut libc::c_char) {
    let mut mon: libc::c_int = 0;
    let mut day: libc::c_int = 0;
    let mut yr: libc::c_int = 0;
    let mut ndays: libc::c_int = 0;
    if n <= 0 as libc::c_int {
        strcpy(d, b"?\0" as *const u8 as *const libc::c_char);
        return;
    }
    yr = 1752 as libc::c_int;
    sprintf(d, b"0101%4d\0" as *const u8 as *const libc::c_char, yr);
    ndays = dap_numdate(d);
    while ndays <= n {
        if yr < 10000 as libc::c_int {
            yr += 1;
            sprintf(d, b"0101%4d\0" as *const u8 as *const libc::c_char, yr);
            ndays = dap_numdate(d);
        } else {
            strcpy(d, b"?\0" as *const u8 as *const libc::c_char);
            return;
        }
    }
    yr -= 1;
    yr;
    mon = 1 as libc::c_int;
    sprintf(d, b"%02d01%4d\0" as *const u8 as *const libc::c_char, mon, yr);
    ndays = dap_numdate(d);
    while ndays <= n {
        mon += 1;
        sprintf(d, b"%02d01%4d\0" as *const u8 as *const libc::c_char, mon, yr);
        if !(mon <= 12 as libc::c_int) {
            break;
        }
        ndays = dap_numdate(d);
    }
    mon -= 1;
    mon;
    day = 1 as libc::c_int;
    sprintf(d, b"%02d%02d%4d\0" as *const u8 as *const libc::c_char, mon, day, yr);
    ndays = dap_numdate(d);
    while ndays < n {
        day += 1;
        sprintf(d, b"%02d%02d%4d\0" as *const u8 as *const libc::c_char, mon, day, yr);
        if day <= mdays[mon as usize] {
            ndays = dap_numdate(d);
        } else {
            exit(1 as libc::c_int);
        }
    }
}
pub unsafe extern "C" fn dap_bincoeff(
    mut n: libc::c_double,
    mut r: libc::c_double,
) -> libc::c_double {
    let mut b: libc::c_double = 0.;
    b = 1.0f64;
    while r > 0.0f64 {
        b *= n / r;
        r -= 1.0f64;
        n -= 1.0f64;
    }
    return rint(b);
}
unsafe extern "C" fn takestep(
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
    mut d: *mut libc::c_double,
    mut n: libc::c_int,
    mut step: libc::c_double,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        *x.offset(i as isize) = *y.offset(i as isize) + *d.offset(i as isize) * step;
        i += 1;
        i;
    }
}
unsafe extern "C" fn vcopy(
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
    mut n: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        *x.offset(i as isize) = *y.offset(i as isize);
        i += 1;
        i;
    }
}
unsafe extern "C" fn vsub(
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
    mut n: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        *x.offset(i as isize) -= *y.offset(i as isize);
        i += 1;
        i;
    }
}
unsafe extern "C" fn vlen(
    mut x: *mut libc::c_double,
    mut nx: libc::c_int,
) -> libc::c_double {
    let mut len: libc::c_double = 0.;
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    len = 0.0f64;
    while n < nx {
        len += *x.offset(n as isize) * *x.offset(n as isize);
        n += 1;
        n;
    }
    return sqrt(len);
}
unsafe extern "C" fn dirstep(
    mut f: Option::<unsafe extern "C" fn(*mut libc::c_double) -> libc::c_double>,
    mut nx: libc::c_int,
    mut x: *mut libc::c_double,
    mut x1: *mut libc::c_double,
    mut f0: libc::c_double,
    mut step: libc::c_double,
    mut tol: libc::c_double,
) -> libc::c_double {
    let mut n: libc::c_int = 0;
    static mut f1: libc::c_double = 0.;
    static mut f2: libc::c_double = 0.;
    static mut f3: libc::c_double = 0.;
    let mut dstep: libc::c_double = 0.;
    n = 0 as libc::c_int;
    while n < nx {
        *x1.offset(n as isize) = *x.offset(n as isize);
        n += 1;
        n;
    }
    n = 0 as libc::c_int;
    while n < nx {
        *x1.offset(n as isize) = *x.offset(n as isize) - step;
        f1 = (Some(f.unwrap())).unwrap()(x1);
        *x1.offset(n as isize) = *x.offset(n as isize) + step;
        f2 = (Some(f.unwrap())).unwrap()(x1);
        dstep = step * (f1 - f2) / (f1 - 2.0f64 * f0 + f2) / 2.0f64;
        if finite(dstep) != 0 && fabs(dstep) > tol
            && (f1 < f0 && f0 < f2 || f1 > f0 && f0 > f2)
        {
            *x1.offset(n as isize) = *x.offset(n as isize) + dstep;
            f3 = (Some(f.unwrap())).unwrap()(x1);
            if finite(f3) != 0 && f3 > f1 && f3 > f2 {
                f0 = f3;
            } else if f1 > f0 {
                f0 = f1;
                *x1.offset(n as isize) = *x.offset(n as isize) - step;
            } else {
                f0 = f2;
                *x1.offset(n as isize) = *x.offset(n as isize) + step;
            }
        } else {
            *x1.offset(n as isize) = *x.offset(n as isize);
        }
        n += 1;
        n;
    }
    return f0;
}
pub unsafe extern "C" fn dap_maximize(
    mut f: Option::<unsafe extern "C" fn(*mut libc::c_double) -> libc::c_double>,
    mut nx: libc::c_int,
    mut x: *mut libc::c_double,
    mut step: libc::c_double,
    mut tol: libc::c_double,
    mut trace: *mut libc::c_char,
) -> libc::c_double {
    let mut tr: libc::c_int = 0;
    let mut ntries: libc::c_int = 0;
    let mut x1: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut x2: *mut libc::c_double = 0 as *mut libc::c_double;
    static mut f0: libc::c_double = 0.;
    static mut f1: libc::c_double = 0.;
    static mut f2: libc::c_double = 0.;
    let mut dir: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut dirlen: libc::c_double = 0.;
    let mut dstep: libc::c_double = 0.;
    let mut n: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut traceout: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut nsteps: libc::c_int = 0;
    x1 = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nx as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    x2 = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nx as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    dir = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nx as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    tr = 0 as libc::c_int;
    if !trace.is_null() && *trace.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        if strncmp(
            trace,
            b"TRACE\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as libc::c_ulong,
        ) == 0
            || strncmp(
                trace,
                b"PAUSE\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int as libc::c_ulong,
            ) == 0
        {
            if *trace.offset(0 as libc::c_int as isize) as libc::c_int == 'T' as i32 {
                tr = 1 as libc::c_int;
            } else {
                tr = 2 as libc::c_int;
            }
            t = 5 as libc::c_int;
            while *trace.offset(t as isize) as libc::c_int == ' ' as i32 {
                t += 1;
                t;
            }
            traceout = 0 as libc::c_int;
            while '0' as i32 <= *trace.offset(t as isize) as libc::c_int
                && *trace.offset(t as isize) as libc::c_int <= '9' as i32
            {
                traceout = 10 as libc::c_int * traceout
                    + *trace.offset(t as isize) as libc::c_int - '0' as i32;
                t += 1;
                t;
            }
            while *trace.offset(t as isize) as libc::c_int == ' ' as i32 {
                t += 1;
                t;
            }
            if *trace.offset(t as isize) != 0 {
                fprintf(
                    dap_err,
                    b"(dap_maximize) bad trace interval: %s\n\0" as *const u8
                        as *const libc::c_char,
                    trace,
                );
                exit(1 as libc::c_int);
            }
        } else {
            fprintf(
                dap_err,
                b"(dap_maximize) bad tracing option: %s\n\0" as *const u8
                    as *const libc::c_char,
                trace,
            );
            exit(1 as libc::c_int);
        }
    } else {
        tr = 0 as libc::c_int;
    }
    f0 = (Some(f.unwrap())).unwrap()(x);
    nsteps = 0 as libc::c_int;
    loop {
        if nsteps > dap_maxiter {
            fprintf(
                dap_err,
                b"(dap_maximize) stepsize = %g failed to reach tolerance = %g after %d iterations\n\0"
                    as *const u8 as *const libc::c_char,
                step,
                tol,
                nsteps,
            );
            break;
        } else {
            f1 = dirstep(f, nx, x, x1, f0, step, tol);
            if f1 > f0 {
                vcopy(dir, x1, nx);
                vsub(dir, x, nx);
                dirlen = vlen(dir, nx);
                vcopy(x2, x, nx);
                vsub(x2, dir, nx);
                f2 = (Some(f.unwrap())).unwrap()(x2);
                dstep = (f2 - f1) / (f1 - 2.0f64 * f0 + f2) / 2.0f64;
                if finite(dstep) != 0 && dstep > tol / step {
                    takestep(x2, x, dir, nx, dstep);
                    f2 = (Some(f.unwrap())).unwrap()(x2);
                    if finite(f2) != 0 && f2 > f1 {
                        vcopy(x1, x2, nx);
                        f1 = f2;
                    }
                }
            }
            if tr != 0 && (traceout == 0 || nsteps % traceout == 0) {
                fprintf(
                    dap_log,
                    b"(dap_maximize) nsteps = %d, f0 = %.16g, f1 = %.16g, step = %g\ndir = \0"
                        as *const u8 as *const libc::c_char,
                    nsteps,
                    f0,
                    f1,
                    step,
                );
                fprintf(
                    stderr,
                    b"(dap_maximize) nsteps = %d, f0 = %.16g, f1 = %.16g, step = %g\ndir = \0"
                        as *const u8 as *const libc::c_char,
                    nsteps,
                    f0,
                    f1,
                    step,
                );
                n = 0 as libc::c_int;
                while n < nx {
                    fprintf(
                        dap_log,
                        b" %g\0" as *const u8 as *const libc::c_char,
                        *dir.offset(n as isize),
                    );
                    fprintf(
                        stderr,
                        b" %g\0" as *const u8 as *const libc::c_char,
                        *dir.offset(n as isize),
                    );
                    n += 1;
                    n;
                }
                fputs(b"\nx =\0" as *const u8 as *const libc::c_char, dap_log);
                fputs(b"\nx =\0" as *const u8 as *const libc::c_char, stderr);
                n = 0 as libc::c_int;
                while n < nx {
                    fprintf(
                        dap_log,
                        b" %g\0" as *const u8 as *const libc::c_char,
                        *x.offset(n as isize),
                    );
                    fprintf(
                        stderr,
                        b" %g\0" as *const u8 as *const libc::c_char,
                        *x.offset(n as isize),
                    );
                    n += 1;
                    n;
                }
                putc('\n' as i32, dap_log);
                putc('\n' as i32, stderr);
                fflush(stderr);
                if tr == 2 as libc::c_int {
                    getchar();
                }
            }
            if f1 <= f0 {
                step /= 2.0f64;
                if step < tol {
                    break;
                }
            } else {
                f0 = f1;
                vcopy(x, x1, nx);
            }
            nsteps += 1;
            nsteps;
        }
    }
    dap_free(
        x1 as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        x2 as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        dir as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return f0;
}
