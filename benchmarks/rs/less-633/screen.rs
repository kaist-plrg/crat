use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fsync(__fd: libc::c_int) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn save(s: *const libc::c_char) -> *mut libc::c_char;
    fn ecalloc(count: libc::c_int, size: libc::c_uint) -> *mut libc::c_void;
    fn lstrtoi(
        _: *mut libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int;
    fn isnullenv(s: *mut libc::c_char) -> libc::c_int;
    fn lgetenv(var: *mut libc::c_char) -> *mut libc::c_char;
    fn putstr(s: *const libc::c_char);
    fn get_color_map(attr: libc::c_int) -> *mut libc::c_char;
    fn clear_bot_if_needed();
    fn flush();
    fn sleep_ms(ms: libc::c_int);
    fn putchr(c: libc::c_int) -> libc::c_int;
    fn pos_init();
    fn help_ckd_mul(
        r: *mut libc::c_void,
        a: uintmax,
        b: uintmax,
        rsize: libc::c_int,
        rsigned: libc::c_int,
    ) -> libc::c_int;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn cfgetospeed(__termios_p: *const termios) -> speed_t;
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    fn tcsetattr(
        __fd: libc::c_int,
        __optional_actions: libc::c_int,
        __termios_p: *const termios,
    ) -> libc::c_int;
    static mut PC: libc::c_char;
    static mut ospeed: libc::c_short;
    fn tgetstr(_: *const libc::c_char, _: *mut *mut libc::c_char) -> *mut libc::c_char;
    fn tgoto(
        _: *const libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    fn tgetent(_: *mut libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn tgetflag(_: *const libc::c_char) -> libc::c_int;
    fn tgetnum(_: *const libc::c_char) -> libc::c_int;
    fn tputs(
        _: *const libc::c_char,
        _: libc::c_int,
        _: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
    ) -> libc::c_int;
    static mut binattr: libc::c_int;
    static mut one_screen: libc::c_int;
    static mut quiet: libc::c_int;
    static mut no_vbell: libc::c_int;
    static mut no_back_scroll: libc::c_int;
    static mut no_init: libc::c_int;
    static mut no_keypad: libc::c_int;
    static mut top_scroll: libc::c_int;
    static mut quit_if_one_screen: libc::c_int;
    static mut oldbot: libc::c_int;
    static mut mousecap: libc::c_int;
    static mut is_tty: libc::c_int;
    static mut use_color: libc::c_int;
    static mut hilite_search: libc::c_int;
    static mut tty: libc::c_int;
}
pub type __uintmax_t = libc::c_ulong;
pub type uintmax_t = __uintmax_t;
pub type uintmax = uintmax_t;
pub type COLOR_TYPE = libc::c_uint;
pub const CT_6BIT: COLOR_TYPE = 2;
pub const CT_4BIT: COLOR_TYPE = 1;
pub const CT_NULL: COLOR_TYPE = 0;
pub type C2RustUnnamed = libc::c_int;
pub const CV_ERROR: C2RustUnnamed = -1;
pub const CV_NOCHANGE: C2RustUnnamed = -2;
pub const CV_BRIGHT: C2RustUnnamed = 8;
pub const CV_RED: C2RustUnnamed = 4;
pub const CV_GREEN: C2RustUnnamed = 2;
pub const CV_BLUE: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
pub type speed_t = libc::c_uint;
pub type cc_t = libc::c_uchar;
pub type tcflag_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct env {
    pub next: *mut env,
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
static mut sc_pad: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut sc_home: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut sc_addline: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut sc_lower_left: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut sc_return: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut sc_move: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut sc_clear: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut sc_eol_clear: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut sc_eos_clear: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut sc_s_in: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut sc_s_out: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut sc_u_in: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut sc_u_out: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut sc_b_in: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut sc_b_out: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut sc_bl_in: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut sc_bl_out: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut sc_visual_bell: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut sc_backspace: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut sc_s_keypad: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut sc_e_keypad: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut sc_s_mousecap: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut sc_e_mousecap: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut sc_init: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut sc_deinit: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut attrcolor: libc::c_int = -(1 as libc::c_int);
static mut init_done: libc::c_int = 0 as libc::c_int;
pub static mut auto_wrap: libc::c_int = 0;
pub static mut ignaw: libc::c_int = 0;
pub static mut erase_char: libc::c_int = 0;
pub static mut erase2_char: libc::c_int = 0;
pub static mut kill_char: libc::c_int = 0;
pub static mut werase_char: libc::c_int = 0;
pub static mut sc_width: libc::c_int = 0;
pub static mut sc_height: libc::c_int = 0;
pub static mut bo_s_width: libc::c_int = 0;
pub static mut bo_e_width: libc::c_int = 0;
pub static mut ul_s_width: libc::c_int = 0;
pub static mut ul_e_width: libc::c_int = 0;
pub static mut so_s_width: libc::c_int = 0;
pub static mut so_e_width: libc::c_int = 0;
pub static mut bl_s_width: libc::c_int = 0;
pub static mut bl_e_width: libc::c_int = 0;
pub static mut above_mem: libc::c_int = 0;
pub static mut below_mem: libc::c_int = 0;
pub static mut can_goto_line: libc::c_int = 0;
pub static mut clear_bg: libc::c_int = 0;
pub static mut missing_cap: libc::c_int = 0 as libc::c_int;
pub static mut kent: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut term_init_done: libc::c_int = 0 as libc::c_int;
pub static mut full_screen: libc::c_int = 1 as libc::c_int;
static mut attrmode: libc::c_int = 0 as libc::c_int;
static mut termcap_debug: libc::c_int = -(1 as libc::c_int);
static mut no_alt_screen: libc::c_int = 0;
unsafe extern "C" fn set_termio_flags(mut s: *mut termios) {
    (*s).c_lflag
        &= !(0 as libc::c_int | 0o2 as libc::c_int | 0o10 as libc::c_int
            | 0o20 as libc::c_int | 0o40 as libc::c_int | 0o100 as libc::c_int)
            as libc::c_uint;
    (*s).c_oflag
        |= (0 as libc::c_int | 0o14000 as libc::c_int | 0o1 as libc::c_int
            | 0o4 as libc::c_int) as libc::c_uint;
    (*s).c_oflag
        &= !(0 as libc::c_int | 0o10 as libc::c_int | 0o20 as libc::c_int
            | 0o40 as libc::c_int) as libc::c_uint;
}
pub unsafe extern "C" fn raw_mode(mut on: libc::c_int) {
    static mut curr_on: libc::c_int = 0 as libc::c_int;
    if on == curr_on {
        return;
    }
    erase2_char = '\u{8}' as i32;
    let mut s: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    static mut save_term: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    static mut saved_term: libc::c_int = 0 as libc::c_int;
    if on != 0 {
        if tcgetattr(tty, &mut s) < 0 as libc::c_int {
            erase_char = '\u{8}' as i32;
            kill_char = 'U' as i32 & 0o37 as libc::c_int;
            werase_char = 'W' as i32 & 0o37 as libc::c_int;
        } else {
            if saved_term == 0 {
                save_term = s;
                saved_term = 1 as libc::c_int;
            }
            match cfgetospeed(&mut s) {
                0 => {
                    ospeed = 0 as libc::c_int as libc::c_short;
                }
                1 => {
                    ospeed = 1 as libc::c_int as libc::c_short;
                }
                2 => {
                    ospeed = 2 as libc::c_int as libc::c_short;
                }
                3 => {
                    ospeed = 3 as libc::c_int as libc::c_short;
                }
                4 => {
                    ospeed = 4 as libc::c_int as libc::c_short;
                }
                5 => {
                    ospeed = 5 as libc::c_int as libc::c_short;
                }
                6 => {
                    ospeed = 6 as libc::c_int as libc::c_short;
                }
                7 => {
                    ospeed = 7 as libc::c_int as libc::c_short;
                }
                8 => {
                    ospeed = 8 as libc::c_int as libc::c_short;
                }
                9 => {
                    ospeed = 9 as libc::c_int as libc::c_short;
                }
                10 => {
                    ospeed = 10 as libc::c_int as libc::c_short;
                }
                11 => {
                    ospeed = 11 as libc::c_int as libc::c_short;
                }
                12 => {
                    ospeed = 12 as libc::c_int as libc::c_short;
                }
                13 => {
                    ospeed = 13 as libc::c_int as libc::c_short;
                }
                14 => {
                    ospeed = 14 as libc::c_int as libc::c_short;
                }
                15 => {
                    ospeed = 15 as libc::c_int as libc::c_short;
                }
                4097 => {
                    ospeed = 16 as libc::c_int as libc::c_short;
                }
                4098 => {
                    ospeed = 17 as libc::c_int as libc::c_short;
                }
                _ => {}
            }
            erase_char = s.c_cc[2 as libc::c_int as usize] as libc::c_int;
            kill_char = s.c_cc[3 as libc::c_int as usize] as libc::c_int;
            werase_char = s.c_cc[14 as libc::c_int as usize] as libc::c_int;
            set_termio_flags(&mut s);
            s.c_cc[6 as libc::c_int as usize] = 1 as libc::c_int as cc_t;
            s.c_cc[5 as libc::c_int as usize] = 0 as libc::c_int as cc_t;
            s.c_cc[15 as libc::c_int as usize] = 0 as libc::c_int as cc_t;
            s.c_cc[9 as libc::c_int as usize] = 0 as libc::c_int as cc_t;
            s.c_cc[8 as libc::c_int as usize] = 0 as libc::c_int as cc_t;
        }
    } else {
        s = save_term;
    }
    fsync(tty);
    tcsetattr(tty, 1 as libc::c_int, &mut s);
    curr_on = on;
}
static mut hardcopy: libc::c_int = 0;
unsafe extern "C" fn ltget_env(mut capname: *mut libc::c_char) -> *mut libc::c_char {
    let mut name: [libc::c_char; 64] = [0; 64];
    if termcap_debug != 0 {
        static mut envs: *mut env = 0 as *const env as *mut env;
        let mut p: *mut env = 0 as *mut env;
        p = envs;
        while !p.is_null() {
            if strcmp((*p).name, capname) == 0 as libc::c_int {
                return (*p).value;
            }
            p = (*p).next;
        }
        p = ecalloc(
            1 as libc::c_int,
            ::std::mem::size_of::<env>() as libc::c_ulong as libc::c_uint,
        ) as *mut env;
        (*p).name = save(capname);
        (*p)
            .value = ecalloc(
            (strlen(capname)).wrapping_add(3 as libc::c_int as libc::c_ulong)
                as libc::c_int,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_uint,
        ) as *mut libc::c_char;
        sprintf((*p).value, b"<%s>\0" as *const u8 as *const libc::c_char, capname);
        (*p).next = envs;
        envs = p;
        return (*p).value;
    }
    snprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        b"LESS_TERMCAP_%s\0" as *const u8 as *const libc::c_char,
        capname,
    );
    return lgetenv(name.as_mut_ptr());
}
unsafe extern "C" fn ltgetflag(mut capname: *mut libc::c_char) -> libc::c_int {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = ltget_env(capname);
    if !s.is_null() {
        return (*s as libc::c_int != '\0' as i32 && *s as libc::c_int != '0' as i32)
            as libc::c_int;
    }
    if hardcopy != 0 {
        return 0 as libc::c_int;
    }
    return tgetflag(capname);
}
unsafe extern "C" fn ltgetnum(mut capname: *mut libc::c_char) -> libc::c_int {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = ltget_env(capname);
    if !s.is_null() {
        return atoi(s);
    }
    if hardcopy != 0 {
        return -(1 as libc::c_int);
    }
    return tgetnum(capname);
}
unsafe extern "C" fn ltgetstr(
    mut capname: *mut libc::c_char,
    mut pp: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = ltget_env(capname);
    if !s.is_null() {
        return s;
    }
    if hardcopy != 0 {
        return 0 as *mut libc::c_char;
    }
    return tgetstr(capname, pp);
}
pub unsafe extern "C" fn scrsize() {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sys_height: libc::c_int = 0;
    let mut sys_width: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    sys_height = 0 as libc::c_int;
    sys_width = sys_height;
    let mut w: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    if ioctl(
        2 as libc::c_int,
        0x5413 as libc::c_int as libc::c_ulong,
        &mut w as *mut winsize,
    ) == 0 as libc::c_int
    {
        if w.ws_row as libc::c_int > 0 as libc::c_int {
            sys_height = w.ws_row as libc::c_int;
        }
        if w.ws_col as libc::c_int > 0 as libc::c_int {
            sys_width = w.ws_col as libc::c_int;
        }
    }
    if sys_height > 0 as libc::c_int {
        sc_height = sys_height;
    } else {
        s = lgetenv(b"LINES\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        if !s.is_null() {
            sc_height = atoi(s);
        } else {
            n = ltgetnum(
                b"li\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            if n > 0 as libc::c_int {
                sc_height = n;
            }
        }
    }
    s = lgetenv(
        b"LESS_LINES\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !s.is_null() {
        let mut height: libc::c_int = atoi(s);
        sc_height = if height < 0 as libc::c_int { sc_height + height } else { height };
        full_screen = 0 as libc::c_int;
    }
    if sc_height <= 0 as libc::c_int {
        sc_height = 24 as libc::c_int;
    }
    if sys_width > 0 as libc::c_int {
        sc_width = sys_width;
    } else {
        s = lgetenv(
            b"COLUMNS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !s.is_null() {
            sc_width = atoi(s);
        } else {
            n = ltgetnum(
                b"co\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            if n > 0 as libc::c_int {
                sc_width = n;
            }
        }
    }
    s = lgetenv(
        b"LESS_COLUMNS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !s.is_null() {
        let mut width: libc::c_int = atoi(s);
        sc_width = if width < 0 as libc::c_int { sc_width + width } else { width };
    }
    if sc_width <= 0 as libc::c_int {
        sc_width = 80 as libc::c_int;
    }
}
pub unsafe extern "C" fn special_key_str(mut key: libc::c_int) -> *mut libc::c_char {
    static mut tbuf: [libc::c_char; 40] = [0; 40];
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sp: *mut libc::c_char = tbuf.as_mut_ptr();
    match key {
        1 => {
            s = ltgetstr(
                b"kr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                &mut sp,
            );
        }
        2 => {
            s = ltgetstr(
                b"kl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                &mut sp,
            );
        }
        3 => {
            s = ltgetstr(
                b"ku\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                &mut sp,
            );
        }
        4 => {
            s = ltgetstr(
                b"kd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                &mut sp,
            );
        }
        5 => {
            s = ltgetstr(
                b"kP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                &mut sp,
            );
        }
        6 => {
            s = ltgetstr(
                b"kN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                &mut sp,
            );
        }
        7 => {
            s = ltgetstr(
                b"kh\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                &mut sp,
            );
        }
        8 => {
            s = ltgetstr(
                b"@7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                &mut sp,
            );
        }
        9 => {
            s = ltgetstr(
                b"kD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                &mut sp,
            );
            if s.is_null() {
                tbuf[0 as libc::c_int as usize] = '\u{7f}' as i32 as libc::c_char;
                tbuf[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                s = tbuf.as_mut_ptr();
            }
        }
        17 => {
            s = ltgetstr(
                b"kb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                &mut sp,
            );
            if s.is_null() {
                tbuf[0 as libc::c_int as usize] = '\u{8}' as i32 as libc::c_char;
                tbuf[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                s = tbuf.as_mut_ptr();
            }
        }
        40 => {
            tbuf[0 as libc::c_int
                as usize] = ('K' as i32 & 0o37 as libc::c_int) as libc::c_char;
            tbuf[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            s = tbuf.as_mut_ptr();
        }
        _ => return 0 as *mut libc::c_char,
    }
    return s;
}
pub unsafe extern "C" fn get_term() {
    termcap_debug = (isnullenv(
        lgetenv(
            b"LESS_TERMCAP_DEBUG\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ),
    ) == 0) as libc::c_int;
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut term: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut termbuf: [libc::c_char; 2048] = [0; 2048];
    static mut sbuf: [libc::c_char; 1024] = [0; 1024];
    term = lgetenv(b"TERM\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if term.is_null() {
        term = b"unknown\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    hardcopy = 0 as libc::c_int;
    if tgetent(termbuf.as_mut_ptr(), term) != 1 as libc::c_int {
        hardcopy = 1 as libc::c_int;
    }
    if ltgetflag(b"hc\0" as *const u8 as *const libc::c_char as *mut libc::c_char) != 0 {
        hardcopy = 1 as libc::c_int;
    }
    scrsize();
    pos_init();
    auto_wrap = ltgetflag(
        b"am\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    ignaw = ltgetflag(b"xn\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    above_mem = ltgetflag(
        b"da\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    below_mem = ltgetflag(
        b"db\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    clear_bg = ltgetflag(
        b"ut\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    no_alt_screen = ltgetflag(
        b"NR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    so_s_width = ltgetnum(
        b"sg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if so_s_width < 0 as libc::c_int {
        so_s_width = 0 as libc::c_int;
    }
    so_e_width = so_s_width;
    bo_e_width = so_s_width;
    bo_s_width = bo_e_width;
    ul_e_width = so_s_width;
    ul_s_width = ul_e_width;
    bl_e_width = so_s_width;
    bl_s_width = bl_e_width;
    if so_s_width > 0 as libc::c_int || so_e_width > 0 as libc::c_int {
        hilite_search = 0 as libc::c_int;
    }
    sp = sbuf.as_mut_ptr();
    sc_pad = ltgetstr(
        b"pc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut sp,
    );
    if !sc_pad.is_null() {
        PC = *sc_pad;
    }
    sc_s_keypad = ltgetstr(
        b"ks\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut sp,
    );
    if sc_s_keypad.is_null() {
        sc_s_keypad = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    sc_e_keypad = ltgetstr(
        b"ke\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut sp,
    );
    if sc_e_keypad.is_null() {
        sc_e_keypad = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    kent = ltgetstr(
        b"@8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut sp,
    );
    sc_s_mousecap = ltgetstr(
        b"MOUSE_START\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut sp,
    );
    if sc_s_mousecap.is_null() {
        sc_s_mousecap = b"\x1B[?1000h\x1B[?1006h\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    sc_e_mousecap = ltgetstr(
        b"MOUSE_END\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut sp,
    );
    if sc_e_mousecap.is_null() {
        sc_e_mousecap = b"\x1B[?1006l\x1B[?1000l\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    sc_init = ltgetstr(
        b"ti\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut sp,
    );
    if sc_init.is_null() {
        sc_init = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    sc_deinit = ltgetstr(
        b"te\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut sp,
    );
    if sc_deinit.is_null() {
        sc_deinit = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    sc_eol_clear = ltgetstr(
        b"ce\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut sp,
    );
    if sc_eol_clear.is_null() || *sc_eol_clear as libc::c_int == '\0' as i32 {
        missing_cap = 1 as libc::c_int;
        sc_eol_clear = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    sc_eos_clear = ltgetstr(
        b"cd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut sp,
    );
    if below_mem != 0
        && (sc_eos_clear.is_null() || *sc_eos_clear as libc::c_int == '\0' as i32)
    {
        missing_cap = 1 as libc::c_int;
        sc_eos_clear = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    sc_clear = ltgetstr(
        b"cl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut sp,
    );
    if sc_clear.is_null() || *sc_clear as libc::c_int == '\0' as i32 {
        missing_cap = 1 as libc::c_int;
        sc_clear = b"\n\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    sc_move = ltgetstr(
        b"cm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut sp,
    );
    if sc_move.is_null() || *sc_move as libc::c_int == '\0' as i32 {
        sc_move = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        can_goto_line = 0 as libc::c_int;
    } else {
        can_goto_line = 1 as libc::c_int;
    }
    tmodes(
        b"so\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"se\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut sc_s_in,
        &mut sc_s_out,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut sp,
    );
    tmodes(
        b"us\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"ue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut sc_u_in,
        &mut sc_u_out,
        sc_s_in,
        sc_s_out,
        &mut sp,
    );
    tmodes(
        b"md\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"me\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut sc_b_in,
        &mut sc_b_out,
        sc_s_in,
        sc_s_out,
        &mut sp,
    );
    tmodes(
        b"mb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"me\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut sc_bl_in,
        &mut sc_bl_out,
        sc_s_in,
        sc_s_out,
        &mut sp,
    );
    sc_visual_bell = ltgetstr(
        b"vb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut sp,
    );
    if sc_visual_bell.is_null() {
        sc_visual_bell = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if ltgetflag(b"bs\0" as *const u8 as *const libc::c_char as *mut libc::c_char) != 0 {
        sc_backspace = b"\x08\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    } else {
        sc_backspace = ltgetstr(
            b"bc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut sp,
        );
        if sc_backspace.is_null() || *sc_backspace as libc::c_int == '\0' as i32 {
            sc_backspace = b"\x08\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
    }
    t1 = ltgetstr(
        b"ho\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut sp,
    );
    if t1.is_null() {
        t1 = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if *sc_move as libc::c_int == '\0' as i32 {
        t2 = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else {
        strcpy(sp, tgoto(sc_move, 0 as libc::c_int, 0 as libc::c_int));
        t2 = sp;
        sp = sp
            .offset(
                (strlen(sp)).wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            );
    }
    sc_home = cheaper(
        t1,
        t2,
        b"|\x08^\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    t1 = ltgetstr(
        b"ll\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut sp,
    );
    if t1.is_null() || full_screen == 0 {
        t1 = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if *sc_move as libc::c_int == '\0' as i32 {
        t2 = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else {
        strcpy(sp, tgoto(sc_move, 0 as libc::c_int, sc_height - 1 as libc::c_int));
        t2 = sp;
        sp = sp
            .offset(
                (strlen(sp)).wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            );
    }
    sc_lower_left = cheaper(
        t1,
        t2,
        b"\r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    sc_return = ltgetstr(
        b"cr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut sp,
    );
    if sc_return.is_null() {
        sc_return = b"\r\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    t1 = ltgetstr(
        b"al\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut sp,
    );
    if t1.is_null() {
        t1 = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    t2 = ltgetstr(
        b"sr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut sp,
    );
    if t2.is_null() {
        t2 = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if above_mem != 0 {
        sc_addline = t1;
    } else {
        sc_addline = cheaper(
            t1,
            t2,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if *sc_addline as libc::c_int == '\0' as i32 {
        no_back_scroll = 1 as libc::c_int;
    }
}
static mut costcount: libc::c_int = 0;
unsafe extern "C" fn inc_costcount(mut c: libc::c_int) -> libc::c_int {
    costcount += 1;
    costcount;
    return c;
}
unsafe extern "C" fn cost(mut t: *mut libc::c_char) -> libc::c_int {
    costcount = 0 as libc::c_int;
    tputs(
        t,
        sc_height,
        Some(inc_costcount as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
    );
    return costcount;
}
unsafe extern "C" fn cheaper(
    mut t1: *mut libc::c_char,
    mut t2: *mut libc::c_char,
    mut def: *mut libc::c_char,
) -> *mut libc::c_char {
    if *t1 as libc::c_int == '\0' as i32 && *t2 as libc::c_int == '\0' as i32 {
        missing_cap = 1 as libc::c_int;
        return def;
    }
    if *t1 as libc::c_int == '\0' as i32 {
        return t2;
    }
    if *t2 as libc::c_int == '\0' as i32 {
        return t1;
    }
    if cost(t1) < cost(t2) {
        return t1;
    }
    return t2;
}
unsafe extern "C" fn tmodes(
    mut incap: *mut libc::c_char,
    mut outcap: *mut libc::c_char,
    mut instr: *mut *mut libc::c_char,
    mut outstr: *mut *mut libc::c_char,
    mut def_instr: *mut libc::c_char,
    mut def_outstr: *mut libc::c_char,
    mut spp: *mut *mut libc::c_char,
) {
    *instr = ltgetstr(incap, spp);
    if (*instr).is_null() {
        *instr = def_instr;
        *outstr = def_outstr;
        return;
    }
    *outstr = ltgetstr(outcap, spp);
    if (*outstr).is_null() {
        *outstr = ltgetstr(
            b"me\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            spp,
        );
    }
    if (*outstr).is_null() {
        *outstr = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
}
unsafe extern "C" fn do_tputs(
    mut str: *mut libc::c_char,
    mut affcnt: libc::c_int,
    mut f_putc: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
) {
    tputs(str, affcnt, f_putc);
}
unsafe extern "C" fn ltputs(
    mut str: *mut libc::c_char,
    mut affcnt: libc::c_int,
    mut f_putc: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
) {
    while !str.is_null() && *str as libc::c_int != '\0' as i32 {
        let mut obrac: *mut libc::c_char = strstr(
            str,
            b"$<\0" as *const u8 as *const libc::c_char,
        );
        if !obrac.is_null() {
            let mut str2: [libc::c_char; 64] = [0; 64];
            let mut slen: libc::c_int = obrac.offset_from(str) as libc::c_long
                as libc::c_int;
            if (slen as libc::c_ulong)
                < ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
            {
                let mut delay: libc::c_int = 0;
                memcpy(
                    str2.as_mut_ptr() as *mut libc::c_void,
                    str as *const libc::c_void,
                    slen as libc::c_ulong,
                );
                str2[slen as usize] = '\0' as i32 as libc::c_char;
                do_tputs(str2.as_mut_ptr(), affcnt, f_putc);
                str = str.offset((slen + 2 as libc::c_int) as isize);
                delay = lstrtoi(str, &mut str, 10 as libc::c_int);
                if *str as libc::c_int == '*' as i32 {
                    if help_ckd_mul(
                        &mut delay as *mut libc::c_int as *mut libc::c_void,
                        delay as uintmax,
                        affcnt as uintmax,
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                            as libc::c_int,
                        (((if 1 as libc::c_int != 0 { 0 as libc::c_int } else { delay })
                            - 1 as libc::c_int) < 0 as libc::c_int) as libc::c_int,
                    ) != 0
                    {
                        delay = 2147483647 as libc::c_int;
                    }
                }
                flush();
                sleep_ms(delay);
                str = strstr(str, b">\0" as *const u8 as *const libc::c_char);
                if !str.is_null() {
                    str = str.offset(1);
                    str;
                }
                continue;
            }
        }
        do_tputs(str, affcnt, f_putc);
        break;
    }
}
pub unsafe extern "C" fn init_mouse() {
    ltputs(
        sc_s_mousecap,
        sc_height,
        Some(putchr as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
    );
}
pub unsafe extern "C" fn deinit_mouse() {
    ltputs(
        sc_e_mousecap,
        sc_height,
        Some(putchr as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
    );
}
pub unsafe extern "C" fn init() {
    clear_bot_if_needed();
    if !(quit_if_one_screen != 0 && one_screen != 0) {
        if no_init == 0 {
            ltputs(
                sc_init,
                sc_height,
                Some(putchr as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
            );
            if *sc_init as libc::c_int != '\0' as i32
                && *sc_deinit as libc::c_int != '\0' as i32 && no_alt_screen == 0
            {
                lower_left();
            }
            term_init_done = 1 as libc::c_int;
        }
        if no_keypad == 0 {
            ltputs(
                sc_s_keypad,
                sc_height,
                Some(putchr as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
            );
        }
        if mousecap != 0 {
            init_mouse();
        }
    }
    init_done = 1 as libc::c_int;
    if top_scroll != 0 {
        let mut i: libc::c_int = 0;
        i = 1 as libc::c_int;
        while i < sc_height {
            putchr('\n' as i32);
            i += 1;
            i;
        }
    } else {
        line_left();
    };
}
pub unsafe extern "C" fn deinit() {
    if init_done == 0 {
        return;
    }
    if !(quit_if_one_screen != 0 && one_screen != 0) {
        if mousecap != 0 {
            deinit_mouse();
        }
        if no_keypad == 0 {
            ltputs(
                sc_e_keypad,
                sc_height,
                Some(putchr as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
            );
        }
        if no_init == 0 {
            ltputs(
                sc_deinit,
                sc_height,
                Some(putchr as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
            );
        }
    }
    init_done = 0 as libc::c_int;
}
pub unsafe extern "C" fn interactive() -> libc::c_int {
    return (is_tty != 0 && init_done != 0) as libc::c_int;
}
unsafe extern "C" fn assert_interactive() {
    if interactive() != 0 {
        return;
    }
}
pub unsafe extern "C" fn home() {
    assert_interactive();
    ltputs(
        sc_home,
        1 as libc::c_int,
        Some(putchr as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
    );
}
pub unsafe extern "C" fn add_line() {
    assert_interactive();
    ltputs(
        sc_addline,
        sc_height,
        Some(putchr as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
    );
}
pub unsafe extern "C" fn lower_left() {
    assert_interactive();
    ltputs(
        sc_lower_left,
        1 as libc::c_int,
        Some(putchr as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
    );
}
pub unsafe extern "C" fn line_left() {
    assert_interactive();
    ltputs(
        sc_return,
        1 as libc::c_int,
        Some(putchr as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
    );
}
pub unsafe extern "C" fn check_winch() {}
pub unsafe extern "C" fn goto_line(mut sindex: libc::c_int) {
    assert_interactive();
    ltputs(
        tgoto(sc_move, 0 as libc::c_int, sindex),
        1 as libc::c_int,
        Some(putchr as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
    );
}
pub unsafe extern "C" fn vbell() {
    if no_vbell != 0 {
        return;
    }
    if *sc_visual_bell as libc::c_int == '\0' as i32 {
        return;
    }
    ltputs(
        sc_visual_bell,
        sc_height,
        Some(putchr as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
    );
}
unsafe extern "C" fn beep() {
    putchr('G' as i32 & 0o37 as libc::c_int);
}
pub unsafe extern "C" fn bell() {
    if quiet == 2 as libc::c_int {
        vbell();
    } else {
        beep();
    };
}
pub unsafe extern "C" fn clear() {
    assert_interactive();
    ltputs(
        sc_clear,
        sc_height,
        Some(putchr as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
    );
}
pub unsafe extern "C" fn clear_eol() {
    ltputs(
        sc_eol_clear,
        1 as libc::c_int,
        Some(putchr as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
    );
}
unsafe extern "C" fn clear_eol_bot() {
    assert_interactive();
    if below_mem != 0 {
        ltputs(
            sc_eos_clear,
            1 as libc::c_int,
            Some(putchr as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
        );
    } else {
        ltputs(
            sc_eol_clear,
            1 as libc::c_int,
            Some(putchr as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
        );
    };
}
pub unsafe extern "C" fn clear_bot() {
    if oldbot != 0 {
        lower_left();
    } else {
        line_left();
    }
    if attrmode == 0 as libc::c_int {
        clear_eol_bot();
    } else {
        let mut saved_attrmode: libc::c_int = attrmode;
        at_exit();
        clear_eol_bot();
        at_enter(saved_attrmode);
    };
}
unsafe extern "C" fn parse_color4(mut ch: libc::c_char) -> libc::c_int {
    match ch as libc::c_int {
        107 => return 0 as libc::c_int,
        114 => return CV_RED as libc::c_int,
        103 => return CV_GREEN as libc::c_int,
        121 => return CV_RED as libc::c_int | CV_GREEN as libc::c_int,
        98 => return CV_BLUE as libc::c_int,
        109 => return CV_RED as libc::c_int | CV_BLUE as libc::c_int,
        99 => return CV_GREEN as libc::c_int | CV_BLUE as libc::c_int,
        119 => {
            return CV_RED as libc::c_int | CV_GREEN as libc::c_int
                | CV_BLUE as libc::c_int;
        }
        75 => return 0 as libc::c_int | CV_BRIGHT as libc::c_int,
        82 => return CV_RED as libc::c_int | CV_BRIGHT as libc::c_int,
        71 => return CV_GREEN as libc::c_int | CV_BRIGHT as libc::c_int,
        89 => {
            return CV_RED as libc::c_int | CV_GREEN as libc::c_int
                | CV_BRIGHT as libc::c_int;
        }
        66 => return CV_BLUE as libc::c_int | CV_BRIGHT as libc::c_int,
        77 => {
            return CV_RED as libc::c_int | CV_BLUE as libc::c_int
                | CV_BRIGHT as libc::c_int;
        }
        67 => {
            return CV_GREEN as libc::c_int | CV_BLUE as libc::c_int
                | CV_BRIGHT as libc::c_int;
        }
        87 => {
            return CV_RED as libc::c_int | CV_GREEN as libc::c_int
                | CV_BLUE as libc::c_int | CV_BRIGHT as libc::c_int;
        }
        45 => return CV_NOCHANGE as libc::c_int,
        _ => return CV_ERROR as libc::c_int,
    };
}
unsafe extern "C" fn parse_color6(mut ps: *mut *mut libc::c_char) -> libc::c_int {
    if **ps as libc::c_int == '-' as i32 {
        *ps = (*ps).offset(1);
        *ps;
        return CV_NOCHANGE as libc::c_int;
    } else {
        let mut ops: *mut libc::c_char = *ps;
        let mut color: libc::c_int = lstrtoi(ops, ps, 10 as libc::c_int);
        if color < 0 as libc::c_int || *ps == ops {
            return CV_ERROR as libc::c_int;
        }
        return color;
    };
}
pub unsafe extern "C" fn parse_color(
    mut str: *mut libc::c_char,
    mut p_fg: *mut libc::c_int,
    mut p_bg: *mut libc::c_int,
) -> COLOR_TYPE {
    let mut fg: libc::c_int = 0;
    let mut bg: libc::c_int = 0;
    let mut type_0: COLOR_TYPE = CT_NULL;
    if str.is_null() || *str as libc::c_int == '\0' as i32 {
        return CT_NULL;
    }
    if *str as libc::c_int == '+' as i32 {
        str = str.offset(1);
        str;
    }
    fg = parse_color4(*str.offset(0 as libc::c_int as isize));
    bg = parse_color4(
        (if strlen(str) < 2 as libc::c_int as libc::c_ulong {
            '-' as i32
        } else {
            *str.offset(1 as libc::c_int as isize) as libc::c_int
        }) as libc::c_char,
    );
    if fg != CV_ERROR as libc::c_int && bg != CV_ERROR as libc::c_int {
        type_0 = CT_4BIT;
    } else {
        fg = parse_color6(&mut str);
        bg = if fg != CV_ERROR as libc::c_int
            && {
                let fresh0 = str;
                str = str.offset(1);
                *fresh0 as libc::c_int == '.' as i32
            }
        {
            parse_color6(&mut str)
        } else {
            CV_NOCHANGE as libc::c_int
        };
        if fg != CV_ERROR as libc::c_int && bg != CV_ERROR as libc::c_int {
            type_0 = CT_6BIT;
        }
    }
    if !p_fg.is_null() {
        *p_fg = fg;
    }
    if !p_bg.is_null() {
        *p_bg = bg;
    }
    return type_0;
}
unsafe extern "C" fn sgr_color(mut color: libc::c_int) -> libc::c_int {
    match color {
        0 => return 30 as libc::c_int,
        4 => return 31 as libc::c_int,
        2 => return 32 as libc::c_int,
        6 => return 33 as libc::c_int,
        1 => return 34 as libc::c_int,
        5 => return 35 as libc::c_int,
        3 => return 36 as libc::c_int,
        7 => return 37 as libc::c_int,
        8 => return 90 as libc::c_int,
        12 => return 91 as libc::c_int,
        10 => return 92 as libc::c_int,
        14 => return 93 as libc::c_int,
        9 => return 94 as libc::c_int,
        13 => return 95 as libc::c_int,
        11 => return 96 as libc::c_int,
        15 => return 97 as libc::c_int,
        _ => return color,
    };
}
unsafe extern "C" fn tput_fmt(
    mut fmt: *mut libc::c_char,
    mut color: libc::c_int,
    mut f_putc: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
) {
    let mut buf: [libc::c_char; 27] = [0; 27];
    if color == attrcolor {
        return;
    }
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong,
        fmt,
        color,
    );
    ltputs(buf.as_mut_ptr(), 1 as libc::c_int, f_putc);
    attrcolor = color;
}
unsafe extern "C" fn tput_color(
    mut str: *mut libc::c_char,
    mut f_putc: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
) {
    let mut fg: libc::c_int = 0;
    let mut bg: libc::c_int = 0;
    if !str.is_null()
        && strcmp(str, b"*\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        tput_fmt(
            b"\x1B[m\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            -(1 as libc::c_int),
            f_putc,
        );
        return;
    }
    match parse_color(str, &mut fg, &mut bg) as libc::c_uint {
        1 => {
            if fg >= 0 as libc::c_int {
                tput_fmt(
                    b"\x1B[%dm\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    sgr_color(fg),
                    f_putc,
                );
            }
            if bg >= 0 as libc::c_int {
                tput_fmt(
                    b"\x1B[%dm\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    sgr_color(bg) + 10 as libc::c_int,
                    f_putc,
                );
            }
        }
        2 => {
            if fg >= 0 as libc::c_int {
                tput_fmt(
                    b"\x1B[38;5;%dm\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    fg,
                    f_putc,
                );
            }
            if bg >= 0 as libc::c_int {
                tput_fmt(
                    b"\x1B[48;5;%dm\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    bg,
                    f_putc,
                );
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn tput_inmode(
    mut mode_str: *mut libc::c_char,
    mut attr: libc::c_int,
    mut attr_bit: libc::c_int,
    mut f_putc: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
) {
    let mut color_str: *mut libc::c_char = 0 as *mut libc::c_char;
    if attr & attr_bit == 0 as libc::c_int {
        return;
    }
    color_str = get_color_map(attr_bit);
    if color_str.is_null() || *color_str as libc::c_int == '\0' as i32
        || *color_str as libc::c_int == '+' as i32
    {
        ltputs(mode_str, 1 as libc::c_int, f_putc);
        if color_str.is_null()
            || {
                let fresh1 = color_str;
                color_str = color_str.offset(1);
                *fresh1 as libc::c_int != '+' as i32
            }
        {
            return;
        }
    }
    tput_color(color_str, f_putc);
}
unsafe extern "C" fn tput_outmode(
    mut mode_str: *mut libc::c_char,
    mut attr_bit: libc::c_int,
    mut f_putc: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
) {
    if attrmode & attr_bit == 0 as libc::c_int {
        return;
    }
    ltputs(mode_str, 1 as libc::c_int, f_putc);
}
pub unsafe extern "C" fn at_enter(mut attr: libc::c_int) {
    attr = apply_at_specials(attr);
    tput_inmode(
        sc_u_in,
        attr,
        (1 as libc::c_int) << 0 as libc::c_int,
        Some(putchr as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
    );
    tput_inmode(
        sc_b_in,
        attr,
        (1 as libc::c_int) << 1 as libc::c_int,
        Some(putchr as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
    );
    tput_inmode(
        sc_bl_in,
        attr,
        (1 as libc::c_int) << 2 as libc::c_int,
        Some(putchr as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
    );
    if use_color != 0
        && attr & (16 as libc::c_int - 1 as libc::c_int) << 8 as libc::c_int != 0
    {
        tput_color(
            get_color_map(attr),
            Some(putchr as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
        );
    } else {
        tput_inmode(
            sc_s_in,
            attr,
            (1 as libc::c_int) << 3 as libc::c_int,
            Some(putchr as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
        );
    }
    attrmode = attr;
}
pub unsafe extern "C" fn at_exit() {
    tput_color(
        b"*\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(putchr as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
    );
    tput_outmode(
        sc_s_out,
        (1 as libc::c_int) << 3 as libc::c_int,
        Some(putchr as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
    );
    tput_outmode(
        sc_bl_out,
        (1 as libc::c_int) << 2 as libc::c_int,
        Some(putchr as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
    );
    tput_outmode(
        sc_b_out,
        (1 as libc::c_int) << 1 as libc::c_int,
        Some(putchr as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
    );
    tput_outmode(
        sc_u_out,
        (1 as libc::c_int) << 0 as libc::c_int,
        Some(putchr as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
    );
    attrmode = 0 as libc::c_int;
}
pub unsafe extern "C" fn at_switch(mut attr: libc::c_int) {
    let mut new_attrmode: libc::c_int = apply_at_specials(attr);
    let mut ignore_modes: libc::c_int = (1 as libc::c_int) << 4 as libc::c_int;
    if new_attrmode & !ignore_modes != attrmode & !ignore_modes {
        at_exit();
        at_enter(attr);
    }
}
pub unsafe extern "C" fn is_at_equiv(
    mut attr1: libc::c_int,
    mut attr2: libc::c_int,
) -> libc::c_int {
    attr1 = apply_at_specials(attr1);
    attr2 = apply_at_specials(attr2);
    return (attr1 == attr2) as libc::c_int;
}
pub unsafe extern "C" fn apply_at_specials(mut attr: libc::c_int) -> libc::c_int {
    if attr & (1 as libc::c_int) << 5 as libc::c_int != 0 {
        attr |= binattr;
    }
    if attr & (1 as libc::c_int) << 6 as libc::c_int != 0 {
        attr |= (1 as libc::c_int) << 3 as libc::c_int;
    }
    attr
        &= !((1 as libc::c_int) << 5 as libc::c_int
            | (1 as libc::c_int) << 6 as libc::c_int);
    return attr;
}
pub unsafe extern "C" fn putbs() {
    if termcap_debug != 0 {
        putstr(b"<bs>\0" as *const u8 as *const libc::c_char);
    } else {
        ltputs(
            sc_backspace,
            1 as libc::c_int,
            Some(putchr as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
        );
    };
}
