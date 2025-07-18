use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn rint(_: libc::c_double) -> libc::c_double;
    fn step() -> libc::c_int;
    fn inset(fname: *mut libc::c_char);
    fn outset(fname: *mut libc::c_char, varlist: *mut libc::c_char);
    fn output();
    fn dap_suffix(
        dst: *mut libc::c_char,
        src: *mut libc::c_char,
        suff: *mut libc::c_char,
    );
    fn dap_vd(varspec: *mut libc::c_char, invar: libc::c_int) -> libc::c_int;
    fn dap_swap();
    fn dap_mark();
    fn dap_rewind();
    fn dap_varnum(vname: *mut libc::c_char) -> libc::c_int;
    fn dap_head(markv: *mut libc::c_int, nmark: libc::c_int);
    fn dap_list(
        varlist: *mut libc::c_char,
        varv: *mut libc::c_int,
        maxvars: libc::c_int,
    ) -> libc::c_int;
    fn dap_newpart(partv: *mut libc::c_int, npartv: libc::c_int) -> libc::c_int;
    fn dap_free(ptr: *mut libc::c_void, mesg: *mut libc::c_char);
    fn dap_malloc(nbytes: libc::c_int, mesg: *mut libc::c_char) -> *mut libc::c_char;
    fn probt(t1: libc::c_double, di: libc::c_int) -> libc::c_double;
    fn zpoint(p: libc::c_double) -> libc::c_double;
    fn tpoint(p: libc::c_double, in_0: libc::c_int) -> libc::c_double;
    fn probz(z1: libc::c_double) -> libc::c_double;
    fn probf(f2: libc::c_double, id1: libc::c_int, id2: libc::c_int) -> libc::c_double;
    fn probchisq(x2: libc::c_double, df: libc::c_int) -> libc::c_double;
    fn dap_bincoeff(n: libc::c_double, r: libc::c_double) -> libc::c_double;
    fn dap_clearobs(varspec: *mut libc::c_char) -> libc::c_int;
    static mut dap_maxvar: libc::c_int;
    static mut dap_namelen: libc::c_int;
    static mut dap_strlen: libc::c_int;
    static mut dap_maxval: libc::c_int;
    static mut dap_maxlev: libc::c_int;
    static mut dap_tol: libc::c_double;
    static mut dap_ctol: libc::c_double;
    static mut dap_ktol: libc::c_double;
    static mut dap_maxiter: libc::c_int;
    static mut dap_maxex1: libc::c_int;
    static mut dap_maxex2: libc::c_int;
    static mut dap_obs: [dataobs; 0];
    static mut dap_lst: *mut FILE;
    static mut dap_err: *mut FILE;
    static mut dap_ono: libc::c_int;
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dataobs {
    pub do_int: *mut libc::c_int,
    pub do_il: *mut *mut libc::c_int,
    pub do_dbl: *mut libc::c_double,
    pub do_dl: *mut *mut libc::c_double,
    pub do_str: *mut *mut libc::c_char,
    pub do_sl: *mut libc::c_int,
    pub do_nam: *mut *mut libc::c_char,
    pub do_len: *mut libc::c_int,
    pub do_in: *mut libc::c_int,
    pub do_out: *mut libc::c_int,
    pub do_ivar: libc::c_int,
    pub do_ovar: libc::c_int,
    pub do_nvar: libc::c_int,
    pub do_valid: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct value {
    pub val_class: libc::c_int,
    pub val_val: libc::c_double,
}
unsafe extern "C" fn matchmark(
    mut markv: *mut libc::c_int,
    mut xmarkv: *mut libc::c_int,
    mut nmark: libc::c_int,
    mut level: libc::c_double,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut diff: libc::c_int = 0;
    if *xmarkv.offset(0 as libc::c_int as isize) < 0 as libc::c_int {
        return (dap_newpart(markv, nmark) == 0) as libc::c_int;
    }
    diff = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while diff == 0 && i < nmark {
        if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
            .offset(*markv.offset(i as isize) as isize) > 0 as libc::c_int
        {
            diff = strcmp(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(*markv.offset(i as isize) as isize),
                *((*dap_obs.as_mut_ptr().offset(1 as libc::c_int as isize)).do_str)
                    .offset(*xmarkv.offset(i as isize) as isize),
            );
        } else if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
            .offset(*markv.offset(i as isize) as isize) == 0 as libc::c_int
        {
            diff = (*((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_int)
                .offset(*markv.offset(i as isize) as isize)
                != *((*dap_obs.as_mut_ptr().offset(1 as libc::c_int as isize)).do_int)
                    .offset(*xmarkv.offset(i as isize) as isize)) as libc::c_int;
        } else {
            diff = (*((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(*markv.offset(i as isize) as isize)
                != *((*dap_obs.as_mut_ptr().offset(1 as libc::c_int as isize)).do_dbl)
                    .offset(*xmarkv.offset(i as isize) as isize)) as libc::c_int;
        }
        i += 1;
        i;
    }
    return (diff == 0) as libc::c_int;
}
pub unsafe extern "C" fn linreg1(
    mut xymat: *mut *mut libc::c_double,
    mut varv: *mut libc::c_int,
    mut nx0: libc::c_int,
    mut nx: libc::c_int,
    mut ny: libc::c_int,
    mut nobs: libc::c_int,
    mut xvarv: *mut libc::c_int,
    mut markv: *mut libc::c_int,
    mut xmarkv: *mut libc::c_int,
    mut nmark: libc::c_int,
    mut level: libc::c_double,
    mut respn: libc::c_int,
    mut param1n: libc::c_int,
    mut param2n: libc::c_int,
    mut covn: libc::c_int,
    mut partv: *mut libc::c_int,
) {
    let mut invmem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut inv: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut r: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut rr: libc::c_int = 0;
    let mut cc: libc::c_int = 0;
    let mut pivot: libc::c_double = 0.;
    let mut tmp: libc::c_double = 0.;
    let mut tmp2: libc::c_double = 0.;
    let mut typen: libc::c_int = 0;
    let mut dnobs: libc::c_double = 0.;
    let mut rss0: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut rss1: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut rss: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut f: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut fch: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut df: libc::c_int = 0;
    let mut ddf: libc::c_double = 0.;
    let mut xi: libc::c_double = 0.;
    let mut xj: libc::c_double = 0.;
    static mut tpt: libc::c_double = 0.;
    let mut yn: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pred: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut sepred: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut v: libc::c_int = 0;
    dap_swap();
    typen = dap_varnum(
        b"_type_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if typen < 0 as libc::c_int {
        fprintf(
            dap_err,
            b"(linreg1) Missing _type_ variable.\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    invmem = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nx as libc::c_ulong)
            .wrapping_mul(nx as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    inv = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nx as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_double;
    r = 0 as libc::c_int;
    while r < nx {
        let ref mut fresh0 = *inv.offset(r as isize);
        *fresh0 = invmem.offset((r * nx) as isize);
        r += 1;
        r;
    }
    rss0 = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(ny as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    rss1 = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(ny as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    rss = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(ny as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    f = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(ny as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    fch = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(ny as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    pred = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(ny as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    sepred = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(ny as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    dnobs = nobs as libc::c_double;
    c = 1 as libc::c_int;
    while c < nx + ny {
        *(*xymat.offset(0 as libc::c_int as isize)).offset(c as isize) /= dnobs;
        c += 1;
        c;
    }
    r = 1 as libc::c_int;
    while r < nx {
        c = 1 as libc::c_int;
        while c < nx {
            if r == c {
                *(*inv.offset(r as isize)).offset(c as isize) = 1.0f64;
            } else {
                *(*inv.offset(r as isize)).offset(c as isize) = 0.0f64;
            }
            c += 1;
            c;
        }
        r += 1;
        r;
    }
    c = 1 as libc::c_int;
    while c < nx {
        if c == 1 as libc::c_int {
            cc = 0 as libc::c_int;
            while cc < ny {
                *rss0
                    .offset(
                        cc as isize,
                    ) = *(*xymat.offset((nx + cc) as isize)).offset((nx + cc) as isize);
                cc += 1;
                cc;
            }
        }
        if c == nx0 {
            cc = 0 as libc::c_int;
            while cc < ny {
                *rss1
                    .offset(
                        cc as isize,
                    ) = *(*xymat.offset((nx + cc) as isize)).offset((nx + cc) as isize);
                cc += 1;
                cc;
            }
        }
        pivot = *(*xymat.offset(c as isize)).offset(c as isize);
        if pivot != 0.0f64 {
            rr = c + 1 as libc::c_int;
            while rr < nx + ny {
                tmp = *(*xymat.offset(rr as isize)).offset(c as isize) / pivot;
                *(*xymat.offset(rr as isize)).offset(c as isize) = 0.0f64;
                cc = c + 1 as libc::c_int;
                while cc < nx + ny {
                    if rr < nx || cc < nx || rr == cc {
                        *(*xymat.offset(rr as isize)).offset(cc as isize)
                            -= tmp * *(*xymat.offset(c as isize)).offset(cc as isize);
                        if fabs(*(*xymat.offset(rr as isize)).offset(cc as isize))
                            < dap_tol * pivot
                        {
                            *(*xymat.offset(rr as isize)).offset(cc as isize) = 0.0f64;
                        }
                    }
                    cc += 1;
                    cc;
                }
                if rr < nx {
                    cc = 1 as libc::c_int;
                    while cc < nx {
                        *(*inv.offset(rr as isize)).offset(cc as isize)
                            -= tmp * *(*inv.offset(c as isize)).offset(cc as isize);
                        if fabs(*(*inv.offset(rr as isize)).offset(cc as isize))
                            < dap_tol * pivot
                        {
                            *(*inv.offset(rr as isize)).offset(cc as isize) = 0.0f64;
                        }
                        cc += 1;
                        cc;
                    }
                }
                rr += 1;
                rr;
            }
        } else {
            fprintf(
                dap_err,
                b"(linreg1) X'X matrix is singular.\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        c += 1;
        c;
    }
    c = nx - 1 as libc::c_int;
    while c > 0 as libc::c_int {
        r = c - 1 as libc::c_int;
        while r > 0 as libc::c_int {
            tmp = *(*xymat.offset(r as isize)).offset(c as isize)
                / *(*xymat.offset(c as isize)).offset(c as isize);
            cc = c + 1 as libc::c_int;
            while cc < nx + ny {
                *(*xymat.offset(r as isize)).offset(cc as isize)
                    -= tmp * *(*xymat.offset(c as isize)).offset(cc as isize);
                cc += 1;
                cc;
            }
            cc = 0 as libc::c_int;
            while cc < nx {
                *(*inv.offset(r as isize)).offset(cc as isize)
                    -= tmp * *(*inv.offset(c as isize)).offset(cc as isize);
                cc += 1;
                cc;
            }
            r -= 1;
            r;
        }
        cc = c + 1 as libc::c_int;
        while cc < nx + ny {
            *(*xymat.offset(c as isize)).offset(cc as isize)
                /= *(*xymat.offset(c as isize)).offset(c as isize);
            cc += 1;
            cc;
        }
        cc = 0 as libc::c_int;
        while cc < nx {
            *(*inv.offset(c as isize)).offset(cc as isize)
                /= *(*xymat.offset(c as isize)).offset(c as isize);
            cc += 1;
            cc;
        }
        c -= 1;
        c;
    }
    df = nobs - nx;
    ddf = df as libc::c_double;
    c = 0 as libc::c_int;
    while c < ny {
        *rss
            .offset(
                c as isize,
            ) = *(*xymat.offset((nx + c) as isize)).offset((nx + c) as isize);
        *f
            .offset(
                c as isize,
            ) = (*rss0.offset(c as isize) - *rss.offset(c as isize))
            / *rss.offset(c as isize) * ddf
            / (nx as libc::c_double - 1 as libc::c_int as libc::c_double);
        *fch
            .offset(
                c as isize,
            ) = (*rss1.offset(c as isize) - *rss.offset(c as isize))
            / *rss.offset(c as isize) * ddf
            / (nx as libc::c_double - nx0 as libc::c_double);
        c += 1;
        c;
    }
    r = 1 as libc::c_int;
    while r < nx {
        c = 1 as libc::c_int;
        while c < nx {
            *(*xymat.offset(r as isize))
                .offset(c as isize) = *(*inv.offset(r as isize)).offset(c as isize);
            c += 1;
            c;
        }
        r += 1;
        r;
    }
    r = 1 as libc::c_int;
    while r < nx {
        c = 1 as libc::c_int;
        while c < nx {
            *(*xymat.offset(r as isize)).offset(0 as libc::c_int as isize)
                -= *(*xymat.offset(r as isize)).offset(c as isize)
                    * *(*xymat.offset(0 as libc::c_int as isize)).offset(c as isize);
            c += 1;
            c;
        }
        r += 1;
        r;
    }
    *(*xymat.offset(0 as libc::c_int as isize))
        .offset(0 as libc::c_int as isize) = 1.0f64 / dnobs;
    c = 1 as libc::c_int;
    while c < nx {
        *(*xymat.offset(0 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
            -= *(*xymat.offset(0 as libc::c_int as isize)).offset(c as isize)
                * *(*xymat.offset(c as isize)).offset(0 as libc::c_int as isize);
        c += 1;
        c;
    }
    c = 0 as libc::c_int;
    while c < ny {
        cc = 1 as libc::c_int;
        while cc < nx {
            *(*xymat.offset(0 as libc::c_int as isize)).offset((nx + c) as isize)
                -= *(*xymat.offset(0 as libc::c_int as isize)).offset(cc as isize)
                    * *(*xymat.offset(cc as isize)).offset((nx + c) as isize);
            cc += 1;
            cc;
        }
        c += 1;
        c;
    }
    c = 1 as libc::c_int;
    while c < nx {
        *(*xymat.offset(0 as libc::c_int as isize))
            .offset(
                c as isize,
            ) = *(*xymat.offset(c as isize)).offset(0 as libc::c_int as isize);
        c += 1;
        c;
    }
    dap_ono = 2 as libc::c_int;
    v = 0 as libc::c_int;
    while v < nmark {
        if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
            .offset(*markv.offset(v as isize) as isize) == -(1 as libc::c_int)
        {
            *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_dbl)
                .offset(
                    *partv.offset(v as isize) as isize,
                ) = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(*markv.offset(v as isize) as isize);
        } else if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
            .offset(*markv.offset(v as isize) as isize) == 0 as libc::c_int
        {
            *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_int)
                .offset(
                    *partv.offset(v as isize) as isize,
                ) = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_int)
                .offset(*markv.offset(v as isize) as isize);
        } else {
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                    .offset(*partv.offset(v as isize) as isize),
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(*markv.offset(v as isize) as isize),
            );
        }
        v += 1;
        v;
    }
    fprintf(
        dap_lst,
        b"Reduced | full model regressors:\0" as *const u8 as *const libc::c_char,
    );
    r = 0 as libc::c_int;
    while r < nx0 {
        fprintf(
            dap_lst,
            b" %s\0" as *const u8 as *const libc::c_char,
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                .offset(*varv.offset(r as isize) as isize),
        );
        r += 1;
        r;
    }
    fprintf(dap_lst, b" |\0" as *const u8 as *const libc::c_char);
    while r < nx {
        let fresh1 = r;
        r = r + 1;
        fprintf(
            dap_lst,
            b" %s\0" as *const u8 as *const libc::c_char,
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                .offset(*varv.offset(fresh1 as isize) as isize),
        );
    }
    putc('\n' as i32, dap_lst);
    fprintf(
        dap_lst,
        b"Number of observations = %d\n\0" as *const u8 as *const libc::c_char,
        nobs,
    );
    c = 0 as libc::c_int;
    while c < ny {
        fprintf(
            dap_lst,
            b"\nResponse: %s\n\0" as *const u8 as *const libc::c_char,
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                .offset(*varv.offset((nx + c) as isize) as isize),
        );
        fprintf(
            dap_lst,
            b"   F0(%d, %d) = %.6g, Prob[F > F0] = %.5f\n   R-sq = %.6g, Adj R-sq = %.6g\n\0"
                as *const u8 as *const libc::c_char,
            nx - 1 as libc::c_int,
            nobs - nx,
            *f.offset(c as isize),
            0.00001f64
                * ceil(
                    100000.0f64
                        * probf(*f.offset(c as isize), nx - 1 as libc::c_int, nobs - nx),
                ),
            1.0f64 - *rss.offset(c as isize) / *rss0.offset(c as isize),
            1.0f64
                - *rss.offset(c as isize) * (nobs - 1 as libc::c_int) as libc::c_double
                    / (*rss0.offset(c as isize) * ddf),
        );
        if nx0 > 1 as libc::c_int {
            fprintf(
                dap_lst,
                b"   F-change(%d, %d) = %.6g, Prob[F > F-change] = %.5f\n\0" as *const u8
                    as *const libc::c_char,
                nx - nx0,
                nobs - nx,
                *fch.offset(c as isize),
                0.00001f64
                    * ceil(
                        100000.0f64 * probf(*fch.offset(c as isize), nx - nx0, nobs - nx),
                    ),
            );
        }
        fprintf(
            dap_lst,
            b"\n   Parameter           Estimate    Std Error   T0[%6d]  Prob[|T|>|T0|]\n\0"
                as *const u8 as *const libc::c_char,
            nobs - nx,
        );
        r = 0 as libc::c_int;
        while r < nx {
            tmp = sqrt(
                *rss.offset(c as isize) / ddf
                    * *(*xymat.offset(r as isize)).offset(r as isize),
            );
            tmp2 = *(*xymat.offset(r as isize)).offset((nx + c) as isize) / tmp;
            fprintf(
                dap_lst,
                b"   %-15s %12.6g %12.6g %12.6g  %14.5f\n\0" as *const u8
                    as *const libc::c_char,
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(*varv.offset(r as isize) as isize),
                *(*xymat.offset(r as isize)).offset((nx + c) as isize),
                tmp,
                tmp2,
                0.00001f64 * ceil(200000.0f64 * probt(fabs(tmp2), nobs - nx)),
            );
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                    .offset(typen as isize),
                b"ESTIMATE\0" as *const u8 as *const libc::c_char,
            );
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                    .offset(respn as isize),
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(*varv.offset((nx + c) as isize) as isize),
            );
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                    .offset(param2n as isize),
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(*varv.offset(r as isize) as isize),
            );
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                    .offset(param1n as isize),
                b"\0" as *const u8 as *const libc::c_char,
            );
            *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_dbl)
                .offset(
                    covn as isize,
                ) = *(*xymat.offset(r as isize)).offset((nx + c) as isize);
            output();
            r += 1;
            r;
        }
        c += 1;
        c;
    }
    strcpy(
        *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
            .offset(typen as isize),
        b"COVAR\0" as *const u8 as *const libc::c_char,
    );
    yn = 0 as libc::c_int;
    while yn < ny {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                .offset(respn as isize),
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                .offset(*varv.offset((nx + yn) as isize) as isize),
        );
        tmp = *rss.offset(yn as isize) / ddf;
        r = 0 as libc::c_int;
        while r < nx {
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                    .offset(param1n as isize),
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(*varv.offset(r as isize) as isize),
            );
            c = 0 as libc::c_int;
            while c < nx {
                strcpy(
                    *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                        .offset(param2n as isize),
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                        .offset(*varv.offset(c as isize) as isize),
                );
                *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_dbl)
                    .offset(
                        covn as isize,
                    ) = tmp * *(*xymat.offset(r as isize)).offset(c as isize);
                output();
                c += 1;
                c;
            }
            r += 1;
            r;
        }
        yn += 1;
        yn;
    }
    dap_ono = 0 as libc::c_int;
    if level < 1.0f64 {
        tpt = tpoint(0.5f64 * (1.0f64 - level), nobs - nx);
    } else {
        tpt = 0.0f64;
    }
    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
        .offset(*varv.offset(0 as libc::c_int as isize) as isize) = 1.0f64;
    if *xvarv.offset(0 as libc::c_int as isize) < 0 as libc::c_int {
        dap_rewind();
        step();
    }
    while matchmark(markv, xmarkv, nmark, level) != 0 {
        dap_ono = 0 as libc::c_int;
        if *xvarv.offset(0 as libc::c_int as isize) >= 0 as libc::c_int {
            i = 1 as libc::c_int;
            while i < nx {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        *varv.offset(i as isize) as isize,
                    ) = *((*dap_obs.as_mut_ptr().offset(1 as libc::c_int as isize))
                    .do_dbl)
                    .offset(*xvarv.offset((i - 1 as libc::c_int) as isize) as isize);
                i += 1;
                i;
            }
        } else {
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                b"OBS\0" as *const u8 as *const libc::c_char,
            );
            output();
        }
        yn = 0 as libc::c_int;
        while yn < ny {
            *pred.offset(yn as isize) = 0.0f64;
            i = 0 as libc::c_int;
            while i < nx {
                *pred.offset(yn as isize)
                    += *(*xymat.offset(i as isize)).offset((nx + yn) as isize)
                        * *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_dbl)
                            .offset(*varv.offset(i as isize) as isize);
                i += 1;
                i;
            }
            *sepred.offset(yn as isize) = 0.0f64;
            i = 0 as libc::c_int;
            while i < nx {
                xi = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(*varv.offset(i as isize) as isize);
                j = 0 as libc::c_int;
                while j < nx {
                    xj = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                        .do_dbl)
                        .offset(*varv.offset(j as isize) as isize);
                    *sepred.offset(yn as isize)
                        += xi * *rss.offset(yn as isize) / ddf
                            * *(*xymat.offset(i as isize)).offset(j as isize) * xj;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
            yn += 1;
            yn;
        }
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typen as isize),
            b"PRED\0" as *const u8 as *const libc::c_char,
        );
        yn = 0 as libc::c_int;
        while yn < ny {
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(
                    *varv.offset((nx + yn) as isize) as isize,
                ) = *pred.offset(yn as isize);
            yn += 1;
            yn;
        }
        output();
        if tpt != 0.0f64 {
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                b"LOWER\0" as *const u8 as *const libc::c_char,
            );
            yn = 0 as libc::c_int;
            while yn < ny {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        *varv.offset((nx + yn) as isize) as isize,
                    ) = *pred.offset(yn as isize)
                    - tpt * sqrt(*sepred.offset(yn as isize));
                yn += 1;
                yn;
            }
            output();
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                b"UPPER\0" as *const u8 as *const libc::c_char,
            );
            yn = 0 as libc::c_int;
            while yn < ny {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        *varv.offset((nx + yn) as isize) as isize,
                    ) = *pred.offset(yn as isize)
                    + tpt * sqrt(*sepred.offset(yn as isize));
                yn += 1;
                yn;
            }
            output();
        }
        if *xvarv.offset(0 as libc::c_int as isize) >= 0 as libc::c_int {
            dap_ono = 1 as libc::c_int;
        }
        dap_mark();
        if step() == 0 {
            break;
        }
    }
    dap_ono = 0 as libc::c_int;
    if *xvarv.offset(0 as libc::c_int as isize) >= 0 as libc::c_int {
        dap_swap();
    }
    dap_free(
        invmem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        inv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        rss0 as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        rss1 as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        rss as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        f as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        fch as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        pred as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        sepred as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn linreg(
    mut fname: *mut libc::c_char,
    mut ylist: *mut libc::c_char,
    mut x0list: *mut libc::c_char,
    mut x1list: *mut libc::c_char,
    mut marks: *mut libc::c_char,
    mut xname: *mut libc::c_char,
    mut level: libc::c_double,
) {
    let mut regname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut varv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut xvarv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ny: libc::c_int = 0;
    let mut nx0: libc::c_int = 0;
    let mut nx1: libc::c_int = 0;
    let mut nx: libc::c_int = 0;
    let mut nvar: libc::c_int = 0;
    let mut nxx: libc::c_int = 0;
    let mut markv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut xmarkv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nmark: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut tmp: libc::c_double = 0.;
    let mut nobs: libc::c_int = 0;
    let mut dnobs: libc::c_double = 0.;
    let mut xymem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut xymat: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut more: libc::c_int = 0;
    let mut covset: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut param1n: libc::c_int = 0;
    let mut param2n: libc::c_int = 0;
    let mut respn: libc::c_int = 0;
    let mut covn: libc::c_int = 0;
    let mut paramlen1: libc::c_int = 0;
    let mut paramlen: libc::c_int = 0;
    let mut paramstr: [libc::c_char; 14] = [0; 14];
    let mut partstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut partv: *mut libc::c_int = 0 as *mut libc::c_int;
    if fname.is_null() {
        fputs(
            b"(linreg) No dataset name given.\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    varv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"dap_maxvar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    xvarv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"dap_maxvar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    markv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"dap_maxvar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    xmarkv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"dap_maxvar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    partv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"dap_maxvar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    regname = dap_malloc(
        (strlen(fname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_suffix(
        regname,
        fname,
        b".reg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    inset(fname);
    dap_vd(
        b"_intercept_ -1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    nx0 = dap_list(
        b"_intercept_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        varv,
        dap_maxvar,
    );
    nx0 += dap_list(x0list, varv.offset(1 as libc::c_int as isize), dap_maxvar);
    nx1 = dap_list(x1list, varv.offset(nx0 as isize), dap_maxvar);
    nx = nx0 + nx1;
    ny = dap_list(ylist, varv.offset(nx as isize), dap_maxvar);
    nvar = nx + ny;
    nmark = dap_list(marks, markv, dap_maxvar);
    covset = dap_malloc(
        (strlen(fname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(covset, fname);
    strcat(covset, b".cov\0" as *const u8 as *const libc::c_char);
    v = 1 as libc::c_int;
    paramlen = strlen(b"_intercept_\0" as *const u8 as *const libc::c_char)
        as libc::c_int;
    while v < nvar {
        paramlen1 = strlen(
            *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_nam)
                .offset(*varv.offset(v as isize) as isize),
        ) as libc::c_int;
        if paramlen1 > paramlen {
            paramlen = paramlen1;
        }
        v += 1;
        v;
    }
    dap_ono = 2 as libc::c_int;
    dap_clearobs(0 as *mut libc::c_void as *mut libc::c_char);
    sprintf(
        paramstr.as_mut_ptr(),
        b"_response_ %d\0" as *const u8 as *const libc::c_char,
        paramlen,
    );
    respn = dap_vd(paramstr.as_mut_ptr(), 0 as libc::c_int);
    sprintf(
        paramstr.as_mut_ptr(),
        b"_param1_ %d\0" as *const u8 as *const libc::c_char,
        paramlen,
    );
    param1n = dap_vd(paramstr.as_mut_ptr(), 0 as libc::c_int);
    sprintf(
        paramstr.as_mut_ptr(),
        b"_param2_ %d\0" as *const u8 as *const libc::c_char,
        paramlen,
    );
    param2n = dap_vd(paramstr.as_mut_ptr(), 0 as libc::c_int);
    covn = dap_vd(
        b"_cov_ -1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    partstr = dap_malloc(
        (strlen(marks)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    v = 0 as libc::c_int;
    while v < nmark {
        strcpy(
            partstr,
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                .offset(*markv.offset(v as isize) as isize),
        );
        sprintf(
            partstr.offset(strlen(partstr) as isize),
            b" %d\0" as *const u8 as *const libc::c_char,
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
                .offset(*markv.offset(v as isize) as isize),
        );
        *partv.offset(v as isize) = dap_vd(partstr, 1 as libc::c_int);
        v += 1;
        v;
    }
    outset(covset, b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    dap_ono = 0 as libc::c_int;
    xymem = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nvar as libc::c_ulong)
            .wrapping_mul(nvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    xymat = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_double;
    v = 0 as libc::c_int;
    while v < nvar {
        let ref mut fresh2 = *xymat.offset(v as isize);
        *fresh2 = xymem.offset((v * nvar) as isize);
        v += 1;
        v;
    }
    dap_ono = 1 as libc::c_int;
    if !xname.is_null() && *xname.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        inset(xname);
        nxx = dap_list(x0list, xvarv, dap_maxvar);
        nxx += dap_list(x1list, xvarv.offset(nxx as isize), dap_maxvar);
        if nxx != nx - 1 as libc::c_int {
            fprintf(
                dap_err,
                b"(linreg) %s and %s have different numbers (%d and %d) of x-variables.\n\0"
                    as *const u8 as *const libc::c_char,
                fname,
                xname,
                nx - 1 as libc::c_int,
                nxx,
            );
            exit(1 as libc::c_int);
        }
        if nmark != 0 {
            v = 0 as libc::c_int;
            while v < nmark {
                let ref mut fresh3 = *xmarkv.offset(v as isize);
                *fresh3 = dap_varnum(
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                        .offset(*markv.offset(v as isize) as isize),
                );
                if *fresh3 < 0 as libc::c_int {
                    fprintf(
                        dap_err,
                        b"(linreg) Mark variable %s in %s not in %s.\n\0" as *const u8
                            as *const libc::c_char,
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_nam)
                            .offset(*markv.offset(v as isize) as isize),
                        fname,
                        xname,
                    );
                    exit(1 as libc::c_int);
                }
                v += 1;
                v;
            }
        }
        if step() == 0 {
            fprintf(
                dap_err,
                b"(linreg) No data in %s.\n\0" as *const u8 as *const libc::c_char,
                xname,
            );
            exit(1 as libc::c_int);
        }
    } else {
        *xvarv.offset(0 as libc::c_int as isize) = -(1 as libc::c_int);
        *xmarkv.offset(0 as libc::c_int as isize) = -(1 as libc::c_int);
    }
    dap_ono = 0 as libc::c_int;
    w = 0 as libc::c_int;
    while w < nvar {
        if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
            .offset(*varv.offset(w as isize) as isize) != -(1 as libc::c_int)
        {
            fprintf(
                dap_err,
                b"(linreg) Variable %s not double.\n\0" as *const u8
                    as *const libc::c_char,
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(*varv.offset(w as isize) as isize),
            );
            exit(1 as libc::c_int);
        }
        v = 0 as libc::c_int;
        while v < nvar {
            *(*xymat.offset(v as isize)).offset(w as isize) = 0.0f64;
            v += 1;
            v;
        }
        w += 1;
        w;
    }
    outset(regname, b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    nobs = 0 as libc::c_int;
    dap_mark();
    more = 1 as libc::c_int;
    while more != 0 {
        more = step();
        if dap_newpart(markv, nmark) != 0 {
            dap_swap();
            dap_head(markv, nmark);
            dap_swap();
            linreg1(
                xymat,
                varv,
                nx0,
                nx,
                ny,
                nobs,
                xvarv,
                markv,
                xmarkv,
                nmark,
                level,
                respn,
                param1n,
                param2n,
                covn,
                partv,
            );
            w = 0 as libc::c_int;
            while w < nvar {
                v = 0 as libc::c_int;
                while v < nvar {
                    *(*xymat.offset(v as isize)).offset(w as isize) = 0.0f64;
                    v += 1;
                    v;
                }
                w += 1;
                w;
            }
            nobs = 0 as libc::c_int;
        }
        if nobs != 0 {
            dnobs = nobs as libc::c_double;
            v = 1 as libc::c_int;
            while v < nvar {
                tmp = *(*xymat.offset(0 as libc::c_int as isize)).offset(v as isize)
                    - dnobs
                        * *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_dbl)
                            .offset(*varv.offset(v as isize) as isize);
                w = 1 as libc::c_int;
                while w < nvar {
                    if v < nx || w < nx || v == w {
                        *(*xymat.offset(v as isize)).offset(w as isize)
                            += tmp
                                * (*(*xymat.offset(0 as libc::c_int as isize))
                                    .offset(w as isize)
                                    - dnobs
                                        * *((*dap_obs
                                            .as_mut_ptr()
                                            .offset(0 as libc::c_int as isize))
                                            .do_dbl)
                                            .offset(*varv.offset(w as isize) as isize))
                                / (dnobs * (dnobs + 1.0f64));
                    }
                    w += 1;
                    w;
                }
                v += 1;
                v;
            }
        }
        w = 1 as libc::c_int;
        while w < nvar {
            *(*xymat.offset(0 as libc::c_int as isize)).offset(w as isize)
                += *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(*varv.offset(w as isize) as isize);
            w += 1;
            w;
        }
        nobs += 1;
        nobs;
    }
    dap_free(
        regname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        varv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        xvarv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        markv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        xmarkv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        xymem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        xymat as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        covset as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        partstr as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        partv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn dap_parsey(
    mut yspec: *mut libc::c_char,
    mut varv: *mut libc::c_int,
) {
    let mut l: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut vname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vn: libc::c_int = 0;
    let mut ntrials: libc::c_int = 0;
    vname = dap_malloc(
        dap_namelen + 1 as libc::c_int,
        b"dap_namelen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    l = 0 as libc::c_int;
    while *yspec.offset(l as isize) as libc::c_int == ' ' as i32 {
        l += 1;
        l;
    }
    i = 0 as libc::c_int;
    while *yspec.offset((l + i) as isize) as libc::c_int != 0
        && *yspec.offset((l + i) as isize) as libc::c_int != ' ' as i32
        && *yspec.offset((l + i) as isize) as libc::c_int != '/' as i32
    {
        if i < dap_namelen {
            *vname.offset(i as isize) = *yspec.offset((l + i) as isize);
        } else {
            fprintf(
                dap_err,
                b"(parsey) Variable name too long: %s\n\0" as *const u8
                    as *const libc::c_char,
                yspec.offset(l as isize),
            );
            exit(1 as libc::c_int);
        }
        i += 1;
        i;
    }
    *vname.offset(i as isize) = '\0' as i32 as libc::c_char;
    l += i;
    vn = dap_varnum(vname);
    if vn < 0 as libc::c_int {
        fprintf(
            dap_err,
            b"(parsey) Unknown variable: %s\n\0" as *const u8 as *const libc::c_char,
            vname,
        );
        exit(1 as libc::c_int);
    }
    if *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_len).offset(vn as isize)
        != -(1 as libc::c_int)
    {
        fprintf(
            dap_err,
            b"(parsey) Events variable not double: %s\n\0" as *const u8
                as *const libc::c_char,
            vname,
        );
        exit(1 as libc::c_int);
    }
    *varv.offset(0 as libc::c_int as isize) = vn;
    while *yspec.offset(l as isize) as libc::c_int == ' ' as i32 {
        l += 1;
        l;
    }
    if *yspec.offset(l as isize) as libc::c_int == '/' as i32 {
        l += 1;
        l;
        while *yspec.offset(l as isize) as libc::c_int == ' ' as i32 {
            l += 1;
            l;
        }
        i = 0 as libc::c_int;
        while *yspec.offset((l + i) as isize) as libc::c_int != 0
            && *yspec.offset((l + i) as isize) as libc::c_int != ' ' as i32
        {
            if i < dap_namelen {
                *vname.offset(i as isize) = *yspec.offset((l + i) as isize);
            } else {
                fprintf(
                    dap_err,
                    b"(parsey) Variable name too long: %s\n\0" as *const u8
                        as *const libc::c_char,
                    yspec.offset(l as isize),
                );
                exit(1 as libc::c_int);
            }
            i += 1;
            i;
        }
        *vname.offset(i as isize) = '\0' as i32 as libc::c_char;
        i = 0 as libc::c_int;
        ntrials = 0 as libc::c_int;
        while '0' as i32 <= *vname.offset(i as isize) as libc::c_int
            && *vname.offset(i as isize) as libc::c_int <= '9' as i32
        {
            ntrials = 10 as libc::c_int * ntrials
                + *vname.offset(i as isize) as libc::c_int - '0' as i32;
            i += 1;
            i;
        }
        if i != 0 {
            if *vname.offset(i as isize) != 0 {
                fprintf(
                    dap_err,
                    b"(parsey) Invalid number of trials: %s\n\0" as *const u8
                        as *const libc::c_char,
                    vname,
                );
                exit(1 as libc::c_int);
            }
            *varv.offset(1 as libc::c_int as isize) = -ntrials;
        } else {
            vn = dap_varnum(vname);
            if vn < 0 as libc::c_int {
                fprintf(
                    dap_err,
                    b"(parsey) Unknown variable: %s\n\0" as *const u8
                        as *const libc::c_char,
                    vname,
                );
                exit(1 as libc::c_int);
            }
            if *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_len)
                .offset(vn as isize) != -(1 as libc::c_int)
            {
                fprintf(
                    dap_err,
                    b"(parsey) Trials variable not double: %s\n\0" as *const u8
                        as *const libc::c_char,
                    vname,
                );
                exit(1 as libc::c_int);
            }
            *varv.offset(1 as libc::c_int as isize) = vn;
        }
    } else {
        fprintf(
            dap_err,
            b"(parsey) Expected / in yspec at: %s\n\0" as *const u8
                as *const libc::c_char,
            yspec.offset(l as isize),
        );
        exit(1 as libc::c_int);
    }
    dap_free(
        vname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn vlen(
    mut v: *mut libc::c_double,
    mut nv: libc::c_int,
) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut len: libc::c_double = 0.;
    len = 0.0f64;
    i = 0 as libc::c_int;
    while i < nv {
        len += *v.offset(i as isize) * *v.offset(i as isize);
        i += 1;
        i;
    }
    return sqrt(len);
}
unsafe extern "C" fn vdiff(
    mut v0: *mut libc::c_double,
    mut v1: *mut libc::c_double,
    mut nv: libc::c_int,
) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_double = 0.;
    let mut diff: libc::c_double = 0.;
    diff = 0.0f64;
    i = 0 as libc::c_int;
    while i < nv {
        tmp = *v0.offset(i as isize) - *v1.offset(i as isize);
        diff += tmp * tmp;
        i += 1;
        i;
    }
    return sqrt(diff);
}
pub unsafe extern "C" fn dap_invert(
    mut a: *mut *mut libc::c_double,
    mut nrc: libc::c_int,
) -> libc::c_int {
    let mut invmem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut inv: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut r: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut cc: libc::c_int = 0;
    let mut tmp: libc::c_double = 0.;
    let mut mult: libc::c_double = 0.;
    invmem = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nrc as libc::c_ulong)
            .wrapping_mul(nrc as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    inv = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nrc as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_double;
    r = 0 as libc::c_int;
    while r < nrc {
        let ref mut fresh4 = *inv.offset(r as isize);
        *fresh4 = invmem.offset((r * nrc) as isize);
        c = 0 as libc::c_int;
        while c < nrc {
            if r == c {
                *(*inv.offset(r as isize)).offset(c as isize) = 1.0f64;
            } else {
                *(*inv.offset(r as isize)).offset(c as isize) = 0.0f64;
            }
            c += 1;
            c;
        }
        r += 1;
        r;
    }
    c = 0 as libc::c_int;
    while c < nrc {
        if *(*a.offset(c as isize)).offset(c as isize) != 0.0f64 {
            tmp = *(*a.offset(c as isize)).offset(c as isize);
            r = c + 1 as libc::c_int;
            while r < nrc {
                mult = *(*a.offset(r as isize)).offset(c as isize) / tmp;
                *(*a.offset(r as isize)).offset(c as isize) = 0.0f64;
                cc = c + 1 as libc::c_int;
                while cc < nrc {
                    *(*a.offset(r as isize)).offset(cc as isize)
                        -= mult * *(*a.offset(c as isize)).offset(cc as isize);
                    if fabs(*(*a.offset(r as isize)).offset(cc as isize)) < dap_tol * tmp
                    {
                        *(*a.offset(r as isize)).offset(cc as isize) = 0.0f64;
                    }
                    cc += 1;
                    cc;
                }
                cc = 0 as libc::c_int;
                while cc < nrc {
                    *(*inv.offset(r as isize)).offset(cc as isize)
                        -= mult * *(*inv.offset(c as isize)).offset(cc as isize);
                    if fabs(*(*inv.offset(r as isize)).offset(cc as isize))
                        < dap_tol * tmp
                    {
                        *(*inv.offset(r as isize)).offset(cc as isize) = 0.0f64;
                    }
                    cc += 1;
                    cc;
                }
                r += 1;
                r;
            }
        } else {
            return 0 as libc::c_int
        }
        c += 1;
        c;
    }
    c = nrc - 1 as libc::c_int;
    while c >= 0 as libc::c_int {
        tmp = *(*a.offset(c as isize)).offset(c as isize);
        cc = c + 1 as libc::c_int;
        while cc < nrc {
            *(*a.offset(c as isize)).offset(cc as isize) /= tmp;
            cc += 1;
            cc;
        }
        cc = 0 as libc::c_int;
        while cc < nrc {
            *(*inv.offset(c as isize)).offset(cc as isize) /= tmp;
            cc += 1;
            cc;
        }
        r = c - 1 as libc::c_int;
        while r >= 0 as libc::c_int {
            tmp = *(*a.offset(r as isize)).offset(c as isize);
            cc = c;
            while cc < nrc {
                *(*a.offset(r as isize)).offset(cc as isize)
                    -= tmp * *(*a.offset(c as isize)).offset(cc as isize);
                cc += 1;
                cc;
            }
            cc = 0 as libc::c_int;
            while cc < nrc {
                *(*inv.offset(r as isize)).offset(cc as isize)
                    -= tmp * *(*inv.offset(c as isize)).offset(cc as isize);
                cc += 1;
                cc;
            }
            r -= 1;
            r;
        }
        c -= 1;
        c;
    }
    r = 0 as libc::c_int;
    while r < nrc {
        c = 0 as libc::c_int;
        while c < nrc {
            *(*a.offset(r as isize))
                .offset(c as isize) = *(*inv.offset(r as isize)).offset(c as isize);
            c += 1;
            c;
        }
        r += 1;
        r;
    }
    dap_free(
        invmem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        inv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn irls(
    mut x: *mut *mut libc::c_double,
    mut y: *mut *mut libc::c_double,
    mut pr: *mut libc::c_double,
    mut beta0: *mut libc::c_double,
    mut cov: *mut *mut libc::c_double,
    mut nx: libc::c_int,
    mut nobs: libc::c_int,
) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut beta1: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut v: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut step_0: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut loglike0: libc::c_double = 0.;
    let mut loglike1: libc::c_double = 0.;
    let mut niter: libc::c_int = 0;
    let mut maxv: libc::c_double = 0.;
    let mut maxcov: libc::c_double = 0.;
    let mut tmp: libc::c_double = 0.;
    beta1 = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nx as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    v = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nx as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    step_0 = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nx as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < nx {
        *beta1.offset(i as isize) = 0.0f64;
        i += 1;
        i;
    }
    loglike1 = 0.0f64;
    n = 0 as libc::c_int;
    while n < nobs {
        *pr.offset(n as isize) = 0.5f64;
        loglike1 += *(*y.offset(1 as libc::c_int as isize)).offset(n as isize);
        n += 1;
        n;
    }
    loglike1 *= log(0.5f64);
    niter = 0 as libc::c_int;
    loop {
        i = 0 as libc::c_int;
        while i < nx {
            *beta0.offset(i as isize) = *beta1.offset(i as isize);
            i += 1;
            i;
        }
        loglike0 = loglike1;
        i = 0 as libc::c_int;
        maxv = 0.0f64;
        maxcov = 0.0f64;
        while i < nx {
            *v.offset(i as isize) = 0.0f64;
            n = 0 as libc::c_int;
            while n < nobs {
                *v.offset(i as isize)
                    += *(*x.offset(i as isize)).offset(n as isize)
                        * (*(*y.offset(0 as libc::c_int as isize)).offset(n as isize)
                            - *(*y.offset(1 as libc::c_int as isize)).offset(n as isize)
                                * *pr.offset(n as isize));
                n += 1;
                n;
            }
            tmp = fabs(*v.offset(i as isize));
            if tmp > maxv {
                maxv = tmp;
            }
            j = 0 as libc::c_int;
            while j < nx {
                *(*cov.offset(i as isize)).offset(j as isize) = 0.0f64;
                n = 0 as libc::c_int;
                while n < nobs {
                    *(*cov.offset(i as isize)).offset(j as isize)
                        += *(*y.offset(1 as libc::c_int as isize)).offset(n as isize)
                            * *pr.offset(n as isize) * (1.0f64 - *pr.offset(n as isize))
                            * *(*x.offset(i as isize)).offset(n as isize)
                            * *(*x.offset(j as isize)).offset(n as isize);
                    n += 1;
                    n;
                }
                tmp = fabs(*(*cov.offset(i as isize)).offset(j as isize));
                if tmp > maxcov {
                    maxcov = tmp;
                }
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        while i < nx {
            if fabs(*v.offset(i as isize)) < dap_ctol * maxv {
                *v.offset(i as isize) = 0.0f64;
            }
            j = 0 as libc::c_int;
            while j < nx {
                if fabs(*(*cov.offset(i as isize)).offset(j as isize))
                    < dap_ctol * maxcov
                {
                    *(*cov.offset(i as isize)).offset(j as isize) = 0.0f64;
                }
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        if dap_invert(cov, nx) == 0 {
            fputs(
                b"(irls) X'DX matrix is singular\n\0" as *const u8
                    as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        i = 0 as libc::c_int;
        while i < nx {
            *step_0.offset(i as isize) = 0.0f64;
            j = 0 as libc::c_int;
            while j < nx {
                *step_0.offset(i as isize)
                    += *(*cov.offset(i as isize)).offset(j as isize)
                        * *v.offset(j as isize);
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        while niter <= dap_maxiter {
            i = 0 as libc::c_int;
            while i < nx {
                *beta1
                    .offset(
                        i as isize,
                    ) = *beta0.offset(i as isize) + *step_0.offset(i as isize);
                i += 1;
                i;
            }
            n = 0 as libc::c_int;
            loglike1 = 0.0f64;
            while n < nobs {
                *pr.offset(n as isize) = 0.0f64;
                i = 0 as libc::c_int;
                while i < nx {
                    *pr.offset(n as isize)
                        += *(*x.offset(i as isize)).offset(n as isize)
                            * *beta1.offset(i as isize);
                    i += 1;
                    i;
                }
                *pr
                    .offset(
                        n as isize,
                    ) = 1.0f64 / (1.0f64 + exp(-*pr.offset(n as isize)));
                loglike1
                    += *(*y.offset(0 as libc::c_int as isize)).offset(n as isize)
                        * log(*pr.offset(n as isize))
                        + (*(*y.offset(1 as libc::c_int as isize)).offset(n as isize)
                            - *(*y.offset(0 as libc::c_int as isize)).offset(n as isize))
                            * log(1.0f64 - *pr.offset(n as isize));
                n += 1;
                n;
            }
            if loglike1 >= loglike0 {
                break;
            }
            i = 0 as libc::c_int;
            while i < nx {
                *step_0.offset(i as isize) *= 0.5f64;
                i += 1;
                i;
            }
            niter += 1;
            niter;
        }
        niter += 1;
        if !(niter <= dap_maxiter
            && vdiff(beta1, beta0, nx) > dap_ctol * vlen(beta0, nx))
        {
            break;
        }
    }
    if niter > dap_maxiter {
        fprintf(
            dap_lst,
            b"Failed to converge after %d iterations.\n\0" as *const u8
                as *const libc::c_char,
            dap_maxiter,
        );
    }
    dap_free(
        beta1 as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        v as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        step_0 as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return loglike0;
}
pub unsafe extern "C" fn logreg1(
    mut y: *mut *mut libc::c_double,
    mut x: *mut *mut libc::c_double,
    mut nx0: libc::c_int,
    mut nx: libc::c_int,
    mut nobs: libc::c_int,
    mut varv: *mut libc::c_int,
    mut xvarv: *mut libc::c_int,
    mut markv: *mut libc::c_int,
    mut xmarkv: *mut libc::c_int,
    mut nmark: libc::c_int,
    mut level: libc::c_double,
    mut param1n: libc::c_int,
    mut param2n: libc::c_int,
    mut covn: libc::c_int,
    mut partv: *mut libc::c_int,
) {
    let mut typen: libc::c_int = 0;
    let mut covmem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut cov: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut pr: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut beta: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tmp: libc::c_double = 0.;
    let mut tmp2: libc::c_double = 0.;
    let mut loglike0: libc::c_double = 0.;
    let mut loglike1: libc::c_double = 0.;
    static mut npt: libc::c_double = 0.;
    let mut xi: libc::c_double = 0.;
    let mut xj: libc::c_double = 0.;
    let mut logit: libc::c_double = 0.;
    let mut selogit: libc::c_double = 0.;
    let mut ntrials: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    covmem = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    cov = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < dap_maxvar {
        let ref mut fresh5 = *cov.offset(i as isize);
        *fresh5 = covmem.offset((i * dap_maxvar) as isize);
        i += 1;
        i;
    }
    beta = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nx as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    dap_swap();
    typen = dap_varnum(
        b"_type_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if typen < 0 as libc::c_int {
        fprintf(
            dap_err,
            b"(logreg1) Missing _type_ variable.\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    pr = dap_malloc(
        (nobs as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    fprintf(
        dap_lst,
        b"Reduced | full model regressors:\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < nx0 {
        fprintf(
            dap_lst,
            b" %s\0" as *const u8 as *const libc::c_char,
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                .offset(*varv.offset(i as isize) as isize),
        );
        i += 1;
        i;
    }
    fprintf(dap_lst, b" |\0" as *const u8 as *const libc::c_char);
    while i < nx {
        let fresh6 = i;
        i = i + 1;
        fprintf(
            dap_lst,
            b" %s\0" as *const u8 as *const libc::c_char,
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                .offset(*varv.offset(fresh6 as isize) as isize),
        );
    }
    putc('\n' as i32, dap_lst);
    fprintf(
        dap_lst,
        b"Number of observations = %d\n\0" as *const u8 as *const libc::c_char,
        nobs,
    );
    i = 0 as libc::c_int;
    ntrials = 0 as libc::c_int;
    while i < nobs {
        ntrials
            += rint(*(*y.offset(1 as libc::c_int as isize)).offset(i as isize))
                as libc::c_int;
        i += 1;
        i;
    }
    fprintf(
        dap_lst,
        b"Number of trials = %d\n\0" as *const u8 as *const libc::c_char,
        ntrials,
    );
    if *varv.offset((nx + 1 as libc::c_int) as isize) >= 0 as libc::c_int {
        fprintf(
            dap_lst,
            b"Events / Trials: %s / %s\n\0" as *const u8 as *const libc::c_char,
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                .offset(*varv.offset(nx as isize) as isize),
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                .offset(*varv.offset((nx + 1 as libc::c_int) as isize) as isize),
        );
    } else {
        fprintf(
            dap_lst,
            b"Events / Trials: %s / %d\n\0" as *const u8 as *const libc::c_char,
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                .offset(*varv.offset(nx as isize) as isize),
            -*varv.offset((nx + 1 as libc::c_int) as isize),
        );
    }
    loglike0 = irls(x, y, pr, beta, cov, nx0, nobs);
    loglike1 = irls(x, y, pr, beta, cov, nx, nobs);
    dap_free(
        pr as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_ono = 2 as libc::c_int;
    v = 0 as libc::c_int;
    while v < nmark {
        if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
            .offset(*markv.offset(v as isize) as isize) == -(1 as libc::c_int)
        {
            *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_dbl)
                .offset(
                    *partv.offset(v as isize) as isize,
                ) = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(*markv.offset(v as isize) as isize);
        } else if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
            .offset(*markv.offset(v as isize) as isize) == 0 as libc::c_int
        {
            *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_int)
                .offset(
                    *partv.offset(v as isize) as isize,
                ) = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_int)
                .offset(*markv.offset(v as isize) as isize);
        } else {
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                    .offset(*partv.offset(v as isize) as isize),
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(*markv.offset(v as isize) as isize),
            );
        }
        v += 1;
        v;
    }
    tmp = -2.0f64 * (loglike0 - loglike1);
    fprintf(
        dap_lst,
        b"-2 (Lred - Lfull) = %.6g = ChiSq0[%d]\n\0" as *const u8 as *const libc::c_char,
        tmp,
        nx - nx0,
    );
    fprintf(
        dap_lst,
        b"Prob[ChiSq > ChiSq0] = %.5f\n\n\0" as *const u8 as *const libc::c_char,
        0.00001f64 * ceil(100000.0f64 * probchisq(fabs(tmp), nx - nx0)),
    );
    fputs(
        b"  Parameter           Estimate    Std Error   Wald ChiSq  Prob[ChiSq>Wald ChiSq]\n\0"
            as *const u8 as *const libc::c_char,
        dap_lst,
    );
    i = 0 as libc::c_int;
    while i < nx {
        tmp = sqrt(*(*cov.offset(i as isize)).offset(i as isize));
        tmp2 = *beta.offset(i as isize) * *beta.offset(i as isize)
            / *(*cov.offset(i as isize)).offset(i as isize);
        fprintf(
            dap_lst,
            b"  %-15s %12.6g %12.6g %12.6g  %14.5f\n\0" as *const u8
                as *const libc::c_char,
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                .offset(*varv.offset(i as isize) as isize),
            *beta.offset(i as isize),
            tmp,
            tmp2,
            0.00001f64 * ceil(100000.0f64 * probchisq(fabs(tmp2), 1 as libc::c_int)),
        );
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                .offset(typen as isize),
            b"ESTIMATE\0" as *const u8 as *const libc::c_char,
        );
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                .offset(param2n as isize),
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                .offset(*varv.offset(i as isize) as isize),
        );
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                .offset(param1n as isize),
            b"\0" as *const u8 as *const libc::c_char,
        );
        *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_dbl)
            .offset(covn as isize) = *beta.offset(i as isize);
        output();
        i += 1;
        i;
    }
    strcpy(
        *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
            .offset(typen as isize),
        b"COVAR\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < nx {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                .offset(param1n as isize),
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                .offset(*varv.offset(i as isize) as isize),
        );
        j = 0 as libc::c_int;
        while j < nx {
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                    .offset(param2n as isize),
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(*varv.offset(j as isize) as isize),
            );
            *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_dbl)
                .offset(covn as isize) = *(*cov.offset(i as isize)).offset(j as isize);
            output();
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    dap_ono = 0 as libc::c_int;
    if fabs(level) < 1.0f64 {
        npt = -zpoint(0.5f64 * (1.0f64 - level));
    } else {
        npt = 0.0f64;
    }
    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
        .offset(*varv.offset(0 as libc::c_int as isize) as isize) = 1.0f64;
    if *xvarv.offset(0 as libc::c_int as isize) < 0 as libc::c_int {
        dap_rewind();
        step();
    }
    while matchmark(markv, xmarkv, nmark, level) != 0 {
        dap_ono = 0 as libc::c_int;
        if *xvarv.offset(0 as libc::c_int as isize) >= 0 as libc::c_int {
            i = 1 as libc::c_int;
            while i < nx {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        *varv.offset(i as isize) as isize,
                    ) = *((*dap_obs.as_mut_ptr().offset(1 as libc::c_int as isize))
                    .do_dbl)
                    .offset(*xvarv.offset((i - 1 as libc::c_int) as isize) as isize);
                i += 1;
                i;
            }
        }
        logit = 0.0f64;
        i = 0 as libc::c_int;
        while i < nx {
            logit
                += *beta.offset(i as isize)
                    * *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                        .offset(*varv.offset(i as isize) as isize);
            i += 1;
            i;
        }
        selogit = 0.0f64;
        i = 0 as libc::c_int;
        while i < nx {
            xi = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(*varv.offset(i as isize) as isize);
            j = 0 as libc::c_int;
            while j < nx {
                xj = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(*varv.offset(j as isize) as isize);
                selogit += xi * *(*cov.offset(i as isize)).offset(j as isize) * xj;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        selogit = sqrt(selogit);
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typen as isize),
            b"PRED\0" as *const u8 as *const libc::c_char,
        );
        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
            .offset(
                *varv.offset(nx as isize) as isize,
            ) = 1.0f64 / (1.0f64 + exp(-logit));
        output();
        if npt != 0.0f64 {
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                b"LOWER\0" as *const u8 as *const libc::c_char,
            );
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(
                    *varv.offset(nx as isize) as isize,
                ) = 1.0f64 / (1.0f64 + exp(-logit + npt * selogit));
            output();
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                b"UPPER\0" as *const u8 as *const libc::c_char,
            );
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(
                    *varv.offset(nx as isize) as isize,
                ) = 1.0f64 / (1.0f64 + exp(-logit - npt * selogit));
            output();
        }
        if *xvarv.offset(0 as libc::c_int as isize) >= 0 as libc::c_int {
            dap_ono = 1 as libc::c_int;
        }
        dap_mark();
        if step() == 0 {
            break;
        }
    }
    dap_ono = 0 as libc::c_int;
    if *xvarv.offset(0 as libc::c_int as isize) >= 0 as libc::c_int {
        dap_swap();
    }
    dap_free(
        covmem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        cov as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        beta as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn logreg(
    mut fname: *mut libc::c_char,
    mut yspec: *mut libc::c_char,
    mut x0list: *mut libc::c_char,
    mut x1list: *mut libc::c_char,
    mut marks: *mut libc::c_char,
    mut xname: *mut libc::c_char,
    mut level: libc::c_double,
) {
    let mut regname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut varv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut xvarv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nx0: libc::c_int = 0;
    let mut nx1: libc::c_int = 0;
    let mut nx: libc::c_int = 0;
    let mut nxx: libc::c_int = 0;
    let mut markv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut xmarkv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nmark: libc::c_int = 0;
    let mut xmem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut x: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut ymem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut y: [*mut libc::c_double; 2] = [0 as *mut libc::c_double; 2];
    let mut v: libc::c_int = 0;
    let mut nobs: libc::c_int = 0;
    let mut more: libc::c_int = 0;
    let mut covset: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut param1n: libc::c_int = 0;
    let mut param2n: libc::c_int = 0;
    let mut covn: libc::c_int = 0;
    let mut paramlen1: libc::c_int = 0;
    let mut paramlen: libc::c_int = 0;
    let mut paramstr: [libc::c_char; 14] = [0; 14];
    let mut partstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut partv: *mut libc::c_int = 0 as *mut libc::c_int;
    if fname.is_null() {
        fputs(
            b"(logreg) No dataset name given.\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    varv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"dap_maxvar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    xvarv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"dap_maxvar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    markv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"dap_maxvar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    xmarkv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"dap_maxvar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    partv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"dap_maxvar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    regname = dap_malloc(
        (strlen(fname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_suffix(
        regname,
        fname,
        b".lgr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    inset(fname);
    dap_vd(
        b"_intercept_ -1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    nx0 = dap_list(
        b"_intercept_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        varv,
        dap_maxvar,
    );
    nx0 += dap_list(x0list, varv.offset(1 as libc::c_int as isize), dap_maxvar);
    nx1 = dap_list(x1list, varv.offset(nx0 as isize), dap_maxvar);
    nx = nx0 + nx1;
    dap_parsey(yspec, varv.offset(nx as isize));
    xmem = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nx as libc::c_ulong)
            .wrapping_mul(dap_maxval as libc::c_ulong) as libc::c_int,
        b"dap_maxval\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    x = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nx as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_double;
    v = 0 as libc::c_int;
    while v < nx {
        let ref mut fresh7 = *x.offset(v as isize);
        *fresh7 = xmem.offset((v * dap_maxval) as isize);
        v += 1;
        v;
    }
    ymem = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(dap_maxval as libc::c_ulong) as libc::c_int,
        b"dap_maxval\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    y[0 as libc::c_int as usize] = ymem;
    y[1 as libc::c_int as usize] = ymem.offset(dap_maxval as isize);
    nmark = dap_list(marks, markv, dap_maxvar);
    covset = dap_malloc(
        (strlen(fname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(covset, fname);
    strcat(covset, b".cov\0" as *const u8 as *const libc::c_char);
    v = 1 as libc::c_int;
    paramlen = strlen(b"_intercept_\0" as *const u8 as *const libc::c_char)
        as libc::c_int;
    while v < nx {
        paramlen1 = strlen(
            *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_nam)
                .offset(*varv.offset(v as isize) as isize),
        ) as libc::c_int;
        if paramlen1 > paramlen {
            paramlen = paramlen1;
        }
        v += 1;
        v;
    }
    dap_ono = 2 as libc::c_int;
    dap_clearobs(0 as *mut libc::c_void as *mut libc::c_char);
    sprintf(
        paramstr.as_mut_ptr(),
        b"_param1_ %d\0" as *const u8 as *const libc::c_char,
        paramlen,
    );
    param1n = dap_vd(paramstr.as_mut_ptr(), 0 as libc::c_int);
    sprintf(
        paramstr.as_mut_ptr(),
        b"_param2_ %d\0" as *const u8 as *const libc::c_char,
        paramlen,
    );
    param2n = dap_vd(paramstr.as_mut_ptr(), 0 as libc::c_int);
    covn = dap_vd(
        b"_cov_ -1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    partstr = dap_malloc(
        (strlen(marks)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    v = 0 as libc::c_int;
    while v < nmark {
        strcpy(
            partstr,
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                .offset(*markv.offset(v as isize) as isize),
        );
        sprintf(
            partstr.offset(strlen(partstr) as isize),
            b" %d\0" as *const u8 as *const libc::c_char,
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
                .offset(*markv.offset(v as isize) as isize),
        );
        *partv.offset(v as isize) = dap_vd(partstr, 1 as libc::c_int);
        v += 1;
        v;
    }
    outset(covset, b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    dap_ono = 1 as libc::c_int;
    if !xname.is_null() && *xname.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        inset(xname);
        nxx = dap_list(x0list, xvarv, dap_maxvar);
        nxx += dap_list(x1list, xvarv.offset(nxx as isize), dap_maxvar);
        if nxx != nx - 1 as libc::c_int {
            fprintf(
                dap_err,
                b"(logreg) %s and %s have different numbers (%d and %d) of x-variables.\n\0"
                    as *const u8 as *const libc::c_char,
                fname,
                xname,
                nx - 1 as libc::c_int,
                nxx,
            );
            exit(1 as libc::c_int);
        }
        if nmark != 0 {
            v = 0 as libc::c_int;
            while v < nmark {
                let ref mut fresh8 = *xmarkv.offset(v as isize);
                *fresh8 = dap_varnum(
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                        .offset(*markv.offset(v as isize) as isize),
                );
                if *fresh8 < 0 as libc::c_int {
                    fprintf(
                        dap_err,
                        b"(logreg) Mark variable %s in %s not in %s.\n\0" as *const u8
                            as *const libc::c_char,
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_nam)
                            .offset(*markv.offset(v as isize) as isize),
                        fname,
                        xname,
                    );
                    exit(1 as libc::c_int);
                }
                v += 1;
                v;
            }
        }
        if step() == 0 {
            fprintf(
                dap_err,
                b"(logreg) No data in %s.\n\0" as *const u8 as *const libc::c_char,
                if *xname.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
                    xname
                } else {
                    fname
                },
            );
            exit(1 as libc::c_int);
        }
    } else {
        *xvarv.offset(0 as libc::c_int as isize) = -(1 as libc::c_int);
        *xmarkv.offset(0 as libc::c_int as isize) = -(1 as libc::c_int);
    }
    dap_ono = 0 as libc::c_int;
    outset(regname, b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    nobs = 0 as libc::c_int;
    dap_mark();
    more = 1 as libc::c_int;
    while more != 0 {
        more = step();
        if dap_newpart(markv, nmark) != 0 {
            dap_swap();
            dap_head(markv, nmark);
            dap_swap();
            logreg1(
                y.as_mut_ptr(),
                x,
                nx0,
                nx,
                nobs,
                varv,
                xvarv,
                markv,
                xmarkv,
                nmark,
                level,
                param1n,
                param2n,
                covn,
                partv,
            );
            nobs = 0 as libc::c_int;
        }
        if nobs < dap_maxval {
            *(*x.offset(0 as libc::c_int as isize)).offset(nobs as isize) = 1.0f64;
            v = 1 as libc::c_int;
            while v < nx {
                *(*x.offset(v as isize))
                    .offset(
                        nobs as isize,
                    ) = *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_dbl)
                    .offset(*varv.offset(v as isize) as isize);
                v += 1;
                v;
            }
            *(y[0 as libc::c_int as usize])
                .offset(
                    nobs as isize,
                ) = *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_dbl)
                .offset(*varv.offset(nx as isize) as isize);
            if *varv.offset((nx + 1 as libc::c_int) as isize) >= 0 as libc::c_int {
                *(y[1 as libc::c_int as usize])
                    .offset(
                        nobs as isize,
                    ) = *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_dbl)
                    .offset(*varv.offset((nx + 1 as libc::c_int) as isize) as isize);
            } else {
                *(y[1 as libc::c_int as usize])
                    .offset(
                        nobs as isize,
                    ) = -(*varv.offset((nx + 1 as libc::c_int) as isize)
                    as libc::c_double);
            }
        } else {
            fputs(
                b"(logreg) Too many data.\n\0" as *const u8 as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        nobs += 1;
        nobs;
    }
    dap_free(
        regname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        varv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        xvarv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        markv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        xmarkv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        xmem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        x as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        ymem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        covset as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        partstr as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        partv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn valcmp1(mut v1: *mut value, mut v2: *mut value) -> libc::c_int {
    if (*v1).val_val < (*v2).val_val {
        return -(1 as libc::c_int);
    }
    if (*v1).val_val > (*v2).val_val {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn probkol(
    mut d: libc::c_double,
    mut n: libc::c_double,
) -> libc::c_double {
    let mut lambda: libc::c_double = 0.;
    let mut l: libc::c_double = 0.;
    let mut k: libc::c_double = 0.;
    let mut term: libc::c_double = 0.;
    let mut sign: libc::c_int = 0;
    if d == 0.0f64 {
        return 0.0f64;
    }
    lambda = d * sqrt(n);
    lambda *= -2.0f64 * lambda;
    k = 1.0f64;
    l = 0.0f64;
    sign = 1 as libc::c_int;
    loop {
        term = exp(k * k * lambda);
        if !(term > dap_ktol) {
            break;
        }
        l += if sign > 0 as libc::c_int { term } else { -term };
        k += 1.0f64;
        sign = -sign;
    }
    return 2.0f64 * l;
}
static mut pvalcmp1: Option::<unsafe extern "C" fn() -> libc::c_int> = unsafe {
    ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut value, *mut value) -> libc::c_int>,
        Option::<unsafe extern "C" fn() -> libc::c_int>,
    >(Some(valcmp1 as unsafe extern "C" fn(*mut value, *mut value) -> libc::c_int))
};
unsafe extern "C" fn nonpar1(
    mut val: *mut value,
    mut nval: libc::c_int,
    mut level: *mut *mut libc::c_char,
    mut nlevels: libc::c_int,
    mut varv: *mut libc::c_int,
    mut nvar: libc::c_int,
) {
    let mut rank: libc::c_int = 0;
    let mut ntied: libc::c_int = 0;
    let mut tied: libc::c_int = 0;
    let mut tottied: libc::c_int = 0;
    let mut tiecorr: libc::c_double = 0.;
    let mut drank: libc::c_double = 0.;
    let mut dn: libc::c_double = 0.;
    let mut stat0: libc::c_double = 0.;
    let mut stat: libc::c_double = 0.;
    let mut prob: libc::c_double = 0.;
    let mut levnobs: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut minnobs: libc::c_int = 0;
    let mut rank1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut r: libc::c_int = 0;
    let mut levn: libc::c_int = 0;
    let mut sumr: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut tmp: libc::c_double = 0.;
    let mut tmp1: libc::c_double = 0.;
    let mut kold: libc::c_double = 0.;
    let mut kolr: libc::c_int = 0;
    let mut kolval: libc::c_double = 0.;
    let mut f: [libc::c_double; 2] = [0.; 2];
    tmp = 0.0f64;
    tmp1 = 0.0f64;
    kolr = 0 as libc::c_int;
    kolval = 0.0f64;
    dap_swap();
    levnobs = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(nlevels as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    sumr = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nlevels as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    rank1 = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxex2 as libc::c_ulong) as libc::c_int,
        b"dap_maxex2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    dn = nval as libc::c_double;
    if nvar == 2 as libc::c_int {
        qsort(
            val as *mut libc::c_void,
            nval as size_t,
            ::std::mem::size_of::<value>() as libc::c_ulong,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> libc::c_int>,
                __compar_fn_t,
            >(pvalcmp1),
        );
        levn = 0 as libc::c_int;
        while levn < nlevels {
            *levnobs.offset(levn as isize) = 0 as libc::c_int;
            levn += 1;
            levn;
        }
        rank = 0 as libc::c_int;
        while rank < nval {
            let ref mut fresh9 = *levnobs
                .offset((*val.offset(rank as isize)).val_class as isize);
            *fresh9 += 1;
            *fresh9;
            rank += 1;
            rank;
        }
        if nlevels == 2 as libc::c_int {
            rank = 0 as libc::c_int;
            stat0 = 0.0f64;
            stat = 0.0f64;
            tottied = 0 as libc::c_int;
            tiecorr = 0.0f64;
            f[0 as libc::c_int as usize] = 0.0f64;
            f[1 as libc::c_int as usize] = 0.0f64;
            kold = 0.0f64;
            while rank < nval {
                ntied = 1 as libc::c_int;
                while rank + ntied < nval
                    && fabs(
                        (*val.offset((rank + ntied) as isize)).val_val
                            - (*val.offset(rank as isize)).val_val,
                    )
                        < dap_tol
                            * (fabs((*val.offset((rank + ntied) as isize)).val_val)
                                + fabs((*val.offset(rank as isize)).val_val))
                {
                    ntied += 1;
                    ntied;
                }
                drank = (2 as libc::c_int * rank + ntied + 1 as libc::c_int)
                    as libc::c_double / 2.0f64;
                if ntied > 1 as libc::c_int {
                    tottied += ntied;
                    tiecorr
                        += (ntied * (ntied + 1 as libc::c_int)
                            * (ntied - 1 as libc::c_int)) as libc::c_double;
                }
                tied = 0 as libc::c_int;
                while tied < ntied {
                    if (*val.offset(rank as isize)).val_class != 0 {
                        stat0 += drank;
                        f[1 as libc::c_int as usize] += 1.0f64;
                    } else {
                        stat += drank;
                        f[0 as libc::c_int as usize] += 1.0f64;
                    }
                    tmp1 = (*val.offset(rank as isize)).val_val;
                    (*val.offset(rank as isize)).val_val = drank;
                    tied += 1;
                    tied;
                    rank += 1;
                    rank;
                }
                tmp = fabs(
                    f[1 as libc::c_int as usize]
                        / *levnobs.offset(1 as libc::c_int as isize) as libc::c_double
                        - f[0 as libc::c_int as usize]
                            / *levnobs.offset(0 as libc::c_int as isize)
                                as libc::c_double,
                );
                if tmp > kold {
                    kold = tmp;
                    kolr = rank;
                    kolval = tmp1;
                }
            }
            fprintf(
                dap_lst,
                b"\nResponse: %s\n\0" as *const u8 as *const libc::c_char,
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(*varv.offset(0 as libc::c_int as isize) as isize),
            );
            fprintf(
                dap_lst,
                b"Classified by %s:\0" as *const u8 as *const libc::c_char,
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(*varv.offset(1 as libc::c_int as isize) as isize),
            );
            r = 0 as libc::c_int;
            while r < nlevels {
                fprintf(
                    dap_lst,
                    b" %s (%d)\0" as *const u8 as *const libc::c_char,
                    *level.offset(r as isize),
                    *levnobs.offset(r as isize),
                );
                r += 1;
                r;
            }
            putc('\n' as i32, dap_lst);
            fprintf(
                dap_lst,
                b"Number of tied observations = %d\n\0" as *const u8
                    as *const libc::c_char,
                tottied,
            );
            if *levnobs.offset(0 as libc::c_int as isize)
                < *levnobs.offset(1 as libc::c_int as isize)
            {
                levn = 0 as libc::c_int;
                stat0 = stat;
            } else {
                levn = 1 as libc::c_int;
            }
            fprintf(
                dap_lst,
                b"\nWilcoxon rank sum statistic S0 (%s) = %g\n\0" as *const u8
                    as *const libc::c_char,
                *level.offset(0 as libc::c_int as isize),
                stat0,
            );
            fprintf(
                dap_lst,
                b"Expected under H0 = %g\n\0" as *const u8 as *const libc::c_char,
                *levnobs.offset(levn as isize) as libc::c_double * (dn + 1.0f64) / 2.0f64,
            );
            stat0 = fabs(
                stat0
                    - *levnobs.offset(levn as isize) as libc::c_double * (dn + 1.0f64)
                        / 2.0f64,
            );
            if nval < dap_maxex2 {
                r = 0 as libc::c_int;
                while r < *levnobs.offset(levn as isize) {
                    *rank1.offset(r as isize) = r;
                    r += 1;
                    r;
                }
                *rank1.offset(r as isize) = nval;
                prob = 0.0f64;
                loop {
                    r = 0 as libc::c_int;
                    stat = -(*levnobs.offset(levn as isize) as libc::c_double)
                        * (dn + 1.0f64) / 2.0f64;
                    while r < *levnobs.offset(levn as isize) {
                        stat
                            += (*val.offset(*rank1.offset(r as isize) as isize)).val_val;
                        r += 1;
                        r;
                    }
                    if fabs(stat) >= stat0 {
                        prob += 1.0f64;
                    }
                    r = *levnobs.offset(levn as isize) - 1 as libc::c_int;
                    while r >= 0 as libc::c_int
                        && *rank1.offset(r as isize) + 1 as libc::c_int
                            == *rank1.offset((r + 1 as libc::c_int) as isize)
                    {
                        r -= 1;
                        r;
                    }
                    if !(r >= 0 as libc::c_int) {
                        break;
                    }
                    let ref mut fresh10 = *rank1.offset(r as isize);
                    *fresh10 += 1;
                    *fresh10;
                    r += 1;
                    r;
                    while r < *levnobs.offset(levn as isize) {
                        *rank1
                            .offset(
                                r as isize,
                            ) = *rank1.offset((r - 1 as libc::c_int) as isize)
                            + 1 as libc::c_int;
                        r += 1;
                        r;
                    }
                }
                fprintf(
                    dap_lst,
                    b"Prob[|S - E(S)| >= |S0 - E(S)|] = %.4g (exact)\n\0" as *const u8
                        as *const libc::c_char,
                    prob
                        / dap_bincoeff(
                            dn,
                            *levnobs.offset(levn as isize) as libc::c_double,
                        ),
                );
            } else {
                if stat0 >= 0.5f64 {
                    stat0 -= 0.5f64;
                }
                prob = 2.0f64
                    * (1.0f64
                        - probz(
                            stat0
                                / sqrt(
                                    (*levnobs.offset(0 as libc::c_int as isize)
                                        * *levnobs.offset(1 as libc::c_int as isize))
                                        as libc::c_double
                                        * (dn + 1.0f64 - tiecorr / (dn * (dn - 1.0f64))) / 12.0f64,
                                ),
                        ));
                fprintf(
                    dap_lst,
                    b"Prob[|S - E(S)| >= |S0 - E(S)|] = %.4g\n\0" as *const u8
                        as *const libc::c_char,
                    prob,
                );
                fputs(
                    b"(normal approximation, with continuity correction)\n\0"
                        as *const u8 as *const libc::c_char,
                    dap_lst,
                );
            }
            fprintf(
                dap_lst,
                b"\nKolmogorov statistic D0 = %g\n\0" as *const u8
                    as *const libc::c_char,
                kold,
            );
            fprintf(
                dap_lst,
                b"Maximum deviation at %g for level %s\n\0" as *const u8
                    as *const libc::c_char,
                kolval,
                *level.offset((*val.offset(kolr as isize)).val_class as isize),
            );
            fprintf(
                dap_lst,
                b"Prob[D >= D0] = %.4g\n\0" as *const u8 as *const libc::c_char,
                probkol(
                    kold,
                    (*levnobs.offset(0 as libc::c_int as isize)
                        * *levnobs.offset(1 as libc::c_int as isize)) as libc::c_double
                        / dn,
                ),
            );
        } else if nlevels > 2 as libc::c_int {
            levn = 0 as libc::c_int;
            while levn < nlevels {
                *sumr.offset(levn as isize) = 0.0f64;
                levn += 1;
                levn;
            }
            rank = 0 as libc::c_int;
            tottied = 0 as libc::c_int;
            tiecorr = 0.0f64;
            while rank < nval {
                ntied = 1 as libc::c_int;
                while rank + ntied < nval
                    && fabs(
                        (*val.offset((rank + ntied) as isize)).val_val
                            - (*val.offset(rank as isize)).val_val,
                    )
                        < dap_tol
                            * (fabs((*val.offset((rank + ntied) as isize)).val_val)
                                + fabs((*val.offset(rank as isize)).val_val))
                {
                    ntied += 1;
                    ntied;
                }
                drank = (2 as libc::c_int * rank + ntied + 1 as libc::c_int)
                    as libc::c_double / 2.0f64;
                if ntied > 1 as libc::c_int {
                    tottied += ntied;
                    tiecorr
                        += (ntied * (ntied + 1 as libc::c_int)
                            * (ntied - 1 as libc::c_int)) as libc::c_double;
                }
                tied = 0 as libc::c_int;
                while tied < ntied {
                    *sumr.offset((*val.offset(rank as isize)).val_class as isize)
                        += drank;
                    (*val.offset(rank as isize)).val_val = drank;
                    tied += 1;
                    tied;
                    rank += 1;
                    rank;
                }
            }
            stat0 = 0.0f64;
            levn = 0 as libc::c_int;
            while levn < nlevels {
                tmp = *sumr.offset(levn as isize)
                    / *levnobs.offset(levn as isize) as libc::c_double
                    - 0.5f64 * (dn + 1.0f64);
                stat0 += tmp * tmp * *levnobs.offset(levn as isize) as libc::c_double;
                levn += 1;
                levn;
            }
            stat0 *= 12.0f64 / (dn * (dn + 1.0f64) - tiecorr / (dn - 1.0f64));
            fprintf(
                dap_lst,
                b"\nResponse: %s\n\0" as *const u8 as *const libc::c_char,
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(*varv.offset(0 as libc::c_int as isize) as isize),
            );
            fprintf(
                dap_lst,
                b"Classified by %s:\0" as *const u8 as *const libc::c_char,
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(*varv.offset(1 as libc::c_int as isize) as isize),
            );
            levn = 0 as libc::c_int;
            minnobs = *levnobs.offset(0 as libc::c_int as isize);
            while levn < nlevels {
                fprintf(
                    dap_lst,
                    b" %s (%d)\0" as *const u8 as *const libc::c_char,
                    *level.offset(levn as isize),
                    *levnobs.offset(levn as isize),
                );
                if *levnobs.offset(levn as isize) < minnobs {
                    minnobs = *levnobs.offset(levn as isize);
                }
                levn += 1;
                levn;
            }
            putc('\n' as i32, dap_lst);
            fprintf(
                dap_lst,
                b"Number of tied observations = %d\n\0" as *const u8
                    as *const libc::c_char,
                tottied,
            );
            fprintf(
                dap_lst,
                b"Kruskal-Wallis statistic T0 = %g\n\0" as *const u8
                    as *const libc::c_char,
                stat0,
            );
            prob = probchisq(stat0, nlevels - 1 as libc::c_int);
            fprintf(
                dap_lst,
                b"Prob[T >= T0] = %g (chi-squared approximation, df = %d)\n\0"
                    as *const u8 as *const libc::c_char,
                prob,
                nlevels - 1 as libc::c_int,
            );
            if nlevels == 3 as libc::c_int && minnobs < 6 as libc::c_int
                || minnobs < 5 as libc::c_int
            {
                fputs(
                    b"Warning: sample may be too small for this approximation.\n\0"
                        as *const u8 as *const libc::c_char,
                    dap_lst,
                );
            }
        }
    } else {
        fprintf(
            dap_lst,
            b"\nResponse: %s\n\0" as *const u8 as *const libc::c_char,
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                .offset(*varv.offset(0 as libc::c_int as isize) as isize),
        );
        fprintf(
            dap_lst,
            b"Number of non-zero observations = %d\n\0" as *const u8
                as *const libc::c_char,
            nval,
        );
        qsort(
            val as *mut libc::c_void,
            nval as size_t,
            ::std::mem::size_of::<value>() as libc::c_ulong,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> libc::c_int>,
                __compar_fn_t,
            >(pvalcmp1),
        );
        rank = 0 as libc::c_int;
        stat0 = 0.0f64;
        tottied = 0 as libc::c_int;
        tiecorr = 0 as libc::c_int as libc::c_double;
        *levnobs.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
        while rank < nval {
            ntied = 1 as libc::c_int;
            while rank + ntied < nval
                && fabs(
                    (*val.offset((rank + ntied) as isize)).val_val
                        - (*val.offset(rank as isize)).val_val,
                )
                    < dap_tol
                        * (fabs((*val.offset((rank + ntied) as isize)).val_val)
                            + fabs((*val.offset(rank as isize)).val_val))
            {
                ntied += 1;
                ntied;
            }
            drank = (2 as libc::c_int * rank + ntied + 1 as libc::c_int)
                as libc::c_double / 2.0f64;
            if ntied > 1 as libc::c_int {
                tottied += ntied;
                tiecorr
                    += (ntied * (ntied + 1 as libc::c_int) * (ntied - 1 as libc::c_int))
                        as libc::c_double / 2.0f64;
            }
            tied = 0 as libc::c_int;
            while tied < ntied {
                if (*val.offset(rank as isize)).val_class != 0 {
                    stat0 += drank;
                    let ref mut fresh11 = *levnobs.offset(0 as libc::c_int as isize);
                    *fresh11 += 1;
                    *fresh11;
                }
                (*val.offset(rank as isize)).val_val = drank;
                tied += 1;
                tied;
                rank += 1;
                rank;
            }
        }
        fprintf(
            dap_lst,
            b"Number of tied observations = %d\n\0" as *const u8 as *const libc::c_char,
            tottied,
        );
        fprintf(
            dap_lst,
            b"Number of positive observations = %d\n\0" as *const u8
                as *const libc::c_char,
            *levnobs.offset(0 as libc::c_int as isize),
        );
        fprintf(
            dap_lst,
            b"\nWilcoxon signed rank statistic W0 = %g\n\0" as *const u8
                as *const libc::c_char,
            stat0,
        );
        fprintf(
            dap_lst,
            b"Expected under H0 = %g\n\0" as *const u8 as *const libc::c_char,
            dn * (dn + 1.0f64) / 4.0f64,
        );
        stat0 = fabs(stat0 - dn * (dn + 1.0f64) / 4.0f64);
        if nval <= dap_maxex1 {
            rank = 0 as libc::c_int;
            while rank < nval {
                (*val.offset(rank as isize)).val_class = 0 as libc::c_int;
                rank += 1;
                rank;
            }
            prob = 0.0f64;
            loop {
                rank = 0 as libc::c_int;
                stat = -(dn * (dn + 1.0f64)) / 4.0f64;
                while rank < nval {
                    if (*val.offset(rank as isize)).val_class != 0 {
                        stat += (*val.offset(rank as isize)).val_val;
                    }
                    rank += 1;
                    rank;
                }
                if fabs(stat) >= stat0 {
                    prob += 1.0f64;
                }
                rank = 0 as libc::c_int;
                while rank < nval {
                    if (*val.offset(rank as isize)).val_class == 0 {
                        (*val.offset(rank as isize)).val_class = 1 as libc::c_int;
                        break;
                    } else {
                        (*val.offset(rank as isize)).val_class = 0 as libc::c_int;
                        rank += 1;
                        rank;
                    }
                }
                if !(rank < nval) {
                    break;
                }
            }
            rank = 0 as libc::c_int;
            while rank < nval {
                prob /= 2.0f64;
                rank += 1;
                rank;
            }
            fprintf(
                dap_lst,
                b"Prob[|W - E(W)| >= |W0 - E(W)|] = %.4g (exact)\n\0" as *const u8
                    as *const libc::c_char,
                prob,
            );
        } else {
            prob = 2.0f64
                * probt(
                    stat0
                        * sqrt(
                            (dn - 1.0f64)
                                / (dn
                                    * (dn * (dn + 1.0f64) * (2.0f64 * dn + 1.0f64) - tiecorr)
                                    / 24.0f64 - stat0 * stat0),
                        ),
                    nval - 1 as libc::c_int,
                );
            fprintf(
                dap_lst,
                b"Prob[|W - E(W)| >= |W0 - E(W)|] = %.4g\n\0" as *const u8
                    as *const libc::c_char,
                prob,
            );
            fprintf(
                dap_lst,
                b"(t-approximation, df = %d, with continuity correction)\n\0"
                    as *const u8 as *const libc::c_char,
                nval - 1 as libc::c_int,
            );
        }
    }
    dap_free(
        levnobs as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        rank1 as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        sumr as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_swap();
}
unsafe extern "C" fn findlev(
    mut levname: *mut libc::c_char,
    mut level: *mut *mut libc::c_char,
    mut nlevels: *mut libc::c_int,
) -> libc::c_int {
    let mut l: libc::c_int = 0;
    l = 0 as libc::c_int;
    while l < *nlevels {
        if strcmp(levname, *level.offset(l as isize)) == 0 {
            return l;
        }
        l += 1;
        l;
    }
    if *nlevels < dap_maxlev {
        strcpy(*level.offset(*nlevels as isize), levname);
        let fresh12 = *nlevels;
        *nlevels = *nlevels + 1;
        return fresh12;
    } else {
        fprintf(
            dap_err,
            b"(findlev) Too many levels (%s)\n\0" as *const u8 as *const libc::c_char,
            levname,
        );
        exit(1 as libc::c_int);
    };
}
pub unsafe extern "C" fn nonparam(
    mut fname: *mut libc::c_char,
    mut variables: *mut libc::c_char,
    mut marks: *mut libc::c_char,
) {
    let mut varv: [libc::c_int; 2] = [0; 2];
    let mut nvar: libc::c_int = 0;
    let mut markv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nmark: libc::c_int = 0;
    let mut val: *mut value = 0 as *mut value;
    let mut nval: libc::c_int = 0;
    let mut nobs: libc::c_int = 0;
    let mut levmem: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut level: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut nlevels: libc::c_int = 0;
    let mut more: libc::c_int = 0;
    if fname.is_null() {
        fputs(
            b"(nonparam) No dataset name given.\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    markv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"dap_maxvar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    levmem = dap_malloc(
        dap_maxlev * (dap_strlen + 1 as libc::c_int),
        b"dap_maxlev, dap_strlen\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    level = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(dap_maxlev as libc::c_ulong) as libc::c_int,
        b"dap_maxlev\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_char;
    nlevels = 0 as libc::c_int;
    while nlevels < dap_maxlev {
        let ref mut fresh13 = *level.offset(nlevels as isize);
        *fresh13 = levmem.offset((nlevels * (dap_strlen + 1 as libc::c_int)) as isize);
        nlevels += 1;
        nlevels;
    }
    inset(fname);
    nvar = dap_list(variables, varv.as_mut_ptr(), dap_maxvar);
    if nvar == 0 {
        fputs(
            b"(nonparam) No variables specified.\n\0" as *const u8
                as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    if nvar > 2 as libc::c_int {
        fputs(
            b"(nonparam) At most one response and one class variable allowed\n\0"
                as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
        .offset(varv[0 as libc::c_int as usize] as isize) != -(1 as libc::c_int)
    {
        fprintf(
            dap_err,
            b"(nonparam) Variable must be dap_double: %s\n\0" as *const u8
                as *const libc::c_char,
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                .offset(varv[0 as libc::c_int as usize] as isize),
        );
        exit(1 as libc::c_int);
    }
    if nvar == 2 as libc::c_int
        && *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
            .offset(varv[1 as libc::c_int as usize] as isize) <= 0 as libc::c_int
    {
        fprintf(
            dap_err,
            b"(nonparam) Classification variable must be dap_char: %s\n\0" as *const u8
                as *const libc::c_char,
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                .offset(varv[1 as libc::c_int as usize] as isize),
        );
        exit(1 as libc::c_int);
    }
    nmark = dap_list(marks, markv, dap_maxvar);
    val = dap_malloc(
        (::std::mem::size_of::<value>() as libc::c_ulong)
            .wrapping_mul(dap_maxval as libc::c_ulong) as libc::c_int,
        b"dap_maxval\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut value;
    more = 1 as libc::c_int;
    nlevels = 0 as libc::c_int;
    nval = 0 as libc::c_int;
    nobs = 0 as libc::c_int;
    while more != 0 {
        more = step();
        if dap_newpart(markv, nmark) != 0 {
            dap_swap();
            dap_head(markv, nmark);
            dap_swap();
            fprintf(
                dap_lst,
                b"Number of observations = %d\n\0" as *const u8 as *const libc::c_char,
                nobs,
            );
            nonpar1(val, nval, level, nlevels, varv.as_mut_ptr(), nvar);
            if more == 0 {
                break;
            }
            nval = 0 as libc::c_int;
            nobs = 0 as libc::c_int;
            nlevels = 0 as libc::c_int;
        }
        if nval >= dap_maxval {
            fputs(
                b"(nonparam) Too many values.\n\0" as *const u8 as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        if nvar == 2 as libc::c_int {
            (*val.offset(nval as isize))
                .val_class = findlev(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(varv[1 as libc::c_int as usize] as isize),
                level,
                &mut nlevels,
            );
            (*val.offset(nval as isize))
                .val_val = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                .do_dbl)
                .offset(varv[0 as libc::c_int as usize] as isize);
            nval += 1;
            nval;
        } else if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
            .offset(varv[0 as libc::c_int as usize] as isize) != 0.0f64
        {
            (*val.offset(nval as isize))
                .val_class = (*((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                .do_dbl)
                .offset(varv[0 as libc::c_int as usize] as isize) > 0.0f64)
                as libc::c_int;
            (*val.offset(nval as isize))
                .val_val = fabs(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(varv[0 as libc::c_int as usize] as isize),
            );
            nval += 1;
            nval;
        }
        nobs += 1;
        nobs;
    }
    dap_free(
        val as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        markv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        levmem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        level as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
