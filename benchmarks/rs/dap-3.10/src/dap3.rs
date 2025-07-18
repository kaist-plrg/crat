use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut dap_maxvar: libc::c_int;
    static mut dap_namelen: libc::c_int;
    static mut dap_maxval: libc::c_int;
    static mut dap_maxbars: libc::c_int;
    static mut dap_linelen: libc::c_int;
    fn pict_initpict(prev: *mut pict, p: *mut pict);
    fn pict_rectangle(
        p: *mut pict,
        cx: libc::c_double,
        cy: libc::c_double,
        sx: libc::c_double,
        sy: libc::c_double,
    );
    fn pict_point(p: *mut pict, x: libc::c_double, y: libc::c_double);
    fn pict_line(
        p: *mut pict,
        x0: libc::c_double,
        y0: libc::c_double,
        x1: libc::c_double,
        y1: libc::c_double,
    );
    fn pict_autoaxes(
        p: *mut pict,
        xlab: *mut libc::c_char,
        ylab: *mut libc::c_char,
        axspec: *mut libc::c_char,
        xfunct: Option::<unsafe extern "C" fn() -> libc::c_double>,
        yfunct: Option::<unsafe extern "C" fn() -> libc::c_double>,
        caption: *mut libc::c_char,
        autopos: libc::c_int,
    ) -> libc::c_double;
    fn asin(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn finite(_: libc::c_double) -> libc::c_int;
    fn rint(_: libc::c_double) -> libc::c_double;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn free(__ptr: *mut libc::c_void);
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn step() -> libc::c_int;
    fn inset(fname: *mut libc::c_char);
    fn outset(fname: *mut libc::c_char, varlist: *mut libc::c_char);
    fn output();
    fn sort(
        fname: *mut libc::c_char,
        varlist: *mut libc::c_char,
        modifiers: *mut libc::c_char,
    );
    fn means(
        fname: *mut libc::c_char,
        varlist: *mut libc::c_char,
        statlist: *mut libc::c_char,
        marks: *mut libc::c_char,
    );
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
    fn dap_parsey(yspec: *mut libc::c_char, varv: *mut libc::c_int);
    fn zpoint(p: libc::c_double) -> libc::c_double;
    fn probz(z1: libc::c_double) -> libc::c_double;
    fn dap_simp(
        f: Option::<unsafe extern "C" fn() -> libc::c_double>,
        a: libc::c_double,
        b: libc::c_double,
        n: libc::c_int,
    ) -> libc::c_double;
    fn dataset(
        oldname: *mut libc::c_char,
        newname: *mut libc::c_char,
        action: *mut libc::c_char,
    );
    fn group(
        fname: *mut libc::c_char,
        varspec: *mut libc::c_char,
        marks: *mut libc::c_char,
    );
    fn linreg(
        fname: *mut libc::c_char,
        ylist: *mut libc::c_char,
        x0list: *mut libc::c_char,
        x1list: *mut libc::c_char,
        marks: *mut libc::c_char,
        xname: *mut libc::c_char,
        level: libc::c_double,
    );
    fn logreg(
        fname: *mut libc::c_char,
        ylist: *mut libc::c_char,
        x0list: *mut libc::c_char,
        x1list: *mut libc::c_char,
        marks: *mut libc::c_char,
        xname: *mut libc::c_char,
        level: libc::c_double,
    );
    fn dap_malloc(nbytes: libc::c_int, mesg: *mut libc::c_char) -> *mut libc::c_char;
    fn dap_free(ptr: *mut libc::c_void, mesg: *mut libc::c_char);
    static mut dap_obs: [dataobs; 0];
    static mut dap_lst: *mut FILE;
    static mut dap_log: *mut FILE;
    static mut dap_err: *mut FILE;
    static mut dap_title: *mut libc::c_char;
    static mut dap_dapname: *mut libc::c_char;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _pict {
    pub pict_npts: libc::c_int,
    pub pict_type: [libc::c_char; 5],
    pub pict_dash: libc::c_double,
    pub pict_pt: *mut [libc::c_double; 2],
    pub pict_minx: libc::c_double,
    pub pict_maxx: libc::c_double,
    pub pict_miny: libc::c_double,
    pub pict_maxy: libc::c_double,
    pub pict_ntxt: libc::c_int,
    pub pict_txt: *mut *mut libc::c_char,
    pub pict_font: *mut libc::c_char,
    pub pict_fs: libc::c_double,
    pub pict_tpt: *mut *mut libc::c_double,
    pub pict_tang: *mut libc::c_double,
    pub pict_pos: *mut *mut libc::c_char,
    pub pict_lw: libc::c_double,
    pub pict_r: libc::c_double,
    pub pict_lgray: libc::c_double,
    pub pict_fgray: libc::c_double,
    pub pict_patt: *mut _pict,
    pub pict_next: *mut _pict,
}
pub type pict = _pict;
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
unsafe extern "C" fn plot1(
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
    mut nobs: libc::c_int,
    mut xvar: *mut libc::c_char,
    mut yvar: *mut libc::c_char,
    mut markv: *mut libc::c_int,
    mut nmark: libc::c_int,
    mut p: *mut pict,
    mut a: *mut pict,
    mut pn: libc::c_int,
    mut style: *mut libc::c_char,
    mut xfunct: Option::<unsafe extern "C" fn() -> libc::c_double>,
    mut yfunct: Option::<unsafe extern "C" fn() -> libc::c_double>,
    mut doaxes: libc::c_int,
) {
    let mut titlelen: libc::c_int = 0;
    static mut title0: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    let mut title1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut overlay: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut axspec: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: libc::c_int = 0;
    let mut pictr: libc::c_double = 0.;
    dap_swap();
    if !dap_title.is_null() {
        titlelen = (strlen(dap_title)).wrapping_add(1 as libc::c_int as libc::c_ulong)
            as libc::c_int;
    } else {
        titlelen = 0 as libc::c_int;
    }
    if title0.is_null() {
        title0 = dap_malloc(
            dap_linelen + titlelen + 1 as libc::c_int,
            b"dap_linelen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !dap_title.is_null() {
            strcpy(title0, dap_title);
        }
    }
    title1 = dap_malloc(
        dap_linelen + titlelen + 1 as libc::c_int,
        b"dap_linelen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    s = 0 as libc::c_int;
    while *style.offset(s as isize) as libc::c_int == ' ' as i32 {
        s += 1;
        s;
    }
    overlay = 0 as libc::c_int;
    if *style.offset(s as isize) as libc::c_int == 'o' as i32 {
        s += 1;
        s;
        while *style.offset(s as isize) as libc::c_int == ' ' as i32 {
            s += 1;
            s;
        }
        while '0' as i32 <= *style.offset(s as isize) as libc::c_int
            && *style.offset(s as isize) as libc::c_int <= '9' as i32
        {
            overlay = 10 as libc::c_int * overlay
                + *style.offset(s as isize) as libc::c_int - '0' as i32;
            s += 1;
            s;
        }
        if overlay == 0 {
            overlay = -(1 as libc::c_int);
        }
    }
    while *style.offset(s as isize) as libc::c_int == ' ' as i32 {
        s += 1;
        s;
    }
    axspec = dap_malloc(
        (3 as libc::c_int as libc::c_ulong)
            .wrapping_add(strlen(style as *const libc::c_char)) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(axspec, style.offset(s as isize) as *const libc::c_char);
    if overlay == -(1 as libc::c_int) {
        if pn != 0 {
            pict_initpict(
                p.offset(pn as isize).offset(-(1 as libc::c_int as isize)),
                p.offset(pn as isize),
            );
        } else {
            pict_initpict(0 as *mut pict, p.offset(pn as isize));
        }
        pict_initpict(p.offset(pn as isize), a);
    } else if overlay != 0 {
        if pn % overlay != 0 {
            pict_initpict(
                p.offset(pn as isize).offset(-(1 as libc::c_int as isize)),
                p.offset(pn as isize),
            );
        } else {
            pict_initpict(0 as *mut pict, p.offset(pn as isize));
        }
        pict_initpict(p.offset(pn as isize), a.offset((pn / overlay) as isize));
    } else {
        pict_initpict(0 as *mut pict, p.offset(pn as isize));
        pict_initpict(p.offset(pn as isize), a.offset(pn as isize));
    }
    n = 0 as libc::c_int;
    while n < nobs {
        pict_point(p.offset(pn as isize), *x.offset(n as isize), *y.offset(n as isize));
        n += 1;
        n;
    }
    strcpy(
        ((*p.offset(pn as isize)).pict_type).as_mut_ptr(),
        b"CIRC\0" as *const u8 as *const libc::c_char,
    );
    *title1.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    if !dap_title.is_null() {
        strcpy(title1, dap_title);
    }
    if overlay >= 0 as libc::c_int && nmark != 0 {
        if !dap_title.is_null() {
            strcat(title1, b"\n\0" as *const u8 as *const libc::c_char);
        }
        v = 0 as libc::c_int;
        while v < nmark {
            if v != 0 {
                strcat(title1, b" \0" as *const u8 as *const libc::c_char);
            }
            if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
                .offset(*markv.offset(v as isize) as isize) == -(1 as libc::c_int)
            {
                sprintf(
                    title1.offset(strlen(title1) as isize),
                    b"%g\0" as *const u8 as *const libc::c_char,
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                        .offset(*markv.offset(v as isize) as isize),
                );
            } else if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
                .offset(*markv.offset(v as isize) as isize) == 0 as libc::c_int
            {
                sprintf(
                    title1.offset(strlen(title1) as isize),
                    b"%d\0" as *const u8 as *const libc::c_char,
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_int)
                        .offset(*markv.offset(v as isize) as isize),
                );
            } else {
                strcat(
                    title1,
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                        .offset(*markv.offset(v as isize) as isize),
                );
            }
            v += 1;
            v;
        }
        if overlay > 0 as libc::c_int {
            if pn % overlay == 0 {
                strcpy(title0, title1);
            } else {
                t = 0 as libc::c_int;
                while *title0.offset(t as isize) as libc::c_int != 0
                    && *title0.offset(t as isize) as libc::c_int
                        == *title1.offset(t as isize) as libc::c_int
                {
                    t += 1;
                    t;
                }
                *title1.offset(t as isize) = '\0' as i32 as libc::c_char;
            }
        }
    }
    if doaxes != 0 {
        if overlay == -(1 as libc::c_int) {
            pictr = 0.05f64
                * pict_autoaxes(
                    p,
                    xvar,
                    yvar,
                    axspec,
                    xfunct,
                    yfunct,
                    title1,
                    1 as libc::c_int,
                );
            while pn >= 0 as libc::c_int {
                let fresh0 = pn;
                pn = pn - 1;
                (*p.offset(fresh0 as isize)).pict_r = pictr;
            }
        } else if overlay != 0 {
            pictr = 0.05f64
                * pict_autoaxes(
                    p.offset((pn / overlay * overlay) as isize),
                    xvar,
                    yvar,
                    axspec,
                    xfunct,
                    yfunct,
                    title1,
                    1 as libc::c_int,
                );
            while pn >= pn / overlay * overlay {
                let fresh1 = pn;
                pn = pn - 1;
                (*p.offset(fresh1 as isize)).pict_r = pictr;
            }
        } else {
            (*p.offset(pn as isize))
                .pict_r = 0.05f64
                * pict_autoaxes(
                    p.offset(pn as isize),
                    xvar,
                    yvar,
                    axspec,
                    xfunct,
                    yfunct,
                    title1,
                    1 as libc::c_int,
                );
        }
    }
    dap_swap();
    if doaxes != 0 {
        dap_free(
            title0 as *mut libc::c_void,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        title0 = 0 as *mut libc::c_char;
    }
    dap_free(
        title1 as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        axspec as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn plotparse(
    mut xyvar: *mut libc::c_char,
    mut xyname: *mut libc::c_char,
    mut xname: *mut libc::c_char,
    mut yname: *mut libc::c_char,
) {
    let mut n: libc::c_int = 0;
    let mut xyn: libc::c_int = 0;
    let mut xn: libc::c_int = 0;
    let mut yn: libc::c_int = 0;
    let mut ystart: libc::c_int = 0;
    n = 0 as libc::c_int;
    while *xyvar.offset(n as isize) as libc::c_int == ' ' as i32 {
        n += 1;
        n;
    }
    xyn = 0 as libc::c_int;
    while *xyvar.offset(n as isize) as libc::c_int != 0
        && *xyvar.offset(n as isize) as libc::c_int != ' ' as i32
        && *xyvar.offset(n as isize) as libc::c_int != '`' as i32
    {
        if xyn < dap_namelen {
            let fresh2 = xyn;
            xyn = xyn + 1;
            *xyname.offset(fresh2 as isize) = *xyvar.offset(n as isize);
        } else {
            fprintf(
                dap_err,
                b"(plotparse) X-variable name too long: %s\n\0" as *const u8
                    as *const libc::c_char,
                xyvar,
            );
            exit(1 as libc::c_int);
        }
        n += 1;
        n;
    }
    strncpy(xname, xyvar, n as libc::c_ulong);
    *xname.offset(n as isize) = '\0' as i32 as libc::c_char;
    let fresh3 = xyn;
    xyn = xyn + 1;
    *xyname.offset(fresh3 as isize) = ' ' as i32 as libc::c_char;
    while *xyvar.offset(n as isize) as libc::c_int == ' ' as i32 {
        n += 1;
        n;
    }
    if *xyvar.offset(n as isize) as libc::c_int == '`' as i32 {
        n += 1;
        n;
        while *xyvar.offset(n as isize) as libc::c_int == ' ' as i32 {
            n += 1;
            n;
        }
        xn = 0 as libc::c_int;
        while *xyvar.offset(n as isize) as libc::c_int != 0
            && *xyvar.offset(n as isize) as libc::c_int != '`' as i32
        {
            if xn < dap_linelen {
                let fresh4 = xn;
                xn = xn + 1;
                *xname.offset(fresh4 as isize) = *xyvar.offset(n as isize);
            } else {
                fprintf(
                    dap_err,
                    b"(plotparse) X-axis label too long: %s\n\0" as *const u8
                        as *const libc::c_char,
                    xyvar,
                );
                exit(1 as libc::c_int);
            }
            n += 1;
            n;
        }
        while *xyvar.offset(n as isize) as libc::c_int == ' ' as i32 {
            n += 1;
            n;
        }
        if *xyvar.offset(n as isize) as libc::c_int == '`' as i32 {
            n += 1;
            n;
            while *xyvar.offset(n as isize) as libc::c_int == ' ' as i32 {
                n += 1;
                n;
            }
        } else {
            fprintf(
                dap_err,
                b"(plotparse) Expected ` after x-axis label: %s\n\0" as *const u8
                    as *const libc::c_char,
                xyvar,
            );
            exit(1 as libc::c_int);
        }
        *xname.offset(xn as isize) = '\0' as i32 as libc::c_char;
    }
    ystart = n;
    while *xyvar.offset(n as isize) as libc::c_int != 0
        && *xyvar.offset(n as isize) as libc::c_int != ' ' as i32
        && *xyvar.offset(n as isize) as libc::c_int != '`' as i32
    {
        if xyn < 2 as libc::c_int * (dap_namelen + 1 as libc::c_int) {
            let fresh5 = xyn;
            xyn = xyn + 1;
            *xyname.offset(fresh5 as isize) = *xyvar.offset(n as isize);
        } else {
            fprintf(
                dap_err,
                b"(plotparse) Y-variable name too long: %s\n\0" as *const u8
                    as *const libc::c_char,
                xyvar,
            );
            exit(1 as libc::c_int);
        }
        n += 1;
        n;
    }
    *xyname.offset(xyn as isize) = '\0' as i32 as libc::c_char;
    if !yname.is_null() {
        strncpy(yname, xyvar.offset(ystart as isize), n as libc::c_ulong);
        *yname.offset((n - ystart) as isize) = '\0' as i32 as libc::c_char;
        let fresh6 = xyn;
        xyn = xyn + 1;
        *xyname.offset(fresh6 as isize) = ' ' as i32 as libc::c_char;
        while *xyvar.offset(n as isize) as libc::c_int == ' ' as i32 {
            n += 1;
            n;
        }
        if *xyvar.offset(n as isize) as libc::c_int == '`' as i32 {
            n += 1;
            n;
            while *xyvar.offset(n as isize) as libc::c_int == ' ' as i32 {
                n += 1;
                n;
            }
            yn = 0 as libc::c_int;
            while *xyvar.offset(n as isize) as libc::c_int != 0
                && *xyvar.offset(n as isize) as libc::c_int != '`' as i32
            {
                if yn < dap_linelen {
                    let fresh7 = yn;
                    yn = yn + 1;
                    *yname.offset(fresh7 as isize) = *xyvar.offset(n as isize);
                } else {
                    fprintf(
                        dap_err,
                        b"(plotparse) Y-axis label too long: %s\n\0" as *const u8
                            as *const libc::c_char,
                        xyvar,
                    );
                    exit(1 as libc::c_int);
                }
                n += 1;
                n;
            }
            *yname.offset(yn as isize) = '\0' as i32 as libc::c_char;
        }
    }
    *xyname.offset(xyn as isize) = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn plot(
    mut fname: *mut libc::c_char,
    mut xyvar: *mut libc::c_char,
    mut marks: *mut libc::c_char,
    mut style: *mut libc::c_char,
    mut xfunct: Option::<unsafe extern "C" fn() -> libc::c_double>,
    mut yfunct: Option::<unsafe extern "C" fn() -> libc::c_double>,
    mut nplots: libc::c_int,
) -> *mut pict {
    let mut p: *mut pict = 0 as *mut pict;
    let mut a: *mut pict = 0 as *mut pict;
    let mut markv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nmark: libc::c_int = 0;
    let mut nobs: libc::c_int = 0;
    let mut nnan: libc::c_int = 0;
    static mut x: *mut libc::c_double = 0 as *const libc::c_double
        as *mut libc::c_double;
    static mut y: *mut libc::c_double = 0 as *const libc::c_double
        as *mut libc::c_double;
    let mut pn: libc::c_int = 0;
    let mut xyv: [libc::c_int; 2] = [0; 2];
    let mut xyname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut xname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut yname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: libc::c_int = 0;
    let mut overlay: libc::c_int = 0;
    let mut more: libc::c_int = 0;
    markv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    p = dap_malloc(
        ((2 as libc::c_int * nplots) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<pict>() as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut pict;
    a = p.offset(nplots as isize);
    x = dap_malloc(
        (dap_maxval as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    y = dap_malloc(
        (dap_maxval as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    if fname.is_null() {
        fputs(
            b"(plot) No dataset name given.\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    inset(fname);
    if xyvar.is_null() {
        fputs(
            b"(plot) No x and y variable list given.\n\0" as *const u8
                as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    xyname = dap_malloc(
        2 as libc::c_int * (dap_namelen + 1 as libc::c_int),
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    xname = dap_malloc(
        dap_linelen + 1 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    yname = dap_malloc(
        dap_linelen + 1 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    plotparse(xyvar, xyname, xname, yname);
    nmark = dap_list(marks, markv, dap_maxvar);
    if dap_list(xyname, xyv.as_mut_ptr(), dap_maxvar) != 2 as libc::c_int {
        fprintf(
            dap_err,
            b"(plot) Invalid x and y variable list: %s\n\0" as *const u8
                as *const libc::c_char,
            xyvar,
        );
        exit(1 as libc::c_int);
    }
    if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
        .offset(xyv[0 as libc::c_int as usize] as isize) != -(1 as libc::c_int)
    {
        fprintf(
            dap_err,
            b"(plot) x-variable is not double variable: %s\n\0" as *const u8
                as *const libc::c_char,
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                .offset(xyv[0 as libc::c_int as usize] as isize),
        );
        exit(1 as libc::c_int);
    }
    if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
        .offset(xyv[1 as libc::c_int as usize] as isize) != -(1 as libc::c_int)
    {
        fprintf(
            dap_err,
            b"(plot) y-variable is not double variable: %s\n\0" as *const u8
                as *const libc::c_char,
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                .offset(xyv[1 as libc::c_int as usize] as isize),
        );
        exit(1 as libc::c_int);
    }
    s = 0 as libc::c_int;
    while *style.offset(s as isize) as libc::c_int == ' ' as i32 {
        s += 1;
        s;
    }
    overlay = 0 as libc::c_int;
    if !style.is_null() && *style.offset(s as isize) as libc::c_int == 'o' as i32 {
        s += 1;
        s;
        while *style.offset(s as isize) as libc::c_int == ' ' as i32 {
            s += 1;
            s;
        }
        while '0' as i32 <= *style.offset(s as isize) as libc::c_int
            && *style.offset(s as isize) as libc::c_int <= '9' as i32
        {
            overlay = 10 as libc::c_int * overlay
                + *style.offset(s as isize) as libc::c_int - '0' as i32;
            s += 1;
            s;
        }
        if overlay == 0 {
            overlay = -(1 as libc::c_int);
        }
    }
    nobs = 0 as libc::c_int;
    nnan = 0 as libc::c_int;
    pn = 0 as libc::c_int;
    more = 1 as libc::c_int;
    while more != 0 {
        more = step();
        if dap_newpart(markv, nmark) != 0 {
            if pn < nplots {
                plot1(
                    x,
                    y,
                    nobs,
                    xname,
                    yname,
                    markv,
                    nmark,
                    p,
                    a,
                    pn,
                    style,
                    xfunct,
                    yfunct,
                    (more == 0 || overlay == 0
                        || overlay > 0 as libc::c_int
                            && (pn + 1 as libc::c_int) % overlay == 0) as libc::c_int,
                );
                pn += 1;
                pn;
                if nnan > 0 as libc::c_int {
                    fprintf(
                        dap_log,
                        b"(plot) %d NaNs\n\0" as *const u8 as *const libc::c_char,
                        nnan,
                    );
                }
            } else {
                fprintf(
                    dap_err,
                    b"(plot) More plots than specified by nplots (%d)\n\0" as *const u8
                        as *const libc::c_char,
                    nplots,
                );
                exit(1 as libc::c_int);
            }
            nobs = 0 as libc::c_int;
            nnan = 0 as libc::c_int;
        }
        if nobs < dap_maxval {
            *x
                .offset(
                    nobs as isize,
                ) = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(xyv[0 as libc::c_int as usize] as isize);
            *y
                .offset(
                    nobs as isize,
                ) = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(xyv[1 as libc::c_int as usize] as isize);
            if finite(*x.offset(nobs as isize)) != 0
                && finite(*y.offset(nobs as isize)) != 0
            {
                nobs += 1;
                nobs;
            } else {
                nnan += 1;
                nnan;
            }
        } else {
            fprintf(
                dap_err,
                b"(plot) Too many points\n\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
    dap_free(
        x as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        y as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        markv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        xyname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        xname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        yname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return p;
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
unsafe extern "C" fn comb(mut n: libc::c_int, mut k: libc::c_int) -> libc::c_double {
    let mut dn: libc::c_double = 0.;
    let mut dk: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    c = 1.0f64;
    dn = n as libc::c_double;
    dk = k as libc::c_double;
    while dk >= 1.0f64 {
        c *= dn / dk;
        dn -= 1.0f64;
        dk -= 1.0f64;
    }
    return c;
}
static mut dnmk: libc::c_double = 0.;
static mut dkm1: libc::c_double = 0.;
unsafe extern "C" fn orderf(mut t: libc::c_double) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    let mut tmp1: libc::c_double = 0.;
    let mut tmp2: libc::c_double = 0.;
    if t == -1.0f64 || t == 1.0f64 {
        return 0.0f64;
    }
    tmp1 = 1.0f64 - t * t;
    x = t / sqrt(tmp1);
    tmp2 = dkm1 * log(probz(x)) + dnmk * log(probz(-x)) - 0.5f64 * x * x;
    if finite(tmp2) != 0 {
        return exp(tmp2) * t / (tmp1 * tmp1);
    }
    return 0.0f64;
}
unsafe extern "C" fn geta(mut a: *mut libc::c_double, mut n: libc::c_int) {
    let mut k: libc::c_int = 0;
    let mut dn: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    if n < 7 as libc::c_int {
        match n {
            3 => {
                *a.offset(0 as libc::c_int as isize) = 0.707107f64;
            }
            4 => {
                *a.offset(0 as libc::c_int as isize) = 0.687155f64;
                *a.offset(1 as libc::c_int as isize) = 0.166787f64;
            }
            5 => {
                *a.offset(0 as libc::c_int as isize) = 0.664647f64;
                *a.offset(1 as libc::c_int as isize) = 0.241337f64;
            }
            6 => {
                *a.offset(0 as libc::c_int as isize) = 0.643105f64;
                *a.offset(1 as libc::c_int as isize) = 0.280635f64;
                *a.offset(2 as libc::c_int as isize) = 0.0875196f64;
            }
            _ => {}
        }
        return;
    }
    k = 0 as libc::c_int;
    while k < n / 2 as libc::c_int {
        dnmk = (n - k - 1 as libc::c_int) as libc::c_double;
        dkm1 = k as libc::c_double;
        *a
            .offset(
                k as isize,
            ) = (k as libc::c_double + 1 as libc::c_int as libc::c_double)
            * comb(n, k + 1 as libc::c_int) * 0.398942280401432677940f64
            * dap_simp(
                ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                    Option::<unsafe extern "C" fn() -> libc::c_double>,
                >(
                    Some(
                        orderf as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                    ),
                ),
                -1.0f64,
                1.0f64,
                512 as libc::c_int,
            );
        k += 1;
        k;
    }
    dn = n as libc::c_double;
    if n <= 20 as libc::c_int {
        *a.offset(0 as libc::c_int as isize) = 0.707106781186547524401f64;
        while dn > 2.0f64 {
            *a.offset(0 as libc::c_int as isize) *= (dn - 2.0f64) / (dn - 1.0f64);
            dn -= 2.0f64;
        }
        if dn == 2.0f64 {
            *a.offset(0 as libc::c_int as isize) *= 1.12837916709551257f64;
        } else {
            *a.offset(0 as libc::c_int as isize) *= 1.77245385090551602729f64;
        }
    } else {
        *a.offset(0 as libc::c_int as isize) = 0.707106781186547524401f64;
        while dn > 1.0f64 {
            *a.offset(0 as libc::c_int as isize) *= (dn - 1.0f64) / dn;
            dn -= 2.0f64;
        }
        if dn == 1.0f64 {
            *a.offset(0 as libc::c_int as isize) *= 1.12837916709551257f64;
        } else {
            *a.offset(0 as libc::c_int as isize) *= 1.77245385090551602729f64;
        }
    }
    c = 0.0f64;
    k = 1 as libc::c_int;
    while k < n / 2 as libc::c_int {
        c += *a.offset(k as isize) * *a.offset(k as isize);
        k += 1;
        k;
    }
    c = sqrt((1.0f64 - 2.0f64 * *a.offset(0 as libc::c_int as isize)) / (2.0f64 * c));
    *a.offset(0 as libc::c_int as isize) = -sqrt(*a.offset(0 as libc::c_int as isize));
    k = 1 as libc::c_int;
    while k < n / 2 as libc::c_int {
        *a.offset(k as isize) *= c;
        k += 1;
        k;
    }
}
static mut clambda1: [libc::c_double; 3] = [0.118898f64, 0.133414f64, 0.327907f64];
static mut clambda2: [libc::c_double; 6] = [
    0.480385f64,
    0.318828f64,
    0.0f64,
    -0.0241665f64,
    0.00879701f64,
    0.002989646f64,
];
static mut clogmu1: [libc::c_double; 4] = [
    -0.37542f64,
    -0.492145f64,
    -1.124332f64,
    -0.199422f64,
];
static mut clogmu2: [libc::c_double; 6] = [
    -1.91487f64,
    -1.37888f64,
    -0.04183209f64,
    0.1066339f64,
    -0.03513666f64,
    -0.01504614f64,
];
static mut clogsigma1: [libc::c_double; 4] = [
    -3.15805f64,
    0.729399f64,
    3.01855f64,
    1.558776f64,
];
static mut clogsigma2: [libc::c_double; 7] = [
    -3.73538f64,
    -1.015807f64,
    -0.331885f64,
    0.1773538f64,
    -0.01638782f64,
    -0.03215018f64,
    0.003852646f64,
];
static mut au: [[[libc::c_double; 5]; 2]; 3] = [
    [
        [-1.26233f64, 1.87969f64, 0.0649583f64, -0.0475604f64, -0.0139682f64],
        [-0.287696f64, 1.78953f64, -0.180114f64, 0.0f64, 0.0f64],
    ],
    [
        [-2.28135f64, 2.26186f64, 0.0f64, 0.0f64, -0.00865763f64],
        [-1.63638f64, 5.60924f64, -3.63738f64, 1.08439f64, 0.0f64],
    ],
    [
        [-3.30623f64, 2.76287f64, -0.83484f64, 1.20857f64, -0.507590f64],
        [-5.991908f64, 21.04575f64, -24.58061f64, 13.78661f64, -2.835295f64],
    ],
];
static mut lowhigh: [[libc::c_double; 2]; 3] = [
    [-3.8f64, 8.6f64],
    [-3.0f64, 5.8f64],
    [-4.0f64, 5.4f64],
];
unsafe extern "C" fn poly(
    mut c: *mut libc::c_double,
    mut x: libc::c_double,
    mut n: libc::c_int,
) -> libc::c_double {
    let mut p: libc::c_double = 0.;
    p = *c.offset(n as isize);
    loop {
        n -= 1;
        if !(n >= 0 as libc::c_int) {
            break;
        }
        p = p * x + *c.offset(n as isize);
    }
    return p;
}
unsafe extern "C" fn probw(
    mut n: libc::c_int,
    mut w0: libc::c_double,
    mut a1: libc::c_double,
) -> libc::c_double {
    let mut u: libc::c_double = 0.;
    let mut d: libc::c_double = 0.;
    let mut lambda: libc::c_double = 0.;
    let mut logmu: libc::c_double = 0.;
    let mut logsigma: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut eps: libc::c_double = 0.;
    let mut r: libc::c_int = 0;
    if n == 3 as libc::c_int {
        return 1.90985931710274381f64 * (asin(sqrt(w0)) - asin(sqrt(0.75f64)));
    }
    if n >= 7 as libc::c_int {
        if n <= 20 as libc::c_int {
            d = 3.0f64;
        } else {
            d = 5.0f64;
        }
        d = log(n as libc::c_double) - d;
        if n <= 20 as libc::c_int {
            lambda = poly(clambda1.as_mut_ptr(), d, 2 as libc::c_int);
            logmu = poly(clogmu1.as_mut_ptr(), d, 3 as libc::c_int);
            logsigma = poly(clogsigma1.as_mut_ptr(), d, 3 as libc::c_int);
        } else {
            lambda = poly(clambda2.as_mut_ptr(), d, 5 as libc::c_int);
            logmu = poly(clogmu2.as_mut_ptr(), d, 5 as libc::c_int);
            logsigma = poly(clogsigma2.as_mut_ptr(), d, 6 as libc::c_int);
        }
        y = pow(1.0f64 - w0, lambda);
        return probz(-(y - exp(logmu)) / exp(logsigma));
    } else {
        if w0 >= 1.0f64 {
            return 1.0f64;
        }
        eps = a1 * a1 * (1.0f64 + 1.0f64 / (n - 1 as libc::c_int) as libc::c_double);
        if w0 <= eps {
            return 0.0f64;
        }
        u = log((w0 - eps) / (1.0f64 - w0));
        if w0 < lowhigh[(n - 4 as libc::c_int) as usize][0 as libc::c_int as usize]
            || w0 > lowhigh[(n - 4 as libc::c_int) as usize][1 as libc::c_int as usize]
        {
            return 0.0f64 / 0.0f64;
        }
        if u < 1.4f64 {
            r = 0 as libc::c_int;
        } else {
            r = 1 as libc::c_int;
            u = log(u);
            if finite(u) == 0 {
                return 0.0f64 / 0.0f64;
            }
        }
        u = poly(
            (au[(n - 4 as libc::c_int) as usize][r as usize]).as_mut_ptr(),
            u,
            4 as libc::c_int,
        );
        if r != 0 {
            u = exp(u);
        }
        u = exp(u);
        return 1.90985931710274381f64
            * (asin(sqrt((u + 0.75f64) / (1.0f64 + u))) - asin(sqrt(0.75f64)));
    };
}
unsafe extern "C" fn normal1(
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
    mut nobs: libc::c_int,
    mut varname: *mut libc::c_char,
    mut varlabel: *mut libc::c_char,
    mut markv: *mut libc::c_int,
    mut nmark: libc::c_int,
    mut p: *mut pict,
    mut l: *mut pict,
    mut a: *mut pict,
    mut pn: libc::c_int,
) {
    let mut titlelen: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut dr: libc::c_double = 0.;
    let mut dnobsp25: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.;
    let mut ss: libc::c_double = 0.;
    let mut vtmp: libc::c_double = 0.;
    let mut tmp: libc::c_double = 0.;
    let mut sd: libc::c_double = 0.;
    let mut minx: libc::c_double = 0.;
    let mut maxx: libc::c_double = 0.;
    let mut swa: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut k: libc::c_int = 0;
    let mut w: libc::c_double = 0.;
    let mut prob: libc::c_double = 0.;
    let mut v: libc::c_int = 0;
    let mut caption: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmp: Option::<unsafe extern "C" fn() -> libc::c_int> = None;
    dap_swap();
    cmp = ::std::mem::transmute::<
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
    );
    if !dap_title.is_null() {
        titlelen = (strlen(dap_title)).wrapping_add(1 as libc::c_int as libc::c_ulong)
            as libc::c_int;
    } else {
        titlelen = 0 as libc::c_int;
    }
    if !p.is_null() {
        caption = dap_malloc(
            dap_linelen + titlelen + 47 as libc::c_int,
            b"dap_linelen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    qsort(
        y as *mut libc::c_void,
        nobs as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> libc::c_int>,
            __compar_fn_t,
        >(cmp),
    );
    dnobsp25 = nobs as libc::c_double + 0.25f64;
    r = 0 as libc::c_int;
    sum = 0.0f64;
    ss = 0.0f64;
    minx = 0.0f64;
    maxx = 0.0f64;
    while r < nobs {
        dr = r as libc::c_double;
        *x.offset(r as isize) = -zpoint((dr + 0.625f64) / dnobsp25);
        if *x.offset(r as isize) < minx {
            minx = *x.offset(r as isize);
        }
        if *x.offset(r as isize) > maxx {
            maxx = *x.offset(r as isize);
        }
        vtmp = *y.offset(r as isize);
        if r != 0 {
            tmp = sum - dr * vtmp;
            ss += tmp * tmp / (dr * (dr + 1.0f64));
        }
        sum += vtmp;
        r += 1;
        r;
    }
    sd = sqrt(ss / (nobs - 1 as libc::c_int) as libc::c_double);
    if sd == 0.0f64 {
        fprintf(
            dap_err,
            b"(normal1) Zero standard deviation for %s\n\0" as *const u8
                as *const libc::c_char,
            varname,
        );
        exit(1 as libc::c_int);
    }
    if 3 as libc::c_int <= nobs && nobs <= 2000 as libc::c_int {
        swa = dap_malloc(
            ((nobs / 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_double;
        geta(swa, nobs);
        w = 0.0f64;
        k = 0 as libc::c_int;
        while k < nobs / 2 as libc::c_int {
            w
                += *swa.offset(k as isize)
                    * (*y.offset(k as isize)
                        - *y.offset((nobs - 1 as libc::c_int - k) as isize));
            k += 1;
            k;
        }
        w *= w / ss;
        prob = probw(nobs, w, *swa.offset(0 as libc::c_int as isize));
        if prob < 0.001f64 {
            prob = 0.001f64;
        }
        dap_free(
            swa as *mut libc::c_void,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !p.is_null() {
            sprintf(
                caption,
                b"q-q plot: W|0| = %.4f, P[W < W|0|] = %.3f\0" as *const u8
                    as *const libc::c_char,
                w,
                prob,
            );
        }
        dap_head(markv, nmark);
        fprintf(
            dap_lst,
            b"Shapiro-Wilk test for %s:\nW0 = %.4f, P[W < W0] = %.3f\n\0" as *const u8
                as *const libc::c_char,
            varname,
            w,
            prob,
        );
    } else if !p.is_null() {
        strcpy(caption, b"q-q plot\0" as *const u8 as *const libc::c_char);
    }
    if !p.is_null() && !dap_title.is_null() {
        strcat(caption, b"\n\0" as *const u8 as *const libc::c_char);
        strcat(caption, dap_title);
    }
    if !p.is_null() && nmark != 0 {
        v = 0 as libc::c_int;
        while v < nmark {
            strcat(caption, b"\n\0" as *const u8 as *const libc::c_char);
            if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
                .offset(*markv.offset(v as isize) as isize) == -(1 as libc::c_int)
            {
                sprintf(
                    caption.offset(strlen(caption) as isize),
                    b"%g\0" as *const u8 as *const libc::c_char,
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                        .offset(*markv.offset(v as isize) as isize),
                );
            } else if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
                .offset(*markv.offset(v as isize) as isize) == 0 as libc::c_int
            {
                sprintf(
                    caption.offset(strlen(caption) as isize),
                    b"%d\0" as *const u8 as *const libc::c_char,
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_int)
                        .offset(*markv.offset(v as isize) as isize),
                );
            } else {
                strcat(
                    caption,
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                        .offset(*markv.offset(v as isize) as isize),
                );
            }
            v += 1;
            v;
        }
    }
    if !p.is_null() {
        pict_initpict(0 as *mut pict, p.offset(pn as isize));
        pict_initpict(p.offset(pn as isize), l.offset(pn as isize));
        pict_initpict(l.offset(pn as isize), a.offset(pn as isize));
        r = 0 as libc::c_int;
        while r < nobs {
            pict_point(
                p.offset(pn as isize),
                *x.offset(r as isize),
                *y.offset(r as isize),
            );
            r += 1;
            r;
        }
        strcpy(
            ((*p.offset(pn as isize)).pict_type).as_mut_ptr(),
            b"CIRC\0" as *const u8 as *const libc::c_char,
        );
        sum /= nobs as libc::c_double;
        strcpy(
            ((*l.offset(pn as isize)).pict_type).as_mut_ptr(),
            b"LINE\0" as *const u8 as *const libc::c_char,
        );
        pict_line(l.offset(pn as isize), minx, sd * minx + sum, maxx, sd * maxx + sum);
        (*p.offset(pn as isize))
            .pict_r = 0.05f64
            * pict_autoaxes(
                p.offset(pn as isize),
                b"z\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                varlabel,
                b"-0 NXDIGITS3\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                None,
                None,
                caption,
                1 as libc::c_int,
            );
        free(caption as *mut libc::c_void);
    }
    dap_swap();
}
pub unsafe extern "C" fn normal(
    mut fname: *mut libc::c_char,
    mut variable: *mut libc::c_char,
    mut marks: *mut libc::c_char,
    mut nplots: libc::c_int,
) -> *mut pict {
    let mut varname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut varlabel: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut p: *mut pict = 0 as *mut pict;
    let mut l: *mut pict = 0 as *mut pict;
    let mut a: *mut pict = 0 as *mut pict;
    let mut markv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nmark: libc::c_int = 0;
    let mut nobs: libc::c_int = 0;
    let mut nnan: libc::c_int = 0;
    let mut x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut y: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut pn: libc::c_int = 0;
    let mut vy: libc::c_int = 0;
    let mut more: libc::c_int = 0;
    varname = dap_malloc(
        dap_namelen + 1 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    varlabel = dap_malloc(
        dap_linelen + 1 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    markv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    if nplots != 0 {
        p = dap_malloc(
            ((3 as libc::c_int * nplots) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<pict>() as libc::c_ulong)
                as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut pict;
        l = p.offset(nplots as isize);
        a = p.offset((2 as libc::c_int * nplots) as isize);
    } else {
        p = 0 as *mut libc::c_void as *mut pict;
    }
    x = dap_malloc(
        (dap_maxval as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    y = dap_malloc(
        (dap_maxval as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    if variable.is_null() {
        fputs(
            b"(normal) No variable specified.\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    t = 0 as libc::c_int;
    while *variable.offset(t as isize) as libc::c_int == ' ' as i32 {
        t += 1;
        t;
    }
    s = 0 as libc::c_int;
    while *variable.offset(t as isize) as libc::c_int != 0
        && *variable.offset(t as isize) as libc::c_int != ' ' as i32
        && *variable.offset(t as isize) as libc::c_int != '`' as i32
    {
        if s < dap_namelen {
            let fresh8 = s;
            s = s + 1;
            *varname.offset(fresh8 as isize) = *variable.offset(t as isize);
        } else {
            fprintf(
                dap_err,
                b"(normal) Variable name too long: %s\n\0" as *const u8
                    as *const libc::c_char,
                variable,
            );
            exit(1 as libc::c_int);
        }
        t += 1;
        t;
    }
    *varname.offset(s as isize) = '\0' as i32 as libc::c_char;
    while *variable.offset(t as isize) as libc::c_int == ' ' as i32 {
        t += 1;
        t;
    }
    s = 0 as libc::c_int;
    if *variable.offset(t as isize) as libc::c_int == '`' as i32 {
        t += 1;
        t;
        while *variable.offset(t as isize) as libc::c_int != 0
            && *variable.offset(t as isize) as libc::c_int != ' ' as i32
            && *variable.offset(t as isize) as libc::c_int != '`' as i32
        {
            if s < dap_linelen {
                let fresh9 = s;
                s = s + 1;
                *varlabel.offset(fresh9 as isize) = *variable.offset(t as isize);
            } else {
                fprintf(
                    dap_err,
                    b"(normal) Variable label too long: %s\n\0" as *const u8
                        as *const libc::c_char,
                    variable,
                );
                exit(1 as libc::c_int);
            }
            t += 1;
            t;
        }
        *varlabel.offset(s as isize) = '\0' as i32 as libc::c_char;
    } else {
        strcpy(varlabel, varname);
    }
    if fname.is_null() {
        fputs(
            b"(normal) No dataset name given.\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    inset(fname);
    nmark = dap_list(marks, markv, dap_maxvar);
    vy = dap_varnum(varname);
    if vy < 0 as libc::c_int {
        fprintf(
            dap_err,
            b"(normal) Variable unknown: %s\n\0" as *const u8 as *const libc::c_char,
            varname,
        );
        exit(1 as libc::c_int);
    }
    if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
        .offset(vy as isize) != -(1 as libc::c_int)
    {
        fprintf(
            dap_err,
            b"(normal) Variable is not double variable: %s\n\0" as *const u8
                as *const libc::c_char,
            varname,
        );
        exit(1 as libc::c_int);
    }
    nobs = 0 as libc::c_int;
    nnan = 0 as libc::c_int;
    pn = 0 as libc::c_int;
    more = 1 as libc::c_int;
    while more != 0 {
        more = step();
        if dap_newpart(markv, nmark) != 0 {
            if nplots == 0 || pn < nplots {
                let fresh10 = pn;
                pn = pn + 1;
                normal1(x, y, nobs, varname, varlabel, markv, nmark, p, l, a, fresh10);
                if nnan > 0 as libc::c_int {
                    fprintf(
                        dap_log,
                        b"(normal) %d NaNs\n\0" as *const u8 as *const libc::c_char,
                        nnan,
                    );
                }
            } else {
                fprintf(
                    dap_err,
                    b"(normal) More plots than specified by nplots (%d)\n\0" as *const u8
                        as *const libc::c_char,
                    nplots,
                );
                exit(1 as libc::c_int);
            }
            nobs = 0 as libc::c_int;
            nnan = 0 as libc::c_int;
        }
        if nobs < dap_maxval {
            *y
                .offset(
                    nobs as isize,
                ) = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(vy as isize);
            if finite(*y.offset(nobs as isize)) != 0 {
                nobs += 1;
                nobs;
            } else {
                nnan += 1;
                nnan;
            }
        } else {
            fprintf(
                dap_err,
                b"(normal) Too many points\n\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
    dap_free(
        x as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        y as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        varname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        varlabel as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        markv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return p;
}
unsafe extern "C" fn arint(mut x: libc::c_double) -> libc::c_double {
    let mut i: libc::c_double = 0.;
    i = rint(x);
    if fabs(i) == 0.0f64 { return 0.0f64 } else { return i };
}
unsafe extern "C" fn histo1(
    mut x: *mut libc::c_double,
    mut xw: *mut [libc::c_double; 2],
    mut nobs: libc::c_int,
    mut varv: *mut libc::c_int,
    mut nbars: libc::c_int,
    mut xname: *mut libc::c_char,
    mut style: *mut libc::c_char,
    mut xfunct: Option::<unsafe extern "C" fn() -> libc::c_double>,
    mut markv: *mut libc::c_int,
    mut nmark: libc::c_int,
    mut p: *mut pict,
    mut a: *mut pict,
    mut pn: libc::c_int,
) {
    let mut titlelen: libc::c_int = 0;
    let mut caption: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut word: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut axspec: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut h: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut part: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut equal: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    let mut whole: libc::c_int = 0;
    let mut min: libc::c_double = 0.;
    let mut max: libc::c_double = 0.;
    let mut xlen: libc::c_double = 0.;
    let mut xspace: libc::c_double = 0.;
    static mut htitle: [libc::c_char; 19] = [0; 19];
    let mut width: libc::c_double = 0.;
    let mut dnobs: libc::c_double = 0.;
    let mut dnbars: libc::c_double = 0.;
    let mut b: libc::c_int = 0;
    let mut xn: libc::c_int = 0;
    let mut xnm1: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut maxy: libc::c_double = 0.;
    let mut cmp: Option::<unsafe extern "C" fn() -> libc::c_int> = None;
    cmp = ::std::mem::transmute::<
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
    );
    if !dap_title.is_null() {
        titlelen = (strlen(dap_title)).wrapping_add(1 as libc::c_int as libc::c_ulong)
            as libc::c_int;
    } else {
        titlelen = 0 as libc::c_int;
    }
    caption = dap_malloc(
        titlelen + dap_linelen + 1 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    axspec = dap_malloc(
        (strlen(style)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    word = dap_malloc(
        dap_namelen + 1 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    h = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxbars as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    part = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul((dap_maxbars + 1 as libc::c_int) as libc::c_ulong)
            as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    if nbars == 0 {
        fputs(
            b"(histo1) Number of bars is zero.\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    equal = 1 as libc::c_int;
    height = 3 as libc::c_int;
    whole = 0 as libc::c_int;
    htitle[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    if !x.is_null() {
        qsort(
            x as *mut libc::c_void,
            nobs as size_t,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> libc::c_int>,
                __compar_fn_t,
            >(cmp),
        );
        *part.offset(0 as libc::c_int as isize) = *x.offset(0 as libc::c_int as isize);
        *part.offset(nbars as isize) = *x.offset((nobs - 1 as libc::c_int) as isize);
    } else {
        qsort(
            xw as *mut libc::c_void,
            nobs as size_t,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> libc::c_int>,
                __compar_fn_t,
            >(cmp),
        );
        *part
            .offset(
                0 as libc::c_int as isize,
            ) = (*xw.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize];
        *part
            .offset(
                nbars as isize,
            ) = (*xw
            .offset((nobs - 1 as libc::c_int) as isize))[0 as libc::c_int as usize];
    }
    maxy = 0.0f64;
    *axspec.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    if !style.is_null() {
        s = 0 as libc::c_int;
        while *style.offset(s as isize) as libc::c_int == ' ' as i32 {
            s += 1;
            s;
        }
        while *style.offset(s as isize) != 0 {
            w = 0 as libc::c_int;
            while *style.offset(s as isize) as libc::c_int != 0
                && *style.offset(s as isize) as libc::c_int != ' ' as i32
            {
                if w < dap_namelen {
                    let fresh11 = s;
                    s = s + 1;
                    let fresh12 = w;
                    w = w + 1;
                    *word.offset(fresh12 as isize) = *style.offset(fresh11 as isize);
                } else {
                    *word.offset(w as isize) = '\0' as i32 as libc::c_char;
                    fprintf(
                        dap_err,
                        b"(histo1) Style word too long: %s\n\0" as *const u8
                            as *const libc::c_char,
                        word,
                    );
                    exit(1 as libc::c_int);
                }
            }
            *word.offset(w as isize) = '\0' as i32 as libc::c_char;
            if strcmp(word, b"EQUAL\0" as *const u8 as *const libc::c_char) == 0 {
                equal = 1 as libc::c_int;
            } else if strcmp(word, b"VARIABLE\0" as *const u8 as *const libc::c_char)
                == 0
            {
                equal = 0 as libc::c_int;
            } else if strcmp(word, b"COUNT\0" as *const u8 as *const libc::c_char) == 0 {
                height = 0 as libc::c_int;
            } else if strcmp(word, b"PERCENT\0" as *const u8 as *const libc::c_char) == 0
            {
                height = 1 as libc::c_int;
            } else if strcmp(word, b"FRACTION\0" as *const u8 as *const libc::c_char)
                == 0
            {
                height = 2 as libc::c_int;
            } else if strcmp(word, b"ROUND\0" as *const u8 as *const libc::c_char) == 0 {
                whole = 1 as libc::c_int;
            } else {
                strcat(axspec, word);
                strcat(axspec, b" \0" as *const u8 as *const libc::c_char);
                if strncmp(
                    word,
                    b"MINX\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    if sscanf(
                        word.offset(4 as libc::c_int as isize),
                        b"%lf\0" as *const u8 as *const libc::c_char,
                        &mut *part.offset(0 as libc::c_int as isize)
                            as *mut libc::c_double,
                    ) != 1 as libc::c_int
                    {
                        fprintf(
                            dap_err,
                            b"(histo1) bad MINX specification: %s\n\0" as *const u8
                                as *const libc::c_char,
                            word,
                        );
                        exit(1 as libc::c_int);
                    }
                } else if strncmp(
                    word,
                    b"MAXX\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    if sscanf(
                        word.offset(4 as libc::c_int as isize),
                        b"%lf\0" as *const u8 as *const libc::c_char,
                        &mut *part.offset(nbars as isize) as *mut libc::c_double,
                    ) != 1 as libc::c_int
                    {
                        fprintf(
                            dap_err,
                            b"(histo1) bad MAXX specification: %s\n\0" as *const u8
                                as *const libc::c_char,
                            word,
                        );
                        exit(1 as libc::c_int);
                    }
                }
            }
            while *style.offset(s as isize) as libc::c_int == ' ' as i32 {
                s += 1;
                s;
            }
        }
    }
    if equal != 0 {
        match height {
            3 | 0 => {
                strcpy(
                    htitle.as_mut_ptr(),
                    b"Count\0" as *const u8 as *const libc::c_char,
                );
            }
            1 => {
                strcpy(
                    htitle.as_mut_ptr(),
                    b"Percent\0" as *const u8 as *const libc::c_char,
                );
            }
            2 => {
                strcpy(
                    htitle.as_mut_ptr(),
                    b"Fraction\0" as *const u8 as *const libc::c_char,
                );
            }
            _ => {}
        }
    } else {
        match height {
            0 => {
                fputs(
                    b"(histo1) Can't use count with variable width bars.\n\0"
                        as *const u8 as *const libc::c_char,
                    dap_err,
                );
                exit(1 as libc::c_int);
            }
            1 => {
                strcpy(
                    htitle.as_mut_ptr(),
                    b"Density (Percent)\0" as *const u8 as *const libc::c_char,
                );
            }
            3 | 2 => {
                strcpy(
                    htitle.as_mut_ptr(),
                    b"Density (Fraction)\0" as *const u8 as *const libc::c_char,
                );
            }
            _ => {}
        }
    }
    dnobs = nobs as libc::c_double;
    dnbars = nbars as libc::c_double;
    if whole != 0 {
        xlen = 1e5f64
            / (*part.offset(nbars as isize) - *part.offset(0 as libc::c_int as isize));
        xlen = (arint(xlen * *part.offset(nbars as isize))
            - arint(xlen * *part.offset(0 as libc::c_int as isize))) / xlen;
        if xlen >= 1.0f64 {
            xspace = 1.0f64;
            while dnbars * xspace < xlen {
                xspace *= dnbars;
            }
            xspace *= ceil(xlen / xspace) / dnbars;
        } else {
            xspace = 0.1f64;
            while xspace / nbars as libc::c_double > xlen {
                xspace /= nbars as libc::c_double;
            }
            xspace *= ceil(xlen / xspace) / dnbars;
        }
        *part
            .offset(
                0 as libc::c_int as isize,
            ) = floor(*part.offset(0 as libc::c_int as isize) / xspace) * xspace;
        *part
            .offset(
                nbars as isize,
            ) = ceil(*part.offset(nbars as isize) / xspace) * xspace;
    }
    if equal != 0 {
        width = (*part.offset(nbars as isize) - *part.offset(0 as libc::c_int as isize))
            / dnbars;
        b = 1 as libc::c_int;
        while b < nbars {
            *part
                .offset(
                    b as isize,
                ) = *part.offset(0 as libc::c_int as isize)
                + width * b as libc::c_double;
            b += 1;
            b;
        }
        b = 0 as libc::c_int;
        while b < nbars {
            *h.offset(b as isize) = 0.0f64;
            b += 1;
            b;
        }
        xn = 0 as libc::c_int;
        b = 0 as libc::c_int;
        while xn < nobs {
            if !x.is_null() {
                while b < nbars
                    && *x.offset(xn as isize)
                        > *part.offset((b + 1 as libc::c_int) as isize)
                {
                    b += 1;
                    b;
                }
                *h.offset(b as isize) += 1.0f64;
            } else {
                while b < nbars
                    && (*xw.offset(xn as isize))[0 as libc::c_int as usize]
                        > *part.offset((b + 1 as libc::c_int) as isize)
                {
                    b += 1;
                    b;
                }
                *h.offset(b as isize)
                    += (*xw.offset(xn as isize))[1 as libc::c_int as usize];
            }
            xn += 1;
            xn;
        }
        b = 0 as libc::c_int;
        while b < nbars {
            let mut current_block_126: u64;
            match height {
                1 => {
                    *h.offset(b as isize) *= 100.0f64;
                    current_block_126 = 10727749401846427386;
                }
                2 => {
                    current_block_126 = 10727749401846427386;
                }
                _ => {
                    current_block_126 = 13740693533991687037;
                }
            }
            match current_block_126 {
                10727749401846427386 => {
                    *h.offset(b as isize) /= dnobs;
                }
                _ => {}
            }
            b += 1;
            b;
        }
    } else {
        b = 1 as libc::c_int;
        xnm1 = 0 as libc::c_int;
        while b < nbars {
            xn = rint(dnobs * b as libc::c_double / dnbars) as libc::c_int;
            if !x.is_null() {
                *part.offset(b as isize) = *x.offset(xn as isize);
            } else {
                *part
                    .offset(
                        b as isize,
                    ) = (*xw.offset(xn as isize))[0 as libc::c_int as usize];
            }
            if *part.offset(b as isize) > *part.offset((b - 1 as libc::c_int) as isize) {
                *h
                    .offset(
                        b as isize,
                    ) = (xn - xnm1) as libc::c_double
                    / ((*part.offset(b as isize)
                        - *part.offset((b - 1 as libc::c_int) as isize)) * dnobs);
            } else {
                *h.offset(b as isize) = 0.0f64;
            }
            if height == 1 as libc::c_int {
                *h.offset(b as isize) *= 100.0f64;
            }
            xnm1 = xn;
            b += 1;
            b;
        }
    }
    pict_initpict(0 as *mut pict, p.offset(pn as isize));
    pict_initpict(p.offset(pn as isize), a.offset(pn as isize));
    b = 0 as libc::c_int;
    while b < nbars {
        pict_rectangle(
            p.offset(pn as isize),
            *part.offset(b as isize),
            0.0f64,
            *part.offset((b + 1 as libc::c_int) as isize) - *part.offset(b as isize),
            *h.offset(b as isize),
        );
        b += 1;
        b;
    }
    *caption.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    if !dap_title.is_null() {
        strcpy(caption, dap_title);
    }
    if nmark != 0 {
        if !dap_title.is_null() {
            strcat(caption, b"\n\0" as *const u8 as *const libc::c_char);
        }
        v = 0 as libc::c_int;
        while v < nmark {
            if v != 0 {
                strcat(caption, b" \0" as *const u8 as *const libc::c_char);
            }
            if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
                .offset(*markv.offset(v as isize) as isize) == -(1 as libc::c_int)
            {
                sprintf(
                    caption.offset(strlen(caption) as isize),
                    b"%g\0" as *const u8 as *const libc::c_char,
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                        .offset(*markv.offset(v as isize) as isize),
                );
            } else if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
                .offset(*markv.offset(v as isize) as isize) == 0 as libc::c_int
            {
                sprintf(
                    caption.offset(strlen(caption) as isize),
                    b"%d\0" as *const u8 as *const libc::c_char,
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_int)
                        .offset(*markv.offset(v as isize) as isize),
                );
            } else {
                strcat(
                    caption,
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                        .offset(*markv.offset(v as isize) as isize),
                );
            }
            v += 1;
            v;
        }
    }
    pict_autoaxes(
        p.offset(pn as isize),
        xname,
        htitle.as_mut_ptr(),
        axspec,
        xfunct,
        None,
        caption,
        1 as libc::c_int,
    );
    dap_free(
        caption as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        axspec as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        word as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        h as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        part as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn histogram(
    mut fname: *mut libc::c_char,
    mut vars: *mut libc::c_char,
    mut marks: *mut libc::c_char,
    mut nbars: libc::c_int,
    mut style: *mut libc::c_char,
    mut xfunct: Option::<unsafe extern "C" fn() -> libc::c_double>,
    mut nplots: libc::c_int,
) -> *mut pict {
    let mut p: *mut pict = 0 as *mut pict;
    let mut a: *mut pict = 0 as *mut pict;
    let mut markv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nmark: libc::c_int = 0;
    let mut varv: [libc::c_int; 2] = [0; 2];
    let mut nvar: libc::c_int = 0;
    let mut nobs: libc::c_int = 0;
    let mut x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut xw: *mut [libc::c_double; 2] = 0 as *mut [libc::c_double; 2];
    let mut xwname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut xname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pn: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut mv: libc::c_int = 0;
    let mut nnan: libc::c_int = 0;
    let mut more: libc::c_int = 0;
    markv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    xwname = dap_malloc(
        2 as libc::c_int * (dap_namelen + 1 as libc::c_int),
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    xname = dap_malloc(
        dap_linelen + 1 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    p = dap_malloc(
        ((2 as libc::c_int * nplots) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<pict>() as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut pict;
    a = p.offset(nplots as isize);
    if fname.is_null() {
        fputs(
            b"(histogram) No dataset name given.\n\0" as *const u8
                as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    inset(fname);
    nmark = dap_list(marks, markv, dap_maxvar);
    if vars.is_null() {
        fputs(
            b"(histogram) No variable given.\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    plotparse(vars, xwname, xname, 0 as *mut libc::c_char);
    nvar = dap_list(xwname, varv.as_mut_ptr(), dap_maxvar);
    v = 0 as libc::c_int;
    while v < nvar {
        if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
            .offset(varv[v as usize] as isize) != -(1 as libc::c_int)
        {
            fprintf(
                dap_err,
                b"(histogram) Variable is not double variable: %s\n\0" as *const u8
                    as *const libc::c_char,
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(varv[v as usize] as isize),
            );
            exit(1 as libc::c_int);
        }
        v += 1;
        v;
    }
    if nvar == 1 as libc::c_int {
        x = dap_malloc(
            (dap_maxval as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_double;
        xw = 0 as *mut libc::c_void as *mut [libc::c_double; 2];
    } else if nvar == 2 as libc::c_int {
        xw = dap_malloc(
            ((dap_maxval * 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut [libc::c_double; 2];
        x = 0 as *mut libc::c_void as *mut libc::c_double;
    } else {
        fprintf(
            dap_err,
            b"(histogram) Variable list contains more than two variables: %s\n\0"
                as *const u8 as *const libc::c_char,
            vars,
        );
        exit(1 as libc::c_int);
    }
    nobs = 0 as libc::c_int;
    nnan = 0 as libc::c_int;
    pn = 0 as libc::c_int;
    more = 1 as libc::c_int;
    while more != 0 {
        more = step();
        if dap_newpart(markv, nmark) != 0 {
            dap_swap();
            if nnan != 0 {
                fprintf(
                    dap_log,
                    b"(histogram) %d missing values for:\0" as *const u8
                        as *const libc::c_char,
                    nnan,
                );
                mv = 0 as libc::c_int;
                while mv < nmark {
                    putc(' ' as i32, dap_log);
                    if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                        .do_len)
                        .offset(*markv.offset(mv as isize) as isize)
                        == -(1 as libc::c_int)
                    {
                        fprintf(
                            dap_log,
                            b"%g\0" as *const u8 as *const libc::c_char,
                            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                                .do_dbl)
                                .offset(*markv.offset(mv as isize) as isize),
                        );
                    } else if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                        .do_len)
                        .offset(*markv.offset(mv as isize) as isize) == 0 as libc::c_int
                    {
                        fprintf(
                            dap_log,
                            b"%d\0" as *const u8 as *const libc::c_char,
                            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                                .do_int)
                                .offset(*markv.offset(mv as isize) as isize),
                        );
                    } else {
                        fputs(
                            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                                .do_str)
                                .offset(*markv.offset(mv as isize) as isize),
                            dap_log,
                        );
                    }
                    mv += 1;
                    mv;
                }
                putc('\n' as i32, dap_log);
            }
            if nobs != 0 {
                if pn < nplots {
                    let fresh13 = pn;
                    pn = pn + 1;
                    histo1(
                        x,
                        xw,
                        nobs,
                        varv.as_mut_ptr(),
                        nbars,
                        xname,
                        style,
                        xfunct,
                        markv,
                        nmark,
                        p,
                        a,
                        fresh13,
                    );
                } else {
                    fprintf(
                        dap_err,
                        b"(histogram) More plots than specified by nplots (%d)\n\0"
                            as *const u8 as *const libc::c_char,
                        nplots,
                    );
                    exit(1 as libc::c_int);
                }
            }
            dap_swap();
            nobs = 0 as libc::c_int;
            nnan = 0 as libc::c_int;
        }
        if nobs < dap_maxval {
            if nvar == 1 as libc::c_int {
                *x
                    .offset(
                        nobs as isize,
                    ) = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .do_dbl)
                    .offset(varv[0 as libc::c_int as usize] as isize);
                if finite(*x.offset(nobs as isize)) != 0 {
                    nobs += 1;
                    nobs;
                } else {
                    nnan += 1;
                    nnan;
                }
            } else {
                (*xw
                    .offset(
                        nobs as isize,
                    ))[0 as libc::c_int
                    as usize] = *((*dap_obs
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                    .do_dbl)
                    .offset(varv[0 as libc::c_int as usize] as isize);
                (*xw
                    .offset(
                        nobs as isize,
                    ))[1 as libc::c_int
                    as usize] = *((*dap_obs
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                    .do_dbl)
                    .offset(varv[1 as libc::c_int as usize] as isize);
                if finite((*xw.offset(nobs as isize))[0 as libc::c_int as usize]) != 0
                    && finite((*xw.offset(nobs as isize))[1 as libc::c_int as usize])
                        != 0
                {
                    nobs += 1;
                    nobs;
                } else {
                    nnan += 1;
                    nnan;
                }
            }
        } else {
            fprintf(
                dap_err,
                b"(histogram) Too many points\n\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
    if nvar == 1 as libc::c_int {
        dap_free(
            x as *mut libc::c_void,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else {
        dap_free(
            xw as *mut libc::c_void,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    dap_free(
        markv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        xwname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        xname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return p;
}
pub unsafe extern "C" fn plotlinreg(
    mut fname: *mut libc::c_char,
    mut ylist0: *mut libc::c_char,
    mut x1list0: *mut libc::c_char,
    mut style: *mut libc::c_char,
    mut marks: *mut libc::c_char,
    mut nmarks: libc::c_int,
    mut level: libc::c_double,
) -> *mut pict {
    let mut ylist: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x1list: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    let mut varv: [libc::c_int; 1] = [0; 1];
    let mut typen: libc::c_int = 0;
    let mut sortord: [libc::c_int; 4] = [0; 4];
    let mut s: libc::c_int = 0;
    let mut mnsname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut regname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut srtarg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut srtname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut plotvars: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut plotmarks: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut pict = 0 as *mut pict;
    let mut pn: libc::c_int = 0;
    let mut plotstyle: *mut libc::c_char = 0 as *mut libc::c_char;
    ylist = dap_malloc(
        (strlen(ylist0)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    x1list = dap_malloc(
        (strlen(x1list0)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    l = 0 as libc::c_int;
    while *ylist0.offset(l as isize) as libc::c_int != 0
        && *ylist0.offset(l as isize) as libc::c_int != '`' as i32
    {
        *ylist.offset(l as isize) = *ylist0.offset(l as isize);
        l += 1;
        l;
    }
    *ylist.offset(l as isize) = '\0' as i32 as libc::c_char;
    l = 0 as libc::c_int;
    while *x1list0.offset(l as isize) as libc::c_int != 0
        && *x1list0.offset(l as isize) as libc::c_int != '`' as i32
    {
        *x1list.offset(l as isize) = *x1list0.offset(l as isize);
        l += 1;
        l;
    }
    *x1list.offset(l as isize) = '\0' as i32 as libc::c_char;
    inset(fname);
    typen = dap_varnum(
        b"_type_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if typen < 0 as libc::c_int {
        fprintf(
            dap_err,
            b"(plotlinreg) missing type variable in %s\n\0" as *const u8
                as *const libc::c_char,
            fname,
        );
        exit(1 as libc::c_int);
    }
    step();
    s = 0 as libc::c_int;
    while s < 4 as libc::c_int {
        sortord[s as usize] = s;
        s += 1;
        s;
    }
    if strcmp(
        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
            .offset(typen as isize),
        b"LOWER\0" as *const u8 as *const libc::c_char,
    ) < 0 as libc::c_int
    {
        sortord[0 as libc::c_int as usize] = 1 as libc::c_int;
        sortord[1 as libc::c_int as usize] = 0 as libc::c_int;
    } else if !(strcmp(
        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
            .offset(typen as isize),
        b"PRED\0" as *const u8 as *const libc::c_char,
    ) < 0 as libc::c_int)
    {
        if strcmp(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typen as isize),
            b"UPPER\0" as *const u8 as *const libc::c_char,
        ) < 0 as libc::c_int
        {
            sortord[1 as libc::c_int as usize] = 2 as libc::c_int;
            sortord[2 as libc::c_int as usize] = 1 as libc::c_int;
        } else {
            sortord[1 as libc::c_int as usize] = 3 as libc::c_int;
            sortord[2 as libc::c_int as usize] = 1 as libc::c_int;
            sortord[3 as libc::c_int as usize] = 2 as libc::c_int;
        }
    }
    dap_list(ylist, varv.as_mut_ptr(), 1 as libc::c_int);
    dap_list(x1list, varv.as_mut_ptr(), 1 as libc::c_int);
    mnsname = dap_malloc(
        (strlen(fname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(mnsname, fname);
    strcat(mnsname, b".mns\0" as *const u8 as *const libc::c_char);
    regname = dap_malloc(
        (strlen(fname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(regname, fname);
    strcat(regname, b".reg\0" as *const u8 as *const libc::c_char);
    srtname = dap_malloc(
        (strlen(regname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(srtname, regname);
    strcat(srtname, b".srt\0" as *const u8 as *const libc::c_char);
    means(
        fname,
        x1list,
        b"STEP100\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        marks,
    );
    linreg(
        fname,
        ylist,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        x1list,
        marks,
        mnsname,
        level,
    );
    dataset(
        fname,
        regname,
        b"APPEND\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    srtarg = dap_malloc(
        (strlen(marks))
            .wrapping_add(strlen(x1list))
            .wrapping_add(9 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(srtarg, marks);
    strcat(srtarg, b" _type_ \0" as *const u8 as *const libc::c_char);
    strcat(srtarg, x1list);
    sort(
        regname,
        srtarg,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    plotvars = dap_malloc(
        (strlen(x1list0))
            .wrapping_add(strlen(ylist0))
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(plotvars, x1list0);
    strcat(plotvars, b" \0" as *const u8 as *const libc::c_char);
    strcat(plotvars, ylist0);
    plotmarks = dap_malloc(
        (strlen(marks))
            .wrapping_add(strlen(b"_type_\0" as *const u8 as *const libc::c_char))
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(plotmarks, marks);
    strcat(plotmarks, b" _type_\0" as *const u8 as *const libc::c_char);
    plotstyle = dap_malloc(
        (strlen(style)).wrapping_add(4 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(plotstyle, b"o4 \0" as *const u8 as *const libc::c_char);
    strcat(plotstyle, style);
    p = plot(
        srtname,
        plotvars,
        plotmarks,
        plotstyle,
        None,
        None,
        4 as libc::c_int * nmarks,
    );
    pn = 0 as libc::c_int;
    while pn < nmarks {
        strcpy(
            ((*p
                .offset(
                    (4 as libc::c_int * pn + sortord[0 as libc::c_int as usize]) as isize,
                ))
                .pict_type)
                .as_mut_ptr(),
            b"LINE\0" as *const u8 as *const libc::c_char,
        );
        strcpy(
            ((*p
                .offset(
                    (4 as libc::c_int * pn + sortord[2 as libc::c_int as usize]) as isize,
                ))
                .pict_type)
                .as_mut_ptr(),
            b"LINE\0" as *const u8 as *const libc::c_char,
        );
        strcpy(
            ((*p
                .offset(
                    (4 as libc::c_int * pn + sortord[3 as libc::c_int as usize]) as isize,
                ))
                .pict_type)
                .as_mut_ptr(),
            b"LINE\0" as *const u8 as *const libc::c_char,
        );
        (*p
            .offset(
                (4 as libc::c_int * pn + sortord[0 as libc::c_int as usize]) as isize,
            ))
            .pict_dash = 4.0f64;
        (*p
            .offset(
                (4 as libc::c_int * pn + sortord[3 as libc::c_int as usize]) as isize,
            ))
            .pict_dash = 4.0f64;
        pn += 1;
        pn;
    }
    dap_free(
        ylist as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        x1list as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        mnsname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        regname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        srtarg as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        srtname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        plotvars as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        plotmarks as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        plotstyle as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return p;
}
pub unsafe extern "C" fn plotlogreg(
    mut fname: *mut libc::c_char,
    mut yspec0: *mut libc::c_char,
    mut x1list0: *mut libc::c_char,
    mut style: *mut libc::c_char,
    mut ngroups: libc::c_int,
    mut marks: *mut libc::c_char,
    mut nmarks: libc::c_int,
    mut level: libc::c_double,
) -> *mut pict {
    let mut yspec: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x1list: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut varv: [libc::c_int; 3] = [0; 3];
    let mut trlname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut trialsn: libc::c_int = 0;
    let mut varspec: [libc::c_char; 12] = [0; 12];
    let mut grpname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut grparg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut grpvar: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mnsarg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mnsname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lgrname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut srtarg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut srtname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut plotvars: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut plotmarks: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut casevar: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut casevar0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    let mut cs: libc::c_int = 0;
    let mut p: *mut pict = 0 as *mut pict;
    let mut pn: libc::c_int = 0;
    let mut plotstyle: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l0: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    yspec = dap_malloc(
        (strlen(yspec0)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    x1list = dap_malloc(
        (strlen(x1list0)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    l0 = 0 as libc::c_int;
    l = 0 as libc::c_int;
    while *yspec0.offset(l0 as isize) as libc::c_int != 0
        && *yspec0.offset(l0 as isize) as libc::c_int != '`' as i32
    {
        let fresh14 = l0;
        l0 = l0 + 1;
        let fresh15 = l;
        l = l + 1;
        *yspec.offset(fresh15 as isize) = *yspec0.offset(fresh14 as isize);
    }
    if *yspec0.offset(l0 as isize) as libc::c_int == '`' as i32 {
        l0 += 1;
        l0;
        while *yspec0.offset(l0 as isize) as libc::c_int != 0
            && *yspec0.offset(l0 as isize) as libc::c_int != '`' as i32
        {
            l0 += 1;
            l0;
        }
        l0 += 1;
        l0;
        while *yspec0.offset(l0 as isize) != 0 {
            let fresh16 = l0;
            l0 = l0 + 1;
            let fresh17 = l;
            l = l + 1;
            *yspec.offset(fresh17 as isize) = *yspec0.offset(fresh16 as isize);
        }
    }
    *yspec.offset(l as isize) = '\0' as i32 as libc::c_char;
    l = 0 as libc::c_int;
    while *x1list0.offset(l as isize) as libc::c_int != 0
        && *x1list0.offset(l as isize) as libc::c_int != '`' as i32
    {
        *x1list.offset(l as isize) = *x1list0.offset(l as isize);
        l += 1;
        l;
    }
    *x1list.offset(l as isize) = '\0' as i32 as libc::c_char;
    trlname = dap_malloc(
        (strlen(fname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(trlname, fname);
    strcat(trlname, b".trl\0" as *const u8 as *const libc::c_char);
    grpname = dap_malloc(
        (strlen(trlname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(grpname, trlname);
    strcat(grpname, b".grp\0" as *const u8 as *const libc::c_char);
    srtname = dap_malloc(
        (strlen(grpname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(srtname, grpname);
    strcat(srtname, b".srt\0" as *const u8 as *const libc::c_char);
    mnsname = dap_malloc(
        (strlen(srtname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(mnsname, srtname);
    strcat(mnsname, b".mns\0" as *const u8 as *const libc::c_char);
    lgrname = dap_malloc(
        (strlen(fname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(lgrname, fname);
    strcat(lgrname, b".lgr\0" as *const u8 as *const libc::c_char);
    grparg = dap_malloc(
        (strlen(x1list)).wrapping_add(14 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    grpvar = dap_malloc(
        (strlen(marks))
            .wrapping_add(strlen(x1list))
            .wrapping_add(3 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    casevar = dap_malloc(
        (strlen(yspec)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    casevar0 = dap_malloc(
        (strlen(yspec0)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    mnsarg = dap_malloc(
        (strlen(yspec))
            .wrapping_add(12 as libc::c_int as libc::c_ulong)
            .wrapping_add(strlen(x1list)) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    srtarg = dap_malloc(
        (strlen(marks))
            .wrapping_add(strlen(x1list))
            .wrapping_add(9 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    plotvars = dap_malloc(
        (strlen(x1list0))
            .wrapping_add(strlen(yspec0))
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    plotmarks = dap_malloc(
        (strlen(marks))
            .wrapping_add(strlen(b"_type_\0" as *const u8 as *const libc::c_char))
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    inset(fname);
    dap_list(x1list, varv.as_mut_ptr(), 1 as libc::c_int);
    strcpy(grpvar, marks);
    strcat(grpvar, b" _\0" as *const u8 as *const libc::c_char);
    strcat(grpvar, x1list);
    strcpy(casevar, yspec);
    cs = 0 as libc::c_int;
    while *casevar.offset(cs as isize) as libc::c_int == ' ' as i32 {
        cs += 1;
        cs;
    }
    c = 0 as libc::c_int;
    while *casevar.offset(cs as isize) as libc::c_int != 0
        && *casevar.offset(cs as isize) as libc::c_int != ' ' as i32
        && *casevar.offset(cs as isize) as libc::c_int != '/' as i32
    {
        let fresh18 = cs;
        cs = cs + 1;
        let fresh19 = c;
        c = c + 1;
        *casevar.offset(fresh19 as isize) = *casevar.offset(fresh18 as isize);
    }
    *casevar.offset(c as isize) = '\0' as i32 as libc::c_char;
    strcpy(casevar0, yspec0);
    cs = 0 as libc::c_int;
    while *casevar0.offset(cs as isize) as libc::c_int == ' ' as i32 {
        cs += 1;
        cs;
    }
    c = 0 as libc::c_int;
    while *casevar0.offset(cs as isize) as libc::c_int != 0
        && *casevar0.offset(cs as isize) as libc::c_int != '/' as i32
    {
        let fresh20 = cs;
        cs = cs + 1;
        let fresh21 = c;
        c = c + 1;
        *casevar0.offset(fresh21 as isize) = *casevar0.offset(fresh20 as isize);
    }
    *casevar0.offset(c as isize) = '\0' as i32 as libc::c_char;
    inset(fname);
    sprintf(
        varspec.as_mut_ptr(),
        b"_ntrials %d\0" as *const u8 as *const libc::c_char,
        -(1 as libc::c_int),
    );
    trialsn = dap_vd(varspec.as_mut_ptr(), 0 as libc::c_int);
    outset(trlname, b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    dap_parsey(yspec, varv.as_mut_ptr());
    while step() != 0 {
        if varv[1 as libc::c_int as usize] >= 0 as libc::c_int {
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(
                    trialsn as isize,
                ) = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(varv[1 as libc::c_int as usize] as isize);
        } else {
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(
                    trialsn as isize,
                ) = -varv[1 as libc::c_int as usize] as libc::c_double;
        }
        output();
    }
    strcpy(grparg, x1list);
    sprintf(
        grparg.offset(strlen(grparg) as isize),
        b" %d#\0" as *const u8 as *const libc::c_char,
        ngroups,
    );
    group(trlname, grparg, marks);
    sort(
        grpname,
        grpvar,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(mnsarg, casevar);
    strcat(mnsarg, b" _ntrials \0" as *const u8 as *const libc::c_char);
    strcat(mnsarg, x1list);
    means(
        srtname,
        mnsarg,
        b"MEAN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        grpvar,
    );
    inset(mnsname);
    outset(grpname, b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    trialsn = dap_varnum(
        b"_ntrials\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_list(casevar, varv.as_mut_ptr(), 1 as libc::c_int);
    while step() != 0 {
        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
            .offset(varv[0 as libc::c_int as usize] as isize)
            /= *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(trialsn as isize);
        output();
    }
    means(
        fname,
        x1list,
        b"STEP100\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        marks,
    );
    strcpy(mnsname, fname);
    strcat(mnsname, b".mns\0" as *const u8 as *const libc::c_char);
    logreg(
        fname,
        yspec,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        x1list,
        marks,
        mnsname,
        level,
    );
    dataset(
        grpname,
        lgrname,
        b"APPEND\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(srtarg, marks);
    strcat(srtarg, b" _type_ \0" as *const u8 as *const libc::c_char);
    strcat(srtarg, x1list);
    sort(
        lgrname,
        srtarg,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(srtname, lgrname);
    strcat(srtname, b".srt\0" as *const u8 as *const libc::c_char);
    strcpy(plotvars, x1list0);
    strcat(plotvars, b" \0" as *const u8 as *const libc::c_char);
    strcat(plotvars, casevar0);
    strcpy(plotmarks, marks);
    strcat(plotmarks, b" _type_\0" as *const u8 as *const libc::c_char);
    plotstyle = dap_malloc(
        (strlen(style)).wrapping_add(4 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(plotstyle, b"o4 \0" as *const u8 as *const libc::c_char);
    strcat(plotstyle, style);
    p = plot(
        srtname,
        plotvars,
        plotmarks,
        plotstyle,
        None,
        None,
        4 as libc::c_int * nmarks,
    );
    pn = 0 as libc::c_int;
    while pn < nmarks {
        strcpy(
            ((*p.offset((4 as libc::c_int * pn + 0 as libc::c_int) as isize)).pict_type)
                .as_mut_ptr(),
            b"LINE\0" as *const u8 as *const libc::c_char,
        );
        strcpy(
            ((*p.offset((4 as libc::c_int * pn + 2 as libc::c_int) as isize)).pict_type)
                .as_mut_ptr(),
            b"LINE\0" as *const u8 as *const libc::c_char,
        );
        strcpy(
            ((*p.offset((4 as libc::c_int * pn + 3 as libc::c_int) as isize)).pict_type)
                .as_mut_ptr(),
            b"LINE\0" as *const u8 as *const libc::c_char,
        );
        (*p.offset((4 as libc::c_int * pn + 0 as libc::c_int) as isize))
            .pict_dash = 4.0f64;
        (*p.offset((4 as libc::c_int * pn + 3 as libc::c_int) as isize))
            .pict_dash = 4.0f64;
        pn += 1;
        pn;
    }
    dap_free(
        yspec as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        x1list as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        trlname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        grpname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        grparg as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        grpvar as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        mnsarg as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        mnsname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        lgrname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        srtarg as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        srtname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        plotvars as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        plotmarks as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        casevar as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        casevar0 as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        plotstyle as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return p;
}
pub unsafe extern "C" fn plotmeans(
    mut dataset_0: *mut libc::c_char,
    mut meanvar0: *mut libc::c_char,
    mut varlist0: *mut libc::c_char,
    mut errbar: *mut libc::c_char,
    mut style: *mut libc::c_char,
    mut partvars: *mut libc::c_char,
    mut noverlay: libc::c_int,
) -> *mut pict {
    let mut meanv: [libc::c_int; 1] = [0; 1];
    let mut partv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut npartv: libc::c_int = 0;
    let mut meanvar: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut varlist: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    let mut mnslist: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mnsname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut errname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut srtname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ebar: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut overstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: libc::c_int = 0;
    let mut scale: libc::c_double = 0.;
    let mut typen: libc::c_int = 0;
    let mut mean: libc::c_double = 0.;
    let mut err: libc::c_double = 0.;
    let mut n: libc::c_int = 0;
    let mut srtarg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut plotvars: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut plotparts: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut pict = 0 as *mut pict;
    let mut more: libc::c_int = 0;
    let mut nparts: libc::c_int = 0;
    let mut pn: libc::c_int = 0;
    partv = 0 as *mut libc::c_int;
    mean = 0.0f64;
    err = 0.0f64;
    meanvar = dap_malloc(
        (strlen(meanvar0)).wrapping_add(1 as libc::c_int as libc::c_ulong)
            as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(meanvar, meanvar0);
    l = 0 as libc::c_int;
    while *meanvar.offset(l as isize) as libc::c_int != 0
        && *meanvar.offset(l as isize) as libc::c_int != '`' as i32
    {
        l += 1;
        l;
    }
    *meanvar.offset(l as isize) = '\0' as i32 as libc::c_char;
    varlist = dap_malloc(
        (strlen(varlist0)).wrapping_add(1 as libc::c_int as libc::c_ulong)
            as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(varlist, varlist0);
    l = 0 as libc::c_int;
    while *varlist.offset(l as isize) as libc::c_int != 0
        && *varlist.offset(l as isize) as libc::c_int != '`' as i32
    {
        l += 1;
        l;
    }
    *varlist.offset(l as isize) = '\0' as i32 as libc::c_char;
    if !partvars.is_null()
        && *partvars.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        partv = dap_malloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_int;
        mnslist = dap_malloc(
            (strlen(varlist))
                .wrapping_add(strlen(partvars))
                .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        strcpy(mnslist, partvars);
        strcat(mnslist, b" \0" as *const u8 as *const libc::c_char);
        strcat(mnslist, varlist);
        plotparts = dap_malloc(
            (strlen(partvars)).wrapping_add(8 as libc::c_int as libc::c_ulong)
                as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        strcpy(plotparts, partvars);
        strcat(plotparts, b" _type_\0" as *const u8 as *const libc::c_char);
    } else {
        mnslist = varlist;
        plotparts = b"_type_\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    mnsname = dap_malloc(
        (strlen(dataset_0)).wrapping_add(5 as libc::c_int as libc::c_ulong)
            as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(mnsname, dataset_0);
    strcat(mnsname, b".mns\0" as *const u8 as *const libc::c_char);
    errname = dap_malloc(
        (strlen(dataset_0)).wrapping_add(5 as libc::c_int as libc::c_ulong)
            as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(errname, dataset_0);
    strcat(errname, b".err\0" as *const u8 as *const libc::c_char);
    srtname = dap_malloc(
        (strlen(errname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(srtname, errname);
    strcat(srtname, b".srt\0" as *const u8 as *const libc::c_char);
    ebar = dap_malloc(
        (strlen(errbar)).wrapping_add(6 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(ebar, b"MEAN \0" as *const u8 as *const libc::c_char);
    strcat(ebar, errbar);
    overstr = dap_malloc(
        (8 as libc::c_int as libc::c_ulong).wrapping_add(strlen(style)) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if noverlay < 1 as libc::c_int {
        noverlay = 1 as libc::c_int;
    }
    sprintf(
        overstr,
        b"o%d %s\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int * noverlay,
        style,
    );
    srtarg = dap_malloc(
        (strlen(mnslist)).wrapping_add(8 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !partvars.is_null()
        && *partvars.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        strcpy(srtarg, partvars);
        strcat(srtarg, b" \0" as *const u8 as *const libc::c_char);
    } else {
        *srtarg.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    }
    strcat(srtarg, b"_type_ \0" as *const u8 as *const libc::c_char);
    strcat(srtarg, varlist);
    plotvars = dap_malloc(
        (strlen(meanvar0))
            .wrapping_add(strlen(varlist0))
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(plotvars, varlist0);
    strcat(plotvars, b" \0" as *const u8 as *const libc::c_char);
    strcat(plotvars, meanvar0);
    e = 0 as libc::c_int;
    while *errbar.offset(e as isize) as libc::c_int == ' ' as i32 {
        e += 1;
        e;
    }
    while *errbar.offset(e as isize) as libc::c_int != 0
        && *errbar.offset(e as isize) as libc::c_int != ' ' as i32
    {
        e += 1;
        e;
    }
    *ebar.offset((e + 5 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
    while *errbar.offset(e as isize) as libc::c_int == ' ' as i32 {
        e += 1;
        e;
    }
    if *errbar.offset(e as isize) != 0 {
        if sscanf(
            errbar.offset(e as isize),
            b"%lf\0" as *const u8 as *const libc::c_char,
            &mut scale as *mut libc::c_double,
        ) != 1 as libc::c_int
        {
            fprintf(
                stderr,
                b"%s: bad scale in call to plotmeans: %s\n\0" as *const u8
                    as *const libc::c_char,
                dap_dapname,
                errbar.offset(e as isize),
            );
            exit(1 as libc::c_int);
        }
    } else {
        scale = 1.0f64;
    }
    means(dataset_0, meanvar, ebar, mnslist);
    inset(mnsname);
    outset(errname, b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    dap_list(varlist, meanv.as_mut_ptr(), 1 as libc::c_int);
    dap_list(meanvar, meanv.as_mut_ptr(), 1 as libc::c_int);
    if !partvars.is_null()
        && *partvars.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        npartv = dap_list(partvars, partv, dap_maxvar);
    } else {
        npartv = 0 as libc::c_int;
    }
    typen = dap_varnum(
        b"_type_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if typen < 0 as libc::c_int {
        fprintf(
            stderr,
            b"%s: missing _type_ variable for plotmeans\n\0" as *const u8
                as *const libc::c_char,
            dap_dapname,
        );
        exit(1 as libc::c_int);
    }
    n = 0 as libc::c_int;
    nparts = 0 as libc::c_int;
    more = 1 as libc::c_int;
    while more != 0 {
        more = step();
        if more != 0 {
            if strcmp(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typen as isize),
                b"MEAN\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                mean = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .do_dbl)
                    .offset(meanv[0 as libc::c_int as usize] as isize);
            } else {
                err = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(meanv[0 as libc::c_int as usize] as isize);
            }
            n += 1;
            if n == 2 as libc::c_int {
                strcpy(
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                        .offset(typen as isize),
                    b"MEAN\0" as *const u8 as *const libc::c_char,
                );
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(meanv[0 as libc::c_int as usize] as isize) = mean;
                output();
                strcpy(
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                        .offset(typen as isize),
                    b"BAR\0" as *const u8 as *const libc::c_char,
                );
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        meanv[0 as libc::c_int as usize] as isize,
                    ) = mean - err * scale;
                output();
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        meanv[0 as libc::c_int as usize] as isize,
                    ) = mean + err * scale;
                output();
                n = 0 as libc::c_int;
            }
        }
        if dap_newpart(partv, npartv) != 0 {
            nparts += 1;
            nparts;
        }
    }
    sort(
        errname,
        srtarg,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    p = plot(
        srtname,
        plotvars,
        plotparts,
        overstr,
        None,
        None,
        2 as libc::c_int * nparts,
    );
    pn = 0 as libc::c_int;
    while pn < nparts {
        strcpy(
            ((*p.offset((2 as libc::c_int * pn) as isize)).pict_type).as_mut_ptr(),
            b"IBEA\0" as *const u8 as *const libc::c_char,
        );
        pn += 1;
        pn;
    }
    if !partvars.is_null()
        && *partvars.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        dap_free(
            partv as *mut libc::c_void,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        dap_free(
            mnslist as *mut libc::c_void,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        dap_free(
            plotparts as *mut libc::c_void,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    dap_free(
        mnsname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        errname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        srtname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        ebar as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        overstr as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        srtarg as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        plotvars as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return p;
}
