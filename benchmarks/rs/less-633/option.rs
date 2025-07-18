use ::libc;
extern "C" {
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn save(s: *const libc::c_char) -> *mut libc::c_char;
    fn ecalloc(count: libc::c_int, size: libc::c_uint) -> *mut libc::c_void;
    fn skipsp(s: *mut libc::c_char) -> *mut libc::c_char;
    fn ch_getflags() -> libc::c_int;
    fn prchar(c: LWCHAR) -> *mut libc::c_char;
    fn ungetcc_back(c: LWCHAR);
    fn ungetsc(s: *mut libc::c_char);
    fn lstrtoi(
        _: *mut libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int;
    fn error(fmt: *mut libc::c_char, parg: *mut PARG);
    fn findopt_name(
        p_optname: *mut *mut libc::c_char,
        p_oname: *mut *mut libc::c_char,
        p_err: *mut libc::c_int,
    ) -> *mut loption;
    fn findopt(c: libc::c_int) -> *mut loption;
    fn chg_hilite();
    fn repaint_hilite(on: libc::c_int);
    static mut screen_trashed: libc::c_int;
    static mut less_is_more: libc::c_int;
    static mut quit_at_eof: libc::c_int;
    static mut every_first_cmd: *mut libc::c_char;
    static mut opt_use_backslash: libc::c_int;
}
pub type __off_t = libc::c_long;
pub type off_t = __off_t;
pub type LWCHAR = libc::c_ulong;
pub type LINENUM = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union parg {
    pub p_string: *mut libc::c_char,
    pub p_int: libc::c_int,
    pub p_linenum: LINENUM,
    pub p_char: libc::c_char,
}
pub type PARG = parg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loption {
    pub oletter: libc::c_char,
    pub onames: *mut optname,
    pub otype: libc::c_int,
    pub odefault: libc::c_int,
    pub ovar: *mut libc::c_int,
    pub ofunc: Option::<unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> ()>,
    pub odesc: [*mut libc::c_char; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct optname {
    pub oname: *mut libc::c_char,
    pub onext: *mut optname,
}
static mut pendopt: *mut loption = 0 as *const loption as *mut loption;
pub static mut plusoption: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn opt_desc(mut o: *mut loption) -> *mut libc::c_char {
    static mut buf: [libc::c_char; 42] = [0; 42];
    if (*o).oletter as libc::c_int == '\u{1}' as i32 {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 42]>() as libc::c_ulong,
            b"--%s\0" as *const u8 as *const libc::c_char,
            (*(*o).onames).oname,
        );
    } else {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 42]>() as libc::c_ulong,
            b"-%c (--%s)\0" as *const u8 as *const libc::c_char,
            (*o).oletter as libc::c_int,
            (*(*o).onames).oname,
        );
    }
    return buf.as_mut_ptr();
}
pub unsafe extern "C" fn propt(mut c: libc::c_int) -> *mut libc::c_char {
    static mut buf: [libc::c_char; 33] = [0; 33];
    sprintf(
        buf.as_mut_ptr(),
        b"-%s\0" as *const u8 as *const libc::c_char,
        prchar(c as LWCHAR),
    );
    return buf.as_mut_ptr();
}
pub unsafe extern "C" fn scan_option(mut s: *mut libc::c_char) {
    let mut o: *mut loption = 0 as *mut loption;
    let mut optc: libc::c_int = 0;
    let mut optname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut printopt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut set_default: libc::c_int = 0;
    let mut lc: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut parg: PARG = parg {
        p_string: 0 as *mut libc::c_char,
    };
    if s.is_null() {
        return;
    }
    if !pendopt.is_null() {
        match (*pendopt).otype
            & (0o1 as libc::c_int | 0o2 as libc::c_int | 0o4 as libc::c_int
                | 0o10 as libc::c_int | 0o20 as libc::c_int)
        {
            8 => {
                (Some(((*pendopt).ofunc).unwrap())).unwrap()(0 as libc::c_int, s);
            }
            4 => {
                printopt = opt_desc(pendopt);
                *(*pendopt)
                    .ovar = getnum(
                    &mut s,
                    printopt,
                    0 as *mut libc::c_void as *mut libc::c_int,
                );
            }
            _ => {}
        }
        pendopt = 0 as *mut loption;
        return;
    }
    set_default = 0 as libc::c_int;
    optname = 0 as *mut libc::c_char;
    while *s as libc::c_int != '\0' as i32 {
        let fresh0 = s;
        s = s.offset(1);
        optc = *fresh0 as libc::c_int;
        match optc {
            32 | 9 | 36 => {
                continue;
            }
            45 => {
                if *s as libc::c_int == '-' as i32 {
                    s = s.offset(1);
                    optname = s;
                } else {
                    set_default = (*s as libc::c_int == '+' as i32) as libc::c_int;
                    if set_default != 0 {
                        s = s.offset(1);
                        s;
                    }
                    continue;
                }
            }
            43 => {
                plusoption = 1 as libc::c_int;
                s = optstring(s, &mut str, propt('+' as i32), 0 as *mut libc::c_char);
                if s.is_null() {
                    return;
                }
                if *str as libc::c_int == '+' as i32 {
                    if !every_first_cmd.is_null() {
                        free(every_first_cmd as *mut libc::c_void);
                    }
                    every_first_cmd = save(str.offset(1 as libc::c_int as isize));
                } else {
                    ungetsc(str);
                    ungetcc_back(0x40000000 as libc::c_int as LWCHAR);
                }
                free(str as *mut libc::c_void);
                continue;
            }
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                s = s.offset(-1);
                s;
                optc = 'z' as i32;
            }
            110 => {
                if less_is_more != 0 {
                    optc = 'z' as i32;
                }
            }
            _ => {}
        }
        err = 0 as libc::c_int;
        if optname.is_null() {
            printopt = propt(optc);
            lc = (optc >= 'a' as i32 && optc <= 'z' as i32) as libc::c_int;
            o = findopt(optc);
        } else {
            printopt = optname;
            lc = (*optname.offset(0 as libc::c_int as isize) as libc::c_int >= 'a' as i32
                && *optname.offset(0 as libc::c_int as isize) as libc::c_int
                    <= 'z' as i32) as libc::c_int;
            o = findopt_name(&mut optname, 0 as *mut *mut libc::c_char, &mut err);
            s = optname;
            optname = 0 as *mut libc::c_char;
            if !(*s as libc::c_int == '\0' as i32 || *s as libc::c_int == ' ' as i32) {
                if *s as libc::c_int == '=' as i32 {
                    if !o.is_null()
                        && (*o).otype
                            & (0o1 as libc::c_int | 0o2 as libc::c_int
                                | 0o4 as libc::c_int | 0o10 as libc::c_int
                                | 0o20 as libc::c_int) != 0o10 as libc::c_int
                        && (*o).otype
                            & (0o1 as libc::c_int | 0o2 as libc::c_int
                                | 0o4 as libc::c_int | 0o10 as libc::c_int
                                | 0o20 as libc::c_int) != 0o4 as libc::c_int
                    {
                        parg.p_string = printopt;
                        error(
                            b"The %s option should not be followed by =\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            &mut parg,
                        );
                        return;
                    }
                    s = s.offset(1);
                    s;
                } else {
                    o = 0 as *mut loption;
                }
            }
        }
        if o.is_null() {
            parg.p_string = printopt;
            if err == 1 as libc::c_int {
                error(
                    b"%s is an ambiguous abbreviation (\"less --help\" for help)\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                    &mut parg,
                );
            } else {
                error(
                    b"There is no %s option (\"less --help\" for help)\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    &mut parg,
                );
            }
            return;
        }
        str = 0 as *mut libc::c_char;
        match (*o).otype
            & (0o1 as libc::c_int | 0o2 as libc::c_int | 0o4 as libc::c_int
                | 0o10 as libc::c_int | 0o20 as libc::c_int)
        {
            1 => {
                if set_default != 0 {
                    *(*o).ovar = (*o).odefault;
                } else {
                    *(*o).ovar = ((*o).odefault == 0) as libc::c_int;
                }
            }
            2 => {
                if set_default != 0 {
                    *(*o).ovar = (*o).odefault;
                } else {
                    *(*o).ovar = flip_triple((*o).odefault, lc);
                }
            }
            8 => {
                if *s as libc::c_int == '\0' as i32 {
                    pendopt = o;
                    return;
                }
                while *s as libc::c_int == ' ' as i32 {
                    s = s.offset(1);
                    s;
                }
                s = optstring(
                    s,
                    &mut str,
                    printopt,
                    (*o).odesc[1 as libc::c_int as usize],
                );
                if s.is_null() {
                    return;
                }
            }
            4 => {
                if *s as libc::c_int == '\0' as i32 {
                    pendopt = o;
                    return;
                }
                *(*o)
                    .ovar = getnum(
                    &mut s,
                    printopt,
                    0 as *mut libc::c_void as *mut libc::c_int,
                );
            }
            _ => {}
        }
        if ((*o).ofunc).is_some() {
            (Some(((*o).ofunc).unwrap())).unwrap()(0 as libc::c_int, str);
        }
        if !str.is_null() {
            free(str as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn toggle_option(
    mut o: *mut loption,
    mut lower: libc::c_int,
    mut s: *mut libc::c_char,
    mut how_toggle: libc::c_int,
) {
    let mut num: libc::c_int = 0;
    let mut no_prompt: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut parg: PARG = parg {
        p_string: 0 as *mut libc::c_char,
    };
    no_prompt = how_toggle & 0o100 as libc::c_int;
    how_toggle &= !(0o100 as libc::c_int);
    if o.is_null() {
        error(
            b"No such option\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_void as *mut PARG,
        );
        return;
    }
    if how_toggle == 1 as libc::c_int && (*o).otype & 0o100 as libc::c_int != 0 {
        parg.p_string = opt_desc(o);
        error(
            b"Cannot change the %s option\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            &mut parg,
        );
        return;
    }
    if how_toggle == 0 as libc::c_int && (*o).otype & 0o400 as libc::c_int != 0 {
        parg.p_string = opt_desc(o);
        error(
            b"Cannot query the %s option\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            &mut parg,
        );
        return;
    }
    match (*o).otype
        & (0o1 as libc::c_int | 0o2 as libc::c_int | 0o4 as libc::c_int
            | 0o10 as libc::c_int | 0o20 as libc::c_int)
    {
        8 | 4 => {
            if how_toggle == 1 as libc::c_int && *s as libc::c_int == '\0' as i32 {
                how_toggle = 0 as libc::c_int;
            }
        }
        _ => {}
    }
    if how_toggle != 0 as libc::c_int && (*o).otype & 0o200 as libc::c_int != 0 {
        repaint_hilite(0 as libc::c_int);
    }
    if how_toggle != 0 as libc::c_int {
        match (*o).otype
            & (0o1 as libc::c_int | 0o2 as libc::c_int | 0o4 as libc::c_int
                | 0o10 as libc::c_int | 0o20 as libc::c_int)
        {
            1 => {
                match how_toggle {
                    1 => {
                        *(*o).ovar = (*(*o).ovar == 0) as libc::c_int;
                    }
                    2 => {
                        *(*o).ovar = (*o).odefault;
                    }
                    3 => {
                        *(*o).ovar = ((*o).odefault == 0) as libc::c_int;
                    }
                    _ => {}
                }
            }
            2 => {
                match how_toggle {
                    1 => {
                        *(*o).ovar = flip_triple(*(*o).ovar, lower);
                    }
                    2 => {
                        *(*o).ovar = (*o).odefault;
                    }
                    3 => {
                        *(*o).ovar = flip_triple((*o).odefault, lower);
                    }
                    _ => {}
                }
            }
            8 => {
                match how_toggle {
                    3 | 2 => {
                        error(
                            b"Cannot use \"-+\" or \"--\" for a string option\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                            0 as *mut libc::c_void as *mut PARG,
                        );
                        return;
                    }
                    _ => {}
                }
            }
            4 => {
                match how_toggle {
                    1 => {
                        num = getnum(&mut s, 0 as *mut libc::c_char, &mut err);
                        if err == 0 {
                            *(*o).ovar = num;
                        }
                    }
                    2 => {
                        *(*o).ovar = (*o).odefault;
                    }
                    3 => {
                        error(
                            b"Can't use \"-!\" for a numeric option\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            0 as *mut libc::c_void as *mut PARG,
                        );
                        return;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    if ((*o).ofunc).is_some() {
        (Some(((*o).ofunc).unwrap()))
            .unwrap()(
            if how_toggle == 0 as libc::c_int {
                1 as libc::c_int
            } else {
                2 as libc::c_int
            },
            s,
        );
    }
    if how_toggle != 0 as libc::c_int && (*o).otype & 0o200 as libc::c_int != 0 {
        chg_hilite();
    }
    if no_prompt == 0 {
        match (*o).otype
            & (0o1 as libc::c_int | 0o2 as libc::c_int | 0o4 as libc::c_int
                | 0o10 as libc::c_int | 0o20 as libc::c_int)
        {
            1 | 2 => {
                error(
                    (*o).odesc[*(*o).ovar as usize],
                    0 as *mut libc::c_void as *mut PARG,
                );
            }
            4 => {
                parg.p_int = *(*o).ovar;
                error((*o).odesc[1 as libc::c_int as usize], &mut parg);
            }
            8 | _ => {}
        }
    }
    if how_toggle != 0 as libc::c_int && (*o).otype & 0o40 as libc::c_int != 0 {
        screen_trashed = 1 as libc::c_int;
    }
}
unsafe extern "C" fn flip_triple(
    mut val: libc::c_int,
    mut lc: libc::c_int,
) -> libc::c_int {
    if lc != 0 {
        return if val == 1 as libc::c_int { 0 as libc::c_int } else { 1 as libc::c_int }
    } else {
        return if val == 2 as libc::c_int { 0 as libc::c_int } else { 2 as libc::c_int }
    };
}
pub unsafe extern "C" fn opt_has_param(mut o: *mut loption) -> libc::c_int {
    if o.is_null() {
        return 0 as libc::c_int;
    }
    if (*o).otype
        & (0o1 as libc::c_int | 0o2 as libc::c_int | 0o20 as libc::c_int
            | 0o100 as libc::c_int) != 0
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn opt_prompt(mut o: *mut loption) -> *mut libc::c_char {
    if o.is_null()
        || (*o).otype & (0o10 as libc::c_int | 0o4 as libc::c_int) == 0 as libc::c_int
    {
        return b"?\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    return (*o).odesc[0 as libc::c_int as usize];
}
pub unsafe extern "C" fn opt_toggle_disallowed(mut c: libc::c_int) -> *mut libc::c_char {
    match c {
        111 => {
            if ch_getflags() & 0o1 as libc::c_int != 0 {
                return b"Input is not a pipe\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
        }
        _ => {}
    }
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn isoptpending() -> libc::c_int {
    return (pendopt != 0 as *mut libc::c_void as *mut loption) as libc::c_int;
}
unsafe extern "C" fn nostring(mut printopt: *mut libc::c_char) {
    let mut parg: PARG = parg {
        p_string: 0 as *mut libc::c_char,
    };
    parg.p_string = printopt;
    error(
        b"Value is required after %s\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut parg,
    );
}
pub unsafe extern "C" fn nopendopt() {
    nostring(opt_desc(pendopt));
}
unsafe extern "C" fn optstring(
    mut s: *mut libc::c_char,
    mut p_str: *mut *mut libc::c_char,
    mut printopt: *mut libc::c_char,
    mut validchars: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    if *s as libc::c_int == '\0' as i32 {
        nostring(printopt);
        return 0 as *mut libc::c_char;
    }
    *p_str = ecalloc(
        (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_uint,
    ) as *mut libc::c_char;
    out = *p_str;
    p = s;
    while *p as libc::c_int != '\0' as i32 {
        if opt_use_backslash != 0 && *p as libc::c_int == '\\' as i32
            && *p.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32
        {
            p = p.offset(1);
            p;
        } else if *p as libc::c_int == '$' as i32
            || !validchars.is_null() && (strchr(validchars, *p as libc::c_int)).is_null()
        {
            break;
        }
        let fresh1 = out;
        out = out.offset(1);
        *fresh1 = *p;
        p = p.offset(1);
        p;
    }
    *out = '\0' as i32 as libc::c_char;
    return p;
}
unsafe extern "C" fn num_error(
    mut printopt: *mut libc::c_char,
    mut errp: *mut libc::c_int,
    mut overflow: libc::c_int,
) -> libc::c_int {
    let mut parg: PARG = parg {
        p_string: 0 as *mut libc::c_char,
    };
    if !errp.is_null() {
        *errp = 1 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if !printopt.is_null() {
        parg.p_string = printopt;
        error(
            (if overflow != 0 {
                b"Number too large in '%s'\0" as *const u8 as *const libc::c_char
            } else {
                b"Number is required after %s\0" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char,
            &mut parg,
        );
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn getnum(
    mut sp: *mut *mut libc::c_char,
    mut printopt: *mut libc::c_char,
    mut errp: *mut libc::c_int,
) -> libc::c_int {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0;
    let mut neg: libc::c_int = 0;
    s = skipsp(*sp);
    neg = 0 as libc::c_int;
    if *s as libc::c_int == '-' as i32 {
        neg = 1 as libc::c_int;
        s = s.offset(1);
        s;
    }
    if (*s as libc::c_int) < '0' as i32 || *s as libc::c_int > '9' as i32 {
        return num_error(printopt, errp, 0 as libc::c_int);
    }
    n = lstrtoi(s, sp, 10 as libc::c_int);
    if n < 0 as libc::c_int {
        return num_error(printopt, errp, 1 as libc::c_int);
    }
    if !errp.is_null() {
        *errp = 0 as libc::c_int;
    }
    if neg != 0 {
        n = -n;
    }
    return n;
}
pub unsafe extern "C" fn getfraction(
    mut sp: *mut *mut libc::c_char,
    mut printopt: *mut libc::c_char,
    mut errp: *mut libc::c_int,
) -> libc::c_long {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut frac: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut fraclen: libc::c_int = 0 as libc::c_int;
    s = skipsp(*sp);
    if (*s as libc::c_int) < '0' as i32 || *s as libc::c_int > '9' as i32 {
        return num_error(printopt, errp, 0 as libc::c_int) as libc::c_long;
    }
    while *s as libc::c_int >= '0' as i32 && *s as libc::c_int <= '9' as i32 {
        if !(6 as libc::c_int <= fraclen) {
            frac = frac * 10 as libc::c_int as libc::c_long
                + (*s as libc::c_int - '0' as i32) as libc::c_long;
            fraclen += 1;
            fraclen;
        }
        s = s.offset(1);
        s;
    }
    loop {
        let fresh2 = fraclen;
        fraclen = fraclen + 1;
        if !(fresh2 < 6 as libc::c_int) {
            break;
        }
        frac *= 10 as libc::c_int as libc::c_long;
    }
    *sp = s;
    if !errp.is_null() {
        *errp = 0 as libc::c_int;
    }
    return frac;
}
pub unsafe extern "C" fn get_quit_at_eof() -> libc::c_int {
    if less_is_more == 0 {
        return quit_at_eof;
    }
    return if quit_at_eof != 0 { 2 as libc::c_int } else { 1 as libc::c_int };
}
