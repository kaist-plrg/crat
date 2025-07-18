use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn finite(_: libc::c_double) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn dap_free(ptr: *mut libc::c_void, mesg: *mut libc::c_char);
    fn dap_malloc(nbytes: libc::c_int, mesg: *mut libc::c_char) -> *mut libc::c_char;
    fn infile(fname: *mut libc::c_char, delim: *mut libc::c_char);
    fn step() -> libc::c_int;
    fn inset(fname: *mut libc::c_char);
    fn outset(fname: *mut libc::c_char, varlist: *mut libc::c_char);
    fn output();
    fn dap_vd(varspec: *mut libc::c_char, invar: libc::c_int) -> libc::c_int;
fn dap_dl(varname: *mut libc::c_char, dbl: *mut libc::c_double);
fn dap_sl(varname: *mut libc::c_char, s: *mut libc::c_char);
fn dap_il(varname: *mut libc::c_char, i: *mut libc::c_int);
    fn dap_varnum(vname: *mut libc::c_char) -> libc::c_int;
    fn dap_arrnum(vname: *mut libc::c_char, dim: *mut libc::c_int) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut dap_namelen: libc::c_int;
    static mut dap_maxntxt: libc::c_int;
    static mut dap_maxtxt: libc::c_int;
    fn pict_text(
        p: *mut pict,
        str: *mut libc::c_char,
        x: libc::c_double,
        y: libc::c_double,
        tang: libc::c_double,
        pos: *mut libc::c_char,
    );
    fn pict_point(p: *mut pict, x: libc::c_double, y: libc::c_double);
    fn pict_scale(
        p: *mut pict,
        cx: libc::c_double,
        cy: libc::c_double,
        sx: libc::c_double,
        sy: libc::c_double,
    );
    fn pict_translate(p: *mut pict, tx: libc::c_double, ty: libc::c_double);
    static mut dap_obs: [dataobs; 0];
    static mut dap_err: *mut FILE;
    fn pict_newstr(str: *mut libc::c_char) -> *mut libc::c_char;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tick {
    pub tick_num: libc::c_double,
    pub tick_lab: *mut libc::c_char,
    pub tick_len: libc::c_double,
}
pub unsafe extern "C" fn pict_maketick(
    mut t: *mut tick,
    mut num: libc::c_double,
    mut label: *mut libc::c_char,
    mut len: libc::c_double,
) {
    (*t).tick_num = num;
    (*t).tick_lab = pict_newstr(label);
    (*t).tick_len = len;
}
unsafe extern "C" fn yaxis(
    mut p: *mut pict,
    mut miny: libc::c_double,
    mut maxy: libc::c_double,
    mut ytick: *mut tick,
    mut nyticks: libc::c_int,
    mut xpos: libc::c_double,
    mut ypos: libc::c_double,
    mut side: libc::c_double,
    mut marks: libc::c_int,
) {
    let mut ny: libc::c_int = 0;
    let mut tpos: [libc::c_char; 4] = [0; 4];
    let mut npos: [libc::c_char; 3] = [0; 3];
    let mut xlaboff: libc::c_double = 0.;
    let mut txtang: libc::c_double = 0.;
    let mut lab1len: libc::c_int = 0;
    let mut labslen: libc::c_int = 0;
    strcpy(tpos.as_mut_ptr(), b"cb \0" as *const u8 as *const libc::c_char);
    if side > 0.0f64 {
        strcpy(npos.as_mut_ptr(), b"rm\0" as *const u8 as *const libc::c_char);
    } else {
        strcpy(npos.as_mut_ptr(), b"lm\0" as *const u8 as *const libc::c_char);
    }
    labslen = 0 as libc::c_int;
    ny = 0 as libc::c_int;
    while ny < nyticks {
        lab1len = strlen((*ytick.offset(ny as isize)).tick_lab) as libc::c_int;
        if lab1len > labslen {
            labslen = lab1len;
        }
        ny += 1;
        ny;
    }
    txtang = 0.0f64;
    if miny <= (*ytick.offset(nyticks as isize)).tick_num
        && (*ytick.offset(nyticks as isize)).tick_num <= maxy
    {
        strcpy(tpos.as_mut_ptr(), b"cb \0" as *const u8 as *const libc::c_char);
        xlaboff = 0.25f64 * (*p).pict_fs * (labslen + 6 as libc::c_int) as libc::c_double
            * fabs((*ytick.offset(nyticks as isize)).tick_len);
        txtang = side * 90.0f64;
    } else if (*ytick.offset(nyticks as isize)).tick_num < miny {
        strcpy(tpos.as_mut_ptr(), b"ct \0" as *const u8 as *const libc::c_char);
        xlaboff = 0.0f64;
    } else {
        strcpy(tpos.as_mut_ptr(), b"cb \0" as *const u8 as *const libc::c_char);
        xlaboff = 0.0f64;
    }
    if side > 0.0f64 || marks != 0 {
        pict_text(
            p,
            (*ytick.offset(nyticks as isize)).tick_lab,
            ypos - side * xlaboff,
            (*ytick.offset(nyticks as isize)).tick_num,
            txtang,
            tpos.as_mut_ptr(),
        );
    }
    pict_point(p, ypos, maxy);
    loop {
        nyticks -= 1;
        if !(nyticks >= 0 as libc::c_int) {
            break;
        }
        pict_point(p, ypos, (*ytick.offset(nyticks as isize)).tick_num);
        pict_point(
            p,
            ypos + side * (*ytick.offset(nyticks as isize)).tick_len,
            (*ytick.offset(nyticks as isize)).tick_num,
        );
        pict_point(p, ypos, (*ytick.offset(nyticks as isize)).tick_num);
        if side > 0.0f64 || marks != 0 {
            pict_text(
                p,
                (*ytick.offset(nyticks as isize)).tick_lab,
                ypos
                    - (4.5f64 * side - 0.5f64)
                        * fabs((*ytick.offset(nyticks as isize)).tick_len),
                (*ytick.offset(nyticks as isize)).tick_num,
                0.0f64,
                npos.as_mut_ptr(),
            );
        }
    }
    pict_point(p, ypos, miny);
    pict_point(p, ypos, xpos);
}
unsafe extern "C" fn xaxis(
    mut p: *mut pict,
    mut minx: libc::c_double,
    mut maxx: libc::c_double,
    mut xtick: *mut tick,
    mut nxticks: libc::c_int,
    mut xpos: libc::c_double,
    mut ypos: libc::c_double,
    mut side: libc::c_double,
    mut marks: libc::c_int,
) {
    let mut nx: libc::c_int = 0;
    let mut tpos: [libc::c_char; 4] = [0; 4];
    let mut npos: [libc::c_char; 3] = [0; 3];
    let mut ylaboff: libc::c_double = 0.;
    let mut lab1len: libc::c_int = 0;
    let mut labslen: libc::c_int = 0;
    strcpy(tpos.as_mut_ptr(), b"lt \0" as *const u8 as *const libc::c_char);
    strcpy(npos.as_mut_ptr(), b"ct\0" as *const u8 as *const libc::c_char);
    labslen = 0 as libc::c_int;
    nx = 0 as libc::c_int;
    while nx < nxticks {
        lab1len = strlen((*xtick.offset(nx as isize)).tick_lab) as libc::c_int;
        if lab1len > labslen {
            labslen = lab1len;
        }
        nx += 1;
        nx;
    }
    if minx <= (*xtick.offset(nxticks as isize)).tick_num
        && (*xtick.offset(nxticks as isize)).tick_num <= maxx
    {
        tpos[0 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
        if side < 0.0f64 {
            tpos[1 as libc::c_int as usize] = 'b' as i32 as libc::c_char;
            npos[1 as libc::c_int as usize] = 'b' as i32 as libc::c_char;
        }
        ylaboff = fabs((*p).pict_fs * (*xtick.offset(nxticks as isize)).tick_len);
    } else if (*xtick.offset(nxticks as isize)).tick_num < minx {
        strcpy(tpos.as_mut_ptr(), b"rm \0" as *const u8 as *const libc::c_char);
        ylaboff = 0.0f64;
    } else {
        strcpy(tpos.as_mut_ptr(), b"lm \0" as *const u8 as *const libc::c_char);
        ylaboff = 0.0f64;
    }
    if side > 0.0f64 || marks != 0 {
        pict_text(
            p,
            (*xtick.offset(nxticks as isize)).tick_lab,
            (*xtick.offset(nxticks as isize)).tick_num,
            xpos - side * ylaboff,
            0.0f64,
            tpos.as_mut_ptr(),
        );
    }
    pict_point(p, maxx, xpos);
    loop {
        nxticks -= 1;
        if !(nxticks >= 0 as libc::c_int) {
            break;
        }
        pict_point(p, (*xtick.offset(nxticks as isize)).tick_num, xpos);
        if side > 0.0f64 || marks != 0 {
            pict_point(
                p,
                (*xtick.offset(nxticks as isize)).tick_num,
                xpos + side * (*xtick.offset(nxticks as isize)).tick_len,
            );
        }
        pict_point(p, (*xtick.offset(nxticks as isize)).tick_num, xpos);
        if side > 0.0f64 || marks != 0 {
            pict_text(
                p,
                (*xtick.offset(nxticks as isize)).tick_lab,
                (*xtick.offset(nxticks as isize)).tick_num,
                xpos - 2.0f64 * side * fabs((*xtick.offset(nxticks as isize)).tick_len),
                0.0f64,
                npos.as_mut_ptr(),
            );
        }
    }
    pict_point(p, minx, xpos);
    pict_point(p, ypos, xpos);
}
pub unsafe extern "C" fn pict_axes(
    mut p: *mut pict,
    mut minx: libc::c_double,
    mut maxx: libc::c_double,
    mut xtick: *mut tick,
    mut nxticks: libc::c_int,
    mut miny: libc::c_double,
    mut maxy: libc::c_double,
    mut ytick: *mut tick,
    mut nyticks: libc::c_int,
    mut style: *mut libc::c_char,
    mut bpos: libc::c_double,
    mut lpos: libc::c_double,
    mut tpos: libc::c_double,
    mut rpos: libc::c_double,
) {
    let mut xpos: libc::c_double = 0.;
    let mut ypos: libc::c_double = 0.;
    let mut rmarks: libc::c_int = 0;
    let mut tmarks: libc::c_int = 0;
    xpos = 0.0f64;
    ypos = 0.0f64;
    rmarks = 0 as libc::c_int;
    tmarks = 0 as libc::c_int;
    if *style.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32
        || *style.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
    {
        tmarks = 1 as libc::c_int;
    }
    if *style.offset(1 as libc::c_int as isize) as libc::c_int == '+' as i32
        || *style.offset(1 as libc::c_int as isize) as libc::c_int == '#' as i32
    {
        rmarks = 1 as libc::c_int;
    }
    if *style.offset(0 as libc::c_int as isize) as libc::c_int != '=' as i32
        && *style.offset(1 as libc::c_int as isize) as libc::c_int != '=' as i32
        && *style.offset(0 as libc::c_int as isize) as libc::c_int != '#' as i32
        && *style.offset(1 as libc::c_int as isize) as libc::c_int != '#' as i32
    {
        match *style.offset(0 as libc::c_int as isize) as libc::c_int {
            45 => {
                xpos = bpos;
            }
            43 => {
                xpos = tpos;
            }
            48 | 110 => {
                xpos = 0.0f64;
            }
            _ => {}
        }
        match *style.offset(1 as libc::c_int as isize) as libc::c_int {
            45 => {
                ypos = lpos;
            }
            43 => {
                ypos = rpos;
            }
            48 | 110 => {
                ypos = 0.0f64;
            }
            _ => {}
        }
        match *style.offset(0 as libc::c_int as isize) as libc::c_int {
            45 | 48 => {
                xaxis(p, minx, maxx, xtick, nxticks, xpos, ypos, 1.0f64, tmarks);
            }
            43 => {
                xaxis(p, minx, maxx, xtick, nxticks, xpos, ypos, -1.0f64, tmarks);
            }
            110 | _ => {}
        }
        match *style.offset(1 as libc::c_int as isize) as libc::c_int {
            45 | 48 => {
                yaxis(p, miny, maxy, ytick, nyticks, xpos, ypos, 1.0f64, rmarks);
            }
            43 => {
                yaxis(p, miny, maxy, ytick, nyticks, xpos, ypos, -1.0f64, rmarks);
            }
            110 | _ => {}
        }
    } else if *style.offset(0 as libc::c_int as isize) as libc::c_int != '=' as i32
        && *style.offset(0 as libc::c_int as isize) as libc::c_int != '#' as i32
    {
        let mut current_block_33: u64;
        match *style.offset(0 as libc::c_int as isize) as libc::c_int {
            45 => {
                xpos = bpos;
                current_block_33 = 9417435502711732008;
            }
            48 => {
                current_block_33 = 9417435502711732008;
            }
            43 => {
                yaxis(p, miny, maxy, ytick, nyticks, tpos, lpos, 1.0f64, rmarks);
                xaxis(p, minx, maxx, xtick, nxticks, tpos, lpos, -1.0f64, tmarks);
                xaxis(p, minx, maxx, xtick, nxticks, tpos, rpos, -1.0f64, tmarks);
                yaxis(p, miny, maxy, ytick, nyticks, tpos, rpos, -1.0f64, rmarks);
                current_block_33 = 11743904203796629665;
            }
            110 => {
                fputs(
                    b"(axes) Can't have double y-axes and no x-axis.\n\0" as *const u8
                        as *const libc::c_char,
                    dap_err,
                );
                exit(1 as libc::c_int);
            }
            _ => {
                current_block_33 = 11743904203796629665;
            }
        }
        match current_block_33 {
            9417435502711732008 => {
                yaxis(p, miny, maxy, ytick, nyticks, xpos, lpos, 1.0f64, rmarks);
                xaxis(p, minx, maxx, xtick, nxticks, xpos, lpos, 1.0f64, tmarks);
                xaxis(p, minx, maxx, xtick, nxticks, xpos, rpos, 1.0f64, tmarks);
                yaxis(p, miny, maxy, ytick, nyticks, xpos, rpos, -1.0f64, rmarks);
            }
            _ => {}
        }
    } else if *style.offset(1 as libc::c_int as isize) as libc::c_int != '=' as i32
        && *style.offset(1 as libc::c_int as isize) as libc::c_int != '#' as i32
    {
        let mut current_block_46: u64;
        match *style.offset(1 as libc::c_int as isize) as libc::c_int {
            45 => {
                ypos = lpos;
                current_block_46 = 16848555411549253182;
            }
            48 => {
                current_block_46 = 16848555411549253182;
            }
            43 => {
                xaxis(p, minx, maxx, xtick, nxticks, bpos, rpos, 1.0f64, tmarks);
                yaxis(p, miny, maxy, ytick, nyticks, bpos, rpos, -1.0f64, rmarks);
                yaxis(p, miny, maxy, ytick, nyticks, tpos, rpos, -1.0f64, rmarks);
                xaxis(p, minx, maxx, xtick, nxticks, tpos, rpos, -1.0f64, tmarks);
                current_block_46 = 7990025728955927862;
            }
            110 => {
                fputs(
                    b"(axes) Can't have double x-axes and no y-axis.\n\0" as *const u8
                        as *const libc::c_char,
                    dap_err,
                );
                exit(1 as libc::c_int);
            }
            _ => {
                current_block_46 = 7990025728955927862;
            }
        }
        match current_block_46 {
            16848555411549253182 => {
                xaxis(p, minx, maxx, xtick, nxticks, bpos, ypos, 1.0f64, tmarks);
                yaxis(p, miny, maxy, ytick, nyticks, bpos, ypos, 1.0f64, rmarks);
                yaxis(p, miny, maxy, ytick, nyticks, tpos, ypos, 1.0f64, rmarks);
                xaxis(p, minx, maxx, xtick, nxticks, tpos, ypos, -1.0f64, tmarks);
            }
            _ => {}
        }
    } else {
        yaxis(p, miny, maxy, ytick, nyticks, bpos, lpos, 1.0f64, rmarks);
        xaxis(p, minx, maxx, xtick, nxticks, bpos, rpos, 1.0f64, tmarks);
        yaxis(p, miny, maxy, ytick, nyticks, tpos, rpos, -1.0f64, rmarks);
        xaxis(p, minx, maxx, xtick, nxticks, tpos, lpos, -1.0f64, tmarks);
    };
}
unsafe extern "C" fn makeform(
    mut form: *mut libc::c_char,
    mut max: libc::c_double,
    mut ndigs: libc::c_int,
) {
    let mut ndec: libc::c_int = 0;
    let mut scale: libc::c_double = 0.;
    if ndigs > 9 as libc::c_int {
        ndigs = 9 as libc::c_int;
    }
    max = fabs(max);
    strcpy(form, b"%.0f\0" as *const u8 as *const libc::c_char);
    if max == 0.0f64 {
        sprintf(form, b"%%.%dg\0" as *const u8 as *const libc::c_char, ndigs);
    } else {
        scale = 1.0f64;
        while ndigs > 1 as libc::c_int {
            scale *= 10.0f64;
            ndigs -= 1;
            ndigs;
        }
        ndec = 0 as libc::c_int;
        while max < scale {
            ndec += 1;
            ndec;
            max *= 10.0f64;
        }
        if ndec > 9 as libc::c_int {
            ndec = 9 as libc::c_int;
        }
        let ref mut fresh0 = *form.offset(2 as libc::c_int as isize);
        *fresh0 = (*fresh0 as libc::c_int + ndec) as libc::c_char;
    };
}
unsafe extern "C" fn ticks(
    mut ticks_0: *mut tick,
    mut min: libc::c_double,
    mut max: libc::c_double,
    mut ndigs: libc::c_int,
    mut ticklen: libc::c_double,
    mut nticks: libc::c_int,
    mut labpos: libc::c_double,
    mut alab: *mut libc::c_char,
    mut tfunct: Option::<unsafe extern "C" fn() -> libc::c_double>,
) {
    let mut n: libc::c_int = 0;
    static mut lab: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    let mut coord: libc::c_double = 0.;
    let mut space: libc::c_double = 0.;
    let mut tcoord: libc::c_double = 0.;
    let mut form: [libc::c_char; 5] = [0; 5];
    let mut tmin: libc::c_double = 0.;
    let mut tmax: libc::c_double = 0.;
    if lab.is_null() {
        lab = dap_malloc(
            dap_maxtxt + 1 as libc::c_int,
            b"dap_maxtxt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if nticks > 1 as libc::c_int {
        space = (max - min) / (nticks - 1 as libc::c_int) as libc::c_double;
    } else {
        space = 0 as libc::c_int as libc::c_double;
    }
    if tfunct.is_some() {
        tmin = ::std::mem::transmute::<_, fn(_) -> libc::c_double>(tfunct.unwrap())(min);
    } else {
        tmin = min;
    }
    tmin = fabs(tmin);
    if tfunct.is_some() {
        tmax = ::std::mem::transmute::<_, fn(_) -> libc::c_double>(tfunct.unwrap())(max);
    } else {
        tmax = max;
    }
    tmax = fabs(tmax);
    if tmin > tmax {
        tmax = tmin;
    }
    makeform(form.as_mut_ptr(), tmax, ndigs);
    n = 0 as libc::c_int;
    while n < nticks {
        if nticks > 1 as libc::c_int {
            coord = min + space * n as libc::c_double;
        } else {
            coord = (min + max) / 2.0f64;
        }
        if tfunct.is_some() {
            tcoord = ::std::mem::transmute::<
                _,
                fn(_) -> libc::c_double,
            >((Some(tfunct.unwrap())).unwrap())(coord);
        } else {
            tcoord = coord;
        }
        sprintf(lab, form.as_mut_ptr(), tcoord);
        pict_maketick(ticks_0.offset(n as isize), coord, lab, ticklen);
        n += 1;
        n;
    }
    pict_maketick(ticks_0.offset(n as isize), labpos, alab, ticklen);
}
pub unsafe extern "C" fn pict_autoaxes(
    mut p: *mut pict,
    mut xlab: *mut libc::c_char,
    mut ylab: *mut libc::c_char,
    mut axspec: *mut libc::c_char,
    mut xfunct: Option::<unsafe extern "C" fn() -> libc::c_double>,
    mut yfunct: Option::<unsafe extern "C" fn() -> libc::c_double>,
    mut caption: *mut libc::c_char,
    mut autopos: libc::c_int,
) -> libc::c_double {
    let mut pp: *mut pict = 0 as *mut pict;
    let mut totpts: libc::c_int = 0;
    let mut minx: libc::c_double = 0.;
    let mut maxx: libc::c_double = 0.;
    let mut miny: libc::c_double = 0.;
    let mut maxy: libc::c_double = 0.;
    let mut minxt: libc::c_double = 0.;
    let mut maxxt: libc::c_double = 0.;
    let mut minyt: libc::c_double = 0.;
    let mut maxyt: libc::c_double = 0.;
    let mut nxticks: libc::c_int = 0;
    let mut nyticks: libc::c_int = 0;
    let mut xticks: *mut tick = 0 as *mut tick;
    let mut yticks: *mut tick = 0 as *mut tick;
    let mut xticklen: libc::c_double = 0.;
    let mut yticklen: libc::c_double = 0.;
    let mut as_0: [libc::c_char; 3] = [0; 3];
    let mut lpos: libc::c_double = 0.;
    let mut rpos: libc::c_double = 0.;
    let mut bpos: libc::c_double = 0.;
    let mut tpos: libc::c_double = 0.;
    let mut xlabpos: libc::c_double = 0.;
    let mut ylabpos: libc::c_double = 0.;
    let mut captoff: libc::c_double = 0.;
    let mut width: libc::c_double = 0.;
    let mut height: libc::c_double = 0.;
    let mut specxmax: libc::c_double = 0.;
    let mut specxmin: libc::c_double = 0.;
    let mut specymax: libc::c_double = 0.;
    let mut specymin: libc::c_double = 0.;
    let mut specxticks: libc::c_int = 0;
    let mut specyticks: libc::c_int = 0;
    let mut nxdigs: libc::c_int = 0;
    let mut nydigs: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut word: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut digs: libc::c_double = 0.;
    let mut places: libc::c_double = 0.;
    let mut sign: libc::c_double = 0.;
    let mut nt: libc::c_int = 0;
    let mut xscale: libc::c_double = 0.;
    let mut yscale: libc::c_double = 0.;
    let mut xlablines: libc::c_int = 0;
    minx = 0.0f64;
    maxx = 0.0f64;
    miny = 0.0f64;
    maxy = 0.0f64;
    lpos = 0.0f64;
    rpos = 0.0f64;
    bpos = 0.0f64;
    tpos = 0.0f64;
    xlabpos = 0.0f64;
    ylabpos = 0.0f64;
    xscale = 1.0f64;
    yscale = 1.0f64;
    captoff = 0.0f64;
    xticks = dap_malloc(
        (::std::mem::size_of::<tick>() as libc::c_ulong)
            .wrapping_mul(dap_maxntxt as libc::c_ulong) as libc::c_int,
        b"dap_maxntxt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut tick;
    yticks = dap_malloc(
        (::std::mem::size_of::<tick>() as libc::c_ulong)
            .wrapping_mul(dap_maxntxt as libc::c_ulong) as libc::c_int,
        b"dap_maxntxt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut tick;
    specxmin = 0.0f64 / 0.0f64;
    specxmax = 0.0f64 / 0.0f64;
    specymin = 0.0f64 / 0.0f64;
    specymax = 0.0f64 / 0.0f64;
    specxticks = -(1 as libc::c_int);
    specyticks = -(1 as libc::c_int);
    nxdigs = 3 as libc::c_int;
    nydigs = 3 as libc::c_int;
    word = dap_malloc(
        dap_namelen + 1 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !axspec.is_null() && *axspec.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        a = 0 as libc::c_int;
        while *axspec.offset(a as isize) as libc::c_int == ' ' as i32 {
            a += 1;
            a;
        }
        if *axspec.offset(a as isize) as libc::c_int == '-' as i32
            || *axspec.offset(a as isize) as libc::c_int == '+' as i32
            || *axspec.offset(a as isize) as libc::c_int == '0' as i32
            || *axspec.offset(a as isize) as libc::c_int == 'n' as i32
            || *axspec.offset(a as isize) as libc::c_int == '=' as i32
            || *axspec.offset(a as isize) as libc::c_int == '#' as i32
        {
            let fresh1 = a;
            a = a + 1;
            as_0[0 as libc::c_int as usize] = *axspec.offset(fresh1 as isize);
            if *axspec.offset(a as isize) as libc::c_int == '-' as i32
                || *axspec.offset(a as isize) as libc::c_int == '+' as i32
                || *axspec.offset(a as isize) as libc::c_int == '0' as i32
                || *axspec.offset(a as isize) as libc::c_int == 'n' as i32
                || *axspec.offset(a as isize) as libc::c_int == '=' as i32
                || *axspec.offset(a as isize) as libc::c_int == '#' as i32
            {
                let fresh2 = a;
                a = a + 1;
                as_0[1 as libc::c_int as usize] = *axspec.offset(fresh2 as isize);
            } else {
                as_0[1 as libc::c_int as usize] = '0' as i32 as libc::c_char;
            }
        } else {
            strcpy(as_0.as_mut_ptr(), b"00\0" as *const u8 as *const libc::c_char);
        }
        as_0[2 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        while *axspec.offset(a as isize) as libc::c_int == ' ' as i32 {
            a += 1;
            a;
        }
        while *axspec.offset(a as isize) != 0 {
            w = 0 as libc::c_int;
            while *axspec.offset(a as isize) as libc::c_int != 0
                && *axspec.offset(a as isize) as libc::c_int != ' ' as i32
            {
                if w < dap_namelen {
                    let fresh3 = a;
                    a = a + 1;
                    let fresh4 = w;
                    w = w + 1;
                    *word.offset(fresh4 as isize) = *axspec.offset(fresh3 as isize);
                } else {
                    *word.offset(w as isize) = '\0' as i32 as libc::c_char;
                    fprintf(
                        dap_err,
                        b"(pict_autoaxes) word in axspec too long: %s\n\0" as *const u8
                            as *const libc::c_char,
                        word,
                    );
                    exit(1 as libc::c_int);
                }
            }
            *word.offset(w as isize) = '\0' as i32 as libc::c_char;
            if strncmp(
                word,
                b"MAXX\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0
                || strncmp(
                    word,
                    b"MINX\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
                || strncmp(
                    word,
                    b"MAXY\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
                || strncmp(
                    word,
                    b"MINY\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
            {
                w = 4 as libc::c_int;
                sign = 1.0f64;
                if *word.offset(w as isize) as libc::c_int == '-' as i32 {
                    sign = -1.0f64;
                    w += 1;
                    w;
                }
                digs = 0.0f64;
                places = 0.0f64;
                while '0' as i32 <= *word.offset(w as isize) as libc::c_int
                    && *word.offset(w as isize) as libc::c_int <= '9' as i32
                    || *word.offset(w as isize) as libc::c_int == '.' as i32
                {
                    if *word.offset(w as isize) as libc::c_int == '.' as i32 {
                        if places > 0.0f64 {
                            fprintf(
                                dap_err,
                                b"(pict_autoaxes) bad number for MIN or MAX: %s\n\0"
                                    as *const u8 as *const libc::c_char,
                                word.offset(4 as libc::c_int as isize),
                            );
                            exit(1 as libc::c_int);
                        }
                        places = 1.0f64;
                    } else {
                        if places > 0.0f64 {
                            places *= 10.0f64;
                        }
                        digs = 10.0f64 * digs
                            + (*word.offset(w as isize) as libc::c_int - '0' as i32)
                                as libc::c_double;
                    }
                    w += 1;
                    w;
                }
                digs *= sign;
                if places > 0.0f64 {
                    digs /= places;
                }
                if *word.offset(w as isize) as libc::c_int != 0
                    && *word.offset(w as isize) as libc::c_int != ' ' as i32
                {
                    fprintf(
                        dap_err,
                        b"(pict_autoaxes) bad number for MIN or MAX: %s\n\0" as *const u8
                            as *const libc::c_char,
                        word.offset(3 as libc::c_int as isize),
                    );
                    exit(1 as libc::c_int);
                }
                if *word.offset(1 as libc::c_int as isize) as libc::c_int == 'A' as i32 {
                    if *word.offset(3 as libc::c_int as isize) as libc::c_int
                        == 'X' as i32
                    {
                        specxmax = digs;
                    } else {
                        specymax = digs;
                    }
                } else if *word.offset(3 as libc::c_int as isize) as libc::c_int
                    == 'X' as i32
                {
                    specxmin = digs;
                } else {
                    specymin = digs;
                }
            } else if strncmp(
                word,
                b"NXTICKS\0" as *const u8 as *const libc::c_char,
                7 as libc::c_int as libc::c_ulong,
            ) == 0
                || strncmp(
                    word,
                    b"NYTICKS\0" as *const u8 as *const libc::c_char,
                    7 as libc::c_int as libc::c_ulong,
                ) == 0
            {
                nt = 0 as libc::c_int;
                w = 7 as libc::c_int;
                while '0' as i32 <= *word.offset(w as isize) as libc::c_int
                    && *word.offset(w as isize) as libc::c_int <= '9' as i32
                {
                    nt = 10 as libc::c_int * nt + *word.offset(w as isize) as libc::c_int
                        - '0' as i32;
                    w += 1;
                    w;
                }
                if *word.offset(w as isize) as libc::c_int != 0
                    && *word.offset(w as isize) as libc::c_int != ' ' as i32
                {
                    fprintf(
                        dap_err,
                        b"(pict_autoaxes) bad number of ticks: %s\n\0" as *const u8
                            as *const libc::c_char,
                        word.offset(7 as libc::c_int as isize),
                    );
                    exit(1 as libc::c_int);
                }
                if *word.offset(1 as libc::c_int as isize) as libc::c_int == 'X' as i32 {
                    specxticks = nt;
                } else {
                    specyticks = nt;
                }
            } else if strncmp(
                word,
                b"NXDIGITS\0" as *const u8 as *const libc::c_char,
                8 as libc::c_int as libc::c_ulong,
            ) == 0
                || strncmp(
                    word,
                    b"NYDIGITS\0" as *const u8 as *const libc::c_char,
                    8 as libc::c_int as libc::c_ulong,
                ) == 0
            {
                nt = 0 as libc::c_int;
                w = 8 as libc::c_int;
                while '0' as i32 <= *word.offset(w as isize) as libc::c_int
                    && *word.offset(w as isize) as libc::c_int <= '9' as i32
                {
                    nt = 10 as libc::c_int * nt + *word.offset(w as isize) as libc::c_int
                        - '0' as i32;
                    w += 1;
                    w;
                }
                if *word.offset(w as isize) as libc::c_int != 0
                    && *word.offset(w as isize) as libc::c_int != ' ' as i32
                {
                    fprintf(
                        dap_err,
                        b"(pict_autoaxes) bad number of digits: %s\n\0" as *const u8
                            as *const libc::c_char,
                        word.offset(8 as libc::c_int as isize),
                    );
                    exit(1 as libc::c_int);
                }
                if *word.offset(1 as libc::c_int as isize) as libc::c_int == 'X' as i32 {
                    nxdigs = nt;
                } else {
                    nydigs = nt;
                }
            } else {
                fprintf(
                    dap_err,
                    b"(pict_autoaxes) bad axes specification: %s\n\0" as *const u8
                        as *const libc::c_char,
                    word,
                );
                exit(1 as libc::c_int);
            }
            while *axspec.offset(a as isize) as libc::c_int == ' ' as i32 {
                a += 1;
                a;
            }
        }
    } else {
        strcpy(as_0.as_mut_ptr(), b"00\0" as *const u8 as *const libc::c_char);
    }
    pp = p;
    totpts = 0 as libc::c_int;
    while !pp.is_null() && !((*pp).pict_next).is_null() {
        if (*pp).pict_npts != 0 {
            if pp == p || minx > (*pp).pict_minx {
                minx = (*pp).pict_minx;
            }
            if pp == p || miny > (*pp).pict_miny {
                miny = (*pp).pict_miny;
            }
            if pp == p || maxx < (*pp).pict_maxx {
                maxx = (*pp).pict_maxx;
            }
            if pp == p || maxy < (*pp).pict_maxy {
                maxy = (*pp).pict_maxy;
            }
            totpts += (*pp).pict_npts;
        }
        pp = (*pp).pict_next;
    }
    if totpts == 0 {
        fputs(
            b"(pict_autoaxes) no points.\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    if finite(specxmin) != 0 {
        minx = specxmin;
    } else if minx > 0.0f64 {
        minx = 0.0f64;
    }
    if finite(specxmax) != 0 {
        maxx = specxmax;
    } else if maxx < 0.0f64 {
        maxx = 0.0f64;
    }
    minxt = minx;
    minx -= 0.05f64 * (maxx - minx);
    maxxt = maxx;
    maxx += 0.05f64 * (maxx - minx);
    if finite(specymin) != 0 {
        miny = specymin;
    } else if miny > 0.0f64 {
        miny = 0.0f64;
    }
    if finite(specymax) != 0 {
        maxy = specymax;
    } else if maxy < 0.0f64 {
        maxy = 0.0f64;
    }
    minyt = miny;
    if miny != 0.0f64 {
        miny -= 0.05f64 * (maxy - miny);
    }
    maxyt = maxy;
    maxy += 0.05f64 * (maxy - miny);
    if specxticks >= 0 as libc::c_int {
        nxticks = specxticks;
    } else {
        nxticks = 11 as libc::c_int;
    }
    if nxticks > dap_maxntxt {
        fprintf(
            dap_err,
            b"(pict_autoaxes) Too many x-ticks (%d)\n\0" as *const u8
                as *const libc::c_char,
            nxticks,
        );
        exit(1 as libc::c_int);
    }
    if specyticks >= 0 as libc::c_int {
        nyticks = specyticks;
    } else {
        nyticks = 11 as libc::c_int;
    }
    if nyticks > dap_maxntxt {
        fprintf(
            dap_err,
            b"(pict_autoaxes) Too many y-ticks (%d)\n\0" as *const u8
                as *const libc::c_char,
            nyticks,
        );
        exit(1 as libc::c_int);
    }
    xticklen = -2.0f64;
    yticklen = -2.0f64;
    if autopos != 0 {
        if autopos == 1 as libc::c_int {
            width = 324.0f64;
            height = 324.0f64;
        } else {
            width = 324.0f64;
            height = 324.0f64;
        }
        xscale = width / (maxx - minx);
        yscale = height / (maxy - miny);
    }
    if as_0[0 as libc::c_int as usize] as libc::c_int == '=' as i32
        || as_0[0 as libc::c_int as usize] as libc::c_int == '#' as i32
    {
        bpos = miny;
        tpos = maxy;
        xlabpos = 0.5f64 * (minx + maxx);
        captoff = 4.0f64 * (*p).pict_fs / yscale;
    } else if as_0[0 as libc::c_int as usize] as libc::c_int != 'n' as i32 {
        match as_0[0 as libc::c_int as usize] as libc::c_int {
            45 => {
                bpos = miny;
                xlabpos = 0.5f64 * (minx + maxx);
                captoff = 4.0f64 * (*p).pict_fs / yscale;
            }
            43 => {
                tpos = maxy;
                xlabpos = 0.5f64 * (minx + maxx);
                captoff = 4.0f64 * (*p).pict_fs / yscale;
            }
            48 => {
                bpos = 0.0f64;
                if as_0[1 as libc::c_int as usize] as libc::c_int == '+' as i32 {
                    xlabpos = minx - 0.05f64 * (maxx - minx);
                } else {
                    xlabpos = maxx + 0.05f64 * (maxx - minx);
                }
                captoff = 2.0f64 * (*p).pict_fs / yscale;
            }
            _ => {
                fprintf(
                    dap_err,
                    b"(pict_autoaxes) Bad axis specification: %s\n\0" as *const u8
                        as *const libc::c_char,
                    axspec,
                );
                exit(1 as libc::c_int);
            }
        }
    }
    if as_0[1 as libc::c_int as usize] as libc::c_int == '=' as i32
        || as_0[1 as libc::c_int as usize] as libc::c_int == '#' as i32
    {
        lpos = minx;
        rpos = maxx;
        ylabpos = 0.5f64 * (miny + maxy);
    } else if as_0[1 as libc::c_int as usize] as libc::c_int != 'n' as i32 {
        match as_0[1 as libc::c_int as usize] as libc::c_int {
            45 => {
                lpos = minx;
                ylabpos = 0.5f64 * (miny + maxy);
            }
            43 => {
                rpos = maxx;
                ylabpos = 0.5f64 * (miny + maxy);
            }
            48 => {
                lpos = 0.0f64;
                if as_0[0 as libc::c_int as usize] as libc::c_int == '+' as i32 {
                    ylabpos = miny - 0.05f64 * (maxy - miny);
                } else {
                    ylabpos = maxy + 0.05f64 * (maxy - miny);
                }
            }
            _ => {
                fprintf(
                    dap_err,
                    b"(autoaxes) Bad axis specification: %s\n\0" as *const u8
                        as *const libc::c_char,
                    axspec,
                );
                exit(1 as libc::c_int);
            }
        }
    }
    w = 0 as libc::c_int;
    xlablines = 0 as libc::c_int;
    while *xlab.offset(w as isize) != 0 {
        if *xlab.offset(w as isize) as libc::c_int == '\n' as i32 {
            xlablines += 1;
            xlablines;
        }
        w += 1;
        w;
    }
    captoff += 1.4f64 * xlablines as libc::c_double * (*p).pict_fs / yscale;
    ticks(
        xticks,
        minxt,
        maxxt,
        nxdigs,
        xticklen / yscale,
        nxticks,
        xlabpos,
        xlab,
        xfunct,
    );
    ticks(
        yticks,
        minyt,
        maxyt,
        nydigs,
        yticklen / xscale,
        nyticks,
        ylabpos,
        ylab,
        yfunct,
    );
    pict_axes(
        pp,
        minx,
        maxx,
        xticks,
        nxticks,
        miny,
        maxy,
        yticks,
        nyticks,
        as_0.as_mut_ptr(),
        bpos,
        lpos,
        tpos,
        rpos,
    );
    pict_text(
        pp,
        caption,
        0.5f64 * (minx + maxx),
        miny - captoff,
        0.0f64,
        b"ct \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if autopos != 0 {
        pict_scale(p, 0.5f64 * (minx + maxx), 0.5f64 * (miny + maxy), xscale, yscale);
        pict_translate(
            p,
            144.0f64 + 0.5f64 * (width - (minx + maxx)),
            144.0f64 + 0.5f64 * (height - (miny + maxy)),
        );
    }
    dap_free(
        xticks as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        yticks as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        word as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return sqrt((width * width + height * height) / 100.0f64);
}
pub unsafe extern "C" fn pict_save(
    mut p: *mut pict,
    mut npicts: libc::c_int,
    mut dataset: *mut libc::c_char,
) {
    let mut firstp: *mut pict = 0 as *mut pict;
    let mut pn: libc::c_int = 0;
    let mut strspec: [libc::c_char; 15] = [0; 15];
    let mut len: libc::c_int = 0;
    let mut maxlen: libc::c_int = 0;
    let mut pict_npts: libc::c_int = 0;
    let mut pict_type: [libc::c_char; 5] = [0; 5];
    let mut pict_dash: libc::c_double = 0.;
    let mut pict_minx: libc::c_double = 0.;
    let mut pict_maxx: libc::c_double = 0.;
    let mut pict_miny: libc::c_double = 0.;
    let mut pict_maxy: libc::c_double = 0.;
    let mut pict_ntxt: libc::c_int = 0;
    let mut pict_font: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pict_fs: libc::c_double = 0.;
    let mut pict_lw: libc::c_double = 0.;
    let mut pict_r: libc::c_double = 0.;
    let mut pict_lgray: libc::c_double = 0.;
    let mut pict_fgray: libc::c_double = 0.;
    let mut pict_next: libc::c_int = 0;
    let mut pict_pt: [libc::c_double; 2] = [0.; 2];
    let mut ptn: libc::c_int = 0;
    let mut pict_txt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pict_tpt: [libc::c_double; 2] = [0.; 2];
    let mut pict_tang: libc::c_double = 0.;
    let mut pict_pos: [libc::c_char; 4] = [0; 4];
    let mut outname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pict_patt: libc::c_int = 0;
    firstp = p;
    p = firstp;
    maxlen = 0 as libc::c_int;
    while !p.is_null() {
        len = strlen((*p).pict_font) as libc::c_int;
        if len > maxlen {
            maxlen = len;
        }
        ptn = 0 as libc::c_int;
        while ptn < (*p).pict_ntxt {
            len = strlen(*((*p).pict_txt).offset(ptn as isize)) as libc::c_int;
            if len > maxlen {
                maxlen = len;
            }
            ptn += 1;
            ptn;
        }
        if npicts != 0 {
            p = p.offset(1);
            if p >= firstp.offset(npicts as isize) {
                break;
            }
        } else {
            p = (*p).pict_next;
        }
    }
    if maxlen > 9998 as libc::c_int {
        fprintf(
            dap_err,
            b"(pict_save) maximum string length too long: %d\n\0" as *const u8
                as *const libc::c_char,
            maxlen,
        );
        exit(1 as libc::c_int);
    }
    pict_font = dap_malloc(
        maxlen + 1 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pict_txt = dap_malloc(
        maxlen + 1 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    infile(0 as *mut libc::c_char, 0 as *mut libc::c_char);
    dap_vd(
        b"pict_npts 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    dap_il(b"pict_npts\0" as *const u8 as *const libc::c_char as *mut libc::c_char, &mut pict_npts);
    dap_vd(
        b"pict_type 5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    dap_sl(b"pict_type\0" as *const u8 as *const libc::c_char as *mut libc::c_char, pict_type.as_mut_ptr());
    dap_vd(
        b"pict_dash -1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    dap_dl(b"pict_dash\0" as *const u8 as *const libc::c_char as *mut libc::c_char, &mut pict_dash);
    dap_vd(
        b"pict_minx -1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    dap_dl(b"pict_minx\0" as *const u8 as *const libc::c_char as *mut libc::c_char, &mut pict_minx);
    dap_vd(
        b"pict_maxx -1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    dap_dl(b"pict_maxx\0" as *const u8 as *const libc::c_char as *mut libc::c_char, &mut pict_maxx);
    dap_vd(
        b"pict_miny -1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    dap_dl(b"pict_miny\0" as *const u8 as *const libc::c_char as *mut libc::c_char, &mut pict_miny);
    dap_vd(
        b"pict_maxy -1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    dap_dl(b"pict_maxy\0" as *const u8 as *const libc::c_char as *mut libc::c_char, &mut pict_maxy);
    dap_vd(
        b"pict_ntxt 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    dap_il(b"pict_ntxt\0" as *const u8 as *const libc::c_char as *mut libc::c_char, &mut pict_ntxt);
    sprintf(
        strspec.as_mut_ptr(),
        b"pict_font %d\0" as *const u8 as *const libc::c_char,
        maxlen,
    );
    dap_vd(strspec.as_mut_ptr(), 0 as libc::c_int);
    dap_sl(b"pict_font\0" as *const u8 as *const libc::c_char as *mut libc::c_char, pict_font);
    dap_vd(
        b"pict_fs -1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    dap_dl(b"pict_fs\0" as *const u8 as *const libc::c_char as *mut libc::c_char, &mut pict_fs);
    dap_vd(
        b"pict_lw -1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    dap_dl(b"pict_lw\0" as *const u8 as *const libc::c_char as *mut libc::c_char, &mut pict_lw);
    dap_vd(
        b"pict_r -1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    dap_dl(b"pict_r\0" as *const u8 as *const libc::c_char as *mut libc::c_char, &mut pict_r);
    dap_vd(
        b"pict_lgray -1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    dap_dl(b"pict_lgray\0" as *const u8 as *const libc::c_char as *mut libc::c_char, &mut pict_lgray);
    dap_vd(
        b"pict_fgray -1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    dap_dl(b"pict_fgray\0" as *const u8 as *const libc::c_char as *mut libc::c_char, &mut pict_fgray);
    dap_vd(
        b"pict_next 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    dap_il(b"pict_next\0" as *const u8 as *const libc::c_char as *mut libc::c_char, &mut pict_next);
    dap_vd(
        b"pict_patt 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    dap_il(b"pict_patt\0" as *const u8 as *const libc::c_char as *mut libc::c_char, &mut pict_patt);
    outname = dap_malloc(
        (strlen(dataset)).wrapping_add(9 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pn = 0 as libc::c_int;
    p = firstp;
    while !p.is_null() {
        if pn < 1000 as libc::c_int {
            sprintf(
                outname,
                b"%s.pic%04d\0" as *const u8 as *const libc::c_char,
                dataset,
                pn,
            );
            outset(
                outname,
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            pict_npts = (*p).pict_npts;
            strcpy(pict_type.as_mut_ptr(), ((*p).pict_type).as_mut_ptr());
            pict_dash = (*p).pict_dash;
            pict_minx = (*p).pict_minx;
            pict_maxx = (*p).pict_maxx;
            pict_miny = (*p).pict_miny;
            pict_maxy = (*p).pict_maxy;
            pict_ntxt = (*p).pict_ntxt;
            strcpy(pict_font, (*p).pict_font);
            pict_fs = (*p).pict_fs;
            pict_lw = (*p).pict_lw;
            pict_r = (*p).pict_r;
            pict_lgray = (*p).pict_lgray;
            pict_fgray = (*p).pict_fgray;
            if npicts != 0 {
                if pn < npicts - 1 as libc::c_int {
                    if !((*p).pict_next).is_null() {
                        pict_next = -(((*p).pict_next).offset_from(firstp)
                            as libc::c_long) as libc::c_int;
                    } else {
                        pict_next = -pn;
                    }
                } else {
                    pict_next = 0 as libc::c_int;
                }
            } else if !((*p).pict_next).is_null() {
                pict_next = pn + 1 as libc::c_int;
            } else {
                pict_next = 0 as libc::c_int;
            }
            if !((*p).pict_patt).is_null() {
                pict_patt = 1 as libc::c_int;
            } else {
                pict_patt = 0 as libc::c_int;
            }
            output();
        } else {
            fputs(
                b"(pict_save) too many picts.\n\0" as *const u8 as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        if npicts != 0 {
            p = p.offset(1);
            if p >= firstp.offset(npicts as isize) {
                break;
            }
        } else {
            p = (*p).pict_next;
        }
        pn += 1;
        pn;
    }
    infile(0 as *mut libc::c_char, 0 as *mut libc::c_char);
    dap_vd(
        b"pict_pt[0] -1 pict_pt[1] - 1\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        0 as libc::c_int,
    );
    dap_dl(b"pict_pt\0" as *const u8 as *const libc::c_char as *mut libc::c_char, pict_pt.as_mut_ptr());
    pn = 0 as libc::c_int;
    p = firstp;
    while !p.is_null() {
        sprintf(
            outname,
            b"%s.pts%04d\0" as *const u8 as *const libc::c_char,
            dataset,
            pn,
        );
        outset(outname, b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        ptn = 0 as libc::c_int;
        while ptn < (*p).pict_npts {
            pict_pt[0 as libc::c_int
                as usize] = (*((*p).pict_pt)
                .offset(ptn as isize))[0 as libc::c_int as usize];
            pict_pt[1 as libc::c_int
                as usize] = (*((*p).pict_pt)
                .offset(ptn as isize))[1 as libc::c_int as usize];
            output();
            ptn += 1;
            ptn;
        }
        if npicts != 0 {
            p = p.offset(1);
            if p >= firstp.offset(npicts as isize) {
                break;
            }
        } else {
            p = (*p).pict_next;
        }
        pn += 1;
        pn;
    }
    infile(0 as *mut libc::c_char, 0 as *mut libc::c_char);
    sprintf(
        strspec.as_mut_ptr(),
        b"pict_txt %d\0" as *const u8 as *const libc::c_char,
        maxlen,
    );
    dap_vd(strspec.as_mut_ptr(), 0 as libc::c_int);
    dap_sl(b"pict_txt\0" as *const u8 as *const libc::c_char as *mut libc::c_char, pict_txt);
    dap_vd(
        b"pict_tpt[0] -1 pict_tpt[1] -1\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        0 as libc::c_int,
    );
    dap_dl(b"pict_tpt\0" as *const u8 as *const libc::c_char as *mut libc::c_char, pict_tpt.as_mut_ptr());
    dap_vd(
        b"pict_tang -1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    dap_dl(b"pict_tang\0" as *const u8 as *const libc::c_char as *mut libc::c_char, &mut pict_tang);
    dap_vd(
        b"pict_pos 3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    dap_sl(b"pict_pos\0" as *const u8 as *const libc::c_char as *mut libc::c_char, pict_pos.as_mut_ptr());
    pn = 0 as libc::c_int;
    p = firstp;
    while !p.is_null() {
        sprintf(
            outname,
            b"%s.txt%04d\0" as *const u8 as *const libc::c_char,
            dataset,
            pn,
        );
        outset(outname, b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        ptn = 0 as libc::c_int;
        while ptn < (*p).pict_ntxt {
            strcpy(pict_txt, *((*p).pict_txt).offset(ptn as isize));
            pict_tpt[0 as libc::c_int
                as usize] = *(*((*p).pict_tpt).offset(ptn as isize))
                .offset(0 as libc::c_int as isize);
            pict_tpt[1 as libc::c_int
                as usize] = *(*((*p).pict_tpt).offset(ptn as isize))
                .offset(1 as libc::c_int as isize);
            pict_tang = *((*p).pict_tang).offset(ptn as isize);
            strcpy(pict_pos.as_mut_ptr(), *((*p).pict_pos).offset(ptn as isize));
            output();
            ptn += 1;
            ptn;
        }
        if npicts != 0 {
            p = p.offset(1);
            if p >= firstp.offset(npicts as isize) {
                break;
            }
        } else {
            p = (*p).pict_next;
        }
        pn += 1;
        pn;
    }
    pn = 0 as libc::c_int;
    p = firstp;
    while !p.is_null() {
        if !((*p).pict_patt).is_null() {
            let fresh5 = pn;
            pn = pn + 1;
            sprintf(
                outname,
                b"%s.pat%04d\0" as *const u8 as *const libc::c_char,
                dataset,
                fresh5,
            );
            pict_save((*p).pict_patt, 0 as libc::c_int, outname);
        }
        if npicts != 0 {
            p = p.offset(1);
            if p >= firstp.offset(npicts as isize) {
                break;
            }
        } else {
            p = (*p).pict_next;
        }
        pn += 1;
        pn;
    }
    dap_free(
        outname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        pict_font as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        pict_txt as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn pict_rest(mut dataset: *mut libc::c_char) -> *mut pict {
    let mut npict: libc::c_int = 0;
    let mut p: *mut pict = 0 as *mut pict;
    let mut inname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pn: libc::c_int = 0;
    let mut ptn: libc::c_int = 0;
    let mut pnext: libc::c_int = 0;
    let mut npicts: libc::c_int = 0;
    let mut pict_npts: libc::c_int = 0;
    let mut pict_type: libc::c_int = 0;
    let mut pict_dash: libc::c_int = 0;
    let mut pict_minx: libc::c_int = 0;
    let mut pict_maxx: libc::c_int = 0;
    let mut pict_miny: libc::c_int = 0;
    let mut pict_maxy: libc::c_int = 0;
    let mut pict_ntxt: libc::c_int = 0;
    let mut pict_font: libc::c_int = 0;
    let mut pict_fs: libc::c_int = 0;
    let mut pict_lw: libc::c_int = 0;
    let mut pict_r: libc::c_int = 0;
    let mut pict_lgray: libc::c_int = 0;
    let mut pict_fgray: libc::c_int = 0;
    let mut pict_pt: libc::c_int = 0;
    let mut pict_txt: libc::c_int = 0;
    let mut pict_tpt: libc::c_int = 0;
    let mut pict_tang: libc::c_int = 0;
    let mut pict_pos: libc::c_int = 0;
    let mut pict_next: libc::c_int = 0;
    let mut pict_patt: libc::c_int = 0;
    let mut dim: libc::c_int = 0;
    let mut npts: libc::c_int = 0;
    let mut dblmem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut charmem: *mut libc::c_char = 0 as *mut libc::c_char;
    inname = dap_malloc(
        (strlen(dataset)).wrapping_add(9 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pn = 0 as libc::c_int;
    npicts = 0 as libc::c_int;
    loop {
        sprintf(
            inname,
            b"%s.pic%04d\0" as *const u8 as *const libc::c_char,
            dataset,
            pn,
        );
        inset(inname);
        step();
        pict_next = dap_varnum(
            b"pict_next\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if pict_next < 0 as libc::c_int {
            fprintf(
                dap_err,
                b"(pict_rest) no pict_next in %s\n\0" as *const u8
                    as *const libc::c_char,
                inname,
            );
            exit(1 as libc::c_int);
        }
        pnext = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_int)
            .offset(pict_next as isize);
        if pnext == 0 {
            break;
        }
        if pnext > 0 as libc::c_int {
            pn = pnext;
        } else if pnext < 0 as libc::c_int {
            pn += 1;
            pn;
        }
        npicts += 1;
        npicts;
    }
    npicts += 1;
    npicts;
    p = dap_malloc(
        (::std::mem::size_of::<pict>() as libc::c_ulong)
            .wrapping_mul(npicts as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut pict;
    pn = 0 as libc::c_int;
    while pn < npicts {
        sprintf(
            inname,
            b"%s.pic%04d\0" as *const u8 as *const libc::c_char,
            dataset,
            pn,
        );
        inset(inname);
        step();
        pict_npts = dap_varnum(
            b"pict_npts\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if pict_npts < 0 as libc::c_int {
            fputs(
                b"(pict_rest) missing pict_npts\n\0" as *const u8 as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        (*p.offset(pn as isize))
            .pict_npts = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
            .do_int)
            .offset(pict_npts as isize);
        pict_type = dap_varnum(
            b"pict_type\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if pict_type < 0 as libc::c_int {
            fputs(
                b"(pict_rest) missing pict_type\n\0" as *const u8 as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        strcpy(
            ((*p.offset(pn as isize)).pict_type).as_mut_ptr(),
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(pict_type as isize),
        );
        pict_dash = dap_varnum(
            b"pict_dash\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if pict_dash < 0 as libc::c_int {
            fputs(
                b"(pict_rest) missing pict_dash\n\0" as *const u8 as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        (*p.offset(pn as isize))
            .pict_dash = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
            .do_dbl)
            .offset(pict_dash as isize);
        pict_minx = dap_varnum(
            b"pict_minx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if pict_minx < 0 as libc::c_int {
            fputs(
                b"(pict_rest) missing pict_minx\n\0" as *const u8 as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        (*p.offset(pn as isize))
            .pict_minx = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
            .do_dbl)
            .offset(pict_minx as isize);
        pict_maxx = dap_varnum(
            b"pict_maxx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if pict_maxx < 0 as libc::c_int {
            fputs(
                b"(pict_rest) missing pict_maxx\n\0" as *const u8 as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        (*p.offset(pn as isize))
            .pict_maxx = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
            .do_dbl)
            .offset(pict_maxx as isize);
        pict_miny = dap_varnum(
            b"pict_miny\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if pict_miny < 0 as libc::c_int {
            fputs(
                b"(pict_rest) missing pict_miny\n\0" as *const u8 as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        (*p.offset(pn as isize))
            .pict_miny = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
            .do_dbl)
            .offset(pict_miny as isize);
        pict_maxy = dap_varnum(
            b"pict_maxy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if pict_maxy < 0 as libc::c_int {
            fputs(
                b"(pict_rest) missing pict_maxy\n\0" as *const u8 as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        (*p.offset(pn as isize))
            .pict_maxy = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
            .do_dbl)
            .offset(pict_maxy as isize);
        pict_ntxt = dap_varnum(
            b"pict_ntxt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if pict_ntxt < 0 as libc::c_int {
            fputs(
                b"(pict_rest) missing pict_ntxt\n\0" as *const u8 as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        (*p.offset(pn as isize))
            .pict_ntxt = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
            .do_int)
            .offset(pict_ntxt as isize);
        pict_font = dap_varnum(
            b"pict_font\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if pict_font < 0 as libc::c_int {
            fputs(
                b"(pict_rest) missing pict_font\n\0" as *const u8 as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        let ref mut fresh6 = (*p.offset(pn as isize)).pict_font;
        *fresh6 = dap_malloc(
            (strlen(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(pict_font as isize),
            ))
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        strcpy(
            (*p.offset(pn as isize)).pict_font,
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(pict_font as isize),
        );
        pict_fs = dap_varnum(
            b"pict_fs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if pict_fs < 0 as libc::c_int {
            fputs(
                b"(pict_rest) missing pict_fs\n\0" as *const u8 as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        (*p.offset(pn as isize))
            .pict_fs = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
            .do_dbl)
            .offset(pict_fs as isize);
        pict_lw = dap_varnum(
            b"pict_lw\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if pict_lw < 0 as libc::c_int {
            fputs(
                b"(pict_rest) missing pict_lw\n\0" as *const u8 as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        (*p.offset(pn as isize))
            .pict_lw = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
            .do_dbl)
            .offset(pict_lw as isize);
        pict_r = dap_varnum(
            b"pict_r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if pict_r < 0 as libc::c_int {
            fputs(
                b"(pict_rest) missing pict_r\n\0" as *const u8 as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        (*p.offset(pn as isize))
            .pict_r = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
            .offset(pict_r as isize);
        pict_lgray = dap_varnum(
            b"pict_lgray\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if pict_lgray < 0 as libc::c_int {
            fputs(
                b"(pict_rest) missing pict_lgray\n\0" as *const u8
                    as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        (*p.offset(pn as isize))
            .pict_lgray = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
            .do_dbl)
            .offset(pict_lgray as isize);
        pict_fgray = dap_varnum(
            b"pict_fgray\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if pict_fgray < 0 as libc::c_int {
            fputs(
                b"(pict_rest) missing pict_fgray\n\0" as *const u8
                    as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        (*p.offset(pn as isize))
            .pict_fgray = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
            .do_dbl)
            .offset(pict_fgray as isize);
        pict_next = dap_varnum(
            b"pict_next\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if pict_next < 0 as libc::c_int {
            fputs(
                b"(pict_rest) missing pict_next\n\0" as *const u8 as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        pnext = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_int)
            .offset(pict_next as isize);
        if pnext < 0 as libc::c_int {
            if pnext == -pn {
                let ref mut fresh7 = (*p.offset(pn as isize)).pict_next;
                *fresh7 = 0 as *mut _pict;
            } else {
                let ref mut fresh8 = (*p.offset(pn as isize)).pict_next;
                *fresh8 = p.offset(-(pnext as isize));
            }
        } else if pnext != 0 {
            let ref mut fresh9 = (*p.offset(pn as isize)).pict_next;
            *fresh9 = p.offset(pn as isize).offset(1 as libc::c_int as isize);
        } else {
            let ref mut fresh10 = (*p.offset(pn as isize)).pict_next;
            *fresh10 = 0 as *mut _pict;
        }
        pict_patt = dap_varnum(
            b"pict_patt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if pict_patt < 0 as libc::c_int {
            fputs(
                b"(pict_rest) missing pict_patt\n\0" as *const u8 as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_int)
            .offset(pict_patt as isize) != 0
        {
            sprintf(
                inname,
                b"%s.pat%04d\0" as *const u8 as *const libc::c_char,
                dataset,
                pn,
            );
            let ref mut fresh11 = (*p.offset(pn as isize)).pict_patt;
            *fresh11 = pict_rest(inname);
        } else {
            let ref mut fresh12 = (*p.offset(pn as isize)).pict_patt;
            *fresh12 = 0 as *mut _pict;
        }
        pn += 1;
        pn;
    }
    pn = 0 as libc::c_int;
    while pn < npicts {
        sprintf(
            inname,
            b"%s.pts%04d\0" as *const u8 as *const libc::c_char,
            dataset,
            pn,
        );
        inset(inname);
        pict_pt = dap_arrnum(
            b"pict_pt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut dim,
        );
        if pict_pt < 0 as libc::c_int {
            fputs(
                b"(pict_rest) missing pict_pt\n\0" as *const u8 as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        if dim != 2 as libc::c_int {
            fprintf(
                dap_err,
                b"(pict_rest) bad dimension for pict_pt: %d\n\0" as *const u8
                    as *const libc::c_char,
                dim,
            );
            exit(1 as libc::c_int);
        }
        npts = (*p.offset(pn as isize)).pict_npts;
        ptn = 0 as libc::c_int;
        (*p.offset(pn as isize)).pict_npts = 0 as libc::c_int;
        while ptn < npts {
            step();
            pict_point(
                p.offset(pn as isize),
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(pict_pt as isize),
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset((pict_pt + 1 as libc::c_int) as isize),
            );
            ptn += 1;
            ptn;
        }
        pn += 1;
        pn;
    }
    pn = 0 as libc::c_int;
    while pn < npicts {
        sprintf(
            inname,
            b"%s.txt%04d\0" as *const u8 as *const libc::c_char,
            dataset,
            pn,
        );
        inset(inname);
        npts = (*p.offset(pn as isize)).pict_ntxt;
        let ref mut fresh13 = (*p.offset(pn as isize)).pict_txt;
        *fresh13 = dap_malloc(
            (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul(dap_maxntxt as libc::c_ulong) as libc::c_int,
            b"dap_maxntxt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut *mut libc::c_char;
        dblmem = dap_malloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(dap_maxntxt as libc::c_ulong) as libc::c_int,
            b"dap_maxntxt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_double;
        let ref mut fresh14 = (*p.offset(pn as isize)).pict_tpt;
        *fresh14 = dap_malloc(
            (::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
                .wrapping_mul(dap_maxntxt as libc::c_ulong) as libc::c_int,
            b"dap_maxntxt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut *mut libc::c_double;
        ptn = 0 as libc::c_int;
        while ptn < dap_maxntxt {
            let ref mut fresh15 = *((*p.offset(pn as isize)).pict_tpt)
                .offset(ptn as isize);
            *fresh15 = dblmem.offset((2 as libc::c_int * ptn) as isize);
            ptn += 1;
            ptn;
        }
        let ref mut fresh16 = (*p.offset(pn as isize)).pict_tang;
        *fresh16 = dap_malloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(dap_maxntxt as libc::c_ulong) as libc::c_int,
            b"dap_maxntxt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_double;
        charmem = dap_malloc(
            3 as libc::c_int * dap_maxntxt,
            b"dap_maxntxt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        let ref mut fresh17 = (*p.offset(pn as isize)).pict_pos;
        *fresh17 = dap_malloc(
            (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul(dap_maxntxt as libc::c_ulong) as libc::c_int,
            b"dap_maxntxt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut *mut libc::c_char;
        ptn = 0 as libc::c_int;
        while ptn < dap_maxntxt {
            let ref mut fresh18 = *((*p.offset(pn as isize)).pict_pos)
                .offset(ptn as isize);
            *fresh18 = charmem.offset((3 as libc::c_int * ptn) as isize);
            ptn += 1;
            ptn;
        }
        pict_txt = dap_varnum(
            b"pict_txt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if pict_txt < 0 as libc::c_int {
            fputs(
                b"(pict_rest) missing pict_txt\n\0" as *const u8 as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        pict_tpt = dap_arrnum(
            b"pict_tpt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut dim,
        );
        if pict_tpt < 0 as libc::c_int {
            fputs(
                b"(pict_rest) missing pict_tpt\n\0" as *const u8 as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        if dim != 2 as libc::c_int {
            fprintf(
                dap_err,
                b"(pict_rest) bad dimension for pict_tpt: %d\n\0" as *const u8
                    as *const libc::c_char,
                dim,
            );
            exit(1 as libc::c_int);
        }
        pict_tang = dap_varnum(
            b"pict_tang\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if pict_tang < 0 as libc::c_int {
            fputs(
                b"(pict_rest) missing pict_tang\n\0" as *const u8 as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        pict_pos = dap_varnum(
            b"pict_pos\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if pict_pos < 0 as libc::c_int {
            fputs(
                b"(pict_rest) missing pict_pos\n\0" as *const u8 as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        ptn = 0 as libc::c_int;
        (*p.offset(pn as isize)).pict_ntxt = 0 as libc::c_int;
        while ptn < npts {
            step();
            pict_text(
                p.offset(pn as isize),
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(pict_txt as isize),
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(pict_tpt as isize),
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset((pict_tpt + 1 as libc::c_int) as isize),
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(pict_tang as isize),
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(pict_pos as isize),
            );
            ptn += 1;
            ptn;
        }
        pn += 1;
        pn;
    }
    dap_free(
        inname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return p;
}
