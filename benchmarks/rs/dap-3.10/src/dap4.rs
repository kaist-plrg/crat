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
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn step() -> libc::c_int;
    fn inset(fname: *mut libc::c_char);
    fn outset(fname: *mut libc::c_char, varlist: *mut libc::c_char);
    fn output();
    fn sort(
        fname: *mut libc::c_char,
        varlist: *mut libc::c_char,
        modifiers: *mut libc::c_char,
    );
    fn table(
        fname: *mut libc::c_char,
        rowvars: *mut libc::c_char,
        colvars: *mut libc::c_char,
        format: *mut libc::c_char,
        marks: *mut libc::c_char,
    );
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
    fn probt(t1: libc::c_double, di: libc::c_int) -> libc::c_double;
    fn tpoint(p: libc::c_double, in_0: libc::c_int) -> libc::c_double;
    fn probf(f2: libc::c_double, id1: libc::c_int, id2: libc::c_int) -> libc::c_double;
    fn dap_sr(
        numdf: libc::c_int,
        dendf: libc::c_int,
        pt: libc::c_double,
    ) -> libc::c_double;
    fn dap_srpt(
        numdf: libc::c_int,
        dendf: libc::c_int,
        pt: libc::c_double,
        pr: libc::c_double,
        alpha: libc::c_double,
    ) -> libc::c_double;
    fn dap_md(
        numdf: libc::c_int,
        dendf: libc::c_int,
        pt: libc::c_double,
    ) -> libc::c_double;
    fn dap_mdpt(
        numdf: libc::c_int,
        dendf: libc::c_int,
        pt: libc::c_double,
        pr: libc::c_double,
        alpha: libc::c_double,
    ) -> libc::c_double;
    fn dap_malloc(nbytes: libc::c_int, mesg: *mut libc::c_char) -> *mut libc::c_char;
    fn dap_free(ptr: *mut libc::c_void, mesg: *mut libc::c_char);
    static mut dap_maxvar: libc::c_int;
    static mut dap_namelen: libc::c_int;
    static mut dap_strlen: libc::c_int;
    static mut dap_maxcell: libc::c_int;
    static mut dap_maxtreat: libc::c_int;
    static mut dap_maxlev: libc::c_int;
    static mut dap_redtol: libc::c_double;
    static mut dap_orthtol: libc::c_double;
    static mut dap_zerotol: libc::c_double;
    static mut dap_obs: [dataobs; 0];
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
unsafe extern "C" fn gettwo(mut x: libc::c_double) -> libc::c_double {
    let mut t: libc::c_double = 0.;
    t = 1.0f64;
    x = fabs(x);
    while t > x {
        t /= 2.0f64;
    }
    while t < x {
        t *= 2.0f64;
    }
    return t;
}
unsafe extern "C" fn rowred(
    mut coeff: *mut *mut libc::c_double,
    mut rterm: *mut libc::c_int,
    mut byterm: libc::c_int,
    mut ncells: libc::c_int,
    mut nobs: *mut libc::c_double,
    mut row1: libc::c_int,
    mut row2: libc::c_int,
    mut nonz: *mut libc::c_int,
) -> libc::c_int {
    let mut misscol: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut celln: libc::c_int = 0;
    let mut ncols: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut subcol: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut subrow: libc::c_int = 0;
    let mut maxrow: libc::c_int = 0;
    let mut colmax: libc::c_double = 0.;
    let mut rowmax: libc::c_double = 0.;
    let mut tmp: libc::c_double = 0.;
    let mut tmp1: libc::c_double = 0.;
    let mut tmp2: libc::c_double = 0.;
    let mut itmp: libc::c_int = 0;
    let mut mult1: libc::c_double = 0.;
    let mut mult2: libc::c_double = 0.;
    let mut nterms: libc::c_int = 0;
    let mut term: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut t: libc::c_int = 0;
    let mut elimrow: libc::c_int = 0;
    nterms = 0 as libc::c_int;
    elimrow = 0 as libc::c_int;
    misscol = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(ncells as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    term = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(ncells as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    if byterm != 0 {
        nterms = 0 as libc::c_int;
        let fresh0 = nterms;
        nterms = nterms + 1;
        *term.offset(fresh0 as isize) = 0 as libc::c_int;
        row = row1 + 1 as libc::c_int;
        while row <= row2 {
            t = 1 as libc::c_int;
            while t < nterms {
                if *rterm.offset(row as isize) == *term.offset(t as isize) {
                    break;
                }
                t += 1;
                t;
            }
            if t == nterms {
                let fresh1 = nterms;
                nterms = nterms + 1;
                *term.offset(fresh1 as isize) = *rterm.offset(row as isize);
            }
            row += 1;
            row;
        }
    }
    celln = 0 as libc::c_int;
    ncols = 0 as libc::c_int;
    while celln < ncells {
        if *nobs.offset(celln as isize) == 0.0f64 {
            let fresh2 = ncols;
            ncols = ncols + 1;
            *misscol.offset(fresh2 as isize) = celln;
        }
        celln += 1;
        celln;
    }
    t = 0 as libc::c_int;
    while byterm != 0 && t < nterms || t < 1 as libc::c_int {
        row = if t != 0 { row1 + 1 as libc::c_int } else { 0 as libc::c_int };
        while row <= (if t != 0 { row2 } else { row1 }) {
            if byterm == 0 || row <= row1
                || *rterm.offset(row as isize) == *term.offset(t as isize)
            {
                break;
            }
            row += 1;
            row;
        }
        col = 0 as libc::c_int;
        while col < ncols {
            while byterm != 0 && row > row1
                && *rterm.offset(row as isize) != *term.offset(t as isize)
                && row < (if t != 0 { row2 } else { row1 })
            {
                row += 1;
                row;
            }
            colmax = 0.0f64;
            maxrow = row;
            subrow = row;
            while subrow <= (if t != 0 { row2 } else { row1 }) {
                if !(byterm != 0 && subrow > row1
                    && *rterm.offset(subrow as isize) != *term.offset(t as isize))
                {
                    tmp = fabs(
                        *(*coeff.offset(subrow as isize))
                            .offset(*misscol.offset(col as isize) as isize),
                    );
                    if tmp > colmax {
                        colmax = tmp;
                        maxrow = subrow;
                    }
                }
                subrow += 1;
                subrow;
            }
            if colmax > dap_redtol {
                if maxrow != row {
                    subcol = 0 as libc::c_int;
                    while subcol < ncells {
                        tmp = *(*coeff.offset(row as isize)).offset(subcol as isize);
                        *(*coeff.offset(row as isize))
                            .offset(
                                subcol as isize,
                            ) = *(*coeff.offset(maxrow as isize))
                            .offset(subcol as isize);
                        *(*coeff.offset(maxrow as isize)).offset(subcol as isize) = tmp;
                        itmp = *rterm.offset(row as isize);
                        *rterm.offset(row as isize) = *rterm.offset(maxrow as isize);
                        *rterm.offset(maxrow as isize) = itmp;
                        subcol += 1;
                        subcol;
                    }
                }
                subrow = row + 1 as libc::c_int;
                while subrow <= row2 {
                    if !(byterm != 0 && subrow > row1
                        && *rterm.offset(subrow as isize) != *term.offset(t as isize))
                    {
                        mult1 = *(*coeff.offset(subrow as isize))
                            .offset(*misscol.offset(col as isize) as isize);
                        mult2 = *(*coeff.offset(row as isize))
                            .offset(*misscol.offset(col as isize) as isize);
                        if fabs(mult1) > dap_redtol * fabs(mult2) {
                            subcol = 0 as libc::c_int;
                            rowmax = 0.0f64;
                            while subcol < ncells {
                                tmp = mult2
                                    * *(*coeff.offset(subrow as isize)).offset(subcol as isize);
                                tmp1 = mult1
                                    * *(*coeff.offset(row as isize)).offset(subcol as isize);
                                *(*coeff.offset(subrow as isize))
                                    .offset(subcol as isize) = tmp - tmp1;
                                tmp2 = fabs(
                                    *(*coeff.offset(subrow as isize)).offset(subcol as isize),
                                );
                                if tmp2 < dap_redtol * (fabs(tmp) + fabs(tmp1)) {
                                    *(*coeff.offset(subrow as isize))
                                        .offset(subcol as isize) = 0.0f64;
                                } else if tmp2 > rowmax {
                                    rowmax = tmp2;
                                }
                                subcol += 1;
                                subcol;
                            }
                            tmp = gettwo(rowmax);
                            subcol = 0 as libc::c_int;
                            while subcol < ncells {
                                *(*coeff.offset(subrow as isize)).offset(subcol as isize)
                                    /= tmp;
                                if fabs(
                                    *(*coeff.offset(subrow as isize)).offset(subcol as isize),
                                ) < dap_redtol
                                {
                                    *(*coeff.offset(subrow as isize))
                                        .offset(subcol as isize) = 0.0f64;
                                }
                                subcol += 1;
                                subcol;
                            }
                        }
                    }
                    subrow += 1;
                    subrow;
                }
                row += 1;
                row;
            }
            col += 1;
            col;
        }
        if t == 0 {
            elimrow = row;
        }
        t += 1;
        t;
    }
    if byterm != 0 {
        t = 0 as libc::c_int;
        while t < nterms {
            row = if t != 0 { row2 } else { row1 };
            while row > (if t != 0 { row1 + 1 as libc::c_int } else { 0 as libc::c_int })
            {
                if row <= row1 || *rterm.offset(row as isize) == *term.offset(t as isize)
                {
                    break;
                }
                row -= 1;
                row;
            }
            while row > (if t != 0 { row1 + 1 as libc::c_int } else { 0 as libc::c_int })
            {
                if !(row > row1
                    && *rterm.offset(row as isize) != *term.offset(t as isize))
                {
                    col = 0 as libc::c_int;
                    while col < ncols {
                        if fabs(
                            *(*coeff.offset(row as isize))
                                .offset(*misscol.offset(col as isize) as isize),
                        ) > dap_redtol
                        {
                            break;
                        }
                        col += 1;
                        col;
                    }
                    if col < ncols {
                        subrow = row - 1 as libc::c_int;
                        while subrow
                            >= (if t != 0 {
                                row1 + 1 as libc::c_int
                            } else {
                                0 as libc::c_int
                            })
                        {
                            if !(subrow > row1
                                && *rterm.offset(subrow as isize)
                                    != *term.offset(t as isize))
                            {
                                mult1 = *(*coeff.offset(subrow as isize))
                                    .offset(*misscol.offset(col as isize) as isize);
                                mult2 = *(*coeff.offset(row as isize))
                                    .offset(*misscol.offset(col as isize) as isize);
                                if fabs(mult1) > dap_redtol * fabs(mult2) {
                                    col = 0 as libc::c_int;
                                    rowmax = 0.0f64;
                                    while col < ncells {
                                        tmp = mult2
                                            * *(*coeff.offset(subrow as isize)).offset(col as isize);
                                        tmp1 = mult1
                                            * *(*coeff.offset(row as isize)).offset(col as isize);
                                        *(*coeff.offset(subrow as isize))
                                            .offset(col as isize) = tmp - tmp1;
                                        if fabs(
                                            *(*coeff.offset(subrow as isize)).offset(col as isize),
                                        ) < dap_redtol * (fabs(tmp) + fabs(tmp1))
                                        {
                                            *(*coeff.offset(subrow as isize))
                                                .offset(col as isize) = 0.0f64;
                                        } else if fabs(
                                            *(*coeff.offset(subrow as isize)).offset(col as isize),
                                        ) > rowmax
                                        {
                                            rowmax = fabs(
                                                *(*coeff.offset(subrow as isize)).offset(col as isize),
                                            );
                                        }
                                        col += 1;
                                        col;
                                    }
                                    tmp = gettwo(rowmax);
                                    col = 0 as libc::c_int;
                                    while col < ncells {
                                        *(*coeff.offset(subrow as isize)).offset(col as isize)
                                            /= tmp;
                                        if fabs(
                                            *(*coeff.offset(subrow as isize)).offset(col as isize),
                                        ) < dap_redtol
                                        {
                                            *(*coeff.offset(subrow as isize))
                                                .offset(col as isize) = 0.0f64;
                                        }
                                        col += 1;
                                        col;
                                    }
                                }
                            }
                            subrow -= 1;
                            subrow;
                        }
                    }
                }
                row -= 1;
                row;
            }
            t += 1;
            t;
        }
    }
    if !nonz.is_null() {
        row = 0 as libc::c_int;
        while row <= row2 {
            *nonz.offset(row as isize) = 0 as libc::c_int;
            col = 0 as libc::c_int;
            while col < ncells {
                if fabs(*(*coeff.offset(row as isize)).offset(col as isize))
                    > dap_zerotol
                {
                    if *nobs.offset(col as isize) > 0.0f64 {
                        *nonz.offset(row as isize) = 1 as libc::c_int;
                        break;
                    }
                }
                col += 1;
                col;
            }
            row += 1;
            row;
        }
    }
    dap_free(
        misscol as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        term as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if elimrow == ncols {
        return elimrow;
    }
    fputs(
        b"error terms insufficient to impute missing cells\n\0" as *const u8
            as *const libc::c_char,
        dap_err,
    );
    exit(1 as libc::c_int);
}
unsafe extern "C" fn lcm(
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> libc::c_double {
    let mut ix: libc::c_int = 0;
    let mut iy: libc::c_int = 0;
    let mut q: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut lcmxy: libc::c_int = 0;
    ix = x as libc::c_int;
    iy = y as libc::c_int;
    lcmxy = ix * iy;
    loop {
        q = iy / ix;
        r = iy - q * ix;
        if !(r != 0) {
            break;
        }
        iy = ix;
        ix = r;
    }
    return (lcmxy / ix) as libc::c_double;
}
unsafe extern "C" fn orthog(
    mut coeff: *mut *mut libc::c_double,
    mut row0: libc::c_int,
    mut row1: libc::c_int,
    mut row2: libc::c_int,
    mut ncells: libc::c_int,
    mut nobs: *mut libc::c_double,
    mut indep: *mut libc::c_int,
    mut rterm: *mut libc::c_int,
) -> libc::c_int {
    let mut lcmnobs: libc::c_double = 0.;
    let mut updown: libc::c_int = 0;
    let mut term: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nterms: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut cr: libc::c_int = 0;
    let mut cc: libc::c_int = 0;
    let mut crr: libc::c_int = 0;
    let mut lensq: libc::c_double = 0.;
    let mut dot: libc::c_double = 0.;
    let mut tmp: libc::c_double = 0.;
    let mut tmp1: libc::c_double = 0.;
    let mut mult1: libc::c_double = 0.;
    let mut mult2: libc::c_double = 0.;
    let mut rowmax: libc::c_double = 0.;
    let mut df: libc::c_int = 0;
    nterms = 0 as libc::c_int;
    term = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(ncells as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    if row2 >= row0 {
        updown = 1 as libc::c_int;
    } else {
        updown = -(1 as libc::c_int);
    }
    df = 0 as libc::c_int;
    if !rterm.is_null() {
        cr = row0;
        nterms = 0 as libc::c_int;
        while cr != row2 + updown {
            t = 0 as libc::c_int;
            while t < nterms {
                if *rterm.offset(cr as isize) == *term.offset(t as isize) {
                    break;
                }
                t += 1;
                t;
            }
            if t == nterms {
                let fresh3 = nterms;
                nterms = nterms + 1;
                *term.offset(fresh3 as isize) = *rterm.offset(cr as isize);
            }
            cr += updown;
        }
    }
    cc = 0 as libc::c_int;
    lcmnobs = 1.0f64;
    while cc < ncells {
        if *nobs.offset(cc as isize) != 0.0f64 {
            lcmnobs = lcm(lcmnobs, *nobs.offset(cc as isize));
        }
        cc += 1;
        cc;
    }
    lcmnobs /= gettwo(lcmnobs);
    t = 0 as libc::c_int;
    while !rterm.is_null() && t < nterms || t < 1 as libc::c_int {
        cr = row0;
        while cr != row1 + updown {
            if !(!rterm.is_null()
                && *rterm.offset(cr as isize) != *term.offset(t as isize))
            {
                cc = 0 as libc::c_int;
                while cc < ncells {
                    if fabs(*(*coeff.offset(cr as isize)).offset(cc as isize))
                        > dap_orthtol
                    {
                        break;
                    }
                    cc += 1;
                    cc;
                }
                if cc < ncells {
                    break;
                }
            }
            cr += updown;
        }
        while cr != row1 + updown {
            if !(!rterm.is_null()
                && *rterm.offset(cr as isize) != *term.offset(t as isize))
            {
                lensq = 0.0f64;
                cc = 0 as libc::c_int;
                while cc < ncells {
                    if *nobs.offset(cc as isize) > 0.0f64 {
                        tmp = *(*coeff.offset(cr as isize)).offset(cc as isize);
                        lensq += tmp * tmp * (lcmnobs / *nobs.offset(cc as isize));
                    }
                    cc += 1;
                    cc;
                }
                if lensq > dap_orthtol {
                    *indep.offset(cr as isize) = 1 as libc::c_int;
                    df += 1;
                    df;
                    crr = cr + updown;
                    while crr != row2 + updown {
                        if !(!rterm.is_null()
                            && *rterm.offset(crr as isize) != *term.offset(t as isize))
                        {
                            dot = 0.0f64;
                            cc = 0 as libc::c_int;
                            while cc < ncells {
                                if *nobs.offset(cc as isize) > 0.0f64 {
                                    dot
                                        += *(*coeff.offset(cr as isize)).offset(cc as isize)
                                            * *(*coeff.offset(crr as isize)).offset(cc as isize)
                                            * (lcmnobs / *nobs.offset(cc as isize));
                                }
                                cc += 1;
                                cc;
                            }
                            if fabs(dot) > dap_orthtol * lensq {
                                mult1 = dot;
                                mult2 = lensq;
                                cc = 0 as libc::c_int;
                                rowmax = 0.0f64;
                                while cc < ncells {
                                    tmp = mult2
                                        * *(*coeff.offset(crr as isize)).offset(cc as isize);
                                    tmp1 = mult1
                                        * *(*coeff.offset(cr as isize)).offset(cc as isize);
                                    *(*coeff.offset(crr as isize))
                                        .offset(cc as isize) = tmp - tmp1;
                                    if fabs(*(*coeff.offset(crr as isize)).offset(cc as isize))
                                        < dap_orthtol * (fabs(tmp) + fabs(tmp1))
                                    {
                                        *(*coeff.offset(crr as isize)).offset(cc as isize) = 0.0f64;
                                    } else if fabs(
                                        *(*coeff.offset(crr as isize)).offset(cc as isize),
                                    ) > rowmax
                                    {
                                        rowmax = fabs(
                                            *(*coeff.offset(crr as isize)).offset(cc as isize),
                                        );
                                    }
                                    cc += 1;
                                    cc;
                                }
                                tmp = gettwo(rowmax);
                                cc = 0 as libc::c_int;
                                while cc < ncells {
                                    *(*coeff.offset(crr as isize)).offset(cc as isize) /= tmp;
                                    if fabs(*(*coeff.offset(crr as isize)).offset(cc as isize))
                                        < dap_orthtol
                                    {
                                        *(*coeff.offset(crr as isize)).offset(cc as isize) = 0.0f64;
                                    }
                                    cc += 1;
                                    cc;
                                }
                            }
                        }
                        crr += updown;
                    }
                } else {
                    cc = 0 as libc::c_int;
                    while cc < ncells {
                        *(*coeff.offset(cr as isize)).offset(cc as isize) = 0.0f64;
                        cc += 1;
                        cc;
                    }
                }
            }
            cr += updown;
        }
        while cr != row2 + updown {
            if rterm.is_null() || *rterm.offset(cr as isize) == *term.offset(t as isize)
            {
                lensq = 0.0f64;
                cc = 0 as libc::c_int;
                while cc < ncells {
                    if *nobs.offset(cc as isize) > 0.0f64 {
                        tmp = *(*coeff.offset(cr as isize)).offset(cc as isize);
                        lensq += tmp * tmp * (lcmnobs / *nobs.offset(cc as isize));
                    }
                    cc += 1;
                    cc;
                }
                if lensq > dap_orthtol {
                    *indep.offset(cr as isize) = 1 as libc::c_int;
                    df += 1;
                    df;
                }
            }
            cr += updown;
        }
        t += 1;
        t;
    }
    dap_free(
        term as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return df;
}
unsafe extern "C" fn sumcheck(
    mut caller: *mut libc::c_char,
    mut coeff: *mut *mut libc::c_double,
    mut ncells: libc::c_int,
    mut nerrors: libc::c_int,
    mut ncontrasts: libc::c_int,
    mut rterm: *mut libc::c_int,
) {
    let mut r: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut elt: libc::c_double = 0.;
    let mut rowsum: libc::c_double = 0.;
    let mut rowmax: libc::c_double = 0.;
    r = 0 as libc::c_int;
    while r < nerrors + ncontrasts {
        c = 0 as libc::c_int;
        rowsum = 0.0f64;
        rowmax = 0.0f64;
        while c < ncells {
            elt = *(*coeff.offset(r as isize)).offset(c as isize);
            rowsum += elt;
            elt = fabs(elt);
            if elt > rowmax {
                rowmax = elt;
            }
            c += 1;
            c;
        }
        if fabs(rowsum) > dap_zerotol * rowmax {
            fprintf(
                dap_err,
                b"(sumcheck:%s) Unable to fit model:\n\0" as *const u8
                    as *const libc::c_char,
                caller,
            );
            if r < nerrors {
                fprintf(
                    dap_err,
                    b"Error %d sums to %g:\n\0" as *const u8 as *const libc::c_char,
                    r,
                    rowsum,
                );
            } else {
                fprintf(
                    dap_err,
                    b"Contrast %d sums to %g:\n\0" as *const u8 as *const libc::c_char,
                    r - nerrors,
                    rowsum,
                );
            }
            fprintf(
                dap_err,
                b"%s%d (%x) \0" as *const u8 as *const libc::c_char,
                if r < nerrors {
                    b"E\0" as *const u8 as *const libc::c_char
                } else {
                    b"C\0" as *const u8 as *const libc::c_char
                },
                if r < nerrors { r } else { r - nerrors },
                if !rterm.is_null() {
                    *rterm.offset(r as isize)
                } else {
                    0 as libc::c_int
                },
            );
            c = 0 as libc::c_int;
            while c < ncells {
                fprintf(
                    dap_err,
                    b" %g\0" as *const u8 as *const libc::c_char,
                    *(*coeff.offset(r as isize)).offset(c as isize),
                );
                c += 1;
                c;
            }
            putc('\n' as i32, dap_err);
            exit(1 as libc::c_int);
        }
        r += 1;
        r;
    }
}
unsafe extern "C" fn testparse(
    mut test: *mut libc::c_char,
    mut termv: *mut libc::c_char,
    mut varv: *mut libc::c_int,
    mut nvars: libc::c_int,
) -> libc::c_int {
    let mut nterms: libc::c_int = 0;
    let mut tv: libc::c_int = 0;
    let mut vname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut bit: libc::c_int = 0;
    let mut term: libc::c_int = 0;
    let mut vv: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut firstv: libc::c_int = 0;
    term = 0 as libc::c_int;
    vname = dap_malloc(
        dap_namelen + 1 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if test.is_null() || *test.offset(0 as libc::c_int as isize) == 0 {
        return 0 as libc::c_int;
    }
    nterms = 1 as libc::c_int;
    tv = 1 as libc::c_int;
    while tv < nvars {
        nterms *= 2 as libc::c_int;
        tv += 1;
        tv;
    }
    nterms -= 1;
    nterms;
    tv = 1 as libc::c_int;
    while tv <= nterms {
        *termv.offset(tv as isize) = 'e' as i32 as libc::c_char;
        tv += 1;
        tv;
    }
    t = 0 as libc::c_int;
    while *test.offset(t as isize) as libc::c_int == ' ' as i32 {
        t += 1;
        t;
    }
    firstv = 1 as libc::c_int;
    while *test.offset(t as isize) != 0 {
        if firstv != 0 {
            term = 0 as libc::c_int;
        }
        firstv = 0 as libc::c_int;
        n = 0 as libc::c_int;
        while *test.offset(t as isize) as libc::c_int != 0
            && *test.offset(t as isize) as libc::c_int != ' ' as i32
            && *test.offset(t as isize) as libc::c_int != '*' as i32
        {
            if n < dap_namelen {
                let fresh4 = t;
                t = t + 1;
                let fresh5 = n;
                n = n + 1;
                *vname.offset(fresh5 as isize) = *test.offset(fresh4 as isize);
            } else {
                *vname.offset(n as isize) = '\0' as i32 as libc::c_char;
                fprintf(
                    dap_err,
                    b"(testparse) name too long: %s\n\0" as *const u8
                        as *const libc::c_char,
                    vname,
                );
                exit(1 as libc::c_int);
            }
        }
        *vname.offset(n as isize) = '\0' as i32 as libc::c_char;
        v = dap_varnum(vname);
        if v < 0 as libc::c_int {
            fprintf(
                dap_err,
                b"(testparse) unknown variable: %s\n\0" as *const u8
                    as *const libc::c_char,
                vname,
            );
            exit(1 as libc::c_int);
        }
        bit = 0x1 as libc::c_int;
        vv = 1 as libc::c_int;
        while vv < nvars {
            if *varv.offset(vv as isize) == v {
                break;
            }
            vv += 1;
            vv;
            bit = bit << 1 as libc::c_int;
        }
        if vv == nvars {
            fprintf(
                dap_err,
                b"(testparse) variable in test not in model: %s\n\0" as *const u8
                    as *const libc::c_char,
                vname,
            );
            exit(1 as libc::c_int);
        }
        term |= bit;
        while *test.offset(t as isize) as libc::c_int == ' ' as i32 {
            t += 1;
            t;
        }
        if *test.offset(t as isize) as libc::c_int == '*' as i32 {
            t += 1;
            t;
            while *test.offset(t as isize) as libc::c_int == ' ' as i32 {
                t += 1;
                t;
            }
        } else {
            *termv.offset(term as isize) = 'c' as i32 as libc::c_char;
            firstv = 1 as libc::c_int;
        }
    }
    dap_free(
        vname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return nterms;
}
unsafe extern "C" fn levn(
    mut levstr: *mut libc::c_char,
    mut levval: *mut *mut libc::c_char,
    mut nlevels: *mut libc::c_int,
) -> libc::c_int {
    let mut l: libc::c_int = 0;
    l = 0 as libc::c_int;
    while l < *nlevels
        && *(*levval.offset(l as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            != 0
    {
        if strcmp(levstr, *levval.offset(l as isize)) == 0 {
            return l;
        }
        l += 1;
        l;
    }
    if l < dap_maxlev - 1 as libc::c_int {
        strcpy(*levval.offset(l as isize), levstr);
        if l < *nlevels {
            return *nlevels
        } else {
            let fresh6 = *nlevels;
            *nlevels = *nlevels + 1;
            return fresh6;
        }
    } else {
        fprintf(
            dap_err,
            b"(levn) too many levels: %s\n\0" as *const u8 as *const libc::c_char,
            levstr,
        );
        exit(1 as libc::c_int);
    };
}
unsafe extern "C" fn putrand(
    mut bits: libc::c_int,
    mut coeff: libc::c_double,
    mut varv: *mut libc::c_int,
) {
    let mut v: libc::c_int = 0;
    let mut termn: libc::c_int = 0;
    if coeff != 0.0f64 {
        fprintf(dap_lst, b"\n    %g Var(\0" as *const u8 as *const libc::c_char, coeff);
    }
    v = 1 as libc::c_int;
    termn = 1 as libc::c_int;
    while bits != 0 {
        if bits & 0x1 as libc::c_int != 0 {
            if termn > 1 as libc::c_int {
                putc('*' as i32, dap_lst);
            }
            fputs(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(*varv.offset(v as isize) as isize),
                dap_lst,
            );
            termn += 1;
            termn;
        }
        bits = bits >> 1 as libc::c_int;
        v += 1;
        v;
    }
    if coeff != 0.0f64 {
        putc(')' as i32, dap_lst);
    }
}
unsafe extern "C" fn ems(
    mut coeff: *mut *mut libc::c_double,
    mut level: *mut *mut libc::c_int,
    mut ncells: libc::c_int,
    mut rterm: *mut libc::c_int,
    mut indep: *mut libc::c_int,
    mut nrows: libc::c_int,
    mut nobs: *mut libc::c_double,
    mut varv: *mut libc::c_int,
    mut termv: *mut libc::c_char,
    mut nterm: libc::c_int,
    mut emscoeff: *mut *mut libc::c_double,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut u: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut cum: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut df: libc::c_double = 0.;
    let mut effrow: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut neffrows: libc::c_int = 0;
    let mut factlev: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nfactlevs: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    let mut emsc: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut lensq: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut basecell: libc::c_int = 0;
    let mut nextcell: libc::c_int = 0;
    let mut used: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut s: libc::c_int = 0;
    let mut same: libc::c_int = 0;
    let mut tmp: libc::c_double = 0.;
    let mut factor: libc::c_int = 0;
    cum = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(ncells as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    emsc = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(ncells as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    lensq = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(ncells as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    effrow = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(ncells as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    factlev = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(ncells as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    used = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(ncells as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    r = 0 as libc::c_int;
    while r < nrows {
        *emsc.offset(r as isize) = 0.0f64;
        r += 1;
        r;
    }
    emscoeff = emscoeff.offset(1);
    emscoeff;
    t = 1 as libc::c_int;
    factor = 0 as libc::c_int;
    while t <= nterm {
        if *termv.offset(t as isize) as libc::c_int == 'c' as i32
            || *termv.offset(t as isize) as libc::c_int == 'n' as i32
        {
            if !varv.is_null() {
                fputs(b"EMS(\0" as *const u8 as *const libc::c_char, dap_lst);
                putrand(t, 0.0f64, varv);
                fputs(b") =\0" as *const u8 as *const libc::c_char, dap_lst);
            }
            *(*emscoeff.offset(factor as isize))
                .offset(0 as libc::c_int as isize) = 1.0f64;
            r = 0 as libc::c_int;
            neffrows = 0 as libc::c_int;
            while r < nrows {
                if *indep.offset(r as isize) != 0 && *rterm.offset(r as isize) == t {
                    let fresh7 = neffrows;
                    neffrows = neffrows + 1;
                    *effrow.offset(fresh7 as isize) = r;
                }
                r += 1;
                r;
            }
            u = 1 as libc::c_int;
            while u <= nterm {
                r = 0 as libc::c_int;
                while r < nrows {
                    if *indep.offset(r as isize) != 0 && *rterm.offset(r as isize) == u {
                        break;
                    }
                    r += 1;
                    r;
                }
                if u & t == t && r < nrows {
                    bits = u;
                    nfactlevs = 0 as libc::c_int;
                    r = 1 as libc::c_int;
                    while bits != 0 {
                        if bits & 0x1 as libc::c_int != 0 {
                            let fresh8 = nfactlevs;
                            nfactlevs = nfactlevs + 1;
                            *factlev.offset(fresh8 as isize) = r;
                        }
                        bits = bits >> 1 as libc::c_int;
                        r += 1;
                        r;
                    }
                    r = 0 as libc::c_int;
                    while r < neffrows {
                        if *indep.offset(r as isize) != 0 {
                            *lensq.offset(r as isize) = 0.0f64;
                            *emsc.offset(r as isize) = 0.0f64;
                        }
                        r += 1;
                        r;
                    }
                    c = 0 as libc::c_int;
                    while c < ncells {
                        *used.offset(c as isize) = 0 as libc::c_int;
                        c += 1;
                        c;
                    }
                    basecell = 0 as libc::c_int;
                    nextcell = 0 as libc::c_int;
                    while basecell < ncells {
                        r = 0 as libc::c_int;
                        while r < neffrows {
                            *cum.offset(r as isize) = 0.0f64;
                            r += 1;
                            r;
                        }
                        c = basecell;
                        while c < ncells {
                            r = 0 as libc::c_int;
                            while r < neffrows {
                                if *indep.offset(r as isize) != 0 {
                                    tmp = *(*coeff.offset(*effrow.offset(r as isize) as isize))
                                        .offset(c as isize);
                                    *cum.offset(r as isize) += tmp;
                                    *lensq.offset(r as isize)
                                        += tmp * tmp / *nobs.offset(c as isize);
                                }
                                r += 1;
                                r;
                            }
                            *used.offset(c as isize) = 1 as libc::c_int;
                            c += 1;
                            c;
                            while c < ncells {
                                s = 0 as libc::c_int;
                                same = 1 as libc::c_int;
                                while s < nfactlevs {
                                    if *(*level.offset(*factlev.offset(s as isize) as isize))
                                        .offset(basecell as isize)
                                        != *(*level.offset(*factlev.offset(s as isize) as isize))
                                            .offset(c as isize)
                                    {
                                        if nextcell == basecell && *used.offset(c as isize) == 0 {
                                            nextcell = c;
                                        }
                                        same = 0 as libc::c_int;
                                        break;
                                    } else {
                                        s += 1;
                                        s;
                                    }
                                }
                                if same != 0 {
                                    break;
                                }
                                c += 1;
                                c;
                            }
                        }
                        r = 0 as libc::c_int;
                        while r < neffrows {
                            if *indep.offset(r as isize) != 0 {
                                *cum.offset(r as isize) *= *cum.offset(r as isize);
                                *emsc.offset(r as isize) += *cum.offset(r as isize);
                            }
                            r += 1;
                            r;
                        }
                        if nextcell == basecell {
                            break;
                        }
                        basecell = nextcell;
                    }
                    r = 0 as libc::c_int;
                    while r < neffrows {
                        if *indep.offset(r as isize) != 0 {
                            *emsc.offset(r as isize) /= *lensq.offset(r as isize);
                        }
                        r += 1;
                        r;
                    }
                    r = 0 as libc::c_int;
                    *(*emscoeff.offset(factor as isize)).offset(u as isize) = 0.0f64;
                    df = 0.0f64;
                    while r < neffrows {
                        if *indep.offset(r as isize) != 0 {
                            *(*emscoeff.offset(factor as isize)).offset(u as isize)
                                += *emsc.offset(r as isize);
                            df += 1.0f64;
                        }
                        r += 1;
                        r;
                    }
                    if df > 0.0f64 {
                        *(*emscoeff.offset(factor as isize)).offset(u as isize) /= df;
                        if !varv.is_null() {
                            putrand(
                                u,
                                *(*emscoeff.offset(factor as isize)).offset(u as isize),
                                varv,
                            );
                        }
                    } else {
                        *(*emscoeff.offset(factor as isize)).offset(u as isize) = 0.0f64;
                    }
                } else {
                    *(*emscoeff.offset(factor as isize)).offset(u as isize) = 0.0f64;
                }
                u += 1;
                u;
            }
            if !varv.is_null() {
                fputs(
                    b"\n    1 Var(Error)\n\0" as *const u8 as *const libc::c_char,
                    dap_lst,
                );
            }
            factor += 1;
            factor;
        }
        t += 1;
        t;
    }
    dap_free(
        cum as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        effrow as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        factlev as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        emsc as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        lensq as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        used as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return factor;
}
unsafe extern "C" fn putms(
    mut bits: libc::c_int,
    mut coeff: libc::c_double,
    mut varv: *mut libc::c_int,
) {
    let mut v: libc::c_int = 0;
    let mut termn: libc::c_int = 0;
    if coeff != 0.0f64 {
        fprintf(dap_lst, b"\n    %g MS(\0" as *const u8 as *const libc::c_char, coeff);
    }
    if bits != 0 {
        v = 1 as libc::c_int;
        termn = 1 as libc::c_int;
        while bits != 0 {
            if bits & 0x1 as libc::c_int != 0 {
                if termn > 1 as libc::c_int {
                    putc('*' as i32, dap_lst);
                }
                fputs(
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                        .offset(*varv.offset(v as isize) as isize),
                    dap_lst,
                );
                termn += 1;
                termn;
            }
            bits = bits >> 1 as libc::c_int;
            v += 1;
            v;
        }
    } else if coeff != 0.0f64 {
        fputs(b"Error\0" as *const u8 as *const libc::c_char, dap_lst);
    }
    if coeff != 0.0f64 {
        putc(')' as i32, dap_lst);
    }
}
unsafe extern "C" fn emssolve(
    mut emscoeff: *mut *mut libc::c_double,
    mut nterms: libc::c_int,
    mut nfactors: libc::c_int,
    mut varv: *mut libc::c_int,
    mut termv: *mut libc::c_int,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut maxr: libc::c_int = 0;
    let mut subr: libc::c_int = 0;
    let mut subc: libc::c_int = 0;
    let mut rowmax: libc::c_double = 0.;
    let mut colmax: libc::c_double = 0.;
    let mut maxmax: libc::c_double = 0.;
    let mut tmp: libc::c_double = 0.;
    let mut itmp: libc::c_int = 0;
    let mut mult: libc::c_double = 0.;
    *(*emscoeff.offset(0 as libc::c_int as isize))
        .offset(0 as libc::c_int as isize) = 1.0f64;
    *termv.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    r = 1 as libc::c_int;
    while r <= nterms {
        *(*emscoeff.offset(0 as libc::c_int as isize)).offset(r as isize) = 0.0f64;
        *termv.offset(r as isize) = r;
        r += 1;
        r;
    }
    c = 1 as libc::c_int;
    r = 1 as libc::c_int;
    while c <= nfactors {
        colmax = 0.0f64;
        maxmax = 0.0f64;
        maxr = r;
        subr = r;
        while subr <= nterms {
            subc = c;
            rowmax = 0.0f64;
            while subc <= nfactors {
                tmp = fabs(*(*emscoeff.offset(subc as isize)).offset(subr as isize));
                if tmp > rowmax {
                    rowmax = tmp;
                }
                subc += 1;
                subc;
            }
            tmp = fabs(*(*emscoeff.offset(c as isize)).offset(subr as isize) / rowmax);
            if tmp > colmax {
                maxmax = rowmax;
                colmax = tmp;
                maxr = subr;
            }
            subr += 1;
            subr;
        }
        if colmax > dap_redtol * maxmax {
            if maxr != r {
                subc = 0 as libc::c_int;
                while subc <= nfactors + 1 as libc::c_int {
                    tmp = *(*emscoeff.offset(subc as isize)).offset(r as isize);
                    *(*emscoeff.offset(subc as isize))
                        .offset(
                            r as isize,
                        ) = *(*emscoeff.offset(subc as isize)).offset(maxr as isize);
                    *(*emscoeff.offset(subc as isize)).offset(maxr as isize) = tmp;
                    subc += 1;
                    subc;
                }
                itmp = *termv.offset(r as isize);
                *termv.offset(r as isize) = *termv.offset(maxr as isize);
                *termv.offset(maxr as isize) = itmp;
            }
            subr = 0 as libc::c_int;
            while subr <= nterms {
                if subr == r {
                    mult = *(*emscoeff.offset(c as isize)).offset(subr as isize);
                    subc = c;
                    while subc <= nfactors + 1 as libc::c_int {
                        *(*emscoeff.offset(subc as isize)).offset(subr as isize) /= mult;
                        subc += 1;
                        subc;
                    }
                } else {
                    mult = *(*emscoeff.offset(c as isize)).offset(subr as isize)
                        / *(*emscoeff.offset(c as isize)).offset(r as isize);
                    if fabs(mult) > dap_redtol * maxmax {
                        subc = c;
                        while subc <= nfactors + 1 as libc::c_int {
                            tmp = fabs(
                                *(*emscoeff.offset(subc as isize)).offset(subr as isize),
                            );
                            *(*emscoeff.offset(subc as isize)).offset(subr as isize)
                                -= mult
                                    * *(*emscoeff.offset(subc as isize)).offset(r as isize);
                            if fabs(
                                *(*emscoeff.offset(subc as isize)).offset(subr as isize),
                            ) < dap_redtol * tmp
                            {
                                *(*emscoeff.offset(subc as isize))
                                    .offset(subr as isize) = 0.0f64;
                            }
                            subc += 1;
                            subc;
                        }
                    }
                }
                subr += 1;
                subr;
            }
            r += 1;
            r;
        }
        c += 1;
        c;
    }
    return r;
}
unsafe extern "C" fn ftest1(
    mut coeff: *mut *mut libc::c_double,
    mut level: *mut *mut libc::c_int,
    mut ncells: libc::c_int,
    mut rterm: *mut libc::c_int,
    mut ncontrasts: libc::c_int,
    mut nerrors: libc::c_int,
    mut mean: *mut libc::c_double,
    mut vari: *mut libc::c_double,
    mut nobs: *mut libc::c_double,
    mut varv: *mut libc::c_int,
    mut numv: *mut libc::c_char,
    mut nnum: libc::c_int,
    mut denv: *mut libc::c_char,
    mut nden: libc::c_int,
    mut typen: libc::c_int,
) {
    let mut corow: libc::c_int = 0;
    let mut cc: libc::c_int = 0;
    let mut cr: libc::c_int = 0;
    let mut numer: libc::c_double = 0.;
    let mut sq: libc::c_double = 0.;
    let mut varnce: libc::c_double = 0.;
    let mut tmp: libc::c_double = 0.;
    let mut denom: libc::c_double = 0.;
    let mut modelss: libc::c_double = 0.;
    let mut n: libc::c_double = 0.;
    let mut cdfi: libc::c_int = 0;
    let mut edfi: libc::c_int = 0;
    let mut dedfi: libc::c_double = 0.;
    let mut indep: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut sse: libc::c_double = 0.;
    let mut dfe: libc::c_int = 0;
    let mut emsmem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut emscoeff: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut nfactors: libc::c_int = 0;
    let mut termv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ndenterm: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut ss: libc::c_double = 0.;
    let mut ms: libc::c_double = 0.;
    let mut df: libc::c_double = 0.;
    let mut dfdown: libc::c_double = 0.;
    let mut dfup: libc::c_double = 0.;
    let mut fdown: libc::c_double = 0.;
    let mut fup: libc::c_double = 0.;
    dedfi = 0.0f64;
    emsmem = 0 as *mut libc::c_double;
    emscoeff = 0 as *mut *mut libc::c_double;
    ndenterm = 0 as libc::c_int;
    ss = 0.0f64;
    df = 0.0f64;
    sumcheck(
        b"ftest1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        coeff,
        ncells,
        nerrors,
        ncontrasts,
        rterm,
    );
    indep = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((nerrors + ncontrasts) as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    if nden != 0 {
        emsmem = dap_malloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(ncells as libc::c_ulong)
                .wrapping_mul(ncells as libc::c_ulong) as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_double;
        emscoeff = dap_malloc(
            (::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(ncells as libc::c_ulong) as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut *mut libc::c_double;
        cr = 0 as libc::c_int;
        while cr < 2 as libc::c_int * ncells {
            let ref mut fresh9 = *emscoeff.offset(cr as isize);
            *fresh9 = emsmem.offset((cr * ncells) as isize);
            cr += 1;
            cr;
        }
    }
    termv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(ncells as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    denom = 0.0f64;
    cr = 0 as libc::c_int;
    while cr < nerrors + ncontrasts {
        *indep.offset(cr as isize) = 0 as libc::c_int;
        cr += 1;
        cr;
    }
    nfactors = 0 as libc::c_int;
    if nerrors != 0 {
        if nden != 0 {
            edfi = orthog(
                coeff,
                0 as libc::c_int,
                nerrors - 1 as libc::c_int,
                nerrors - 1 as libc::c_int,
                ncells,
                nobs,
                indep,
                rterm,
            );
            nfactors = ems(
                coeff,
                level,
                ncells,
                rterm,
                indep,
                nerrors + ncontrasts,
                nobs,
                varv,
                denv,
                nden,
                emscoeff,
            );
        } else {
            edfi = nerrors;
            cr = 0 as libc::c_int;
            while cr < nerrors {
                cc = 0 as libc::c_int;
                sq = 0.0f64;
                varnce = 0.0f64;
                while cc < ncells {
                    tmp = *(*coeff.offset(cr as isize)).offset(cc as isize);
                    sq += tmp * *mean.offset(cc as isize);
                    varnce += tmp * tmp / *nobs.offset(cc as isize);
                    cc += 1;
                    cc;
                }
                denom += sq * sq / varnce;
                cr += 1;
                cr;
            }
        }
    } else {
        edfi = 0 as libc::c_int;
    }
    corow = nerrors + ncontrasts - 1 as libc::c_int;
    cc = 0 as libc::c_int;
    n = 0.0f64;
    sse = 0.0f64;
    while cc < ncells {
        if *nobs.offset(cc as isize) > 0.0f64 {
            sse += (*nobs.offset(cc as isize) - 1.0f64) * *vari.offset(cc as isize);
            n += *nobs.offset(cc as isize);
        }
        cc += 1;
        cc;
    }
    dfe = n as libc::c_int - ncells;
    if nden != 0 {
        orthog(
            coeff,
            nerrors,
            nerrors,
            corow,
            ncells,
            nobs,
            indep,
            0 as *mut libc::c_int,
        );
        cdfi = orthog(coeff, nerrors, corow, corow, ncells, nobs, indep, rterm);
        if ems(
            coeff,
            level,
            ncells,
            rterm,
            indep,
            nerrors + ncontrasts,
            nobs,
            varv,
            numv,
            nnum,
            emscoeff.offset(nfactors as isize),
        ) != 1 as libc::c_int
        {
            fputs(
                b"(ftest1) Only one one term allowed in numerator of F-test with denominator\n\0"
                    as *const u8 as *const libc::c_char,
                dap_lst,
            );
            exit(1 as libc::c_int);
        }
        ndenterm = emssolve(emscoeff, nden, nfactors, varv, termv);
        fputs(b"Error for \0" as *const u8 as *const libc::c_char, dap_lst);
        putms(*rterm.offset(nerrors as isize), 0.0f64, varv);
        fputs(b" =\0" as *const u8 as *const libc::c_char, dap_lst);
        t = 0 as libc::c_int;
        denom = 0.0f64;
        dedfi = 0.0f64;
        while t < ndenterm {
            if *termv.offset(t as isize) != *rterm.offset(nerrors as isize)
                && *(*emscoeff.offset((nfactors + 1 as libc::c_int) as isize))
                    .offset(t as isize) != 0.
            {
                putms(
                    *termv.offset(t as isize),
                    *(*emscoeff.offset((nfactors + 1 as libc::c_int) as isize))
                        .offset(t as isize),
                    varv,
                );
                if *termv.offset(t as isize) != 0 {
                    cr = 0 as libc::c_int;
                    ss = 0.0f64;
                    df = 0.0f64;
                    while cr < nerrors + ncontrasts {
                        if *rterm.offset(cr as isize) == *termv.offset(t as isize)
                            && *indep.offset(cr as isize) != 0
                        {
                            cc = 0 as libc::c_int;
                            sq = 0.0f64;
                            varnce = 0.0f64;
                            while cc < ncells {
                                tmp = *(*coeff.offset(cr as isize)).offset(cc as isize);
                                sq += tmp * *mean.offset(cc as isize);
                                varnce += tmp * tmp / *nobs.offset(cc as isize);
                                cc += 1;
                                cc;
                            }
                            ss += sq * sq / varnce;
                            df += 1.0f64;
                        }
                        cr += 1;
                        cr;
                    }
                } else {
                    ss = sse;
                    df = dfe as libc::c_double;
                }
                if ndenterm > 1 as libc::c_int {
                    ms = ss / df;
                    tmp = *(*emscoeff.offset((nfactors + 1 as libc::c_int) as isize))
                        .offset(t as isize) * ms;
                    denom += tmp;
                    dedfi += tmp * tmp / df;
                }
            }
            t += 1;
            t;
        }
        putc('\n' as i32, dap_lst);
        if ndenterm > 1 as libc::c_int {
            dedfi = denom * denom / dedfi;
        } else {
            denom = *(*emscoeff.offset(nfactors as isize)).offset(t as isize) * ss;
            edfi = df as libc::c_int;
        }
    } else {
        denom += sse;
        cdfi = orthog(
            coeff,
            0 as libc::c_int,
            corow,
            corow,
            ncells,
            nobs,
            indep,
            0 as *mut libc::c_int,
        ) - edfi;
        edfi += dfe;
    }
    cr = nerrors;
    numer = 0.0f64;
    while cr <= corow {
        if *indep.offset(cr as isize) != 0 {
            cc = 0 as libc::c_int;
            sq = 0.0f64;
            varnce = 0.0f64;
            while cc < ncells {
                tmp = *(*coeff.offset(cr as isize)).offset(cc as isize);
                sq += tmp * *mean.offset(cc as isize);
                varnce += tmp * tmp / *nobs.offset(cc as isize);
                cc += 1;
                cc;
            }
            numer += sq * sq / varnce;
        }
        cr += 1;
        cr;
    }
    fprintf(
        dap_lst,
        b"Number of observations = %d\n\0" as *const u8 as *const libc::c_char,
        n as libc::c_int,
    );
    fprintf(
        dap_lst,
        b"H0 SS = %g, df = %d, MS = %g\n\0" as *const u8 as *const libc::c_char,
        numer,
        cdfi,
        numer / cdfi as libc::c_double,
    );
    modelss = numer;
    numer /= cdfi as libc::c_double;
    if nerrors != 0 {
        fputs(b"Residual \0" as *const u8 as *const libc::c_char, dap_lst);
    } else {
        fputs(b"Error \0" as *const u8 as *const libc::c_char, dap_lst);
    }
    if nden != 0 && ndenterm > 1 as libc::c_int {
        fprintf(
            dap_lst,
            b"df = %g, MS = %g\n\0" as *const u8 as *const libc::c_char,
            dedfi,
            denom,
        );
        numer /= denom;
        fprintf(dap_lst, b"F0 = %g\n\0" as *const u8 as *const libc::c_char, numer);
        dfdown = floor(dedfi);
        dfup = ceil(dedfi);
        if dfup == dfdown {
            fprintf(
                dap_lst,
                b"Prob[F > F0] = %.5f\n\0" as *const u8 as *const libc::c_char,
                0.00001f64 * ceil(100000.0f64 * probf(numer, cdfi, dedfi as libc::c_int)),
            );
        } else {
            fdown = probf(numer, cdfi, dfdown as libc::c_int);
            fup = probf(numer, cdfi, dfup as libc::c_int);
            fprintf(
                dap_lst,
                b"Prob[F > F0] = %.5f\n\0" as *const u8 as *const libc::c_char,
                0.00001f64
                    * ceil(
                        100000.0f64
                            * (fdown
                                + (dedfi - dfdown) / (dfup - dfdown) * (fup - fdown)),
                    ),
            );
        }
    } else {
        dedfi = edfi as libc::c_double;
        fprintf(
            dap_lst,
            b"SS = %g, df = %d, MS = %g\n\0" as *const u8 as *const libc::c_char,
            denom,
            edfi,
            denom / dedfi,
        );
        if ncontrasts + nerrors == ncells - 1 as libc::c_int {
            fprintf(
                dap_lst,
                b"R-sq = %g\n\0" as *const u8 as *const libc::c_char,
                modelss / (modelss + denom),
            );
        }
        denom /= dedfi;
        numer /= denom;
        fprintf(
            dap_lst,
            b"F0 = %g\nProb[F > F0] = %.5f\n\0" as *const u8 as *const libc::c_char,
            numer,
            0.00001f64 * ceil(100000.0f64 * probf(numer, cdfi, edfi)),
        );
    }
    strcpy(
        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
            .offset(typen as isize),
        b"MSERROR\0" as *const u8 as *const libc::c_char,
    );
    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
        .offset(*varv.offset(0 as libc::c_int as isize) as isize) = denom;
    output();
    strcpy(
        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
            .offset(typen as isize),
        b"ERRORDF\0" as *const u8 as *const libc::c_char,
    );
    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
        .offset(*varv.offset(0 as libc::c_int as isize) as isize) = dedfi;
    output();
    if nden != 0 {
        dap_free(
            emsmem as *mut libc::c_void,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        dap_free(
            emscoeff as *mut libc::c_void,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    dap_free(
        indep as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        termv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn puttest(
    mut testv: *mut libc::c_char,
    mut ntest: libc::c_int,
    mut varv: *mut libc::c_int,
    mut nvars: libc::c_int,
) {
    let mut t: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    let mut first: libc::c_int = 0;
    t = 1 as libc::c_int;
    while t <= ntest {
        if *testv.offset(t as isize) as libc::c_int == 'c' as i32
            || *testv.offset(t as isize) as libc::c_int == 'n' as i32
        {
            bits = t;
            v = 1 as libc::c_int;
            first = 1 as libc::c_int;
            while v < nvars {
                if bits & 0x1 as libc::c_int != 0 {
                    if first != 0 {
                        putc(' ' as i32, dap_lst);
                        first = 0 as libc::c_int;
                    } else {
                        putc('*' as i32, dap_lst);
                    }
                    fprintf(
                        dap_lst,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_nam)
                            .offset(*varv.offset(v as isize) as isize),
                    );
                }
                bits = bits >> 1 as libc::c_int;
                v += 1;
                v;
            }
        }
        t += 1;
        t;
    }
    putc('\n' as i32, dap_lst);
}
pub unsafe extern "C" fn ftest(
    mut fname: *mut libc::c_char,
    mut variables: *mut libc::c_char,
    mut numerator: *mut libc::c_char,
    mut denominator: *mut libc::c_char,
    mut marks: *mut libc::c_char,
) {
    let mut tstname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut typen: libc::c_int = 0;
    let mut termn: libc::c_int = 0;
    let mut varv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nvars: libc::c_int = 0;
    let mut markv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nmark: libc::c_int = 0;
    let mut rterm: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nnum: libc::c_int = 0;
    let mut nden: libc::c_int = 0;
    let mut numv: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut denv: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut num: libc::c_int = 0;
    let mut den: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut morecells: libc::c_int = 0;
    let mut more: libc::c_int = 0;
    let mut statn: libc::c_int = 0;
    let mut gotm: libc::c_int = 0;
    let mut gotn: libc::c_int = 0;
    let mut gotv: libc::c_int = 0;
    let mut levmem: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut levptr: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut levval: *mut *mut *mut libc::c_char = 0 as *mut *mut *mut libc::c_char;
    let mut nlevels: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut levelmem: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut level: *mut *mut libc::c_int = 0 as *mut *mut libc::c_int;
    let mut v: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut comem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut coeff: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut mean: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut nobs: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut vari: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut ncells: libc::c_int = 0;
    let mut ncontrasts: libc::c_int = 0;
    let mut nerrors: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut con: libc::c_int = 0;
    varv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((dap_maxtreat + 1 as libc::c_int) as libc::c_ulong)
            as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    markv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    rterm = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((dap_maxcell - 1 as libc::c_int) as libc::c_ulong)
            as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    numv = dap_malloc(
        dap_maxcell,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    denv = dap_malloc(
        dap_maxcell,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    levmem = dap_malloc(
        (dap_maxtreat + 1 as libc::c_int) * dap_maxlev * (dap_strlen + 1 as libc::c_int),
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    levptr = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul((dap_maxtreat + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(dap_maxlev as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_char;
    levval = dap_malloc(
        (::std::mem::size_of::<*mut *mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul((dap_maxtreat + 1 as libc::c_int) as libc::c_ulong)
            as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut *mut libc::c_char;
    v = 0 as libc::c_int;
    while v < dap_maxtreat + 1 as libc::c_int {
        let ref mut fresh10 = *levval.offset(v as isize);
        *fresh10 = levptr.offset((v * dap_maxlev) as isize);
        l = 0 as libc::c_int;
        while l < dap_maxlev {
            let ref mut fresh11 = *(*levval.offset(v as isize)).offset(l as isize);
            *fresh11 = levmem
                .offset((v * dap_maxlev * (dap_strlen + 1 as libc::c_int)) as isize)
                .offset((l * (dap_strlen + 1 as libc::c_int)) as isize);
            l += 1;
            l;
        }
        v += 1;
        v;
    }
    nlevels = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((dap_maxtreat + 1 as libc::c_int) as libc::c_ulong)
            as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    levelmem = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((dap_maxtreat + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(dap_maxcell as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    level = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_int>() as libc::c_ulong)
            .wrapping_mul((dap_maxtreat + 1 as libc::c_int) as libc::c_ulong)
            as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_int;
    v = 0 as libc::c_int;
    while v < dap_maxtreat + 1 as libc::c_int {
        let ref mut fresh12 = *level.offset(v as isize);
        *fresh12 = levelmem.offset((v * dap_maxcell) as isize);
        v += 1;
        v;
    }
    comem = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxcell as libc::c_ulong)
            .wrapping_mul(dap_maxcell as libc::c_ulong) as libc::c_int,
        b"dap_maxcell\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    coeff = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxcell as libc::c_ulong) as libc::c_int,
        b"dap_maxcell\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_double;
    ncells = 0 as libc::c_int;
    while ncells < dap_maxcell {
        let ref mut fresh13 = *coeff.offset(ncells as isize);
        *fresh13 = comem.offset((ncells * dap_maxcell) as isize);
        ncells += 1;
        ncells;
    }
    mean = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxcell as libc::c_ulong) as libc::c_int,
        b"dap_maxcell\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    nobs = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxcell as libc::c_ulong) as libc::c_int,
        b"dap_maxcell\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    vari = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxcell as libc::c_ulong) as libc::c_int,
        b"dap_maxcell\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    if fname.is_null() {
        fputs(
            b"(ftest) No dataset name given.\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    tstname = dap_malloc(
        (strlen(fname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_suffix(
        tstname,
        fname,
        b"<tst\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    inset(fname);
    typen = dap_varnum(
        b"_type_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if typen < 0 as libc::c_int {
        fprintf(
            dap_err,
            b"(ftest) no _type_ variable\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    termn = dap_varnum(
        b"_term_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if termn < 0 as libc::c_int {
        fprintf(
            dap_err,
            b"(ftest) no _term_ variable\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if variables.is_null() || *variables.offset(0 as libc::c_int as isize) == 0 {
        fputs(
            b"(ftest) No variables given.\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    nvars = dap_list(variables, varv, dap_maxtreat + 1 as libc::c_int);
    if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
        .offset(*varv.offset(0 as libc::c_int as isize) as isize) != -(1 as libc::c_int)
    {
        fprintf(
            dap_err,
            b"(ftest) response variable %s must be of type double\n\0" as *const u8
                as *const libc::c_char,
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                .offset(*varv.offset(0 as libc::c_int as isize) as isize),
        );
        exit(1 as libc::c_int);
    }
    v = 1 as libc::c_int;
    while v < nvars {
        if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
            .offset(*varv.offset(v as isize) as isize) <= 0 as libc::c_int
        {
            fprintf(
                dap_err,
                b"(ftest) classification variable %s must be string\n\0" as *const u8
                    as *const libc::c_char,
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(*varv.offset(v as isize) as isize),
            );
            exit(1 as libc::c_int);
        }
        v += 1;
        v;
    }
    outset(tstname, b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if !numerator.is_null()
        && *numerator.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        nnum = testparse(numerator, numv, varv, nvars);
        num = 1 as libc::c_int;
    } else {
        num = 0 as libc::c_int;
        nnum = 0 as libc::c_int;
        t = 1 as libc::c_int;
        while t < dap_maxcell {
            *numv.offset(t as isize) = 'e' as i32 as libc::c_char;
            t += 1;
            t;
        }
    }
    if !denominator.is_null()
        && *denominator.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        nden = testparse(denominator, denv, varv, nvars);
        t = 1 as libc::c_int;
        while t <= nnum {
            if *numv.offset(t as isize) as libc::c_int == 'c' as i32 {
                *denv.offset(t as isize) = 'e' as i32 as libc::c_char;
            }
            t += 1;
            t;
        }
        t = 1 as libc::c_int;
        while t <= nden {
            if *denv.offset(t as isize) as libc::c_int != 'e' as i32 {
                den = 1 as libc::c_int;
                break;
            } else {
                t += 1;
                t;
            }
        }
        if t > nden {
            den = 0 as libc::c_int;
            nden = 0 as libc::c_int;
            while t < dap_maxcell {
                let fresh14 = t;
                t = t + 1;
                *denv.offset(fresh14 as isize) = 'e' as i32 as libc::c_char;
            }
        }
    } else {
        den = 0 as libc::c_int;
        nden = 0 as libc::c_int;
        t = 1 as libc::c_int;
        while t < dap_maxcell {
            *denv.offset(t as isize) = 'e' as i32 as libc::c_char;
            t += 1;
            t;
        }
    }
    v = 1 as libc::c_int;
    while v < nvars {
        *nlevels.offset(v as isize) = 0 as libc::c_int;
        l = 0 as libc::c_int;
        while l < dap_maxlev {
            *(*(*levval.offset(v as isize)).offset(l as isize))
                .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            l += 1;
            l;
        }
        v += 1;
        v;
    }
    nmark = dap_list(marks, markv, dap_maxvar);
    ncells = 0 as libc::c_int;
    ncontrasts = 0 as libc::c_int;
    nerrors = 0 as libc::c_int;
    more = step();
    morecells = 1 as libc::c_int;
    while morecells != 0 {
        gotn = 0 as libc::c_int;
        gotm = 0 as libc::c_int;
        gotv = 0 as libc::c_int;
        morecells = more;
        if dap_newpart(markv, nmark) != 0 {
            dap_swap();
            dap_head(markv, nmark);
            if num != 0 {
                fprintf(
                    dap_lst,
                    b"Testing Ho: %s\n\0" as *const u8 as *const libc::c_char,
                    numerator,
                );
            } else if nnum > 0 as libc::c_int {
                fputs(b"Testing Ho:\0" as *const u8 as *const libc::c_char, dap_lst);
                puttest(numv, nnum, varv, nvars);
            }
            if den != 0 {
                fprintf(
                    dap_lst,
                    b"Denominator: %s\n\0" as *const u8 as *const libc::c_char,
                    denominator,
                );
            } else if nden > 0 as libc::c_int {
                fputs(b"Denominator:\0" as *const u8 as *const libc::c_char, dap_lst);
                puttest(denv, nden, varv, nvars);
            }
            ftest1(
                coeff,
                level,
                ncells,
                rterm,
                ncontrasts,
                nerrors,
                mean,
                vari,
                nobs,
                varv,
                numv,
                num * nnum,
                denv,
                den * nden,
                typen,
            );
            dap_swap();
            v = 1 as libc::c_int;
            while v < nvars {
                *nlevels.offset(v as isize) = 0 as libc::c_int;
                l = 0 as libc::c_int;
                while l < dap_maxlev {
                    *(*(*levval.offset(v as isize)).offset(l as isize))
                        .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                    l += 1;
                    l;
                }
                v += 1;
                v;
            }
            ncells = 0 as libc::c_int;
            ncontrasts = 0 as libc::c_int;
            nerrors = 0 as libc::c_int;
        }
        statn = 0 as libc::c_int;
        while more != 0 {
            if strcmp(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                b"N\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                *nobs
                    .offset(
                        ncells as isize,
                    ) = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .do_dbl)
                    .offset(*varv.offset(0 as libc::c_int as isize) as isize);
                gotn = 1 as libc::c_int;
                output();
            } else if strcmp(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                b"MEAN\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                *mean
                    .offset(
                        ncells as isize,
                    ) = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .do_dbl)
                    .offset(*varv.offset(0 as libc::c_int as isize) as isize);
                gotm = 1 as libc::c_int;
                output();
            } else if strcmp(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                b"VAR\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                *vari
                    .offset(
                        ncells as isize,
                    ) = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .do_dbl)
                    .offset(*varv.offset(0 as libc::c_int as isize) as isize);
                gotv = 1 as libc::c_int;
                output();
            } else {
                fprintf(
                    dap_err,
                    b"(ftest) Bad cell statistic: %s\n\0" as *const u8
                        as *const libc::c_char,
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                        .offset(typen as isize),
                );
                exit(1 as libc::c_int);
            }
            statn += 1;
            if statn < 3 as libc::c_int {
                more = step();
            } else {
                if *nobs.offset(ncells as isize) == 1.0f64 {
                    *vari.offset(ncells as isize) = 0.0f64;
                }
                break;
            }
        }
        if more != 0 {
            if gotm == 0 {
                fputs(
                    b"(ftest) Missing MEAN.\n\0" as *const u8 as *const libc::c_char,
                    dap_err,
                );
                exit(1 as libc::c_int);
            }
            if gotn == 0 {
                fputs(
                    b"(ftest) Missing N.\n\0" as *const u8 as *const libc::c_char,
                    dap_err,
                );
                exit(1 as libc::c_int);
            }
            if gotv == 0 {
                fputs(
                    b"(ftest) Missing VAR.\n\0" as *const u8 as *const libc::c_char,
                    dap_err,
                );
                exit(1 as libc::c_int);
            }
            v = 1 as libc::c_int;
            while v < nvars {
                *(*level.offset(v as isize))
                    .offset(
                        ncells as isize,
                    ) = levn(
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                        .offset(*varv.offset(v as isize) as isize),
                    *levval.offset(v as isize),
                    &mut *nlevels.offset(v as isize),
                );
                v += 1;
                v;
            }
        }
        dap_mark();
        nerrors = 0 as libc::c_int;
        while more != 0 {
            more = step();
            err = (strcmp(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                b"ERROR\0" as *const u8 as *const libc::c_char,
            ) == 0) as libc::c_int;
            con = (strcmp(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                b"CONTR\0" as *const u8 as *const libc::c_char,
            ) == 0) as libc::c_int;
            if den != 0 && err != 0 {
                dap_mark();
            }
            if err != 0 || den != 0 && con != 0 {
                t = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_int)
                    .offset(termn as isize);
                *rterm.offset(nerrors as isize) = t;
                if den == 0 {
                    *denv.offset(t as isize) = 'c' as i32 as libc::c_char;
                    if nden < t {
                        nden = t;
                    }
                }
                if *denv.offset(t as isize) as libc::c_int == 'c' as i32
                    || *denv.offset(t as isize) as libc::c_int == 'n' as i32
                {
                    *(*coeff.offset(nerrors as isize))
                        .offset(
                            ncells as isize,
                        ) = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                        .do_dbl)
                        .offset(*varv.offset(0 as libc::c_int as isize) as isize);
                    nerrors += 1;
                    nerrors;
                    output();
                }
            } else {
                if den != 0 {
                    dap_rewind();
                    more = step();
                }
                break;
            }
        }
        ncontrasts = 0 as libc::c_int;
        while more != 0 {
            if !(strcmp(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                b"CONTR\0" as *const u8 as *const libc::c_char,
            ) == 0)
            {
                break;
            }
            t = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_int)
                .offset(termn as isize);
            *rterm.offset((nerrors + ncontrasts) as isize) = t;
            if num == 0 {
                *numv.offset(t as isize) = 'c' as i32 as libc::c_char;
                if nnum < t {
                    nnum = t;
                }
            }
            if *numv.offset(t as isize) as libc::c_int == 'c' as i32
                || *numv.offset(t as isize) as libc::c_int == 'n' as i32
            {
                *(*coeff.offset((nerrors + ncontrasts) as isize))
                    .offset(
                        ncells as isize,
                    ) = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .do_dbl)
                    .offset(*varv.offset(0 as libc::c_int as isize) as isize);
                ncontrasts += 1;
                ncontrasts;
            }
            more = step();
        }
        while more != 0 {
            if !(strcmp(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                b"LSMEAN\0" as *const u8 as *const libc::c_char,
            ) == 0)
            {
                break;
            }
            t = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_int)
                .offset(termn as isize);
            if t <= nnum
                && (*numv.offset(t as isize) as libc::c_int == 'c' as i32
                    || *numv.offset(t as isize) as libc::c_int == 'n' as i32)
            {
                output();
            }
            more = step();
        }
        ncells += 1;
        ncells;
    }
    dap_free(
        comem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        coeff as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        tstname as *mut libc::c_void,
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
        rterm as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        numv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        denv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        levmem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        levptr as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        levval as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        nlevels as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        levelmem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        level as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        mean as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        nobs as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        vari as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn putlev(
    mut nlevels: *mut libc::c_int,
    mut varv: *mut libc::c_int,
    mut nvars: libc::c_int,
    mut levval: *mut *mut *mut libc::c_char,
) {
    let mut v: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    fprintf(
        dap_lst,
        b"Response variable: %s\n\n\0" as *const u8 as *const libc::c_char,
        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
            .offset(*varv.offset(0 as libc::c_int as isize) as isize),
    );
    fprintf(
        dap_lst,
        b"%-15s Levels\n\0" as *const u8 as *const libc::c_char,
        b"Treatment\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        dap_lst,
        b"%-15s ------\n\0" as *const u8 as *const libc::c_char,
        b"--------\0" as *const u8 as *const libc::c_char,
    );
    v = 1 as libc::c_int;
    while v < nvars {
        fprintf(
            dap_lst,
            b"%-15s\0" as *const u8 as *const libc::c_char,
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                .offset(*varv.offset(v as isize) as isize),
        );
        l = 0 as libc::c_int;
        while l < *nlevels.offset(v as isize) {
            fprintf(
                dap_lst,
                b" %s\0" as *const u8 as *const libc::c_char,
                *(*levval.offset(v as isize)).offset(l as isize),
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
unsafe extern "C" fn maketerm(
    mut nterm: libc::c_int,
    mut termv: *mut libc::c_char,
    mut nvars: libc::c_int,
    mut varv: *mut libc::c_int,
    mut nlevels: *mut libc::c_int,
    mut coeff: *mut *mut libc::c_double,
    mut ncells: libc::c_int,
    mut nobs: *mut libc::c_double,
    mut rterm: *mut libc::c_int,
    mut nrows: *mut libc::c_int,
    mut clevel: *mut *mut libc::c_int,
) {
    let mut termtype: [libc::c_char; 4] = *::std::mem::transmute::<
        &[u8; 4],
        &mut [libc::c_char; 4],
    >(b"ecc\0");
    let mut tt: libc::c_int = 0;
    let mut tn: libc::c_int = 0;
    let mut nest: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut reset: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut iv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ivn: libc::c_int = 0;
    let mut ntreat: libc::c_int = 0;
    let mut rlevel: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut bits: libc::c_int = 0;
    let mut prod: libc::c_double = 0.;
    let mut vn: libc::c_int = 0;
    let mut nextr: libc::c_int = 0;
    let mut change: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nbits: libc::c_int = 0;
    row = 0 as libc::c_int;
    nest = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(ncells as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    iv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((dap_maxtreat + 1 as libc::c_int) as libc::c_ulong)
            as libc::c_int,
        b"dap_maxtreat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    rlevel = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((dap_maxtreat + 1 as libc::c_int) as libc::c_ulong)
            as libc::c_int,
        b"dap_maxtreat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    change = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((dap_maxtreat + 1 as libc::c_int) as libc::c_ulong)
            as libc::c_int,
        b"dap_maxtreat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    tn = 1 as libc::c_int;
    while tn <= nterm {
        *nest.offset(tn as isize) = tn;
        tn += 1;
        tn;
    }
    tn = 0x1 as libc::c_int;
    while tn <= nterm {
        if *termv.offset(tn as isize) as libc::c_int == 'e' as i32 {
            tt = 1 as libc::c_int;
            bits = 0 as libc::c_int;
            while tt <= nterm {
                if tt != tn && tt & tn == tn
                    && *termv.offset(tt as isize) as libc::c_int == 'c' as i32
                {
                    if bits != 0 {
                        bits &= tt;
                    } else {
                        bits = tt;
                    }
                }
                tt += 1;
                tt;
            }
            if bits != 0 && bits != tn {
                tt = 1 as libc::c_int;
                while tt <= nterm {
                    if tt & tn == tn {
                        if *termv.offset(tt as isize) as libc::c_int == 'e' as i32 {
                            nbits = tt | bits;
                            *termv.offset(tt as isize) = 'n' as i32 as libc::c_char;
                            *nest.offset(tt as isize) = nbits;
                        }
                    }
                    tt += 1;
                    tt;
                }
            }
        }
        tn = tn << 1 as libc::c_int;
    }
    vn = 1 as libc::c_int;
    c = 0 as libc::c_int;
    while vn < nvars {
        *(*clevel.offset(vn as isize)).offset(c as isize) = 0 as libc::c_int;
        vn += 1;
        vn;
    }
    c += 1;
    c;
    while c < ncells {
        vn = 1 as libc::c_int;
        while vn < nvars {
            *(*clevel.offset(vn as isize))
                .offset(
                    c as isize,
                ) = *(*clevel.offset(vn as isize))
                .offset((c - 1 as libc::c_int) as isize);
            vn += 1;
            vn;
        }
        vn = nvars - 1 as libc::c_int;
        while vn >= 0 as libc::c_int {
            let ref mut fresh15 = *(*clevel.offset(vn as isize)).offset(c as isize);
            *fresh15 += 1;
            if !(*fresh15 == *nlevels.offset(vn as isize)) {
                break;
            }
            *(*clevel.offset(vn as isize)).offset(c as isize) = 0 as libc::c_int;
            vn -= 1;
            vn;
        }
        c += 1;
        c;
    }
    tt = 0 as libc::c_int;
    r = 0 as libc::c_int;
    while tt < 3 as libc::c_int {
        reset = (tt < 2 as libc::c_int) as libc::c_int;
        tn = 1 as libc::c_int;
        *nrows.offset(tt as isize) = 0 as libc::c_int;
        while tn <= nterm {
            if *termv.offset(tn as isize) as libc::c_int
                == termtype[tt as usize] as libc::c_int
                || *termv.offset(tn as isize) as libc::c_int == 'n' as i32
                    && termtype[tt as usize] as libc::c_int == 'c' as i32
            {
                bits = tn;
                ntreat = 0 as libc::c_int;
                vn = 1 as libc::c_int;
                while vn < nvars {
                    if bits & 0x1 as libc::c_int != 0 {
                        let fresh16 = ntreat;
                        ntreat = ntreat + 1;
                        *iv.offset(fresh16 as isize) = vn;
                    }
                    vn += 1;
                    vn;
                    bits = bits >> 1 as libc::c_int;
                }
                c = 0 as libc::c_int;
                vn = nvars - 1 as libc::c_int;
                while vn > 0 as libc::c_int {
                    ivn = 0 as libc::c_int;
                    while ivn < ntreat {
                        *rlevel.offset(ivn as isize) = reset;
                        ivn += 1;
                        ivn;
                    }
                    row = 0 as libc::c_int;
                    ivn = 0 as libc::c_int;
                    while ivn < ntreat {
                        if tt < 2 as libc::c_int {
                            ivn = 0 as libc::c_int;
                            prod = 1.0f64;
                            while ivn < ntreat {
                                if !(*(*clevel.offset(*iv.offset(ivn as isize) as isize))
                                    .offset(c as isize) == 0 as libc::c_int)
                                {
                                    if *(*clevel.offset(*iv.offset(ivn as isize) as isize))
                                        .offset(c as isize) == *rlevel.offset(ivn as isize)
                                    {
                                        prod = -prod;
                                    } else {
                                        prod = 0.0f64;
                                        break;
                                    }
                                }
                                ivn += 1;
                                ivn;
                            }
                            *(*coeff.offset((r + row) as isize))
                                .offset(c as isize) = prod;
                            row += 1;
                            row;
                        } else if ntreat == 1 as libc::c_int {
                            if *(*clevel.offset(*iv.offset(ivn as isize) as isize))
                                .offset(c as isize) == *rlevel.offset(ivn as isize)
                            {
                                *(*coeff.offset((r + row) as isize))
                                    .offset(c as isize) = 1.0f64;
                            } else {
                                *(*coeff.offset((r + row) as isize))
                                    .offset(c as isize) = 0.0f64;
                            }
                            row += 1;
                            row;
                        }
                        ivn = 0 as libc::c_int;
                        while ivn < ntreat {
                            let ref mut fresh17 = *rlevel.offset(ivn as isize);
                            *fresh17 += 1;
                            if !(*fresh17
                                == *nlevels.offset(*iv.offset(ivn as isize) as isize))
                            {
                                break;
                            }
                            *rlevel.offset(ivn as isize) = reset;
                            ivn += 1;
                            ivn;
                        }
                    }
                    c += 1;
                    if c == ncells {
                        break;
                    }
                }
                nextr = r + row;
                while r < nextr {
                    *rterm.offset(r as isize) = *nest.offset(tn as isize);
                    r += 1;
                    r;
                    let ref mut fresh18 = *nrows.offset(tt as isize);
                    *fresh18 += 1;
                    *fresh18;
                }
            }
            tn += 1;
            tn;
        }
        tt += 1;
        tt;
    }
    dap_free(
        nest as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        iv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        rlevel as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        change as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn cmpstr(
    mut s1: *mut *mut libc::c_char,
    mut s2: *mut *mut libc::c_char,
) -> libc::c_int {
    return strcmp(*s1, *s2);
}
unsafe extern "C" fn eff1(
    mut incells: libc::c_int,
    mut levval: *mut *mut *mut libc::c_char,
    mut nlevels: *mut libc::c_int,
    mut varv: *mut libc::c_int,
    mut nvars: libc::c_int,
    mut termv: *mut libc::c_char,
    mut nterm: libc::c_int,
    mut typen: libc::c_int,
    mut termn: libc::c_int,
) {
    let mut comem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut coeff: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut v: libc::c_int = 0;
    let mut ncells: libc::c_int = 0;
    let mut celli: libc::c_int = 0;
    let mut level: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut clevmem: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut clevel: *mut *mut libc::c_int = 0 as *mut *mut libc::c_int;
    let mut statn: libc::c_int = 0;
    let mut nobs1: libc::c_double = 0.;
    let mut mean1: libc::c_double = 0.;
    let mut vari1: libc::c_double = 0.;
    let mut gotn: libc::c_int = 0;
    let mut gotm: libc::c_int = 0;
    let mut gotv: libc::c_int = 0;
    let mut nobs: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut mean: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut vari: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut celln: libc::c_int = 0;
    let mut sumlev: libc::c_int = 0;
    let mut miss: libc::c_int = 0;
    let mut rterm: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut indep: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nonz: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nrows: [libc::c_int; 3] = [0; 3];
    let mut r: libc::c_int = 0;
    let mut errow: libc::c_int = 0;
    let mut corow: libc::c_int = 0;
    let mut cr: libc::c_int = 0;
    let mut max: libc::c_double = 0.;
    let mut tmp: libc::c_double = 0.;
    let mut scmp: Option::<unsafe extern "C" fn() -> libc::c_int> = None;
    nobs1 = 0.0f64;
    mean1 = 0.0f64;
    vari1 = 0.0f64;
    scmp = ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut *mut libc::c_char,
                *mut *mut libc::c_char,
            ) -> libc::c_int,
        >,
        Option::<unsafe extern "C" fn() -> libc::c_int>,
    >(
        Some(
            cmpstr
                as unsafe extern "C" fn(
                    *mut *mut libc::c_char,
                    *mut *mut libc::c_char,
                ) -> libc::c_int,
        ),
    );
    level = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((dap_maxtreat + 1 as libc::c_int) as libc::c_ulong)
            as libc::c_int,
        b"dap_maxtreat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    v = 1 as libc::c_int;
    while v < nvars {
        qsort(
            *levval.offset(v as isize) as *mut libc::c_void,
            *nlevels.offset(v as isize) as size_t,
            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> libc::c_int>,
                __compar_fn_t,
            >(scmp),
        );
        v += 1;
        v;
    }
    v = 1 as libc::c_int;
    ncells = 1 as libc::c_int;
    sumlev = 0 as libc::c_int;
    while v < nvars {
        ncells *= *nlevels.offset(v as isize);
        sumlev += *nlevels.offset(v as isize);
        *level.offset(v as isize) = 0 as libc::c_int;
        v += 1;
        v;
    }
    clevmem = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(nvars as libc::c_ulong)
            .wrapping_mul(ncells as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    clevel = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_int>() as libc::c_ulong)
            .wrapping_mul(nvars as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_int;
    v = 0 as libc::c_int;
    while v < nvars {
        let ref mut fresh19 = *clevel.offset(v as isize);
        *fresh19 = clevmem.offset((v * ncells) as isize);
        v += 1;
        v;
    }
    nobs = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(ncells as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    mean = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(ncells as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    vari = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(ncells as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    rterm = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((ncells + sumlev - 1 as libc::c_int) as libc::c_ulong)
            as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    indep = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((ncells + sumlev - 1 as libc::c_int) as libc::c_ulong)
            as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    nonz = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((ncells + sumlev - 1 as libc::c_int) as libc::c_ulong)
            as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    comem = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul((ncells + sumlev) as libc::c_ulong)
            .wrapping_mul(ncells as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    coeff = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
            .wrapping_mul((ncells + sumlev) as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_double;
    r = 0 as libc::c_int;
    while r < ncells + sumlev {
        let ref mut fresh20 = *coeff.offset(r as isize);
        *fresh20 = comem.offset((r * ncells) as isize);
        r += 1;
        r;
    }
    celli = 0 as libc::c_int;
    celln = 0 as libc::c_int;
    while celli < incells {
        gotn = 0 as libc::c_int;
        gotm = 0 as libc::c_int;
        gotv = 0 as libc::c_int;
        statn = 0 as libc::c_int;
        while statn < 3 as libc::c_int {
            step();
            if strcmp(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                b"N\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                nobs1 = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .do_dbl)
                    .offset(*varv.offset(0 as libc::c_int as isize) as isize);
                gotn = 1 as libc::c_int;
            } else if strcmp(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                b"MEAN\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                mean1 = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .do_dbl)
                    .offset(*varv.offset(0 as libc::c_int as isize) as isize);
                gotm = 1 as libc::c_int;
            } else if strcmp(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                b"VAR\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                vari1 = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .do_dbl)
                    .offset(*varv.offset(0 as libc::c_int as isize) as isize);
                gotv = 1 as libc::c_int;
            } else {
                fprintf(
                    dap_err,
                    b"(eff1) Bad cell statistic: %s\n\0" as *const u8
                        as *const libc::c_char,
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                        .offset(typen as isize),
                );
                exit(1 as libc::c_int);
            }
            statn += 1;
            statn;
        }
        if gotn == 0 {
            fputs(b"(eff1) Missing N.\n\0" as *const u8 as *const libc::c_char, dap_err);
            exit(1 as libc::c_int);
        }
        if gotm == 0 {
            fputs(
                b"(eff1) Missing MEAN.\n\0" as *const u8 as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        if gotv == 0 {
            fputs(
                b"(eff1) Missing VAR.\n\0" as *const u8 as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        loop {
            v = 1 as libc::c_int;
            miss = 0 as libc::c_int;
            while v < nvars {
                if strcmp(
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                        .offset(*varv.offset(v as isize) as isize),
                    *(*levval.offset(v as isize))
                        .offset(*level.offset(v as isize) as isize),
                ) != 0
                {
                    miss = 1 as libc::c_int;
                    break;
                } else {
                    v += 1;
                    v;
                }
            }
            if miss != 0 {
                *nobs.offset(celln as isize) = 0.0f64;
                *mean.offset(celln as isize) = 0.0f64;
                *vari.offset(celln as isize) = 0.0f64;
                celln += 1;
                celln;
                v = nvars - 1 as libc::c_int;
                while v > 0 as libc::c_int {
                    let ref mut fresh21 = *level.offset(v as isize);
                    *fresh21 += 1;
                    if !(*fresh21 == *nlevels.offset(v as isize)) {
                        break;
                    }
                    *level.offset(v as isize) = 0 as libc::c_int;
                    v -= 1;
                    v;
                }
            }
            if !(miss != 0) {
                break;
            }
        }
        *nobs.offset(celln as isize) = nobs1;
        *mean.offset(celln as isize) = mean1;
        if nobs1 > 1.0f64 {
            *vari.offset(celln as isize) = vari1;
        } else {
            *vari.offset(celln as isize) = 0.0f64;
        }
        v = nvars - 1 as libc::c_int;
        while v > 0 as libc::c_int {
            let ref mut fresh22 = *level.offset(v as isize);
            *fresh22 += 1;
            if !(*fresh22 == *nlevels.offset(v as isize)) {
                break;
            }
            *level.offset(v as isize) = 0 as libc::c_int;
            v -= 1;
            v;
        }
        celli += 1;
        celli;
        celln += 1;
        celln;
    }
    while celln < ncells {
        *nobs.offset(celln as isize) = 0.0f64;
        *mean.offset(celln as isize) = 0.0f64;
        *vari.offset(celln as isize) = 0.0f64;
        celln += 1;
        celln;
    }
    maketerm(
        nterm,
        termv,
        nvars,
        varv,
        nlevels,
        coeff,
        ncells,
        nobs,
        rterm,
        nrows.as_mut_ptr(),
        clevel,
    );
    cr = 0 as libc::c_int;
    while cr
        < nrows[0 as libc::c_int as usize] + nrows[1 as libc::c_int as usize]
            + nrows[2 as libc::c_int as usize]
    {
        *indep.offset(cr as isize) = 0 as libc::c_int;
        cr += 1;
        cr;
    }
    if nrows[0 as libc::c_int as usize] != 0 {
        errow = rowred(
            coeff,
            rterm,
            0 as libc::c_int,
            ncells,
            nobs,
            nrows[0 as libc::c_int as usize] - 1 as libc::c_int,
            nrows[0 as libc::c_int as usize] - 1 as libc::c_int,
            0 as *mut libc::c_int,
        );
        if errow < nrows[0 as libc::c_int as usize] {
            orthog(
                coeff,
                nrows[0 as libc::c_int as usize] - 1 as libc::c_int,
                errow,
                0 as libc::c_int,
                ncells,
                nobs,
                indep,
                0 as *mut libc::c_int,
            );
        }
        corow = rowred(
            coeff,
            rterm,
            0 as libc::c_int,
            ncells,
            nobs,
            nrows[0 as libc::c_int as usize] - 1 as libc::c_int,
            nrows[0 as libc::c_int as usize] + nrows[1 as libc::c_int as usize]
                + nrows[2 as libc::c_int as usize] - 1 as libc::c_int,
            nonz,
        );
        if corow < nrows[0 as libc::c_int as usize] {
            corow = nrows[0 as libc::c_int as usize];
        }
    } else {
        errow = 0 as libc::c_int;
        corow = nrows[0 as libc::c_int as usize];
    }
    v = 1 as libc::c_int;
    while v < nvars {
        *level.offset(v as isize) = 0 as libc::c_int;
        v += 1;
        v;
    }
    r = 0 as libc::c_int;
    max = 0.0f64;
    while r
        < nrows[0 as libc::c_int as usize] + nrows[1 as libc::c_int as usize]
            + nrows[2 as libc::c_int as usize]
    {
        celln = 0 as libc::c_int;
        while celln < ncells {
            tmp = fabs(*(*coeff.offset(r as isize)).offset(celln as isize));
            if tmp > max {
                max = tmp;
            }
            celln += 1;
            celln;
        }
        r += 1;
        r;
    }
    r = 0 as libc::c_int;
    while r
        < nrows[0 as libc::c_int as usize] + nrows[1 as libc::c_int as usize]
            + nrows[2 as libc::c_int as usize]
    {
        celln = 0 as libc::c_int;
        while celln < ncells {
            if fabs(*(*coeff.offset(r as isize)).offset(celln as isize))
                < dap_zerotol * max
            {
                *(*coeff.offset(r as isize)).offset(celln as isize) = 0.0f64;
            }
            celln += 1;
            celln;
        }
        r += 1;
        r;
    }
    celln = 0 as libc::c_int;
    while celln < ncells {
        if *nobs.offset(celln as isize) != 0. {
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_int)
                .offset(termn as isize) = 0 as libc::c_int;
            v = 1 as libc::c_int;
            while v < nvars {
                strcpy(
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                        .offset(*varv.offset(v as isize) as isize),
                    *(*levval.offset(v as isize))
                        .offset(*level.offset(v as isize) as isize),
                );
                v += 1;
                v;
            }
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                b"N\0" as *const u8 as *const libc::c_char,
            );
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(
                    *varv.offset(0 as libc::c_int as isize) as isize,
                ) = *nobs.offset(celln as isize);
            output();
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                b"MEAN\0" as *const u8 as *const libc::c_char,
            );
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(
                    *varv.offset(0 as libc::c_int as isize) as isize,
                ) = *mean.offset(celln as isize);
            output();
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                b"VAR\0" as *const u8 as *const libc::c_char,
            );
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(
                    *varv.offset(0 as libc::c_int as isize) as isize,
                ) = *vari.offset(celln as isize);
            output();
            r = errow;
            while r < nrows[0 as libc::c_int as usize] {
                if *indep.offset(r as isize) != 0 {
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                        .offset(
                            *varv.offset(0 as libc::c_int as isize) as isize,
                        ) = *(*coeff.offset(r as isize)).offset(celln as isize);
                    strcpy(
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_str)
                            .offset(typen as isize),
                        b"ERROR\0" as *const u8 as *const libc::c_char,
                    );
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_int)
                        .offset(termn as isize) = *rterm.offset(r as isize);
                    output();
                }
                r += 1;
                r;
            }
            r = corow;
            while r < nrows[0 as libc::c_int as usize] + nrows[1 as libc::c_int as usize]
            {
                if nrows[0 as libc::c_int as usize] == 0 || *nonz.offset(r as isize) != 0
                {
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                        .offset(
                            *varv.offset(0 as libc::c_int as isize) as isize,
                        ) = *(*coeff.offset(r as isize)).offset(celln as isize);
                    strcpy(
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_str)
                            .offset(typen as isize),
                        b"CONTR\0" as *const u8 as *const libc::c_char,
                    );
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_int)
                        .offset(termn as isize) = *rterm.offset(r as isize);
                    output();
                }
                r += 1;
                r;
            }
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                b"LSMEAN\0" as *const u8 as *const libc::c_char,
            );
            while r
                < nrows[0 as libc::c_int as usize] + nrows[1 as libc::c_int as usize]
                    + nrows[2 as libc::c_int as usize]
            {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_int)
                    .offset(termn as isize) = *rterm.offset(r as isize);
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        *varv.offset(0 as libc::c_int as isize) as isize,
                    ) = *(*coeff.offset(r as isize)).offset(celln as isize);
                output();
                r += 1;
                r;
            }
        }
        v = nvars - 1 as libc::c_int;
        while v > 0 as libc::c_int {
            let ref mut fresh23 = *level.offset(v as isize);
            *fresh23 += 1;
            if !(*fresh23 == *nlevels.offset(v as isize)) {
                break;
            }
            *level.offset(v as isize) = 0 as libc::c_int;
            v -= 1;
            v;
        }
        celln += 1;
        celln;
    }
    dap_free(
        comem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        coeff as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        level as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        clevmem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        clevel as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        nobs as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        mean as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        vari as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        rterm as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        indep as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        nonz as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn effects(
    mut fname: *mut libc::c_char,
    mut varlist: *mut libc::c_char,
    mut model: *mut libc::c_char,
    mut marks: *mut libc::c_char,
) {
    let mut conname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outlist: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut varv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nvars: libc::c_int = 0;
    let mut markv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nmark: libc::c_int = 0;
    let mut typen: libc::c_int = 0;
    let mut termn: libc::c_int = 0;
    let mut termv: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nterm: libc::c_int = 0;
    let mut levmem: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut levptr: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut levval: *mut *mut *mut libc::c_char = 0 as *mut *mut *mut libc::c_char;
    let mut nlevels: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut incells: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut more: libc::c_int = 0;
    conname = dap_malloc(
        (strlen(fname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        fname,
    );
    outlist = dap_malloc(
        (strlen(varlist))
            .wrapping_add(strlen(marks))
            .wrapping_add(9 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    varv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((dap_maxtreat + 1 as libc::c_int) as libc::c_ulong)
            as libc::c_int,
        b"dap_maxtreat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    markv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"dap_maxvar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    termv = dap_malloc(
        dap_maxcell,
        b"dap_maxcell\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    levmem = dap_malloc(
        (dap_maxtreat + 1 as libc::c_int) * dap_maxlev * (dap_strlen + 1 as libc::c_int),
        b"dap_maxtreat, dap_maxlev, dap_strlen\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    levptr = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul((dap_maxtreat + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(dap_maxlev as libc::c_ulong) as libc::c_int,
        b"dap_maxtreat, dap_maxlev\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ) as *mut *mut libc::c_char;
    levval = dap_malloc(
        (::std::mem::size_of::<*mut *mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul((dap_maxtreat + 1 as libc::c_int) as libc::c_ulong)
            as libc::c_int,
        b"dap_maxtreat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut *mut libc::c_char;
    v = 0 as libc::c_int;
    while v < dap_maxtreat + 1 as libc::c_int {
        let ref mut fresh24 = *levval.offset(v as isize);
        *fresh24 = levptr.offset((v * dap_maxlev) as isize);
        l = 0 as libc::c_int;
        while l < dap_maxlev {
            let ref mut fresh25 = *(*levval.offset(v as isize)).offset(l as isize);
            *fresh25 = levmem
                .offset((v * dap_maxlev * (dap_strlen + 1 as libc::c_int)) as isize)
                .offset((l * (dap_strlen + 1 as libc::c_int)) as isize);
            l += 1;
            l;
        }
        v += 1;
        v;
    }
    nlevels = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((dap_maxtreat + 1 as libc::c_int) as libc::c_ulong)
            as libc::c_int,
        b"dap_maxtreat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    if fname.is_null() {
        fputs(
            b"(effects) No dataset name given.\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    dap_suffix(
        conname,
        fname,
        b".con\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    inset(fname);
    typen = dap_varnum(
        b"_type_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if typen < 0 as libc::c_int {
        fputs(
            b"(effects) no _type_ variable\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    termn = dap_vd(
        b"_term_ 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    nvars = dap_list(varlist, varv, dap_maxtreat + 1 as libc::c_int);
    if nvars > dap_maxtreat + 1 as libc::c_int {
        fprintf(
            dap_err,
            b"(effects) too many variables in model: %s\n\0" as *const u8
                as *const libc::c_char,
            model,
        );
        exit(1 as libc::c_int);
    }
    if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
        .offset(*varv.offset(0 as libc::c_int as isize) as isize) != -(1 as libc::c_int)
    {
        fprintf(
            dap_err,
            b"(effects) response variable %s must be of type double\n\0" as *const u8
                as *const libc::c_char,
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                .offset(*varv.offset(0 as libc::c_int as isize) as isize),
        );
        exit(1 as libc::c_int);
    }
    v = 1 as libc::c_int;
    while v < nvars {
        if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
            .offset(*varv.offset(v as isize) as isize) <= 0 as libc::c_int
        {
            fprintf(
                dap_err,
                b"(effects) classification variable %s must be string\n\0" as *const u8
                    as *const libc::c_char,
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(*varv.offset(v as isize) as isize),
            );
            exit(1 as libc::c_int);
        }
        v += 1;
        v;
    }
    nterm = testparse(model, termv, varv, nvars);
    nmark = dap_list(marks, markv, dap_maxvar);
    strcpy(outlist, varlist);
    strcat(outlist, b" _term_\0" as *const u8 as *const libc::c_char);
    if nmark != 0 {
        strcat(outlist, b" \0" as *const u8 as *const libc::c_char);
        strcat(outlist, marks);
    }
    outset(conname, outlist);
    v = 1 as libc::c_int;
    while v < nvars {
        *nlevels.offset(v as isize) = 0 as libc::c_int;
        l = 0 as libc::c_int;
        while l < dap_maxlev {
            *(*(*levval.offset(v as isize)).offset(l as isize))
                .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            l += 1;
            l;
        }
        v += 1;
        v;
    }
    dap_mark();
    more = 1 as libc::c_int;
    incells = 0 as libc::c_int;
    while more != 0 {
        more = step();
        if dap_newpart(markv, nmark) != 0 {
            dap_swap();
            dap_head(markv, nmark);
            dap_swap();
            dap_rewind();
            putlev(nlevels, varv, nvars, levval);
            eff1(incells, levval, nlevels, varv, nvars, termv, nterm, typen, termn);
            v = 1 as libc::c_int;
            while v < nvars {
                *nlevels.offset(v as isize) = 0 as libc::c_int;
                l = 0 as libc::c_int;
                while l < dap_maxlev {
                    *(*(*levval.offset(v as isize)).offset(l as isize))
                        .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                    l += 1;
                    l;
                }
                v += 1;
                v;
            }
            incells = 0 as libc::c_int;
            dap_mark();
            more = step();
        }
        if more != 0 {
            v = 1 as libc::c_int;
            while v < nvars {
                levn(
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                        .offset(*varv.offset(v as isize) as isize),
                    *levval.offset(v as isize),
                    &mut *nlevels.offset(v as isize),
                );
                v += 1;
                v;
            }
            if step() == 0 || step() == 0 {
                fprintf(
                    dap_err,
                    b"(effects) Incomplete cell statistics for: \0" as *const u8
                        as *const libc::c_char,
                );
                v = 1 as libc::c_int;
                while v < nvars {
                    fprintf(
                        dap_err,
                        b"%s (%s) \0" as *const u8 as *const libc::c_char,
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_nam)
                            .offset(*varv.offset(v as isize) as isize),
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_str)
                            .offset(*varv.offset(v as isize) as isize),
                    );
                    v += 1;
                    v;
                }
                putc('\n' as i32, dap_err);
                exit(1 as libc::c_int);
            }
        }
        incells += 1;
        incells;
    }
    if !model.is_null() && *model.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        ftest(
            conname,
            varlist,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            marks,
        );
    }
    dap_free(
        conname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        outlist as *mut libc::c_void,
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
        termv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        levval as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        nlevels as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        levmem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        levptr as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn lsmeans1(
    mut methn: libc::c_int,
    mut alpha: libc::c_double,
    mut coeff: *mut *mut libc::c_double,
    mut ncells: libc::c_int,
    mut err: libc::c_double,
    mut dedfi: libc::c_double,
    mut nerrors: libc::c_int,
    mut nlsmeans: libc::c_int,
    mut mean: *mut libc::c_double,
    mut vari: *mut libc::c_double,
    mut nobs: *mut libc::c_double,
    mut nlevels: libc::c_int,
    mut levval: *mut *mut libc::c_char,
    mut respn: libc::c_int,
    mut treatn: libc::c_int,
    mut resp2n: libc::c_int,
    mut treat2n: libc::c_int,
    mut typen: libc::c_int,
    mut statn: libc::c_int,
    mut lsm1: libc::c_int,
    mut lsm2: libc::c_int,
) {
    let mut l1: libc::c_int = 0;
    let mut l2: libc::c_int = 0;
    let mut lsrow: libc::c_int = 0;
    let mut tmp: libc::c_double = 0.;
    let mut n: libc::c_double = 0.;
    let mut cr: libc::c_int = 0;
    let mut cc: libc::c_int = 0;
    let mut indep: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut sumwt: libc::c_double = 0.;
    let mut lsmean: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut effinvn: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut effin: libc::c_double = 0.;
    let mut diffmem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut diff: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut probmem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut prob: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut pt: libc::c_double = 0.;
    let mut pr: libc::c_double = 0.;
    let mut edfi: libc::c_int = 0;
    let mut dfdown: libc::c_double = 0.;
    let mut dfup: libc::c_double = 0.;
    let mut pdown: libc::c_double = 0.;
    let mut pup: libc::c_double = 0.;
    pt = 0.0f64;
    dap_swap();
    indep = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(ncells as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    lsmean = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nlevels as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    effinvn = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nlevels as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    diffmem = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nlevels as libc::c_ulong)
            .wrapping_mul(nlevels as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    probmem = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nlevels as libc::c_ulong)
            .wrapping_mul(nlevels as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    diff = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nlevels as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_double;
    prob = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nlevels as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_double;
    l1 = 0 as libc::c_int;
    while l1 < nlevels {
        let ref mut fresh26 = *diff.offset(l1 as isize);
        *fresh26 = diffmem.offset((l1 * nlevels) as isize);
        let ref mut fresh27 = *prob.offset(l1 as isize);
        *fresh27 = probmem.offset((l1 * nlevels) as isize);
        l1 += 1;
        l1;
    }
    sumcheck(
        b"lsmeans1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        coeff,
        ncells,
        nerrors,
        0 as libc::c_int,
        0 as *mut libc::c_int,
    );
    cr = 0 as libc::c_int;
    while cr < nerrors + nlsmeans {
        *indep.offset(cr as isize) = 0 as libc::c_int;
        cr += 1;
        cr;
    }
    lsrow = orthog(
        coeff,
        0 as libc::c_int,
        nerrors + nlevels - 1 as libc::c_int,
        nerrors + nlevels - 1 as libc::c_int,
        ncells,
        nobs,
        indep,
        0 as *mut libc::c_int,
    );
    if lsrow - nerrors != nlevels {
        fprintf(
            dap_err,
            b"(lsmeans1) Number of independent LS means %d differs from number of levels %d\n\0"
                as *const u8 as *const libc::c_char,
            lsrow - nerrors,
            nlevels,
        );
        exit(1 as libc::c_int);
    }
    cc = 0 as libc::c_int;
    n = 0.0f64;
    while cc < ncells {
        n += *nobs.offset(cc as isize);
        cc += 1;
        cc;
    }
    l1 = 0 as libc::c_int;
    effin = 0.0f64;
    while l1 < nlevels {
        cc = 0 as libc::c_int;
        *lsmean.offset(l1 as isize) = 0.0f64;
        *effinvn.offset(l1 as isize) = 0.0f64;
        sumwt = 0.0f64;
        while cc < ncells {
            tmp = *(*coeff.offset((nerrors + l1) as isize)).offset(cc as isize);
            *lsmean.offset(l1 as isize) += tmp * *mean.offset(cc as isize);
            sumwt += tmp;
            *effinvn.offset(l1 as isize) += tmp * tmp / *nobs.offset(cc as isize);
            cc += 1;
            cc;
        }
        *lsmean.offset(l1 as isize) /= sumwt;
        *effinvn.offset(l1 as isize) /= sumwt * sumwt;
        effin += *effinvn.offset(l1 as isize);
        l1 += 1;
        l1;
    }
    effin /= nlevels as libc::c_double;
    putc('\n' as i32, dap_lst);
    strcpy(
        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
            .offset(typen as isize),
        b"EFFN\0" as *const u8 as *const libc::c_char,
    );
    l1 = 0 as libc::c_int;
    while l1 < nlevels {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(treatn as isize),
            *levval.offset(l1 as isize),
        );
        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
            .offset(respn as isize) = 1.0f64 / *effinvn.offset(l1 as isize);
        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
            .offset(resp2n as isize) = 1.0f64 / *effinvn.offset(l1 as isize);
        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
            .offset(statn as isize) = 1.0f64 / *effinvn.offset(l1 as isize);
        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
            .offset(lsm1 as isize) = *lsmean.offset(l1 as isize);
        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
            .offset(lsm2 as isize) = 0.0f64;
        output();
        l1 += 1;
        l1;
    }
    dfdown = floor(dedfi);
    dfup = ceil(dedfi);
    edfi = dfdown as libc::c_int;
    if methn == 0 as libc::c_int || methn == 1 as libc::c_int {
        pr = -1.0f64;
        l1 = 0 as libc::c_int;
        while l1 < nlevels {
            l2 = 0 as libc::c_int;
            while l2 < nlevels {
                if l2 == l1 {
                    *(*diff.offset(l1 as isize)).offset(l2 as isize) = 0.0f64;
                    *(*prob.offset(l1 as isize)).offset(l2 as isize) = 1.0f64;
                } else if methn == 0 as libc::c_int {
                    *(*diff.offset(l1 as isize))
                        .offset(
                            l2 as isize,
                        ) = (*lsmean.offset(l1 as isize) - *lsmean.offset(l2 as isize))
                        / sqrt(
                            err * 0.5f64
                                * (*effinvn.offset(l1 as isize)
                                    + *effinvn.offset(l2 as isize)),
                        );
                    *(*diff.offset(l2 as isize))
                        .offset(
                            l1 as isize,
                        ) = -*(*diff.offset(l1 as isize)).offset(l2 as isize);
                    if dfdown == dfup {
                        *(*prob.offset(l1 as isize))
                            .offset(
                                l2 as isize,
                            ) = dap_sr(
                            nlevels,
                            edfi,
                            fabs(*(*diff.offset(l1 as isize)).offset(l2 as isize)),
                        );
                    } else {
                        pdown = dap_sr(
                            nlevels,
                            edfi,
                            fabs(*(*diff.offset(l1 as isize)).offset(l2 as isize)),
                        );
                        pup = dap_sr(
                            nlevels,
                            dfup as libc::c_int,
                            fabs(*(*diff.offset(l1 as isize)).offset(l2 as isize)),
                        );
                        *(*prob.offset(l1 as isize))
                            .offset(
                                l2 as isize,
                            ) = pdown
                            + (dedfi - dfdown) / (dfup - dfdown) * (pup - pdown);
                    }
                    *(*prob.offset(l2 as isize))
                        .offset(
                            l1 as isize,
                        ) = *(*prob.offset(l1 as isize)).offset(l2 as isize);
                } else {
                    *(*diff.offset(l1 as isize))
                        .offset(
                            l2 as isize,
                        ) = (*lsmean.offset(l1 as isize) - *lsmean.offset(l2 as isize))
                        / sqrt(
                            err
                                * (*effinvn.offset(l1 as isize)
                                    + *effinvn.offset(l2 as isize)),
                        );
                    *(*diff.offset(l2 as isize))
                        .offset(
                            l1 as isize,
                        ) = -*(*diff.offset(l1 as isize)).offset(l2 as isize);
                    if dfdown == dfup {
                        *(*prob.offset(l1 as isize))
                            .offset(
                                l2 as isize,
                            ) = 2.0f64
                            * probt(
                                fabs(*(*diff.offset(l1 as isize)).offset(l2 as isize)),
                                edfi,
                            );
                    } else {
                        pdown = probt(
                            fabs(*(*diff.offset(l1 as isize)).offset(l2 as isize)),
                            edfi,
                        );
                        pup = probt(
                            fabs(*(*diff.offset(l1 as isize)).offset(l2 as isize)),
                            dfup as libc::c_int,
                        );
                        *(*prob.offset(l1 as isize))
                            .offset(
                                l2 as isize,
                            ) = 2.0f64
                            * (pdown
                                + (dedfi - dfdown) / (dfup - dfdown) * (pup - pdown));
                    }
                    *(*prob.offset(l2 as isize))
                        .offset(
                            l1 as isize,
                        ) = *(*prob.offset(l1 as isize)).offset(l2 as isize);
                }
                if pr < 0.0f64
                    || fabs(*(*prob.offset(l1 as isize)).offset(l2 as isize) - alpha)
                        < fabs(pr - alpha)
                {
                    pt = fabs(*(*diff.offset(l1 as isize)).offset(l2 as isize));
                    pr = *(*prob.offset(l1 as isize)).offset(l2 as isize);
                }
                l2 += 1;
                l2;
            }
            l1 += 1;
            l1;
        }
        l1 = 0 as libc::c_int;
        while l1 < nlevels {
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(treatn as isize),
                *levval.offset(l1 as isize),
            );
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(respn as isize) = *lsmean.offset(l1 as isize);
            l2 = 0 as libc::c_int;
            while l2 < nlevels {
                if !(l2 == l1) {
                    strcpy(
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_str)
                            .offset(treat2n as isize),
                        *levval.offset(l2 as isize),
                    );
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                        .offset(resp2n as isize) = *lsmean.offset(l2 as isize);
                    strcpy(
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_str)
                            .offset(typen as isize),
                        b"LSMDIFF\0" as *const u8 as *const libc::c_char,
                    );
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                        .offset(
                            statn as isize,
                        ) = *lsmean.offset(l1 as isize) - *lsmean.offset(l2 as isize);
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                        .offset(lsm1 as isize) = *lsmean.offset(l1 as isize);
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                        .offset(lsm2 as isize) = *lsmean.offset(l2 as isize);
                    output();
                    if methn == 0 as libc::c_int {
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_dbl)
                            .offset(resp2n as isize) = *lsmean.offset(l2 as isize);
                        strcpy(
                            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                                .do_str)
                                .offset(typen as isize),
                            b"MINDIFF\0" as *const u8 as *const libc::c_char,
                        );
                        if dfdown == dfup {
                            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                                .do_dbl)
                                .offset(
                                    statn as isize,
                                ) = dap_srpt(nlevels, edfi, pt, pr, alpha)
                                * sqrt(
                                    err * 0.5f64
                                        * (*effinvn.offset(l1 as isize)
                                            + *effinvn.offset(l2 as isize)),
                                );
                        } else {
                            pdown = dap_srpt(nlevels, edfi, pt, pr, alpha);
                            pup = dap_srpt(nlevels, dfup as libc::c_int, pt, pr, alpha);
                            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                                .do_dbl)
                                .offset(
                                    statn as isize,
                                ) = (pdown
                                + (dedfi - dfdown) / (dfup - dfdown) * (pup - pdown))
                                * sqrt(
                                    err * 0.5f64
                                        * (*effinvn.offset(l1 as isize)
                                            + *effinvn.offset(l2 as isize)),
                                );
                        }
                        output();
                    } else {
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_dbl)
                            .offset(resp2n as isize) = *lsmean.offset(l2 as isize);
                        strcpy(
                            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                                .do_str)
                                .offset(typen as isize),
                            b"MINDIFF\0" as *const u8 as *const libc::c_char,
                        );
                        if dfdown == dfup {
                            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                                .do_dbl)
                                .offset(
                                    statn as isize,
                                ) = tpoint(alpha / 2.0f64, edfi)
                                * sqrt(
                                    err
                                        * (*effinvn.offset(l1 as isize)
                                            + *effinvn.offset(l2 as isize)),
                                );
                        } else {
                            pdown = tpoint(alpha / 2.0f64, edfi);
                            pup = tpoint(alpha / 2.0f64, dfup as libc::c_int);
                            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                                .do_dbl)
                                .offset(
                                    statn as isize,
                                ) = (pdown
                                + (dedfi - dfdown) / (dfup - dfdown) * (pup - pdown))
                                * sqrt(
                                    err
                                        * (*effinvn.offset(l1 as isize)
                                            + *effinvn.offset(l2 as isize)),
                                );
                        }
                        output();
                    }
                    strcpy(
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_str)
                            .offset(typen as isize),
                        b"PROB\0" as *const u8 as *const libc::c_char,
                    );
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                        .offset(
                            statn as isize,
                        ) = *(*prob.offset(l1 as isize)).offset(l2 as isize);
                    output();
                }
                l2 += 1;
                l2;
            }
            l1 += 1;
            l1;
        }
    } else {
        pr = -1.0f64;
        l2 = 1 as libc::c_int;
        while l2 < nlevels {
            *(*diff.offset(0 as libc::c_int as isize))
                .offset(
                    l2 as isize,
                ) = (*lsmean.offset(l2 as isize)
                - *lsmean.offset(0 as libc::c_int as isize)) / sqrt(err * effin);
            if dfdown == dfup {
                *(*prob.offset(0 as libc::c_int as isize))
                    .offset(
                        l2 as isize,
                    ) = dap_md(
                    nlevels - 1 as libc::c_int,
                    edfi,
                    fabs(*(*diff.offset(0 as libc::c_int as isize)).offset(l2 as isize)),
                );
            } else {
                pdown = dap_md(
                    nlevels - 1 as libc::c_int,
                    edfi,
                    fabs(*(*diff.offset(0 as libc::c_int as isize)).offset(l2 as isize)),
                );
                pup = dap_md(
                    nlevels - 1 as libc::c_int,
                    dfup as libc::c_int,
                    fabs(*(*diff.offset(0 as libc::c_int as isize)).offset(l2 as isize)),
                );
                *(*prob.offset(l1 as isize))
                    .offset(
                        l2 as isize,
                    ) = pdown + (dedfi - dfdown) / (dfup - dfdown) * (pup - pdown);
            }
            if pr < 0.0f64
                || fabs(
                    *(*prob.offset(0 as libc::c_int as isize)).offset(l2 as isize)
                        - alpha,
                ) < fabs(pr - alpha)
            {
                pt = fabs(
                    *(*diff.offset(0 as libc::c_int as isize)).offset(l2 as isize),
                );
                pr = *(*prob.offset(0 as libc::c_int as isize)).offset(l2 as isize);
            }
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(treat2n as isize),
                *levval.offset(0 as libc::c_int as isize),
            );
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(treatn as isize),
                *levval.offset(l2 as isize),
            );
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(respn as isize) = *lsmean.offset(l2 as isize);
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                b"LSMDIFF\0" as *const u8 as *const libc::c_char,
            );
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(
                    statn as isize,
                ) = *lsmean.offset(l2 as isize)
                - *lsmean.offset(0 as libc::c_int as isize);
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(lsm1 as isize) = *lsmean.offset(l2 as isize);
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(lsm2 as isize) = *lsmean.offset(0 as libc::c_int as isize);
            output();
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                b"PROB\0" as *const u8 as *const libc::c_char,
            );
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(
                    statn as isize,
                ) = *(*prob.offset(0 as libc::c_int as isize)).offset(l2 as isize);
            output();
            l2 += 1;
            l2;
        }
    }
    match methn {
        0 => {
            fputs(b"Tukey method\n\n\0" as *const u8 as *const libc::c_char, dap_lst);
            fprintf(
                dap_lst,
                b"Minimum significant differences are for level %.5f\n\0" as *const u8
                    as *const libc::c_char,
                alpha,
            );
        }
        1 => {
            fputs(b"LSD  method\n\0" as *const u8 as *const libc::c_char, dap_lst);
            fprintf(
                dap_lst,
                b"Minimum significant differences are for level %.5f\n\0" as *const u8
                    as *const libc::c_char,
                alpha,
            );
        }
        2 => {
            pt = dap_mdpt(nlevels, edfi, pt, pr, alpha) * sqrt(err * effin);
            fputs(b"Dunnett method\n\0" as *const u8 as *const libc::c_char, dap_lst);
            fprintf(
                dap_lst,
                b"At level %.5f, minimum significant difference = %.6g\n\0" as *const u8
                    as *const libc::c_char,
                alpha,
                pt,
            );
        }
        _ => {}
    }
    dap_free(
        indep as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        lsmean as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        effinvn as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        diffmem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        diff as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        probmem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        prob as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_swap();
}
pub unsafe extern "C" fn lsmeans(
    mut fname: *mut libc::c_char,
    mut method: *mut libc::c_char,
    mut alpha: libc::c_double,
    mut varlist: *mut libc::c_char,
    mut treat: *mut libc::c_char,
    mut marks: *mut libc::c_char,
    mut format: *mut libc::c_char,
) {
    let mut typen: libc::c_int = 0;
    let mut varv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut markv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nmark: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut lsmname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lsmsrt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut varstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut treat2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut args1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut args2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut treatn: libc::c_int = 0;
    let mut treat2n: libc::c_int = 0;
    let mut resp2n: libc::c_int = 0;
    let mut statn: libc::c_int = 0;
    let mut methn: libc::c_int = 0;
    let mut levmem: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut levval: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut l: libc::c_int = 0;
    static mut nlevels: libc::c_int = 0;
    let mut comem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut coeff: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut gotm: libc::c_int = 0;
    let mut gotn: libc::c_int = 0;
    let mut gotv: libc::c_int = 0;
    let mut mean: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut nobs: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut vari: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut s: libc::c_int = 0;
    let mut ncells: libc::c_int = 0;
    let mut nerrors: libc::c_int = 0;
    let mut nlsmeans: libc::c_int = 0;
    let mut morecells: libc::c_int = 0;
    let mut more: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut con: libc::c_int = 0;
    let mut mse: libc::c_double = 0.;
    let mut edf: libc::c_double = 0.;
    let mut lsm1: libc::c_int = 0;
    let mut lsm2: libc::c_int = 0;
    mse = 0.0f64;
    edf = 0.0f64;
    if fname.is_null() || *fname.offset(0 as libc::c_int as isize) == 0 {
        fputs(
            b"(lsmeans) no dataset name given.\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    lsmname = dap_malloc(
        (strlen(fname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        fname,
    );
    dap_suffix(
        lsmname,
        fname,
        b".lsm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    lsmsrt = dap_malloc(
        (strlen(lsmname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        lsmname,
    );
    dap_suffix(
        lsmsrt,
        lsmname,
        b".srt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    varv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((dap_maxtreat + 1 as libc::c_int) as libc::c_ulong)
            as libc::c_int,
        b"dap_maxtreat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
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
    levval = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(dap_maxlev as libc::c_ulong) as libc::c_int,
        b"dap_maxlev\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_char;
    l = 0 as libc::c_int;
    while l < dap_maxlev {
        let ref mut fresh28 = *levval.offset(l as isize);
        *fresh28 = levmem.offset((l * (dap_strlen + 1 as libc::c_int)) as isize);
        l += 1;
        l;
    }
    comem = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxcell as libc::c_ulong)
            .wrapping_mul(dap_maxcell as libc::c_ulong) as libc::c_int,
        b"dap_maxcell\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    coeff = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxcell as libc::c_ulong) as libc::c_int,
        b"dap_maxcell\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_double;
    ncells = 0 as libc::c_int;
    while ncells < dap_maxcell {
        let ref mut fresh29 = *coeff.offset(ncells as isize);
        *fresh29 = comem.offset((ncells * dap_maxcell) as isize);
        ncells += 1;
        ncells;
    }
    mean = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxcell as libc::c_ulong) as libc::c_int,
        b"dap_maxcell\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    nobs = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxcell as libc::c_ulong) as libc::c_int,
        b"dap_maxcell\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    vari = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxcell as libc::c_ulong) as libc::c_int,
        b"dap_maxcell\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    inset(fname);
    if varlist.is_null() || *varlist.offset(0 as libc::c_int as isize) == 0 {
        fputs(
            b"(lsmeans) no variable list given.\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    dap_list(varlist, varv, dap_maxtreat + 1 as libc::c_int);
    if treat.is_null() || *treat.offset(0 as libc::c_int as isize) == 0 {
        fputs(
            b"(lsmeans) no treatments specified.\n\0" as *const u8
                as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    treatn = dap_varnum(treat);
    if treatn < 0 as libc::c_int {
        fprintf(
            dap_err,
            b"(lsmeans) Treatment variable unknown: %s\n\0" as *const u8
                as *const libc::c_char,
            treat,
        );
        exit(1 as libc::c_int);
    }
    treat2 = dap_malloc(
        (strlen(treat)).wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int,
        treat,
    );
    strcpy(treat2, b"_\0" as *const u8 as *const libc::c_char);
    strcat(treat2, treat);
    varstr = dap_malloc(
        (strlen(treat2))
            .wrapping_add(
                strlen(
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                        .offset(*varv.offset(0 as libc::c_int as isize) as isize),
                ),
            )
            .wrapping_add(10 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    sprintf(
        varstr,
        b"%s %d\0" as *const u8 as *const libc::c_char,
        treat2,
        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
            .offset(treatn as isize),
    );
    treat2n = dap_vd(varstr, 0 as libc::c_int);
    lsm1 = dap_vd(
        b"_lsm_ -1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    lsm2 = dap_vd(
        b"_LSMEAN_ -1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    r = 0 as libc::c_int;
    *varstr.offset(0 as libc::c_int as isize) = '_' as i32 as libc::c_char;
    while *(*((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
        .offset(*varv.offset(0 as libc::c_int as isize) as isize))
        .offset(r as isize) != 0
    {
        if r < dap_namelen - 1 as libc::c_int {
            *varstr
                .offset(
                    (r + 1 as libc::c_int) as isize,
                ) = *(*((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                .offset(*varv.offset(0 as libc::c_int as isize) as isize))
                .offset(r as isize);
        }
        r += 1;
        r;
    }
    sprintf(
        varstr.offset(r as isize).offset(1 as libc::c_int as isize),
        b" %d\0" as *const u8 as *const libc::c_char,
        -(1 as libc::c_int),
    );
    resp2n = dap_vd(varstr, 0 as libc::c_int);
    sprintf(
        varstr,
        b"_stat_ %d\0" as *const u8 as *const libc::c_char,
        -(1 as libc::c_int),
    );
    statn = dap_vd(varstr, 0 as libc::c_int);
    typen = dap_varnum(
        b"_type_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if typen < 0 as libc::c_int {
        fprintf(
            dap_err,
            b"(lsmeans) no _type_ variable\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if dap_varnum(b"_term_\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
        < 0 as libc::c_int
    {
        fprintf(
            dap_err,
            b"(lsmeans) no _term_ variable\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    outset(lsmname, b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if method.is_null() || *method.offset(0 as libc::c_int as isize) == 0 {
        fputs(
            b"(lsmeans) no method specified.\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    if strcmp(method, b"TUKEY\0" as *const u8 as *const libc::c_char) == 0 {
        methn = 0 as libc::c_int;
    } else if strcmp(method, b"LSD\0" as *const u8 as *const libc::c_char) == 0 {
        methn = 1 as libc::c_int;
    } else if strcmp(method, b"DUNNETT\0" as *const u8 as *const libc::c_char) == 0 {
        methn = 2 as libc::c_int;
    } else {
        fprintf(
            dap_err,
            b"(lsmeans) unknown method: %s\n\0" as *const u8 as *const libc::c_char,
            method,
        );
        exit(1 as libc::c_int);
    }
    nmark = dap_list(marks, markv, dap_maxvar);
    l = 0 as libc::c_int;
    while l < dap_maxlev {
        *(*levval.offset(l as isize))
            .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        l += 1;
        l;
    }
    ncells = 0 as libc::c_int;
    nlevels = 0 as libc::c_int;
    nlsmeans = 0 as libc::c_int;
    nerrors = 0 as libc::c_int;
    more = step();
    morecells = 1 as libc::c_int;
    while morecells != 0 {
        gotn = 0 as libc::c_int;
        gotm = 0 as libc::c_int;
        gotv = 0 as libc::c_int;
        morecells = more;
        if dap_newpart(markv, nmark) != 0 {
            dap_swap();
            dap_head(markv, nmark);
            dap_swap();
            fprintf(
                dap_lst,
                b"Least-squares means for: %s\n\0" as *const u8 as *const libc::c_char,
                treat,
            );
            lsmeans1(
                methn,
                alpha,
                coeff,
                ncells,
                mse,
                edf,
                nerrors,
                nlsmeans,
                mean,
                vari,
                nobs,
                nlevels,
                levval,
                *varv.offset(0 as libc::c_int as isize),
                treatn,
                resp2n,
                treat2n,
                typen,
                statn,
                lsm1,
                lsm2,
            );
            ncells = 0 as libc::c_int;
            nlsmeans = 0 as libc::c_int;
            nerrors = 0 as libc::c_int;
            nlevels = 0 as libc::c_int;
            l = 0 as libc::c_int;
            while l < dap_maxlev {
                *(*levval.offset(l as isize))
                    .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                l += 1;
                l;
            }
        }
        s = 0 as libc::c_int;
        while more != 0 {
            if strcmp(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                b"N\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                *nobs
                    .offset(
                        ncells as isize,
                    ) = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .do_dbl)
                    .offset(*varv.offset(0 as libc::c_int as isize) as isize);
                gotn = 1 as libc::c_int;
            } else if strcmp(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                b"MEAN\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                *mean
                    .offset(
                        ncells as isize,
                    ) = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .do_dbl)
                    .offset(*varv.offset(0 as libc::c_int as isize) as isize);
                gotm = 1 as libc::c_int;
            } else if strcmp(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                b"VAR\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                *vari
                    .offset(
                        ncells as isize,
                    ) = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .do_dbl)
                    .offset(*varv.offset(0 as libc::c_int as isize) as isize);
                gotv = 1 as libc::c_int;
            } else {
                fprintf(
                    dap_err,
                    b"(lsmeans) Bad cell statistic: %s\n\0" as *const u8
                        as *const libc::c_char,
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                        .offset(typen as isize),
                );
                exit(1 as libc::c_int);
            }
            s += 1;
            if s < 3 as libc::c_int {
                more = step();
            } else {
                if *nobs.offset(ncells as isize) == 1.0f64 {
                    *vari.offset(ncells as isize) = 0.0f64;
                }
                break;
            }
        }
        if more != 0 {
            if gotm == 0 {
                fputs(
                    b"(lsmeans) Missing MEAN.\n\0" as *const u8 as *const libc::c_char,
                    dap_err,
                );
                exit(1 as libc::c_int);
            }
            if gotn == 0 {
                fputs(
                    b"(lsmeans) Missing N.\n\0" as *const u8 as *const libc::c_char,
                    dap_err,
                );
                exit(1 as libc::c_int);
            }
            if gotv == 0 {
                fputs(
                    b"(lsmeans) Missing VAR.\n\0" as *const u8 as *const libc::c_char,
                    dap_err,
                );
                exit(1 as libc::c_int);
            }
            levn(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(treatn as isize),
                levval,
                &mut nlevels,
            );
        }
        nerrors = 0 as libc::c_int;
        while more != 0 {
            more = step();
            err = (strcmp(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                b"ERROR\0" as *const u8 as *const libc::c_char,
            ) == 0) as libc::c_int;
            con = (strcmp(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                b"CONTR\0" as *const u8 as *const libc::c_char,
            ) == 0) as libc::c_int;
            if !(err != 0 || con != 0) {
                break;
            }
            *(*coeff.offset(nerrors as isize))
                .offset(
                    ncells as isize,
                ) = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(*varv.offset(0 as libc::c_int as isize) as isize);
            nerrors += 1;
            nerrors;
        }
        nlsmeans = 0 as libc::c_int;
        while more != 0 {
            if !(strcmp(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                b"LSMEAN\0" as *const u8 as *const libc::c_char,
            ) == 0)
            {
                break;
            }
            *(*coeff.offset((nerrors + nlsmeans) as isize))
                .offset(
                    ncells as isize,
                ) = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(*varv.offset(0 as libc::c_int as isize) as isize);
            nlsmeans += 1;
            nlsmeans;
            more = step();
        }
        if strcmp(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typen as isize),
            b"MSERROR\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            mse = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(*varv.offset(0 as libc::c_int as isize) as isize);
            if more != 0
                && {
                    more = step();
                    more != 0
                }
                && strcmp(
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                        .offset(typen as isize),
                    b"ERRORDF\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                edf = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(*varv.offset(0 as libc::c_int as isize) as isize);
                if more != 0 {
                    more = step();
                }
            } else {
                fprintf(
                    dap_err,
                    b"(lsmeans1) Expected ERRORDF: %s\n\0" as *const u8
                        as *const libc::c_char,
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                        .offset(typen as isize),
                );
                exit(1 as libc::c_int);
            }
        }
        ncells += 1;
        ncells;
    }
    args1 = dap_malloc(
        (strlen(marks))
            .wrapping_add(strlen(treat2))
            .wrapping_add(strlen(treat))
            .wrapping_add(10 as libc::c_int as libc::c_ulong)
            .wrapping_add(15 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    sprintf(
        args1,
        b"%s _type_ _LSMEAN_ %s _lsm_ %s\0" as *const u8 as *const libc::c_char,
        marks,
        treat2,
        treat,
    );
    sort(lsmname, args1, b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    sprintf(args1, b"_type_ _LSMEAN_ %s\0" as *const u8 as *const libc::c_char, treat2);
    args2 = dap_malloc(
        (strlen(treat))
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_add(6 as libc::c_int as libc::c_ulong) as libc::c_int,
        treat,
    );
    sprintf(args2, b"_lsm_ %s _stat_\0" as *const u8 as *const libc::c_char, treat);
    while *format as libc::c_int == ' ' as i32 {
        format = format.offset(1);
        format;
    }
    if *format as libc::c_int == 's' as i32 {
        format = format.offset(1);
        format;
    }
    table(lsmsrt, args1, args2, format, marks);
    dap_free(
        varv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        markv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        lsmname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        lsmsrt as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        varstr as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        treat2 as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        args1 as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        args2 as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        levmem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        levval as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        comem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        coeff as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        mean as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        nobs as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        vari as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
