use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    fn read_files(_: *mut file_data, _: bool) -> bool;
    fn file_block_read(_: *mut file_data, _: size_t);
    fn find_change(_: *mut change) -> *mut change;
    fn abort() -> !;
    fn rpl_free(ptr: *mut libc::c_void);
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut output_style: output_style;
    static mut no_diff_means_no_output: bool;
    static mut ignore_blank_lines: bool;
    static mut files_can_be_treated_as_binary: bool;
    static mut file_label: [*mut libc::c_char; 2];
    static mut ignore_regexp: re_pattern_buffer;
    static mut brief: bool;
    static mut speed_large_files: bool;
    static mut minimal: bool;
    static mut files: [file_data; 2];
    fn analyze_hunk(
        _: *mut change,
        _: *mut lin,
        _: *mut lin,
        _: *mut lin,
        _: *mut lin,
    ) -> changes;
    fn message(_: *const libc::c_char, _: ...);
    fn setup_output(_: *const libc::c_char, _: *const libc::c_char, _: bool);
    fn finish_output();
    fn print_sdiff_script(_: *mut change);
    fn print_ifdef_script(_: *mut change);
    fn print_normal_script(_: *mut change);
    fn print_rcs_script(_: *mut change);
    fn pr_forward_ed_script(_: *mut change);
    fn print_ed_script(_: *mut change);
    fn print_context_script(_: *mut change, _: bool);
    fn buffer_lcm(_: size_t, _: size_t, _: size_t) -> size_t;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xzalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type ptrdiff_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type lin = ptrdiff_t;
pub type __re_long_size_t = size_t;
pub type reg_syntax_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut libc::c_char,
    pub translate: *mut libc::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type changes = libc::c_uint;
