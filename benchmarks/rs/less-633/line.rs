use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn is_utf8_well_formed(ss: *mut libc::c_char, slen: libc::c_int) -> libc::c_int;
    fn xbuf_init(xbuf: *mut xbuffer);
    fn xbuf_reset(xbuf: *mut xbuffer);
    fn xbuf_add_byte(xbuf: *mut xbuffer, b: libc::c_uchar);
    fn xbuf_char_data(xbuf: *mut xbuffer) -> *mut libc::c_char;
    fn ecalloc(count: libc::c_int, size: libc::c_uint) -> *mut libc::c_void;
    fn parse_color(
        str: *mut libc::c_char,
        p_fg: *mut libc::c_int,
        p_bg: *mut libc::c_int,
    ) -> COLOR_TYPE;
    fn is_at_equiv(attr1: libc::c_int, attr2: libc::c_int) -> libc::c_int;
    fn apply_at_specials(attr: libc::c_int) -> libc::c_int;
    fn ch_seek(pos: POSITION) -> libc::c_int;
    fn ch_tell() -> POSITION;
    fn ch_forw_get() -> libc::c_int;
    fn ch_back_get() -> libc::c_int;
    fn control_char(c: LWCHAR) -> libc::c_int;
    fn prchar(c: LWCHAR) -> *mut libc::c_char;
    fn prutfchar(ch: LWCHAR) -> *mut libc::c_char;
    fn utf_len(ch: libc::c_int) -> libc::c_int;
    fn get_wchar(p: *const libc::c_char) -> LWCHAR;
    fn step_char(
        pp: *mut *mut libc::c_char,
        dir: libc::c_int,
        limit: *const libc::c_char,
    ) -> LWCHAR;
    fn is_composing_char(ch: LWCHAR) -> libc::c_int;
    fn is_ubin_char(ch: LWCHAR) -> libc::c_int;
    fn is_wide_char(ch: LWCHAR) -> libc::c_int;
    fn is_combining_char(ch1: LWCHAR, ch2: LWCHAR) -> libc::c_int;
    fn lgetenv(var: *mut libc::c_char) -> *mut libc::c_char;
    fn isnullenv(s: *mut libc::c_char) -> libc::c_int;
    fn forw_line(curr_pos: POSITION) -> POSITION;
    fn linenumtoa(_: LINENUM, _: *mut libc::c_char, _: libc::c_int);
    fn vlinenum(linenum: LINENUM) -> LINENUM;
    fn posmark(pos: POSITION) -> libc::c_char;
    fn find_linenum(pos: POSITION) -> LINENUM;
    fn is_hilited_attr(
        pos: POSITION,
        epos: POSITION,
        nohide: libc::c_int,
        p_matches: *mut libc::c_int,
    ) -> libc::c_int;
    fn xbuf_set(dst: *mut xbuffer, src: *mut xbuffer);
    fn position(sindex: libc::c_int) -> POSITION;
    static mut sigs: libc::c_int;
    static mut bs_mode: libc::c_int;
    static mut proc_backspace: libc::c_int;
    static mut proc_tab: libc::c_int;
    static mut proc_return: libc::c_int;
    static mut linenums: libc::c_int;
    static mut ctldisp: libc::c_int;
    static mut twiddle: libc::c_int;
    static mut status_col: libc::c_int;
    static mut status_col_width: libc::c_int;
    static mut linenum_width: libc::c_int;
    static mut auto_wrap: libc::c_int;
    static mut ignaw: libc::c_int;
    static mut bo_s_width: libc::c_int;
    static mut bo_e_width: libc::c_int;
    static mut ul_s_width: libc::c_int;
    static mut ul_e_width: libc::c_int;
    static mut bl_s_width: libc::c_int;
    static mut bl_e_width: libc::c_int;
    static mut so_s_width: libc::c_int;
    static mut so_e_width: libc::c_int;
    static mut sc_width: libc::c_int;
    static mut sc_height: libc::c_int;
    static mut utf_mode: libc::c_int;
    static mut start_attnpos: POSITION;
    static mut end_attnpos: POSITION;
    static mut rscroll_char: libc::c_char;
    static mut rscroll_attr: libc::c_int;
    static mut use_color: libc::c_int;
    static mut status_line: libc::c_int;
}
pub type __off_t = libc::c_long;
pub type off_t = __off_t;
pub type LWCHAR = libc::c_ulong;
pub type POSITION = off_t;
pub type LINENUM = off_t;
pub type COLOR_TYPE = libc::c_uint;
pub const CT_6BIT: COLOR_TYPE = 2;
pub const CT_4BIT: COLOR_TYPE = 1;
pub const CT_NULL: COLOR_TYPE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ansi_state {
    pub hindex: libc::c_int,
    pub hlink: libc::c_int,
    pub prev_esc: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xbuffer {
    pub data: *mut libc::c_uchar,
    pub end: libc::c_int,
    pub size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub buf: *mut libc::c_char,
    pub attr: *mut libc::c_int,
    pub print: libc::c_int,
    pub end: libc::c_int,
    pub pfx: [libc::c_char; 21],
    pub pfx_attr: [libc::c_int; 21],
    pub pfx_end: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct color_map {
    pub attr: libc::c_int,
    pub color: [libc::c_char; 12],
}
static mut linebuf: C2RustUnnamed = C2RustUnnamed {
    buf: 0 as *const libc::c_char as *mut libc::c_char,
    attr: 0 as *const libc::c_int as *mut libc::c_int,
    print: 0,
    end: 0,
    pfx: [0; 21],
    pfx_attr: [0; 21],
    pfx_end: 0,
};
static mut shifted_ansi: xbuffer = xbuffer {
    data: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    end: 0,
    size: 0,
};
static mut last_ansi: xbuffer = xbuffer {
    data: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    end: 0,
    size: 0,
};
static mut last_ansis: [xbuffer; 3] = [xbuffer {
    data: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    end: 0,
    size: 0,
}; 3];
static mut curr_last_ansi: libc::c_int = 0;
pub static mut size_linebuf: libc::c_int = 0 as libc::c_int;
static mut line_ansi: *mut ansi_state = 0 as *const ansi_state as *mut ansi_state;
static mut ansi_in_line: libc::c_int = 0;
static mut hlink_in_line: libc::c_int = 0;
static mut line_mark_attr: libc::c_int = 0;
static mut cshift: libc::c_int = 0;
pub static mut hshift: libc::c_int = 0;
pub static mut tabstops: [libc::c_int; 128] = [
    0 as libc::c_int,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
pub static mut ntabstops: libc::c_int = 1 as libc::c_int;
pub static mut tabdefault: libc::c_int = 8 as libc::c_int;
pub static mut highest_hilite: POSITION = 0;
static mut line_pos: POSITION = 0;
static mut end_column: libc::c_int = 0;
static mut right_curr: libc::c_int = 0;
static mut right_column: libc::c_int = 0;
static mut overstrike: libc::c_int = 0;
static mut last_overstrike: libc::c_int = 0 as libc::c_int;
static mut is_null_line: libc::c_int = 0;
static mut pendc: LWCHAR = 0;
static mut pendpos: POSITION = 0;
static mut end_ansi_chars: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut mid_ansi_chars: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut in_hilite: libc::c_int = 0;
static mut mbc_buf: [libc::c_char; 6] = [0; 6];
static mut mbc_buf_len: libc::c_int = 0 as libc::c_int;
static mut mbc_buf_index: libc::c_int = 0 as libc::c_int;
static mut mbc_pos: POSITION = 0;
static mut saved_line_end: libc::c_int = 0;
static mut saved_end_column: libc::c_int = 0;
static mut color_map: [color_map; 19] = unsafe {
    [
        {
            let mut init = color_map {
                attr: (1 as libc::c_int) << 0 as libc::c_int,
                color: *::std::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (1 as libc::c_int) << 1 as libc::c_int,
                color: *::std::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (1 as libc::c_int) << 2 as libc::c_int,
                color: *::std::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (1 as libc::c_int) << 3 as libc::c_int,
                color: *::std::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (1 as libc::c_int) << 8 as libc::c_int,
                color: *::std::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"Wm\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (2 as libc::c_int) << 8 as libc::c_int,
                color: *::std::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"kR\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (3 as libc::c_int) << 8 as libc::c_int,
                color: *::std::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"kR\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (4 as libc::c_int) << 8 as libc::c_int,
                color: *::std::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"kY\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (5 as libc::c_int) << 8 as libc::c_int,
                color: *::std::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"c\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (6 as libc::c_int) << 8 as libc::c_int,
                color: *::std::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"Wb\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (7 as libc::c_int) << 8 as libc::c_int,
                color: *::std::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"kC\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (8 as libc::c_int) << 8 as libc::c_int,
                color: *::std::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"kc\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (9 as libc::c_int) << 8 as libc::c_int,
                color: *::std::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (10 as libc::c_int) << 8 as libc::c_int,
                color: *::std::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"kG\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (10 as libc::c_int + 1 as libc::c_int) << 8 as libc::c_int,
                color: *::std::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"ky\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (10 as libc::c_int + 2 as libc::c_int) << 8 as libc::c_int,
                color: *::std::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"wb\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (10 as libc::c_int + 3 as libc::c_int) << 8 as libc::c_int,
                color: *::std::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"YM\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (10 as libc::c_int + 4 as libc::c_int) << 8 as libc::c_int,
                color: *::std::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"Yr\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = color_map {
                attr: (10 as libc::c_int + 5 as libc::c_int) << 8 as libc::c_int,
                color: *::std::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"Wc\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
    ]
};
pub unsafe extern "C" fn init_line() {
    let mut ax: libc::c_int = 0;
    end_ansi_chars = lgetenv(
        b"LESSANSIENDCHARS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if isnullenv(end_ansi_chars) != 0 {
        end_ansi_chars = b"m\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    mid_ansi_chars = lgetenv(
        b"LESSANSIMIDCHARS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if isnullenv(mid_ansi_chars) != 0 {
        mid_ansi_chars = b"0123456789:;[?!\"'#%()*+ \0" as *const u8
            as *const libc::c_char as *mut libc::c_char;
    }
    linebuf
        .buf = ecalloc(
        1024 as libc::c_int,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_uint,
    ) as *mut libc::c_char;
    linebuf
        .attr = ecalloc(
        1024 as libc::c_int,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint,
    ) as *mut libc::c_int;
    size_linebuf = 1024 as libc::c_int;
    xbuf_init(&mut shifted_ansi);
    xbuf_init(&mut last_ansi);
    ax = 0 as libc::c_int;
    while ax < 3 as libc::c_int {
        xbuf_init(&mut *last_ansis.as_mut_ptr().offset(ax as isize));
        ax += 1;
        ax;
    }
    curr_last_ansi = 0 as libc::c_int;
}
unsafe extern "C" fn expand_linebuf() -> libc::c_int {
    let mut new_size: libc::c_int = size_linebuf * 2 as libc::c_int;
    let mut new_buf: *mut libc::c_char = calloc(
        new_size as libc::c_ulong,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
    ) as *mut libc::c_char;
    let mut new_attr: *mut libc::c_int = calloc(
        new_size as libc::c_ulong,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    if new_buf.is_null() || new_attr.is_null() {
        if !new_attr.is_null() {
            free(new_attr as *mut libc::c_void);
        }
        if !new_buf.is_null() {
            free(new_buf as *mut libc::c_void);
        }
        return 1 as libc::c_int;
    }
    memcpy(
        new_buf as *mut libc::c_void,
        linebuf.buf as *const libc::c_void,
        (size_linebuf as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    memcpy(
        new_attr as *mut libc::c_void,
        linebuf.attr as *const libc::c_void,
        (size_linebuf as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    free(linebuf.attr as *mut libc::c_void);
    free(linebuf.buf as *mut libc::c_void);
    linebuf.buf = new_buf;
    linebuf.attr = new_attr;
    size_linebuf = new_size;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn is_ascii_char(mut ch: LWCHAR) -> libc::c_int {
    return (ch <= 0x7f as libc::c_int as libc::c_ulong) as libc::c_int;
}
unsafe extern "C" fn inc_end_column(mut w: libc::c_int) {
    if end_column > right_column && w > 0 as libc::c_int {
        right_column = end_column;
        right_curr = linebuf.end;
    }
    end_column += w;
}
pub unsafe extern "C" fn line_position() -> POSITION {
    return line_pos;
}
pub unsafe extern "C" fn prewind() {
    let mut ax: libc::c_int = 0;
    linebuf.print = 6 as libc::c_int;
    linebuf.pfx_end = 0 as libc::c_int;
    linebuf.end = 0 as libc::c_int;
    while linebuf.end < linebuf.print {
        *(linebuf.buf).offset(linebuf.end as isize) = '\0' as i32 as libc::c_char;
        *(linebuf.attr).offset(linebuf.end as isize) = 0 as libc::c_int;
        linebuf.end += 1;
        linebuf.end;
    }
    end_column = 0 as libc::c_int;
    right_curr = 0 as libc::c_int;
    right_column = 0 as libc::c_int;
    cshift = 0 as libc::c_int;
    overstrike = 0 as libc::c_int;
    last_overstrike = 0 as libc::c_int;
    mbc_buf_len = 0 as libc::c_int;
    is_null_line = 0 as libc::c_int;
    pendc = '\0' as i32 as LWCHAR;
    in_hilite = 0 as libc::c_int;
    ansi_in_line = 0 as libc::c_int;
    hlink_in_line = 0 as libc::c_int;
    line_mark_attr = 0 as libc::c_int;
    line_pos = -(1 as libc::c_int) as POSITION;
    xbuf_reset(&mut shifted_ansi);
    xbuf_reset(&mut last_ansi);
    ax = 0 as libc::c_int;
    while ax < 3 as libc::c_int {
        xbuf_reset(&mut *last_ansis.as_mut_ptr().offset(ax as isize));
        ax += 1;
        ax;
    }
    curr_last_ansi = 0 as libc::c_int;
}
unsafe extern "C" fn set_linebuf(
    mut n: libc::c_int,
    mut ch: libc::c_char,
    mut attr: libc::c_int,
) {
    if n >= size_linebuf {
        if expand_linebuf() != 0 {
            return;
        }
    }
    *(linebuf.buf).offset(n as isize) = ch;
    *(linebuf.attr).offset(n as isize) = attr;
}
unsafe extern "C" fn add_linebuf(
    mut ch: libc::c_char,
    mut attr: libc::c_int,
    mut w: libc::c_int,
) {
    let fresh0 = linebuf.end;
    linebuf.end = linebuf.end + 1;
    set_linebuf(fresh0, ch, attr);
    inc_end_column(w);
}
unsafe extern "C" fn addstr_linebuf(
    mut s: *mut libc::c_char,
    mut attr: libc::c_int,
    mut cw: libc::c_int,
) {
    while *s as libc::c_int != '\0' as i32 {
        add_linebuf(*s, attr, cw);
        s = s.offset(1);
        s;
    }
}
unsafe extern "C" fn set_pfx(
    mut n: libc::c_int,
    mut ch: libc::c_char,
    mut attr: libc::c_int,
) {
    linebuf.pfx[n as usize] = ch;
    linebuf.pfx_attr[n as usize] = attr;
}
unsafe extern "C" fn add_pfx(mut ch: libc::c_char, mut attr: libc::c_int) {
    let fresh1 = linebuf.pfx_end;
    linebuf.pfx_end = linebuf.pfx_end + 1;
    set_pfx(fresh1, ch, attr);
}
pub unsafe extern "C" fn plinestart(mut pos: POSITION) {
    let mut linenum: LINENUM = 0 as libc::c_int as LINENUM;
    let mut i: libc::c_int = 0;
    if linenums == 2 as libc::c_int {
        linenum = find_linenum(pos);
    }
    if status_col != 0 || status_line != 0 {
        let mut c: libc::c_char = posmark(pos);
        if c as libc::c_int != 0 as libc::c_int {
            line_mark_attr = (1 as libc::c_int) << 6 as libc::c_int
                | (6 as libc::c_int) << 8 as libc::c_int;
        } else if start_attnpos != -(1 as libc::c_int) as POSITION
            && pos >= start_attnpos && pos <= end_attnpos
        {
            line_mark_attr = (1 as libc::c_int) << 6 as libc::c_int
                | (1 as libc::c_int) << 8 as libc::c_int;
        }
        if status_col != 0 {
            add_pfx(
                (if c as libc::c_int != 0 { c as libc::c_int } else { ' ' as i32 })
                    as libc::c_char,
                line_mark_attr,
            );
            while linebuf.pfx_end < status_col_width {
                add_pfx(' ' as i32 as libc::c_char, 0 as libc::c_int);
            }
        }
    }
    if linenums == 2 as libc::c_int {
        let mut buf: [libc::c_char; 23] = [0; 23];
        let mut len: libc::c_int = 0;
        linenum = vlinenum(linenum);
        if linenum == 0 as libc::c_int as libc::c_long {
            len = 0 as libc::c_int;
        } else {
            linenumtoa(linenum, buf.as_mut_ptr(), 10 as libc::c_int);
            len = strlen(buf.as_mut_ptr()) as libc::c_int;
        }
        i = 0 as libc::c_int;
        while i < linenum_width - len {
            add_pfx(' ' as i32 as libc::c_char, 0 as libc::c_int);
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        while i < len {
            add_pfx(
                buf[i as usize],
                (1 as libc::c_int) << 1 as libc::c_int
                    | (5 as libc::c_int) << 8 as libc::c_int,
            );
            i += 1;
            i;
        }
        add_pfx(' ' as i32 as libc::c_char, 0 as libc::c_int);
    }
    end_column = linebuf.pfx_end;
}
pub unsafe extern "C" fn line_pfx_width() -> libc::c_int {
    let mut width: libc::c_int = 0 as libc::c_int;
    if status_col != 0 {
        width += status_col_width;
    }
    if linenums == 2 as libc::c_int {
        width += linenum_width + 1 as libc::c_int;
    }
    return width;
}
pub unsafe extern "C" fn pshift_all() {
    let mut i: libc::c_int = 0;
    i = linebuf.print;
    while i < linebuf.end {
        if *(linebuf.attr).offset(i as isize) == (1 as libc::c_int) << 4 as libc::c_int {
            xbuf_add_byte(
                &mut shifted_ansi,
                *(linebuf.buf).offset(i as isize) as libc::c_uchar,
            );
        }
        i += 1;
        i;
    }
    linebuf.end = linebuf.print;
    end_column = linebuf.pfx_end;
}
unsafe extern "C" fn attr_swidth(mut a: libc::c_int) -> libc::c_int {
    let mut w: libc::c_int = 0 as libc::c_int;
    a = apply_at_specials(a);
    if a & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        w += ul_s_width;
    }
    if a & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        w += bo_s_width;
    }
    if a & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        w += bl_s_width;
    }
    if a & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        w += so_s_width;
    }
    return w;
}
unsafe extern "C" fn attr_ewidth(mut a: libc::c_int) -> libc::c_int {
    let mut w: libc::c_int = 0 as libc::c_int;
    a = apply_at_specials(a);
    if a & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        w += ul_e_width;
    }
    if a & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        w += bo_e_width;
    }
    if a & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        w += bl_e_width;
    }
    if a & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        w += so_e_width;
    }
    return w;
}
pub unsafe extern "C" fn pwidth(
    mut ch: LWCHAR,
    mut a: libc::c_int,
    mut prev_ch: LWCHAR,
    mut prev_a: libc::c_int,
) -> libc::c_int {
    let mut w: libc::c_int = 0;
    if ch == '\u{8}' as i32 as libc::c_ulong {
        if prev_a
            & ((1 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 5 as libc::c_int) != 0
        {
            return strlen(prchar('\u{8}' as i32 as LWCHAR)) as libc::c_int;
        }
        return if utf_mode != 0 && is_wide_char(prev_ch) != 0 {
            -(2 as libc::c_int)
        } else {
            -(1 as libc::c_int)
        };
    }
    if utf_mode == 0 || is_ascii_char(ch) != 0 {
        if control_char(ch as libc::c_char as LWCHAR) != 0 {
            return 0 as libc::c_int;
        }
    } else if is_composing_char(ch) != 0 || is_combining_char(prev_ch, ch) != 0 {
        return 0 as libc::c_int
    }
    w = 1 as libc::c_int;
    if is_wide_char(ch) != 0 {
        w += 1;
        w;
    }
    if linebuf.end > 0 as libc::c_int
        && is_at_equiv(
            *(linebuf.attr).offset((linebuf.end - 1 as libc::c_int) as isize),
            a,
        ) == 0
    {
        w
            += attr_ewidth(
                *(linebuf.attr).offset((linebuf.end - 1 as libc::c_int) as isize),
            );
    }
    if apply_at_specials(a) != 0 as libc::c_int
        && (linebuf.end == 0 as libc::c_int
            || is_at_equiv(
                *(linebuf.attr).offset((linebuf.end - 1 as libc::c_int) as isize),
                a,
            ) == 0)
    {
        w += attr_swidth(a);
    }
    return w;
}
unsafe extern "C" fn backc() -> libc::c_int {
    let mut ch: LWCHAR = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if linebuf.end == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    p = &mut *(linebuf.buf).offset(linebuf.end as isize) as *mut libc::c_char;
    ch = step_char(&mut p, -(1 as libc::c_int), linebuf.buf);
    while p > linebuf.buf {
        let mut prev_ch: LWCHAR = 0;
        let mut width: libc::c_int = 0;
        linebuf.end = p.offset_from(linebuf.buf) as libc::c_long as libc::c_int;
        prev_ch = step_char(&mut p, -(1 as libc::c_int), linebuf.buf);
        width = pwidth(
            ch,
            *(linebuf.attr).offset(linebuf.end as isize),
            prev_ch,
            *(linebuf.attr).offset((linebuf.end - 1 as libc::c_int) as isize),
        );
        end_column -= width;
        if width > 0 as libc::c_int {
            break;
        }
        ch = prev_ch;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn savec() {
    saved_line_end = linebuf.end;
    saved_end_column = end_column;
}
pub unsafe extern "C" fn loadc() {
    linebuf.end = saved_line_end;
    end_column = saved_end_column;
}
pub unsafe extern "C" fn is_ansi_end(mut ch: LWCHAR) -> libc::c_int {
    if is_ascii_char(ch) == 0 {
        return 0 as libc::c_int;
    }
    return (strchr(end_ansi_chars, ch as libc::c_char as libc::c_int)
        != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
}
pub unsafe extern "C" fn is_ansi_middle(mut ch: LWCHAR) -> libc::c_int {
    if is_ascii_char(ch) == 0 {
        return 0 as libc::c_int;
    }
    if is_ansi_end(ch) != 0 {
        return 0 as libc::c_int;
    }
    return (strchr(mid_ansi_chars, ch as libc::c_char as libc::c_int)
        != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
}
pub unsafe extern "C" fn skip_ansi(
    mut pansi: *mut ansi_state,
    mut pp: *mut *mut libc::c_char,
    mut limit: *const libc::c_char,
) {
    let mut c: LWCHAR = 0;
    loop {
        c = step_char(pp, 1 as libc::c_int, limit);
        if !(*pp < limit as *mut libc::c_char && ansi_step(pansi, c) == 1 as libc::c_int)
        {
            break;
        }
    };
}
pub unsafe extern "C" fn ansi_start(mut ch: LWCHAR) -> *mut ansi_state {
    let mut pansi: *mut ansi_state = 0 as *mut ansi_state;
    if !(ch == ('[' as i32 & 0o37 as libc::c_int) as libc::c_ulong
        || ch == -101i32 as libc::c_uchar as libc::c_ulong)
    {
        return 0 as *mut ansi_state;
    }
    pansi = ecalloc(
        1 as libc::c_int,
        ::std::mem::size_of::<ansi_state>() as libc::c_ulong as libc::c_uint,
    ) as *mut ansi_state;
    (*pansi).hindex = 0 as libc::c_int;
    (*pansi).hlink = 0 as libc::c_int;
    (*pansi).prev_esc = 0 as libc::c_int;
    return pansi;
}
pub unsafe extern "C" fn ansi_step(
    mut pansi: *mut ansi_state,
    mut ch: LWCHAR,
) -> libc::c_int {
    if (*pansi).hlink != 0 {
        if ch == '\u{7}' as i32 as libc::c_ulong {
            return 3 as libc::c_int;
        }
        if (*pansi).prev_esc != 0 {
            return if ch == '\\' as i32 as libc::c_ulong {
                3 as libc::c_int
            } else {
                2 as libc::c_int
            };
        }
        (*pansi)
            .prev_esc = (ch == ('[' as i32 & 0o37 as libc::c_int) as libc::c_ulong)
            as libc::c_int;
        return 1 as libc::c_int;
    }
    if (*pansi).hindex >= 0 as libc::c_int {
        static mut hlink_prefix: [libc::c_char; 5] = unsafe {
            *::std::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"\x1B]8;\0")
        };
        if ch == hlink_prefix[(*pansi).hindex as usize] as libc::c_ulong
            || (*pansi).hindex == 0 as libc::c_int
                && (ch == ('[' as i32 & 0o37 as libc::c_int) as libc::c_ulong
                    || ch == -101i32 as libc::c_uchar as libc::c_ulong)
        {
            (*pansi).hindex += 1;
            (*pansi).hindex;
            if hlink_prefix[(*pansi).hindex as usize] as libc::c_int == '\0' as i32 {
                (*pansi).hlink = 1 as libc::c_int;
            }
            return 1 as libc::c_int;
        }
        (*pansi).hindex = -(1 as libc::c_int);
    }
    if is_ansi_middle(ch) != 0 {
        return 1 as libc::c_int;
    }
    if is_ansi_end(ch) != 0 {
        return 3 as libc::c_int;
    }
    return 2 as libc::c_int;
}
pub unsafe extern "C" fn ansi_done(mut pansi: *mut ansi_state) {
    free(pansi as *mut libc::c_void);
}
unsafe extern "C" fn fits_on_screen(
    mut w: libc::c_int,
    mut a: libc::c_int,
) -> libc::c_int {
    if ctldisp == 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    return (end_column - cshift + w + attr_ewidth(a) <= sc_width) as libc::c_int;
}
unsafe extern "C" fn store_char(
    mut ch: LWCHAR,
    mut a: libc::c_int,
    mut rep: *mut libc::c_char,
    mut pos: POSITION,
) -> libc::c_int {
    let mut w: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut replen: libc::c_int = 0;
    let mut cs: libc::c_char = 0;
    i = a
        & ((1 as libc::c_int) << 0 as libc::c_int
            | (1 as libc::c_int) << 1 as libc::c_int);
    if i != 0 as libc::c_int {
        last_overstrike = i;
    }
    let mut matches: libc::c_int = 0;
    let mut resend_last: libc::c_int = 0 as libc::c_int;
    let mut hl_attr: libc::c_int = 0 as libc::c_int;
    if pos == -(1 as libc::c_int) as POSITION {
        hl_attr = if ansi_in_line != 0 {
            0 as libc::c_int
        } else {
            (1 as libc::c_int) << 3 as libc::c_int
                | (7 as libc::c_int) << 8 as libc::c_int
        };
    } else if a != (1 as libc::c_int) << 4 as libc::c_int {
        hl_attr = is_hilited_attr(
            pos,
            pos + 1 as libc::c_int as libc::c_long,
            0 as libc::c_int,
            &mut matches,
        );
        if hl_attr == 0 as libc::c_int && status_line != 0 {
            hl_attr = line_mark_attr;
        }
    }
    if hl_attr != 0 {
        a |= hl_attr;
        if highest_hilite != -(1 as libc::c_int) as POSITION
            && pos != -(1 as libc::c_int) as POSITION && pos > highest_hilite
        {
            highest_hilite = pos;
        }
        in_hilite = 1 as libc::c_int;
    } else {
        if in_hilite != 0 {
            resend_last = 1 as libc::c_int;
        }
        in_hilite = 0 as libc::c_int;
    }
    if resend_last != 0 {
        let mut ai: libc::c_int = 0;
        ai = 0 as libc::c_int;
        while ai < 3 as libc::c_int {
            let mut ax: libc::c_int = (curr_last_ansi + ai) % 3 as libc::c_int;
            i = 0 as libc::c_int;
            while i < last_ansis[ax as usize].end {
                if store_char(
                    *(last_ansis[ax as usize].data).offset(i as isize) as LWCHAR,
                    (1 as libc::c_int) << 4 as libc::c_int,
                    0 as *mut libc::c_char,
                    pos,
                ) != 0
                {
                    return 1 as libc::c_int;
                }
                i += 1;
                i;
            }
            ai += 1;
            ai;
        }
    }
    if a == (1 as libc::c_int) << 4 as libc::c_int {
        w = 0 as libc::c_int;
    } else {
        let mut p: *mut libc::c_char = &mut *(linebuf.buf).offset(linebuf.end as isize)
            as *mut libc::c_char;
        let mut prev_ch: LWCHAR = if linebuf.end > 0 as libc::c_int {
            step_char(&mut p, -(1 as libc::c_int), linebuf.buf)
        } else {
            0 as libc::c_int as libc::c_ulong
        };
        let mut prev_a: libc::c_int = if linebuf.end > 0 as libc::c_int {
            *(linebuf.attr).offset((linebuf.end - 1 as libc::c_int) as isize)
        } else {
            0 as libc::c_int
        };
        w = pwidth(ch, a, prev_ch, prev_a);
    }
    if fits_on_screen(w, a) == 0 {
        return 1 as libc::c_int;
    }
    if rep.is_null() {
        cs = ch as libc::c_char;
        rep = &mut cs;
        replen = 1 as libc::c_int;
    } else {
        replen = utf_len(*rep.offset(0 as libc::c_int as isize) as libc::c_int);
    }
    if cshift == hshift {
        if line_pos == -(1 as libc::c_int) as POSITION {
            line_pos = pos;
        }
        if shifted_ansi.end > 0 as libc::c_int {
            i = 0 as libc::c_int;
            while i < shifted_ansi.end {
                add_linebuf(
                    *(shifted_ansi.data).offset(i as isize) as libc::c_char,
                    (1 as libc::c_int) << 4 as libc::c_int,
                    0 as libc::c_int,
                );
                i += 1;
                i;
            }
            xbuf_reset(&mut shifted_ansi);
        }
    }
    inc_end_column(w);
    i = 0 as libc::c_int;
    while i < replen {
        let fresh2 = rep;
        rep = rep.offset(1);
        add_linebuf(*fresh2, a, 0 as libc::c_int);
        i += 1;
        i;
    }
    if cshift < hshift {
        if a == (1 as libc::c_int) << 4 as libc::c_int {
            xbuf_add_byte(&mut shifted_ansi, ch as libc::c_uchar);
        }
        if linebuf.end > linebuf.print {
            let mut i_0: libc::c_int = 0;
            i_0 = 0 as libc::c_int;
            while i_0 < linebuf.print {
                *(linebuf.buf)
                    .offset(
                        i_0 as isize,
                    ) = *(linebuf.buf).offset((i_0 + replen) as isize);
                *(linebuf.attr)
                    .offset(
                        i_0 as isize,
                    ) = *(linebuf.attr).offset((i_0 + replen) as isize);
                i_0 += 1;
                i_0;
            }
            linebuf.end -= replen;
            cshift += w;
            while cshift > hshift {
                add_linebuf(' ' as i32 as libc::c_char, rscroll_attr, 0 as libc::c_int);
                cshift -= 1;
                cshift;
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn store_string(
    mut s: *mut libc::c_char,
    mut a: libc::c_int,
    mut pos: POSITION,
) -> libc::c_int {
    if fits_on_screen(strlen(s) as libc::c_int, a) == 0 {
        return 1 as libc::c_int;
    }
    while *s as libc::c_int != 0 as libc::c_int {
        if store_char(*s as LWCHAR, a, 0 as *mut libc::c_char, pos) != 0 {
            return 1 as libc::c_int;
        }
        s = s.offset(1);
        s;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn store_tab(mut attr: libc::c_int, mut pos: POSITION) -> libc::c_int {
    let mut to_tab: libc::c_int = end_column - linebuf.pfx_end;
    if ntabstops < 2 as libc::c_int
        || to_tab >= tabstops[(ntabstops - 1 as libc::c_int) as usize]
    {
        to_tab = tabdefault
            - (to_tab - tabstops[(ntabstops - 1 as libc::c_int) as usize]) % tabdefault;
    } else {
        let mut i: libc::c_int = 0;
        i = ntabstops - 2 as libc::c_int;
        while i >= 0 as libc::c_int {
            if to_tab >= tabstops[i as usize] {
                break;
            }
            i -= 1;
            i;
        }
        to_tab = tabstops[(i + 1 as libc::c_int) as usize] - to_tab;
    }
    loop {
        if store_char(
            ' ' as i32 as LWCHAR,
            attr,
            b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            pos,
        ) != 0
        {
            return 1 as libc::c_int;
        }
        to_tab -= 1;
        if !(to_tab > 0 as libc::c_int) {
            break;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn store_prchar(mut c: LWCHAR, mut pos: POSITION) -> libc::c_int {
    if store_string(
        prchar(c),
        (1 as libc::c_int) << 5 as libc::c_int | (3 as libc::c_int) << 8 as libc::c_int,
        pos,
    ) != 0
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn flush_mbc_buf(mut pos: POSITION) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < mbc_buf_index {
        if store_prchar(mbc_buf[i as usize] as LWCHAR, pos) != 0 {
            return mbc_buf_index - i;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn pappend(mut c: libc::c_int, mut pos: POSITION) -> libc::c_int {
    let mut r: libc::c_int = 0;
    if pendc != 0 {
        if c == '\r' as i32 && pendc == '\r' as i32 as libc::c_ulong {
            return 0 as libc::c_int;
        }
        if do_append(pendc, 0 as *mut libc::c_char, pendpos) != 0 {
            return 1 as libc::c_int;
        }
        pendc = '\0' as i32 as LWCHAR;
    }
    if c == '\r' as i32
        && (proc_return == 1 as libc::c_int
            || bs_mode == 0 as libc::c_int && proc_return == 0 as libc::c_int)
    {
        if mbc_buf_len > 0 as libc::c_int {
            r = flush_mbc_buf(mbc_pos);
            mbc_buf_index = r + 1 as libc::c_int;
            mbc_buf_len = 0 as libc::c_int;
            if r != 0 {
                return mbc_buf_index;
            }
        }
        pendc = c as LWCHAR;
        pendpos = pos;
        return 0 as libc::c_int;
    }
    if utf_mode == 0 {
        r = do_append(c as LWCHAR, 0 as *mut libc::c_char, pos);
    } else {
        let mut current_block_41: u64;
        if mbc_buf_len == 0 as libc::c_int {
            current_block_41 = 14859409266032611811;
        } else if c & 0xc0 as libc::c_int == 0x80 as libc::c_int {
            let fresh3 = mbc_buf_index;
            mbc_buf_index = mbc_buf_index + 1;
            mbc_buf[fresh3 as usize] = c as libc::c_char;
            if mbc_buf_index < mbc_buf_len {
                return 0 as libc::c_int;
            }
            if is_utf8_well_formed(mbc_buf.as_mut_ptr(), mbc_buf_index) != 0 {
                r = do_append(
                    get_wchar(mbc_buf.as_mut_ptr()),
                    mbc_buf.as_mut_ptr(),
                    mbc_pos,
                );
            } else {
                r = flush_mbc_buf(mbc_pos);
                mbc_buf_index = r;
            }
            mbc_buf_len = 0 as libc::c_int;
            current_block_41 = 9520865839495247062;
        } else {
            r = flush_mbc_buf(mbc_pos);
            mbc_buf_index = r + 1 as libc::c_int;
            mbc_buf_len = 0 as libc::c_int;
            if r == 0 {
                current_block_41 = 14859409266032611811;
            } else {
                current_block_41 = 9520865839495247062;
            }
        }
        match current_block_41 {
            14859409266032611811 => {
                mbc_buf_index = 1 as libc::c_int;
                *mbc_buf.as_mut_ptr() = c as libc::c_char;
                if c & 0x80 as libc::c_int == 0 as libc::c_int {
                    r = do_append(c as LWCHAR, 0 as *mut libc::c_char, pos);
                } else if c & 0xc0 as libc::c_int == 0xc0 as libc::c_int
                    && !(c & 0xfe as libc::c_int == 0xfe as libc::c_int)
                {
                    mbc_buf_len = utf_len(c);
                    mbc_pos = pos;
                    return 0 as libc::c_int;
                } else {
                    r = flush_mbc_buf(pos);
                }
            }
            _ => {}
        }
    }
    if r != 0 {
        r = if utf_mode == 0 { 1 as libc::c_int } else { mbc_buf_index };
    }
    return r;
}
unsafe extern "C" fn store_control_char(
    mut ch: LWCHAR,
    mut rep: *mut libc::c_char,
    mut pos: POSITION,
) -> libc::c_int {
    if ctldisp == 1 as libc::c_int {
        if store_char(ch, 0 as libc::c_int, rep, pos) != 0 {
            return 1 as libc::c_int;
        }
    } else if store_prchar(ch as libc::c_char as LWCHAR, pos) != 0 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn store_ansi(
    mut ch: LWCHAR,
    mut rep: *mut libc::c_char,
    mut pos: POSITION,
) -> libc::c_int {
    match ansi_step(line_ansi, ch) {
        1 => {
            if store_char(ch, (1 as libc::c_int) << 4 as libc::c_int, rep, pos) != 0 {
                return 1 as libc::c_int;
            }
            if (*line_ansi).hlink != 0 {
                hlink_in_line = 1 as libc::c_int;
            }
            xbuf_add_byte(&mut last_ansi, ch as libc::c_uchar);
        }
        3 => {
            if store_char(ch, (1 as libc::c_int) << 4 as libc::c_int, rep, pos) != 0 {
                return 1 as libc::c_int;
            }
            ansi_done(line_ansi);
            line_ansi = 0 as *mut ansi_state;
            xbuf_add_byte(&mut last_ansi, ch as libc::c_uchar);
            xbuf_set(
                &mut *last_ansis.as_mut_ptr().offset(curr_last_ansi as isize),
                &mut last_ansi,
            );
            xbuf_reset(&mut last_ansi);
            curr_last_ansi = (curr_last_ansi + 1 as libc::c_int) % 3 as libc::c_int;
        }
        2 => {
            let mut start: *mut libc::c_char = if cshift < hshift {
                xbuf_char_data(&mut shifted_ansi)
            } else {
                linebuf.buf
            };
            let mut end: *mut libc::c_int = if cshift < hshift {
                &mut shifted_ansi.end
            } else {
                &mut linebuf.end
            };
            let mut p: *mut libc::c_char = start.offset(*end as isize);
            let mut bch: LWCHAR = 0;
            loop {
                bch = step_char(&mut p, -(1 as libc::c_int), start);
                if !(p > start
                    && !(bch == ('[' as i32 & 0o37 as libc::c_int) as libc::c_ulong
                        || bch == -101i32 as libc::c_uchar as libc::c_ulong))
                {
                    break;
                }
            }
            *end = p.offset_from(start) as libc::c_long as libc::c_int;
            xbuf_reset(&mut last_ansi);
            ansi_done(line_ansi);
            line_ansi = 0 as *mut ansi_state;
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn store_bs(
    mut ch: LWCHAR,
    mut rep: *mut libc::c_char,
    mut pos: POSITION,
) -> libc::c_int {
    if proc_backspace == 2 as libc::c_int
        || bs_mode == 2 as libc::c_int && proc_backspace == 0 as libc::c_int
    {
        return store_control_char(ch, rep, pos);
    }
    if linebuf.end > 0 as libc::c_int
        && (linebuf.end <= linebuf.print
            && *(linebuf.buf).offset((linebuf.end - 1 as libc::c_int) as isize)
                as libc::c_int == '\0' as i32
            || linebuf.end > 0 as libc::c_int
                && *(linebuf.attr).offset((linebuf.end - 1 as libc::c_int) as isize)
                    & ((1 as libc::c_int) << 4 as libc::c_int
                        | (1 as libc::c_int) << 5 as libc::c_int) != 0)
    {
        if store_prchar('\u{8}' as i32 as LWCHAR, pos) != 0 {
            return 1 as libc::c_int;
        }
    } else if proc_backspace == 0 as libc::c_int && bs_mode == 1 as libc::c_int {
        if store_char(ch, 0 as libc::c_int, 0 as *mut libc::c_char, pos) != 0 {
            return 1 as libc::c_int;
        }
    } else if proc_backspace == 1 as libc::c_int
        || bs_mode == 0 as libc::c_int && proc_backspace == 0 as libc::c_int
    {
        overstrike = backc();
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn do_append(
    mut ch: LWCHAR,
    mut rep: *mut libc::c_char,
    mut pos: POSITION,
) -> libc::c_int {
    let mut a: libc::c_int = 0 as libc::c_int;
    let mut in_overstrike: libc::c_int = overstrike;
    if ctldisp == 2 as libc::c_int && line_ansi.is_null() {
        line_ansi = ansi_start(ch);
        if !line_ansi.is_null() {
            ansi_in_line = 1 as libc::c_int;
        }
    }
    overstrike = 0 as libc::c_int;
    if !line_ansi.is_null() {
        return store_ansi(ch, rep, pos);
    }
    if ch == '\u{8}' as i32 as libc::c_ulong {
        return store_bs(ch, rep, pos);
    }
    if in_overstrike > 0 as libc::c_int {
        let mut prev_ch: LWCHAR = 0;
        overstrike = if utf_mode != 0 { -(1 as libc::c_int) } else { 0 as libc::c_int };
        if utf_mode != 0 {
            prev_ch = get_wchar(&mut *(linebuf.buf).offset(linebuf.end as isize));
        } else {
            prev_ch = *(linebuf.buf).offset(linebuf.end as isize) as libc::c_uchar
                as LWCHAR;
        }
        a = *(linebuf.attr).offset(linebuf.end as isize);
        if ch == prev_ch {
            if ch == '_' as i32 as libc::c_ulong {
                if a
                    & ((1 as libc::c_int) << 1 as libc::c_int
                        | (1 as libc::c_int) << 0 as libc::c_int) != 0 as libc::c_int
                {
                    a
                        |= (1 as libc::c_int) << 1 as libc::c_int
                            | (1 as libc::c_int) << 0 as libc::c_int;
                } else if last_overstrike != 0 as libc::c_int {
                    a |= last_overstrike;
                } else {
                    a |= (1 as libc::c_int) << 1 as libc::c_int;
                }
            } else {
                a |= (1 as libc::c_int) << 1 as libc::c_int;
            }
        } else if ch == '_' as i32 as libc::c_ulong {
            a |= (1 as libc::c_int) << 0 as libc::c_int;
            ch = prev_ch;
            rep = &mut *(linebuf.buf).offset(linebuf.end as isize) as *mut libc::c_char;
        } else if prev_ch == '_' as i32 as libc::c_ulong {
            a |= (1 as libc::c_int) << 0 as libc::c_int;
        }
    } else if in_overstrike < 0 as libc::c_int {
        if is_composing_char(ch) != 0
            || is_combining_char(
                get_wchar(&mut *(linebuf.buf).offset(linebuf.end as isize)),
                ch,
            ) != 0
        {
            a = last_overstrike;
        } else {
            overstrike = 0 as libc::c_int;
        }
    }
    if ch == '\t' as i32 as libc::c_ulong {
        if proc_tab == 2 as libc::c_int
            || bs_mode == 2 as libc::c_int && proc_tab == 0 as libc::c_int
        {
            return store_control_char(ch, rep, pos);
        }
        if store_tab(a, pos) != 0 {
            return 1 as libc::c_int;
        }
        return 0 as libc::c_int;
    }
    if (utf_mode == 0 || is_ascii_char(ch) != 0)
        && control_char(ch as libc::c_char as LWCHAR) != 0
    {
        return store_control_char(ch, rep, pos)
    } else if utf_mode != 0 && ctldisp != 1 as libc::c_int && is_ubin_char(ch) != 0 {
        if store_string(prutfchar(ch), (1 as libc::c_int) << 5 as libc::c_int, pos) != 0
        {
            return 1 as libc::c_int;
        }
    } else if store_char(ch, a, rep, pos) != 0 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn pflushmbc() -> libc::c_int {
    let mut r: libc::c_int = 0 as libc::c_int;
    if mbc_buf_len > 0 as libc::c_int {
        r = flush_mbc_buf(mbc_pos);
        mbc_buf_len = 0 as libc::c_int;
    }
    return r;
}
unsafe extern "C" fn add_attr_normal() {
    if ctldisp != 2 as libc::c_int || is_ansi_end('m' as i32 as LWCHAR) == 0 {
        return;
    }
    addstr_linebuf(
        b"\x1B[m\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (1 as libc::c_int) << 4 as libc::c_int,
        0 as libc::c_int,
    );
    if hlink_in_line != 0 {
        addstr_linebuf(
            b"\x1B]8;;\x1B\\\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (1 as libc::c_int) << 4 as libc::c_int,
            0 as libc::c_int,
        );
    }
}
pub unsafe extern "C" fn pdone(
    mut endline: libc::c_int,
    mut chopped: libc::c_int,
    mut forw: libc::c_int,
) {
    pflushmbc();
    if pendc != 0 && (pendc != '\r' as i32 as libc::c_ulong || endline == 0) {
        do_append(pendc, 0 as *mut libc::c_char, pendpos);
    }
    if chopped != 0 && rscroll_char as libc::c_int != 0 {
        if end_column >= sc_width + cshift {
            end_column = right_column;
            linebuf.end = right_curr;
        }
        add_attr_normal();
        while end_column < sc_width - 1 as libc::c_int + cshift {
            add_linebuf(' ' as i32 as libc::c_char, rscroll_attr, 1 as libc::c_int);
        }
        add_linebuf(rscroll_char, rscroll_attr, 1 as libc::c_int);
    } else {
        add_attr_normal();
    }
    if status_line != 0 && line_mark_attr != 0 as libc::c_int {
        while (end_column + 1 as libc::c_int) < sc_width + cshift {
            add_linebuf(' ' as i32 as libc::c_char, line_mark_attr, 1 as libc::c_int);
        }
    }
    if end_column < sc_width + cshift || auto_wrap == 0 || endline != 0 && ignaw != 0
        || ctldisp == 1 as libc::c_int
    {
        add_linebuf('\n' as i32 as libc::c_char, 0 as libc::c_int, 0 as libc::c_int);
    } else if ignaw != 0 && end_column >= sc_width + cshift && forw != 0 {
        add_linebuf(' ' as i32 as libc::c_char, 0 as libc::c_int, 1 as libc::c_int);
        add_linebuf(
            '\u{8}' as i32 as libc::c_char,
            0 as libc::c_int,
            -(1 as libc::c_int),
        );
    }
    set_linebuf(linebuf.end, '\0' as i32 as libc::c_char, 0 as libc::c_int);
}
pub unsafe extern "C" fn set_attr_line(mut a: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = linebuf.print;
    while i < linebuf.end {
        if *(linebuf.attr).offset(i as isize)
            & (16 as libc::c_int - 1 as libc::c_int) << 8 as libc::c_int
            == 0 as libc::c_int
            || a & (16 as libc::c_int - 1 as libc::c_int) << 8 as libc::c_int
                == 0 as libc::c_int
        {
            *(linebuf.attr).offset(i as isize) |= a;
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn set_status_col(mut c: libc::c_char, mut attr: libc::c_int) {
    set_pfx(0 as libc::c_int, c, attr);
}
pub unsafe extern "C" fn gline(
    mut i: libc::c_int,
    mut ap: *mut libc::c_int,
) -> libc::c_int {
    if is_null_line != 0 {
        if twiddle != 0 {
            if i == 0 as libc::c_int {
                *ap = (1 as libc::c_int) << 1 as libc::c_int;
                return '~' as i32;
            }
            i -= 1;
            i;
        }
        *ap = 0 as libc::c_int;
        return if i != 0 { '\0' as i32 } else { '\n' as i32 };
    }
    if i < linebuf.pfx_end {
        *ap = linebuf.pfx_attr[i as usize];
        return linebuf.pfx[i as usize] as libc::c_int;
    }
    i += linebuf.print - linebuf.pfx_end;
    *ap = *(linebuf.attr).offset(i as isize);
    return *(linebuf.buf).offset(i as isize) as libc::c_int & 0xff as libc::c_int;
}
pub unsafe extern "C" fn null_line() {
    is_null_line = 1 as libc::c_int;
    cshift = 0 as libc::c_int;
}
pub unsafe extern "C" fn forw_raw_line(
    mut curr_pos: POSITION,
    mut linep: *mut *mut libc::c_char,
    mut line_lenp: *mut libc::c_int,
) -> POSITION {
    let mut n: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut new_pos: POSITION = 0;
    if curr_pos == -(1 as libc::c_int) as POSITION || ch_seek(curr_pos) != 0
        || {
            c = ch_forw_get();
            c == -(1 as libc::c_int)
        }
    {
        return -(1 as libc::c_int) as POSITION;
    }
    n = 0 as libc::c_int;
    loop {
        if c == '\n' as i32 || c == -(1 as libc::c_int)
            || sigs & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0
        {
            new_pos = ch_tell();
            break;
        } else {
            if n >= size_linebuf - 1 as libc::c_int {
                if expand_linebuf() != 0 {
                    new_pos = ch_tell() - 1 as libc::c_int as libc::c_long;
                    break;
                }
            }
            let fresh4 = n;
            n = n + 1;
            *(linebuf.buf).offset(fresh4 as isize) = c as libc::c_char;
            c = ch_forw_get();
        }
    }
    *(linebuf.buf).offset(n as isize) = '\0' as i32 as libc::c_char;
    if !linep.is_null() {
        *linep = linebuf.buf;
    }
    if !line_lenp.is_null() {
        *line_lenp = n;
    }
    return new_pos;
}
pub unsafe extern "C" fn back_raw_line(
    mut curr_pos: POSITION,
    mut linep: *mut *mut libc::c_char,
    mut line_lenp: *mut libc::c_int,
) -> POSITION {
    let mut n: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut new_pos: POSITION = 0;
    if curr_pos == -(1 as libc::c_int) as POSITION
        || curr_pos <= 0 as libc::c_int as POSITION
        || ch_seek(curr_pos - 1 as libc::c_int as libc::c_long) != 0
    {
        return -(1 as libc::c_int) as POSITION;
    }
    n = size_linebuf;
    n -= 1;
    *(linebuf.buf).offset(n as isize) = '\0' as i32 as libc::c_char;
    loop {
        c = ch_back_get();
        if c == '\n' as i32 || sigs & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0 {
            new_pos = ch_tell() + 1 as libc::c_int as libc::c_long;
            break;
        } else if c == -(1 as libc::c_int) {
            new_pos = 0 as libc::c_int as POSITION;
            break;
        } else {
            if n <= 0 as libc::c_int {
                let mut old_size_linebuf: libc::c_int = size_linebuf;
                let mut fm: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut to: *mut libc::c_char = 0 as *mut libc::c_char;
                if expand_linebuf() != 0 {
                    new_pos = ch_tell() + 1 as libc::c_int as libc::c_long;
                    break;
                } else {
                    fm = (linebuf.buf)
                        .offset(old_size_linebuf as isize)
                        .offset(-(1 as libc::c_int as isize));
                    to = (linebuf.buf)
                        .offset(size_linebuf as isize)
                        .offset(-(1 as libc::c_int as isize));
                    while fm >= linebuf.buf {
                        *to = *fm;
                        fm = fm.offset(-1);
                        fm;
                        to = to.offset(-1);
                        to;
                    }
                    n = size_linebuf - old_size_linebuf;
                }
            }
            n -= 1;
            *(linebuf.buf).offset(n as isize) = c as libc::c_char;
        }
    }
    if !linep.is_null() {
        *linep = &mut *(linebuf.buf).offset(n as isize) as *mut libc::c_char;
    }
    if !line_lenp.is_null() {
        *line_lenp = size_linebuf - 1 as libc::c_int - n;
    }
    return new_pos;
}
pub unsafe extern "C" fn skip_columns(
    mut cols: libc::c_int,
    mut linep: *mut *mut libc::c_char,
    mut line_lenp: *mut libc::c_int,
) -> libc::c_int {
    let mut line: *mut libc::c_char = *linep;
    let mut eline: *mut libc::c_char = line.offset(*line_lenp as isize);
    let mut pch: LWCHAR = 0 as libc::c_int as LWCHAR;
    let mut bytes: libc::c_int = 0;
    while cols > 0 as libc::c_int && line < eline {
        let mut ch: LWCHAR = step_char(&mut line, 1 as libc::c_int, eline);
        let mut pansi: *mut ansi_state = ansi_start(ch);
        if !pansi.is_null() {
            skip_ansi(pansi, &mut line, eline);
            ansi_done(pansi);
            pch = 0 as libc::c_int as LWCHAR;
        } else {
            let mut w: libc::c_int = pwidth(ch, 0 as libc::c_int, pch, 0 as libc::c_int);
            cols -= w;
            pch = ch;
        }
    }
    bytes = line.offset_from(*linep) as libc::c_long as libc::c_int;
    *linep = line;
    *line_lenp -= bytes;
    return bytes;
}
unsafe extern "C" fn pappstr(mut str: *const libc::c_char) -> libc::c_int {
    while *str as libc::c_int != '\0' as i32 {
        let fresh5 = str;
        str = str.offset(1);
        if pappend(*fresh5 as libc::c_int, -(1 as libc::c_int) as POSITION) != 0 {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn load_line(mut str: *const libc::c_char) {
    let mut save_hshift: libc::c_int = hshift;
    hshift = 0 as libc::c_int;
    loop {
        prewind();
        if pappstr(str) == 0 as libc::c_int {
            break;
        }
        hshift += 1 as libc::c_int;
    }
    set_linebuf(linebuf.end, '\0' as i32 as libc::c_char, 0 as libc::c_int);
    hshift = save_hshift;
}
pub unsafe extern "C" fn rrshift() -> libc::c_int {
    let mut pos: POSITION = 0;
    let mut save_width: libc::c_int = 0;
    let mut line: libc::c_int = 0;
    let mut longest: libc::c_int = 0 as libc::c_int;
    save_width = sc_width;
    sc_width = 2147483647 as libc::c_int;
    pos = position(0 as libc::c_int);
    line = 0 as libc::c_int;
    while line < sc_height && pos != -(1 as libc::c_int) as POSITION {
        pos = forw_line(pos);
        if end_column > longest {
            longest = end_column;
        }
        line += 1;
        line;
    }
    sc_width = save_width;
    if longest < sc_width {
        return 0 as libc::c_int;
    }
    return longest - sc_width;
}
unsafe extern "C" fn lookup_color_index(mut attr: libc::c_int) -> libc::c_int {
    let mut cx: libc::c_int = 0;
    cx = 0 as libc::c_int;
    while (cx as libc::c_ulong)
        < (::std::mem::size_of::<[color_map; 19]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<color_map>() as libc::c_ulong)
    {
        if color_map[cx as usize].attr == attr {
            return cx;
        }
        cx += 1;
        cx;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn color_index(mut attr: libc::c_int) -> libc::c_int {
    if use_color != 0
        && attr & (16 as libc::c_int - 1 as libc::c_int) << 8 as libc::c_int != 0
    {
        return lookup_color_index(
            attr & (16 as libc::c_int - 1 as libc::c_int) << 8 as libc::c_int,
        );
    }
    if attr & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        return lookup_color_index((1 as libc::c_int) << 0 as libc::c_int);
    }
    if attr & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        return lookup_color_index((1 as libc::c_int) << 1 as libc::c_int);
    }
    if attr & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        return lookup_color_index((1 as libc::c_int) << 2 as libc::c_int);
    }
    if attr & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        return lookup_color_index((1 as libc::c_int) << 3 as libc::c_int);
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn set_color_map(
    mut attr: libc::c_int,
    mut colorstr: *mut libc::c_char,
) -> libc::c_int {
    let mut cx: libc::c_int = color_index(attr);
    if cx < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (strlen(colorstr)).wrapping_add(1 as libc::c_int as libc::c_ulong)
        > ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
    {
        return -(1 as libc::c_int);
    }
    if *colorstr as libc::c_int != '\0' as i32
        && parse_color(colorstr, 0 as *mut libc::c_int, 0 as *mut libc::c_int)
            as libc::c_uint == CT_NULL as libc::c_int as libc::c_uint
    {
        return -(1 as libc::c_int);
    }
    strcpy((color_map[cx as usize].color).as_mut_ptr(), colorstr);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn get_color_map(mut attr: libc::c_int) -> *mut libc::c_char {
    let mut cx: libc::c_int = color_index(attr);
    if cx < 0 as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    return (color_map[cx as usize].color).as_mut_ptr();
}
