use ::libc;
extern "C" {
    pub type ldat;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn can_change_color() -> bool;
    fn curs_set(_: libc::c_int) -> libc::c_int;
    fn echo() -> libc::c_int;
    fn endwin() -> libc::c_int;
    fn initscr() -> *mut WINDOW;
    fn init_color(
        _: libc::c_short,
        _: libc::c_short,
        _: libc::c_short,
        _: libc::c_short,
    ) -> libc::c_int;
    fn init_pair(_: libc::c_short, _: libc::c_short, _: libc::c_short) -> libc::c_int;
    fn noecho() -> libc::c_int;
    fn start_color() -> libc::c_int;
    fn waddch(_: *mut WINDOW, _: chtype) -> libc::c_int;
    fn wattrset(_: *mut WINDOW, _: libc::c_int) -> libc::c_int;
    fn wattr_on(_: *mut WINDOW, _: attr_t, _: *mut libc::c_void) -> libc::c_int;
    fn wbkgd(_: *mut WINDOW, _: chtype) -> libc::c_int;
    fn wclear(_: *mut WINDOW) -> libc::c_int;
    fn wmove(_: *mut WINDOW, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn wrefresh(_: *mut WINDOW) -> libc::c_int;
    fn wtimeout(_: *mut WINDOW, _: libc::c_int);
    fn use_default_colors() -> libc::c_int;
    static mut stdscr: *mut WINDOW;
    static mut LINES: libc::c_int;
    static mut COLS: libc::c_int;
    fn waddnwstr(_: *mut WINDOW, _: *const wchar_t, _: libc::c_int) -> libc::c_int;
    static mut COLORS: libc::c_int;
    static mut COLOR_PAIRS: libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn system(__command: *const libc::c_char) -> libc::c_int;
}
pub type WINDOW = _win_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _win_st {
    pub _cury: libc::c_short,
    pub _curx: libc::c_short,
    pub _maxy: libc::c_short,
    pub _maxx: libc::c_short,
    pub _begy: libc::c_short,
    pub _begx: libc::c_short,
    pub _flags: libc::c_short,
    pub _attrs: attr_t,
    pub _bkgd: chtype,
    pub _notimeout: bool,
    pub _clear: bool,
    pub _leaveok: bool,
    pub _scroll: bool,
    pub _idlok: bool,
    pub _idcok: bool,
    pub _immed: bool,
    pub _sync: bool,
    pub _use_keypad: bool,
    pub _delay: libc::c_int,
    pub _line: *mut ldat,
    pub _regtop: libc::c_short,
    pub _regbottom: libc::c_short,
    pub _parx: libc::c_int,
    pub _pary: libc::c_int,
    pub _parent: *mut WINDOW,
    pub _pad: pdat,
    pub _yoffset: libc::c_short,
    pub _bkgrnd: cchar_t,
    pub _color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cchar_t {
    pub attr: attr_t,
    pub chars: [wchar_t; 5],
    pub ext_color: libc::c_int,
}
pub type wchar_t = libc::c_int;
pub type attr_t = chtype;
pub type chtype = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdat {
    pub _pad_y: libc::c_short,
    pub _pad_x: libc::c_short,
    pub _pad_top: libc::c_short,
    pub _pad_left: libc::c_short,
    pub _pad_bottom: libc::c_short,
    pub _pad_right: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct colors {
    pub color: libc::c_short,
    pub R: libc::c_short,
    pub G: libc::c_short,
    pub B: libc::c_short,
}
pub type FILE = _IO_FILE;
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
pub type size_t = libc::c_ulong;
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub static mut gradient_size: libc::c_int = 64 as libc::c_int;
pub static mut bar_heights: [*const wchar_t; 8] = unsafe {
    [
        (*::std::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b"\x81%\0\0\0\0\0\0"))
            .as_ptr(),
        (*::std::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b"\x82%\0\0\0\0\0\0"))
            .as_ptr(),
        (*::std::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b"\x83%\0\0\0\0\0\0"))
            .as_ptr(),
        (*::std::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b"\x84%\0\0\0\0\0\0"))
            .as_ptr(),
        (*::std::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b"\x85%\0\0\0\0\0\0"))
            .as_ptr(),
        (*::std::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b"\x86%\0\0\0\0\0\0"))
            .as_ptr(),
        (*::std::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b"\x87%\0\0\0\0\0\0"))
            .as_ptr(),
        (*::std::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b"\x88%\0\0\0\0\0\0"))
            .as_ptr(),
    ]
};
pub static mut num_bar_heights: libc::c_int = 0;
unsafe extern "C" fn parse_color(
    mut color_string: *mut libc::c_char,
    mut color: *mut colors,
) {
    if *color_string.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32 {
        if !can_change_color() {
            cleanup_terminal_ncurses();
            fprintf(
                stderr,
                b"Your terminal can not change color definitions, please use one of the predefined colors.\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        (*color).color = -(2 as libc::c_int) as libc::c_short;
        color_string = color_string.offset(1);
        sscanf(
            color_string,
            b"%02hx%02hx%02hx\0" as *const u8 as *const libc::c_char,
            &mut (*color).R as *mut libc::c_short,
            &mut (*color).G as *mut libc::c_short,
            &mut (*color).B as *mut libc::c_short,
        );
    }
}
unsafe extern "C" fn change_color_definition(
    mut color_number: libc::c_short,
    color_string: *mut libc::c_char,
    mut predef_color: libc::c_short,
) -> libc::c_short {
    let mut color: colors = {
        let mut init = colors {
            color: 0 as libc::c_int as libc::c_short,
            R: 0,
            G: 0,
            B: 0,
        };
        init
    };
    parse_color(color_string, &mut color);
    let mut return_color_number: libc::c_short = predef_color;
    if color.color as libc::c_int == -(2 as libc::c_int) {
        init_color(
            color_number,
            (color.R as libc::c_int as libc::c_double * 1000.0f64
                / 0xff as libc::c_int as libc::c_double + 0.5f64) as libc::c_short,
            (color.G as libc::c_int as libc::c_double * 1000.0f64
                / 0xff as libc::c_int as libc::c_double + 0.5f64) as libc::c_short,
            (color.B as libc::c_int as libc::c_double * 1000.0f64
                / 0xff as libc::c_int as libc::c_double + 0.5f64) as libc::c_short,
        );
        return_color_number = color_number;
    }
    return return_color_number;
}
pub unsafe extern "C" fn init_terminal_ncurses(
    fg_color_string: *mut libc::c_char,
    bg_color_string: *mut libc::c_char,
    mut predef_fg_color: libc::c_int,
    mut predef_bg_color: libc::c_int,
    mut gradient: libc::c_int,
    mut gradient_count: libc::c_int,
    mut gradient_colors: *mut *mut libc::c_char,
    mut width: *mut libc::c_int,
    mut lines: *mut libc::c_int,
) {
    initscr();
    curs_set(0 as libc::c_int);
    wtimeout(stdscr, 0 as libc::c_int);
    noecho();
    start_color();
    use_default_colors();
    *lines = (if !(stdscr as *const libc::c_void).is_null() {
        (*stdscr)._maxy as libc::c_int + 1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    });
    *width = (if !(stdscr as *const libc::c_void).is_null() {
        (*stdscr)._maxx as libc::c_int + 1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    });
    wclear(stdscr);
    let mut color_pair_number: libc::c_short = 16 as libc::c_int as libc::c_short;
    let mut bg_color_number: libc::c_short = 0;
    bg_color_number = change_color_definition(
        0 as libc::c_int as libc::c_short,
        bg_color_string,
        predef_bg_color as libc::c_short,
    );
    if gradient == 0 {
        let mut fg_color_number: libc::c_short = 0;
        fg_color_number = change_color_definition(
            1 as libc::c_int as libc::c_short,
            fg_color_string,
            predef_fg_color as libc::c_short,
        );
        init_pair(color_pair_number, fg_color_number, bg_color_number);
    } else if gradient != 0 {
        let vla = (2 as libc::c_int * gradient_count - 1 as libc::c_int) as usize;
        let mut rgb: Vec::<[libc::c_ushort; 3]> = ::std::vec::from_elem([0; 3], vla);
        let mut next_color: [libc::c_char; 14] = [0; 14];
        gradient_size = *lines;
        if gradient_size > COLORS {
            gradient_size = COLORS - 1 as libc::c_int;
        }
        if gradient_size > COLOR_PAIRS {
            gradient_size = COLOR_PAIRS - 1 as libc::c_int;
        }
        if gradient_size > 256 as libc::c_int {
            gradient_size = 256 as libc::c_int - 1 as libc::c_int;
        }
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < gradient_count {
            let mut col: libc::c_int = (i + 1 as libc::c_int) * 2 as libc::c_int
                - 2 as libc::c_int;
            sscanf(
                (*gradient_colors.offset(i as isize)).offset(1 as libc::c_int as isize),
                b"%02hx%02hx%02hx\0" as *const u8 as *const libc::c_char,
                &mut *(*rgb.as_mut_ptr().offset(col as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as *mut libc::c_ushort,
                &mut *(*rgb.as_mut_ptr().offset(col as isize))
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize) as *mut libc::c_ushort,
                &mut *(*rgb.as_mut_ptr().offset(col as isize))
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize) as *mut libc::c_ushort,
            );
            i += 1;
            i;
        }
        let mut individual_size: libc::c_int = gradient_size
            / (gradient_count - 1 as libc::c_int);
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < gradient_count - 1 as libc::c_int {
            let mut col_0: libc::c_int = (i_0 + 1 as libc::c_int) * 2 as libc::c_int
                - 2 as libc::c_int;
            if i_0 == gradient_count - 1 as libc::c_int {
                col_0 = 2 as libc::c_int * (gradient_count - 1 as libc::c_int)
                    - 2 as libc::c_int;
            }
            let mut j: libc::c_int = 0 as libc::c_int;
            while j < individual_size {
                let mut k: libc::c_int = 0 as libc::c_int;
                while k < 3 as libc::c_int {
                    (*rgb
                        .as_mut_ptr()
                        .offset(
                            (col_0 + 1 as libc::c_int) as isize,
                        ))[k
                        as usize] = ((*rgb
                        .as_mut_ptr()
                        .offset(col_0 as isize))[k as usize] as libc::c_int
                        as libc::c_double
                        + ((*rgb
                            .as_mut_ptr()
                            .offset((col_0 + 2 as libc::c_int) as isize))[k as usize]
                            as libc::c_int
                            - (*rgb.as_mut_ptr().offset(col_0 as isize))[k as usize]
                                as libc::c_int) as libc::c_double
                            * (j as libc::c_double
                                / (individual_size as libc::c_double * 0.85f64)))
                        as libc::c_ushort;
                    if (*rgb
                        .as_mut_ptr()
                        .offset((col_0 + 1 as libc::c_int) as isize))[k as usize]
                        as libc::c_int > 255 as libc::c_int
                    {
                        (*rgb
                            .as_mut_ptr()
                            .offset(
                                col_0 as isize,
                            ))[k as usize] = 0 as libc::c_int as libc::c_ushort;
                    }
                    if j as libc::c_double > individual_size as libc::c_double * 0.85f64
                    {
                        (*rgb
                            .as_mut_ptr()
                            .offset(
                                (col_0 + 1 as libc::c_int) as isize,
                            ))[k
                            as usize] = (*rgb
                            .as_mut_ptr()
                            .offset((col_0 + 2 as libc::c_int) as isize))[k as usize];
                    }
                    k += 1;
                    k;
                }
                sprintf(
                    next_color.as_mut_ptr(),
                    b"#%02x%02x%02x\0" as *const u8 as *const libc::c_char,
                    (*rgb
                        .as_mut_ptr()
                        .offset(
                            (col_0 + 1 as libc::c_int) as isize,
                        ))[0 as libc::c_int as usize] as libc::c_int,
                    (*rgb
                        .as_mut_ptr()
                        .offset(
                            (col_0 + 1 as libc::c_int) as isize,
                        ))[1 as libc::c_int as usize] as libc::c_int,
                    (*rgb
                        .as_mut_ptr()
                        .offset(
                            (col_0 + 1 as libc::c_int) as isize,
                        ))[2 as libc::c_int as usize] as libc::c_int,
                );
                change_color_definition(
                    color_pair_number,
                    next_color.as_mut_ptr(),
                    color_pair_number,
                );
                init_pair(color_pair_number, color_pair_number, bg_color_number);
                color_pair_number += 1;
                color_pair_number;
                j += 1;
                j;
            }
            i_0 += 1;
            i_0;
        }
        let mut left: libc::c_int = individual_size
            * (gradient_count - 1 as libc::c_int);
        let mut col_1: libc::c_int = 2 as libc::c_int * gradient_count
            - 2 as libc::c_int;
        while left < gradient_size {
            sprintf(
                next_color.as_mut_ptr(),
                b"#%02x%02x%02x\0" as *const u8 as *const libc::c_char,
                (*rgb.as_mut_ptr().offset(col_1 as isize))[0 as libc::c_int as usize]
                    as libc::c_int,
                (*rgb.as_mut_ptr().offset(col_1 as isize))[1 as libc::c_int as usize]
                    as libc::c_int,
                (*rgb.as_mut_ptr().offset(col_1 as isize))[2 as libc::c_int as usize]
                    as libc::c_int,
            );
            change_color_definition(
                color_pair_number,
                next_color.as_mut_ptr(),
                color_pair_number,
            );
            init_pair(color_pair_number, color_pair_number, bg_color_number);
            color_pair_number += 1;
            color_pair_number;
            left += 1;
            left;
        }
        color_pair_number -= 1;
        color_pair_number;
    }
    wattr_on(
        stdscr,
        (color_pair_number as chtype) << 0 as libc::c_int + 8 as libc::c_int
            & ((1 as libc::c_uint) << 8 as libc::c_int).wrapping_sub(1 as libc::c_uint)
                << 0 as libc::c_int + 8 as libc::c_int,
        0 as *mut libc::c_void,
    );
    if bg_color_number as libc::c_int != -(1 as libc::c_int) {
        wbkgd(
            stdscr,
            (color_pair_number as chtype) << 0 as libc::c_int + 8 as libc::c_int
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint)
                    << 0 as libc::c_int + 8 as libc::c_int,
        );
    }
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < *lines {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < *width {
            if wmove(stdscr, y, x) == -(1 as libc::c_int) {
                -(1 as libc::c_int);
            } else {
                waddch(stdscr, ' ' as i32 as chtype);
            };
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    wrefresh(stdscr);
}
pub unsafe extern "C" fn change_colors(
    mut cur_height: libc::c_int,
    mut tot_height: libc::c_int,
) {
    tot_height /= gradient_size;
    if tot_height < 1 as libc::c_int {
        tot_height = 1 as libc::c_int;
    }
    cur_height /= tot_height;
    if cur_height > gradient_size - 1 as libc::c_int {
        cur_height = gradient_size - 1 as libc::c_int;
    }
    wattr_on(
        stdscr,
        ((cur_height + 16 as libc::c_int) as chtype)
            << 0 as libc::c_int + 8 as libc::c_int
            & ((1 as libc::c_uint) << 8 as libc::c_int).wrapping_sub(1 as libc::c_uint)
                << 0 as libc::c_int + 8 as libc::c_int,
        0 as *mut libc::c_void,
    );
}
pub unsafe extern "C" fn get_terminal_dim_ncurses(
    mut width: *mut libc::c_int,
    mut height: *mut libc::c_int,
) {
    *height = (if !(stdscr as *const libc::c_void).is_null() {
        (*stdscr)._maxy as libc::c_int + 1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    });
    *width = (if !(stdscr as *const libc::c_void).is_null() {
        (*stdscr)._maxx as libc::c_int + 1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    });
    gradient_size = *height;
    wclear(stdscr);
}
pub unsafe extern "C" fn draw_terminal_ncurses(
    mut is_tty: libc::c_int,
    mut terminal_height: libc::c_int,
    mut terminal_width: libc::c_int,
    mut bars_count: libc::c_int,
    mut bar_width: libc::c_int,
    mut bar_spacing: libc::c_int,
    mut rest: libc::c_int,
    mut bars: *const libc::c_int,
    mut previous_frame: *mut libc::c_int,
    mut gradient: libc::c_int,
    mut x_axis_info: libc::c_int,
) -> libc::c_int {
    let height: libc::c_int = terminal_height - 1 as libc::c_int;
    if is_tty == 0 {
        if x_axis_info != 0 {
            terminal_height += 1;
            terminal_height;
        }
        if LINES != terminal_height || COLS != terminal_width {
            return -(1 as libc::c_int);
        }
    }
    let mut max_update_y: libc::c_int = 0 as libc::c_int;
    let mut bar: libc::c_int = 0 as libc::c_int;
    while bar < bars_count {
        max_update_y = ({
            let mut _a: libc::c_int = max_update_y;
            let mut _b: libc::c_int = ({
                let _a_0: libc::c_int = *bars.offset(bar as isize);
                let mut _b_0: libc::c_int = *previous_frame.offset(bar as isize);
                if _a_0 > _b_0 { _a_0 } else { _b_0 }
            });
            if _a > _b { _a } else { _b }
        });
        bar += 1;
        bar;
    }
    max_update_y = (max_update_y + num_bar_heights) / num_bar_heights;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < max_update_y {
        if gradient != 0 {
            change_colors(y, height);
        }
        let mut current_block_31: u64;
        let mut bar_0: libc::c_int = 0 as libc::c_int;
        while bar_0 < bars_count {
            if !(*bars.offset(bar_0 as isize) == *previous_frame.offset(bar_0 as isize))
            {
                let mut cur_col: libc::c_int = bar_0 * bar_width + bar_0 * bar_spacing
                    + rest;
                let mut f_cell: libc::c_int = (*bars.offset(bar_0 as isize)
                    - 1 as libc::c_int) / num_bar_heights;
                let mut f_last_cell: libc::c_int = (*previous_frame
                    .offset(bar_0 as isize) - 1 as libc::c_int) / num_bar_heights;
                if f_cell >= y {
                    let mut bar_step: libc::c_int = 0;
                    if f_cell == y {
                        bar_step = (*bars.offset(bar_0 as isize) - 1 as libc::c_int)
                            % num_bar_heights;
                        current_block_31 = 7056779235015430508;
                    } else if f_last_cell <= y {
                        bar_step = num_bar_heights - 1 as libc::c_int;
                        current_block_31 = 7056779235015430508;
                    } else {
                        current_block_31 = 7149356873433890176;
                    }
                    match current_block_31 {
                        7149356873433890176 => {}
                        _ => {
                            let mut col: libc::c_int = cur_col;
                            let mut i: libc::c_int = 0 as libc::c_int;
                            while i < bar_width {
                                if is_tty != 0 {
                                    if wmove(stdscr, height - y, col) == -(1 as libc::c_int) {
                                        -(1 as libc::c_int);
                                    } else {
                                        waddch(stdscr, (0x41 as libc::c_int + bar_step) as chtype);
                                    };
                                } else {
                                    if wmove(stdscr, height - y, col) == -(1 as libc::c_int) {
                                        -(1 as libc::c_int);
                                    } else {
                                        waddnwstr(
                                            stdscr,
                                            bar_heights[bar_step as usize],
                                            -(1 as libc::c_int),
                                        );
                                    };
                                }
                                i += 1;
                                i;
                                col += 1;
                                col;
                            }
                        }
                    }
                } else if f_last_cell >= y {
                    let mut col_0: libc::c_int = cur_col;
                    let mut i_0: libc::c_int = 0 as libc::c_int;
                    while i_0 < bar_width {
                        if wmove(stdscr, height - y, col_0) == -(1 as libc::c_int) {
                            -(1 as libc::c_int);
                        } else {
                            waddch(stdscr, ' ' as i32 as chtype);
                        };
                        i_0 += 1;
                        i_0;
                        col_0 += 1;
                        col_0;
                    }
                }
            }
            bar_0 += 1;
            bar_0;
        }
        y += 1;
        y;
    }
    wrefresh(stdscr);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn cleanup_terminal_ncurses() {
    echo();
    system(b"setfont  >/dev/null 2>&1\0" as *const u8 as *const libc::c_char);
    system(
        b"setfont /usr/share/consolefonts/Lat2-Fixed16.psf.gz  >/dev/null 2>&1\0"
            as *const u8 as *const libc::c_char,
    );
    system(b"setterm -blank 10  >/dev/null 2>&1\0" as *const u8 as *const libc::c_char);
    wattrset(stdscr, (1 as libc::c_uint).wrapping_sub(1 as libc::c_uint) as libc::c_int);
    endwin();
    system(b"clear\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn run_static_initializers() {
    num_bar_heights = (::std::mem::size_of::<[*const wchar_t; 8]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<*const wchar_t>() as libc::c_ulong)
        as libc::c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