pub const CHANGED: changes = 3;
pub const NEW: changes = 2;
pub const OLD: changes = 1;
pub const UNCHANGED: changes = 0;
pub type output_style = libc::c_uint;
pub const OUTPUT_SDIFF: output_style = 8;
pub const OUTPUT_IFDEF: output_style = 7;
pub const OUTPUT_RCS: output_style = 6;
pub const OUTPUT_FORWARD_ED: output_style = 5;
pub const OUTPUT_ED: output_style = 4;
pub const OUTPUT_UNIFIED: output_style = 3;
pub const OUTPUT_CONTEXT: output_style = 2;
pub const OUTPUT_NORMAL: output_style = 1;
pub const OUTPUT_UNSPECIFIED: output_style = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct change {
    pub link: *mut change,
    pub inserted: lin,
    pub deleted: lin,
    pub line0: lin,
    pub line1: lin,
    pub ignore: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_data {
    pub desc: libc::c_int,
    pub name: *const libc::c_char,
    pub stat: stat,
    pub buffer: *mut size_t,
    pub bufsize: size_t,
    pub buffered: size_t,
    pub linbuf: *mut *const libc::c_char,
    pub linbuf_base: lin,
    pub buffered_lines: lin,
    pub valid_lines: lin,
    pub alloc_lines: lin,
    pub prefix_end: *const libc::c_char,
    pub prefix_lines: lin,
    pub suffix_begin: *const libc::c_char,
    pub equivs: *mut lin,
    pub undiscarded: *mut lin,
    pub realindexes: *mut lin,
    pub nondiscarded_lines: lin,
    pub changed: *mut libc::c_char,
    pub missing_newline: bool,
    pub eof: bool,
    pub equiv_max: lin,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct comparison {
    pub file: [file_data; 2],
    pub parent: *const comparison,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct context {
    pub xvec: *const lin,
    pub yvec: *const lin,
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
#[inline]
unsafe extern "C" fn robust_output_style(mut s: output_style) -> bool {
    return s as libc::c_uint != OUTPUT_ED as libc::c_int as libc::c_uint
        && s as libc::c_uint != OUTPUT_FORWARD_ED as libc::c_int as libc::c_uint;
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
    let xv: *const lin = (*ctxt).xvec;
    let yv: *const lin = (*ctxt).yvec;
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
            while x < xlim && y < ylim
                && *xv.offset(x as isize) == *yv.offset(y as isize)
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
                ) = 9223372036854775807 as libc::c_long;
        } else {
            bmin += 1;
            bmin;
        }
        if bmax < dmax {
            bmax += 1;
            *bd
                .offset(
                    (bmax + 1 as libc::c_int as libc::c_long) as isize,
                ) = 9223372036854775807 as libc::c_long;
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
                && *xv.offset((x_0 - 1 as libc::c_int as libc::c_long) as isize)
                    == *yv.offset((y_0 - 1 as libc::c_int as libc::c_long) as isize)
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
                            while *xv.offset((x_1 - k as libc::c_long) as isize)
                                == *yv.offset((y_1 - k as libc::c_long) as isize)
                            {
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
                            while *xv.offset((x_2 + k_0 as libc::c_long) as isize)
                                == *yv.offset((y_2 + k_0 as libc::c_long) as isize)
                            {
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
                    let mut x_3: lin = if *fd.offset(d as isize) <= xlim {
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
                bxybest = 9223372036854775807 as libc::c_long;
                d = bmax;
                while d >= bmin {
                    let mut x_4: lin = if xoff >= *bd.offset(d as isize) {
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
    let mut xv: *const lin = (*ctxt).xvec;
    let mut yv: *const lin = (*ctxt).yvec;
    loop {
        while xoff < xlim && yoff < ylim
            && *xv.offset(xoff as isize) == *yv.offset(yoff as isize)
        {
            xoff += 1;
            xoff;
            yoff += 1;
            yoff;
        }
        while xoff < xlim && yoff < ylim
            && *xv.offset((xlim - 1 as libc::c_int as libc::c_long) as isize)
                == *yv.offset((ylim - 1 as libc::c_int as libc::c_long) as isize)
        {
            xlim -= 1;
            xlim;
            ylim -= 1;
            ylim;
        }
        if xoff == xlim {
            while yoff < ylim {
                *(files[1 as libc::c_int as usize].changed)
                    .offset(
                        *(files[1 as libc::c_int as usize].realindexes)
                            .offset(yoff as isize) as isize,
                    ) = 1 as libc::c_int as libc::c_char;
                yoff += 1;
                yoff;
            }
            break;
        } else if yoff == ylim {
            while xoff < xlim {
                *(files[0 as libc::c_int as usize].changed)
                    .offset(
                        *(files[0 as libc::c_int as usize].realindexes)
                            .offset(xoff as isize) as isize,
                    ) = 1 as libc::c_int as libc::c_char;
                xoff += 1;
                xoff;
            }
            break;
        } else {
            let mut part: partition = partition {
                xmid: 0,
                ymid: 0,
                lo_minimal: false,
                hi_minimal: false,
            };
            diag(xoff, xlim, yoff, ylim, find_minimal, &mut part, ctxt);
            let mut xoff1: lin = 0;
            let mut xlim1: lin = 0;
            let mut yoff1: lin = 0;
            let mut ylim1: lin = 0;
            let mut xoff2: lin = 0;
            let mut xlim2: lin = 0;
            let mut yoff2: lin = 0;
            let mut ylim2: lin = 0;
            let mut find_minimal1: bool = false;
            let mut find_minimal2: bool = false;
            if 0 as libc::c_int == 0
                && xlim + ylim - (part.xmid + part.ymid)
                    < part.xmid + part.ymid - (xoff + yoff)
            {
                xoff1 = part.xmid;
                xlim1 = xlim;
                yoff1 = part.ymid;
                ylim1 = ylim;
                find_minimal1 = part.hi_minimal;
                xoff2 = xoff;
                xlim2 = part.xmid;
                yoff2 = yoff;
                ylim2 = part.ymid;
                find_minimal2 = part.lo_minimal;
            } else {
                xoff1 = xoff;
                xlim1 = part.xmid;
                yoff1 = yoff;
                ylim1 = part.ymid;
                find_minimal1 = part.lo_minimal;
                xoff2 = part.xmid;
                xlim2 = xlim;
                yoff2 = part.ymid;
                ylim2 = ylim;
                find_minimal2 = part.hi_minimal;
            }
            let mut early: bool = compareseq(
                xoff1,
                xlim1,
                yoff1,
                ylim1,
                find_minimal1,
                ctxt,
            );
            if early {
                return early;
            }
            xoff = xoff2;
            xlim = xlim2;
            yoff = yoff2;
            ylim = ylim2;
            find_minimal = find_minimal2;
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn discard_confusing_lines(mut filevec: *mut file_data) {
    let mut f: libc::c_int = 0;
    let mut i: lin = 0;
    let mut discarded: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
    let mut equiv_count: [*mut lin; 2] = [0 as *mut lin; 2];
    let mut p: *mut lin = 0 as *mut lin;
    p = xmalloc(
        (((*filevec.offset(0 as libc::c_int as isize)).buffered_lines
            + (*filevec.offset(1 as libc::c_int as isize)).buffered_lines)
            as libc::c_ulong)
            .wrapping_mul(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<lin>() as libc::c_ulong),
            ),
    ) as *mut lin;
    f = 0 as libc::c_int;
    while f < 2 as libc::c_int {
        let ref mut fresh0 = (*filevec.offset(f as isize)).undiscarded;
        *fresh0 = p;
        p = p.offset((*filevec.offset(f as isize)).buffered_lines as isize);
        let ref mut fresh1 = (*filevec.offset(f as isize)).realindexes;
        *fresh1 = p;
        p = p.offset((*filevec.offset(f as isize)).buffered_lines as isize);
        f += 1;
        f;
    }
    p = xcalloc(
        (*filevec.offset(0 as libc::c_int as isize)).equiv_max as size_t,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<lin>() as libc::c_ulong),
    ) as *mut lin;
    equiv_count[0 as libc::c_int as usize] = p;
    equiv_count[1 as libc::c_int
        as usize] = p
        .offset((*filevec.offset(0 as libc::c_int as isize)).equiv_max as isize);
    i = 0 as libc::c_int as lin;
    while i < (*filevec.offset(0 as libc::c_int as isize)).buffered_lines {
        let ref mut fresh2 = *(equiv_count[0 as libc::c_int as usize])
            .offset(
                *((*filevec.offset(0 as libc::c_int as isize)).equivs).offset(i as isize)
                    as isize,
            );
        *fresh2 += 1;
        *fresh2;
        i += 1;
        i;
    }
    i = 0 as libc::c_int as lin;
    while i < (*filevec.offset(1 as libc::c_int as isize)).buffered_lines {
        let ref mut fresh3 = *(equiv_count[1 as libc::c_int as usize])
            .offset(
                *((*filevec.offset(1 as libc::c_int as isize)).equivs).offset(i as isize)
                    as isize,
            );
        *fresh3 += 1;
        *fresh3;
        i += 1;
        i;
    }
    discarded[0 as libc::c_int
        as usize] = xzalloc(
        ((*filevec.offset(0 as libc::c_int as isize)).buffered_lines
            + (*filevec.offset(1 as libc::c_int as isize)).buffered_lines) as size_t,
    ) as *mut libc::c_char;
    discarded[1 as libc::c_int
        as usize] = (discarded[0 as libc::c_int as usize])
        .offset((*filevec.offset(0 as libc::c_int as isize)).buffered_lines as isize);
    f = 0 as libc::c_int;
    while f < 2 as libc::c_int {
        let mut end: size_t = (*filevec.offset(f as isize)).buffered_lines as size_t;
        let mut discards: *mut libc::c_char = discarded[f as usize];
        let mut counts: *mut lin = equiv_count[(1 as libc::c_int - f) as usize];
        let mut equivs: *mut lin = (*filevec.offset(f as isize)).equivs;
        let mut many: size_t = 5 as libc::c_int as size_t;
        let mut tem: size_t = end.wrapping_div(64 as libc::c_int as libc::c_ulong);
        loop {
            tem = tem >> 2 as libc::c_int;
            if !(tem > 0 as libc::c_int as libc::c_ulong) {
                break;
            }
            many = (many as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
        i = 0 as libc::c_int as lin;
        while (i as libc::c_ulong) < end {
            let mut nmatch: lin = 0;
            if !(*equivs.offset(i as isize) == 0 as libc::c_int as libc::c_long) {
                nmatch = *counts.offset(*equivs.offset(i as isize) as isize);
                if nmatch == 0 as libc::c_int as libc::c_long {
                    *discards.offset(i as isize) = 1 as libc::c_int as libc::c_char;
                } else if nmatch as libc::c_ulong > many {
                    *discards.offset(i as isize) = 2 as libc::c_int as libc::c_char;
                }
            }
            i += 1;
            i;
        }
        f += 1;
        f;
    }
    f = 0 as libc::c_int;
    while f < 2 as libc::c_int {
        let mut end_0: lin = (*filevec.offset(f as isize)).buffered_lines;
        let mut discards_0: *mut libc::c_char = discarded[f as usize];
        i = 0 as libc::c_int as lin;
        while i < end_0 {
            if *discards_0.offset(i as isize) as libc::c_int == 2 as libc::c_int {
                *discards_0.offset(i as isize) = 0 as libc::c_int as libc::c_char;
            } else if *discards_0.offset(i as isize) as libc::c_int != 0 as libc::c_int {
                let mut j: lin = 0;
                let mut length: lin = 0;
                let mut provisional: lin = 0 as libc::c_int as lin;
                j = i;
                while j < end_0 {
                    if *discards_0.offset(j as isize) as libc::c_int == 0 as libc::c_int
                    {
                        break;
                    }
                    if *discards_0.offset(j as isize) as libc::c_int == 2 as libc::c_int
                    {
                        provisional += 1;
                        provisional;
                    }
                    j += 1;
                    j;
                }
                while j > i
                    && *discards_0
                        .offset((j - 1 as libc::c_int as libc::c_long) as isize)
                        as libc::c_int == 2 as libc::c_int
                {
                    j -= 1;
                    *discards_0.offset(j as isize) = 0 as libc::c_int as libc::c_char;
                    provisional -= 1;
                    provisional;
                }
                length = j - i;
                if provisional * 4 as libc::c_int as libc::c_long > length {
                    while j > i {
                        j -= 1;
                        if *discards_0.offset(j as isize) as libc::c_int
                            == 2 as libc::c_int
                        {
                            *discards_0
                                .offset(j as isize) = 0 as libc::c_int as libc::c_char;
                        }
                    }
                } else {
                    let mut consec: lin = 0;
                    let mut minimum: lin = 1 as libc::c_int as lin;
                    let mut tem_0: lin = length >> 2 as libc::c_int;
                    loop {
                        tem_0 >>= 2 as libc::c_int;
                        if !((0 as libc::c_int as libc::c_long) < tem_0) {
                            break;
                        }
                        minimum <<= 1 as libc::c_int;
                    }
                    minimum += 1;
                    minimum;
                    j = 0 as libc::c_int as lin;
                    consec = 0 as libc::c_int as lin;
                    while j < length {
                        if *discards_0.offset((i + j) as isize) as libc::c_int
                            != 2 as libc::c_int
                        {
                            consec = 0 as libc::c_int as lin;
                        } else {
                            consec += 1;
                            if minimum == consec {
                                j -= consec;
                            } else if minimum < consec {
                                *discards_0
                                    .offset(
                                        (i + j) as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                            }
                        }
                        j += 1;
                        j;
                    }
                    j = 0 as libc::c_int as lin;
                    consec = 0 as libc::c_int as lin;
                    while j < length {
                        if j >= 8 as libc::c_int as libc::c_long
                            && *discards_0.offset((i + j) as isize) as libc::c_int
                                == 1 as libc::c_int
                        {
                            break;
                        }
                        if *discards_0.offset((i + j) as isize) as libc::c_int
                            == 2 as libc::c_int
                        {
                            consec = 0 as libc::c_int as lin;
                            *discards_0
                                .offset(
                                    (i + j) as isize,
                                ) = 0 as libc::c_int as libc::c_char;
                        } else if *discards_0.offset((i + j) as isize) as libc::c_int
                            == 0 as libc::c_int
                        {
                            consec = 0 as libc::c_int as lin;
                        } else {
                            consec += 1;
                            consec;
                        }
                        if consec == 3 as libc::c_int as libc::c_long {
                            break;
                        }
                        j += 1;
                        j;
                    }
                    i += length - 1 as libc::c_int as libc::c_long;
                    j = 0 as libc::c_int as lin;
                    consec = 0 as libc::c_int as lin;
                    while j < length {
                        if j >= 8 as libc::c_int as libc::c_long
                            && *discards_0.offset((i - j) as isize) as libc::c_int
                                == 1 as libc::c_int
                        {
                            break;
                        }
                        if *discards_0.offset((i - j) as isize) as libc::c_int
                            == 2 as libc::c_int
                        {
                            consec = 0 as libc::c_int as lin;
                            *discards_0
                                .offset(
                                    (i - j) as isize,
                                ) = 0 as libc::c_int as libc::c_char;
                        } else if *discards_0.offset((i - j) as isize) as libc::c_int
                            == 0 as libc::c_int
                        {
                            consec = 0 as libc::c_int as lin;
                        } else {
                            consec += 1;
                            consec;
                        }
                        if consec == 3 as libc::c_int as libc::c_long {
                            break;
                        }
                        j += 1;
                        j;
                    }
                }
            }
            i += 1;
            i;
        }
        f += 1;
        f;
    }
    f = 0 as libc::c_int;
    while f < 2 as libc::c_int {
        let mut discards_1: *mut libc::c_char = discarded[f as usize];
        let mut end_1: lin = (*filevec.offset(f as isize)).buffered_lines;
        let mut j_0: lin = 0 as libc::c_int as lin;
        i = 0 as libc::c_int as lin;
        while i < end_1 {
            if minimal as libc::c_int != 0
                || *discards_1.offset(i as isize) as libc::c_int == 0 as libc::c_int
            {
                *((*filevec.offset(f as isize)).undiscarded)
                    .offset(
                        j_0 as isize,
                    ) = *((*filevec.offset(f as isize)).equivs).offset(i as isize);
                let fresh4 = j_0;
                j_0 = j_0 + 1;
                *((*filevec.offset(f as isize)).realindexes).offset(fresh4 as isize) = i;
            } else {
                *((*filevec.offset(f as isize)).changed)
                    .offset(i as isize) = 1 as libc::c_int as libc::c_char;
            }
            i += 1;
            i;
        }
        (*filevec.offset(f as isize)).nondiscarded_lines = j_0;
        f += 1;
        f;
    }
    rpl_free(discarded[0 as libc::c_int as usize] as *mut libc::c_void);
    rpl_free(equiv_count[0 as libc::c_int as usize] as *mut libc::c_void);
}
unsafe extern "C" fn shift_boundaries(mut filevec: *mut file_data) {
    let mut f: libc::c_int = 0;
    f = 0 as libc::c_int;
    while f < 2 as libc::c_int {
        let mut changed: *mut libc::c_char = (*filevec.offset(f as isize)).changed;
        let mut other_changed: *mut libc::c_char = (*filevec
            .offset((1 as libc::c_int - f) as isize))
            .changed;
        let mut equivs: *const lin = (*filevec.offset(f as isize)).equivs;
        let mut i: lin = 0 as libc::c_int as lin;
        let mut j: lin = 0 as libc::c_int as lin;
        let mut i_end: lin = (*filevec.offset(f as isize)).buffered_lines;
        loop {
            let mut runlength: lin = 0;
            let mut start: lin = 0;
            let mut corresponding: lin = 0;
            while i < i_end && *changed.offset(i as isize) == 0 {
                loop {
                    let fresh5 = j;
                    j = j + 1;
                    if !(*other_changed.offset(fresh5 as isize) != 0) {
                        break;
                    }
                }
                i += 1;
                i;
            }
            if i == i_end {
                break;
            }
            start = i;
            loop {
                i += 1;
                if !(*changed.offset(i as isize) != 0) {
                    break;
                }
            }
            while *other_changed.offset(j as isize) != 0 {
                j += 1;
                j;
            }
            loop {
                runlength = i - start;
                while start != 0
                    && *equivs
                        .offset((start - 1 as libc::c_int as libc::c_long) as isize)
                        == *equivs
                            .offset((i - 1 as libc::c_int as libc::c_long) as isize)
                {
                    start -= 1;
                    *changed.offset(start as isize) = 1 as libc::c_int as libc::c_char;
                    i -= 1;
                    *changed.offset(i as isize) = 0 as libc::c_int as libc::c_char;
                    while *changed
                        .offset((start - 1 as libc::c_int as libc::c_long) as isize) != 0
                    {
                        start -= 1;
                        start;
                    }
                    loop {
                        j -= 1;
                        if !(*other_changed.offset(j as isize) != 0) {
                            break;
                        }
                    }
                }
                corresponding = if *other_changed
                    .offset((j - 1 as libc::c_int as libc::c_long) as isize)
                    as libc::c_int != 0
                {
                    i
                } else {
                    i_end
                };
                while i != i_end
                    && *equivs.offset(start as isize) == *equivs.offset(i as isize)
                {
                    let fresh6 = start;
                    start = start + 1;
                    *changed.offset(fresh6 as isize) = 0 as libc::c_int as libc::c_char;
                    let fresh7 = i;
                    i = i + 1;
                    *changed.offset(fresh7 as isize) = 1 as libc::c_int as libc::c_char;
                    while *changed.offset(i as isize) != 0 {
                        i += 1;
                        i;
                    }
                    loop {
                        j += 1;
                        if !(*other_changed.offset(j as isize) != 0) {
                            break;
                        }
                        corresponding = i;
                    }
                }
                if !(runlength != i - start) {
                    break;
                }
            }
            while corresponding < i {
                start -= 1;
                *changed.offset(start as isize) = 1 as libc::c_int as libc::c_char;
                i -= 1;
                *changed.offset(i as isize) = 0 as libc::c_int as libc::c_char;
                loop {
                    j -= 1;
                    if !(*other_changed.offset(j as isize) != 0) {
                        break;
                    }
                }
            }
        }
        f += 1;
        f;
    }
}
unsafe extern "C" fn add_change(
    mut line0: lin,
    mut line1: lin,
    mut deleted: lin,
    mut inserted: lin,
    mut old: *mut change,
) -> *mut change {
    let mut new: *mut change = xmalloc(::std::mem::size_of::<change>() as libc::c_ulong)
        as *mut change;
    (*new).line0 = line0;
    (*new).line1 = line1;
    (*new).inserted = inserted;
    (*new).deleted = deleted;
    (*new).link = old;
    return new;
}
unsafe extern "C" fn build_reverse_script(mut filevec: *const file_data) -> *mut change {
    let mut script: *mut change = 0 as *mut change;
    let mut changed0: *mut libc::c_char = (*filevec.offset(0 as libc::c_int as isize))
        .changed;
    let mut changed1: *mut libc::c_char = (*filevec.offset(1 as libc::c_int as isize))
        .changed;
    let mut len0: lin = (*filevec.offset(0 as libc::c_int as isize)).buffered_lines;
    let mut len1: lin = (*filevec.offset(1 as libc::c_int as isize)).buffered_lines;
    let mut i0: lin = 0 as libc::c_int as lin;
    let mut i1: lin = 0 as libc::c_int as lin;
    while i0 < len0 || i1 < len1 {
        if *changed0.offset(i0 as isize) as libc::c_int
            | *changed1.offset(i1 as isize) as libc::c_int != 0
        {
            let mut line0: lin = i0;
            let mut line1: lin = i1;
            while *changed0.offset(i0 as isize) != 0 {
                i0 += 1;
                i0;
            }
            while *changed1.offset(i1 as isize) != 0 {
                i1 += 1;
                i1;
            }
            script = add_change(line0, line1, i0 - line0, i1 - line1, script);
        }
        i0 += 1;
        i0;
        i1 += 1;
        i1;
    }
    return script;
}
unsafe extern "C" fn build_script(mut filevec: *const file_data) -> *mut change {
    let mut script: *mut change = 0 as *mut change;
    let mut changed0: *mut libc::c_char = (*filevec.offset(0 as libc::c_int as isize))
        .changed;
    let mut changed1: *mut libc::c_char = (*filevec.offset(1 as libc::c_int as isize))
        .changed;
    let mut i0: lin = (*filevec.offset(0 as libc::c_int as isize)).buffered_lines;
    let mut i1: lin = (*filevec.offset(1 as libc::c_int as isize)).buffered_lines;
    while i0 >= 0 as libc::c_int as libc::c_long
        || i1 >= 0 as libc::c_int as libc::c_long
    {
        if *changed0.offset((i0 - 1 as libc::c_int as libc::c_long) as isize)
            as libc::c_int
            | *changed1.offset((i1 - 1 as libc::c_int as libc::c_long) as isize)
                as libc::c_int != 0
        {
            let mut line0: lin = i0;
            let mut line1: lin = i1;
            while *changed0.offset((i0 - 1 as libc::c_int as libc::c_long) as isize) != 0
            {
                i0 -= 1;
                i0;
            }
            while *changed1.offset((i1 - 1 as libc::c_int as libc::c_long) as isize) != 0
            {
                i1 -= 1;
                i1;
            }
            script = add_change(i0, i1, line0 - i0, line1 - i1, script);
        }
        i0 -= 1;
        i0;
        i1 -= 1;
        i1;
    }
    return script;
}
unsafe extern "C" fn briefly_report(
    mut changes: libc::c_int,
    mut filevec: *const file_data,
) {
    if changes != 0 {
        message(
            if brief as libc::c_int != 0 {
                b"Files %s and %s differ\n\0" as *const u8 as *const libc::c_char
            } else {
                b"Binary files %s and %s differ\n\0" as *const u8 as *const libc::c_char
            },
            if !(file_label[0 as libc::c_int as usize]).is_null() {
                file_label[0 as libc::c_int as usize] as *const libc::c_char
            } else {
                (*filevec.offset(0 as libc::c_int as isize)).name
            },
            if !(file_label[1 as libc::c_int as usize]).is_null() {
                file_label[1 as libc::c_int as usize] as *const libc::c_char
            } else {
                (*filevec.offset(1 as libc::c_int as isize)).name
            },
        );
    }
}
pub unsafe extern "C" fn diff_2_files(mut cmp: *mut comparison) -> libc::c_int {
    let mut f: libc::c_int = 0;
    let mut e: *mut change = 0 as *mut change;
    let mut p: *mut change = 0 as *mut change;
    let mut script: *mut change = 0 as *mut change;
    let mut changes: libc::c_int = 0;
    if read_files(((*cmp).file).as_mut_ptr(), files_can_be_treated_as_binary) {
        if (*cmp).file[0 as libc::c_int as usize].stat.st_size
            != (*cmp).file[1 as libc::c_int as usize].stat.st_size
            && (0 as libc::c_int as libc::c_long)
                < (*cmp).file[0 as libc::c_int as usize].stat.st_size
            && (0 as libc::c_int as libc::c_long)
                < (*cmp).file[1 as libc::c_int as usize].stat.st_size
            && ((*cmp).file[0 as libc::c_int as usize].desc < 0 as libc::c_int
                || (*cmp).file[0 as libc::c_int as usize].stat.st_mode
                    & 0o170000 as libc::c_int as libc::c_uint
                    == 0o100000 as libc::c_int as libc::c_uint)
            && ((*cmp).file[1 as libc::c_int as usize].desc < 0 as libc::c_int
                || (*cmp).file[1 as libc::c_int as usize].stat.st_mode
                    & 0o170000 as libc::c_int as libc::c_uint
                    == 0o100000 as libc::c_int as libc::c_uint)
        {
            changes = 1 as libc::c_int;
        } else if (*cmp).file[0 as libc::c_int as usize].desc
            == (*cmp).file[1 as libc::c_int as usize].desc
        {
            changes = 0 as libc::c_int;
        } else {
            let mut lcm_max: size_t = (9223372036854775807 as libc::c_long
                - 1 as libc::c_int as libc::c_long) as size_t;
            let mut buffer_size: size_t = buffer_lcm(
                ::std::mem::size_of::<size_t>() as libc::c_ulong,
                buffer_lcm(
                    (*cmp).file[0 as libc::c_int as usize].stat.st_blksize as size_t,
                    (*cmp).file[1 as libc::c_int as usize].stat.st_blksize as size_t,
                    lcm_max,
                ),
                lcm_max,
            );
            f = 0 as libc::c_int;
            while f < 2 as libc::c_int {
                (*cmp)
                    .file[f as usize]
                    .buffer = xrealloc(
                    (*cmp).file[f as usize].buffer as *mut libc::c_void,
                    buffer_size,
                ) as *mut size_t;
                f += 1;
                f;
            }
            loop {
                f = 0 as libc::c_int;
                while f < 2 as libc::c_int {
                    if 0 as libc::c_int <= (*cmp).file[f as usize].desc {
                        file_block_read(
                            &mut *((*cmp).file).as_mut_ptr().offset(f as isize),
                            buffer_size.wrapping_sub((*cmp).file[f as usize].buffered),
                        );
                    }
                    f += 1;
                    f;
                }
                if (*cmp).file[0 as libc::c_int as usize].buffered
                    != (*cmp).file[1 as libc::c_int as usize].buffered
                    || memcmp(
                        (*cmp).file[0 as libc::c_int as usize].buffer
                            as *const libc::c_void,
                        (*cmp).file[1 as libc::c_int as usize].buffer
                            as *const libc::c_void,
                        (*cmp).file[0 as libc::c_int as usize].buffered,
                    ) != 0
                {
                    changes = 1 as libc::c_int;
                    break;
                } else if (*cmp).file[0 as libc::c_int as usize].buffered != buffer_size
                {
                    changes = 0 as libc::c_int;
                    break;
                } else {
                    (*cmp)
                        .file[1 as libc::c_int as usize]
                        .buffered = 0 as libc::c_int as size_t;
                    (*cmp)
                        .file[0 as libc::c_int as usize]
                        .buffered = (*cmp).file[1 as libc::c_int as usize].buffered;
                }
            }
        }
        briefly_report(changes, ((*cmp).file).as_mut_ptr() as *const file_data);
    } else {
        let mut ctxt: context = context {
            xvec: 0 as *const lin,
            yvec: 0 as *const lin,
            fdiag: 0 as *mut lin,
            bdiag: 0 as *mut lin,
            heuristic: false,
            too_expensive: 0,
        };
        let mut diags: lin = 0;
        let mut too_expensive: lin = 0;
        let mut s: size_t = ((*cmp).file[0 as libc::c_int as usize].buffered_lines
            + (*cmp).file[1 as libc::c_int as usize].buffered_lines
            + 4 as libc::c_int as libc::c_long) as size_t;
        let mut flag_space: *mut libc::c_char = xzalloc(s) as *mut libc::c_char;
        (*cmp)
            .file[0 as libc::c_int as usize]
            .changed = flag_space.offset(1 as libc::c_int as isize);
        (*cmp)
            .file[1 as libc::c_int as usize]
            .changed = flag_space
            .offset((*cmp).file[0 as libc::c_int as usize].buffered_lines as isize)
            .offset(3 as libc::c_int as isize);
        discard_confusing_lines(((*cmp).file).as_mut_ptr());
        ctxt.xvec = (*cmp).file[0 as libc::c_int as usize].undiscarded;
        ctxt.yvec = (*cmp).file[1 as libc::c_int as usize].undiscarded;
        diags = (*cmp).file[0 as libc::c_int as usize].nondiscarded_lines
            + (*cmp).file[1 as libc::c_int as usize].nondiscarded_lines
            + 3 as libc::c_int as libc::c_long;
        ctxt
            .fdiag = xmalloc(
            (diags as libc::c_ulong)
                .wrapping_mul(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<lin>() as libc::c_ulong),
                ),
        ) as *mut lin;
        ctxt.bdiag = (ctxt.fdiag).offset(diags as isize);
        ctxt
            .fdiag = (ctxt.fdiag)
            .offset(
                ((*cmp).file[1 as libc::c_int as usize].nondiscarded_lines
                    + 1 as libc::c_int as libc::c_long) as isize,
            );
        ctxt
            .bdiag = (ctxt.bdiag)
            .offset(
                ((*cmp).file[1 as libc::c_int as usize].nondiscarded_lines
                    + 1 as libc::c_int as libc::c_long) as isize,
            );
        ctxt.heuristic = speed_large_files;
        too_expensive = 1 as libc::c_int as lin;
        while diags != 0 as libc::c_int as libc::c_long {
            too_expensive <<= 1 as libc::c_int;
            diags >>= 2 as libc::c_int;
        }
        ctxt
            .too_expensive = if 4096 as libc::c_int as libc::c_long >= too_expensive {
            4096 as libc::c_int as libc::c_long
        } else {
            too_expensive
        };
        files[0 as libc::c_int as usize] = (*cmp).file[0 as libc::c_int as usize];
        files[1 as libc::c_int as usize] = (*cmp).file[1 as libc::c_int as usize];
        compareseq(
            0 as libc::c_int as lin,
            (*cmp).file[0 as libc::c_int as usize].nondiscarded_lines,
            0 as libc::c_int as lin,
            (*cmp).file[1 as libc::c_int as usize].nondiscarded_lines,
            minimal,
            &mut ctxt,
        );
        rpl_free(
            (ctxt.fdiag)
                .offset(
                    -(((*cmp).file[1 as libc::c_int as usize].nondiscarded_lines
                        + 1 as libc::c_int as libc::c_long) as isize),
                ) as *mut libc::c_void,
        );
        shift_boundaries(((*cmp).file).as_mut_ptr());
        if output_style as libc::c_uint == OUTPUT_ED as libc::c_int as libc::c_uint {
            script = build_reverse_script(
                ((*cmp).file).as_mut_ptr() as *const file_data,
            );
        } else {
            script = build_script(((*cmp).file).as_mut_ptr() as *const file_data);
        }
        if ignore_blank_lines as libc::c_int != 0 || !(ignore_regexp.fastmap).is_null() {
            let mut next: *mut change = script;
            changes = 0 as libc::c_int;
            while !next.is_null() && changes == 0 as libc::c_int {
                let mut this: *mut change = 0 as *mut change;
                let mut end: *mut change = 0 as *mut change;
                let mut first0: lin = 0;
                let mut last0: lin = 0;
                let mut first1: lin = 0;
                let mut last1: lin = 0;
                this = next;
                end = find_change(next);
                next = (*end).link;
                (*end).link = 0 as *mut change;
                if analyze_hunk(this, &mut first0, &mut last0, &mut first1, &mut last1)
                    as u64 != 0
                {
                    changes = 1 as libc::c_int;
                }
                (*end).link = next;
            }
        } else {
            changes = (script != 0 as *mut change) as libc::c_int;
        }
        if brief {
            briefly_report(changes, ((*cmp).file).as_mut_ptr() as *const file_data);
        } else if changes != 0 || !no_diff_means_no_output {
            setup_output(
                if !(file_label[0 as libc::c_int as usize]).is_null() {
                    file_label[0 as libc::c_int as usize] as *const libc::c_char
                } else {
                    (*cmp).file[0 as libc::c_int as usize].name
                },
                if !(file_label[1 as libc::c_int as usize]).is_null() {
                    file_label[1 as libc::c_int as usize] as *const libc::c_char
                } else {
                    (*cmp).file[1 as libc::c_int as usize].name
                },
                !((*cmp).parent).is_null(),
            );
            match output_style as libc::c_uint {
                2 => {
                    print_context_script(script, 0 as libc::c_int != 0);
                }
                3 => {
                    print_context_script(script, 1 as libc::c_int != 0);
                }
                4 => {
                    print_ed_script(script);
                }
                5 => {
                    pr_forward_ed_script(script);
                }
                6 => {
                    print_rcs_script(script);
                }
                1 => {
                    print_normal_script(script);
                }
                7 => {
                    print_ifdef_script(script);
                }
                8 => {
                    print_sdiff_script(script);
                }
                _ => {
                    abort();
                }
            }
            finish_output();
        }
        rpl_free(
            (*cmp).file[0 as libc::c_int as usize].undiscarded as *mut libc::c_void,
        );
        rpl_free(flag_space as *mut libc::c_void);
        f = 0 as libc::c_int;
        while f < 2 as libc::c_int {
            rpl_free((*cmp).file[f as usize].equivs as *mut libc::c_void);
            rpl_free(
                ((*cmp).file[f as usize].linbuf)
                    .offset((*cmp).file[f as usize].linbuf_base as isize)
                    as *mut libc::c_void,
            );
            f += 1;
            f;
        }
        e = script;
        while !e.is_null() {
            p = (*e).link;
            rpl_free(e as *mut libc::c_void);
            e = p;
        }
        if !robust_output_style(output_style) {
            f = 0 as libc::c_int;
            while f < 2 as libc::c_int {
                if (*cmp).file[f as usize].missing_newline {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        b"%s: %s\n\0" as *const u8 as *const libc::c_char,
                        if !(file_label[f as usize]).is_null() {
                            file_label[f as usize] as *const libc::c_char
                        } else {
                            (*cmp).file[f as usize].name
                        },
                        dcgettext(
                            0 as *const libc::c_char,
                            b"No newline at end of file\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    changes = 2 as libc::c_int;
                }
                f += 1;
                f;
            }
        }
    }
    if (*cmp).file[0 as libc::c_int as usize].buffer
        != (*cmp).file[1 as libc::c_int as usize].buffer
    {
        rpl_free((*cmp).file[0 as libc::c_int as usize].buffer as *mut libc::c_void);
    }
    rpl_free((*cmp).file[1 as libc::c_int as usize].buffer as *mut libc::c_void);
    return changes;
}
