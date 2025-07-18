use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut dap_maxvar: libc::c_int;
    static mut dap_namelen: libc::c_int;
    static mut dap_listlen: libc::c_int;
    static mut dap_strlen: libc::c_int;
    static mut dap_maxval: libc::c_int;
    static mut dap_maxbars: libc::c_int;
    static mut dap_maxlev: libc::c_int;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn exit(_: libc::c_int) -> !;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn finite(_: libc::c_double) -> libc::c_int;
    fn rint(_: libc::c_double) -> libc::c_double;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn dap_stats(statlist: *mut libc::c_char, stats: *mut libc::c_int);
    fn dap_newpart(partv: *mut libc::c_int, npartv: libc::c_int) -> libc::c_int;
    fn probt(t1: libc::c_double, di: libc::c_int) -> libc::c_double;
    fn probchisq(x2: libc::c_double, df: libc::c_int) -> libc::c_double;
    fn dap_bincoeff(n: libc::c_double, r: libc::c_double) -> libc::c_double;
    fn dap_free(ptr: *mut libc::c_void, mesg: *mut libc::c_char);
    fn dap_mnsparse(
        varlist: *mut libc::c_char,
        outlist: *mut libc::c_char,
        varv: *mut libc::c_int,
        wtvar: *mut libc::c_int,
        stats: *mut libc::c_int,
    ) -> libc::c_int;
    fn dap_malloc(nbytes: libc::c_int, mesg: *mut libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut dap_obs: [dataobs; 0];
    static mut dap_lst: *mut FILE;
    static mut dap_log: *mut FILE;
    static mut dap_err: *mut FILE;
    static mut dap_sttnm: [[libc::c_char; 9]; 43];
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
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
unsafe extern "C" fn dblcmp(
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
) -> libc::c_int {
    if *x < *y {
        return -(1 as libc::c_int);
    }
    if *x > *y {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ddblcmp(
    mut x: *mut *mut libc::c_double,
    mut y: *mut *mut libc::c_double,
) -> libc::c_int {
    if **x < **y {
        return -(1 as libc::c_int);
    }
    if **x > **y {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
static mut cmp: Option::<unsafe extern "C" fn() -> libc::c_int> = unsafe {
    ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(*mut libc::c_double, *mut libc::c_double) -> libc::c_int,
        >,
        Option::<unsafe extern "C" fn() -> libc::c_int>,
    >(
        Some(
            dblcmp
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
    )
};
static mut dcmp: Option::<unsafe extern "C" fn() -> libc::c_int> = unsafe {
    ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut *mut libc::c_double,
                *mut *mut libc::c_double,
            ) -> libc::c_int,
        >,
        Option::<unsafe extern "C" fn() -> libc::c_int>,
    >(
        Some(
            ddblcmp
                as unsafe extern "C" fn(
                    *mut *mut libc::c_double,
                    *mut *mut libc::c_double,
                ) -> libc::c_int,
        ),
    )
};
unsafe extern "C" fn pctpttest(
    mut wtpt: libc::c_double,
    mut cumwt: libc::c_double,
    mut nextcum: libc::c_double,
    mut pctpt: *mut libc::c_int,
    mut n: libc::c_int,
    mut excess: *mut libc::c_int,
) {
    if cumwt <= wtpt && wtpt < nextcum {
        *pctpt = n;
        if wtpt > cumwt {
            *excess = 1 as libc::c_int;
        }
    }
}
unsafe extern "C" fn pctile2(
    mut val: *mut *mut *mut libc::c_double,
    mut nobs: libc::c_int,
    mut nvar: libc::c_int,
    mut varv: *mut libc::c_int,
    mut wtvar: *mut libc::c_int,
    mut stats: *mut libc::c_int,
) {
    let mut v: libc::c_int = 0;
    let mut pctptmem: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pctpt: *mut *mut libc::c_int = 0 as *mut *mut libc::c_int;
    let mut excessmem: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut excess: *mut *mut libc::c_int = 0 as *mut *mut libc::c_int;
    let mut pn: libc::c_int = 0;
    let mut pi: libc::c_int = 0;
    let mut pct: libc::c_double = 0.;
    static mut sumwt: libc::c_double = 0.;
    static mut cumwt: libc::c_double = 0.;
    static mut nextcum: libc::c_double = 0.;
    static mut wtpt: libc::c_double = 0.;
    let mut upct: [libc::c_double; 9] = [0.; 9];
    let mut n: libc::c_int = 0;
    let mut ptindex: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut q1: libc::c_double = 0.;
    let mut q3: libc::c_double = 0.;
    let mut typen: libc::c_int = 0;
    dap_swap();
    typen = dap_varnum(
        b"_type_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if typen < 0 as libc::c_int {
        fprintf(
            dap_err,
            b"(pctile2) missing _type_ variable\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    pctptmem = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(nvar as libc::c_ulong)
            .wrapping_mul((9 as libc::c_int + 9 as libc::c_int) as libc::c_ulong)
            as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    pctpt = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_int>() as libc::c_ulong)
            .wrapping_mul(nvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_int;
    excessmem = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(nvar as libc::c_ulong)
            .wrapping_mul((9 as libc::c_int + 9 as libc::c_int) as libc::c_ulong)
            as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    excess = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_int>() as libc::c_ulong)
            .wrapping_mul(nvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_int;
    pn = 0 as libc::c_int;
    while pn < 9 as libc::c_int
        && *stats
            .offset(
                (0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int - 9 as libc::c_int + pn)
                    as isize,
            ) != 0
    {
        if sscanf(
            (dap_sttnm[(0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int - 9 as libc::c_int + pn) as usize])
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize),
            b"%lf\0" as *const u8 as *const libc::c_char,
            &mut *upct.as_mut_ptr().offset(pn as isize) as *mut libc::c_double,
        ) != 1 as libc::c_int
        {
            fprintf(
                dap_err,
                b"(pctile2) invalid percentile: %s\n\0" as *const u8
                    as *const libc::c_char,
                (dap_sttnm[(0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int - 9 as libc::c_int + pn)
                    as usize])
                    .as_mut_ptr(),
            );
            exit(1 as libc::c_int);
        }
        pn += 1;
        pn;
    }
    v = 0 as libc::c_int;
    while v < nvar {
        let ref mut fresh0 = *pctpt.offset(v as isize);
        *fresh0 = pctptmem.offset((v * (9 as libc::c_int + 9 as libc::c_int)) as isize);
        let ref mut fresh1 = *excess.offset(v as isize);
        *fresh1 = excessmem.offset((v * (9 as libc::c_int + 9 as libc::c_int)) as isize);
        qsort(
            *val.offset(v as isize) as *mut libc::c_void,
            nobs as size_t,
            ::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> libc::c_int>,
                __compar_fn_t,
            >(dcmp),
        );
        n = 0 as libc::c_int;
        sumwt = 0.0f64;
        while n < nobs {
            sumwt
                += *(*(*val.offset(v as isize)).offset(n as isize))
                    .offset(1 as libc::c_int as isize);
            n += 1;
            n;
        }
        s = 0 as libc::c_int;
        while s < 9 as libc::c_int + pn {
            *(*excess.offset(v as isize)).offset(s as isize) = 0 as libc::c_int;
            *(*pctpt.offset(v as isize)).offset(s as isize) = nobs - 1 as libc::c_int;
            s += 1;
            s;
        }
        n = 0 as libc::c_int;
        cumwt = 0.0f64;
        while n < nobs {
            nextcum = cumwt
                + *(*(*val.offset(v as isize)).offset(n as isize))
                    .offset(1 as libc::c_int as isize);
            pctpttest(
                sumwt / 100.0f64,
                cumwt,
                nextcum,
                *pctpt.offset(v as isize),
                n,
                *excess.offset(v as isize),
            );
            pctpttest(
                sumwt / 20.0f64,
                cumwt,
                nextcum,
                (*pctpt.offset(v as isize)).offset(1 as libc::c_int as isize),
                n,
                (*excess.offset(v as isize)).offset(1 as libc::c_int as isize),
            );
            pctpttest(
                sumwt / 10.0f64,
                cumwt,
                nextcum,
                (*pctpt.offset(v as isize)).offset(2 as libc::c_int as isize),
                n,
                (*excess.offset(v as isize)).offset(2 as libc::c_int as isize),
            );
            pctpttest(
                sumwt / 4.0f64,
                cumwt,
                nextcum,
                (*pctpt.offset(v as isize)).offset(3 as libc::c_int as isize),
                n,
                (*excess.offset(v as isize)).offset(3 as libc::c_int as isize),
            );
            pctpttest(
                sumwt / 2.0f64,
                cumwt,
                nextcum,
                (*pctpt.offset(v as isize)).offset(4 as libc::c_int as isize),
                n,
                (*excess.offset(v as isize)).offset(4 as libc::c_int as isize),
            );
            pctpttest(
                3.0f64 * sumwt / 4.0f64,
                cumwt,
                nextcum,
                (*pctpt.offset(v as isize)).offset(5 as libc::c_int as isize),
                n,
                (*excess.offset(v as isize)).offset(5 as libc::c_int as isize),
            );
            pctpttest(
                9.0f64 * sumwt / 10.0f64,
                cumwt,
                nextcum,
                (*pctpt.offset(v as isize)).offset(6 as libc::c_int as isize),
                n,
                (*excess.offset(v as isize)).offset(6 as libc::c_int as isize),
            );
            pctpttest(
                95.0f64 * sumwt / 100.0f64,
                cumwt,
                nextcum,
                (*pctpt.offset(v as isize)).offset(7 as libc::c_int as isize),
                n,
                (*excess.offset(v as isize)).offset(7 as libc::c_int as isize),
            );
            pctpttest(
                99.0f64 * sumwt / 100.0f64,
                cumwt,
                nextcum,
                (*pctpt.offset(v as isize)).offset(8 as libc::c_int as isize),
                n,
                (*excess.offset(v as isize)).offset(8 as libc::c_int as isize),
            );
            pi = 0 as libc::c_int;
            while pi < pn {
                pctpttest(
                    upct[pi as usize] * sumwt / 100.0f64,
                    cumwt,
                    nextcum,
                    (*pctpt.offset(v as isize))
                        .offset(9 as libc::c_int as isize)
                        .offset(pi as isize),
                    n,
                    (*excess.offset(v as isize))
                        .offset(9 as libc::c_int as isize)
                        .offset(pi as isize),
                );
                pi += 1;
                pi;
            }
            n += 1;
            n;
            cumwt = nextcum;
        }
        v += 1;
        v;
    }
    s = 0 as libc::c_int;
    while s < 9 as libc::c_int + pn {
        if *stats
            .offset(
                (0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + s) as isize,
            ) != 0
        {
            v = 0 as libc::c_int;
            while v < nvar {
                ptindex = *(*pctpt.offset(v as isize)).offset(s as isize)
                    - 1 as libc::c_int;
                if ptindex < 0 as libc::c_int {
                    ptindex = 0 as libc::c_int;
                }
                if *(*excess.offset(v as isize)).offset(s as isize) != 0 {
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                        .offset(
                            *varv.offset(v as isize) as isize,
                        ) = *(*(*val.offset(v as isize))
                        .offset(
                            *(*pctpt.offset(v as isize)).offset(s as isize) as isize,
                        ))
                        .offset(0 as libc::c_int as isize);
                } else {
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                        .offset(
                            *varv.offset(v as isize) as isize,
                        ) = 0.5f64
                        * (*(*(*val.offset(v as isize))
                            .offset(
                                *(*pctpt.offset(v as isize)).offset(s as isize) as isize,
                            ))
                            .offset(0 as libc::c_int as isize)
                            + *(*(*val.offset(v as isize)).offset(ptindex as isize))
                                .offset(0 as libc::c_int as isize));
                }
                v += 1;
                v;
            }
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                (dap_sttnm[(0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + s) as usize])
                    .as_mut_ptr(),
            );
            output();
        }
        s += 1;
        s;
    }
    if *stats.offset(0 as libc::c_int as isize) != 0 {
        v = 0 as libc::c_int;
        while v < nvar {
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(*varv.offset(v as isize) as isize) = nobs as libc::c_double;
            v += 1;
            v;
        }
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typen as isize),
            b"N\0" as *const u8 as *const libc::c_char,
        );
        output();
    }
    if *stats
        .offset(
            (0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int) as isize,
        ) != 0
    {
        v = 0 as libc::c_int;
        while v < nvar {
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(
                    *varv.offset(v as isize) as isize,
                ) = *(*(*val.offset(v as isize)).offset(0 as libc::c_int as isize))
                .offset(0 as libc::c_int as isize);
            v += 1;
            v;
        }
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typen as isize),
            b"MIN\0" as *const u8 as *const libc::c_char,
        );
        output();
    }
    if *stats
        .offset(
            (0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int) as isize,
        ) != 0
    {
        v = 0 as libc::c_int;
        while v < nvar {
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(
                    *varv.offset(v as isize) as isize,
                ) = *(*(*val.offset(v as isize))
                .offset((nobs - 1 as libc::c_int) as isize))
                .offset(0 as libc::c_int as isize);
            v += 1;
            v;
        }
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typen as isize),
            b"MAX\0" as *const u8 as *const libc::c_char,
        );
        output();
    }
    if *stats
        .offset(
            (0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int) as isize,
        ) != 0
    {
        v = 0 as libc::c_int;
        while v < nvar {
            ptindex = *(*pctpt.offset(v as isize)).offset(3 as libc::c_int as isize)
                - 1 as libc::c_int;
            if ptindex < 0 as libc::c_int {
                ptindex = 0 as libc::c_int;
            }
            if *(*excess.offset(v as isize)).offset(3 as libc::c_int as isize) != 0 {
                q1 = *(*(*val.offset(v as isize))
                    .offset(
                        *(*pctpt.offset(v as isize)).offset(3 as libc::c_int as isize)
                            as isize,
                    ))
                    .offset(0 as libc::c_int as isize);
            } else {
                q1 = 0.5f64
                    * (*(*(*val.offset(v as isize))
                        .offset(
                            *(*pctpt.offset(v as isize))
                                .offset(3 as libc::c_int as isize) as isize,
                        ))
                        .offset(0 as libc::c_int as isize)
                        + *(*(*val.offset(v as isize)).offset(ptindex as isize))
                            .offset(0 as libc::c_int as isize));
            }
            ptindex = *(*pctpt.offset(v as isize)).offset(5 as libc::c_int as isize)
                - 1 as libc::c_int;
            if ptindex < 0 as libc::c_int {
                ptindex = 0 as libc::c_int;
            }
            if *(*excess.offset(v as isize)).offset(5 as libc::c_int as isize) != 0 {
                q3 = *(*(*val.offset(v as isize))
                    .offset(
                        *(*pctpt.offset(v as isize)).offset(5 as libc::c_int as isize)
                            as isize,
                    ))
                    .offset(0 as libc::c_int as isize);
            } else {
                q3 = 0.5f64
                    * (*(*(*val.offset(v as isize))
                        .offset(
                            *(*pctpt.offset(v as isize))
                                .offset(5 as libc::c_int as isize) as isize,
                        ))
                        .offset(0 as libc::c_int as isize)
                        + *(*(*val.offset(v as isize)).offset(ptindex as isize))
                            .offset(0 as libc::c_int as isize));
            }
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(*varv.offset(v as isize) as isize) = q3 - q1;
            v += 1;
            v;
        }
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typen as isize),
            b"QRANGE\0" as *const u8 as *const libc::c_char,
        );
        output();
    }
    dap_swap();
    dap_free(
        pctptmem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        pctpt as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        excessmem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        excess as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn pctile1(
    mut val: *mut *mut *mut libc::c_double,
    mut nobs: libc::c_int,
    mut nvar: libc::c_int,
    mut varv: *mut libc::c_int,
    mut stats: *mut libc::c_int,
) {
    let mut v: libc::c_int = 0;
    let mut dnobs: libc::c_double = 0.;
    static mut pctpt: [libc::c_int; 18] = [0; 18];
    let mut pn: libc::c_int = 0;
    let mut pct: libc::c_double = 0.;
    static mut excess: [libc::c_int; 18] = [0; 18];
    let mut ptindex: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut q1: libc::c_double = 0.;
    let mut q3: libc::c_double = 0.;
    let mut typen: libc::c_int = 0;
    dap_swap();
    typen = dap_varnum(
        b"_type_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if typen < 0 as libc::c_int {
        fprintf(
            dap_err,
            b"(pctile1) missing _type_ variable\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    dnobs = nobs as libc::c_double;
    pctpt[0 as libc::c_int as usize] = floor(dnobs / 100.0f64) as libc::c_int;
    excess[0 as libc::c_int
        as usize] = (dnobs / 100.0f64 > floor(dnobs / 100.0f64)) as libc::c_int;
    pctpt[1 as libc::c_int as usize] = floor(dnobs / 20.0f64) as libc::c_int;
    excess[1 as libc::c_int
        as usize] = (dnobs / 20.0f64 > floor(dnobs / 20.0f64)) as libc::c_int;
    pctpt[2 as libc::c_int as usize] = floor(dnobs / 10.0f64) as libc::c_int;
    excess[2 as libc::c_int
        as usize] = (dnobs / 10.0f64 > floor(dnobs / 10.0f64)) as libc::c_int;
    pctpt[3 as libc::c_int as usize] = floor(dnobs / 4.0f64) as libc::c_int;
    excess[3 as libc::c_int
        as usize] = (dnobs / 4.0f64 > floor(dnobs / 4.0f64)) as libc::c_int;
    pctpt[4 as libc::c_int as usize] = floor(dnobs / 2.0f64) as libc::c_int;
    excess[4 as libc::c_int
        as usize] = (dnobs / 2.0f64 > floor(dnobs / 2.0f64)) as libc::c_int;
    pctpt[5 as libc::c_int as usize] = floor(3.0f64 * dnobs / 4.0f64) as libc::c_int;
    excess[5 as libc::c_int
        as usize] = (3.0f64 * dnobs / 4.0f64 > floor(3.0f64 * dnobs / 4.0f64))
        as libc::c_int;
    pctpt[6 as libc::c_int as usize] = floor(9.0f64 * dnobs / 10.0f64) as libc::c_int;
    excess[6 as libc::c_int
        as usize] = (9.0f64 * dnobs / 10.0f64 > floor(9.0f64 * dnobs / 10.0f64))
        as libc::c_int;
    pctpt[7 as libc::c_int as usize] = floor(95.0f64 * dnobs / 100.0f64) as libc::c_int;
    excess[7 as libc::c_int
        as usize] = (95.0f64 * dnobs / 100.0f64 > floor(95.0f64 * dnobs / 100.0f64))
        as libc::c_int;
    pctpt[8 as libc::c_int as usize] = floor(99.0f64 * dnobs / 100.0f64) as libc::c_int;
    excess[8 as libc::c_int
        as usize] = (99.0f64 * dnobs / 100.0f64 > floor(99.0f64 * dnobs / 100.0f64))
        as libc::c_int;
    pn = 0 as libc::c_int;
    while pn < 9 as libc::c_int
        && *stats
            .offset(
                (0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int - 9 as libc::c_int + pn)
                    as isize,
            ) != 0
    {
        sscanf(
            (dap_sttnm[(0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int - 9 as libc::c_int + pn) as usize])
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize),
            b"%lf\0" as *const u8 as *const libc::c_char,
            &mut pct as *mut libc::c_double,
        );
        pctpt[(9 as libc::c_int + pn)
            as usize] = floor(pct * dnobs / 100.0f64) as libc::c_int;
        excess[(9 as libc::c_int + pn)
            as usize] = (pct * dnobs / 100.0f64 > floor(pct * dnobs / 100.0f64))
            as libc::c_int;
        pn += 1;
        pn;
    }
    v = 0 as libc::c_int;
    while v < nvar {
        qsort(
            *val.offset(v as isize) as *mut libc::c_void,
            nobs as size_t,
            ::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> libc::c_int>,
                __compar_fn_t,
            >(dcmp),
        );
        v += 1;
        v;
    }
    s = 0 as libc::c_int;
    while s < 9 as libc::c_int + pn {
        if *stats
            .offset(
                (0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + s) as isize,
            ) != 0
        {
            v = 0 as libc::c_int;
            while v < nvar {
                ptindex = pctpt[s as usize] - 1 as libc::c_int;
                if ptindex < 0 as libc::c_int {
                    ptindex = 0 as libc::c_int;
                }
                if excess[s as usize] != 0 {
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                        .offset(
                            *varv.offset(v as isize) as isize,
                        ) = *(*(*val.offset(v as isize))
                        .offset(pctpt[s as usize] as isize))
                        .offset(0 as libc::c_int as isize);
                } else {
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                        .offset(
                            *varv.offset(v as isize) as isize,
                        ) = 0.5f64
                        * (*(*(*val.offset(v as isize))
                            .offset(pctpt[s as usize] as isize))
                            .offset(0 as libc::c_int as isize)
                            + *(*(*val.offset(v as isize)).offset(ptindex as isize))
                                .offset(0 as libc::c_int as isize));
                }
                v += 1;
                v;
            }
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                (dap_sttnm[(0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + s) as usize])
                    .as_mut_ptr(),
            );
            output();
        }
        s += 1;
        s;
    }
    if *stats.offset(0 as libc::c_int as isize) != 0 {
        v = 0 as libc::c_int;
        while v < nvar {
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(*varv.offset(v as isize) as isize) = dnobs;
            v += 1;
            v;
        }
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typen as isize),
            b"N\0" as *const u8 as *const libc::c_char,
        );
        output();
    }
    if *stats
        .offset(
            (0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int) as isize,
        ) != 0
    {
        v = 0 as libc::c_int;
        while v < nvar {
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(
                    *varv.offset(v as isize) as isize,
                ) = *(*(*val.offset(v as isize)).offset(0 as libc::c_int as isize))
                .offset(0 as libc::c_int as isize);
            v += 1;
            v;
        }
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typen as isize),
            b"MIN\0" as *const u8 as *const libc::c_char,
        );
        output();
    }
    if *stats
        .offset(
            (0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int) as isize,
        ) != 0
    {
        v = 0 as libc::c_int;
        while v < nvar {
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(
                    *varv.offset(v as isize) as isize,
                ) = *(*(*val.offset(v as isize))
                .offset((nobs - 1 as libc::c_int) as isize))
                .offset(0 as libc::c_int as isize);
            v += 1;
            v;
        }
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typen as isize),
            b"MAX\0" as *const u8 as *const libc::c_char,
        );
        output();
    }
    if *stats
        .offset(
            (0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int) as isize,
        ) != 0
    {
        v = 0 as libc::c_int;
        while v < nvar {
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(
                    *varv.offset(v as isize) as isize,
                ) = *(*(*val.offset(v as isize))
                .offset((nobs - 1 as libc::c_int) as isize))
                .offset(0 as libc::c_int as isize)
                - *(*(*val.offset(v as isize)).offset(0 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize);
            v += 1;
            v;
        }
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typen as isize),
            b"RANGE\0" as *const u8 as *const libc::c_char,
        );
        output();
    }
    if *stats
        .offset(
            (0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int) as isize,
        ) != 0
    {
        v = 0 as libc::c_int;
        while v < nvar {
            ptindex = pctpt[3 as libc::c_int as usize] - 1 as libc::c_int;
            if ptindex < 0 as libc::c_int {
                ptindex = 0 as libc::c_int;
            }
            if excess[3 as libc::c_int as usize] != 0 {
                q1 = *(*(*val.offset(v as isize))
                    .offset(pctpt[3 as libc::c_int as usize] as isize))
                    .offset(0 as libc::c_int as isize);
            } else {
                q1 = 0.5f64
                    * (*(*(*val.offset(v as isize))
                        .offset(pctpt[3 as libc::c_int as usize] as isize))
                        .offset(0 as libc::c_int as isize)
                        + *(*(*val.offset(v as isize)).offset(ptindex as isize))
                            .offset(0 as libc::c_int as isize));
            }
            ptindex = pctpt[5 as libc::c_int as usize] - 1 as libc::c_int;
            if ptindex < 0 as libc::c_int {
                ptindex = 0 as libc::c_int;
            }
            if excess[5 as libc::c_int as usize] != 0 {
                q3 = *(*(*val.offset(v as isize))
                    .offset(pctpt[5 as libc::c_int as usize] as isize))
                    .offset(0 as libc::c_int as isize);
            } else {
                q3 = 0.5f64
                    * (*(*(*val.offset(v as isize))
                        .offset(pctpt[5 as libc::c_int as usize] as isize))
                        .offset(0 as libc::c_int as isize)
                        + *(*(*val.offset(v as isize)).offset(ptindex as isize))
                            .offset(0 as libc::c_int as isize));
            }
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(*varv.offset(v as isize) as isize) = q3 - q1;
            v += 1;
            v;
        }
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typen as isize),
            b"QRANGE\0" as *const u8 as *const libc::c_char,
        );
        output();
    }
    dap_swap();
}
pub unsafe extern "C" fn pctiles(
    mut fname: *mut libc::c_char,
    mut varlist: *mut libc::c_char,
    mut statlist: *mut libc::c_char,
    mut marks: *mut libc::c_char,
) {
    let mut outname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stats: [libc::c_int; 43] = [0; 43];
    let mut varv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut markv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nvar: libc::c_int = 0;
    let mut nmark: libc::c_int = 0;
    let mut nobs: libc::c_int = 0;
    let mut outlist: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut wtvar: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut v: libc::c_int = 0;
    let mut valmem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut valpair: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut val: *mut *mut *mut libc::c_double = 0 as *mut *mut *mut libc::c_double;
    let mut weighted: libc::c_int = 0;
    let mut more: libc::c_int = 0;
    if fname.is_null() {
        fputs(
            b"(pctiles) No dataset name given.\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    outname = dap_malloc(
        (strlen(fname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_suffix(
        outname,
        fname,
        b".pct\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    varv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    markv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    wtvar = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    outlist = dap_malloc(
        dap_listlen + 1 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    inset(fname);
    dap_stats(statlist, stats.as_mut_ptr());
    nvar = dap_mnsparse(varlist, outlist, varv, wtvar, stats.as_mut_ptr());
    if !marks.is_null() && *marks.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        strcat(outlist, b" \0" as *const u8 as *const libc::c_char);
        strcat(outlist, marks);
    }
    outset(outname, outlist);
    nmark = dap_list(marks, markv, dap_maxvar);
    valmem = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nvar as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(dap_maxval as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    valpair = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nvar as libc::c_ulong)
            .wrapping_mul(dap_maxval as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_double;
    val = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut *mut libc::c_double;
    v = 0 as libc::c_int;
    weighted = 0 as libc::c_int;
    while v < nvar {
        if *wtvar.offset(v as isize) >= 0 as libc::c_int {
            weighted = 1 as libc::c_int;
        }
        v += 1;
        v;
    }
    nobs = 0 as libc::c_int;
    more = 1 as libc::c_int;
    while more != 0 {
        more = step();
        if dap_newpart(markv, nmark) != 0 {
            if weighted != 0 {
                pctile2(val, nobs, nvar, varv, wtvar, stats.as_mut_ptr());
            } else {
                pctile1(val, nobs, nvar, varv, stats.as_mut_ptr());
            }
            nobs = 0 as libc::c_int;
        }
        if more != 0 {
            if nobs < dap_maxval {
                v = 0 as libc::c_int;
                while v < nvar {
                    let ref mut fresh2 = *valpair
                        .offset((dap_maxval * v + nobs) as isize);
                    *fresh2 = valmem
                        .offset((2 as libc::c_int * (dap_maxval * v + nobs)) as isize);
                    let ref mut fresh3 = *val.offset(v as isize);
                    *fresh3 = valpair.offset((v * dap_maxval) as isize);
                    *(*(*val.offset(v as isize)).offset(nobs as isize))
                        .offset(
                            0 as libc::c_int as isize,
                        ) = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                        .do_dbl)
                        .offset(*varv.offset(v as isize) as isize);
                    if *wtvar.offset(v as isize) >= 0 as libc::c_int {
                        *(*(*val.offset(v as isize)).offset(nobs as isize))
                            .offset(
                                1 as libc::c_int as isize,
                            ) = *((*dap_obs
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize))
                            .do_dbl)
                            .offset(*wtvar.offset(v as isize) as isize);
                    } else {
                        *(*(*val.offset(v as isize)).offset(nobs as isize))
                            .offset(1 as libc::c_int as isize) = 1.0f64;
                    }
                    if finite(
                        *(*(*val.offset(v as isize)).offset(nobs as isize))
                            .offset(0 as libc::c_int as isize),
                    ) == 0
                        || finite(
                            *(*(*val.offset(v as isize)).offset(nobs as isize))
                                .offset(1 as libc::c_int as isize),
                        ) == 0
                    {
                        fprintf(
                            dap_err,
                            b"(pctiles) NaN value %d for %s\n\0" as *const u8
                                as *const libc::c_char,
                            nobs,
                            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                                .do_nam)
                                .offset(*varv.offset(v as isize) as isize),
                        );
                        exit(1 as libc::c_int);
                    }
                    v += 1;
                    v;
                }
            } else {
                fputs(
                    b"(pctiles) Too many data.\n\0" as *const u8 as *const libc::c_char,
                    dap_err,
                );
                exit(1 as libc::c_int);
            }
        }
        nobs += 1;
        nobs;
    }
    dap_free(
        outname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        varv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        markv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        wtvar as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        outlist as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        valmem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        valpair as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        val as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn corr1(
    mut varv: *mut libc::c_int,
    mut nvar: libc::c_int,
    mut cormat: *mut *mut libc::c_double,
    mut ss: *mut libc::c_double,
    mut nobs: libc::c_int,
) {
    let mut varn: [libc::c_int; 3] = [0; 3];
    let mut typen: libc::c_int = 0;
    let mut nf: libc::c_double = 0.;
    let mut v: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut r: libc::c_double = 0.;
    if nobs < 2 as libc::c_int {
        return;
    }
    dap_swap();
    nf = sqrt((nobs - 2 as libc::c_int) as libc::c_double);
    varn[0 as libc::c_int
        as usize] = dap_varnum(
        b"_var1_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    varn[1 as libc::c_int
        as usize] = dap_varnum(
        b"_var2_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    varn[2 as libc::c_int
        as usize] = dap_varnum(
        b"_corr_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    typen = dap_varnum(
        b"_type_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    v = 0 as libc::c_int;
    while v < nvar {
        w = 0 as libc::c_int;
        while w < v {
            *(*cormat.offset(v as isize))
                .offset(w as isize) = *(*cormat.offset(w as isize)).offset(v as isize);
            w += 1;
            w;
        }
        v += 1;
        v;
    }
    strcpy(
        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
            .offset(typen as isize),
        b"N\0" as *const u8 as *const libc::c_char,
    );
    v = 0 as libc::c_int;
    while v < nvar {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(varn[0 as libc::c_int as usize] as isize),
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                .offset(*varv.offset(v as isize) as isize),
        );
        w = 0 as libc::c_int;
        while w < nvar {
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(varn[1 as libc::c_int as usize] as isize),
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(*varv.offset(w as isize) as isize),
            );
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(
                    varn[2 as libc::c_int as usize] as isize,
                ) = nobs as libc::c_double;
            output();
            w += 1;
            w;
        }
        v += 1;
        v;
    }
    strcpy(
        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
            .offset(typen as isize),
        b"CORR\0" as *const u8 as *const libc::c_char,
    );
    v = 0 as libc::c_int;
    while v < nvar {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(varn[0 as libc::c_int as usize] as isize),
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                .offset(*varv.offset(v as isize) as isize),
        );
        w = 0 as libc::c_int;
        while w < nvar {
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(varn[1 as libc::c_int as usize] as isize),
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(*varv.offset(w as isize) as isize),
            );
            if w == v {
                *(*cormat.offset(v as isize)).offset(w as isize) = 1.0f64;
            } else {
                *(*cormat.offset(v as isize)).offset(w as isize)
                    /= sqrt(*ss.offset(v as isize) * *ss.offset(w as isize));
            }
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(
                    varn[2 as libc::c_int as usize] as isize,
                ) = *(*cormat.offset(v as isize)).offset(w as isize);
            output();
            w += 1;
            w;
        }
        v += 1;
        v;
    }
    strcpy(
        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
            .offset(typen as isize),
        b"PCORR\0" as *const u8 as *const libc::c_char,
    );
    v = 0 as libc::c_int;
    while v < nvar {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(varn[0 as libc::c_int as usize] as isize),
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                .offset(*varv.offset(v as isize) as isize),
        );
        w = 0 as libc::c_int;
        while w < nvar {
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(varn[1 as libc::c_int as usize] as isize),
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(*varv.offset(w as isize) as isize),
            );
            r = fabs(*(*cormat.offset(v as isize)).offset(w as isize));
            if r == 1.0f64 {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(varn[2 as libc::c_int as usize] as isize) = 0.0f64;
            } else {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        varn[2 as libc::c_int as usize] as isize,
                    ) = 2.0f64
                    * probt(nf * r / sqrt(1.0f64 - r * r), nobs - 2 as libc::c_int);
            }
            output();
            w += 1;
            w;
        }
        v += 1;
        v;
    }
    dap_swap();
}
pub unsafe extern "C" fn corr(
    mut fname: *mut libc::c_char,
    mut varlist: *mut libc::c_char,
    mut marks: *mut libc::c_char,
) {
    let mut outname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut varstr: [libc::c_char; 11] = [0; 11];
    let mut outlist: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut markv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nmark: libc::c_int = 0;
    let mut varv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nvar: libc::c_int = 0;
    let mut nobs: libc::c_int = 0;
    let mut cormem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut cormat: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut v: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut sum: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut ss: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut vtmp: libc::c_double = 0.;
    let mut tmp: libc::c_double = 0.;
    let mut dn: libc::c_double = 0.;
    let mut more: libc::c_int = 0;
    if fname.is_null() {
        fputs(
            b"(corr) No dataset name given.\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    markv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    varv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    outname = dap_malloc(
        (strlen(fname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_suffix(
        outname,
        fname,
        b".cor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    outlist = dap_malloc(
        (strlen(marks)).wrapping_add(22 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    inset(fname);
    nvar = dap_list(varlist, varv, dap_maxvar);
    cormem = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nvar as libc::c_ulong)
            .wrapping_mul(nvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    cormat = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_double;
    v = 0 as libc::c_int;
    while v < nvar {
        let ref mut fresh4 = *cormat.offset(v as isize);
        *fresh4 = cormem.offset((v * nvar) as isize);
        v += 1;
        v;
    }
    sum = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    ss = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    strcpy(outlist, marks);
    v = 0 as libc::c_int;
    while v < nvar {
        if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
            .offset(*varv.offset(v as isize) as isize) >= 0 as libc::c_int
        {
            fprintf(
                dap_err,
                b"(corr) Variable not of type dap_double: %s\n\0" as *const u8
                    as *const libc::c_char,
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(*varv.offset(v as isize) as isize),
            );
            exit(1 as libc::c_int);
        }
        v += 1;
        v;
    }
    sprintf(
        varstr.as_mut_ptr(),
        b"_var1_ %d\0" as *const u8 as *const libc::c_char,
        dap_namelen,
    );
    dap_vd(varstr.as_mut_ptr(), 0 as libc::c_int);
    sprintf(
        varstr.as_mut_ptr(),
        b"_var2_ %d\0" as *const u8 as *const libc::c_char,
        dap_namelen,
    );
    dap_vd(varstr.as_mut_ptr(), 0 as libc::c_int);
    sprintf(
        varstr.as_mut_ptr(),
        b"_corr_ %d\0" as *const u8 as *const libc::c_char,
        -(1 as libc::c_int),
    );
    dap_vd(varstr.as_mut_ptr(), 0 as libc::c_int);
    strcat(outlist, b" _var1_ _var2_ _corr_\0" as *const u8 as *const libc::c_char);
    outset(outname, outlist);
    nmark = dap_list(marks, markv, dap_maxvar);
    v = 0 as libc::c_int;
    while v < nvar {
        if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
            .offset(*varv.offset(v as isize) as isize) != -(1 as libc::c_int)
        {
            fprintf(
                dap_err,
                b"(corr) variables must be of type double: %s\n\0" as *const u8
                    as *const libc::c_char,
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(*varv.offset(v as isize) as isize),
            );
            exit(1 as libc::c_int);
        }
        w = v + 1 as libc::c_int;
        while w < nvar {
            *(*cormat.offset(v as isize)).offset(w as isize) = 0.0f64;
            w += 1;
            w;
        }
        *sum.offset(v as isize) = 0.0f64;
        *ss.offset(v as isize) = 0.0f64;
        v += 1;
        v;
    }
    nobs = 0 as libc::c_int;
    more = 1 as libc::c_int;
    while more != 0 {
        more = step();
        if dap_newpart(markv, nmark) != 0 {
            corr1(varv, nvar, cormat, ss, nobs);
            v = 0 as libc::c_int;
            while v < nvar {
                w = v + 1 as libc::c_int;
                while w < nvar {
                    *(*cormat.offset(v as isize)).offset(w as isize) = 0.0f64;
                    w += 1;
                    w;
                }
                *sum.offset(v as isize) = 0.0f64;
                *ss.offset(v as isize) = 0.0f64;
                v += 1;
                v;
            }
            nobs = 0 as libc::c_int;
        }
        if more != 0 {
            dn = nobs as libc::c_double;
            v = 0 as libc::c_int;
            while v < nvar {
                vtmp = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .do_dbl)
                    .offset(*varv.offset(v as isize) as isize);
                if finite(vtmp) == 0 {
                    fprintf(
                        dap_err,
                        b"(corr) NaN value %d for %s\n\0" as *const u8
                            as *const libc::c_char,
                        nobs,
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_nam)
                            .offset(*varv.offset(v as isize) as isize),
                    );
                    exit(1 as libc::c_int);
                }
                if nobs != 0 {
                    tmp = *sum.offset(v as isize) - dn * vtmp;
                    w = v + 1 as libc::c_int;
                    while w < nvar {
                        *(*cormat.offset(v as isize)).offset(w as isize)
                            += tmp
                                * (*sum.offset(w as isize)
                                    - dn
                                        * *((*dap_obs
                                            .as_mut_ptr()
                                            .offset(0 as libc::c_int as isize))
                                            .do_dbl)
                                            .offset(*varv.offset(w as isize) as isize))
                                / (dn * (dn + 1.0f64));
                        w += 1;
                        w;
                    }
                    *ss.offset(v as isize) += tmp * tmp / (dn * (dn + 1.0f64));
                }
                *sum.offset(v as isize) += vtmp;
                v += 1;
                v;
            }
        }
        nobs += 1;
        nobs;
    }
    dap_free(
        markv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        varv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        outname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        outlist as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        cormem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        cormat as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        sum as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        ss as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn groupparse(
    mut varspec: *mut libc::c_char,
    mut varv: *mut libc::c_int,
    mut classtype: *mut libc::c_int,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut varname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0;
    let mut nvar: libc::c_int = 0;
    let mut number: libc::c_int = 0;
    if varspec.is_null() {
        return 0 as libc::c_int;
    }
    varname = dap_malloc(
        dap_namelen + 1 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    s = 0 as libc::c_int;
    while *varspec.offset(s as isize) as libc::c_int == ' ' as i32 {
        s += 1;
        s;
    }
    nvar = 0 as libc::c_int;
    number = 0 as libc::c_int;
    while *varspec.offset(s as isize) != 0 {
        *classtype.offset(nvar as isize) = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while *varspec.offset((s + i) as isize) as libc::c_int != 0
            && *varspec.offset((s + i) as isize) as libc::c_int != ' ' as i32
        {
            if i < dap_namelen {
                *varname.offset(i as isize) = *varspec.offset((s + i) as isize);
            } else {
                *varname.offset(i as isize) = '\0' as i32 as libc::c_char;
                fprintf(
                    dap_err,
                    b"(groupparse) variable name too long: %s\n\0" as *const u8
                        as *const libc::c_char,
                    varname,
                );
                exit(1 as libc::c_int);
            }
            i += 1;
            i;
        }
        *varname.offset(i as isize) = '\0' as i32 as libc::c_char;
        s += i;
        v = dap_varnum(varname);
        if v >= 0 as libc::c_int {
            *varv.offset(nvar as isize) = v;
            while *varspec.offset(s as isize) as libc::c_int == ' ' as i32 {
                s += 1;
                s;
            }
            if number != 0 {
                if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
                    .offset(v as isize) != -(1 as libc::c_int)
                {
                    fprintf(
                        dap_err,
                        b"(groupparse) grouping variable must be of type double: %s\n\0"
                            as *const u8 as *const libc::c_char,
                        varname,
                    );
                    exit(1 as libc::c_int);
                }
            } else if '0' as i32 <= *varspec.offset(s as isize) as libc::c_int
                && *varspec.offset(s as isize) as libc::c_int <= '9' as i32
            {
                if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
                    .offset(v as isize) == -(1 as libc::c_int)
                {
                    n = 0 as libc::c_int;
                    i = 0 as libc::c_int;
                    while '0' as i32 <= *varspec.offset((s + i) as isize) as libc::c_int
                        && *varspec.offset((s + i) as isize) as libc::c_int <= '9' as i32
                    {
                        n = 10 as libc::c_int * n
                            + *varspec.offset((s + i) as isize) as libc::c_int
                            - '0' as i32;
                        i += 1;
                        i;
                    }
                    if n == 0
                        || *varspec.offset((s + i) as isize) as libc::c_int != '#' as i32
                            && *varspec.offset((s + i) as isize) as libc::c_int
                                != '^' as i32
                    {
                        fprintf(
                            dap_err,
                            b"(groupparse) invalid number of groups: %s\n\0" as *const u8
                                as *const libc::c_char,
                            varspec.offset(s as isize),
                        );
                        exit(1 as libc::c_int);
                    }
                    match *varspec.offset((s + i) as isize) as libc::c_int {
                        94 => {
                            *classtype.offset(nvar as isize) = -n;
                        }
                        35 => {
                            *classtype.offset(nvar as isize) = n;
                        }
                        _ => {
                            fprintf(
                                dap_err,
                                b"(groupparse) invalid class type: %s\n\0" as *const u8
                                    as *const libc::c_char,
                                varspec.offset(s as isize).offset(i as isize),
                            );
                            exit(1 as libc::c_int);
                        }
                    }
                    s += i + 1 as libc::c_int;
                    while *varspec.offset(s as isize) as libc::c_int == ' ' as i32 {
                        s += 1;
                        s;
                    }
                    if n > dap_maxbars {
                        fprintf(
                            dap_err,
                            b"(groupparse) too many classes: %d\n\0" as *const u8
                                as *const libc::c_char,
                            n,
                        );
                        exit(1 as libc::c_int);
                    }
                } else {
                    fprintf(
                        dap_err,
                        b"(groupparse) grouping variable must be of type double: %s\n\0"
                            as *const u8 as *const libc::c_char,
                        varname,
                    );
                    exit(1 as libc::c_int);
                }
            } else {
                fprintf(
                    dap_err,
                    b"(groupparse) missing number of groups for %s\n\0" as *const u8
                        as *const libc::c_char,
                    varname,
                );
                exit(1 as libc::c_int);
            }
            nvar += 1;
            nvar;
        } else if number < 0 as libc::c_int {
            if *varname.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
                *classtype.offset(0 as libc::c_int as isize) |= 0x1 as libc::c_int;
            }
            if *varname.offset(1 as libc::c_int as isize) as libc::c_int == '0' as i32 {
                *classtype.offset(0 as libc::c_int as isize) |= 0x2 as libc::c_int;
            }
        } else if nvar == 0 {
            if strcmp(varname, b"#\0" as *const u8 as *const libc::c_char) == 0 {
                number = -(1 as libc::c_int);
            } else if strcmp(varname, b"/\0" as *const u8 as *const libc::c_char) == 0 {
                number = -(2 as libc::c_int);
            } else if strcmp(varname, b"%\0" as *const u8 as *const libc::c_char) == 0 {
                number = -(3 as libc::c_int);
            }
            *classtype.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
            nvar += 1;
            nvar;
        } else {
            fprintf(
                dap_err,
                b"(groupparse) unknown variable: %s\n\0" as *const u8
                    as *const libc::c_char,
                varname,
            );
            exit(1 as libc::c_int);
        }
        while *varspec.offset(s as isize) as libc::c_int == ' ' as i32 {
            s += 1;
            s;
        }
    }
    dap_free(
        varname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if number < 0 as libc::c_int {
        return -(4 as libc::c_int) * nvar + number;
    }
    return nvar;
}
unsafe extern "C" fn getpoints(
    mut numval: *mut *mut libc::c_double,
    mut nonum: libc::c_int,
    mut ctype: *mut libc::c_int,
    mut nobs: libc::c_int,
    mut point: *mut *mut libc::c_double,
) {
    let mut v: libc::c_int = 0;
    let mut width: libc::c_double = 0.;
    let mut p: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut excess: libc::c_double = 0.;
    v = 0 as libc::c_int;
    while v < nonum {
        qsort(
            *numval.offset(v as isize) as *mut libc::c_void,
            nobs as size_t,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> libc::c_int>,
                __compar_fn_t,
            >(cmp),
        );
        if *ctype.offset(v as isize) < 0 as libc::c_int {
            width = (*(*numval.offset(v as isize))
                .offset((nobs - 1 as libc::c_int) as isize)
                - *(*numval.offset(v as isize)).offset(0 as libc::c_int as isize))
                / -*ctype.offset(v as isize) as libc::c_double;
            p = 0 as libc::c_int;
            while p < -*ctype.offset(v as isize) {
                *(*point.offset(v as isize))
                    .offset(
                        p as isize,
                    ) = *(*numval.offset(v as isize)).offset(0 as libc::c_int as isize)
                    + p as libc::c_double * width;
                p += 1;
                p;
            }
            *(*point.offset(v as isize))
                .offset(
                    p as isize,
                ) = *(*numval.offset(v as isize))
                .offset((nobs - 1 as libc::c_int) as isize);
        } else if *ctype.offset(v as isize) > 0 as libc::c_int {
            p = 0 as libc::c_int;
            while p < *ctype.offset(v as isize) {
                index = floor(
                    (p * nobs) as libc::c_double
                        / *ctype.offset(v as isize) as libc::c_double,
                ) as libc::c_int;
                excess = (p * nobs) as libc::c_double
                    / *ctype.offset(v as isize) as libc::c_double
                    - index as libc::c_double;
                if excess > 0.0f64 && index < nobs - 1 as libc::c_int {
                    *(*point.offset(v as isize))
                        .offset(
                            p as isize,
                        ) = 0.5f64
                        * (*(*numval.offset(v as isize)).offset(index as isize)
                            + *(*numval.offset(v as isize))
                                .offset((index + 1 as libc::c_int) as isize));
                } else {
                    *(*point.offset(v as isize))
                        .offset(
                            p as isize,
                        ) = *(*numval.offset(v as isize))
                        .offset(
                            rint(
                                (p * nobs) as libc::c_double
                                    / *ctype.offset(v as isize) as libc::c_double,
                            ) as libc::c_int as isize,
                        );
                }
                p += 1;
                p;
            }
            *(*point.offset(v as isize))
                .offset(
                    p as isize,
                ) = *(*numval.offset(v as isize))
                .offset((nobs - 1 as libc::c_int) as isize);
        }
        v += 1;
        v;
    }
}
pub unsafe extern "C" fn group(
    mut fname: *mut libc::c_char,
    mut varspec: *mut libc::c_char,
    mut marks: *mut libc::c_char,
) {
    let mut outname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut markv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nmark: libc::c_int = 0;
    let mut varv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ctype: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nummem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut numval: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut nvar: libc::c_int = 0;
    let mut number: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut grpname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut grpv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nobs: libc::c_int = 0;
    let mut nnan: libc::c_int = 0;
    let mut allgood: libc::c_int = 0;
    let mut ptmem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut point: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut dnobs: libc::c_double = 0.;
    let mut count: libc::c_double = 0.;
    let mut countinc: libc::c_double = 0.;
    let mut p: libc::c_int = 0;
    let mut more: libc::c_int = 0;
    if fname.is_null() {
        fputs(
            b"(group) No dataset name given.\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    outname = dap_malloc(
        (strlen(fname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_suffix(
        outname,
        fname,
        b".grp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    grpname = dap_malloc(
        dap_namelen + 3 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    markv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    varv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    ctype = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    inset(fname);
    nmark = dap_list(marks, markv, dap_maxvar);
    nvar = groupparse(varspec, varv, ctype);
    if nvar < 0 as libc::c_int {
        number = -(-nvar % 4 as libc::c_int);
        nvar = -(nvar - number) / 4 as libc::c_int;
    } else {
        number = 0 as libc::c_int;
    }
    grpv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(
                (if number != 0 { 1 as libc::c_int } else { nvar }) as libc::c_ulong,
            ) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    if number == 0 {
        strcpy(grpname, b"_\0" as *const u8 as *const libc::c_char);
        v = 0 as libc::c_int;
        while v < nvar {
            strcpy(
                grpname.offset(1 as libc::c_int as isize),
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(*varv.offset(v as isize) as isize),
            );
            *grpname.offset(dap_namelen as isize) = '\0' as i32 as libc::c_char;
            strcat(grpname, b" -1\0" as *const u8 as *const libc::c_char);
            *grpv.offset(v as isize) = dap_vd(grpname, 0 as libc::c_int);
            v += 1;
            v;
        }
    } else {
        *grpv
            .offset(
                0 as libc::c_int as isize,
            ) = dap_vd(
            b"_N_ -1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int,
        );
        *varv
            .offset(0 as libc::c_int as isize) = *grpv.offset(0 as libc::c_int as isize);
    }
    outset(outname, b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    nummem = 0 as *mut libc::c_double;
    numval = 0 as *mut *mut libc::c_double;
    if number == 0 {
        nummem = dap_malloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(nvar as libc::c_ulong)
                .wrapping_mul(dap_maxval as libc::c_ulong) as libc::c_int,
            b"dap_maxval\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_double;
        numval = dap_malloc(
            ((nvar * dap_maxval) as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong,
                ) as libc::c_int,
            b"dap_maxval\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut *mut libc::c_double;
        ptmem = dap_malloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(nvar as libc::c_ulong)
                .wrapping_mul((dap_maxbars + 1 as libc::c_int) as libc::c_ulong)
                as libc::c_int,
            b"dap_maxbars\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_double;
        point = dap_malloc(
            (::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
                .wrapping_mul(nvar as libc::c_ulong) as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut *mut libc::c_double;
        v = 0 as libc::c_int;
        while v < nvar {
            let ref mut fresh5 = *numval.offset(v as isize);
            *fresh5 = nummem.offset((v * dap_maxval) as isize);
            let ref mut fresh6 = *point.offset(v as isize);
            *fresh6 = ptmem.offset((v * (dap_maxbars + 1 as libc::c_int)) as isize);
            v += 1;
            v;
        }
    }
    dap_mark();
    nobs = 0 as libc::c_int;
    nnan = 0 as libc::c_int;
    more = 1 as libc::c_int;
    while more != 0 {
        more = step();
        if dap_newpart(markv, nmark) != 0 {
            if number == 0 {
                getpoints(numval, nvar, ctype, nobs, point);
            }
            dnobs = nobs as libc::c_double;
            dap_rewind();
            count = 1.0f64;
            countinc = 1.0f64;
            if nnan > 0 as libc::c_int {
                fprintf(
                    dap_log,
                    b"(group) %d NaNs\n\0" as *const u8 as *const libc::c_char,
                    nnan,
                );
            }
            if number != 0 {
                if *ctype.offset(0 as libc::c_int as isize) & 0x2 as libc::c_int != 0 {
                    count = 0.0f64;
                }
                if *ctype.offset(0 as libc::c_int as isize) & 0x1 as libc::c_int != 0 {
                    countinc = -1.0f64;
                    count = dnobs - 1.0f64 + count;
                }
            }
            while step() != 0 && dap_newpart(markv, nmark) == 0 {
                if number != 0 {
                    v = 1 as libc::c_int;
                    allgood = 1 as libc::c_int;
                    while v < nvar {
                        if finite(
                            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                                .do_dbl)
                                .offset(*varv.offset(v as isize) as isize),
                        ) == 0
                        {
                            allgood = 0 as libc::c_int;
                            break;
                        } else {
                            v += 1;
                            v;
                        }
                    }
                    if allgood != 0 {
                        match number {
                            -1 => {
                                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                                    .do_dbl)
                                    .offset(
                                        *grpv.offset(0 as libc::c_int as isize) as isize,
                                    ) = count;
                            }
                            -2 => {
                                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                                    .do_dbl)
                                    .offset(
                                        *grpv.offset(0 as libc::c_int as isize) as isize,
                                    ) = count / dnobs;
                            }
                            -3 => {
                                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                                    .do_dbl)
                                    .offset(
                                        *grpv.offset(0 as libc::c_int as isize) as isize,
                                    ) = 100.0f64 * count / dnobs;
                            }
                            _ => {}
                        }
                        count += countinc;
                    } else {
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_dbl)
                            .offset(
                                *grpv.offset(0 as libc::c_int as isize) as isize,
                            ) = 0.0f64 / 0.0f64;
                    }
                } else {
                    v = 0 as libc::c_int;
                    allgood = 1 as libc::c_int;
                    while v < nvar {
                        if finite(
                            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                                .do_dbl)
                                .offset(*varv.offset(v as isize) as isize),
                        ) == 0
                        {
                            allgood = 0 as libc::c_int;
                            break;
                        } else {
                            v += 1;
                            v;
                        }
                    }
                    if allgood != 0 {
                        v = 0 as libc::c_int;
                        while v < nvar {
                            p = 1 as libc::c_int;
                            while *((*dap_obs
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize))
                                .do_dbl)
                                .offset(*varv.offset(v as isize) as isize)
                                > *(*point.offset(v as isize)).offset(p as isize)
                            {
                                p += 1;
                                p;
                            }
                            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                                .do_dbl)
                                .offset(
                                    *grpv.offset(v as isize) as isize,
                                ) = p as libc::c_double;
                            v += 1;
                            v;
                        }
                    } else {
                        v = 0 as libc::c_int;
                        while v < nvar {
                            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                                .do_dbl)
                                .offset(
                                    *grpv.offset(v as isize) as isize,
                                ) = 0.0f64 / 0.0f64;
                            v += 1;
                            v;
                        }
                    }
                }
                output();
                dap_mark();
            }
            nobs = 0 as libc::c_int;
            nnan = 0 as libc::c_int;
        }
        if number != 0 {
            v = 1 as libc::c_int;
            allgood = 1 as libc::c_int;
            while v < nvar {
                if finite(
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                        .offset(*varv.offset(v as isize) as isize),
                ) == 0
                {
                    allgood = 0 as libc::c_int;
                    break;
                } else {
                    v += 1;
                    v;
                }
            }
            if allgood != 0 {
                nobs += 1;
                nobs;
            } else {
                nnan += 1;
                nnan;
            }
        } else if nobs < dap_maxval {
            v = 0 as libc::c_int;
            allgood = 1 as libc::c_int;
            while v < nvar {
                if finite(
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                        .offset(*varv.offset(v as isize) as isize),
                ) == 0
                {
                    allgood = 0 as libc::c_int;
                    break;
                } else {
                    *(*numval.offset(v as isize))
                        .offset(
                            nobs as isize,
                        ) = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                        .do_dbl)
                        .offset(*varv.offset(v as isize) as isize);
                    v += 1;
                    v;
                }
            }
            if allgood != 0 {
                nobs += 1;
                nobs;
            } else {
                nnan += 1;
                nnan;
            }
        } else {
            fputs(
                b"(group) too many data.\n\0" as *const u8 as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
    }
    if number == 0 {
        dap_free(
            nummem as *mut libc::c_void,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        dap_free(
            numval as *mut libc::c_void,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        dap_free(
            ptmem as *mut libc::c_void,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        dap_free(
            point as *mut libc::c_void,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    dap_free(
        outname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        grpname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        grpv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        markv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        varv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        ctype as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn freq1(
    mut varv: *mut libc::c_int,
    mut nvar: libc::c_int,
    mut count: libc::c_double,
    mut sumcount: libc::c_double,
    mut statv: *mut libc::c_int,
    mut typen: libc::c_int,
    mut celln: libc::c_int,
) {
    dap_swap();
    if *statv.offset(0 as libc::c_int as isize) != 0 {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typen as isize),
            b"COUNT\0" as *const u8 as *const libc::c_char,
        );
        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
            .offset(celln as isize) = count;
        output();
    }
    if *statv.offset(1 as libc::c_int as isize) != 0 {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typen as isize),
            b"PERCENT\0" as *const u8 as *const libc::c_char,
        );
        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
            .offset(celln as isize) = 100.0f64 * count / sumcount;
        output();
    }
    if *statv.offset(2 as libc::c_int as isize) != 0 {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typen as isize),
            b"FRACTION\0" as *const u8 as *const libc::c_char,
        );
        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
            .offset(celln as isize) = count / sumcount;
        output();
    }
    dap_swap();
}
unsafe extern "C" fn statparse(
    mut stats: *mut libc::c_char,
    mut statv: *mut libc::c_int,
) {
    let mut s: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut stat: *mut libc::c_char = 0 as *mut libc::c_char;
    stat = dap_malloc(
        dap_namelen + 1 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    s = 0 as libc::c_int;
    while s < 13 as libc::c_int {
        *statv.offset(s as isize) = 0 as libc::c_int;
        s += 1;
        s;
    }
    s = 0 as libc::c_int;
    while *stats.offset(s as isize) as libc::c_int == ' ' as i32 {
        s += 1;
        s;
    }
    while *stats.offset(s as isize) != 0 {
        i = 0 as libc::c_int;
        while *stats.offset((s + i) as isize) as libc::c_int != 0
            && *stats.offset((s + i) as isize) as libc::c_int != ' ' as i32
        {
            if i < dap_namelen {
                *stat.offset(i as isize) = *stats.offset((s + i) as isize);
            } else {
                *stat.offset(i as isize) = '\0' as i32 as libc::c_char;
                fprintf(
                    dap_err,
                    b"(statparse) Statistic name too long: %s\n\0" as *const u8
                        as *const libc::c_char,
                    stat,
                );
                exit(1 as libc::c_int);
            }
            i += 1;
            i;
        }
        *stat.offset(i as isize) = '\0' as i32 as libc::c_char;
        s += i;
        if strcmp(stat, b"COUNT\0" as *const u8 as *const libc::c_char) == 0 {
            *statv.offset(0 as libc::c_int as isize) = 1 as libc::c_int;
        } else if strcmp(stat, b"PERCENT\0" as *const u8 as *const libc::c_char) == 0 {
            *statv.offset(1 as libc::c_int as isize) = 1 as libc::c_int;
        } else if strcmp(stat, b"ROWPERC\0" as *const u8 as *const libc::c_char) == 0 {
            *statv.offset(9 as libc::c_int as isize) = 1 as libc::c_int;
        } else if strcmp(stat, b"COLPERC\0" as *const u8 as *const libc::c_char) == 0 {
            *statv.offset(10 as libc::c_int as isize) = 1 as libc::c_int;
        } else if strcmp(stat, b"FRACTION\0" as *const u8 as *const libc::c_char) == 0 {
            *statv.offset(2 as libc::c_int as isize) = 1 as libc::c_int;
        } else if strcmp(stat, b"EXPECTED\0" as *const u8 as *const libc::c_char) == 0 {
            *statv.offset(3 as libc::c_int as isize) = 1 as libc::c_int;
        } else if strcmp(stat, b"CHISQ\0" as *const u8 as *const libc::c_char) == 0 {
            *statv.offset(4 as libc::c_int as isize) = 1 as libc::c_int;
        } else if strcmp(stat, b"ODDSRAT\0" as *const u8 as *const libc::c_char) == 0 {
            *statv.offset(5 as libc::c_int as isize) = 1 as libc::c_int;
        } else if strcmp(stat, b"ORDINAL\0" as *const u8 as *const libc::c_char) == 0 {
            *statv.offset(6 as libc::c_int as isize) = 1 as libc::c_int;
        } else if strcmp(stat, b"FISHER\0" as *const u8 as *const libc::c_char) == 0 {
            *statv.offset(7 as libc::c_int as isize) = 1 as libc::c_int;
        } else if strcmp(stat, b"CMH\0" as *const u8 as *const libc::c_char) == 0 {
            *statv.offset(8 as libc::c_int as isize) = 1 as libc::c_int;
        } else if strcmp(stat, b"PAIR\0" as *const u8 as *const libc::c_char) == 0 {
            *statv.offset(11 as libc::c_int as isize) = 1 as libc::c_int;
        } else if strcmp(stat, b"NOMINAL\0" as *const u8 as *const libc::c_char) == 0 {
            *statv.offset(12 as libc::c_int as isize) = 1 as libc::c_int;
        } else {
            fprintf(
                dap_err,
                b"(statparse) Invalid statistic name: %s\n\0" as *const u8
                    as *const libc::c_char,
                stat,
            );
            exit(1 as libc::c_int);
        }
        while *stats.offset(s as isize) as libc::c_int == ' ' as i32 {
            s += 1;
            s;
        }
    }
    dap_free(
        stat as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn findlev(
    mut v: libc::c_int,
    mut level: *mut *mut libc::c_char,
    mut nlevels: *mut libc::c_int,
) -> libc::c_int {
    let mut l: libc::c_int = 0;
    static mut str: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if str.is_null() {
        str = dap_malloc(
            21 as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
        .offset(v as isize) > 0 as libc::c_int
    {
        s = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
            .offset(v as isize);
    } else {
        s = str;
        if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
            .offset(v as isize) == 0 as libc::c_int
        {
            sprintf(
                str,
                b"%d\0" as *const u8 as *const libc::c_char,
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_int)
                    .offset(v as isize),
            );
        } else {
            sprintf(
                str,
                b"%g\0" as *const u8 as *const libc::c_char,
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(v as isize),
            );
        }
    }
    l = 0 as libc::c_int;
    while l < *nlevels {
        if strcmp(s, *level.offset(l as isize)) == 0 {
            return l;
        }
        l += 1;
        l;
    }
    if *nlevels < dap_maxlev {
        strcpy(*level.offset(l as isize), s);
        *nlevels += 1;
        *nlevels;
        return l;
    } else {
        fprintf(
            dap_err,
            b"(findlev) Too many levels at: %s\n\0" as *const u8 as *const libc::c_char,
            s,
        );
        exit(1 as libc::c_int);
    };
}
unsafe extern "C" fn tabentry(
    mut varv: *mut libc::c_int,
    mut tab: *mut *mut libc::c_double,
    mut level: *mut *mut *mut libc::c_char,
    mut nlevels: *mut libc::c_int,
    mut count: libc::c_double,
) {
    dap_swap();
    *(*tab
        .offset(
            findlev(
                *varv.offset(0 as libc::c_int as isize),
                *level.offset(0 as libc::c_int as isize),
                nlevels,
            ) as isize,
        ))
        .offset(
            findlev(
                *varv.offset(1 as libc::c_int as isize),
                *level.offset(1 as libc::c_int as isize),
                nlevels.offset(1 as libc::c_int as isize),
            ) as isize,
        ) = count;
    dap_swap();
}
unsafe extern "C" fn valcpy(mut v: libc::c_int, mut val: *mut libc::c_char) {
    if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
        .offset(v as isize) > 0 as libc::c_int
    {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(v as isize),
            val,
        );
    } else if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
        .offset(v as isize) == 0 as libc::c_int
    {
        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_int)
            .offset(v as isize) = atoi(val);
    } else {
        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
            .offset(v as isize) = atof(val);
    };
}
unsafe extern "C" fn freq2(
    mut tab: *mut *mut libc::c_double,
    mut level: *mut *mut *mut libc::c_char,
    mut nlevels: *mut libc::c_int,
    mut statv: *mut libc::c_int,
    mut markv: *mut libc::c_int,
    mut nmark: libc::c_int,
    mut varv: *mut libc::c_int,
    mut typen: libc::c_int,
    mut celln: libc::c_int,
) {
    let mut v: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut expmem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut expect: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut rowsum: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut colsum: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut sum: libc::c_double = 0.;
    let mut amem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut a: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut dmem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut d: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut p: libc::c_double = 0.;
    let mut q: libc::c_double = 0.;
    let mut tmp1: libc::c_double = 0.;
    let mut tmp2: libc::c_double = 0.;
    let mut tmp3: libc::c_double = 0.;
    let mut tmp4: libc::c_double = 0.;
    let mut w: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut var: libc::c_double = 0.;
    let mut r: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut diff: libc::c_double = 0.;
    let mut chisq: libc::c_double = 0.;
    let mut rr: libc::c_int = 0;
    let mut cc: libc::c_int = 0;
    let mut upleft: libc::c_double = 0.;
    let mut dnleft: libc::c_double = 0.;
    let mut denom: libc::c_double = 0.;
    let mut prob: libc::c_double = 0.;
    let mut oneprob: libc::c_double = 0.;
    let mut othprob: libc::c_double = 0.;
    let mut hx: libc::c_double = 0.;
    let mut hy: libc::c_double = 0.;
    let mut hxy: libc::c_double = 0.;
    let mut uv: libc::c_double = 0.;
    sum = 0.0f64;
    oneprob = 0.0f64;
    expmem = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxlev as libc::c_ulong)
            .wrapping_mul(dap_maxlev as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    expect = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxlev as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_double;
    l = 0 as libc::c_int;
    while l < dap_maxlev {
        let ref mut fresh7 = *expect.offset(l as isize);
        *fresh7 = expmem.offset((l * dap_maxlev) as isize);
        l += 1;
        l;
    }
    rowsum = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxlev as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    colsum = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxlev as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    amem = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxlev as libc::c_ulong)
            .wrapping_mul(dap_maxlev as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    dmem = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxlev as libc::c_ulong)
            .wrapping_mul(dap_maxlev as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    a = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxlev as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_double;
    d = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxlev as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_double;
    l = 0 as libc::c_int;
    while l < dap_maxlev {
        let ref mut fresh8 = *a.offset(l as isize);
        *fresh8 = amem.offset((l * dap_maxlev) as isize);
        let ref mut fresh9 = *d.offset(l as isize);
        *fresh9 = dmem.offset((l * dap_maxlev) as isize);
        l += 1;
        l;
    }
    dap_swap();
    if *statv.offset(4 as libc::c_int as isize) != 0
        || *statv.offset(5 as libc::c_int as isize) != 0
        || *statv.offset(6 as libc::c_int as isize) != 0
        || *statv.offset(7 as libc::c_int as isize) != 0
        || *statv.offset(11 as libc::c_int as isize) != 0
        || *statv.offset(12 as libc::c_int as isize) != 0
    {
        dap_head(markv, nmark);
        fputs(b"Variable: Levels\n\0" as *const u8 as *const libc::c_char, dap_lst);
        fputs(b"----------------\n\0" as *const u8 as *const libc::c_char, dap_lst);
        v = 0 as libc::c_int;
        while v < 2 as libc::c_int {
            fprintf(
                dap_lst,
                b"%s:\0" as *const u8 as *const libc::c_char,
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(*varv.offset(v as isize) as isize),
            );
            l = 0 as libc::c_int;
            while l < *nlevels.offset(v as isize) {
                fprintf(
                    dap_lst,
                    b" %s\0" as *const u8 as *const libc::c_char,
                    *(*level.offset(v as isize)).offset(l as isize),
                );
                l += 1;
                l;
            }
            putc('\n' as i32, dap_lst);
            v += 1;
            v;
        }
        putc('\n' as i32, dap_lst);
    }
    if *statv.offset(3 as libc::c_int as isize) != 0
        || *statv.offset(4 as libc::c_int as isize) != 0
        || *statv.offset(6 as libc::c_int as isize) != 0
        || *statv.offset(7 as libc::c_int as isize) != 0
        || *statv.offset(9 as libc::c_int as isize) != 0
        || *statv.offset(10 as libc::c_int as isize) != 0
        || *statv.offset(11 as libc::c_int as isize) != 0
        || *statv.offset(12 as libc::c_int as isize) != 0
    {
        r = 0 as libc::c_int;
        sum = 0.0f64;
        while r < *nlevels.offset(0 as libc::c_int as isize) {
            *rowsum.offset(r as isize) = 0.0f64;
            c = 0 as libc::c_int;
            while c < *nlevels.offset(1 as libc::c_int as isize) {
                *rowsum.offset(r as isize)
                    += *(*tab.offset(r as isize)).offset(c as isize);
                c += 1;
                c;
            }
            sum += *rowsum.offset(r as isize);
            r += 1;
            r;
        }
        c = 0 as libc::c_int;
        while c < *nlevels.offset(1 as libc::c_int as isize) {
            *colsum.offset(c as isize) = 0.0f64;
            r = 0 as libc::c_int;
            while r < *nlevels.offset(0 as libc::c_int as isize) {
                *colsum.offset(c as isize)
                    += *(*tab.offset(r as isize)).offset(c as isize);
                r += 1;
                r;
            }
            c += 1;
            c;
        }
        r = 0 as libc::c_int;
        while r < *nlevels.offset(0 as libc::c_int as isize) {
            c = 0 as libc::c_int;
            while c < *nlevels.offset(1 as libc::c_int as isize) {
                *(*expect.offset(r as isize))
                    .offset(
                        c as isize,
                    ) = *rowsum.offset(r as isize) * *colsum.offset(c as isize) / sum;
                c += 1;
                c;
            }
            r += 1;
            r;
        }
    }
    if *statv.offset(3 as libc::c_int as isize) != 0 {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typen as isize),
            b"EXPECTED\0" as *const u8 as *const libc::c_char,
        );
        r = 0 as libc::c_int;
        while r < *nlevels.offset(0 as libc::c_int as isize) {
            valcpy(
                *varv.offset(0 as libc::c_int as isize),
                *(*level.offset(0 as libc::c_int as isize)).offset(r as isize),
            );
            c = 0 as libc::c_int;
            while c < *nlevels.offset(1 as libc::c_int as isize) {
                valcpy(
                    *varv.offset(1 as libc::c_int as isize),
                    *(*level.offset(1 as libc::c_int as isize)).offset(c as isize),
                );
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        celln as isize,
                    ) = *(*expect.offset(r as isize)).offset(c as isize);
                output();
                c += 1;
                c;
            }
            r += 1;
            r;
        }
    }
    if *statv.offset(9 as libc::c_int as isize) != 0 {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typen as isize),
            b"ROWPERC\0" as *const u8 as *const libc::c_char,
        );
        r = 0 as libc::c_int;
        while r < *nlevels.offset(0 as libc::c_int as isize) {
            valcpy(
                *varv.offset(0 as libc::c_int as isize),
                *(*level.offset(0 as libc::c_int as isize)).offset(r as isize),
            );
            c = 0 as libc::c_int;
            while c < *nlevels.offset(1 as libc::c_int as isize) {
                valcpy(
                    *varv.offset(1 as libc::c_int as isize),
                    *(*level.offset(1 as libc::c_int as isize)).offset(c as isize),
                );
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        celln as isize,
                    ) = 100.0f64 * *(*tab.offset(r as isize)).offset(c as isize)
                    / *rowsum.offset(r as isize);
                output();
                c += 1;
                c;
            }
            r += 1;
            r;
        }
    }
    if *statv.offset(10 as libc::c_int as isize) != 0 {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typen as isize),
            b"COLPERC\0" as *const u8 as *const libc::c_char,
        );
        r = 0 as libc::c_int;
        while r < *nlevels.offset(0 as libc::c_int as isize) {
            valcpy(
                *varv.offset(0 as libc::c_int as isize),
                *(*level.offset(0 as libc::c_int as isize)).offset(r as isize),
            );
            c = 0 as libc::c_int;
            while c < *nlevels.offset(1 as libc::c_int as isize) {
                valcpy(
                    *varv.offset(1 as libc::c_int as isize),
                    *(*level.offset(1 as libc::c_int as isize)).offset(c as isize),
                );
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        celln as isize,
                    ) = 100.0f64 * *(*tab.offset(r as isize)).offset(c as isize)
                    / *colsum.offset(c as isize);
                output();
                c += 1;
                c;
            }
            r += 1;
            r;
        }
    }
    if *statv.offset(4 as libc::c_int as isize) != 0 {
        r = 0 as libc::c_int;
        chisq = 0.0f64;
        while r < *nlevels.offset(0 as libc::c_int as isize) {
            c = 0 as libc::c_int;
            while c < *nlevels.offset(1 as libc::c_int as isize) {
                diff = *(*tab.offset(r as isize)).offset(c as isize)
                    - *(*expect.offset(r as isize)).offset(c as isize);
                chisq += diff * diff / *(*expect.offset(r as isize)).offset(c as isize);
                c += 1;
                c;
            }
            r += 1;
            r;
        }
        fprintf(
            dap_lst,
            b"Chisq0[%d] = %g, Prob[Chisq > Chisq0] = %.5f\n\0" as *const u8
                as *const libc::c_char,
            (*nlevels.offset(0 as libc::c_int as isize) - 1 as libc::c_int)
                * (*nlevels.offset(1 as libc::c_int as isize) - 1 as libc::c_int),
            chisq,
            ceil(
                100000.0f64
                    * probchisq(
                        chisq,
                        (*nlevels.offset(0 as libc::c_int as isize) - 1 as libc::c_int)
                            * (*nlevels.offset(1 as libc::c_int as isize)
                                - 1 as libc::c_int),
                    ),
            ) / 100000.0f64,
        );
    }
    if *statv.offset(5 as libc::c_int as isize) != 0 {
        if *nlevels.offset(0 as libc::c_int as isize) == 2 as libc::c_int
            && *nlevels.offset(1 as libc::c_int as isize) == 2 as libc::c_int
        {
            tmp1 = (*(*tab.offset(0 as libc::c_int as isize))
                .offset(0 as libc::c_int as isize) + 0.5f64)
                * (*(*tab.offset(1 as libc::c_int as isize))
                    .offset(1 as libc::c_int as isize) + 0.5f64)
                / ((*(*tab.offset(0 as libc::c_int as isize))
                    .offset(1 as libc::c_int as isize) + 0.5f64)
                    * (*(*tab.offset(1 as libc::c_int as isize))
                        .offset(0 as libc::c_int as isize) + 0.5f64));
            fprintf(
                dap_lst,
                b"Odds ratio = %g\nlog(Odds ratio) = %g, ASE = %g\n\0" as *const u8
                    as *const libc::c_char,
                tmp1,
                log(tmp1),
                sqrt(
                    1.0f64
                        / (*(*tab.offset(0 as libc::c_int as isize))
                            .offset(0 as libc::c_int as isize) + 0.5f64)
                        + 1.0f64
                            / (*(*tab.offset(1 as libc::c_int as isize))
                                .offset(0 as libc::c_int as isize) + 0.5f64)
                        + 1.0f64
                            / (*(*tab.offset(0 as libc::c_int as isize))
                                .offset(1 as libc::c_int as isize) + 0.5f64)
                        + 1.0f64
                            / (*(*tab.offset(1 as libc::c_int as isize))
                                .offset(1 as libc::c_int as isize) + 0.5f64),
                ),
            );
        } else {
            fputs(
                b"(freq2) Odds ratio computed for 2 x 2 tables only.\n\0" as *const u8
                    as *const libc::c_char,
                dap_log,
            );
        }
    }
    if *statv.offset(7 as libc::c_int as isize) != 0 {
        if *nlevels.offset(0 as libc::c_int as isize) == 2 as libc::c_int
            && *nlevels.offset(1 as libc::c_int as isize) == 2 as libc::c_int
        {
            if *(*tab.offset(0 as libc::c_int as isize))
                .offset(0 as libc::c_int as isize)
                >= *(*expect.offset(0 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize)
            {
                denom = dap_bincoeff(sum, *colsum.offset(0 as libc::c_int as isize));
                upleft = *(*tab.offset(0 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize);
                dnleft = *(*tab.offset(1 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize);
                prob = 0.0f64;
                while upleft <= *rowsum.offset(0 as libc::c_int as isize)
                    && upleft <= *colsum.offset(0 as libc::c_int as isize)
                {
                    if prob == 0.0f64 {
                        oneprob = dap_bincoeff(
                            *rowsum.offset(0 as libc::c_int as isize),
                            upleft,
                        )
                            * dap_bincoeff(
                                *rowsum.offset(1 as libc::c_int as isize),
                                dnleft,
                            );
                        prob = oneprob;
                    } else {
                        prob
                            += dap_bincoeff(
                                *rowsum.offset(0 as libc::c_int as isize),
                                upleft,
                            )
                                * dap_bincoeff(
                                    *rowsum.offset(1 as libc::c_int as isize),
                                    dnleft,
                                );
                    }
                    upleft += 1.0f64;
                    dnleft -= 1.0f64;
                }
                fprintf(
                    dap_lst,
                    b"Fisher's exact test: right     %g\n\0" as *const u8
                        as *const libc::c_char,
                    prob / denom,
                );
                upleft = ceil(
                    *(*expect.offset(1 as libc::c_int as isize))
                        .offset(0 as libc::c_int as isize),
                );
                dnleft = *colsum.offset(0 as libc::c_int as isize) - upleft;
                while upleft <= *rowsum.offset(1 as libc::c_int as isize)
                    && upleft <= *colsum.offset(0 as libc::c_int as isize)
                {
                    othprob = dap_bincoeff(
                        *rowsum.offset(1 as libc::c_int as isize),
                        upleft,
                    ) * dap_bincoeff(*rowsum.offset(0 as libc::c_int as isize), dnleft);
                    if othprob <= oneprob {
                        prob += othprob;
                    }
                    upleft += 1.0f64;
                    dnleft -= 1.0f64;
                }
                if *(*tab.offset(0 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize)
                    == *(*expect.offset(0 as libc::c_int as isize))
                        .offset(0 as libc::c_int as isize)
                {
                    prob = 1.0f64;
                }
                fprintf(
                    dap_lst,
                    b"                     2-tailed  %g\n\0" as *const u8
                        as *const libc::c_char,
                    prob / denom,
                );
            }
            if *(*tab.offset(0 as libc::c_int as isize))
                .offset(0 as libc::c_int as isize)
                <= *(*expect.offset(0 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize)
            {
                denom = dap_bincoeff(sum, *colsum.offset(1 as libc::c_int as isize));
                upleft = *(*tab.offset(1 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize);
                dnleft = *(*tab.offset(0 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize);
                prob = 0.0f64;
                while upleft <= *rowsum.offset(1 as libc::c_int as isize)
                    && upleft <= *colsum.offset(0 as libc::c_int as isize)
                {
                    if prob == 0.0f64 {
                        oneprob = dap_bincoeff(
                            *rowsum.offset(1 as libc::c_int as isize),
                            upleft,
                        )
                            * dap_bincoeff(
                                *rowsum.offset(0 as libc::c_int as isize),
                                dnleft,
                            );
                        prob = oneprob;
                    } else {
                        prob
                            += dap_bincoeff(
                                *rowsum.offset(1 as libc::c_int as isize),
                                upleft,
                            )
                                * dap_bincoeff(
                                    *rowsum.offset(0 as libc::c_int as isize),
                                    dnleft,
                                );
                    }
                    upleft += 1.0f64;
                    dnleft -= 1.0f64;
                }
                fprintf(
                    dap_lst,
                    b"Fisher's exact test: left      %g\n\0" as *const u8
                        as *const libc::c_char,
                    prob / denom,
                );
                upleft = ceil(
                    *(*expect.offset(0 as libc::c_int as isize))
                        .offset(0 as libc::c_int as isize),
                );
                dnleft = *colsum.offset(0 as libc::c_int as isize) - upleft;
                while upleft <= *rowsum.offset(0 as libc::c_int as isize)
                    && upleft <= *colsum.offset(0 as libc::c_int as isize)
                {
                    othprob = dap_bincoeff(
                        *rowsum.offset(0 as libc::c_int as isize),
                        upleft,
                    ) * dap_bincoeff(*rowsum.offset(1 as libc::c_int as isize), dnleft);
                    if othprob <= oneprob {
                        prob += othprob;
                    }
                    upleft += 1.0f64;
                    dnleft -= 1.0f64;
                }
                if *(*tab.offset(0 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize)
                    == *(*expect.offset(0 as libc::c_int as isize))
                        .offset(0 as libc::c_int as isize)
                {
                    prob = 1.0f64;
                }
                fprintf(
                    dap_lst,
                    b"                     2-tailed  %g\n\0" as *const u8
                        as *const libc::c_char,
                    prob / denom,
                );
            }
        } else {
            fputs(
                b"(freq2) Fisher's exact test computed for 2 x 2 tables only.\n\0"
                    as *const u8 as *const libc::c_char,
                dap_log,
            );
        }
    }
    if *statv.offset(6 as libc::c_int as isize) != 0 {
        r = 0 as libc::c_int;
        while r < *nlevels.offset(0 as libc::c_int as isize) {
            c = 0 as libc::c_int;
            while c < *nlevels.offset(1 as libc::c_int as isize) {
                *(*a.offset(r as isize)).offset(c as isize) = 0.0f64;
                *(*d.offset(r as isize)).offset(c as isize) = 0.0f64;
                rr = 0 as libc::c_int;
                while rr < *nlevels.offset(0 as libc::c_int as isize) {
                    cc = 0 as libc::c_int;
                    while cc < *nlevels.offset(1 as libc::c_int as isize) {
                        if rr < r && cc < c || rr > r && cc > c {
                            *(*a.offset(r as isize)).offset(c as isize)
                                += *(*tab.offset(rr as isize)).offset(cc as isize);
                        } else if rr < r && cc > c || rr > r && cc < c {
                            *(*d.offset(r as isize)).offset(c as isize)
                                += *(*tab.offset(rr as isize)).offset(cc as isize);
                        }
                        cc += 1;
                        cc;
                    }
                    rr += 1;
                    rr;
                }
                c += 1;
                c;
            }
            r += 1;
            r;
        }
        r = 0 as libc::c_int;
        p = 0.0f64;
        q = 0.0f64;
        while r < *nlevels.offset(0 as libc::c_int as isize) {
            c = 0 as libc::c_int;
            while c < *nlevels.offset(1 as libc::c_int as isize) {
                p
                    += *(*tab.offset(r as isize)).offset(c as isize)
                        * *(*a.offset(r as isize)).offset(c as isize);
                q
                    += *(*tab.offset(r as isize)).offset(c as isize)
                        * *(*d.offset(r as isize)).offset(c as isize);
                c += 1;
                c;
            }
            r += 1;
            r;
        }
        fprintf(
            dap_lst,
            b"Statistic          Value   ASE\n\0" as *const u8 as *const libc::c_char,
        );
        r = 0 as libc::c_int;
        var = 0.0f64;
        while r < *nlevels.offset(0 as libc::c_int as isize) {
            c = 0 as libc::c_int;
            while c < *nlevels.offset(1 as libc::c_int as isize) {
                tmp1 = q * *(*a.offset(r as isize)).offset(c as isize)
                    - p * *(*d.offset(r as isize)).offset(c as isize);
                var += *(*tab.offset(r as isize)).offset(c as isize) * tmp1 * tmp1;
                c += 1;
                c;
            }
            r += 1;
            r;
        }
        tmp2 = p + q;
        tmp2 *= tmp2;
        tmp2 *= tmp2;
        var *= 16.0f64 / tmp2;
        fprintf(
            dap_lst,
            b"Gamma             %6.3f  %5.3f\n\0" as *const u8 as *const libc::c_char,
            (p - q) / (p + q),
            sqrt(var),
        );
        r = 0 as libc::c_int;
        tmp1 = sum * sum;
        while r < *nlevels.offset(0 as libc::c_int as isize) {
            tmp1 -= *rowsum.offset(r as isize) * *rowsum.offset(r as isize);
            r += 1;
            r;
        }
        c = 0 as libc::c_int;
        tmp2 = sum * sum;
        while c < *nlevels.offset(1 as libc::c_int as isize) {
            tmp2 -= *colsum.offset(c as isize) * *colsum.offset(c as isize);
            c += 1;
            c;
        }
        w = sqrt(tmp1 * tmp2);
        t = (p - q) / w;
        r = 0 as libc::c_int;
        var = 0.0f64;
        while r < *nlevels.offset(0 as libc::c_int as isize) {
            c = 0 as libc::c_int;
            while c < *nlevels.offset(1 as libc::c_int as isize) {
                tmp3 = 2.0f64 * w
                    * (*(*a.offset(r as isize)).offset(c as isize)
                        - *(*d.offset(r as isize)).offset(c as isize))
                    + t
                        * (*rowsum.offset(r as isize) * tmp2
                            + *colsum.offset(c as isize) * tmp1);
                var += *(*tab.offset(r as isize)).offset(c as isize) * tmp3 * tmp3;
                c += 1;
                c;
            }
            r += 1;
            r;
        }
        tmp4 = tmp1 + tmp2;
        var = (var - sum * sum * sum * t * t * tmp4 * tmp4) / (w * w * w * w);
        fprintf(
            dap_lst,
            b"Kendall's Tau-b   %6.3f  %5.3f\n\0" as *const u8 as *const libc::c_char,
            t,
            sqrt(var),
        );
        r = 0 as libc::c_int;
        var = 0.0f64;
        while r < *nlevels.offset(0 as libc::c_int as isize) {
            c = 0 as libc::c_int;
            while c < *nlevels.offset(1 as libc::c_int as isize) {
                tmp3 = tmp1
                    * (*(*a.offset(r as isize)).offset(c as isize)
                        - *(*d.offset(r as isize)).offset(c as isize))
                    - (p - q) * (sum - *rowsum.offset(r as isize));
                var += *(*tab.offset(r as isize)).offset(c as isize) * tmp3 * tmp3;
                c += 1;
                c;
            }
            r += 1;
            r;
        }
        var *= 4.0f64 / (tmp1 * tmp1 * tmp1 * tmp1);
        fprintf(
            dap_lst,
            b"Somers' D C|R     %6.3f  %5.3f\n\0" as *const u8 as *const libc::c_char,
            (p - q) / tmp1,
            sqrt(var),
        );
        r = 0 as libc::c_int;
        var = 0.0f64;
        while r < *nlevels.offset(0 as libc::c_int as isize) {
            c = 0 as libc::c_int;
            while c < *nlevels.offset(1 as libc::c_int as isize) {
                tmp3 = tmp2
                    * (*(*a.offset(r as isize)).offset(c as isize)
                        - *(*d.offset(r as isize)).offset(c as isize))
                    - (p - q) * (sum - *colsum.offset(c as isize));
                var += *(*tab.offset(r as isize)).offset(c as isize) * tmp3 * tmp3;
                c += 1;
                c;
            }
            r += 1;
            r;
        }
        var *= 4.0f64 / (tmp2 * tmp2 * tmp2 * tmp2);
        fprintf(
            dap_lst,
            b"Somers' D R|C     %6.3f  %5.3f\n\0" as *const u8 as *const libc::c_char,
            (p - q) / tmp2,
            sqrt(var),
        );
    }
    if *statv.offset(11 as libc::c_int as isize) != 0 {
        if *nlevels.offset(0 as libc::c_int as isize)
            != *nlevels.offset(1 as libc::c_int as isize)
        {
            fprintf(
                dap_err,
                b"(freq2) PAIR requires square table, table is %d x %d.\n\0" as *const u8
                    as *const libc::c_char,
                *nlevels.offset(0 as libc::c_int as isize),
                *nlevels.offset(1 as libc::c_int as isize),
            );
            exit(1 as libc::c_int);
        }
        r = 0 as libc::c_int;
        p = 0.0f64;
        q = 0.0f64;
        tmp1 = 0.0f64;
        tmp2 = 0.0f64;
        while r < *nlevels.offset(0 as libc::c_int as isize) {
            p += *(*tab.offset(r as isize)).offset(r as isize);
            q += *(*expect.offset(r as isize)).offset(r as isize);
            tmp1
                += *(*tab.offset(r as isize)).offset(r as isize)
                    * (*rowsum.offset(r as isize) + *colsum.offset(r as isize));
            c = 0 as libc::c_int;
            while c < *nlevels.offset(0 as libc::c_int as isize) {
                tmp3 = *rowsum.offset(c as isize) + *colsum.offset(r as isize);
                tmp2 += *(*tab.offset(r as isize)).offset(c as isize) * tmp3 * tmp3;
                c += 1;
                c;
            }
            r += 1;
            r;
        }
        p /= sum;
        q /= sum;
        tmp1 /= sum * sum;
        tmp2 /= sum * sum * sum;
        tmp3 = 1.0f64 - p;
        tmp4 = 1.0f64 - q;
        fprintf(
            dap_lst,
            b"Statistic          Value   ASE\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            dap_lst,
            b"Kappa             %6.3f  %5.3f\n\0" as *const u8 as *const libc::c_char,
            (p - q) / tmp4,
            sqrt(
                (p * tmp3 / (tmp4 * tmp4)
                    + 2.0f64 * tmp3 * (2.0f64 * p * q - tmp1) / (tmp4 * tmp4 * tmp4)
                    + tmp3 * tmp3 * (tmp2 - 4.0f64 * q * q)
                        / (tmp4 * tmp4 * tmp4 * tmp4)) / sum,
            ),
        );
    }
    if *statv.offset(12 as libc::c_int as isize) != 0 {
        hx = 0.0f64;
        r = 0 as libc::c_int;
        while r < *nlevels.offset(0 as libc::c_int as isize) {
            hx -= *rowsum.offset(r as isize) * log(*rowsum.offset(r as isize) / sum);
            r += 1;
            r;
        }
        hx /= sum;
        hy = 0.0f64;
        c = 0 as libc::c_int;
        while c < *nlevels.offset(1 as libc::c_int as isize) {
            hy -= *colsum.offset(c as isize) * log(*colsum.offset(c as isize) / sum);
            c += 1;
            c;
        }
        hy /= sum;
        hxy = 0.0f64;
        r = 0 as libc::c_int;
        while r < *nlevels.offset(0 as libc::c_int as isize) {
            c = 0 as libc::c_int;
            while c < *nlevels.offset(1 as libc::c_int as isize) {
                hxy
                    -= *(*tab.offset(r as isize)).offset(c as isize)
                        * log(*(*tab.offset(r as isize)).offset(c as isize) / sum);
                c += 1;
                c;
            }
            r += 1;
            r;
        }
        hxy /= sum;
        uv = hx + hy - hxy;
        fprintf(
            dap_lst,
            b"Statistic              Value   ASE\n\0" as *const u8 as *const libc::c_char,
        );
        var = 0.0f64;
        r = 0 as libc::c_int;
        while r < *nlevels.offset(0 as libc::c_int as isize) {
            c = 0 as libc::c_int;
            while c < *nlevels.offset(1 as libc::c_int as isize) {
                tmp1 = hy
                    * log(
                        *(*tab.offset(r as isize)).offset(c as isize)
                            / *rowsum.offset(r as isize),
                    ) + (hx - hxy) * log(*colsum.offset(c as isize) / sum);
                var += *(*tab.offset(r as isize)).offset(c as isize) * tmp1 * tmp1;
                c += 1;
                c;
            }
            r += 1;
            r;
        }
        var = sqrt(var) / (sum * hy * hy);
        fprintf(
            dap_lst,
            b"Uncertainty C|R       %6.3f  %5.3f\n\0" as *const u8
                as *const libc::c_char,
            uv / hy,
            var,
        );
        var = 0.0f64;
        r = 0 as libc::c_int;
        while r < *nlevels.offset(0 as libc::c_int as isize) {
            c = 0 as libc::c_int;
            while c < *nlevels.offset(1 as libc::c_int as isize) {
                tmp1 = hx
                    * log(
                        *(*tab.offset(r as isize)).offset(c as isize)
                            / *colsum.offset(c as isize),
                    ) + (hy - hxy) * log(*rowsum.offset(r as isize) / sum);
                var += *(*tab.offset(r as isize)).offset(c as isize) * tmp1 * tmp1;
                c += 1;
                c;
            }
            r += 1;
            r;
        }
        var = sqrt(var) / (sum * hx * hx);
        fprintf(
            dap_lst,
            b"Uncertainty R|C       %6.3f  %5.3f\n\0" as *const u8
                as *const libc::c_char,
            uv / hx,
            var,
        );
        var = 0.0f64;
        r = 0 as libc::c_int;
        while r < *nlevels.offset(0 as libc::c_int as isize) {
            c = 0 as libc::c_int;
            while c < *nlevels.offset(1 as libc::c_int as isize) {
                tmp1 = hxy
                    * log(
                        *rowsum.offset(r as isize) * *colsum.offset(c as isize)
                            / (sum * sum),
                    )
                    - (hx + hy)
                        * log(*(*tab.offset(r as isize)).offset(c as isize) / sum);
                var += *(*tab.offset(r as isize)).offset(c as isize) * tmp1 * tmp1;
                c += 1;
                c;
            }
            r += 1;
            r;
        }
        tmp2 = hx + hy;
        var = 2.0f64 * sqrt(var) / (sum * tmp2 * tmp2);
        fprintf(
            dap_lst,
            b"Uncertainty Symmetric %6.3f  %5.3f\n\0" as *const u8
                as *const libc::c_char,
            2.0f64 * uv / (hx + hy),
            var,
        );
    }
    dap_swap();
    dap_free(
        expmem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        expect as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        rowsum as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        colsum as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        amem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        a as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        dmem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        d as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn freqparse(
    mut varlist: *mut libc::c_char,
    mut varv: *mut libc::c_int,
    mut wt: *mut libc::c_int,
) -> libc::c_int {
    let mut nvars: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut mname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut wtvar: libc::c_int = 0;
    *wt.offset(0 as libc::c_int as isize) = -(1 as libc::c_int);
    if varlist.is_null() {
        return 0 as libc::c_int;
    }
    mname = dap_malloc(
        dap_namelen + 1 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    m = 0 as libc::c_int;
    while *varlist.offset(m as isize) as libc::c_int == ' ' as i32 {
        m += 1;
        m;
    }
    nvars = 0 as libc::c_int;
    wtvar = 0 as libc::c_int;
    while *varlist.offset(m as isize) != 0 {
        if *varlist.offset(m as isize) as libc::c_int == '*' as i32 {
            if wtvar != 0 {
                fprintf(
                    dap_err,
                    b"(freqparse) Only one weight variable allowed: %s\n\0" as *const u8
                        as *const libc::c_char,
                    varlist,
                );
                exit(1 as libc::c_int);
            }
            wtvar = 1 as libc::c_int;
            m += 1;
            m;
            while *varlist.offset(m as isize) as libc::c_int == ' ' as i32 {
                m += 1;
                m;
            }
        }
        i = 0 as libc::c_int;
        while *varlist.offset((m + i) as isize) as libc::c_int != 0
            && *varlist.offset((m + i) as isize) as libc::c_int != ' ' as i32
            && *varlist.offset((m + i) as isize) as libc::c_int != '*' as i32
        {
            if i < dap_namelen {
                *mname.offset(i as isize) = *varlist.offset((m + i) as isize);
            } else {
                *mname.offset(i as isize) = '\0' as i32 as libc::c_char;
                fprintf(
                    dap_err,
                    b"(dap_list) Variable name too long: %s\n\0" as *const u8
                        as *const libc::c_char,
                    mname,
                );
                exit(1 as libc::c_int);
            }
            i += 1;
            i;
        }
        *mname.offset(i as isize) = '\0' as i32 as libc::c_char;
        if wtvar != 0 {
            let ref mut fresh10 = *wt.offset(0 as libc::c_int as isize);
            *fresh10 = dap_varnum(mname);
            if *fresh10 < 0 as libc::c_int {
                fprintf(
                    dap_err,
                    b"(dap_list) Weight variable unknown: %s\n\0" as *const u8
                        as *const libc::c_char,
                    mname,
                );
                exit(1 as libc::c_int);
            }
        } else {
            let fresh11 = nvars;
            nvars = nvars + 1;
            let ref mut fresh12 = *varv.offset(fresh11 as isize);
            *fresh12 = dap_varnum(mname);
            if *fresh12 < 0 as libc::c_int {
                fprintf(
                    dap_err,
                    b"(dap_list) Variable unknown: %s\n\0" as *const u8
                        as *const libc::c_char,
                    mname,
                );
                exit(1 as libc::c_int);
            }
        }
        m += i;
        while *varlist.offset(m as isize) as libc::c_int == ' ' as i32 {
            m += 1;
            m;
        }
    }
    dap_free(
        mname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return nvars;
}
unsafe extern "C" fn cmh1(
    mut tab: *mut *mut libc::c_double,
    mut cmh: *mut libc::c_double,
    mut cmhvar: *mut libc::c_double,
) {
    let mut rowsum: [libc::c_double; 2] = [0.; 2];
    let mut colsum: [libc::c_double; 2] = [0.; 2];
    let mut tabsum: libc::c_double = 0.;
    rowsum[0 as libc::c_int
        as usize] = *(*tab.offset(0 as libc::c_int as isize))
        .offset(0 as libc::c_int as isize)
        + *(*tab.offset(0 as libc::c_int as isize)).offset(1 as libc::c_int as isize);
    rowsum[1 as libc::c_int
        as usize] = *(*tab.offset(1 as libc::c_int as isize))
        .offset(0 as libc::c_int as isize)
        + *(*tab.offset(1 as libc::c_int as isize)).offset(1 as libc::c_int as isize);
    colsum[0 as libc::c_int
        as usize] = *(*tab.offset(0 as libc::c_int as isize))
        .offset(0 as libc::c_int as isize)
        + *(*tab.offset(1 as libc::c_int as isize)).offset(0 as libc::c_int as isize);
    colsum[1 as libc::c_int
        as usize] = *(*tab.offset(0 as libc::c_int as isize))
        .offset(1 as libc::c_int as isize)
        + *(*tab.offset(1 as libc::c_int as isize)).offset(1 as libc::c_int as isize);
    tabsum = rowsum[0 as libc::c_int as usize] + rowsum[1 as libc::c_int as usize];
    *cmh
        += *(*tab.offset(0 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
            - rowsum[0 as libc::c_int as usize] * colsum[0 as libc::c_int as usize]
                / tabsum;
    *cmhvar
        += rowsum[0 as libc::c_int as usize] * rowsum[1 as libc::c_int as usize]
            * colsum[0 as libc::c_int as usize] * colsum[1 as libc::c_int as usize]
            / (tabsum * tabsum * (tabsum - 1.0f64));
}
unsafe extern "C" fn printcmh(
    mut cmh: libc::c_double,
    mut cmhvar: libc::c_double,
    mut varv: *mut libc::c_int,
    mut nvar: libc::c_int,
    mut markv: *mut libc::c_int,
    mut nmark: libc::c_int,
) {
    let mut v: libc::c_int = 0;
    dap_swap();
    dap_head(markv, nmark);
    cmh = fabs(cmh) - 0.5f64;
    cmh *= cmh / cmhvar;
    fprintf(
        dap_lst,
        b"Cochran-Mantel-Haenszel test for %s x %s, stratified by\0" as *const u8
            as *const libc::c_char,
        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
            .offset(*varv.offset(1 as libc::c_int as isize) as isize),
        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
            .offset(*varv.offset(2 as libc::c_int as isize) as isize),
    );
    v = 0 as libc::c_int;
    while v < nvar - 2 as libc::c_int {
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
    fprintf(
        dap_lst,
        b"M0-squared = %g, Prob[M-squared > M0-squared] = %g\n\0" as *const u8
            as *const libc::c_char,
        cmh,
        rint(10000.0f64 * probchisq(cmh, 1 as libc::c_int)) / 10000.0f64,
    );
    dap_swap();
}
pub unsafe extern "C" fn freq(
    mut fname: *mut libc::c_char,
    mut varlist: *mut libc::c_char,
    mut stats: *mut libc::c_char,
    mut marks: *mut libc::c_char,
) {
    let mut outname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut statv: [libc::c_int; 13] = [0; 13];
    let mut typen: libc::c_int = 0;
    let mut celln: libc::c_int = 0;
    let mut markv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nmark: libc::c_int = 0;
    let mut varv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nvar: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut wt: libc::c_int = 0;
    let mut outlist: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut count: libc::c_double = 0.;
    let mut sumcount: libc::c_double = 0.;
    let mut tabmem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut tab: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut levmem: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut level: [*mut *mut libc::c_char; 2] = [0 as *mut *mut libc::c_char; 2];
    let mut nlevels: [libc::c_int; 2] = [0; 2];
    let mut l1: libc::c_int = 0;
    let mut l2: libc::c_int = 0;
    let mut cmh: libc::c_double = 0.;
    let mut cmhvar: libc::c_double = 0.;
    let mut more: libc::c_int = 0;
    let mut moremore: libc::c_int = 0;
    let mut strc: Option::<unsafe extern "C" fn() -> libc::c_int> = None;
    strc = ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
        >,
        Option::<unsafe extern "C" fn() -> libc::c_int>,
    >(
        Some(
            strcmp
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
    if fname.is_null() {
        fputs(
            b"(freq) No dataset name given.\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    outname = dap_malloc(
        (strlen(fname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_suffix(
        outname,
        fname,
        b".frq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    outlist = dap_malloc(
        dap_listlen + 1 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    markv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    varv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    tabmem = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxlev as libc::c_ulong)
            .wrapping_mul(dap_maxlev as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    tab = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxlev as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_double;
    l1 = 0 as libc::c_int;
    while l1 < dap_maxlev {
        let ref mut fresh13 = *tab.offset(l1 as isize);
        *fresh13 = tabmem.offset((l1 * dap_maxlev) as isize);
        l1 += 1;
        l1;
    }
    levmem = dap_malloc(
        2 as libc::c_int * dap_maxlev * (dap_strlen + 1 as libc::c_int),
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    level[0 as libc::c_int
        as usize] = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(dap_maxlev as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_char;
    level[1 as libc::c_int
        as usize] = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(dap_maxlev as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_char;
    l1 = 0 as libc::c_int;
    while l1 < dap_maxlev {
        let ref mut fresh14 = *(level[0 as libc::c_int as usize]).offset(l1 as isize);
        *fresh14 = levmem.offset((l1 * (dap_strlen + 1 as libc::c_int)) as isize);
        let ref mut fresh15 = *(level[1 as libc::c_int as usize]).offset(l1 as isize);
        *fresh15 = (*(level[0 as libc::c_int as usize]).offset(l1 as isize))
            .offset((dap_maxlev * (dap_strlen + 1 as libc::c_int)) as isize);
        l1 += 1;
        l1;
    }
    statparse(stats, statv.as_mut_ptr());
    inset(fname);
    typen = dap_varnum(
        b"_type_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if typen < 0 as libc::c_int {
        fputs(
            b"(freq) Missing _type_ variable.\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    nmark = dap_list(marks, markv, dap_maxvar);
    nvar = freqparse(varlist, varv, &mut wt);
    if statv[8 as libc::c_int as usize] != 0 && nvar < 3 as libc::c_int {
        fputs(
            b"(freq) Cochran-Mantel-Haenszel test performed only for tables with dimension >= 3.\n\0"
                as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    celln = dap_vd(
        b"_cell_ -1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    v = 0 as libc::c_int;
    while v < nvar {
        if v == 0 {
            strcpy(
                outlist,
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(*varv.offset(v as isize) as isize),
            );
        } else {
            strcat(outlist, b" \0" as *const u8 as *const libc::c_char);
            strcat(
                outlist,
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(*varv.offset(v as isize) as isize),
            );
        }
        v += 1;
        v;
    }
    strcat(outlist, b" _cell_ \0" as *const u8 as *const libc::c_char);
    strcat(outlist, marks);
    outset(outname, outlist);
    nlevels[0 as libc::c_int as usize] = 0 as libc::c_int;
    nlevels[1 as libc::c_int as usize] = 0 as libc::c_int;
    l1 = 0 as libc::c_int;
    while l1 < dap_maxlev {
        l2 = 0 as libc::c_int;
        while l2 < dap_maxlev {
            *(*tab.offset(l1 as isize)).offset(l2 as isize) = 0.0f64;
            l2 += 1;
            l2;
        }
        l1 += 1;
        l1;
    }
    dap_mark();
    sumcount = 0.0f64;
    more = 1 as libc::c_int;
    while more != 0 {
        more = step();
        if dap_newpart(markv, nmark) != 0 {
            dap_rewind();
            if nvar == 2 as libc::c_int {
                qsort(
                    *(level[0 as libc::c_int as usize]).offset(0 as libc::c_int as isize)
                        as *mut libc::c_void,
                    nlevels[0 as libc::c_int as usize] as size_t,
                    (dap_strlen + 1 as libc::c_int) as size_t,
                    ::std::mem::transmute::<
                        Option::<unsafe extern "C" fn() -> libc::c_int>,
                        __compar_fn_t,
                    >(strc),
                );
                qsort(
                    *(level[1 as libc::c_int as usize]).offset(0 as libc::c_int as isize)
                        as *mut libc::c_void,
                    nlevels[1 as libc::c_int as usize] as size_t,
                    (dap_strlen + 1 as libc::c_int) as size_t,
                    ::std::mem::transmute::<
                        Option::<unsafe extern "C" fn() -> libc::c_int>,
                        __compar_fn_t,
                    >(strc),
                );
            }
            count = 0.0f64;
            cmh = 0.0f64;
            cmhvar = 0.0f64;
            moremore = 1 as libc::c_int;
            while moremore != 0 {
                dap_mark();
                moremore = (step() != 0 && dap_newpart(markv, nmark) == 0)
                    as libc::c_int;
                if dap_newpart(varv, nvar) != 0 || dap_newpart(markv, nmark) != 0 {
                    freq1(varv, nvar, count, sumcount, statv.as_mut_ptr(), typen, celln);
                    if nvar == 2 as libc::c_int {
                        tabentry(
                            varv,
                            tab,
                            level.as_mut_ptr(),
                            nlevels.as_mut_ptr(),
                            count,
                        );
                    } else if statv[8 as libc::c_int as usize] != 0 {
                        tabentry(
                            varv
                                .offset(nvar as isize)
                                .offset(-(2 as libc::c_int as isize)),
                            tab,
                            level.as_mut_ptr(),
                            nlevels.as_mut_ptr(),
                            count,
                        );
                        if dap_newpart(varv, nvar - 2 as libc::c_int) != 0 {
                            cmh1(tab, &mut cmh, &mut cmhvar);
                        }
                    }
                    count = 0.0f64;
                }
                if wt >= 0 as libc::c_int {
                    count
                        += *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_dbl)
                            .offset(wt as isize);
                } else {
                    count += 1.0f64;
                }
            }
            if nvar == 2 as libc::c_int {
                freq2(
                    tab,
                    level.as_mut_ptr(),
                    nlevels.as_mut_ptr(),
                    statv.as_mut_ptr(),
                    markv,
                    nmark,
                    varv,
                    typen,
                    celln,
                );
            } else if statv[8 as libc::c_int as usize] != 0 {
                printcmh(cmh, cmhvar, varv, nvar, markv, nmark);
            }
            l1 = 0 as libc::c_int;
            while l1 < dap_maxlev {
                l2 = 0 as libc::c_int;
                while l2 < dap_maxlev {
                    *(*tab.offset(l1 as isize)).offset(l2 as isize) = 0.0f64;
                    l2 += 1;
                    l2;
                }
                l1 += 1;
                l1;
            }
            sumcount = 0.0f64;
            nlevels[0 as libc::c_int as usize] = 0 as libc::c_int;
            nlevels[1 as libc::c_int as usize] = 0 as libc::c_int;
        }
        if nvar == 2 as libc::c_int {
            findlev(
                *varv.offset(0 as libc::c_int as isize),
                level[0 as libc::c_int as usize],
                nlevels.as_mut_ptr(),
            );
            findlev(
                *varv.offset(1 as libc::c_int as isize),
                level[1 as libc::c_int as usize],
                nlevels.as_mut_ptr().offset(1 as libc::c_int as isize),
            );
        }
        if wt >= 0 as libc::c_int {
            sumcount
                += *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(wt as isize);
        } else {
            sumcount += 1.0f64;
        }
    }
    dap_free(
        outname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        outlist as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        markv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        varv as *mut libc::c_void,
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
        levmem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        level[0 as libc::c_int as usize] as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        level[1 as libc::c_int as usize] as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn trim1(
    mut vpct: *mut libc::c_double,
    mut nvar: libc::c_int,
    mut val: *mut *mut libc::c_double,
    mut nobs: libc::c_int,
    mut vmin: *mut libc::c_double,
    mut vmax: *mut libc::c_double,
) {
    let mut v: libc::c_int = 0;
    let mut trimcnt: libc::c_int = 0;
    v = 0 as libc::c_int;
    while v < nvar {
        trimcnt = rint(*vpct.offset(v as isize) / 100.0f64 * nobs as libc::c_double)
            as libc::c_int;
        qsort(
            *val.offset(v as isize) as *mut libc::c_void,
            nobs as size_t,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> libc::c_int>,
                __compar_fn_t,
            >(cmp),
        );
        *vmin.offset(v as isize) = *(*val.offset(v as isize)).offset(trimcnt as isize);
        *vmax
            .offset(
                v as isize,
            ) = *(*val.offset(v as isize))
            .offset((nobs - trimcnt - 1 as libc::c_int) as isize);
        v += 1;
        v;
    }
}
unsafe extern "C" fn trimparse(
    mut trimspec: *mut libc::c_char,
    mut varv: *mut libc::c_int,
    mut vpct: *mut libc::c_double,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut varname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: libc::c_int = 0;
    let mut div: libc::c_double = 0.;
    let mut digits: libc::c_int = 0;
    let mut nvar: libc::c_int = 0;
    if trimspec.is_null() {
        return 0 as libc::c_int;
    }
    varname = dap_malloc(
        dap_namelen + 1 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    n = 0 as libc::c_int;
    while *trimspec.offset(n as isize) as libc::c_int == ' ' as i32 {
        n += 1;
        n;
    }
    nvar = 0 as libc::c_int;
    while *trimspec.offset(n as isize) != 0 {
        i = 0 as libc::c_int;
        while *trimspec.offset((n + i) as isize) as libc::c_int != 0
            && *trimspec.offset((n + i) as isize) as libc::c_int != ' ' as i32
        {
            if i < dap_namelen {
                *varname.offset(i as isize) = *trimspec.offset((n + i) as isize);
            } else {
                *varname.offset(i as isize) = '\0' as i32 as libc::c_char;
                fprintf(
                    dap_err,
                    b"(trimparse) trim variable name too long: %s\n\0" as *const u8
                        as *const libc::c_char,
                    varname,
                );
                exit(1 as libc::c_int);
            }
            i += 1;
            i;
        }
        *varname.offset(i as isize) = '\0' as i32 as libc::c_char;
        n += i;
        v = dap_varnum(varname);
        if v >= 0 as libc::c_int {
            *varv.offset(nvar as isize) = v;
            while *trimspec.offset(n as isize) as libc::c_int == ' ' as i32 {
                n += 1;
                n;
            }
            digits = 0 as libc::c_int;
            div = 0.0f64;
            while '0' as i32 <= *trimspec.offset(n as isize) as libc::c_int
                && *trimspec.offset(n as isize) as libc::c_int <= '9' as i32
                || *trimspec.offset(n as isize) as libc::c_int == '.' as i32
            {
                if *trimspec.offset(n as isize) as libc::c_int == '.' as i32 {
                    if div >= 1.0f64 {
                        fprintf(
                            dap_err,
                            b"(trimparse) multiple decimal points in percent for %s\n\0"
                                as *const u8 as *const libc::c_char,
                            varname,
                        );
                        exit(1 as libc::c_int);
                    }
                    div = 1.0f64;
                } else {
                    digits = 10 as libc::c_int * digits
                        + *trimspec.offset(n as isize) as libc::c_int - '0' as i32;
                    if div >= 1.0f64 {
                        div *= 10.0f64;
                    }
                }
                n += 1;
                n;
            }
            *vpct.offset(nvar as isize) = digits as libc::c_double;
            if div >= 1.0f64 {
                *vpct.offset(nvar as isize) /= div;
            }
            if *vpct.offset(nvar as isize) > 0.0f64 {
                if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
                    .offset(v as isize) == -(1 as libc::c_int)
                {
                    nvar += 1;
                    nvar;
                } else {
                    fprintf(
                        dap_err,
                        b"(trimparse) trim variable not double: %s\n\0" as *const u8
                            as *const libc::c_char,
                        varname,
                    );
                    exit(1 as libc::c_int);
                }
            } else {
                fprintf(
                    dap_err,
                    b"(trimparse) no percent for trim variable: %s\n\0" as *const u8
                        as *const libc::c_char,
                    varname,
                );
                exit(1 as libc::c_int);
            }
        } else {
            fprintf(
                dap_err,
                b"(trimparse) unknown trim variable: %s\n\0" as *const u8
                    as *const libc::c_char,
                varname,
            );
            exit(1 as libc::c_int);
        }
        while *trimspec.offset(n as isize) as libc::c_int == ' ' as i32 {
            n += 1;
            n;
        }
    }
    dap_free(
        varname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return nvar;
}
pub unsafe extern "C" fn trim(
    mut fname: *mut libc::c_char,
    mut trimspec: *mut libc::c_char,
    mut marks: *mut libc::c_char,
) {
    let mut outname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut markv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nmark: libc::c_int = 0;
    let mut varv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut vpct: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut nvar: libc::c_int = 0;
    let mut valmem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut val: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut v: libc::c_int = 0;
    let mut nobs: libc::c_int = 0;
    let mut vmin: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut vmax: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut more: libc::c_int = 0;
    if fname.is_null() {
        fputs(
            b"(trim) No dataset name given.\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    outname = dap_malloc(
        (strlen(fname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_suffix(
        outname,
        fname,
        b".trm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    markv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    varv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    vpct = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    vmin = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    vmax = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    inset(fname);
    nmark = dap_list(marks, markv, dap_maxvar);
    nvar = trimparse(trimspec, varv, vpct);
    outset(outname, b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    valmem = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nvar as libc::c_ulong)
            .wrapping_mul(dap_maxval as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    val = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_double;
    v = 0 as libc::c_int;
    while v <= nvar {
        let ref mut fresh16 = *val.offset(v as isize);
        *fresh16 = valmem.offset((v * dap_maxval) as isize);
        v += 1;
        v;
    }
    dap_mark();
    nobs = 0 as libc::c_int;
    more = 1 as libc::c_int;
    while more != 0 {
        more = step();
        if dap_newpart(markv, nmark) != 0 {
            trim1(vpct, nvar, val, nobs, vmin, vmax);
            dap_rewind();
            while step() != 0 && dap_newpart(markv, nmark) == 0 {
                v = 0 as libc::c_int;
                while v < nvar {
                    if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                        .do_dbl)
                        .offset(*varv.offset(v as isize) as isize)
                        < *vmin.offset(v as isize)
                        || *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_dbl)
                            .offset(*varv.offset(v as isize) as isize)
                            > *vmax.offset(v as isize)
                    {
                        break;
                    }
                    v += 1;
                    v;
                }
                if v == nvar {
                    output();
                }
                dap_mark();
            }
            nobs = 0 as libc::c_int;
        }
        if nobs < dap_maxval {
            v = 0 as libc::c_int;
            while v < nvar {
                if finite(
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                        .offset(*varv.offset(v as isize) as isize),
                ) != 0
                {
                    *(*val.offset(v as isize))
                        .offset(
                            nobs as isize,
                        ) = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                        .do_dbl)
                        .offset(*varv.offset(v as isize) as isize);
                } else {
                    fprintf(
                        dap_err,
                        b"(trim) NaN value %d for %s\n\0" as *const u8
                            as *const libc::c_char,
                        nobs,
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_nam)
                            .offset(*varv.offset(v as isize) as isize),
                    );
                    exit(1 as libc::c_int);
                }
                v += 1;
                v;
            }
        } else {
            fputs(
                b"(group) Too many data.\n\0" as *const u8 as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        nobs += 1;
        nobs;
    }
    dap_free(
        outname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        markv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        varv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        vpct as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        valmem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        val as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        vmin as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        vmax as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
