use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn time(__timer: *mut time_t) -> time_t;
    fn ctime(__timer: *const time_t) -> *mut libc::c_char;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn dap_malloc(nbytes: libc::c_int, mesg: *mut libc::c_char) -> *mut libc::c_char;
    fn dap_free(ptr: *mut libc::c_void, mesg: *mut libc::c_char);
    static mut dap_maxpts: libc::c_int;
    static mut dap_maxchar: libc::c_int;
    static mut dap_maxntxt: libc::c_int;
    static mut dap_maxtxt: libc::c_int;
    static mut dap_maxfont: libc::c_int;
    static mut dap_err: *mut FILE;
    static mut dap_psname: *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
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
pub type time_t = __time_t;
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
static mut orient: libc::c_int = 0;
static mut bboxx0: libc::c_int = 0;
static mut bboxy0: libc::c_int = 0;
static mut bboxx1: libc::c_int = 0;
static mut bboxy1: libc::c_int = 0;
static mut pict_out: *mut FILE = 0 as *const FILE as *mut FILE;
static mut pageno: libc::c_int = 0;
static mut ptbuf: *mut *mut libc::c_double = 0 as *const *mut libc::c_double
    as *mut *mut libc::c_double;
static mut ptnext: libc::c_int = 0 as libc::c_int;
static mut charbuf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut charnext: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn dap_initpict() {
    let mut ptmem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p: libc::c_int = 0;
    ptmem = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(dap_maxpts as libc::c_ulong) as libc::c_int,
        b"dap_maxpts\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    ptbuf = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxpts as libc::c_ulong) as libc::c_int,
        b"dap_maxpts\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_double;
    p = 0 as libc::c_int;
    while p < dap_maxpts {
        let ref mut fresh0 = *ptbuf.offset(p as isize);
        *fresh0 = ptmem.offset((p * 2 as libc::c_int) as isize);
        p += 1;
        p;
    }
    charbuf = dap_malloc(
        dap_maxchar + 1 as libc::c_int,
        b"dap_maxchar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn pict_newpoint(
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> *mut [libc::c_double; 2] {
    let mut pt: *mut [libc::c_double; 2] = 0 as *mut [libc::c_double; 2];
    if ptnext < dap_maxpts {
        *(*ptbuf.offset(ptnext as isize)).offset(0 as libc::c_int as isize) = x;
        *(*ptbuf.offset(ptnext as isize)).offset(1 as libc::c_int as isize) = y;
        pt = *ptbuf.offset(ptnext as isize) as *mut [libc::c_double; 2];
        ptnext += 1;
        ptnext;
    } else {
        fputs(
            b"(pict_newpoint) Too many points.\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    return pt;
}
pub unsafe extern "C" fn pict_newstr(mut str: *mut libc::c_char) -> *mut libc::c_char {
    let mut s: libc::c_int = 0;
    let mut s0: *mut libc::c_char = 0 as *mut libc::c_char;
    s0 = charbuf.offset(charnext as isize);
    if !str.is_null() {
        s = 0 as libc::c_int;
        while *str.offset(s as isize) != 0 {
            if charnext < dap_maxchar {
                let fresh1 = charnext;
                charnext = charnext + 1;
                *charbuf.offset(fresh1 as isize) = *str.offset(s as isize);
            } else {
                fputs(
                    b"(pict_newstr) Too many characters.\n\0" as *const u8
                        as *const libc::c_char,
                    dap_err,
                );
                exit(1 as libc::c_int);
            }
            s += 1;
            s;
        }
    }
    let fresh2 = charnext;
    charnext = charnext + 1;
    *charbuf.offset(fresh2 as isize) = '\0' as i32 as libc::c_char;
    return s0;
}
pub unsafe extern "C" fn pict_init(
    mut or: libc::c_int,
    mut bbx0: libc::c_int,
    mut bby0: libc::c_int,
    mut bbx1: libc::c_int,
    mut bby1: libc::c_int,
    mut npages: libc::c_int,
) {
    let mut t: time_t = 0;
    if pict_out.is_null() {
        pict_out = fopen(dap_psname, b"w\0" as *const u8 as *const libc::c_char);
        if pict_out.is_null() {
            fprintf(
                dap_err,
                b"(pict_init) Can't create .ps file: %s\n\0" as *const u8
                    as *const libc::c_char,
                dap_psname,
            );
            exit(1 as libc::c_int);
        }
    }
    orient = or;
    bboxx0 = bbx0;
    bboxy0 = bby0;
    bboxx1 = bbx1;
    bboxy1 = bby1;
    fputs(b"%!PS-Adobe-2.0\n\0" as *const u8 as *const libc::c_char, pict_out);
    fprintf(
        pict_out,
        b"%%Title: %s\n\0" as *const u8 as *const libc::c_char,
        dap_psname,
    );
    fputs(b"%%Creator: ps.c\n\0" as *const u8 as *const libc::c_char, pict_out);
    fputs(b"%%CreationDate: \0" as *const u8 as *const libc::c_char, pict_out);
    time(&mut t);
    fputs(ctime(&mut t), pict_out);
    fputs(
        b"%%For: bassein@localhost.localdomain (,,,,)\n\0" as *const u8
            as *const libc::c_char,
        pict_out,
    );
    fprintf(
        pict_out,
        b"%s %s\n\0" as *const u8 as *const libc::c_char,
        b"%%Orientation:\0" as *const u8 as *const libc::c_char,
        if orient == 'p' as i32 {
            b"Portrait\0" as *const u8 as *const libc::c_char
        } else {
            b"Landscape\0" as *const u8 as *const libc::c_char
        },
    );
    fprintf(
        pict_out,
        b"%s %d %d %d %d\n\0" as *const u8 as *const libc::c_char,
        b"%%BoundingBox:\0" as *const u8 as *const libc::c_char,
        bboxx0,
        bboxy0,
        bboxx1,
        bboxy1,
    );
    fprintf(
        pict_out,
        b"%s %d\n\0" as *const u8 as *const libc::c_char,
        b"%%Pages:\0" as *const u8 as *const libc::c_char,
        npages,
    );
    fputs(b"%%BeginSetup\n\0" as *const u8 as *const libc::c_char, pict_out);
    fputs(
        b"%%IncludeFeature: *PageSize Letter\n\0" as *const u8 as *const libc::c_char,
        pict_out,
    );
    fputs(b"%%EndSetup\n\0" as *const u8 as *const libc::c_char, pict_out);
    fputs(b"%%Magnification: 1.0000\n\0" as *const u8 as *const libc::c_char, pict_out);
    fputs(b"%%EndComments\n\0" as *const u8 as *const libc::c_char, pict_out);
    fputs(
        b"/cp {closepath} bind def /gr {grestore} bind def /gs {gsave} bind def\n\0"
            as *const u8 as *const libc::c_char,
        pict_out,
    );
    fputs(
        b"/sa {save} bind def /rs {restore} bind def /l {lineto} bind def\n\0"
            as *const u8 as *const libc::c_char,
        pict_out,
    );
    fputs(
        b"/rl {rlineto} bind def /ar {arc} bind def\n\0" as *const u8
            as *const libc::c_char,
        pict_out,
    );
    fputs(
        b"/m {moveto} bind def /rm {rmoveto} bind def /n {newpath} bind def\n\0"
            as *const u8 as *const libc::c_char,
        pict_out,
    );
    fputs(
        b"/f {fill} bind def /s {stroke} bind def /sh {show} bind def\n\0" as *const u8
            as *const libc::c_char,
        pict_out,
    );
    fputs(
        b"/slw {setlinewidth} bind def /sg {setgray} bind def /rot {rotate} bind def\n\0"
            as *const u8 as *const libc::c_char,
        pict_out,
    );
    fputs(
        b"/sc {scale} bind def /sd {setdash} bind def /ff {findfont} bind def\n\0"
            as *const u8 as *const libc::c_char,
        pict_out,
    );
    fputs(
        b"/sf {setfont} bind def /scf {scalefont} bind def /sw {stringwidth} bind def\n\0"
            as *const u8 as *const libc::c_char,
        pict_out,
    );
    fputs(b"/tr {translate} bind def\n\0" as *const u8 as *const libc::c_char, pict_out);
    fputs(b"%%EndProlog\n\0" as *const u8 as *const libc::c_char, pict_out);
    pageno = 0 as libc::c_int;
}
pub unsafe extern "C" fn pict_port(mut npages: libc::c_int) {
    pict_init(
        'p' as i32,
        0 as libc::c_int,
        0 as libc::c_int,
        612 as libc::c_int,
        792 as libc::c_int,
        npages,
    );
}
pub unsafe extern "C" fn pict_land(mut npages: libc::c_int) {
    pict_init(
        'l' as i32,
        0 as libc::c_int,
        0 as libc::c_int,
        612 as libc::c_int,
        792 as libc::c_int,
        npages,
    );
}
pub unsafe extern "C" fn pict_end() {
    fputs(b"gr\n\0" as *const u8 as *const libc::c_char, pict_out);
    fputs(b"showpage\n\0" as *const u8 as *const libc::c_char, pict_out);
    fputs(b"%%Trailer\n\0" as *const u8 as *const libc::c_char, pict_out);
    fflush(pict_out);
}
pub unsafe extern "C" fn pict_page() {
    if pageno != 0 {
        fputs(b"gr\n\0" as *const u8 as *const libc::c_char, pict_out);
        fputs(b"showpage\n\0" as *const u8 as *const libc::c_char, pict_out);
    }
    pageno += 1;
    pageno;
    fprintf(
        pict_out,
        b"%%%%Page: %d %d\n\0" as *const u8 as *const libc::c_char,
        pageno,
        pageno,
    );
    fputs(b"gs\n\0" as *const u8 as *const libc::c_char, pict_out);
    if orient == 'l' as i32 {
        fprintf(
            pict_out,
            b"%d %d tr 90 rot %d %d tr\n\0" as *const u8 as *const libc::c_char,
            bboxy0 + bboxx1,
            bboxy0,
            -bboxx0,
            -bboxy0,
        );
    }
}
pub unsafe extern "C" fn pict_clearpict(mut p: *mut pict) {
    dap_free(
        (*p).pict_txt as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*p).pict_txt = 0 as *mut *mut libc::c_char;
    dap_free(
        (*p).pict_font as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*p).pict_font = 0 as *mut libc::c_char;
    dap_free(
        *((*p).pict_tpt).offset(0 as libc::c_int as isize) as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        (*p).pict_tang as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*p).pict_tang = 0 as *mut libc::c_double;
    dap_free(
        *((*p).pict_pos).offset(0 as libc::c_int as isize) as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn pict_initpict(mut prev: *mut pict, mut p: *mut pict) {
    let mut dblmem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut charmem: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: libc::c_int = 0;
    (*p).pict_npts = 0 as libc::c_int;
    (*p).pict_ntxt = 0 as libc::c_int;
    (*p)
        .pict_txt = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(dap_maxntxt as libc::c_ulong) as libc::c_int,
        b"dap_maxntxt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_char;
    strcpy(((*p).pict_type).as_mut_ptr(), b"LINE\0" as *const u8 as *const libc::c_char);
    (*p).pict_dash = 0.0f64;
    (*p)
        .pict_font = dap_malloc(
        dap_maxfont + 1 as libc::c_int,
        b"dap_maxfont\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy((*p).pict_font, b"Helvetica-Bold\0" as *const u8 as *const libc::c_char);
    dblmem = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(dap_maxntxt as libc::c_ulong) as libc::c_int,
        b"dap_maxntxt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    (*p)
        .pict_tpt = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxntxt as libc::c_ulong) as libc::c_int,
        b"dap_maxntxt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_double;
    t = 0 as libc::c_int;
    while t < dap_maxntxt {
        let ref mut fresh3 = *((*p).pict_tpt).offset(t as isize);
        *fresh3 = dblmem.offset((2 as libc::c_int * t) as isize);
        t += 1;
        t;
    }
    (*p)
        .pict_tang = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxntxt as libc::c_ulong) as libc::c_int,
        b"dap_maxntxt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    charmem = dap_malloc(
        3 as libc::c_int * dap_maxntxt,
        b"dap_maxntxt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*p)
        .pict_pos = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(dap_maxntxt as libc::c_ulong) as libc::c_int,
        b"dap_maxntxt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_char;
    t = 0 as libc::c_int;
    while t < dap_maxntxt {
        let ref mut fresh4 = *((*p).pict_pos).offset(t as isize);
        *fresh4 = charmem.offset((3 as libc::c_int * t) as isize);
        t += 1;
        t;
    }
    (*p).pict_fs = 12.0f64;
    (*p).pict_lw = 0.4f64;
    (*p).pict_lgray = 0.0f64;
    (*p).pict_fgray = -1.0f64;
    (*p).pict_patt = 0 as *mut _pict;
    (*p).pict_next = 0 as *mut _pict;
    if !prev.is_null() {
        (*prev).pict_next = p;
    }
}
pub unsafe extern "C" fn pict_text(
    mut p: *mut pict,
    mut str: *mut libc::c_char,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut tang: libc::c_double,
    mut pos: *mut libc::c_char,
) {
    if str.is_null() {
        return;
    }
    if (*p).pict_ntxt < dap_maxntxt - 1 as libc::c_int {
        let ref mut fresh5 = *((*p).pict_txt).offset((*p).pict_ntxt as isize);
        *fresh5 = pict_newstr(str);
        *(*((*p).pict_tpt).offset((*p).pict_ntxt as isize))
            .offset(0 as libc::c_int as isize) = x;
        *(*((*p).pict_tpt).offset((*p).pict_ntxt as isize))
            .offset(1 as libc::c_int as isize) = y;
        if strlen(pos as *const libc::c_char) <= 3 as libc::c_int as libc::c_ulong {
            strcpy(
                *((*p).pict_pos).offset((*p).pict_ntxt as isize),
                pos as *const libc::c_char,
            );
            if *pos.offset(0 as libc::c_int as isize) as libc::c_int != 'l' as i32
                && *pos.offset(0 as libc::c_int as isize) as libc::c_int != 'c' as i32
                && *pos.offset(0 as libc::c_int as isize) as libc::c_int != 'r' as i32
                || *pos.offset(1 as libc::c_int as isize) as libc::c_int != 't' as i32
                    && *pos.offset(1 as libc::c_int as isize) as libc::c_int
                        != 'm' as i32
                    && *pos.offset(1 as libc::c_int as isize) as libc::c_int
                        != 'b' as i32
            {
                fprintf(
                    dap_err,
                    b"(pict_text) Invalid position string: %s\n\0" as *const u8
                        as *const libc::c_char,
                    pos,
                );
                exit(1 as libc::c_int);
            }
        } else {
            fprintf(
                dap_err,
                b"(pict_text) Position string too long: %s\n\0" as *const u8
                    as *const libc::c_char,
                pos,
            );
            exit(1 as libc::c_int);
        }
        *((*p).pict_tang).offset((*p).pict_ntxt as isize) = tang;
        (*p).pict_ntxt += 1;
        (*p).pict_ntxt;
    } else {
        fprintf(
            dap_err,
            b"(pict_text) Too many texts in pict\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    };
}
pub unsafe extern "C" fn pict_circle(
    mut p: *mut pict,
    mut cx: libc::c_double,
    mut cy: libc::c_double,
    mut r: libc::c_double,
) {
    (*p).pict_npts = 1 as libc::c_int;
    strcpy(((*p).pict_type).as_mut_ptr(), b"CIRC\0" as *const u8 as *const libc::c_char);
    (*p).pict_pt = pict_newpoint(cx, cy);
    (*p).pict_r = r;
}
pub unsafe extern "C" fn pict_rectangle(
    mut p: *mut pict,
    mut llx: libc::c_double,
    mut lly: libc::c_double,
    mut sx: libc::c_double,
    mut sy: libc::c_double,
) {
    strcpy(((*p).pict_type).as_mut_ptr(), b"LINE\0" as *const u8 as *const libc::c_char);
    pict_point(p, llx, lly);
    pict_point(p, llx + sx, lly);
    pict_point(p, llx + sx, lly + sy);
    pict_point(p, llx, lly + sy);
    pict_point(p, llx, lly);
}
pub unsafe extern "C" fn pict_hrect(
    mut p: *mut pict,
    mut spacing: libc::c_double,
    mut x0: libc::c_double,
    mut y0: libc::c_double,
    mut xside: libc::c_double,
    mut yside: libc::c_double,
) {
    let mut vlines: libc::c_int = 0;
    let mut hlines: libc::c_int = 0;
    let mut linen: libc::c_int = 0;
    let mut ptn: libc::c_int = 0;
    let mut xl: libc::c_double = 0.;
    let mut xr: libc::c_double = 0.;
    let mut yb: libc::c_double = 0.;
    let mut yt: libc::c_double = 0.;
    let mut even: libc::c_int = 0;
    vlines = floor(yside / spacing) as libc::c_int;
    hlines = floor(xside / spacing) as libc::c_int;
    pict_point(p, x0, y0 + yside);
    pict_point(p, x0, y0);
    pict_point(p, x0 + xside, y0);
    pict_point(p, x0 + xside, y0 + yside);
    pict_point(p, x0, y0 + yside);
    linen = -vlines;
    ptn = 0 as libc::c_int;
    even = 0 as libc::c_int;
    while linen <= hlines {
        xl = x0 + linen as libc::c_double * spacing;
        yb = y0;
        xr = xl + yside;
        yt = y0 + yside;
        if xl < x0 {
            yb += x0 - xl;
            xl = x0;
        }
        if xr > x0 + xside {
            if even == 0 && xr < x0 + xside + spacing {
                pict_point(p, x0 + xside, y0 + yside);
            }
            yt -= xr - x0 - xside;
            xr = x0 + xside;
        }
        if even != 0 {
            pict_point(p, xl, yb);
            pict_point(p, xr, yt);
        } else {
            pict_point(p, xr, yt);
            pict_point(p, xl, yb);
        }
        linen += 1;
        linen;
        ptn += 2 as libc::c_int;
        even = 1 as libc::c_int - even;
    }
}
pub unsafe extern "C" fn pict_bhrect(
    mut p: *mut pict,
    mut spacing: libc::c_double,
    mut x0: libc::c_double,
    mut y0: libc::c_double,
    mut xside: libc::c_double,
    mut yside: libc::c_double,
) {
    let mut vlines: libc::c_int = 0;
    let mut hlines: libc::c_int = 0;
    let mut linen: libc::c_int = 0;
    let mut ptn: libc::c_int = 0;
    let mut xl: libc::c_double = 0.;
    let mut xr: libc::c_double = 0.;
    let mut yb: libc::c_double = 0.;
    let mut yt: libc::c_double = 0.;
    let mut even: libc::c_int = 0;
    vlines = floor(yside / spacing) as libc::c_int;
    hlines = floor(xside / spacing) as libc::c_int;
    pict_point(p, x0, y0);
    pict_point(p, x0 + xside, y0);
    pict_point(p, x0 + xside, y0 + yside);
    pict_point(p, x0, y0 + yside);
    pict_point(p, x0, y0);
    linen = -vlines;
    ptn = 0 as libc::c_int;
    even = 0 as libc::c_int;
    while linen <= hlines {
        xl = x0 + linen as libc::c_double * spacing;
        yt = y0 + yside;
        xr = xl + yside;
        yb = y0;
        if xl < x0 {
            yt -= x0 - xl;
            xl = x0;
        }
        if xr > x0 + xside {
            if even == 0 && xr < x0 + xside + spacing {
                pict_point(p, x0 + xside, y0);
            }
            yb += xr - x0 - xside;
            xr = x0 + xside;
        }
        if even != 0 {
            pict_point(p, xl, yt);
            pict_point(p, xr, yb);
        } else {
            pict_point(p, xr, yb);
            pict_point(p, xl, yt);
        }
        linen += 1;
        linen;
        ptn += 2 as libc::c_int;
        even = 1 as libc::c_int - even;
    }
}
pub unsafe extern "C" fn pict_point(
    mut p: *mut pict,
    mut x: libc::c_double,
    mut y: libc::c_double,
) {
    if (*p).pict_npts == 0 {
        (*p).pict_minx = x;
        (*p).pict_maxx = x;
        (*p).pict_miny = y;
        (*p).pict_maxy = y;
        (*p).pict_pt = pict_newpoint(x, y);
    } else {
        if x < (*p).pict_minx {
            (*p).pict_minx = x;
        }
        if x > (*p).pict_maxx {
            (*p).pict_maxx = x;
        }
        if y < (*p).pict_miny {
            (*p).pict_miny = y;
        }
        if y > (*p).pict_maxy {
            (*p).pict_maxy = y;
        }
        pict_newpoint(x, y);
    }
    (*p).pict_npts += 1;
    (*p).pict_npts;
}
pub unsafe extern "C" fn pict_line(
    mut p: *mut pict,
    mut x0: libc::c_double,
    mut y0: libc::c_double,
    mut x1: libc::c_double,
    mut y1: libc::c_double,
) {
    pict_point(p, x0, y0);
    pict_point(p, x1, y1);
}
pub unsafe extern "C" fn pict_curve(
    mut p: *mut pict,
    mut xf: Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
    mut yf: Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
    mut t0: libc::c_double,
    mut t1: libc::c_double,
    mut nsteps: libc::c_int,
) {
    let mut step: libc::c_int = 0;
    let mut h: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    h = (t1 - t0) / nsteps as libc::c_double;
    step = 0 as libc::c_int;
    while step <= nsteps {
        t = t0 + step as libc::c_double * h;
        if xf.is_some() {
            pict_point(
                p,
                (Some(xf.unwrap())).unwrap()(t),
                (Some(yf.unwrap())).unwrap()(t),
            );
        } else {
            pict_point(p, t, (Some(yf.unwrap())).unwrap()(t));
        }
        step += 1;
        step;
    }
}
pub unsafe extern "C" fn pict_scale(
    mut p: *mut pict,
    mut cx: libc::c_double,
    mut cy: libc::c_double,
    mut sx: libc::c_double,
    mut sy: libc::c_double,
) {
    let mut ptn: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    while !p.is_null() {
        if sx >= sy {
            (*p).pict_r *= sy;
        } else {
            (*p).pict_r *= sx;
        }
        t = 0 as libc::c_int;
        while t < (*p).pict_ntxt {
            *(*((*p).pict_tpt).offset(t as isize))
                .offset(
                    0 as libc::c_int as isize,
                ) = cx
                + sx
                    * (*(*((*p).pict_tpt).offset(t as isize))
                        .offset(0 as libc::c_int as isize) - cx);
            *(*((*p).pict_tpt).offset(t as isize))
                .offset(
                    1 as libc::c_int as isize,
                ) = cy
                + sy
                    * (*(*((*p).pict_tpt).offset(t as isize))
                        .offset(1 as libc::c_int as isize) - cy);
            t += 1;
            t;
        }
        ptn = 0 as libc::c_int;
        while ptn < (*p).pict_npts {
            (*((*p).pict_pt)
                .offset(
                    ptn as isize,
                ))[0 as libc::c_int
                as usize] = cx
                + sx
                    * ((*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize]
                        - cx);
            (*((*p).pict_pt)
                .offset(
                    ptn as isize,
                ))[1 as libc::c_int
                as usize] = cy
                + sy
                    * ((*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize]
                        - cy);
            ptn += 1;
            ptn;
        }
        (*p).pict_minx = cx + sx * ((*p).pict_minx - cx);
        (*p).pict_maxx = cx + sx * ((*p).pict_maxx - cx);
        (*p).pict_miny = cx + sx * ((*p).pict_miny - cx);
        (*p).pict_maxy = cx + sx * ((*p).pict_maxy - cx);
        p = (*p).pict_next;
    }
}
pub unsafe extern "C" fn pict_rotate(
    mut p: *mut pict,
    mut cx: libc::c_double,
    mut cy: libc::c_double,
    mut deg: libc::c_double,
    mut texttoo: libc::c_int,
) {
    let mut c: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    let mut ptn: libc::c_int = 0;
    let mut tmpx: libc::c_double = 0.;
    let mut tmpy: libc::c_double = 0.;
    let mut angle: libc::c_double = 0.;
    let mut t: libc::c_int = 0;
    angle = 3.14159265358979323846f64 / 180.0f64 * deg;
    c = cos(angle);
    s = sin(angle);
    while !p.is_null() {
        t = 0 as libc::c_int;
        while t < (*p).pict_ntxt {
            tmpx = *(*((*p).pict_tpt).offset(t as isize))
                .offset(0 as libc::c_int as isize) - cx;
            tmpy = *(*((*p).pict_tpt).offset(t as isize))
                .offset(1 as libc::c_int as isize) - cy;
            *(*((*p).pict_tpt).offset(t as isize))
                .offset(0 as libc::c_int as isize) = cx + c * tmpx - s * tmpy;
            *(*((*p).pict_tpt).offset(t as isize))
                .offset(1 as libc::c_int as isize) = cy + s * tmpx + c * tmpy;
            *((*p).pict_tang).offset(t as isize) += deg;
            t += 1;
            t;
        }
        ptn = 0 as libc::c_int;
        while ptn < (*p).pict_npts {
            tmpx = (*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize]
                - cx;
            tmpy = (*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize]
                - cy;
            (*((*p).pict_pt)
                .offset(
                    ptn as isize,
                ))[0 as libc::c_int as usize] = cx + c * tmpx - s * tmpy;
            (*((*p).pict_pt)
                .offset(
                    ptn as isize,
                ))[1 as libc::c_int as usize] = cy + s * tmpx + c * tmpy;
            if ptn == 0 {
                (*p)
                    .pict_minx = (*((*p).pict_pt)
                    .offset(ptn as isize))[0 as libc::c_int as usize];
                (*p)
                    .pict_maxx = (*((*p).pict_pt)
                    .offset(ptn as isize))[0 as libc::c_int as usize];
                (*p)
                    .pict_miny = (*((*p).pict_pt)
                    .offset(ptn as isize))[1 as libc::c_int as usize];
                (*p)
                    .pict_maxy = (*((*p).pict_pt)
                    .offset(ptn as isize))[1 as libc::c_int as usize];
            } else {
                if (*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize]
                    < (*p).pict_minx
                {
                    (*p)
                        .pict_minx = (*((*p).pict_pt)
                        .offset(ptn as isize))[0 as libc::c_int as usize];
                }
                if (*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize]
                    > (*p).pict_maxx
                {
                    (*p)
                        .pict_maxx = (*((*p).pict_pt)
                        .offset(ptn as isize))[0 as libc::c_int as usize];
                }
                if (*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize]
                    < (*p).pict_miny
                {
                    (*p)
                        .pict_miny = (*((*p).pict_pt)
                        .offset(ptn as isize))[1 as libc::c_int as usize];
                }
                if (*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize]
                    > (*p).pict_maxy
                {
                    (*p)
                        .pict_maxy = (*((*p).pict_pt)
                        .offset(ptn as isize))[1 as libc::c_int as usize];
                }
            }
            ptn += 1;
            ptn;
        }
        p = (*p).pict_next;
    }
}
pub unsafe extern "C" fn pict_translate(
    mut p: *mut pict,
    mut tx: libc::c_double,
    mut ty: libc::c_double,
) {
    let mut ptn: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    while !p.is_null() {
        t = 0 as libc::c_int;
        while t < (*p).pict_ntxt {
            *(*((*p).pict_tpt).offset(t as isize)).offset(0 as libc::c_int as isize)
                += tx;
            *(*((*p).pict_tpt).offset(t as isize)).offset(1 as libc::c_int as isize)
                += ty;
            t += 1;
            t;
        }
        ptn = 0 as libc::c_int;
        while ptn < (*p).pict_npts {
            (*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize] += tx;
            (*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize] += ty;
            ptn += 1;
            ptn;
        }
        (*p).pict_minx += tx;
        (*p).pict_maxx += tx;
        (*p).pict_miny += ty;
        (*p).pict_maxy += ty;
        p = (*p).pict_next;
    }
}
unsafe extern "C" fn putmode(mut mode: libc::c_int) {
    match mode {
        115 => {
            fputs(b"s\n\0" as *const u8 as *const libc::c_char, pict_out);
        }
        102 => {
            fputs(b"f\n\0" as *const u8 as *const libc::c_char, pict_out);
        }
        112 => {
            fputs(b"clip\n\0" as *const u8 as *const libc::c_char, pict_out);
        }
        _ => {}
    };
}
unsafe extern "C" fn picttype(mut type_0: *mut libc::c_char) -> libc::c_int {
    if strcmp(
        type_0 as *const libc::c_char,
        b"LINE\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return 0 as libc::c_int
    } else if strcmp(
        type_0 as *const libc::c_char,
        b"SEGM\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return 1 as libc::c_int
    } else if strcmp(
        type_0 as *const libc::c_char,
        b"IBEA\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return 2 as libc::c_int
    } else if strcmp(
        type_0 as *const libc::c_char,
        b"CIRC\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return 4 as libc::c_int
    } else if strcmp(
        type_0 as *const libc::c_char,
        b"SQUA\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return 5 as libc::c_int
    } else if strcmp(
        type_0 as *const libc::c_char,
        b"TRIA\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return 6 as libc::c_int
    } else if strcmp(
        type_0 as *const libc::c_char,
        b"UTRI\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return 7 as libc::c_int
    } else if strcmp(
        type_0 as *const libc::c_char,
        b"DIAM\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return 8 as libc::c_int
    } else if strcmp(
        type_0 as *const libc::c_char,
        b"PATT\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return 9 as libc::c_int
    } else {
        fprintf(
            dap_err,
            b"bad pict type: %s\n\0" as *const u8 as *const libc::c_char,
            type_0,
        );
        exit(1 as libc::c_int);
    };
}
unsafe extern "C" fn show0(mut p: *mut pict, mut mode: libc::c_int) {
    let mut ptn: libc::c_int = 0;
    match picttype(((*p).pict_type).as_mut_ptr()) {
        1 => {
            if (*p).pict_npts % 2 as libc::c_int != 0 {
                fputs(
                    b"(show0) Requested SEGM with odd number of points.\n\0" as *const u8
                        as *const libc::c_char,
                    dap_err,
                );
                exit(1 as libc::c_int);
            }
            ptn = 0 as libc::c_int;
            while ptn < (*p).pict_npts {
                fprintf(
                    pict_out,
                    b"n %.6f %.6f m %.6f %.6f l\n\0" as *const u8 as *const libc::c_char,
                    (*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize],
                    (*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize],
                    (*((*p).pict_pt)
                        .offset(
                            (ptn + 1 as libc::c_int) as isize,
                        ))[0 as libc::c_int as usize],
                    (*((*p).pict_pt)
                        .offset(
                            (ptn + 1 as libc::c_int) as isize,
                        ))[1 as libc::c_int as usize],
                );
                putmode(mode);
                ptn += 2 as libc::c_int;
            }
        }
        2 => {
            if (*p).pict_npts % 2 as libc::c_int != 0 {
                fputs(
                    b"(show0) Requested IBEA with odd number of points.\n\0" as *const u8
                        as *const libc::c_char,
                    dap_err,
                );
                exit(1 as libc::c_int);
            }
            ptn = 0 as libc::c_int;
            while ptn < (*p).pict_npts {
                if (*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize]
                    == (*((*p).pict_pt)
                        .offset(
                            (ptn + 1 as libc::c_int) as isize,
                        ))[0 as libc::c_int as usize]
                {
                    fprintf(
                        pict_out,
                        b"n %.6f %.6f m %.6f %.6f l\n\0" as *const u8
                            as *const libc::c_char,
                        (*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize]
                            - (*p).pict_r,
                        (*((*p).pict_pt)
                            .offset(ptn as isize))[1 as libc::c_int as usize],
                        (*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize]
                            + (*p).pict_r,
                        (*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize],
                    );
                    putmode(mode);
                    fprintf(
                        pict_out,
                        b"n %.6f %.6f m %.6f %.6f l\n\0" as *const u8
                            as *const libc::c_char,
                        (*((*p).pict_pt)
                            .offset(ptn as isize))[0 as libc::c_int as usize],
                        (*((*p).pict_pt)
                            .offset(ptn as isize))[1 as libc::c_int as usize],
                        (*((*p).pict_pt)
                            .offset(
                                (ptn + 1 as libc::c_int) as isize,
                            ))[0 as libc::c_int as usize],
                        (*((*p).pict_pt)
                            .offset(
                                (ptn + 1 as libc::c_int) as isize,
                            ))[1 as libc::c_int as usize],
                    );
                    putmode(mode);
                    fprintf(
                        pict_out,
                        b"n %.6f %.6f m %.6f %.6f l\n\0" as *const u8
                            as *const libc::c_char,
                        (*((*p).pict_pt)
                            .offset(
                                (ptn + 1 as libc::c_int) as isize,
                            ))[0 as libc::c_int as usize] - (*p).pict_r,
                        (*((*p).pict_pt)
                            .offset(
                                (ptn + 1 as libc::c_int) as isize,
                            ))[1 as libc::c_int as usize],
                        (*((*p).pict_pt)
                            .offset(
                                (ptn + 1 as libc::c_int) as isize,
                            ))[0 as libc::c_int as usize] + (*p).pict_r,
                        (*((*p).pict_pt)
                            .offset(
                                (ptn + 1 as libc::c_int) as isize,
                            ))[1 as libc::c_int as usize],
                    );
                    putmode(mode);
                } else if (*((*p).pict_pt)
                    .offset(ptn as isize))[1 as libc::c_int as usize]
                    == (*((*p).pict_pt)
                        .offset(
                            (ptn + 1 as libc::c_int) as isize,
                        ))[1 as libc::c_int as usize]
                {
                    fprintf(
                        pict_out,
                        b"n %.6f %.6f m %.6f %.6f l\n\0" as *const u8
                            as *const libc::c_char,
                        (*((*p).pict_pt)
                            .offset(ptn as isize))[0 as libc::c_int as usize],
                        (*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize]
                            - (*p).pict_r,
                        (*((*p).pict_pt)
                            .offset(ptn as isize))[0 as libc::c_int as usize],
                        (*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize]
                            + (*p).pict_r,
                    );
                    putmode(mode);
                    fprintf(
                        pict_out,
                        b"n %.6f %.6f m %.6f %.6f l\n\0" as *const u8
                            as *const libc::c_char,
                        (*((*p).pict_pt)
                            .offset(ptn as isize))[0 as libc::c_int as usize],
                        (*((*p).pict_pt)
                            .offset(ptn as isize))[1 as libc::c_int as usize],
                        (*((*p).pict_pt)
                            .offset(
                                (ptn + 1 as libc::c_int) as isize,
                            ))[0 as libc::c_int as usize],
                        (*((*p).pict_pt)
                            .offset(
                                (ptn + 1 as libc::c_int) as isize,
                            ))[1 as libc::c_int as usize],
                    );
                    putmode(mode);
                    fprintf(
                        pict_out,
                        b"n %.6f %.6f m %.6f %.6f l\n\0" as *const u8
                            as *const libc::c_char,
                        (*((*p).pict_pt)
                            .offset(
                                (ptn + 1 as libc::c_int) as isize,
                            ))[0 as libc::c_int as usize],
                        (*((*p).pict_pt)
                            .offset(
                                (ptn + 1 as libc::c_int) as isize,
                            ))[1 as libc::c_int as usize] - (*p).pict_r,
                        (*((*p).pict_pt)
                            .offset(
                                (ptn + 1 as libc::c_int) as isize,
                            ))[0 as libc::c_int as usize],
                        (*((*p).pict_pt)
                            .offset(
                                (ptn + 1 as libc::c_int) as isize,
                            ))[1 as libc::c_int as usize] + (*p).pict_r,
                    );
                    putmode(mode);
                } else {
                    fprintf(
                        dap_err,
                        b"(show0) IBEA requested but neither x nor y coordinates match: (%g, %g), (%g, %g)\n\0"
                            as *const u8 as *const libc::c_char,
                        (*((*p).pict_pt)
                            .offset(ptn as isize))[0 as libc::c_int as usize],
                        (*((*p).pict_pt)
                            .offset(ptn as isize))[1 as libc::c_int as usize],
                        (*((*p).pict_pt)
                            .offset(
                                (ptn + 1 as libc::c_int) as isize,
                            ))[0 as libc::c_int as usize],
                        (*((*p).pict_pt)
                            .offset(
                                (ptn + 1 as libc::c_int) as isize,
                            ))[1 as libc::c_int as usize],
                    );
                    exit(1 as libc::c_int);
                }
                ptn += 2 as libc::c_int;
            }
        }
        7 => {
            ptn = 0 as libc::c_int;
            while ptn < (*p).pict_npts {
                fprintf(
                    pict_out,
                    b"n %.6f %.6f m %.6f %.6f l %.6f %.6f l %.6f %.6f l\n\0" as *const u8
                        as *const libc::c_char,
                    (*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize]
                        - (*p).pict_r,
                    (*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize]
                        + 0.57735026918962576f64 * (*p).pict_r,
                    (*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize]
                        + (*p).pict_r,
                    (*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize]
                        + 0.57735026918962576f64 * (*p).pict_r,
                    (*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize],
                    (*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize]
                        - 1.15470053837925153f64 * (*p).pict_r,
                    (*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize]
                        - (*p).pict_r,
                    (*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize]
                        + 0.57735026918962576f64 * (*p).pict_r,
                );
                putmode(mode);
                ptn += 1;
                ptn;
            }
        }
        8 => {
            ptn = 0 as libc::c_int;
            while ptn < (*p).pict_npts {
                fprintf(
                    pict_out,
                    b"n %.6f %.6f m %.6f %.6f l %.6f %.6f l %.6f %.6f l %.6f %.6f l\n\0"
                        as *const u8 as *const libc::c_char,
                    (*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize],
                    (*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize]
                        - (*p).pict_r,
                    (*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize]
                        + (*p).pict_r,
                    (*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize],
                    (*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize],
                    (*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize]
                        + (*p).pict_r,
                    (*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize]
                        - (*p).pict_r,
                    (*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize],
                    (*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize],
                    (*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize]
                        - (*p).pict_r,
                );
                putmode(mode);
                ptn += 1;
                ptn;
            }
        }
        0 => {
            fprintf(
                pict_out,
                b"n %.6f %.6f m\n\0" as *const u8 as *const libc::c_char,
                (*((*p).pict_pt)
                    .offset(0 as libc::c_int as isize))[0 as libc::c_int as usize],
                (*((*p).pict_pt)
                    .offset(0 as libc::c_int as isize))[1 as libc::c_int as usize],
            );
            ptn = 1 as libc::c_int;
            while ptn < (*p).pict_npts {
                fprintf(
                    pict_out,
                    b"%.6f %.6f l\n\0" as *const u8 as *const libc::c_char,
                    (*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize],
                    (*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize],
                );
                ptn += 1;
                ptn;
            }
            putmode(mode);
        }
        4 => {
            ptn = 0 as libc::c_int;
            while ptn < (*p).pict_npts {
                fprintf(
                    pict_out,
                    b"n %.6f %.6f %.3f 0 360 ar\n\0" as *const u8 as *const libc::c_char,
                    (*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize],
                    (*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize],
                    (*p).pict_r,
                );
                putmode(mode);
                ptn += 1;
                ptn;
            }
        }
        5 => {
            ptn = 0 as libc::c_int;
            while ptn < (*p).pict_npts {
                fprintf(
                    pict_out,
                    b"n %.6f %.6f m %.6f %.6f l %.6f %.6f l %.6f %.6f l %.6f %.6f l\n\0"
                        as *const u8 as *const libc::c_char,
                    (*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize]
                        - (*p).pict_r,
                    (*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize]
                        - (*p).pict_r,
                    (*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize]
                        + (*p).pict_r,
                    (*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize]
                        - (*p).pict_r,
                    (*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize]
                        + (*p).pict_r,
                    (*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize]
                        + (*p).pict_r,
                    (*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize]
                        - (*p).pict_r,
                    (*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize]
                        + (*p).pict_r,
                    (*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize]
                        - (*p).pict_r,
                    (*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize]
                        - (*p).pict_r,
                );
                putmode(mode);
                ptn += 1;
                ptn;
            }
        }
        6 => {
            ptn = 0 as libc::c_int;
            while ptn < (*p).pict_npts {
                fprintf(
                    pict_out,
                    b"n %.6f %.6f m %.6f %.6f l %.6f %.6f l %.6f %.6f l\n\0" as *const u8
                        as *const libc::c_char,
                    (*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize]
                        - (*p).pict_r,
                    (*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize]
                        - 0.57735026918962576f64 * (*p).pict_r,
                    (*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize]
                        + (*p).pict_r,
                    (*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize]
                        - 0.57735026918962576f64 * (*p).pict_r,
                    (*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize],
                    (*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize]
                        + 1.15470053837925153f64 * (*p).pict_r,
                    (*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize]
                        - (*p).pict_r,
                    (*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize]
                        - 0.57735026918962576f64 * (*p).pict_r,
                );
                putmode(mode);
                ptn += 1;
                ptn;
            }
        }
        9 => {
            ptn = 0 as libc::c_int;
            while ptn < (*p).pict_npts {
                fprintf(
                    pict_out,
                    b"gs n %.6f %.6f tr\n\0" as *const u8 as *const libc::c_char,
                    (*((*p).pict_pt).offset(ptn as isize))[0 as libc::c_int as usize],
                    (*((*p).pict_pt).offset(ptn as isize))[1 as libc::c_int as usize],
                );
                pict_show((*p).pict_patt);
                fputs(b"gr\n\0" as *const u8 as *const libc::c_char, pict_out);
                ptn += 1;
                ptn;
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn show1(mut p: *mut pict) -> *mut pict {
    static mut show1init: libc::c_int = 0 as libc::c_int;
    let mut t: libc::c_int = 0;
    let mut yoff: libc::c_double = 0.;
    let mut tc: libc::c_int = 0;
    let mut nfull: libc::c_double = 0.;
    let mut nscript: libc::c_double = 0.;
    let mut nchange: libc::c_double = 0.;
    let mut inscript: libc::c_int = 0;
    let mut lfact: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    static mut tpart: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut baret: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    let mut hside: libc::c_double = 0.;
    let mut efffs: libc::c_double = 0.;
    let mut nlines: libc::c_int = 0;
    let mut toff: libc::c_int = 0;
    let mut linespace: libc::c_double = 0.;
    linespace = 1.4f64;
    yoff = 0.0f64;
    if show1init == 0 {
        show1init = 1 as libc::c_int;
        tpart = dap_malloc(
            dap_maxtxt + 1 as libc::c_int,
            b"dap_maxtxt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        baret = dap_malloc(
            dap_maxtxt + 1 as libc::c_int,
            b"dap_maxtxt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if (*p).pict_npts > 0 as libc::c_int {
        if (*p).pict_fgray >= 0.0f64 {
            fprintf(
                pict_out,
                b"%.2f sg\n\0" as *const u8 as *const libc::c_char,
                (*p).pict_fgray,
            );
            show0(p, 'f' as i32);
        }
        if (*p).pict_lw >= 0.0f64 {
            fprintf(
                pict_out,
                b"%.2f slw %.2f sg\n\0" as *const u8 as *const libc::c_char,
                (*p).pict_lw,
                (*p).pict_lgray,
            );
            if (*p).pict_dash > 0.0f64 {
                fprintf(
                    pict_out,
                    b"[%.3f] 0 sd\n\0" as *const u8 as *const libc::c_char,
                    (*p).pict_dash,
                );
            }
            show0(p, 's' as i32);
            if (*p).pict_dash > 0.0f64 {
                fputs(b"[] 0 sd\n\0" as *const u8 as *const libc::c_char, pict_out);
            }
        }
        if !((*p).pict_patt).is_null()
            && strcmp(
                ((*p).pict_type).as_mut_ptr(),
                b"PATT\0" as *const u8 as *const libc::c_char,
            ) != 0
        {
            show0(p, 'p' as i32);
        }
    }
    t = 0 as libc::c_int;
    while t < (*p).pict_ntxt {
        nlines = 1 as libc::c_int;
        tc = 0 as libc::c_int;
        while *(*((*p).pict_txt).offset(t as isize)).offset(tc as isize) != 0 {
            if *(*((*p).pict_txt).offset(t as isize)).offset(tc as isize) as libc::c_int
                == '\n' as i32
            {
                nlines += 1;
                nlines;
            }
            tc += 1;
            tc;
        }
        match *(*((*p).pict_pos).offset(t as isize)).offset(1 as libc::c_int as isize)
            as libc::c_int
        {
            116 => {
                yoff = -(*p).pict_fs;
            }
            109 => {
                yoff = -0.2f64 * (*p).pict_fs
                    + 0.5f64 * linespace * (*p).pict_fs
                        * (nlines - 1 as libc::c_int) as libc::c_double;
            }
            98 => {
                yoff = linespace * (*p).pict_fs
                    * (nlines - 1 as libc::c_int) as libc::c_double;
            }
            _ => {}
        }
        toff = 0 as libc::c_int;
        while *(*((*p).pict_txt).offset(t as isize)).offset(toff as isize) != 0 {
            tc = toff;
            i = 0 as libc::c_int;
            inscript = 0 as libc::c_int;
            nfull = 0.0f64;
            nscript = 0.0f64;
            nchange = 0.0f64;
            while *(*((*p).pict_txt).offset(t as isize)).offset(tc as isize)
                as libc::c_int != 0
                && *(*((*p).pict_txt).offset(t as isize)).offset(tc as isize)
                    as libc::c_int != '\n' as i32
            {
                if *(*((*p).pict_txt).offset(t as isize)).offset(tc as isize)
                    as libc::c_int == '^' as i32
                    || *(*((*p).pict_txt).offset(t as isize)).offset(tc as isize)
                        as libc::c_int == '|' as i32
                {
                    inscript = (inscript == 0) as libc::c_int;
                    nchange += 1.0f64;
                } else {
                    if inscript != 0 {
                        nscript += 1.0f64;
                    } else {
                        nfull += 1.0f64;
                    }
                    if i < dap_maxtxt {
                        if *(*((*p).pict_txt).offset(t as isize)).offset(tc as isize)
                            as libc::c_int == '(' as i32
                            || *(*((*p).pict_txt).offset(t as isize)).offset(tc as isize)
                                as libc::c_int == ')' as i32
                        {
                            let fresh6 = i;
                            i = i + 1;
                            *baret.offset(fresh6 as isize) = '\\' as i32 as libc::c_char;
                            if i == dap_maxtxt {
                                fprintf(
                                    dap_err,
                                    b"(show1) Text too long: %s\n\0" as *const u8
                                        as *const libc::c_char,
                                    *((*p).pict_txt).offset(t as isize),
                                );
                                exit(1 as libc::c_int);
                            }
                        }
                        let fresh7 = i;
                        i = i + 1;
                        *baret
                            .offset(
                                fresh7 as isize,
                            ) = *(*((*p).pict_txt).offset(t as isize))
                            .offset(tc as isize);
                    } else {
                        fprintf(
                            dap_err,
                            b"(show1) Text too long: %s\n\0" as *const u8
                                as *const libc::c_char,
                            *((*p).pict_txt).offset(t as isize),
                        );
                        exit(1 as libc::c_int);
                    }
                }
                tc += 1;
                tc;
            }
            *baret.offset(i as isize) = '\0' as i32 as libc::c_char;
            if i == 0 {
                continue;
            }
            lfact = (nfull + 0.7f64 * nscript) / (nfull + nscript + nchange);
            if *(*((*p).pict_pos).offset(t as isize)).offset(2 as libc::c_int as isize)
                as libc::c_int == ' ' as i32
            {
                efffs = (*p).pict_fs;
                if nchange > 0.0f64 {
                    efffs *= 1.4f64;
                }
                hside = strlen(baret) as libc::c_double * lfact * (*p).pict_fs * 0.9f64;
                fputs(b"gs n 1 sg\n\0" as *const u8 as *const libc::c_char, pict_out);
                fprintf(
                    pict_out,
                    b"%.6f %.6f tr %.3f rot\n\0" as *const u8 as *const libc::c_char,
                    *(*((*p).pict_tpt).offset(t as isize))
                        .offset(0 as libc::c_int as isize),
                    *(*((*p).pict_tpt).offset(t as isize))
                        .offset(1 as libc::c_int as isize),
                    *((*p).pict_tang).offset(t as isize),
                );
                match *(*((*p).pict_pos).offset(t as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_int
                {
                    108 => {
                        fprintf(
                            pict_out,
                            b"%.3f %.3f tr\n\0" as *const u8 as *const libc::c_char,
                            -(1.3f64 - 1.0f64) * (*p).pict_fs,
                            yoff - 0.5f64 * efffs,
                        );
                    }
                    99 => {
                        fprintf(
                            pict_out,
                            b"%.3f %.3f tr\n\0" as *const u8 as *const libc::c_char,
                            -0.5f64 * hside,
                            yoff - 0.5f64 * efffs,
                        );
                    }
                    114 => {
                        fprintf(
                            pict_out,
                            b"%.3f %.3f tr\n\0" as *const u8 as *const libc::c_char,
                            -hside + (1.3f64 - 1.0f64) * (*p).pict_fs,
                            yoff - 0.5f64 * efffs,
                        );
                    }
                    _ => {}
                }
                fprintf(
                    pict_out,
                    b" 0 0 m %.3f 0 rl 0 %.3f rl %.3f 0 rl cp f\n\0" as *const u8
                        as *const libc::c_char,
                    hside,
                    1.3f64 * efffs,
                    -hside,
                );
                fputs(b"gr\n\0" as *const u8 as *const libc::c_char, pict_out);
            }
            fputs(b"gs\n\0" as *const u8 as *const libc::c_char, pict_out);
            fprintf(
                pict_out,
                b"n %.2f sg\n\0" as *const u8 as *const libc::c_char,
                (*p).pict_lgray,
            );
            fprintf(
                pict_out,
                b"/%s ff %.3f scf sf\n\0" as *const u8 as *const libc::c_char,
                (*p).pict_font,
                (*p).pict_fs,
            );
            fprintf(
                pict_out,
                b"%.6f %.6f tr %.3f rot\n\0" as *const u8 as *const libc::c_char,
                *(*((*p).pict_tpt).offset(t as isize)).offset(0 as libc::c_int as isize),
                *(*((*p).pict_tpt).offset(t as isize)).offset(1 as libc::c_int as isize),
                *((*p).pict_tang).offset(t as isize),
            );
            if *(*((*p).pict_pos).offset(t as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int == 'c' as i32
                || *(*((*p).pict_pos).offset(t as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_int == 'r' as i32
            {
                fprintf(
                    pict_out,
                    b"0 %.3f m (%s) sw pop %.3f mul \0" as *const u8
                        as *const libc::c_char,
                    yoff,
                    baret,
                    lfact,
                );
                if *(*((*p).pict_pos).offset(t as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_int == 'c' as i32
                {
                    fputs(
                        b"-2 div 0 rm\n\0" as *const u8 as *const libc::c_char,
                        pict_out,
                    );
                } else {
                    fputs(b"neg 0 rm\n\0" as *const u8 as *const libc::c_char, pict_out);
                }
            } else {
                fprintf(
                    pict_out,
                    b"0 %.3f m \0" as *const u8 as *const libc::c_char,
                    yoff,
                );
            }
            tc = toff;
            inscript = 0 as libc::c_int;
            while *(*((*p).pict_txt).offset(t as isize)).offset(tc as isize)
                as libc::c_int != 0
                && *(*((*p).pict_txt).offset(t as isize)).offset(tc as isize)
                    as libc::c_int != '\n' as i32
            {
                i = 0 as libc::c_int;
                while *(*((*p).pict_txt).offset(t as isize)).offset(tc as isize)
                    as libc::c_int != 0
                    && *(*((*p).pict_txt).offset(t as isize)).offset(tc as isize)
                        as libc::c_int != '^' as i32
                    && *(*((*p).pict_txt).offset(t as isize)).offset(tc as isize)
                        as libc::c_int != '|' as i32
                    && *(*((*p).pict_txt).offset(t as isize)).offset(tc as isize)
                        as libc::c_int != '\n' as i32
                {
                    if i < dap_maxtxt {
                        if *(*((*p).pict_txt).offset(t as isize)).offset(tc as isize)
                            as libc::c_int == '(' as i32
                            || *(*((*p).pict_txt).offset(t as isize)).offset(tc as isize)
                                as libc::c_int == ')' as i32
                        {
                            let fresh8 = i;
                            i = i + 1;
                            *tpart.offset(fresh8 as isize) = '\\' as i32 as libc::c_char;
                            if i == dap_maxtxt {
                                fprintf(
                                    dap_err,
                                    b"(show1) Text too long: %s\n\0" as *const u8
                                        as *const libc::c_char,
                                    *((*p).pict_txt).offset(t as isize),
                                );
                                exit(1 as libc::c_int);
                            }
                        }
                        *tpart
                            .offset(
                                i as isize,
                            ) = *(*((*p).pict_txt).offset(t as isize))
                            .offset(tc as isize);
                    } else {
                        fprintf(
                            dap_err,
                            b"(show1) Text too long: %s\n\0" as *const u8
                                as *const libc::c_char,
                            *((*p).pict_txt).offset(t as isize),
                        );
                        exit(1 as libc::c_int);
                    }
                    i += 1;
                    i;
                    tc += 1;
                    tc;
                }
                *tpart.offset(i as isize) = '\0' as i32 as libc::c_char;
                fprintf(
                    pict_out,
                    b"(%s) sh \0" as *const u8 as *const libc::c_char,
                    tpart,
                );
                if *(*((*p).pict_txt).offset(t as isize)).offset(tc as isize)
                    as libc::c_int == '^' as i32
                    || *(*((*p).pict_txt).offset(t as isize)).offset(tc as isize)
                        as libc::c_int == '|' as i32
                {
                    if inscript != 0 {
                        fprintf(
                            pict_out,
                            b"/%s ff %.3f scf sf\n\0" as *const u8
                                as *const libc::c_char,
                            (*p).pict_font,
                            (*p).pict_fs,
                        );
                        if *(*((*p).pict_txt).offset(t as isize)).offset(tc as isize)
                            as libc::c_int == '^' as i32
                        {
                            fprintf(
                                pict_out,
                                b"0 %.3f rm \0" as *const u8 as *const libc::c_char,
                                -0.4f64 * (*p).pict_fs,
                            );
                        } else {
                            fprintf(
                                pict_out,
                                b"0 %.3f rm \0" as *const u8 as *const libc::c_char,
                                0.4f64 * (*p).pict_fs,
                            );
                        }
                        inscript = 0 as libc::c_int;
                    } else {
                        fprintf(
                            pict_out,
                            b"/%s ff %.3f scf sf\n\0" as *const u8
                                as *const libc::c_char,
                            (*p).pict_font,
                            0.7f64 * (*p).pict_fs,
                        );
                        if *(*((*p).pict_txt).offset(t as isize)).offset(tc as isize)
                            as libc::c_int == '^' as i32
                        {
                            fprintf(
                                pict_out,
                                b"0 %.3f rm \0" as *const u8 as *const libc::c_char,
                                0.4f64 * (*p).pict_fs,
                            );
                        } else {
                            fprintf(
                                pict_out,
                                b"0 %.3f rm \0" as *const u8 as *const libc::c_char,
                                -0.4f64 * (*p).pict_fs,
                            );
                        }
                        inscript = 1 as libc::c_int;
                    }
                    tc += 1;
                    tc;
                }
            }
            fputs(b"\ngr\n\0" as *const u8 as *const libc::c_char, pict_out);
            toff = tc;
            while *(*((*p).pict_txt).offset(t as isize)).offset(toff as isize)
                as libc::c_int == '\n' as i32
            {
                toff += 1;
                toff;
                yoff -= linespace * (*p).pict_fs;
            }
        }
        t += 1;
        t;
    }
    return (*p).pict_next;
}
pub unsafe extern "C" fn pict_show(mut p: *mut pict) {
    while !p.is_null() {
        p = show1(p);
    }
}
pub unsafe extern "C" fn nport(
    mut p: *mut pict,
    mut nplots: libc::c_int,
    mut nperpage: libc::c_int,
) {
    let mut pn: libc::c_int = 0;
    if nplots % nperpage != 0 {
        fprintf(
            dap_err,
            b"(nport) Number of plots %d not a multiple of number per page %d\n\0"
                as *const u8 as *const libc::c_char,
            nplots,
            nperpage,
        );
        exit(1 as libc::c_int);
    }
    pict_port(nplots / nperpage);
    pn = 0 as libc::c_int;
    while pn < nplots {
        pict_page();
        pict_show(p.offset(pn as isize));
        pn += nperpage;
    }
    pict_end();
}
pub unsafe extern "C" fn nland(
    mut p: *mut pict,
    mut nplots: libc::c_int,
    mut nperpage: libc::c_int,
) {
    let mut pn: libc::c_int = 0;
    if nplots % nperpage != 0 {
        fprintf(
            dap_err,
            b"(pict_nlandscape) Number of plots %d not a multiple of number per page %d\n\0"
                as *const u8 as *const libc::c_char,
            nplots,
            nperpage,
        );
        exit(1 as libc::c_int);
    }
    pict_land(nplots / nperpage);
    pn = 0 as libc::c_int;
    while pn < nplots {
        pict_page();
        pict_show(p.offset(pn as isize));
        pn += nperpage;
    }
    pict_end();
}
