use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn dap_free(ptr: *mut libc::c_void, mesg: *mut libc::c_char);
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn index(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn finite(_: libc::c_double) -> libc::c_int;
    fn step() -> libc::c_int;
    fn inset(fname: *mut libc::c_char);
    fn outset(fname: *mut libc::c_char, varlist: *mut libc::c_char);
    fn output();
    fn dap_vd(varspec: *mut libc::c_char, invar: libc::c_int) -> libc::c_int;
    fn dap_swap();
    fn dap_varnum(vname: *mut libc::c_char) -> libc::c_int;
    fn dap_head(markv: *mut libc::c_int, nmark: libc::c_int);
    fn dap_list(
        varlist: *mut libc::c_char,
        varv: *mut libc::c_int,
        maxvars: libc::c_int,
    ) -> libc::c_int;
    fn dap_newpart(partv: *mut libc::c_int, npartv: libc::c_int) -> libc::c_int;
    fn dap_invert(mat: *mut *mut libc::c_double, nrc: libc::c_int) -> libc::c_int;
    fn probchisq(x2: libc::c_double, df: libc::c_int) -> libc::c_double;
    fn dap_maximize(
        f: Option::<unsafe extern "C" fn(*mut libc::c_double) -> libc::c_double>,
        nx: libc::c_int,
        x: *mut libc::c_double,
        step_0: libc::c_double,
        tol: libc::c_double,
        trace: *mut libc::c_char,
    ) -> libc::c_double;
    fn dataset(
        oldname: *mut libc::c_char,
        newname: *mut libc::c_char,
        action: *mut libc::c_char,
    );
    fn dap_malloc(nbytes: libc::c_int, mesg: *mut libc::c_char) -> *mut libc::c_char;
    fn dap_clearobs(varspec: *mut libc::c_char) -> libc::c_int;
    static mut dap_maxvar: libc::c_int;
    static mut dap_maxcell: libc::c_int;
    static mut dap_addtozero: libc::c_double;
    static mut dap_cattol: libc::c_double;
    static mut dap_obs: [dataobs; 0];
    static mut dap_ono: libc::c_int;
    static mut dap_lst: *mut FILE;
    static mut dap_err: *mut FILE;
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
pub struct node {
    pub value: *mut libc::c_char,
    pub next: *mut node,
}
pub type valnode = node;
static mut allparam: *mut libc::c_double = 0 as *const libc::c_double
    as *mut libc::c_double;
static mut sel: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut selred: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut ex: Option::<unsafe extern "C" fn() -> libc::c_double> = None;
static mut tab: *mut *mut libc::c_double = 0 as *const *mut libc::c_double
    as *mut *mut libc::c_double;
static mut nc: libc::c_int = 0;
unsafe extern "C" fn loglike(mut selparam: *mut libc::c_double) -> libc::c_double {
    let mut s: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut expected: libc::c_double = 0.;
    let mut ll: libc::c_double = 0.;
    let mut c: libc::c_int = 0;
    s = 0 as libc::c_int;
    p = 0 as libc::c_int;
    while *sel.offset(s as isize) != 0 {
        if *sel.offset(s as isize) as libc::c_int != '!' as i32 {
            let fresh0 = p;
            p = p + 1;
            *allparam.offset(s as isize) = *selparam.offset(fresh0 as isize);
        } else {
            *allparam.offset(s as isize) = 0.0f64;
        }
        s += 1;
        s;
    }
    ll = 0.0f64;
    c = 0 as libc::c_int;
    while c < nc {
        expected = ::std::mem::transmute::<
            _,
            fn(_, _) -> libc::c_double,
        >(
            ex.unwrap(),
        )(allparam, (*tab.offset(c as isize)).offset(1 as libc::c_int as isize));
        ll
            += *(*tab.offset(c as isize)).offset(0 as libc::c_int as isize)
                * log(expected) - expected;
        c += 1;
        c;
    }
    return ll;
}
unsafe extern "C" fn selparse(
    mut names: *mut libc::c_char,
    mut codes: *mut libc::c_char,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    n = 0 as libc::c_int;
    while *names.offset(n as isize) as libc::c_int == ' ' as i32 {
        n += 1;
        n;
    }
    c = 0 as libc::c_int;
    while *names.offset(n as isize) != 0 {
        if *names.offset(n as isize) as libc::c_int == '!' as i32
            || *names.offset(n as isize) as libc::c_int == '?' as i32
        {
            *codes.offset(c as isize) = *names.offset(n as isize);
            n += 1;
            n;
            while *names.offset(n as isize) as libc::c_int == ' ' as i32 {
                n += 1;
                n;
            }
        } else {
            *codes.offset(c as isize) = '1' as i32 as libc::c_char;
        }
        while *names.offset(n as isize) as libc::c_int != 0
            && *names.offset(n as isize) as libc::c_int != ' ' as i32
        {
            n += 1;
            n;
        }
        while *names.offset(n as isize) as libc::c_int == ' ' as i32 {
            n += 1;
            n;
        }
        c += 1;
        c;
    }
    *codes.offset(c as isize) = '\0' as i32 as libc::c_char;
    return c;
}
unsafe extern "C" fn categ1(
    mut tab_0: *mut *mut libc::c_double,
    mut ncell: libc::c_int,
    mut varv: *mut libc::c_int,
    mut nvar: libc::c_int,
    mut expect: Option::<unsafe extern "C" fn() -> libc::c_double>,
    mut param: *mut libc::c_double,
    mut select: *mut libc::c_char,
    mut selcodes: *mut libc::c_char,
    mut param1n: libc::c_int,
    mut param2n: libc::c_int,
    mut covn: libc::c_int,
    mut partv: *mut libc::c_int,
    mut partv2: *mut libc::c_int,
    mut npart: libc::c_int,
    mut trace: *mut libc::c_char,
) {
    let mut typen: libc::c_int = 0;
    let mut sparam: libc::c_int = 0;
    let mut sparamr: libc::c_int = 0;
    let mut nparam: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut c1: libc::c_int = 0;
    let mut c0: libc::c_int = 0;
    let mut x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut xch: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p: libc::c_int = 0;
    let mut step_0: libc::c_double = 0.;
    let mut tol: libc::c_double = 0.;
    let mut tmp: libc::c_double = 0.;
    let mut v: libc::c_int = 0;
    let mut likerat: libc::c_double = 0.;
    let mut likered: libc::c_double = 0.;
    let mut pearson: libc::c_double = 0.;
    let mut infomem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut info: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut p1: libc::c_int = 0;
    let mut p2: libc::c_int = 0;
    let mut lpp: libc::c_double = 0.;
    let mut lpm: libc::c_double = 0.;
    let mut lmp: libc::c_double = 0.;
    let mut lmm: libc::c_double = 0.;
    let mut h: libc::c_double = 0.;
    let mut halfh: libc::c_double = 0.;
    let mut s: libc::c_int = 0;
    let mut s1: libc::c_int = 0;
    let mut nonsing: libc::c_int = 0;
    typen = dap_varnum(
        b"_type_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if typen < 0 as libc::c_int {
        fputs(
            b"(categ1) missing _type_ variable\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    sparamr = 0 as libc::c_int;
    likered = 0.0f64;
    nc = ncell;
    ex = expect;
    x = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(strlen(sel)) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    xch = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(strlen(sel)) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    if !selred.is_null() {
        sel = selred;
        nparam = 0 as libc::c_int;
        sparamr = 0 as libc::c_int;
        while *sel.offset(nparam as isize) != 0 {
            if *sel.offset(nparam as isize) as libc::c_int != '!' as i32 {
                *allparam.offset(nparam as isize) = *param.offset(nparam as isize);
                sparamr += 1;
                sparamr;
            } else {
                *allparam.offset(nparam as isize) = 0.0f64;
            }
            nparam += 1;
            nparam;
        }
        p = 0 as libc::c_int;
        nparam = 0 as libc::c_int;
        while *sel.offset(nparam as isize) != 0 {
            if *sel.offset(nparam as isize) as libc::c_int != '!' as i32 {
                let fresh1 = p;
                p = p + 1;
                *x.offset(fresh1 as isize) = *param.offset(nparam as isize);
            }
            nparam += 1;
            nparam;
        }
        step_0 = 0.0f64;
        p = 0 as libc::c_int;
        while p < sparamr {
            tmp = *x.offset(p as isize);
            step_0 += tmp * tmp;
            p += 1;
            p;
        }
        if step_0 > 0.0f64 {
            step_0 = 0.1f64 * sqrt(step_0);
        } else {
            step_0 = 0.01f64;
        }
        tol = dap_cattol * step_0;
        dap_maximize(
            Some(loglike as unsafe extern "C" fn(*mut libc::c_double) -> libc::c_double),
            sparamr,
            x,
            step_0,
            tol,
            trace,
        );
        c = 0 as libc::c_int;
        likerat = 0.0f64;
        while c < ncell {
            likered
                += (*(*tab_0.offset(c as isize)).offset(0 as libc::c_int as isize)
                    + dap_addtozero)
                    * log(
                        (*(*tab_0.offset(c as isize)).offset(0 as libc::c_int as isize)
                            + dap_addtozero)
                            / ::std::mem::transmute::<
                                _,
                                fn(_, _) -> libc::c_double,
                            >(
                                expect.unwrap(),
                            )(
                                allparam,
                                (*tab_0.offset(c as isize))
                                    .offset(1 as libc::c_int as isize),
                            ),
                    );
            c += 1;
            c;
        }
        likered *= 2.0f64;
    }
    sel = selcodes;
    nparam = 0 as libc::c_int;
    sparam = 0 as libc::c_int;
    while *sel.offset(nparam as isize) != 0 {
        if *sel.offset(nparam as isize) as libc::c_int != '!' as i32 {
            *allparam.offset(nparam as isize) = *param.offset(nparam as isize);
            sparam += 1;
            sparam;
        } else {
            *allparam.offset(nparam as isize) = 0.0f64;
        }
        nparam += 1;
        nparam;
    }
    p = 0 as libc::c_int;
    nparam = 0 as libc::c_int;
    while *sel.offset(nparam as isize) != 0 {
        if *sel.offset(nparam as isize) as libc::c_int != '!' as i32 {
            let fresh2 = p;
            p = p + 1;
            *x.offset(fresh2 as isize) = *param.offset(nparam as isize);
        }
        nparam += 1;
        nparam;
    }
    step_0 = 0.0f64;
    p = 0 as libc::c_int;
    while p < sparam {
        tmp = *x.offset(p as isize);
        step_0 += tmp * tmp;
        p += 1;
        p;
    }
    if step_0 > 0.0f64 {
        step_0 = 0.1f64 * sqrt(step_0);
    } else {
        step_0 = 0.01f64;
    }
    tol = dap_cattol * step_0;
    dap_maximize(
        Some(loglike as unsafe extern "C" fn(*mut libc::c_double) -> libc::c_double),
        sparam,
        x,
        step_0,
        tol,
        trace,
    );
    c = 0 as libc::c_int;
    likerat = 0.0f64;
    pearson = 0.0f64;
    while c < ncell {
        v = 0 as libc::c_int;
        while v < nvar {
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(
                    *varv.offset(v as isize) as isize,
                ) = *(*tab_0.offset(c as isize)).offset(v as isize);
            v += 1;
            v;
        }
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typen as isize),
            b"OBS\0" as *const u8 as *const libc::c_char,
        );
        output();
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typen as isize),
            b"FIT\0" as *const u8 as *const libc::c_char,
        );
        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
            .offset(
                *varv.offset(0 as libc::c_int as isize) as isize,
            ) = ::std::mem::transmute::<
            _,
            fn(_, _) -> libc::c_double,
        >(
            expect.unwrap(),
        )(allparam, (*tab_0.offset(c as isize)).offset(1 as libc::c_int as isize));
        likerat
            += (*(*tab_0.offset(c as isize)).offset(0 as libc::c_int as isize)
                + dap_addtozero)
                * log(
                    (*(*tab_0.offset(c as isize)).offset(0 as libc::c_int as isize)
                        + dap_addtozero)
                        / *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_dbl)
                            .offset(*varv.offset(0 as libc::c_int as isize) as isize),
                );
        tmp = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
            .offset(*varv.offset(0 as libc::c_int as isize) as isize)
            - *(*tab_0.offset(c as isize)).offset(0 as libc::c_int as isize);
        pearson
            += tmp * tmp
                / *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(*varv.offset(0 as libc::c_int as isize) as isize);
        output();
        c += 1;
        c;
    }
    likerat *= 2.0f64;
    infomem = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(sparam as libc::c_ulong)
            .wrapping_mul(sparam as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    info = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
            .wrapping_mul(sparam as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_double;
    p = 0 as libc::c_int;
    while p < sparam {
        let ref mut fresh3 = *info.offset(p as isize);
        *fresh3 = infomem.offset((p * sparam) as isize);
        p += 1;
        p;
    }
    h = 0.0001f64;
    halfh = h / 2.0f64;
    p1 = 0 as libc::c_int;
    while p1 < sparam {
        p = 0 as libc::c_int;
        while p < sparam {
            *xch.offset(p as isize) = *x.offset(p as isize);
            p += 1;
            p;
        }
        lpm = loglike(xch);
        *xch.offset(p1 as isize) += h;
        lpp = loglike(xch);
        *xch.offset(p1 as isize) = *x.offset(p1 as isize) - h;
        lmm = loglike(xch);
        *(*info.offset(p1 as isize))
            .offset(p1 as isize) = -(lpp - 2.0f64 * lpm + lmm) / (h * h);
        p1 += 1;
        p1;
    }
    p1 = 0 as libc::c_int;
    while p1 < sparam {
        p2 = 0 as libc::c_int;
        while p2 < p1 {
            p = 0 as libc::c_int;
            while p < sparam {
                *xch.offset(p as isize) = *x.offset(p as isize);
                p += 1;
                p;
            }
            *xch.offset(p1 as isize) += halfh;
            *xch.offset(p2 as isize) += halfh;
            lpp = loglike(xch);
            *xch.offset(p1 as isize) = *x.offset(p1 as isize) - halfh;
            lmp = loglike(xch);
            *xch.offset(p2 as isize) = *x.offset(p2 as isize) - halfh;
            lmm = loglike(xch);
            *xch.offset(p1 as isize) = *x.offset(p1 as isize) + halfh;
            lpm = loglike(xch);
            *(*info.offset(p1 as isize))
                .offset(p2 as isize) = -(lpp - lpm - lmp + lmm) / (h * h);
            *(*info.offset(p2 as isize))
                .offset(p1 as isize) = *(*info.offset(p1 as isize)).offset(p2 as isize);
            p2 += 1;
            p2;
        }
        p1 += 1;
        p1;
    }
    nonsing = dap_invert(info, sparam);
    if nonsing == 0 {
        fputs(
            b"(categ1) covariance matrix is singular\n\0" as *const u8
                as *const libc::c_char,
            dap_err,
        );
    }
    dap_ono = 1 as libc::c_int;
    typen = dap_varnum(
        b"_type_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if typen < 0 as libc::c_int {
        fprintf(
            dap_err,
            b"(categ1) output dataset has no _type_ variable\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    v = 0 as libc::c_int;
    while v < npart {
        if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
            .offset(*partv.offset(v as isize) as isize) == -(1 as libc::c_int)
        {
            *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_dbl)
                .offset(
                    *partv2.offset(v as isize) as isize,
                ) = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(*partv.offset(v as isize) as isize);
        } else if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
            .offset(*partv.offset(v as isize) as isize) == 0 as libc::c_int
        {
            *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_int)
                .offset(
                    *partv2.offset(v as isize) as isize,
                ) = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_int)
                .offset(*partv.offset(v as isize) as isize);
        } else {
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                    .offset(*partv2.offset(v as isize) as isize),
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(*partv.offset(v as isize) as isize),
            );
        }
        v += 1;
        v;
    }
    fputs(
        b"Maximum likelihood estimation\n\n\0" as *const u8 as *const libc::c_char,
        dap_lst,
    );
    fprintf(
        dap_lst,
        b"Cell count: %s\n\0" as *const u8 as *const libc::c_char,
        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
            .offset(*varv.offset(0 as libc::c_int as isize) as isize),
    );
    fputs(b"Class and aux variables:\0" as *const u8 as *const libc::c_char, dap_lst);
    v = 1 as libc::c_int;
    while v < nvar {
        fprintf(
            dap_lst,
            b" %s\0" as *const u8 as *const libc::c_char,
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                .offset(*varv.offset(v as isize) as isize),
        );
        v += 1;
        v;
    }
    putc('\n' as i32, dap_lst);
    fputs(
        b"\nStatistic              df      Prob\n\0" as *const u8 as *const libc::c_char,
        dap_lst,
    );
    fprintf(
        dap_lst,
        b"G2[Model]   = %6.2f  %3d    %.4f\n\0" as *const u8 as *const libc::c_char,
        likerat,
        ncell - sparam,
        if ncell > sparam {
            ceil(10000.0f64 * probchisq(likerat, ncell - sparam)) / 10000.0f64
        } else {
            1.0f64
        },
    );
    if !selred.is_null() {
        fprintf(
            dap_lst,
            b"G2[Reduced] = %6.2f  %3d    %.4f\n\0" as *const u8 as *const libc::c_char,
            likered,
            ncell - sparamr,
            ceil(10000.0f64 * probchisq(likered, ncell - sparamr)) / 10000.0f64,
        );
        fprintf(
            dap_lst,
            b"G2[Diff]    = %6.2f  %3d    %.4f\n\0" as *const u8 as *const libc::c_char,
            likered - likerat,
            sparam - sparamr,
            ceil(10000.0f64 * probchisq(likered - likerat, sparam - sparamr))
                / 10000.0f64,
        );
    }
    fprintf(
        dap_lst,
        b"X2[Model]   = %6.2f  %3d    %.4f\n\0" as *const u8 as *const libc::c_char,
        pearson,
        ncell - sparam,
        if ncell > sparam {
            ceil(10000.0f64 * probchisq(pearson, ncell - sparam)) / 10000.0f64
        } else {
            1.0f64
        },
    );
    putc('\n' as i32, dap_lst);
    fputs(
        b"    Estimate          ASE  Model  Parameter\n\0" as *const u8
            as *const libc::c_char,
        dap_lst,
    );
    c = 0 as libc::c_int;
    while *select.offset(c as isize) as libc::c_int == ' ' as i32 {
        c += 1;
        c;
    }
    p = 0 as libc::c_int;
    s = 0 as libc::c_int;
    while p < nparam {
        fprintf(
            dap_lst,
            b"%12g \0" as *const u8 as *const libc::c_char,
            *allparam.offset(p as isize),
        );
        if *sel.offset(p as isize) as libc::c_int == '!' as i32 {
            fputs(b"              \0" as *const u8 as *const libc::c_char, dap_lst);
        } else if nonsing != 0 {
            fprintf(
                dap_lst,
                b"%12g  \0" as *const u8 as *const libc::c_char,
                sqrt(*(*info.offset(s as isize)).offset(s as isize)),
            );
        } else {
            fputs(b"           ?  \0" as *const u8 as *const libc::c_char, dap_lst);
        }
        match *selcodes.offset(p as isize) as libc::c_int {
            49 => {
                fprintf(dap_lst, b"  *    \0" as *const u8 as *const libc::c_char);
            }
            63 => {
                fprintf(dap_lst, b"  ?    \0" as *const u8 as *const libc::c_char);
                c += 1;
                c;
                while *select.offset(c as isize) as libc::c_int == ' ' as i32 {
                    c += 1;
                    c;
                }
            }
            _ => {
                fprintf(dap_lst, b"       \0" as *const u8 as *const libc::c_char);
                c += 1;
                c;
                while *select.offset(c as isize) as libc::c_int == ' ' as i32 {
                    c += 1;
                    c;
                }
            }
        }
        p2 = 0 as libc::c_int;
        while *select.offset(c as isize) as libc::c_int != 0
            && *select.offset(c as isize) as libc::c_int != ' ' as i32
        {
            putc(*select.offset(c as isize) as libc::c_int, dap_lst);
            *(*((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                .offset(param2n as isize))
                .offset(p2 as isize) = *select.offset(c as isize);
            c += 1;
            c;
            p2 += 1;
            p2;
        }
        *(*((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
            .offset(param2n as isize))
            .offset(p2 as isize) = '\0' as i32 as libc::c_char;
        while *select.offset(c as isize) as libc::c_int == ' ' as i32 {
            c += 1;
            c;
        }
        if *sel.offset(p as isize) as libc::c_int != '!' as i32 {
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                    .offset(typen as isize),
                b"ESTIMATE\0" as *const u8 as *const libc::c_char,
            );
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                    .offset(param1n as isize),
                b"\0" as *const u8 as *const libc::c_char,
            );
            *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_dbl)
                .offset(covn as isize) = *allparam.offset(p as isize);
            output();
            s += 1;
            s;
        }
        putc('\n' as i32, dap_lst);
        p += 1;
        p;
    }
    putc('\n' as i32, dap_lst);
    strcpy(
        *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
            .offset(typen as isize),
        b"COVAR\0" as *const u8 as *const libc::c_char,
    );
    c = 0 as libc::c_int;
    while *select.offset(c as isize) as libc::c_int == ' ' as i32 {
        c += 1;
        c;
    }
    p = 0 as libc::c_int;
    s = 0 as libc::c_int;
    while p < nparam {
        if *selcodes.offset(p as isize) as libc::c_int != '1' as i32 {
            c += 1;
            c;
            while *select.offset(c as isize) as libc::c_int == ' ' as i32 {
                c += 1;
                c;
            }
        }
        while *select.offset(c as isize) as libc::c_int == ' ' as i32 {
            c += 1;
            c;
        }
        c1 = 0 as libc::c_int;
        while *select.offset(c1 as isize) as libc::c_int == ' ' as i32 {
            c1 += 1;
            c1;
        }
        p1 = 0 as libc::c_int;
        s1 = 0 as libc::c_int;
        while p1 < nparam {
            if *sel.offset(p as isize) as libc::c_int != '!' as i32
                && *sel.offset(p1 as isize) as libc::c_int != '!' as i32
            {
                *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_dbl)
                    .offset(
                        covn as isize,
                    ) = *(*info.offset(s as isize)).offset(s1 as isize);
            }
            if *sel.offset(p1 as isize) as libc::c_int != '!' as i32 {
                s1 += 1;
                s1;
            }
            if *selcodes.offset(p1 as isize) as libc::c_int != '1' as i32 {
                c1 += 1;
                c1;
                while *select.offset(c1 as isize) as libc::c_int == ' ' as i32 {
                    c1 += 1;
                    c1;
                }
            }
            if *sel.offset(p as isize) as libc::c_int != '!' as i32
                && *sel.offset(p1 as isize) as libc::c_int != '!' as i32
            {
                p2 = 0 as libc::c_int;
                while *select.offset(c1 as isize) as libc::c_int != 0
                    && *select.offset(c1 as isize) as libc::c_int != ' ' as i32
                {
                    *(*((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                        .offset(param2n as isize))
                        .offset(p2 as isize) = *select.offset(c1 as isize);
                    c1 += 1;
                    c1;
                    p2 += 1;
                    p2;
                }
                *(*((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                    .offset(param2n as isize))
                    .offset(p2 as isize) = '\0' as i32 as libc::c_char;
                c0 = c;
                if *select.offset(c0 as isize) as libc::c_int == '?' as i32 {
                    c0 += 1;
                    c0;
                    while *select.offset(c0 as isize) as libc::c_int == ' ' as i32 {
                        c0 += 1;
                        c0;
                    }
                }
                p2 = 0 as libc::c_int;
                while *select.offset(c0 as isize) as libc::c_int != 0
                    && *select.offset(c0 as isize) as libc::c_int != ' ' as i32
                {
                    *(*((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                        .offset(param1n as isize))
                        .offset(p2 as isize) = *select.offset(c0 as isize);
                    c0 += 1;
                    c0;
                    p2 += 1;
                    p2;
                }
                *(*((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                    .offset(param1n as isize))
                    .offset(p2 as isize) = '\0' as i32 as libc::c_char;
                output();
            }
            while *select.offset(c1 as isize) as libc::c_int == ' ' as i32 {
                c1 += 1;
                c1;
            }
            p1 += 1;
            p1;
        }
        while *select.offset(c as isize) as libc::c_int != 0
            && *select.offset(c as isize) as libc::c_int != ' ' as i32
        {
            c += 1;
            c;
        }
        if *sel.offset(p as isize) as libc::c_int != '!' as i32 {
            s += 1;
            s;
        }
        p += 1;
        p;
    }
    dap_ono = 0 as libc::c_int;
    dap_free(
        x as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        xch as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        infomem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        info as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn categ(
    mut dname: *mut libc::c_char,
    mut varlist: *mut libc::c_char,
    mut auxvarlist: *mut libc::c_char,
    mut expect: Option::<unsafe extern "C" fn() -> libc::c_double>,
    mut param: *mut libc::c_double,
    mut select: *mut libc::c_char,
    mut part: *mut libc::c_char,
    mut trace: *mut libc::c_char,
) {
    let mut p: libc::c_int = 0;
    let mut filset: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filarg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut catset: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut covset: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut param1n: libc::c_int = 0;
    let mut param2n: libc::c_int = 0;
    let mut covn: libc::c_int = 0;
    let mut paramlen1: libc::c_int = 0;
    let mut paramlen: libc::c_int = 0;
    let mut paramstr: [libc::c_char; 12] = [0; 12];
    let mut partstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut varv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut partv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut partv2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ncvar: libc::c_int = 0;
    let mut navar: libc::c_int = 0;
    let mut nvar: libc::c_int = 0;
    let mut npart: libc::c_int = 0;
    let mut more: libc::c_int = 0;
    let mut nparam: libc::c_int = 0;
    let mut selcodes: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tabmem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut v: libc::c_int = 0;
    let mut ncell: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    varv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"dap_maxvar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    partv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"dap_maxvar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    partv2 = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"dap_maxvar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    filarg = dap_malloc(
        (strlen(varlist))
            .wrapping_add(strlen(part))
            .wrapping_add(8 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(filarg, b"FILL \0" as *const u8 as *const libc::c_char);
    s = 0 as libc::c_int;
    while *varlist.offset(s as isize) as libc::c_int == ' ' as i32 {
        s += 1;
        s;
    }
    v = 5 as libc::c_int;
    while *varlist.offset(s as isize) as libc::c_int != 0
        && *varlist.offset(s as isize) as libc::c_int != ' ' as i32
    {
        let fresh4 = s;
        s = s + 1;
        let fresh5 = v;
        v = v + 1;
        *filarg.offset(fresh5 as isize) = *varlist.offset(fresh4 as isize);
    }
    let fresh6 = v;
    v = v + 1;
    *filarg.offset(fresh6 as isize) = ':' as i32 as libc::c_char;
    strcpy(filarg.offset(v as isize), part);
    strcat(filarg, b" \0" as *const u8 as *const libc::c_char);
    strcat(filarg, varlist.offset(s as isize));
    filset = dap_malloc(
        (strlen(dname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(filset, dname);
    strcat(filset, b".fil\0" as *const u8 as *const libc::c_char);
    dataset(dname, filset, filarg);
    catset = dap_malloc(
        (strlen(dname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(catset, dname);
    strcat(catset, b".cat\0" as *const u8 as *const libc::c_char);
    covset = dap_malloc(
        (strlen(dname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(covset, dname);
    strcat(covset, b".cov\0" as *const u8 as *const libc::c_char);
    s = 0 as libc::c_int;
    paramlen = 0 as libc::c_int;
    while *select.offset(s as isize) != 0 {
        while *select.offset(s as isize) as libc::c_int == ' ' as i32
            || *select.offset(s as isize) as libc::c_int == '?' as i32
            || *select.offset(s as isize) as libc::c_int == '!' as i32
        {
            s += 1;
            s;
        }
        paramlen1 = 0 as libc::c_int;
        while *select.offset(s as isize) as libc::c_int != 0
            && *select.offset(s as isize) as libc::c_int != ' ' as i32
        {
            paramlen1 += 1;
            paramlen1;
            s += 1;
            s;
        }
        if paramlen1 > paramlen {
            paramlen = paramlen1;
        }
    }
    dap_ono = 0 as libc::c_int;
    inset(filset);
    ncvar = dap_list(varlist, varv, dap_maxvar);
    navar = dap_list(auxvarlist, varv.offset(ncvar as isize), dap_maxvar - ncvar);
    nvar = ncvar + navar;
    v = 1 as libc::c_int;
    while v < nvar {
        if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
            .offset(*varv.offset(v as isize) as isize) != -(1 as libc::c_int)
        {
            fprintf(
                dap_err,
                b"(categ) classification or auxiliary variable not of type double: %s\n\0"
                    as *const u8 as *const libc::c_char,
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(*varv.offset(v as isize) as isize),
            );
            exit(1 as libc::c_int);
        }
        v += 1;
        v;
    }
    npart = dap_list(part, partv, dap_maxvar);
    dap_ono = 1 as libc::c_int;
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
        (strlen(part)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    v = 0 as libc::c_int;
    while v < npart {
        strcpy(
            partstr,
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                .offset(*partv.offset(v as isize) as isize),
        );
        sprintf(
            partstr.offset(strlen(partstr) as isize),
            b" %d\0" as *const u8 as *const libc::c_char,
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
                .offset(*partv.offset(v as isize) as isize),
        );
        *partv2.offset(v as isize) = dap_vd(partstr, 1 as libc::c_int);
        v += 1;
        v;
    }
    outset(covset, b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    dap_ono = 0 as libc::c_int;
    outset(catset, b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    tabmem = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nvar as libc::c_ulong)
            .wrapping_mul(dap_maxcell as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    tab = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxcell as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_double;
    ncell = 0 as libc::c_int;
    while ncell < dap_maxcell {
        let ref mut fresh7 = *tab.offset(ncell as isize);
        *fresh7 = tabmem.offset((ncell * nvar) as isize);
        ncell += 1;
        ncell;
    }
    selcodes = dap_malloc(
        (strlen(select)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    nparam = selparse(select, selcodes);
    allparam = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nparam as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    sel = selcodes;
    if !(index(selcodes, '?' as i32)).is_null() {
        selred = dap_malloc(
            nparam + 1 as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        s = 0 as libc::c_int;
        while *selcodes.offset(s as isize) != 0 {
            if *selcodes.offset(s as isize) as libc::c_int == '?' as i32 {
                *selred.offset(s as isize) = '!' as i32 as libc::c_char;
            } else {
                *selred.offset(s as isize) = *selcodes.offset(s as isize);
            }
            s += 1;
            s;
        }
        *selred.offset(s as isize) = '\0' as i32 as libc::c_char;
    } else {
        selred = 0 as *mut libc::c_char;
    }
    p = 0 as libc::c_int;
    while p < nparam {
        *allparam.offset(p as isize) = *param.offset(p as isize);
        p += 1;
        p;
    }
    ncell = 0 as libc::c_int;
    more = 1 as libc::c_int;
    while more != 0 {
        more = step();
        if dap_newpart(partv, npart) != 0 {
            dap_swap();
            dap_head(partv, npart);
            categ1(
                tab,
                ncell,
                varv,
                nvar,
                expect,
                param,
                select,
                selcodes,
                param1n,
                param2n,
                covn,
                partv,
                partv2,
                npart,
                trace,
            );
            dap_swap();
            ncell = 0 as libc::c_int;
            p = 0 as libc::c_int;
            while p < nparam {
                *allparam.offset(p as isize) = *param.offset(p as isize);
                p += 1;
                p;
            }
        }
        if more != 0 {
            if ncell < dap_maxcell {
                v = 0 as libc::c_int;
                while v < nvar {
                    *(*tab.offset(ncell as isize))
                        .offset(
                            v as isize,
                        ) = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                        .do_dbl)
                        .offset(*varv.offset(v as isize) as isize);
                    v += 1;
                    v;
                }
                ncell += 1;
                ncell;
            } else {
                fputs(
                    b"(categ) too many cells\n\0" as *const u8 as *const libc::c_char,
                    dap_err,
                );
                exit(1 as libc::c_int);
            }
        }
    }
    if !selred.is_null() {
        dap_free(
            selred as *mut libc::c_void,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        selred = 0 as *mut libc::c_char;
    }
    dap_free(
        filarg as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        varv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        partv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        partv2 as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        filset as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        catset as *mut libc::c_void,
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
        tabmem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        tab as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        allparam as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        selcodes as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
static mut paramlist: *mut *mut libc::c_int = 0 as *const *mut libc::c_int
    as *mut *mut libc::c_int;
static mut maxval: *mut libc::c_double = 0 as *const libc::c_double
    as *mut libc::c_double;
static mut numparam: libc::c_int = 0;
static mut nclass: libc::c_int = 0;
unsafe extern "C" fn llexpect(
    mut param: *mut libc::c_double,
    mut class: *mut libc::c_double,
) -> libc::c_double {
    let mut p: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut neg: [libc::c_int; 32] = [0; 32];
    let mut sign: libc::c_double = 0.;
    let mut match_0: libc::c_int = 0;
    let mut logc: libc::c_double = 0.;
    c = 0 as libc::c_int;
    while c < nclass {
        neg[c
            as usize] = (*class.offset(c as isize) == *maxval.offset(c as isize))
            as libc::c_int;
        c += 1;
        c;
    }
    logc = *param.offset(0 as libc::c_int as isize);
    p = 1 as libc::c_int;
    while p < numparam {
        sign = 1.0f64;
        match_0 = 1 as libc::c_int;
        c = 0 as libc::c_int;
        while c < nclass {
            if *(*paramlist.offset(p as isize)).offset(c as isize) >= 0 as libc::c_int {
                if neg[c as usize] != 0 {
                    sign *= -1.0f64;
                } else {
                    match_0
                        &= (*(*paramlist.offset(p as isize)).offset(c as isize)
                            == *class.offset(c as isize) as libc::c_int) as libc::c_int;
                }
            }
            c += 1;
            c;
        }
        if match_0 != 0 {
            logc += sign * *param.offset(p as isize);
        }
        p += 1;
        p;
    }
    return exp(logc);
}
unsafe extern "C" fn findclass(
    mut cname: *mut libc::c_char,
    mut class: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    c = 0 as libc::c_int;
    while c < nclass {
        if strcmp(cname, *class.offset(c as isize)) == 0 {
            return c;
        }
        c += 1;
        c;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn llparse(
    mut class: *mut *mut libc::c_char,
    mut nterm: libc::c_int,
    mut pattern: *mut libc::c_uint,
    mut model0: *mut libc::c_char,
    mut model1: *mut libc::c_char,
    mut term: *mut libc::c_int,
) -> libc::c_int {
    let mut classlen1: libc::c_int = 0;
    let mut classlen: libc::c_int = 0;
    let mut nt: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut cm: libc::c_int = 0;
    let mut oneclass: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut oneterm: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut t1: libc::c_int = 0;
    let mut firstclass: libc::c_int = 0;
    c = 0 as libc::c_int;
    classlen = 0 as libc::c_int;
    while c < nclass {
        classlen1 = strlen(*class.offset(c as isize)) as libc::c_int;
        if classlen1 > classlen {
            classlen = classlen1;
        }
        c += 1;
        c;
    }
    oneclass = dap_malloc(
        classlen + 1 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    t = 0 as libc::c_int;
    while t < nterm {
        *term.offset(t as isize) = 0 as libc::c_int;
        t += 1;
        t;
    }
    nterm = 0 as libc::c_int;
    m = 0 as libc::c_int;
    while *model1.offset(m as isize) as libc::c_int == ' ' as i32 {
        m += 1;
        m;
    }
    oneterm = 0 as libc::c_int;
    while *model1.offset(m as isize) != 0 {
        c = 0 as libc::c_int;
        while *model1.offset(m as isize) as libc::c_int != 0
            && *model1.offset(m as isize) as libc::c_int != '*' as i32
            && *model1.offset(m as isize) as libc::c_int != ' ' as i32
        {
            let fresh8 = c;
            c = c + 1;
            *oneclass.offset(fresh8 as isize) = *model1.offset(m as isize);
            m += 1;
            m;
        }
        *oneclass.offset(c as isize) = '\0' as i32 as libc::c_char;
        while *model1.offset(m as isize) as libc::c_int == ' ' as i32 {
            m += 1;
            m;
        }
        c = findclass(oneclass, class);
        if c < 0 as libc::c_int {
            fprintf(
                dap_err,
                b"(llparse) Unknown class variable %s in model %s\n\0" as *const u8
                    as *const libc::c_char,
                oneclass,
                model1,
            );
            exit(1 as libc::c_int);
        }
        oneterm = (oneterm as libc::c_uint | *pattern.offset(c as isize)) as libc::c_int;
        if *model1.offset(m as isize) as libc::c_int != '*' as i32 {
            *term.offset(oneterm as isize) = 1 as libc::c_int;
            if oneterm > nterm {
                nterm = oneterm;
            }
            oneterm = 0 as libc::c_int;
        } else {
            m += 1;
            m;
        }
        while *model1.offset(m as isize) as libc::c_int == ' ' as i32 {
            m += 1;
            m;
        }
    }
    t1 = 0 as libc::c_int;
    while t1 <= nterm {
        if *term.offset(t1 as isize) != 0 {
            t = 0 as libc::c_int;
            while t < nterm {
                if t & !t1 == 0 {
                    *term.offset(t as isize) = 1 as libc::c_int;
                }
                t += 1;
                t;
            }
        }
        t1 += 1;
        t1;
    }
    m = 0 as libc::c_int;
    while *model0.offset(m as isize) as libc::c_int == ' ' as i32 {
        m += 1;
        m;
    }
    oneterm = 0 as libc::c_int;
    while *model0.offset(m as isize) != 0 {
        c = 0 as libc::c_int;
        while *model0.offset(m as isize) as libc::c_int != 0
            && *model0.offset(m as isize) as libc::c_int != '*' as i32
            && *model0.offset(m as isize) as libc::c_int != ' ' as i32
        {
            let fresh9 = c;
            c = c + 1;
            *oneclass.offset(fresh9 as isize) = *model0.offset(m as isize);
            m += 1;
            m;
        }
        *oneclass.offset(c as isize) = '\0' as i32 as libc::c_char;
        while *model0.offset(m as isize) as libc::c_int == ' ' as i32 {
            m += 1;
            m;
        }
        c = findclass(oneclass, class);
        if c < 0 as libc::c_int {
            fprintf(
                dap_err,
                b"(llparse) Unknown class variable %s in model %s\n\0" as *const u8
                    as *const libc::c_char,
                oneclass,
                model0,
            );
            exit(1 as libc::c_int);
        }
        oneterm = (oneterm as libc::c_uint | *pattern.offset(c as isize)) as libc::c_int;
        if *model0.offset(m as isize) as libc::c_int != '*' as i32 {
            if *term.offset(oneterm as isize) == 0 {
                fputs(
                    b"(llparse) Term in model0 (\0" as *const u8 as *const libc::c_char,
                    dap_err,
                );
                c = 0 as libc::c_int;
                firstclass = 1 as libc::c_int;
                while c < nclass {
                    if *pattern.offset(c as isize) & !oneterm as libc::c_uint == 0 {
                        if firstclass != 0 {
                            firstclass = 0 as libc::c_int;
                        } else {
                            fputs(b"*\0" as *const u8 as *const libc::c_char, dap_err);
                        }
                        fprintf(
                            dap_err,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            *class.offset(c as isize),
                        );
                    }
                    c += 1;
                    c;
                }
                fprintf(
                    dap_err,
                    b") not in in model1 (%s)\n\0" as *const u8 as *const libc::c_char,
                    model1,
                );
                exit(1 as libc::c_int);
            }
            *term.offset(oneterm as isize) = 2 as libc::c_int;
            oneterm = 0 as libc::c_int;
        } else {
            m += 1;
            m;
        }
        while *model0.offset(m as isize) as libc::c_int == ' ' as i32 {
            m += 1;
            m;
        }
    }
    t1 = 0 as libc::c_int;
    while t1 <= nterm {
        if *term.offset(t1 as isize) == 2 as libc::c_int {
            t = 0 as libc::c_int;
            while t < nterm {
                if t & !t1 == 0 {
                    *term.offset(t as isize) = 2 as libc::c_int;
                }
                t += 1;
                t;
            }
        }
        t1 += 1;
        t1;
    }
    dap_free(
        oneclass as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return nterm;
}
pub unsafe extern "C" fn loglin(
    mut fname: *mut libc::c_char,
    mut varlist: *mut libc::c_char,
    mut model0: *mut libc::c_char,
    mut model1: *mut libc::c_char,
    mut part: *mut libc::c_char,
) {
    let mut fnamefil: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filarg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fname1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut catname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut varlist1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outlist: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vardef: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nvar: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut l1: libc::c_int = 0;
    let mut vd: libc::c_int = 0;
    let mut classv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut npart: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut cc: libc::c_int = 0;
    let mut nv: libc::c_int = 0;
    let mut coff: libc::c_int = 0;
    let mut maxval1: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut maxmaxval: libc::c_int = 0;
    let mut nvl: libc::c_int = 0;
    let mut firstpart: libc::c_int = 0;
    let mut more: libc::c_int = 0;
    let mut oneval: libc::c_double = 0.;
    let mut term: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nterm: libc::c_int = 0;
    let mut param: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut termparam: libc::c_int = 0;
    let mut nparam0: libc::c_int = 0;
    let mut classmem: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut class: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut cm: libc::c_int = 0;
    let mut select: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sellen: libc::c_int = 0;
    let mut termlen: libc::c_int = 0;
    let mut selterm: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: libc::c_int = 0;
    let mut sub: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pattern: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut t: libc::c_int = 0;
    let mut paramlistmem: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut p: libc::c_int = 0;
    let mut classval: *mut *mut valnode = 0 as *mut *mut valnode;
    let mut endcv: *mut *mut valnode = 0 as *mut *mut valnode;
    let mut maxlen: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut onelen: libc::c_int = 0;
    let mut formstr: [libc::c_char; 8] = [0; 8];
    let mut nodeptr: *mut valnode = 0 as *mut valnode;
    fnamefil = dap_malloc(
        (strlen(fname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(fnamefil, fname);
    strcat(fnamefil, b".fil\0" as *const u8 as *const libc::c_char);
    filarg = dap_malloc(
        (strlen(varlist))
            .wrapping_add(strlen(part))
            .wrapping_add(8 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(filarg, b"FILL \0" as *const u8 as *const libc::c_char);
    s = 0 as libc::c_int;
    while *varlist.offset(s as isize) as libc::c_int == ' ' as i32 {
        s += 1;
        s;
    }
    l = 5 as libc::c_int;
    while *varlist.offset(s as isize) as libc::c_int != 0
        && *varlist.offset(s as isize) as libc::c_int != ' ' as i32
    {
        let fresh10 = s;
        s = s + 1;
        let fresh11 = l;
        l = l + 1;
        *filarg.offset(fresh11 as isize) = *varlist.offset(fresh10 as isize);
    }
    let fresh12 = l;
    l = l + 1;
    *filarg.offset(fresh12 as isize) = ':' as i32 as libc::c_char;
    strcpy(filarg.offset(l as isize), part);
    strcat(filarg, b" \0" as *const u8 as *const libc::c_char);
    strcat(filarg, varlist.offset(s as isize));
    dataset(fname, fnamefil, filarg);
    fname1 = dap_malloc(
        (strlen(fname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(fname1, fname);
    strcat(fname1, b".llm\0" as *const u8 as *const libc::c_char);
    catname = dap_malloc(
        (strlen(fname1)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(catname, fname1);
    strcat(catname, b".cat\0" as *const u8 as *const libc::c_char);
    l = 0 as libc::c_int;
    nvar = 0 as libc::c_int;
    while *varlist.offset(l as isize) != 0 {
        while *varlist.offset(l as isize) as libc::c_int == ' ' as i32 {
            l += 1;
            l;
        }
        if *varlist.offset(l as isize) != 0 {
            nvar += 1;
            nvar;
            while *varlist.offset(l as isize) as libc::c_int != 0
                && *varlist.offset(l as isize) as libc::c_int != ' ' as i32
            {
                l += 1;
                l;
            }
        }
    }
    nclass = nvar - 1 as libc::c_int;
    varlist1 = dap_malloc(
        (strlen(varlist))
            .wrapping_add(nvar as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    outlist = dap_malloc(
        (strlen(varlist))
            .wrapping_add(nvar as libc::c_ulong)
            .wrapping_add(strlen(part))
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    vardef = dap_malloc(
        (strlen(varlist)).wrapping_add(4 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    classv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"dap_maxvar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    classval = dap_malloc(
        (::std::mem::size_of::<*mut valnode>() as libc::c_ulong)
            .wrapping_mul(nclass as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut valnode;
    endcv = dap_malloc(
        (::std::mem::size_of::<*mut valnode>() as libc::c_ulong)
            .wrapping_mul(nclass as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut valnode;
    maxlen = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(nclass as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    inset(fnamefil);
    npart = dap_list(part, classv, dap_maxvar);
    l = 0 as libc::c_int;
    while *varlist.offset(l as isize) as libc::c_int == ' ' as i32 {
        l += 1;
        l;
    }
    l1 = 0 as libc::c_int;
    while *varlist.offset(l as isize) as libc::c_int != 0
        && *varlist.offset(l as isize) as libc::c_int != ' ' as i32
    {
        *varlist1.offset(l1 as isize) = *varlist.offset(l as isize);
        let fresh13 = l;
        l = l + 1;
        let fresh14 = l1;
        l1 = l1 + 1;
        *outlist.offset(fresh14 as isize) = *varlist.offset(fresh13 as isize);
    }
    coff = l;
    dap_list(
        varlist.offset(coff as isize),
        classv.offset(npart as isize),
        dap_maxvar - npart,
    );
    if nclass > 32 as libc::c_int {
        fprintf(
            dap_err,
            b"(loglin) Number of classification variables (%d) exceeds %d.\n\0"
                as *const u8 as *const libc::c_char,
            nclass,
            32 as libc::c_int,
        );
        exit(1 as libc::c_int);
    }
    *varlist1.offset(l1 as isize) = '\0' as i32 as libc::c_char;
    c = dap_varnum(varlist1);
    if c < 0 as libc::c_int {
        fprintf(
            dap_err,
            b"(loglin) Count variable %s unknown.\n\0" as *const u8
                as *const libc::c_char,
            varlist1,
        );
        exit(1 as libc::c_int);
    }
    if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
        .offset(c as isize) >= 0 as libc::c_int
    {
        fprintf(
            dap_err,
            b"(loglin) Count variable %s not of type double.\n\0" as *const u8
                as *const libc::c_char,
            varlist1,
        );
        exit(1 as libc::c_int);
    }
    c = 0 as libc::c_int;
    while c < nclass {
        if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
            .offset(*classv.offset((npart + c) as isize) as isize) <= 0 as libc::c_int
        {
            fprintf(
                dap_err,
                b"(loglin) Classification variable %s not a string.\n\0" as *const u8
                    as *const libc::c_char,
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(*classv.offset((npart + c) as isize) as isize),
            );
            exit(1 as libc::c_int);
        }
        c += 1;
        c;
    }
    classmem = dap_malloc(
        (strlen(varlist.offset(coff as isize)))
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    class = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(nclass as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_char;
    c = 0 as libc::c_int;
    cm = 0 as libc::c_int;
    while *varlist.offset(l as isize) != 0 {
        while *varlist.offset(l as isize) as libc::c_int == ' ' as i32 {
            l += 1;
            l;
        }
        if *varlist.offset(l as isize) != 0 {
            *varlist1.offset(l1 as isize) = ' ' as i32 as libc::c_char;
            let fresh15 = l1;
            l1 = l1 + 1;
            *outlist.offset(fresh15 as isize) = ' ' as i32 as libc::c_char;
            *varlist1.offset(l1 as isize) = '_' as i32 as libc::c_char;
            let fresh16 = l1;
            l1 = l1 + 1;
            *outlist.offset(fresh16 as isize) = '_' as i32 as libc::c_char;
            *vardef.offset(0 as libc::c_int as isize) = '_' as i32 as libc::c_char;
            let ref mut fresh17 = *class.offset(c as isize);
            *fresh17 = classmem.offset(cm as isize);
            vd = 1 as libc::c_int;
            while *varlist.offset(l as isize) as libc::c_int != 0
                && *varlist.offset(l as isize) as libc::c_int != ' ' as i32
            {
                let fresh18 = cm;
                cm = cm + 1;
                *classmem.offset(fresh18 as isize) = *varlist.offset(l as isize);
                let fresh19 = vd;
                vd = vd + 1;
                *vardef.offset(fresh19 as isize) = *varlist.offset(l as isize);
                *varlist1.offset(l1 as isize) = *varlist.offset(l as isize);
                let fresh20 = l1;
                l1 = l1 + 1;
                *outlist.offset(fresh20 as isize) = *varlist.offset(l as isize);
                l += 1;
                l;
            }
            *vardef.offset(vd as isize) = '\0' as i32 as libc::c_char;
            let fresh21 = cm;
            cm = cm + 1;
            *classmem.offset(fresh21 as isize) = '\0' as i32 as libc::c_char;
            strcat(vardef, b" -1\0" as *const u8 as *const libc::c_char);
            let fresh22 = c;
            c = c + 1;
            *classv
                .offset(
                    (npart + nclass + fresh22) as isize,
                ) = dap_vd(vardef, 0 as libc::c_int);
        }
    }
    *varlist1.offset(l1 as isize) = '\0' as i32 as libc::c_char;
    *outlist.offset(l1 as isize) = '\0' as i32 as libc::c_char;
    strcat(outlist, b" \0" as *const u8 as *const libc::c_char);
    strcat(outlist, part);
    outset(fname1, outlist);
    maxval = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nclass as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    maxval1 = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nclass as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    nvl = 0 as libc::c_int;
    while nvl < nclass {
        *maxval1.offset(nvl as isize) = 0.0f64;
        nvl += 1;
        nvl;
    }
    c = 0 as libc::c_int;
    while c < nclass {
        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
            .offset(*classv.offset((npart + nclass + c) as isize) as isize) = 0.0f64;
        *maxlen.offset(c as isize) = 0 as libc::c_int;
        let ref mut fresh23 = *classval.offset(c as isize);
        *fresh23 = 0 as *mut valnode;
        c += 1;
        c;
    }
    firstpart = 1 as libc::c_int;
    more = 1 as libc::c_int;
    while more != 0 {
        more = step();
        if dap_newpart(classv, npart) != 0 {
            if firstpart != 0 {
                nvl = 0 as libc::c_int;
                while nvl < nclass {
                    *maxval.offset(nvl as isize) = *maxval1.offset(nvl as isize);
                    nvl += 1;
                    nvl;
                }
                firstpart = 0 as libc::c_int;
            } else {
                nvl = 0 as libc::c_int;
                while nvl < nclass {
                    if *maxval.offset(nvl as isize) != *maxval1.offset(nvl as isize) {
                        fprintf(
                            dap_err,
                            b"(loglin) Variable %s has different numbers of levels in different parts of dataset %s\n\0"
                                as *const u8 as *const libc::c_char,
                            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                                .do_nam)
                                .offset(*classv.offset((npart + nvl) as isize) as isize),
                            fname,
                        );
                        exit(1 as libc::c_int);
                    }
                    nvl += 1;
                    nvl;
                }
            }
            nvl = 0 as libc::c_int;
            while nvl < nclass {
                *maxval1.offset(nvl as isize) = 0.0f64;
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        *classv.offset((npart + nclass + nvl) as isize) as isize,
                    ) = 0.0f64;
                nvl += 1;
                nvl;
            }
        }
        if more != 0 {
            if dap_newpart(classv, npart) != 0 {
                nv = nclass + 1 as libc::c_int;
            } else {
                nv = 1 as libc::c_int;
                while nv <= nclass {
                    if dap_newpart(classv, npart + nv) != 0 {
                        break;
                    }
                    nv += 1;
                    nv;
                }
            }
            if nv <= nclass {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        *classv.offset((npart + nclass + nv - 1 as libc::c_int) as isize)
                            as isize,
                    ) += 1.0f64;
                oneval = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .do_dbl)
                    .offset(
                        *classv.offset((npart + nclass + nv - 1 as libc::c_int) as isize)
                            as isize,
                    );
                if oneval > 99.0f64 {
                    fprintf(
                        dap_err,
                        b"(loglin) Number of levels (%g) for %s exceeds maximum (100)\n\0"
                            as *const u8 as *const libc::c_char,
                        oneval + 1.0f64,
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_nam)
                            .offset(
                                *classv
                                    .offset((npart + nclass + nv - 1 as libc::c_int) as isize)
                                    as isize,
                            ),
                    );
                    exit(1 as libc::c_int);
                }
                if oneval > *maxval1.offset((nv - 1 as libc::c_int) as isize) {
                    *maxval1.offset((nv - 1 as libc::c_int) as isize) = oneval;
                    onelen = strlen(
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_str)
                            .offset(
                                *classv.offset((npart + nv - 1 as libc::c_int) as isize)
                                    as isize,
                            ),
                    ) as libc::c_int;
                    if onelen > *maxlen.offset((nv - 1 as libc::c_int) as isize) {
                        *maxlen.offset((nv - 1 as libc::c_int) as isize) = onelen;
                    }
                    if firstpart != 0 {
                        let ref mut fresh24 = (**endcv
                            .offset((nv - 1 as libc::c_int) as isize))
                            .next;
                        *fresh24 = dap_malloc(
                            ::std::mem::size_of::<valnode>() as libc::c_ulong
                                as libc::c_int,
                            b"\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        ) as *mut valnode;
                        let ref mut fresh25 = *endcv
                            .offset((nv - 1 as libc::c_int) as isize);
                        *fresh25 = (**endcv.offset((nv - 1 as libc::c_int) as isize))
                            .next;
                        let ref mut fresh26 = (**endcv
                            .offset((nv - 1 as libc::c_int) as isize))
                            .value;
                        *fresh26 = dap_malloc(
                            onelen + 1 as libc::c_int,
                            b"\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        let ref mut fresh27 = (**endcv
                            .offset((nv - 1 as libc::c_int) as isize))
                            .next;
                        *fresh27 = 0 as *mut node;
                        strcpy(
                            (**endcv.offset((nv - 1 as libc::c_int) as isize)).value,
                            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                                .do_str)
                                .offset(
                                    *classv.offset((npart + nv - 1 as libc::c_int) as isize)
                                        as isize,
                                ),
                        );
                    }
                }
            } else if firstpart != 0 {
                nvl = 0 as libc::c_int;
                while nvl < nclass {
                    let ref mut fresh28 = *classval.offset(nvl as isize);
                    *fresh28 = dap_malloc(
                        ::std::mem::size_of::<valnode>() as libc::c_ulong as libc::c_int,
                        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ) as *mut valnode;
                    let ref mut fresh29 = *endcv.offset(nvl as isize);
                    *fresh29 = *classval.offset(nvl as isize);
                    onelen = strlen(
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_str)
                            .offset(*classv.offset((npart + nvl) as isize) as isize),
                    ) as libc::c_int;
                    if onelen > *maxlen.offset(nvl as isize) {
                        *maxlen.offset(nvl as isize) = onelen;
                    }
                    let ref mut fresh30 = (**endcv.offset(nvl as isize)).value;
                    *fresh30 = dap_malloc(
                        onelen + 1 as libc::c_int,
                        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                    let ref mut fresh31 = (**endcv.offset(nvl as isize)).next;
                    *fresh31 = 0 as *mut node;
                    strcpy(
                        (**endcv.offset(nvl as isize)).value,
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_str)
                            .offset(*classv.offset((npart + nvl) as isize) as isize),
                    );
                    nvl += 1;
                    nvl;
                }
            }
            c = nv;
            while c < nclass {
                if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(*classv.offset((npart + nclass + c) as isize) as isize)
                    != *maxval1.offset(c as isize)
                {
                    fprintf(
                        dap_err,
                        b"(loglin) Variable %s has different numbers of levels\n\0"
                            as *const u8 as *const libc::c_char,
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_nam)
                            .offset(*classv.offset((npart + c) as isize) as isize),
                    );
                    exit(1 as libc::c_int);
                }
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        *classv.offset((npart + nclass + c) as isize) as isize,
                    ) = 0.0f64;
                c += 1;
                c;
            }
            output();
        }
    }
    dap_head(0 as *mut libc::c_int, 0 as libc::c_int);
    fputs(
        b"Loglinear model:\nnumerical indexes of classification variables\n\n\0"
            as *const u8 as *const libc::c_char,
        dap_lst,
    );
    fputs(b"Number\0" as *const u8 as *const libc::c_char, dap_lst);
    maxmaxval = 0 as libc::c_int;
    c = 0 as libc::c_int;
    while c < nclass {
        onelen = strlen(*class.offset(c as isize)) as libc::c_int;
        if *maxlen.offset(c as isize) < onelen {
            *maxlen.offset(c as isize) = onelen;
        }
        sprintf(
            formstr.as_mut_ptr(),
            b"  %%-%ds\0" as *const u8 as *const libc::c_char,
            *maxlen.offset(c as isize),
        );
        fprintf(dap_lst, formstr.as_mut_ptr(), *class.offset(c as isize));
        if *maxval.offset(c as isize) as libc::c_int > maxmaxval {
            maxmaxval = *maxval.offset(c as isize) as libc::c_int;
        }
        c += 1;
        c;
    }
    putc('\n' as i32, dap_lst);
    fputs(b"------\0" as *const u8 as *const libc::c_char, dap_lst);
    c = 0 as libc::c_int;
    while c < nclass {
        putc(' ' as i32, dap_lst);
        putc(' ' as i32, dap_lst);
        cc = 0 as libc::c_int;
        while cc < *maxlen.offset(c as isize) {
            putc('-' as i32, dap_lst);
            cc += 1;
            cc;
        }
        let ref mut fresh32 = *endcv.offset(c as isize);
        *fresh32 = *classval.offset(c as isize);
        c += 1;
        c;
    }
    putc('\n' as i32, dap_lst);
    cc = 0 as libc::c_int;
    while cc <= maxmaxval {
        fprintf(
            dap_lst,
            b"%6d\0" as *const u8 as *const libc::c_char,
            cc + 1 as libc::c_int,
        );
        c = 0 as libc::c_int;
        while c < nclass {
            if !(*endcv.offset(c as isize)).is_null() {
                sprintf(
                    formstr.as_mut_ptr(),
                    b"  %%-%ds\0" as *const u8 as *const libc::c_char,
                    *maxlen.offset(c as isize),
                );
                fprintf(
                    dap_lst,
                    formstr.as_mut_ptr(),
                    (**endcv.offset(c as isize)).value,
                );
                let ref mut fresh33 = *endcv.offset(c as isize);
                *fresh33 = (**endcv.offset(c as isize)).next;
            } else {
                sprintf(
                    formstr.as_mut_ptr(),
                    b"  %%%ds\0" as *const u8 as *const libc::c_char,
                    *maxlen.offset(c as isize),
                );
                fprintf(
                    dap_lst,
                    formstr.as_mut_ptr(),
                    b"\0" as *const u8 as *const libc::c_char,
                );
            }
            c += 1;
            c;
        }
        putc('\n' as i32, dap_lst);
        cc += 1;
        cc;
    }
    putc('\n' as i32, dap_lst);
    nterm = 1 as libc::c_int;
    nv = 0 as libc::c_int;
    while nv < nclass {
        nterm *= 2 as libc::c_int;
        nv += 1;
        nv;
    }
    term = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(nterm as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    pattern = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(nclass as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int as *mut libc::c_uint;
    c = 1 as libc::c_int;
    *pattern.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uint;
    while c < nclass {
        *pattern
            .offset(
                c as isize,
            ) = (2 as libc::c_int as libc::c_uint)
            .wrapping_mul(*pattern.offset((c - 1 as libc::c_int) as isize));
        c += 1;
        c;
    }
    nterm = llparse(class, nterm, pattern, model0, model1, term);
    nparam0 = 1 as libc::c_int;
    sellen = 5 as libc::c_int;
    t = 1 as libc::c_int;
    while t <= nterm {
        if *term.offset(t as isize) != 0 {
            termparam = 1 as libc::c_int;
            c = 0 as libc::c_int;
            termlen = 0 as libc::c_int;
            while c < nclass {
                if *pattern.offset(c as isize) & !t as libc::c_uint == 0 {
                    termparam *= *maxval.offset(c as isize) as libc::c_int;
                    termlen = (termlen as libc::c_ulong)
                        .wrapping_add(
                            (strlen(*class.offset(c as isize)))
                                .wrapping_add(4 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                }
                c += 1;
                c;
            }
            nparam0 += termparam;
            sellen += termparam * (termlen + 2 as libc::c_int);
        }
        t += 1;
        t;
    }
    param = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nparam0 as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    paramlistmem = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(nparam0 as libc::c_ulong)
            .wrapping_mul(nclass as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    paramlist = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_int>() as libc::c_ulong)
            .wrapping_mul(nparam0 as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_int;
    p = 0 as libc::c_int;
    while p < nparam0 {
        let ref mut fresh34 = *paramlist.offset(p as isize);
        *fresh34 = paramlistmem.offset((p * nclass) as isize);
        p += 1;
        p;
    }
    select = dap_malloc(
        sellen,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    selterm = dap_malloc(
        sellen,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    sub = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(nclass as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    t = 1 as libc::c_int;
    strcpy(select, b"_mu\0" as *const u8 as *const libc::c_char);
    numparam = 0 as libc::c_int;
    while t <= nterm {
        *selterm.offset(0 as libc::c_int as isize) = ' ' as i32 as libc::c_char;
        *selterm.offset(1 as libc::c_int as isize) = ' ' as i32 as libc::c_char;
        if *term.offset(t as isize) != 0 {
            if *term.offset(t as isize) == 2 as libc::c_int
                || *model0.offset(0 as libc::c_int as isize) == 0
            {
                *selterm.offset(1 as libc::c_int as isize) = ' ' as i32 as libc::c_char;
            } else {
                *selterm.offset(1 as libc::c_int as isize) = '?' as i32 as libc::c_char;
            }
            *selterm.offset(2 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            c = 0 as libc::c_int;
            while c < nclass {
                if *pattern.offset(c as isize) & !t as libc::c_uint == 0 {
                    if *selterm.offset(2 as libc::c_int as isize) == 0 {
                        strcat(selterm, *class.offset(c as isize));
                    } else {
                        strcat(selterm, b"*\0" as *const u8 as *const libc::c_char);
                        strcat(selterm, *class.offset(c as isize));
                    }
                }
                c += 1;
                c;
            }
            c = 0 as libc::c_int;
            while c < nclass {
                *sub.offset(c as isize) = 1 as libc::c_int;
                c += 1;
                c;
            }
            loop {
                numparam += 1;
                numparam;
                strcat(select, selterm);
                cc = 0 as libc::c_int;
                while cc < nclass {
                    if *pattern.offset(cc as isize) & !t as libc::c_uint == 0 {
                        strcat(select, b":\0" as *const u8 as *const libc::c_char);
                        sprintf(
                            select.offset(strlen(select) as isize),
                            b"%d\0" as *const u8 as *const libc::c_char,
                            *sub.offset(cc as isize),
                        );
                        *(*paramlist.offset(numparam as isize))
                            .offset(
                                cc as isize,
                            ) = *sub.offset(cc as isize) - 1 as libc::c_int;
                    } else {
                        *(*paramlist.offset(numparam as isize))
                            .offset(cc as isize) = -(1 as libc::c_int);
                    }
                    cc += 1;
                    cc;
                }
                c = nclass - 1 as libc::c_int;
                while c >= 0 as libc::c_int {
                    if *pattern.offset(c as isize) & !t as libc::c_uint == 0
                        && *sub.offset(c as isize)
                            < *maxval.offset(c as isize) as libc::c_int
                    {
                        let ref mut fresh35 = *sub.offset(c as isize);
                        *fresh35 += 1;
                        *fresh35;
                        cc = c + 1 as libc::c_int;
                        while cc < nclass {
                            *sub.offset(cc as isize) = 1 as libc::c_int;
                            cc += 1;
                            cc;
                        }
                        break;
                    } else {
                        c -= 1;
                        c;
                    }
                }
                if !(c >= 0 as libc::c_int) {
                    break;
                }
            }
        }
        t += 1;
        t;
    }
    numparam += 1;
    numparam;
    *param.offset(0 as libc::c_int as isize) = 1.0f64;
    p = 1 as libc::c_int;
    while p < numparam {
        *param.offset(p as isize) = 0.0f64;
        p += 1;
        p;
    }
    categ(
        fname1,
        varlist1,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                ) -> libc::c_double,
            >,
            Option::<unsafe extern "C" fn() -> libc::c_double>,
        >(
            Some(
                llexpect
                    as unsafe extern "C" fn(
                        *mut libc::c_double,
                        *mut libc::c_double,
                    ) -> libc::c_double,
            ),
        ),
        param,
        select,
        part,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    inset(catname);
    c = 0 as libc::c_int;
    while c < nclass {
        strcpy(vardef, *class.offset(c as isize));
        sprintf(
            vardef.offset(strlen(vardef) as isize),
            b" %d\0" as *const u8 as *const libc::c_char,
            *maxlen.offset(c as isize),
        );
        *classv.offset(c as isize) = dap_vd(vardef, 0 as libc::c_int);
        c += 1;
        c;
    }
    outset(fname1, b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    dap_list(varlist1, classv.offset(nclass as isize), nclass + 1 as libc::c_int);
    while step() != 0 {
        c = 0 as libc::c_int;
        while c < nclass {
            nodeptr = *classval.offset(c as isize);
            nvl = 0 as libc::c_int;
            while nvl
                < *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        *classv.offset((nclass + 1 as libc::c_int + c) as isize) as isize,
                    ) as libc::c_int
            {
                nodeptr = (*nodeptr).next;
                nvl += 1;
                nvl;
            }
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(*classv.offset(c as isize) as isize),
                (*nodeptr).value,
            );
            c += 1;
            c;
        }
        output();
    }
    dap_free(
        fnamefil as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        filarg as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        fname1 as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        catname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        varlist1 as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        outlist as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        vardef as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        classv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        maxval as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        maxval1 as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        term as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        param as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        select as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        selterm as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        sub as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        classmem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        class as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        pattern as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        paramlistmem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        paramlist as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    c = 0 as libc::c_int;
    while c < nclass {
        while !(*classval.offset(c as isize)).is_null() {
            dap_free(
                (**classval.offset(c as isize)).value as *mut libc::c_void,
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            let ref mut fresh36 = *endcv.offset(c as isize);
            *fresh36 = (**classval.offset(c as isize)).next;
            dap_free(
                *classval.offset(c as isize) as *mut libc::c_void,
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            let ref mut fresh37 = *classval.offset(c as isize);
            *fresh37 = *endcv.offset(c as isize);
        }
        c += 1;
        c;
    }
    dap_free(
        classval as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        endcv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        maxlen as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn findparam(
    mut pname: *mut libc::c_char,
    mut param: *mut *mut libc::c_char,
    mut nparam: libc::c_int,
) -> libc::c_int {
    let mut p: libc::c_int = 0;
    p = 0 as libc::c_int;
    while p < nparam {
        if strcmp(pname, *param.offset(p as isize)) == 0 {
            return p;
        }
        p += 1;
        p;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn estimate(
    mut fname: *mut libc::c_char,
    mut parameters: *mut libc::c_char,
    mut definitions: *mut libc::c_char,
    mut part: *mut libc::c_char,
) {
    let mut parammem: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut param: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut start: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut nparam: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut p1: libc::c_int = 0;
    let mut defstate: libc::c_int = 0;
    let mut defmem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut def: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut coeff: libc::c_double = 0.;
    let mut sign: libc::c_double = 0.;
    let mut place: libc::c_double = 0.;
    let mut pnum1: libc::c_int = 0;
    let mut pnum2: libc::c_int = 0;
    let mut pnum3: libc::c_int = 0;
    let mut defnum: libc::c_int = 0;
    let mut ninput: libc::c_int = 0;
    let mut estimate_0: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut covmem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut cov: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut typen: libc::c_int = 0;
    let mut param1n: libc::c_int = 0;
    let mut param2n: libc::c_int = 0;
    let mut covn: libc::c_int = 0;
    let mut more: libc::c_int = 0;
    let mut partv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut npart: libc::c_int = 0;
    parammem = dap_malloc(
        (strlen(parameters))
            .wrapping_add(strlen(definitions))
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    nparam = (strlen(parameters))
        .wrapping_add(strlen(definitions))
        .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    param = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(nparam as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_char;
    defmem = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nparam as libc::c_ulong)
            .wrapping_mul(nparam as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    def = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nparam as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_double;
    p = 0 as libc::c_int;
    while p < nparam {
        let ref mut fresh38 = *def.offset(p as isize);
        *fresh38 = defmem.offset((p * nparam) as isize);
        p1 = 0 as libc::c_int;
        while p1 < nparam {
            *(*def.offset(p as isize)).offset(p1 as isize) = 0.0f64;
            p1 += 1;
            p1;
        }
        p += 1;
        p;
    }
    start = 0 as libc::c_int;
    while *parameters.offset(start as isize) as libc::c_int == ' ' as i32 {
        start += 1;
        start;
    }
    nparam = 0 as libc::c_int;
    p = 0 as libc::c_int;
    while *parameters.offset(start as isize) != 0 {
        let fresh39 = nparam;
        nparam = nparam + 1;
        let ref mut fresh40 = *param.offset(fresh39 as isize);
        *fresh40 = parammem.offset(p as isize);
        end = start;
        while *parameters.offset(end as isize) as libc::c_int != 0
            && *parameters.offset(end as isize) as libc::c_int != ' ' as i32
        {
            let fresh41 = end;
            end = end + 1;
            let fresh42 = p;
            p = p + 1;
            *parammem.offset(fresh42 as isize) = *parameters.offset(fresh41 as isize);
        }
        let fresh43 = p;
        p = p + 1;
        *parammem.offset(fresh43 as isize) = '\0' as i32 as libc::c_char;
        while *parameters.offset(end as isize) as libc::c_int == ' ' as i32 {
            end += 1;
            end;
        }
        start = end;
    }
    ninput = nparam;
    start = 0 as libc::c_int;
    while *definitions.offset(start as isize) as libc::c_int == ' ' as i32 {
        start += 1;
        start;
    }
    defstate = 0 as libc::c_int;
    while *definitions.offset(start as isize) != 0 {
        let ref mut fresh44 = *param.offset(nparam as isize);
        *fresh44 = parammem.offset(p as isize);
        if *definitions.offset(start as isize) as libc::c_int == '+' as i32
            || *definitions.offset(start as isize) as libc::c_int == '-' as i32
        {
            sign = 2.0f64
                * (*definitions.offset(start as isize) as libc::c_int == '+' as i32)
                    as libc::c_int as libc::c_double - 1.0f64;
            start += 1;
            start;
            while *definitions.offset(start as isize) as libc::c_int == ' ' as i32 {
                start += 1;
                start;
            }
        }
        if *definitions.offset(start as isize) as libc::c_int == '.' as i32
            || '0' as i32 <= *definitions.offset(start as isize) as libc::c_int
                && *definitions.offset(start as isize) as libc::c_int <= '9' as i32
        {
            coeff = 0.0f64;
            place = 0.0f64;
            while *definitions.offset(start as isize) as libc::c_int == '.' as i32
                || '0' as i32 <= *definitions.offset(start as isize) as libc::c_int
                    && *definitions.offset(start as isize) as libc::c_int <= '9' as i32
            {
                if *definitions.offset(start as isize) as libc::c_int == '.' as i32 {
                    if place > 0.0f64 {
                        fprintf(
                            dap_err,
                            b"(estimate) bad coefficient in definition: %s\n\0"
                                as *const u8 as *const libc::c_char,
                            definitions.offset(start as isize),
                        );
                        exit(1 as libc::c_int);
                    } else {
                        place = 1.0f64;
                    }
                } else {
                    coeff = 10.0f64 * coeff
                        + (*definitions.offset(start as isize) as libc::c_int
                            - '0' as i32) as libc::c_double;
                    if place > 0.0f64 {
                        place *= 10.0f64;
                    }
                }
                start += 1;
                start;
            }
            coeff *= sign;
            if place > 0.0f64 {
                coeff /= place;
            }
            while *definitions.offset(start as isize) as libc::c_int == ' ' as i32 {
                start += 1;
                start;
            }
        } else {
            coeff = sign;
        }
        end = start;
        while *definitions.offset(end as isize) as libc::c_int != 0
            && *definitions.offset(end as isize) as libc::c_int != ' ' as i32
            && *definitions.offset(end as isize) as libc::c_int != '=' as i32
            && *definitions.offset(end as isize) as libc::c_int != '+' as i32
            && *definitions.offset(end as isize) as libc::c_int != '-' as i32
        {
            let fresh45 = end;
            end = end + 1;
            let fresh46 = p;
            p = p + 1;
            *parammem.offset(fresh46 as isize) = *definitions.offset(fresh45 as isize);
        }
        let fresh47 = p;
        p = p + 1;
        *parammem.offset(fresh47 as isize) = '\0' as i32 as libc::c_char;
        pnum1 = findparam(*param.offset(nparam as isize), param, nparam);
        if pnum1 < 0 as libc::c_int {
            nparam += 1;
            nparam;
        } else {
            p = (*param.offset(nparam as isize)).offset_from(parammem) as libc::c_long
                as libc::c_int;
        }
        while *definitions.offset(end as isize) as libc::c_int == ' ' as i32 {
            end += 1;
            end;
        }
        match defstate {
            0 => {
                if *definitions.offset(end as isize) as libc::c_int == '=' as i32 {
                    defstate = 1 as libc::c_int;
                    end += 1;
                    end;
                    while *definitions.offset(end as isize) as libc::c_int == ' ' as i32
                    {
                        end += 1;
                        end;
                    }
                } else {
                    fprintf(
                        dap_err,
                        b"(estimate) definition starting at %s missing an =\n\0"
                            as *const u8 as *const libc::c_char,
                        definitions.offset(start as isize),
                    );
                    exit(1 as libc::c_int);
                }
                defnum = nparam - 1 as libc::c_int;
                sign = 1.0f64;
            }
            1 => {
                if pnum1 < 0 as libc::c_int {
                    fprintf(
                        dap_err,
                        b"(estimate) undefined parameter %s in definition\n\0"
                            as *const u8 as *const libc::c_char,
                        *param.offset((nparam - 1 as libc::c_int) as isize),
                    );
                    exit(1 as libc::c_int);
                }
                if sign == 0.0f64 {
                    fprintf(
                        dap_err,
                        b"(estimate) missing sign or coefficient for parameter %s in definition\n\0"
                            as *const u8 as *const libc::c_char,
                        *param.offset((nparam - 1 as libc::c_int) as isize),
                    );
                    exit(1 as libc::c_int);
                }
                *(*def.offset(defnum as isize)).offset(pnum1 as isize) = coeff;
                if *definitions.offset(end as isize) as libc::c_int != '+' as i32
                    && *definitions.offset(end as isize) as libc::c_int != '-' as i32
                {
                    defstate = 0 as libc::c_int;
                }
                sign = 0.0f64;
            }
            _ => {}
        }
        start = end;
    }
    estimate_0 = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nparam as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    covmem = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nparam as libc::c_ulong)
            .wrapping_mul(nparam as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    cov = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nparam as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_double;
    p = 0 as libc::c_int;
    while p < nparam {
        let ref mut fresh48 = *cov.offset(p as isize);
        *fresh48 = covmem.offset((nparam * p) as isize);
        p += 1;
        p;
    }
    inset(fname);
    typen = dap_varnum(
        b"_type_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if typen < 0 as libc::c_int {
        fprintf(
            dap_err,
            b"(estimate) missing _type_ variable in dataset %s\n\0" as *const u8
                as *const libc::c_char,
            fname,
        );
        exit(1 as libc::c_int);
    }
    param1n = dap_varnum(
        b"_param1_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if param1n < 0 as libc::c_int {
        fprintf(
            dap_err,
            b"(estimate) missing _param1_ variable in dataset %s\n\0" as *const u8
                as *const libc::c_char,
            fname,
        );
        exit(1 as libc::c_int);
    }
    param2n = dap_varnum(
        b"_param2_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if param2n < 0 as libc::c_int {
        fprintf(
            dap_err,
            b"(estimate) missing _param2_ variable in dataset %s\n\0" as *const u8
                as *const libc::c_char,
            fname,
        );
        exit(1 as libc::c_int);
    }
    covn = dap_varnum(
        b"_cov_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if covn < 0 as libc::c_int {
        fprintf(
            dap_err,
            b"(estimate) missing _cov_ variable in dataset %s\n\0" as *const u8
                as *const libc::c_char,
            fname,
        );
        exit(1 as libc::c_int);
    }
    partv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    npart = dap_list(part, partv, dap_maxvar);
    pnum1 = 0 as libc::c_int;
    while pnum1 < nparam {
        *estimate_0.offset(pnum1 as isize) = 0.0f64 / 0.0f64;
        pnum2 = 0 as libc::c_int;
        while pnum2 < nparam {
            *(*cov.offset(pnum1 as isize)).offset(pnum2 as isize) = 0.0f64 / 0.0f64;
            pnum2 += 1;
            pnum2;
        }
        pnum1 += 1;
        pnum1;
    }
    more = 1 as libc::c_int;
    while more != 0 {
        more = step();
        if dap_newpart(partv, npart) != 0 {
            dap_swap();
            dap_head(partv, npart);
            fputs(
                b"    Estimate           SE  Parameter\n\0" as *const u8
                    as *const libc::c_char,
                dap_lst,
            );
            pnum1 = ninput;
            while pnum1 < nparam {
                *estimate_0.offset(pnum1 as isize) = 0.0f64;
                pnum2 = 0 as libc::c_int;
                while pnum2 < pnum1 {
                    if *(*def.offset(pnum1 as isize)).offset(pnum2 as isize) != 0.
                        && finite(*estimate_0.offset(pnum2 as isize)) == 0
                    {
                        fprintf(
                            dap_err,
                            b"(estimate) estimate for parameter %s not in dataset %s\n\0"
                                as *const u8 as *const libc::c_char,
                            *param.offset(pnum2 as isize),
                            fname,
                        );
                        exit(1 as libc::c_int);
                    }
                    *estimate_0.offset(pnum1 as isize)
                        += *(*def.offset(pnum1 as isize)).offset(pnum2 as isize)
                            * *estimate_0.offset(pnum2 as isize);
                    pnum2 += 1;
                    pnum2;
                }
                *(*cov.offset(pnum1 as isize)).offset(pnum1 as isize) = 0.0f64;
                pnum2 = 0 as libc::c_int;
                while pnum2 < pnum1 {
                    pnum3 = 0 as libc::c_int;
                    while pnum3 < pnum1 {
                        coeff = *(*def.offset(pnum1 as isize)).offset(pnum2 as isize)
                            * *(*def.offset(pnum1 as isize)).offset(pnum3 as isize);
                        *(*cov.offset(pnum1 as isize)).offset(pnum1 as isize)
                            += coeff
                                * *(*cov.offset(pnum2 as isize)).offset(pnum3 as isize);
                        pnum3 += 1;
                        pnum3;
                    }
                    pnum2 += 1;
                    pnum2;
                }
                pnum2 = 0 as libc::c_int;
                while pnum2 < pnum1 {
                    *(*cov.offset(pnum1 as isize)).offset(pnum2 as isize) = 0.0f64;
                    pnum3 = 0 as libc::c_int;
                    while pnum3 < pnum1 {
                        coeff = *(*def.offset(pnum1 as isize)).offset(pnum2 as isize);
                        *(*cov.offset(pnum1 as isize)).offset(pnum2 as isize)
                            += coeff
                                * *(*cov.offset(pnum2 as isize)).offset(pnum3 as isize);
                        pnum3 += 1;
                        pnum3;
                    }
                    *(*cov.offset(pnum2 as isize))
                        .offset(
                            pnum1 as isize,
                        ) = *(*cov.offset(pnum1 as isize)).offset(pnum2 as isize);
                    pnum2 += 1;
                    pnum2;
                }
                pnum1 += 1;
                pnum1;
            }
            pnum1 = ninput;
            while pnum1 < nparam {
                fprintf(
                    dap_lst,
                    b"%12g %12g  %s =\0" as *const u8 as *const libc::c_char,
                    *estimate_0.offset(pnum1 as isize),
                    sqrt(*(*cov.offset(pnum1 as isize)).offset(pnum1 as isize)),
                    *param.offset(pnum1 as isize),
                );
                pnum2 = 0 as libc::c_int;
                while pnum2 < nparam {
                    if *(*def.offset(pnum1 as isize)).offset(pnum2 as isize) != 0.0f64 {
                        putc(' ' as i32, dap_lst);
                        if *(*def.offset(pnum1 as isize)).offset(pnum2 as isize) > 0.0f64
                        {
                            putc('+' as i32, dap_lst);
                        }
                        if *(*def.offset(pnum1 as isize)).offset(pnum2 as isize)
                            == 1.0f64
                        {
                            fprintf(
                                dap_lst,
                                b"%s\0" as *const u8 as *const libc::c_char,
                                *param.offset(pnum2 as isize),
                            );
                        } else if *(*def.offset(pnum1 as isize)).offset(pnum2 as isize)
                            == -1.0f64
                        {
                            fprintf(
                                dap_lst,
                                b" -%s\0" as *const u8 as *const libc::c_char,
                                *param.offset(pnum2 as isize),
                            );
                        } else {
                            fprintf(
                                dap_lst,
                                b"%g%s\0" as *const u8 as *const libc::c_char,
                                *(*def.offset(pnum1 as isize)).offset(pnum2 as isize),
                                *param.offset(pnum2 as isize),
                            );
                        }
                    }
                    pnum2 += 1;
                    pnum2;
                }
                putc('\n' as i32, dap_lst);
                pnum1 += 1;
                pnum1;
            }
            dap_swap();
            pnum1 = 0 as libc::c_int;
            while pnum1 < nparam {
                *estimate_0.offset(pnum1 as isize) = 0.0f64 / 0.0f64;
                pnum2 = 0 as libc::c_int;
                while pnum2 < nparam {
                    *(*cov.offset(pnum1 as isize))
                        .offset(pnum2 as isize) = 0.0f64 / 0.0f64;
                    pnum2 += 1;
                    pnum2;
                }
                pnum1 += 1;
                pnum1;
            }
        }
        if more != 0 {
            if strcmp(
                *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                    .offset(typen as isize),
                b"ESTIMATE\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                pnum2 = findparam(
                    *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                        .offset(param2n as isize),
                    param,
                    ninput,
                );
                if pnum2 >= 0 as libc::c_int {
                    *estimate_0
                        .offset(
                            pnum2 as isize,
                        ) = *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_dbl)
                        .offset(covn as isize);
                }
            } else if strcmp(
                *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                    .offset(typen as isize),
                b"COVAR\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                pnum1 = findparam(
                    *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                        .offset(param1n as isize),
                    param,
                    ninput,
                );
                if pnum1 >= 0 as libc::c_int
                    && {
                        pnum2 = findparam(
                            *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                                .offset(param2n as isize),
                            param,
                            ninput,
                        );
                        pnum2 >= 0 as libc::c_int
                    }
                {
                    *(*cov.offset(pnum1 as isize))
                        .offset(
                            pnum2 as isize,
                        ) = *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_dbl)
                        .offset(covn as isize);
                }
            }
        }
    }
    dap_free(
        parammem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        param as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        defmem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        def as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        estimate_0 as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        covmem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        cov as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        partv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
