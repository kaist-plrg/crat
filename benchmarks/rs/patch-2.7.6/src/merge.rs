use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    static mut conflict_style: conflict_style;
    fn similar(
        _: *const libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: size_t,
    ) -> bool;
    fn copy_till(_: *mut outstate, _: lin) -> bool;
    static mut debug: libc::c_int;
    static mut verbosity: C2RustUnnamed;
    static mut canonicalize_ws: bool;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut last_frozen_line: lin;
    fn free(__ptr: *mut libc::c_void);
    static mut in_offset: lin;
    static mut out_offset: lin;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    static mut input_lines: lin;
    fn ifetch(_: lin, _: bool, _: *mut size_t) -> *const libc::c_char;
    fn pch_end() -> lin;
    fn pch_first() -> lin;
    fn pch_prefix_context() -> lin;
    fn pch_ptrn_lines() -> lin;
    fn pch_suffix_context() -> lin;
    fn pch_write_line(_: lin, _: *mut FILE) -> bool;
    fn pfetch(_: lin) -> *mut libc::c_char;
    fn pch_char(_: lin) -> libc::c_char;
    fn pch_line_len(_: lin) -> size_t;
    fn pch_normalize(_: diff);
    fn say(_: *const libc::c_char, _: ...);
    fn format_linenum(_: *mut libc::c_char, _: lin) -> *mut libc::c_char;
    fn write_fatal() -> !;
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
pub type off_t = __off_t;
pub type lin = off_t;
pub type C2RustUnnamed = libc::c_uint;
pub const VERBOSE: C2RustUnnamed = 2;
pub const SILENT: C2RustUnnamed = 1;
pub const DEFAULT_VERBOSITY: C2RustUnnamed = 0;
pub type diff = libc::c_uint;
pub const GIT_BINARY_DIFF: diff = 6;
pub const UNI_DIFF: diff = 5;
pub const NEW_CONTEXT_DIFF: diff = 4;
pub const ED_DIFF: diff = 3;
pub const NORMAL_DIFF: diff = 2;
pub const CONTEXT_DIFF: diff = 1;
pub const NO_DIFF: diff = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct outstate {
    pub ofp: *mut FILE,
    pub after_newline: bool,
    pub zero_output: bool,
}
pub type conflict_style = libc::c_uint;
pub const MERGE_DIFF3: conflict_style = 1;
pub const MERGE_MERGE: conflict_style = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct context {
    pub xchar: *mut libc::c_char,
    pub ychar: *mut libc::c_char,
    pub fdiag: *mut lin,
    pub bdiag: *mut lin,
    pub heuristic: bool,
    pub too_expensive: lin,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct partition {
    pub xmid: lin,
    pub ymid: lin,
    pub lo_minimal: bool,
    pub hi_minimal: bool,
}
unsafe extern "C" fn bestmatch(
    mut xoff: lin,
    mut xlim: lin,
    mut yoff: lin,
    mut ylim: lin,
    mut min: lin,
    mut max: lin,
    mut py: *mut lin,
) -> lin {
    let mut current_block: u64;
    let dmin: lin = xoff - ylim;
    let dmax: lin = xlim - yoff;
    let fmid: lin = xoff - yoff;
    let mut fmin: lin = fmid;
    let mut fmax: lin = fmid;
    let mut V: *mut lin = 0 as *mut lin;
    let mut fd: *mut lin = 0 as *mut lin;
    let mut fmid_plus_2_min: lin = 0;
    let mut ymax: lin = -(1 as libc::c_int) as lin;
    let mut c: lin = 0;
    V = xmalloc(
        ((2 as libc::c_int as libc::c_long * max + 3 as libc::c_int as libc::c_long)
            as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<lin>() as libc::c_ulong),
    ) as *mut lin;
    fd = V
        .offset(max as isize)
        .offset(1 as libc::c_int as isize)
        .offset(-(fmid as isize));
    if min != 0 {
        fmid_plus_2_min = fmid + 2 as libc::c_int as libc::c_long * min;
        min += yoff;
        if min > ylim {
            c = max + 1 as libc::c_int as libc::c_long;
            current_block = 362341649396712597;
        } else {
            current_block = 1394248824506584008;
        }
    } else {
        fmid_plus_2_min = 0 as libc::c_int as lin;
        current_block = 1394248824506584008;
    }
    match current_block {
        1394248824506584008 => {
            if py.is_null() {
                min = ylim;
            }
            while xoff < xlim && yoff < ylim
                && context_matches_file(xoff, yoff) as libc::c_int != 0
            {
                xoff += 1;
                xoff;
                yoff += 1;
                yoff;
            }
            if xoff == xlim && yoff >= min && xoff + yoff >= fmid_plus_2_min {
                ymax = yoff;
                c = 0 as libc::c_int as lin;
            } else {
                *fd.offset(fmid as isize) = xoff;
                c = 1 as libc::c_int as lin;
                's_92: while c <= max {
                    let mut d: lin = 0;
                    if fmin > dmin {
                        fmin -= 1;
                        *fd
                            .offset(
                                (fmin - 1 as libc::c_int as libc::c_long) as isize,
                            ) = -(1 as libc::c_int) as lin;
                    } else {
                        fmin += 1;
                        fmin;
                    }
                    if fmax < dmax {
                        fmax += 1;
                        *fd
                            .offset(
                                (fmax + 1 as libc::c_int as libc::c_long) as isize,
                            ) = -(1 as libc::c_int) as lin;
                    } else {
                        fmax -= 1;
                        fmax;
                    }
                    d = fmax;
                    while d >= fmin {
                        let mut x: lin = 0;
                        let mut y: lin = 0;
                        if *fd.offset((d - 1 as libc::c_int as libc::c_long) as isize)
                            < *fd.offset((d + 1 as libc::c_int as libc::c_long) as isize)
                        {
                            x = *fd
                                .offset((d + 1 as libc::c_int as libc::c_long) as isize);
                        } else {
                            x = *fd
                                .offset((d - 1 as libc::c_int as libc::c_long) as isize)
                                + 1 as libc::c_int as libc::c_long;
                        }
                        y = x - d;
                        while x < xlim && y < ylim
                            && context_matches_file(x, y) as libc::c_int != 0
                        {
                            x += 1;
                            x;
                            y += 1;
                            y;
                        }
                        *fd.offset(d as isize) = x;
                        if x == xlim && y >= min && x + y - c >= fmid_plus_2_min {
                            if ymax < y {
                                ymax = y;
                            }
                            if y == ylim {
                                break 's_92;
                            }
                        }
                        d -= 2 as libc::c_int as libc::c_long;
                    }
                    if ymax != -(1 as libc::c_int) as libc::c_long {
                        break;
                    }
                    c += 1;
                    c;
                }
            }
            if !py.is_null() {
                *py = ymax;
            }
        }
        _ => {}
    }
    free(V as *mut libc::c_void);
    return c;
}
unsafe extern "C" fn diag(
    mut xoff: lin,
    mut xlim: lin,
    mut yoff: lin,
    mut ylim: lin,
    mut find_minimal: bool,
    mut part: *mut partition,
    mut ctxt: *mut context,
) {
    let fd: *mut lin = (*ctxt).fdiag;
    let bd: *mut lin = (*ctxt).bdiag;
    let dmin: lin = xoff - ylim;
    let dmax: lin = xlim - yoff;
    let fmid: lin = xoff - yoff;
    let bmid: lin = xlim - ylim;
    let mut fmin: lin = fmid;
    let mut fmax: lin = fmid;
    let mut bmin: lin = bmid;
    let mut bmax: lin = bmid;
    let mut c: lin = 0;
    let mut odd: bool = fmid - bmid & 1 as libc::c_int as libc::c_long != 0;
    *fd.offset(fmid as isize) = xoff;
    *bd.offset(bmid as isize) = xlim;
    c = 1 as libc::c_int as lin;
    loop {
        let mut d: lin = 0;
        let mut big_snake: bool = 0 as libc::c_int != 0;
        if fmin > dmin {
            fmin -= 1;
            *fd
                .offset(
                    (fmin - 1 as libc::c_int as libc::c_long) as isize,
                ) = -(1 as libc::c_int) as lin;
        } else {
            fmin += 1;
            fmin;
        }
        if fmax < dmax {
            fmax += 1;
            *fd
                .offset(
                    (fmax + 1 as libc::c_int as libc::c_long) as isize,
                ) = -(1 as libc::c_int) as lin;
        } else {
            fmax -= 1;
            fmax;
        }
        d = fmax;
        while d >= fmin {
            let mut x: lin = 0;
            let mut y: lin = 0;
            let mut tlo: lin = *fd
                .offset((d - 1 as libc::c_int as libc::c_long) as isize);
            let mut thi: lin = *fd
                .offset((d + 1 as libc::c_int as libc::c_long) as isize);
            let mut x0: lin = if tlo < thi {
                thi
            } else {
                tlo + 1 as libc::c_int as libc::c_long
            };
            x = x0;
            y = x0 - d;
            while x < xlim && y < ylim && context_matches_file(x, y) as libc::c_int != 0
            {
                x += 1;
                x;
                y += 1;
                y;
            }
            if x - x0 > 20 as libc::c_int as libc::c_long {
                big_snake = 1 as libc::c_int != 0;
            }
            *fd.offset(d as isize) = x;
            if odd as libc::c_int != 0 && bmin <= d && d <= bmax
                && *bd.offset(d as isize) <= x
            {
                (*part).xmid = x;
                (*part).ymid = y;
                (*part).hi_minimal = 1 as libc::c_int != 0;
                (*part).lo_minimal = (*part).hi_minimal;
                return;
            }
            d -= 2 as libc::c_int as libc::c_long;
        }
        if bmin > dmin {
            bmin -= 1;
            *bd
                .offset(
                    (bmin - 1 as libc::c_int as libc::c_long) as isize,
                ) = (((1 as libc::c_int as lin)
                << (::std::mem::size_of::<lin>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long;
        } else {
            bmin += 1;
            bmin;
        }
        if bmax < dmax {
            bmax += 1;
            *bd
                .offset(
                    (bmax + 1 as libc::c_int as libc::c_long) as isize,
                ) = (((1 as libc::c_int as lin)
                << (::std::mem::size_of::<lin>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long;
        } else {
            bmax -= 1;
            bmax;
        }
        d = bmax;
        while d >= bmin {
            let mut x_0: lin = 0;
            let mut y_0: lin = 0;
            let mut tlo_0: lin = *bd
                .offset((d - 1 as libc::c_int as libc::c_long) as isize);
            let mut thi_0: lin = *bd
                .offset((d + 1 as libc::c_int as libc::c_long) as isize);
            let mut x0_0: lin = if tlo_0 < thi_0 {
                tlo_0
            } else {
                thi_0 - 1 as libc::c_int as libc::c_long
            };
            x_0 = x0_0;
            y_0 = x0_0 - d;
            while xoff < x_0 && yoff < y_0
                && context_matches_file(
                    x_0 - 1 as libc::c_int as libc::c_long,
                    y_0 - 1 as libc::c_int as libc::c_long,
                ) as libc::c_int != 0
            {
                x_0 -= 1;
                x_0;
                y_0 -= 1;
                y_0;
            }
            if x0_0 - x_0 > 20 as libc::c_int as libc::c_long {
                big_snake = 1 as libc::c_int != 0;
            }
            *bd.offset(d as isize) = x_0;
            if !odd && fmin <= d && d <= fmax && x_0 <= *fd.offset(d as isize) {
                (*part).xmid = x_0;
                (*part).ymid = y_0;
                (*part).hi_minimal = 1 as libc::c_int != 0;
                (*part).lo_minimal = (*part).hi_minimal;
                return;
            }
            d -= 2 as libc::c_int as libc::c_long;
        }
        if !find_minimal {
            let mut heuristic: bool = (*ctxt).heuristic;
            if (200 as libc::c_int as libc::c_long) < c && big_snake as libc::c_int != 0
                && heuristic as libc::c_int != 0
            {
                let mut best: lin = 0 as libc::c_int as lin;
                d = fmax;
                while d >= fmin {
                    let mut dd: lin = d - fmid;
                    let mut x_1: lin = *fd.offset(d as isize);
                    let mut y_1: lin = x_1 - d;
                    let mut v: lin = (x_1 - xoff) * 2 as libc::c_int as libc::c_long
                        - dd;
                    if v
                        > 12 as libc::c_int as libc::c_long
                            * (c
                                + (if dd < 0 as libc::c_int as libc::c_long {
                                    -dd
                                } else {
                                    dd
                                }))
                    {
                        if v > best && xoff + 20 as libc::c_int as libc::c_long <= x_1
                            && x_1 < xlim
                            && yoff + 20 as libc::c_int as libc::c_long <= y_1
                            && y_1 < ylim
                        {
                            let mut k: libc::c_int = 0;
                            k = 1 as libc::c_int;
                            while context_matches_file(
                                x_1 - k as libc::c_long,
                                y_1 - k as libc::c_long,
                            ) {
                                if k == 20 as libc::c_int {
                                    best = v;
                                    (*part).xmid = x_1;
                                    (*part).ymid = y_1;
                                    break;
                                } else {
                                    k += 1;
                                    k;
                                }
                            }
                        }
                    }
                    d -= 2 as libc::c_int as libc::c_long;
                }
                if best > 0 as libc::c_int as libc::c_long {
                    (*part).lo_minimal = 1 as libc::c_int != 0;
                    (*part).hi_minimal = 0 as libc::c_int != 0;
                    return;
                }
                let mut best_0: lin = 0 as libc::c_int as lin;
                d = bmax;
                while d >= bmin {
                    let mut dd_0: lin = d - bmid;
                    let mut x_2: lin = *bd.offset(d as isize);
                    let mut y_2: lin = x_2 - d;
                    let mut v_0: lin = (xlim - x_2) * 2 as libc::c_int as libc::c_long
                        + dd_0;
                    if v_0
                        > 12 as libc::c_int as libc::c_long
                            * (c
                                + (if dd_0 < 0 as libc::c_int as libc::c_long {
                                    -dd_0
                                } else {
                                    dd_0
                                }))
                    {
                        if v_0 > best_0 && xoff < x_2
                            && x_2 <= xlim - 20 as libc::c_int as libc::c_long
                            && yoff < y_2
                            && y_2 <= ylim - 20 as libc::c_int as libc::c_long
                        {
                            let mut k_0: libc::c_int = 0;
                            k_0 = 0 as libc::c_int;
                            while context_matches_file(
                                x_2 + k_0 as libc::c_long,
                                y_2 + k_0 as libc::c_long,
                            ) {
                                if k_0 == 20 as libc::c_int - 1 as libc::c_int {
                                    best_0 = v_0;
                                    (*part).xmid = x_2;
                                    (*part).ymid = y_2;
                                    break;
                                } else {
                                    k_0 += 1;
                                    k_0;
                                }
                            }
                        }
                    }
                    d -= 2 as libc::c_int as libc::c_long;
                }
                if best_0 > 0 as libc::c_int as libc::c_long {
                    (*part).lo_minimal = 0 as libc::c_int != 0;
                    (*part).hi_minimal = 1 as libc::c_int != 0;
                    return;
                }
            }
            if c >= (*ctxt).too_expensive {
                let mut fxybest: lin = 0;
                let mut fxbest: lin = 0;
                let mut bxybest: lin = 0;
                let mut bxbest: lin = 0;
                fxybest = -(1 as libc::c_int) as lin;
                d = fmax;
                while d >= fmin {
                    let mut x_3: lin = if *fd.offset(d as isize) < xlim {
                        *fd.offset(d as isize)
                    } else {
                        xlim
                    };
                    let mut y_3: lin = x_3 - d;
                    if ylim < y_3 {
                        x_3 = ylim + d;
                        y_3 = ylim;
                    }
                    if fxybest < x_3 + y_3 {
                        fxybest = x_3 + y_3;
                        fxbest = x_3;
                    }
                    d -= 2 as libc::c_int as libc::c_long;
                }
                bxybest = (((1 as libc::c_int as lin)
                    << (::std::mem::size_of::<lin>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long
                    + 1 as libc::c_int as libc::c_long;
                d = bmax;
                while d >= bmin {
                    let mut x_4: lin = if xoff > *bd.offset(d as isize) {
                        xoff
                    } else {
                        *bd.offset(d as isize)
                    };
                    let mut y_4: lin = x_4 - d;
                    if y_4 < yoff {
                        x_4 = yoff + d;
                        y_4 = yoff;
                    }
                    if x_4 + y_4 < bxybest {
                        bxybest = x_4 + y_4;
                        bxbest = x_4;
                    }
                    d -= 2 as libc::c_int as libc::c_long;
                }
                if xlim + ylim - bxybest < fxybest - (xoff + yoff) {
                    (*part).xmid = fxbest;
                    (*part).ymid = fxybest - fxbest;
                    (*part).lo_minimal = 1 as libc::c_int != 0;
                    (*part).hi_minimal = 0 as libc::c_int != 0;
                } else {
                    (*part).xmid = bxbest;
                    (*part).ymid = bxybest - bxbest;
                    (*part).lo_minimal = 0 as libc::c_int != 0;
                    (*part).hi_minimal = 1 as libc::c_int != 0;
                }
                return;
            }
        }
        c += 1;
        c;
    };
}
unsafe extern "C" fn compareseq(
    mut xoff: lin,
    mut xlim: lin,
    mut yoff: lin,
    mut ylim: lin,
    mut find_minimal: bool,
    mut ctxt: *mut context,
) -> bool {
    while xoff < xlim && yoff < ylim
        && context_matches_file(xoff, yoff) as libc::c_int != 0
    {
        xoff += 1;
        xoff;
        yoff += 1;
        yoff;
    }
    while xoff < xlim && yoff < ylim
        && context_matches_file(
            xlim - 1 as libc::c_int as libc::c_long,
            ylim - 1 as libc::c_int as libc::c_long,
        ) as libc::c_int != 0
    {
        xlim -= 1;
        xlim;
        ylim -= 1;
        ylim;
    }
    if xoff == xlim {
        while yoff < ylim {
            *((*ctxt).ychar).offset(yoff as isize) = '+' as i32 as libc::c_char;
            yoff += 1;
            yoff;
        }
    } else if yoff == ylim {
        while xoff < xlim {
            *((*ctxt).xchar).offset(xoff as isize) = '-' as i32 as libc::c_char;
            xoff += 1;
            xoff;
        }
    } else {
        let mut part: partition = partition {
            xmid: 0,
            ymid: 0,
            lo_minimal: false,
            hi_minimal: false,
        };
        diag(xoff, xlim, yoff, ylim, find_minimal, &mut part, ctxt);
        if compareseq(xoff, part.xmid, yoff, part.ymid, part.lo_minimal, ctxt) {
            return 1 as libc::c_int != 0;
        }
        if compareseq(part.xmid, xlim, part.ymid, ylim, part.hi_minimal, ctxt) {
            return 1 as libc::c_int != 0;
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn locate_merge(mut matched: *mut lin) -> lin {
    let mut first_guess: lin = pch_first() + in_offset;
    let mut pat_lines: lin = pch_ptrn_lines();
    let mut context_lines: lin = count_context_lines();
    let mut max_where: lin = input_lines - pat_lines + context_lines
        + 1 as libc::c_int as libc::c_long;
    let mut min_where: lin = last_frozen_line + 1 as libc::c_int as libc::c_long;
    let mut max_pos_offset: lin = max_where - first_guess;
    let mut max_neg_offset: lin = first_guess - min_where;
    let mut max_offset: lin = if max_pos_offset < max_neg_offset {
        max_neg_offset
    } else {
        max_pos_offset
    };
    let mut where_0: lin = first_guess;
    let mut max_matched: lin = 0 as libc::c_int as lin;
    let mut min: lin = 0;
    let mut max: lin = 0;
    let mut offset: lin = 0;
    let mut match_until_eof: bool = false;
    if !(context_lines == 0 as libc::c_int as libc::c_long) {
        max = 2 as libc::c_int as libc::c_long * context_lines;
        min = pat_lines - context_lines;
        if debug & 1 as libc::c_int != 0 {
            let mut numbuf0: [libc::c_char; 23] = [0; 23];
            let mut numbuf1: [libc::c_char; 23] = [0; 23];
            say(
                b"locating merge: min=%s max=%s \0" as *const u8 as *const libc::c_char,
                format_linenum(numbuf0.as_mut_ptr(), min),
                format_linenum(numbuf1.as_mut_ptr(), max),
            );
        }
        offset = pch_suffix_context() - pch_prefix_context();
        if offset > 0 as libc::c_int as libc::c_long
            && pch_first() <= 1 as libc::c_int as libc::c_long
        {
            max_pos_offset = 0 as libc::c_int as lin;
        }
        match_until_eof = offset < 0 as libc::c_int as libc::c_long;
        if first_guess <= max_neg_offset {
            max_neg_offset = first_guess - 1 as libc::c_int as libc::c_long;
        }
        offset = 0 as libc::c_int as lin;
        while offset <= max_offset {
            if offset <= max_pos_offset {
                let mut guess: lin = first_guess + offset;
                let mut last: lin = 0;
                let mut changes: lin = 0;
                changes = bestmatch(
                    1 as libc::c_int as lin,
                    pat_lines + 1 as libc::c_int as libc::c_long,
                    guess,
                    input_lines + 1 as libc::c_int as libc::c_long,
                    if match_until_eof as libc::c_int != 0 {
                        input_lines - guess + 1 as libc::c_int as libc::c_long
                    } else {
                        min
                    },
                    max,
                    &mut last,
                );
                if changes <= max && max_matched < last - guess {
                    max_matched = last - guess;
                    where_0 = guess;
                    if changes == 0 as libc::c_int as libc::c_long {
                        break;
                    }
                    min = last - guess;
                    max = changes - 1 as libc::c_int as libc::c_long;
                }
            }
            if (0 as libc::c_int as libc::c_long) < offset && offset <= max_neg_offset {
                let mut guess_0: lin = first_guess - offset;
                let mut last_0: lin = 0;
                let mut changes_0: lin = 0;
                changes_0 = bestmatch(
                    1 as libc::c_int as lin,
                    pat_lines + 1 as libc::c_int as libc::c_long,
                    guess_0,
                    input_lines + 1 as libc::c_int as libc::c_long,
                    if match_until_eof as libc::c_int != 0 {
                        input_lines - guess_0 + 1 as libc::c_int as libc::c_long
                    } else {
                        min
                    },
                    max,
                    &mut last_0,
                );
                if changes_0 <= max && max_matched < last_0 - guess_0 {
                    max_matched = last_0 - guess_0;
                    where_0 = guess_0;
                    if changes_0 == 0 as libc::c_int as libc::c_long {
                        break;
                    }
                    min = last_0 - guess_0;
                    max = changes_0 - 1 as libc::c_int as libc::c_long;
                }
            }
            offset += 1;
            offset;
        }
        if debug & 1 as libc::c_int != 0 {
            let mut numbuf0_0: [libc::c_char; 23] = [0; 23];
            let mut numbuf1_0: [libc::c_char; 23] = [0; 23];
            let mut numbuf2: [libc::c_char; 23] = [0; 23];
            say(
                b"where=%s matched=%s changes=%s\n\0" as *const u8
                    as *const libc::c_char,
                format_linenum(numbuf0_0.as_mut_ptr(), where_0),
                format_linenum(numbuf1_0.as_mut_ptr(), max_matched),
                format_linenum(
                    numbuf2.as_mut_ptr(),
                    max + 1 as libc::c_int as libc::c_long,
                ),
            );
        }
    }
    *matched = max_matched;
    if where_0 < min_where {
        where_0 = min_where;
    }
    return where_0;
}
unsafe extern "C" fn print_linerange(mut from: lin, mut to: lin) {
    let mut numbuf0: [libc::c_char; 23] = [0; 23];
    let mut numbuf1: [libc::c_char; 23] = [0; 23];
    if to <= from {
        printf(
            b"%s\0" as *const u8 as *const libc::c_char,
            format_linenum(numbuf0.as_mut_ptr(), from),
        );
    } else {
        printf(
            b"%s-%s\0" as *const u8 as *const libc::c_char,
            format_linenum(numbuf0.as_mut_ptr(), from),
            format_linenum(numbuf1.as_mut_ptr(), to),
        );
    };
}
unsafe extern "C" fn merge_result(
    mut first_result: *mut bool,
    mut hunk: libc::c_int,
    mut what: *const libc::c_char,
    mut from: lin,
    mut to: lin,
) {
    static mut last_what: *const libc::c_char = 0 as *const libc::c_char;
    if *first_result as libc::c_int != 0 && !what.is_null() {
        printf(b"Hunk #%d %s at \0" as *const u8 as *const libc::c_char, hunk, what);
        last_what = what;
    } else if what.is_null() {
        if !*first_result {
            fputs(b".\n\0" as *const u8 as *const libc::c_char, stdout);
            fflush(stdout);
            last_what = 0 as *const libc::c_char;
        }
        return;
    } else if last_what == what {
        fputs(b",\0" as *const u8 as *const libc::c_char, stdout);
    } else {
        printf(b", %s at \0" as *const u8 as *const libc::c_char, what);
    }
    print_linerange(from + out_offset, to + out_offset);
    *first_result = 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn merge_hunk(
    mut hunk: libc::c_int,
    mut outstate: *mut outstate,
    mut where_0: lin,
    mut somefailed: *mut bool,
) -> bool {
    let mut current_block: u64;
    let mut applies_cleanly: bool = false;
    let mut first_result: bool = 1 as libc::c_int != 0;
    let mut already_applied: bool = false;
    let mut fp: *mut FILE = (*outstate).ofp;
    let mut old: lin = 1 as libc::c_int as lin;
    let mut firstold: lin = pch_ptrn_lines();
    let mut new: lin = firstold + 1 as libc::c_int as libc::c_long;
    let mut firstnew: lin = pch_end();
    let mut in_0: lin = 0;
    let mut firstin: lin = 0;
    let mut oldin: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut matched: lin = 0;
    let mut lastwhere: lin = 0;
    pch_normalize(UNI_DIFF);
    if pch_char(firstnew + 1 as libc::c_int as libc::c_long) as libc::c_int == '^' as i32
    {} else {
        __assert_fail(
            b"pch_char (firstnew + 1) == '^'\0" as *const u8 as *const libc::c_char,
            b"merge.c\0" as *const u8 as *const libc::c_char,
            218 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"_Bool merge_hunk(int, struct outstate *, lin, _Bool *)\0"))
                .as_ptr(),
        );
    }
    'c_9533: {
        if pch_char(firstnew + 1 as libc::c_int as libc::c_long) as libc::c_int
            == '^' as i32
        {} else {
            __assert_fail(
                b"pch_char (firstnew + 1) == '^'\0" as *const u8 as *const libc::c_char,
                b"merge.c\0" as *const u8 as *const libc::c_char,
                218 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"_Bool merge_hunk(int, struct outstate *, lin, _Bool *)\0"))
                    .as_ptr(),
            );
        }
    };
    while pch_char(new) as libc::c_int == '=' as i32
        || pch_char(new) as libc::c_int == '\n' as i32
    {
        new += 1;
        new;
    }
    if where_0 != 0 {
        applies_cleanly = 1 as libc::c_int != 0;
        matched = pch_ptrn_lines();
    } else {
        where_0 = locate_merge(&mut matched);
        applies_cleanly = 0 as libc::c_int != 0;
    }
    in_0 = firstold + 2 as libc::c_int as libc::c_long;
    oldin = xmalloc((in_0 + matched + 1 as libc::c_int as libc::c_long) as size_t)
        as *mut libc::c_char;
    memset(oldin as *mut libc::c_void, ' ' as i32, (in_0 + matched) as libc::c_ulong);
    *oldin.offset(0 as libc::c_int as isize) = '*' as i32 as libc::c_char;
    *oldin
        .offset(
            (in_0 - 1 as libc::c_int as libc::c_long) as isize,
        ) = '=' as i32 as libc::c_char;
    *oldin.offset((in_0 + matched) as isize) = '^' as i32 as libc::c_char;
    compute_changes(
        old,
        in_0 - 1 as libc::c_int as libc::c_long,
        where_0,
        where_0 + matched,
        oldin.offset(old as isize),
        oldin.offset(in_0 as isize),
    );
    if debug & 2 as libc::c_int != 0 {
        let mut numbuf0: [libc::c_char; 23] = [0; 23];
        let mut numbuf1: [libc::c_char; 23] = [0; 23];
        let mut n: lin = 0;
        fputc('\n' as i32, stderr);
        n = 0 as libc::c_int as lin;
        while n <= in_0 + matched {
            fprintf(
                stderr,
                b"%s %c\0" as *const u8 as *const libc::c_char,
                format_linenum(numbuf0.as_mut_ptr(), n),
                *oldin.offset(n as isize) as libc::c_int,
            );
            if n == 0 as libc::c_int as libc::c_long {
                fprintf(
                    stderr,
                    b" %s,%s\n\0" as *const u8 as *const libc::c_char,
                    format_linenum(numbuf0.as_mut_ptr(), pch_first()),
                    format_linenum(numbuf1.as_mut_ptr(), pch_ptrn_lines()),
                );
            } else if n <= firstold {
                fprintf(
                    stderr,
                    b" |%.*s\0" as *const u8 as *const libc::c_char,
                    pch_line_len(n) as libc::c_int,
                    pfetch(n),
                );
            } else if n == in_0 - 1 as libc::c_int as libc::c_long {
                fprintf(
                    stderr,
                    b" %s,%s\n\0" as *const u8 as *const libc::c_char,
                    format_linenum(numbuf0.as_mut_ptr(), where_0),
                    format_linenum(numbuf1.as_mut_ptr(), matched),
                );
            } else if n >= in_0 && n < in_0 + matched {
                let mut size: size_t = 0;
                let mut line: *const libc::c_char = 0 as *const libc::c_char;
                line = ifetch(where_0 + n - in_0, 0 as libc::c_int != 0, &mut size);
                fprintf(
                    stderr,
                    b" |%.*s\0" as *const u8 as *const libc::c_char,
                    size as libc::c_int,
                    line,
                );
            } else {
                fputc('\n' as i32, stderr);
            }
            n += 1;
            n;
        }
        fflush(stderr);
    }
    if last_frozen_line < where_0 - 1 as libc::c_int as libc::c_long {
        if !copy_till(outstate, where_0 - 1 as libc::c_int as libc::c_long) {
            return 0 as libc::c_int != 0;
        }
    }
    loop {
        firstold = old;
        firstnew = new;
        firstin = in_0;
        if pch_char(old) as libc::c_int == '-' as i32
            || pch_char(new) as libc::c_int == '+' as i32
        {
            let mut lines: lin = 0;
            loop {
                if !(pch_char(old) as libc::c_int == '-' as i32) {
                    current_block = 11763295167351361500;
                    break;
                }
                if *oldin.offset(old as isize) as libc::c_int == '-' as i32
                    || *oldin.offset(in_0 as isize) as libc::c_int == '+' as i32
                {
                    current_block = 8970507510497246736;
                    break;
                }
                if *oldin.offset(old as isize) as libc::c_int == ' ' as i32 {
                    if *oldin.offset(in_0 as isize) as libc::c_int == ' ' as i32
                    {} else {
                        __assert_fail(
                            b"oldin[in] == ' '\0" as *const u8 as *const libc::c_char,
                            b"merge.c\0" as *const u8 as *const libc::c_char,
                            300 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 55],
                                &[libc::c_char; 55],
                            >(
                                b"_Bool merge_hunk(int, struct outstate *, lin, _Bool *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_6368: {
                        if *oldin.offset(in_0 as isize) as libc::c_int == ' ' as i32
                        {} else {
                            __assert_fail(
                                b"oldin[in] == ' '\0" as *const u8 as *const libc::c_char,
                                b"merge.c\0" as *const u8 as *const libc::c_char,
                                300 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 55],
                                    &[libc::c_char; 55],
                                >(
                                    b"_Bool merge_hunk(int, struct outstate *, lin, _Bool *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    in_0 += 1;
                    in_0;
                }
                old += 1;
                old;
            }
            match current_block {
                8970507510497246736 => {}
                _ => {
                    if !(*oldin.offset(old as isize) as libc::c_int == '-' as i32
                        || *oldin.offset(in_0 as isize) as libc::c_int == '+' as i32)
                    {
                        while pch_char(new) as libc::c_int == '+' as i32 {
                            new += 1;
                            new;
                        }
                        lines = new - firstnew;
                        if verbosity as libc::c_uint
                            == VERBOSE as libc::c_int as libc::c_uint
                            || verbosity as libc::c_uint
                                != SILENT as libc::c_int as libc::c_uint && !applies_cleanly
                        {
                            merge_result(
                                &mut first_result,
                                hunk,
                                b"merged\0" as *const u8 as *const libc::c_char,
                                where_0,
                                where_0 + lines - 1 as libc::c_int as libc::c_long,
                            );
                        }
                        last_frozen_line += old - firstold;
                        where_0 += old - firstold;
                        out_offset += new - firstnew;
                        if firstnew < new {
                            while firstnew < new {
                                (*outstate).after_newline = pch_write_line(firstnew, fp);
                                firstnew += 1;
                                firstnew;
                            }
                            (*outstate).zero_output = 0 as libc::c_int != 0;
                        }
                        continue;
                    }
                }
            }
        } else if pch_char(old) as libc::c_int == ' ' as i32 {
            if *oldin.offset(old as isize) as libc::c_int == '-' as i32 {
                loop {
                    if !(pch_char(old) as libc::c_int == ' ' as i32) {
                        current_block = 10067844863897285902;
                        break;
                    }
                    if *oldin.offset(old as isize) as libc::c_int != '-' as i32 {
                        current_block = 10067844863897285902;
                        break;
                    }
                    if pch_char(new) as libc::c_int == '+' as i32 {
                        current_block = 8970507510497246736;
                        break;
                    }
                    if pch_char(new) as libc::c_int == ' ' as i32 {} else {
                        __assert_fail(
                            b"pch_char (new) == ' '\0" as *const u8
                                as *const libc::c_char,
                            b"merge.c\0" as *const u8 as *const libc::c_char,
                            340 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 55],
                                &[libc::c_char; 55],
                            >(
                                b"_Bool merge_hunk(int, struct outstate *, lin, _Bool *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_6122: {
                        if pch_char(new) as libc::c_int == ' ' as i32 {} else {
                            __assert_fail(
                                b"pch_char (new) == ' '\0" as *const u8
                                    as *const libc::c_char,
                                b"merge.c\0" as *const u8 as *const libc::c_char,
                                340 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 55],
                                    &[libc::c_char; 55],
                                >(
                                    b"_Bool merge_hunk(int, struct outstate *, lin, _Bool *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    old += 1;
                    old;
                    new += 1;
                    new;
                }
                match current_block {
                    8970507510497246736 => {}
                    _ => {
                        if !(pch_char(old) as libc::c_int == '-' as i32
                            || pch_char(new) as libc::c_int == '+' as i32)
                        {
                            continue;
                        }
                    }
                }
            } else {
                if *oldin.offset(in_0 as isize) as libc::c_int == '+' as i32 {
                    while *oldin.offset(in_0 as isize) as libc::c_int == '+' as i32 {
                        in_0 += 1;
                        in_0;
                    }
                    where_0 += in_0 - firstin;
                    if !copy_till(outstate, where_0 - 1 as libc::c_int as libc::c_long) {
                        return 0 as libc::c_int != 0;
                    }
                } else if *oldin.offset(old as isize) as libc::c_int == ' ' as i32 {
                    while pch_char(old) as libc::c_int == ' ' as i32
                        && *oldin.offset(old as isize) as libc::c_int == ' ' as i32
                        && pch_char(new) as libc::c_int == ' ' as i32
                        && *oldin.offset(in_0 as isize) as libc::c_int == ' ' as i32
                    {
                        old += 1;
                        old;
                        new += 1;
                        new;
                        in_0 += 1;
                        in_0;
                    }
                    where_0 += in_0 - firstin;
                    if !copy_till(outstate, where_0 - 1 as libc::c_int as libc::c_long) {
                        return 0 as libc::c_int != 0;
                    }
                }
                continue;
            }
        } else {
            if pch_char(old) as libc::c_int == '=' as i32
                && pch_char(new) as libc::c_int == '^' as i32
            {} else {
                __assert_fail(
                    b"pch_char (old) == '=' && pch_char (new) == '^'\0" as *const u8
                        as *const libc::c_char,
                    b"merge.c\0" as *const u8 as *const libc::c_char,
                    377 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 55],
                        &[libc::c_char; 55],
                    >(b"_Bool merge_hunk(int, struct outstate *, lin, _Bool *)\0"))
                        .as_ptr(),
                );
            }
            'c_5893: {
                if pch_char(old) as libc::c_int == '=' as i32
                    && pch_char(new) as libc::c_int == '^' as i32
                {} else {
                    __assert_fail(
                        b"pch_char (old) == '=' && pch_char (new) == '^'\0" as *const u8
                            as *const libc::c_char,
                        b"merge.c\0" as *const u8 as *const libc::c_char,
                        377 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 55],
                            &[libc::c_char; 55],
                        >(b"_Bool merge_hunk(int, struct outstate *, lin, _Bool *)\0"))
                            .as_ptr(),
                    );
                }
            };
            break;
        }
        loop {
            if pch_char(old) as libc::c_int == '-' as i32 {
                while *oldin.offset(in_0 as isize) as libc::c_int == '+' as i32 {
                    in_0 += 1;
                    in_0;
                }
                if *oldin.offset(old as isize) as libc::c_int == ' ' as i32 {
                    if *oldin.offset(in_0 as isize) as libc::c_int == ' ' as i32
                    {} else {
                        __assert_fail(
                            b"oldin[in] == ' '\0" as *const u8 as *const libc::c_char,
                            b"merge.c\0" as *const u8 as *const libc::c_char,
                            393 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 55],
                                &[libc::c_char; 55],
                            >(
                                b"_Bool merge_hunk(int, struct outstate *, lin, _Bool *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_5811: {
                        if *oldin.offset(in_0 as isize) as libc::c_int == ' ' as i32
                        {} else {
                            __assert_fail(
                                b"oldin[in] == ' '\0" as *const u8 as *const libc::c_char,
                                b"merge.c\0" as *const u8 as *const libc::c_char,
                                393 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 55],
                                    &[libc::c_char; 55],
                                >(
                                    b"_Bool merge_hunk(int, struct outstate *, lin, _Bool *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    in_0 += 1;
                    in_0;
                }
                old += 1;
                old;
            } else if *oldin.offset(old as isize) as libc::c_int == '-' as i32 {
                while pch_char(new) as libc::c_int == '+' as i32 {
                    new += 1;
                    new;
                }
                if pch_char(old) as libc::c_int == ' ' as i32 {
                    if pch_char(new) as libc::c_int == ' ' as i32 {} else {
                        __assert_fail(
                            b"pch_char (new) == ' '\0" as *const u8
                                as *const libc::c_char,
                            b"merge.c\0" as *const u8 as *const libc::c_char,
                            404 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 55],
                                &[libc::c_char; 55],
                            >(
                                b"_Bool merge_hunk(int, struct outstate *, lin, _Bool *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_5729: {
                        if pch_char(new) as libc::c_int == ' ' as i32 {} else {
                            __assert_fail(
                                b"pch_char (new) == ' '\0" as *const u8
                                    as *const libc::c_char,
                                b"merge.c\0" as *const u8 as *const libc::c_char,
                                404 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 55],
                                    &[libc::c_char; 55],
                                >(
                                    b"_Bool merge_hunk(int, struct outstate *, lin, _Bool *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    new += 1;
                    new;
                }
                old += 1;
                old;
            } else if pch_char(new) as libc::c_int == '+' as i32 {
                while pch_char(new) as libc::c_int == '+' as i32 {
                    new += 1;
                    new;
                }
            } else {
                if !(*oldin.offset(in_0 as isize) as libc::c_int == '+' as i32) {
                    break;
                }
                while *oldin.offset(in_0 as isize) as libc::c_int == '+' as i32 {
                    in_0 += 1;
                    in_0;
                }
            }
        }
        if (pch_char(old) as libc::c_int == ' ' as i32
            && pch_char(new) as libc::c_int == ' ' as i32
            || pch_char(old) as libc::c_int == '=' as i32
                && pch_char(new) as libc::c_int == '^' as i32)
            && (*oldin.offset(old as isize) as libc::c_int == ' ' as i32
                && *oldin.offset(in_0 as isize) as libc::c_int == ' ' as i32
                || *oldin.offset(old as isize) as libc::c_int == '=' as i32
                    && *oldin.offset(in_0 as isize) as libc::c_int == '^' as i32)
        {} else {
            __assert_fail(
                b"((pch_char (old) == ' ' && pch_char (new) == ' ') || (pch_char (old) == '=' && pch_char (new) == '^')) && ((oldin[old] == ' ' && oldin[in] == ' ') || (oldin[old] == '=' && oldin[in] == '^'))\0"
                    as *const u8 as *const libc::c_char,
                b"merge.c\0" as *const u8 as *const libc::c_char,
                421 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"_Bool merge_hunk(int, struct outstate *, lin, _Bool *)\0"))
                    .as_ptr(),
            );
        }
        'c_5470: {
            if (pch_char(old) as libc::c_int == ' ' as i32
                && pch_char(new) as libc::c_int == ' ' as i32
                || pch_char(old) as libc::c_int == '=' as i32
                    && pch_char(new) as libc::c_int == '^' as i32)
                && (*oldin.offset(old as isize) as libc::c_int == ' ' as i32
                    && *oldin.offset(in_0 as isize) as libc::c_int == ' ' as i32
                    || *oldin.offset(old as isize) as libc::c_int == '=' as i32
                        && *oldin.offset(in_0 as isize) as libc::c_int == '^' as i32)
            {} else {
                __assert_fail(
                    b"((pch_char (old) == ' ' && pch_char (new) == ' ') || (pch_char (old) == '=' && pch_char (new) == '^')) && ((oldin[old] == ' ' && oldin[in] == ' ') || (oldin[old] == '=' && oldin[in] == '^'))\0"
                        as *const u8 as *const libc::c_char,
                    b"merge.c\0" as *const u8 as *const libc::c_char,
                    421 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 55],
                        &[libc::c_char; 55],
                    >(b"_Bool merge_hunk(int, struct outstate *, lin, _Bool *)\0"))
                        .as_ptr(),
                );
            }
        };
        lastwhere = where_0;
        while firstin < in_0 && firstnew < new
            && context_matches_file(firstnew, lastwhere) as libc::c_int != 0
        {
            firstin += 1;
            firstin;
            firstnew += 1;
            firstnew;
            lastwhere += 1;
            lastwhere;
        }
        already_applied = firstin == in_0 && firstnew == new;
        if already_applied {
            merge_result(
                &mut first_result,
                hunk,
                b"already applied\0" as *const u8 as *const libc::c_char,
                where_0,
                lastwhere - 1 as libc::c_int as libc::c_long,
            );
        }
        if conflict_style as libc::c_uint == MERGE_DIFF3 as libc::c_int as libc::c_uint {
            let mut common_prefix: lin = lastwhere - where_0;
            firstin -= common_prefix;
            firstnew -= common_prefix;
            lastwhere -= common_prefix;
        }
        if where_0 != lastwhere {
            where_0 = lastwhere;
            if !copy_till(outstate, where_0 - 1 as libc::c_int as libc::c_long) {
                return 0 as libc::c_int != 0;
            }
        }
        if !already_applied {
            let mut common_suffix: lin = 0 as libc::c_int as lin;
            let mut lines_0: lin = 0;
            if conflict_style as libc::c_uint
                == MERGE_MERGE as libc::c_int as libc::c_uint
            {
                lastwhere = where_0 + (in_0 - firstin);
                while firstin < in_0 && firstnew < new
                    && context_matches_file(
                        new - 1 as libc::c_int as libc::c_long,
                        lastwhere - 1 as libc::c_int as libc::c_long,
                    ) as libc::c_int != 0
                {
                    in_0 -= 1;
                    in_0;
                    new -= 1;
                    new;
                    lastwhere -= 1;
                    lastwhere;
                    common_suffix += 1;
                    common_suffix;
                }
            }
            lines_0 = 3 as libc::c_int as libc::c_long + (in_0 - firstin)
                + (new - firstnew);
            if conflict_style as libc::c_uint
                == MERGE_DIFF3 as libc::c_int as libc::c_uint
            {
                lines_0 += 1 as libc::c_int as libc::c_long + (old - firstold);
            }
            merge_result(
                &mut first_result,
                hunk,
                b"NOT MERGED\0" as *const u8 as *const libc::c_char,
                where_0,
                where_0 + lines_0 - 1 as libc::c_int as libc::c_long,
            );
            out_offset += lines_0 - (in_0 - firstin);
            fputs(
                (b"\n<<<<<<<\n\0" as *const u8 as *const libc::c_char)
                    .offset((*outstate).after_newline as libc::c_int as isize),
                fp,
            );
            (*outstate).after_newline = 1 as libc::c_int != 0;
            if firstin < in_0 {
                where_0 += in_0 - firstin;
                if !copy_till(outstate, where_0 - 1 as libc::c_int as libc::c_long) {
                    return 0 as libc::c_int != 0;
                }
            }
            if conflict_style as libc::c_uint
                == MERGE_DIFF3 as libc::c_int as libc::c_uint
            {
                fputs(
                    (b"\n|||||||\n\0" as *const u8 as *const libc::c_char)
                        .offset((*outstate).after_newline as libc::c_int as isize),
                    fp,
                );
                (*outstate).after_newline = 1 as libc::c_int != 0;
                while firstold < old {
                    (*outstate).after_newline = pch_write_line(firstold, fp);
                    firstold += 1;
                    firstold;
                }
            }
            fputs(
                (b"\n=======\n\0" as *const u8 as *const libc::c_char)
                    .offset((*outstate).after_newline as libc::c_int as isize),
                fp,
            );
            (*outstate).after_newline = 1 as libc::c_int != 0;
            while firstnew < new {
                (*outstate).after_newline = pch_write_line(firstnew, fp);
                firstnew += 1;
                firstnew;
            }
            fputs(
                (b"\n>>>>>>>\n\0" as *const u8 as *const libc::c_char)
                    .offset((*outstate).after_newline as libc::c_int as isize),
                fp,
            );
            (*outstate).after_newline = 1 as libc::c_int != 0;
            (*outstate).zero_output = 0 as libc::c_int != 0;
            if ferror(fp) != 0 {
                write_fatal();
            }
            if common_suffix != 0 {
                where_0 += common_suffix;
                if !copy_till(outstate, where_0 - 1 as libc::c_int as libc::c_long) {
                    return 0 as libc::c_int != 0;
                }
                in_0 += common_suffix;
                new += common_suffix;
            }
            *somefailed = 1 as libc::c_int != 0;
        }
    }
    merge_result(
        &mut first_result,
        0 as libc::c_int,
        0 as *const libc::c_char,
        0 as libc::c_int as lin,
        0 as libc::c_int as lin,
    );
    if last_frozen_line == where_0 - 1 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"last_frozen_line == where - 1\0" as *const u8 as *const libc::c_char,
            b"merge.c\0" as *const u8 as *const libc::c_char,
            518 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"_Bool merge_hunk(int, struct outstate *, lin, _Bool *)\0"))
                .as_ptr(),
        );
    }
    'c_4597: {
        if last_frozen_line == where_0 - 1 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"last_frozen_line == where - 1\0" as *const u8 as *const libc::c_char,
                b"merge.c\0" as *const u8 as *const libc::c_char,
                518 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"_Bool merge_hunk(int, struct outstate *, lin, _Bool *)\0"))
                    .as_ptr(),
            );
        }
    };
    free(oldin as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn count_context_lines() -> lin {
    let mut old: lin = 0;
    let mut lastold: lin = pch_ptrn_lines();
    let mut context: lin = 0;
    context = 0 as libc::c_int as lin;
    old = 1 as libc::c_int as lin;
    while old <= lastold {
        if pch_char(old) as libc::c_int == ' ' as i32 {
            context += 1;
            context;
        }
        old += 1;
        old;
    }
    return context;
}
unsafe extern "C" fn context_matches_file(mut old: lin, mut where_0: lin) -> bool {
    let mut size: size_t = 0;
    let mut line: *const libc::c_char = 0 as *const libc::c_char;
    line = ifetch(where_0, 0 as libc::c_int != 0, &mut size);
    return size != 0
        && (if canonicalize_ws as libc::c_int != 0 {
            similar(pfetch(old), pch_line_len(old), line, size) as libc::c_int
        } else {
            (size == pch_line_len(old)
                && memcmp(
                    line as *const libc::c_void,
                    pfetch(old) as *const libc::c_void,
                    size,
                ) == 0 as libc::c_int) as libc::c_int
        }) != 0;
}
unsafe extern "C" fn compute_changes(
    mut xmin: lin,
    mut xmax: lin,
    mut ymin: lin,
    mut ymax: lin,
    mut xchar: *mut libc::c_char,
    mut ychar: *mut libc::c_char,
) {
    let mut ctxt: context = context {
        xchar: 0 as *mut libc::c_char,
        ychar: 0 as *mut libc::c_char,
        fdiag: 0 as *mut lin,
        bdiag: 0 as *mut lin,
        heuristic: false,
        too_expensive: 0,
    };
    let mut diags: lin = 0;
    ctxt.xchar = xchar.offset(-(xmin as isize));
    ctxt.ychar = ychar.offset(-(ymin as isize));
    diags = xmax + ymax + 3 as libc::c_int as libc::c_long;
    ctxt
        .fdiag = xmalloc(
        ((2 as libc::c_int as libc::c_long * diags) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<lin>() as libc::c_ulong),
    ) as *mut lin;
    ctxt.bdiag = (ctxt.fdiag).offset(diags as isize);
    ctxt.fdiag = (ctxt.fdiag).offset((ymax + 1 as libc::c_int as libc::c_long) as isize);
    ctxt.bdiag = (ctxt.bdiag).offset((ymax + 1 as libc::c_int as libc::c_long) as isize);
    ctxt.heuristic = 1 as libc::c_int != 0;
    compareseq(xmin, xmax, ymin, ymax, 0 as libc::c_int != 0, &mut ctxt);
    ctxt
        .fdiag = (ctxt.fdiag)
        .offset(-((ymax + 1 as libc::c_int as libc::c_long) as isize));
    free(ctxt.fdiag as *mut libc::c_void);
}
