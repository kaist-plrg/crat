use ::libc;
extern "C" {
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn quit(status: libc::c_int);
    fn interactive() -> libc::c_int;
    fn lower_left();
    fn clear_eol();
    fn clear_bot();
    fn at_enter(attr: libc::c_int);
    fn at_exit();
    fn at_switch(attr: libc::c_int);
    fn putbs();
    fn prchar(c: LWCHAR) -> *mut libc::c_char;
    fn ungetcc(c: LWCHAR);
    fn squish_check();
    fn gline(i: libc::c_int, ap: *mut libc::c_int) -> libc::c_int;
    fn supports_ctrl_x() -> libc::c_int;
    fn getchr() -> libc::c_int;
    fn help_ckd_mul(
        r: *mut libc::c_void,
        a: uintmax,
        b: uintmax,
        rsize: libc::c_int,
        rsigned: libc::c_int,
    ) -> libc::c_int;
    fn help_ckd_add(
        r: *mut libc::c_void,
        a: uintmax,
        b: uintmax,
        rsize: libc::c_int,
        rsigned: libc::c_int,
    ) -> libc::c_int;
    static mut sigs: libc::c_int;
    static mut sc_width: libc::c_int;
    static mut so_s_width: libc::c_int;
    static mut so_e_width: libc::c_int;
    static mut screen_trashed: libc::c_int;
    static mut oldbot: libc::c_int;
    static mut intr_char: libc::c_char;
}
pub type __uintmax_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type uintmax_t = __uintmax_t;
pub type uintmax = uintmax_t;
pub type LWCHAR = libc::c_ulong;
pub type POSITION = off_t;
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
pub static mut errmsgs: libc::c_int = 0;
pub static mut need_clr: libc::c_int = 0;
pub static mut final_attr: libc::c_int = 0;
pub static mut at_prompt: libc::c_int = 0;
pub unsafe extern "C" fn put_line() {
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    if sigs & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0 {
        screen_trashed = 1 as libc::c_int;
        return;
    }
    final_attr = 0 as libc::c_int;
    i = 0 as libc::c_int;
    loop {
        c = gline(i, &mut a);
        if !(c != '\0' as i32) {
            break;
        }
        at_switch(a);
        final_attr = a;
        if c == '\u{8}' as i32 {
            putbs();
        } else {
            putchr(c);
        }
        i += 1;
        i;
    }
    at_exit();
}
static mut obuf: [libc::c_char; 1024] = [0; 1024];
static mut ob: *mut libc::c_char = unsafe { obuf.as_ptr() as *mut _ };
static mut outfd: libc::c_int = 2 as libc::c_int;
pub unsafe extern "C" fn flush() {
    let mut n: libc::c_int = 0;
    n = ob.offset_from(obuf.as_mut_ptr()) as libc::c_long as libc::c_int;
    if n == 0 as libc::c_int {
        return;
    }
    ob = obuf.as_mut_ptr();
    if write(outfd, obuf.as_mut_ptr() as *const libc::c_void, n as size_t)
        != n as libc::c_long
    {
        screen_trashed = 1 as libc::c_int;
    }
}
pub unsafe extern "C" fn set_output(mut fd: libc::c_int) {
    flush();
    outfd = fd;
}
pub unsafe extern "C" fn putchr(mut c: libc::c_int) -> libc::c_int {
    clear_bot_if_needed();
    if ob
        >= &mut *obuf
            .as_mut_ptr()
            .offset(
                (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) as *mut libc::c_char
    {
        flush();
    }
    let fresh0 = ob;
    ob = ob.offset(1);
    *fresh0 = c as libc::c_char;
    at_prompt = 0 as libc::c_int;
    return c;
}
pub unsafe extern "C" fn clear_bot_if_needed() {
    if need_clr == 0 {
        return;
    }
    need_clr = 0 as libc::c_int;
    clear_bot();
}
pub unsafe extern "C" fn putstr(mut s: *const libc::c_char) {
    while *s as libc::c_int != '\0' as i32 {
        let fresh1 = s;
        s = s.offset(1);
        putchr(*fresh1 as libc::c_int);
    }
}
pub unsafe extern "C" fn postoa(
    mut num: POSITION,
    mut buf: *mut libc::c_char,
    mut radix: libc::c_int,
) {
    let mut neg: libc::c_int = (num < 0 as libc::c_int as libc::c_long) as libc::c_int;
    let mut tbuf: [libc::c_char; 23] = [0; 23];
    let mut s: *mut libc::c_char = tbuf
        .as_mut_ptr()
        .offset(::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong as isize);
    if neg != 0 {
        num = -num;
    }
    s = s.offset(-1);
    *s = '\0' as i32 as libc::c_char;
    loop {
        s = s.offset(-1);
        *s = (*::std::mem::transmute::<
            &[u8; 17],
            &[libc::c_char; 17],
        >(b"0123456789ABCDEF\0"))[(num % radix as libc::c_long) as usize];
        num /= radix as libc::c_long;
        if !(num != 0 as libc::c_int as libc::c_long) {
            break;
        }
    }
    if neg != 0 {
        s = s.offset(-1);
        *s = '-' as i32 as libc::c_char;
    }
    strcpy(buf, s);
}
pub unsafe extern "C" fn linenumtoa(
    mut num: LINENUM,
    mut buf: *mut libc::c_char,
    mut radix: libc::c_int,
) {
    let mut neg: libc::c_int = (num < 0 as libc::c_int as libc::c_long) as libc::c_int;
    let mut tbuf: [libc::c_char; 23] = [0; 23];
    let mut s: *mut libc::c_char = tbuf
        .as_mut_ptr()
        .offset(::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong as isize);
    if neg != 0 {
        num = -num;
    }
    s = s.offset(-1);
    *s = '\0' as i32 as libc::c_char;
    loop {
        s = s.offset(-1);
        *s = (*::std::mem::transmute::<
            &[u8; 17],
            &[libc::c_char; 17],
        >(b"0123456789ABCDEF\0"))[(num % radix as libc::c_long) as usize];
        num /= radix as libc::c_long;
        if !(num != 0 as libc::c_int as libc::c_long) {
            break;
        }
    }
    if neg != 0 {
        s = s.offset(-1);
        *s = '-' as i32 as libc::c_char;
    }
    strcpy(buf, s);
}
pub unsafe extern "C" fn inttoa(
    mut num: libc::c_int,
    mut buf: *mut libc::c_char,
    mut radix: libc::c_int,
) {
    let mut neg: libc::c_int = (num < 0 as libc::c_int) as libc::c_int;
    let mut tbuf: [libc::c_char; 13] = [0; 13];
    let mut s: *mut libc::c_char = tbuf
        .as_mut_ptr()
        .offset(::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as isize);
    if neg != 0 {
        num = -num;
    }
    s = s.offset(-1);
    *s = '\0' as i32 as libc::c_char;
    loop {
        s = s.offset(-1);
        *s = (*::std::mem::transmute::<
            &[u8; 17],
            &[libc::c_char; 17],
        >(b"0123456789ABCDEF\0"))[(num % radix) as usize];
        num /= radix;
        if !(num != 0 as libc::c_int) {
            break;
        }
    }
    if neg != 0 {
        s = s.offset(-1);
        *s = '-' as i32 as libc::c_char;
    }
    strcpy(buf, s);
}
pub unsafe extern "C" fn lstrtopos(
    mut buf: *mut libc::c_char,
    mut ebuf: *mut *mut libc::c_char,
    mut radix: libc::c_int,
) -> POSITION {
    let mut val: POSITION = 0 as libc::c_int as POSITION;
    let mut v: libc::c_int = 0 as libc::c_int;
    loop {
        let mut c: libc::c_char = *buf;
        let mut digit: libc::c_int = if c as libc::c_int >= '0' as i32
            && c as libc::c_int <= '9' as i32
        {
            c as libc::c_int - '0' as i32
        } else if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'f' as i32 {
            c as libc::c_int - 'a' as i32 + 10 as libc::c_int
        } else if c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'F' as i32 {
            c as libc::c_int - 'A' as i32 + 10 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
        if digit < 0 as libc::c_int || digit >= radix {
            break;
        }
        v
            |= help_ckd_mul(
                &mut val as *mut POSITION as *mut libc::c_void,
                val as uintmax,
                radix as uintmax,
                ::std::mem::size_of::<POSITION>() as libc::c_ulong as libc::c_int,
                (((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    val
                }) - 1 as libc::c_int as libc::c_long)
                    < 0 as libc::c_int as libc::c_long) as libc::c_int,
            );
        v
            |= help_ckd_add(
                &mut val as *mut POSITION as *mut libc::c_void,
                val as uintmax,
                digit as uintmax,
                ::std::mem::size_of::<POSITION>() as libc::c_ulong as libc::c_int,
                (((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    val
                }) - 1 as libc::c_int as libc::c_long)
                    < 0 as libc::c_int as libc::c_long) as libc::c_int,
            );
        buf = buf.offset(1);
        buf;
    }
    if !ebuf.is_null() {
        *ebuf = buf;
    }
    return if v != 0 { -(1 as libc::c_int) as libc::c_long } else { val };
}
pub unsafe extern "C" fn lstrtoi(
    mut buf: *mut libc::c_char,
    mut ebuf: *mut *mut libc::c_char,
    mut radix: libc::c_int,
) -> libc::c_int {
    let mut val: libc::c_int = 0 as libc::c_int;
    let mut v: libc::c_int = 0 as libc::c_int;
    loop {
        let mut c: libc::c_char = *buf;
        let mut digit: libc::c_int = if c as libc::c_int >= '0' as i32
            && c as libc::c_int <= '9' as i32
        {
            c as libc::c_int - '0' as i32
        } else if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'f' as i32 {
            c as libc::c_int - 'a' as i32 + 10 as libc::c_int
        } else if c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'F' as i32 {
            c as libc::c_int - 'A' as i32 + 10 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
        if digit < 0 as libc::c_int || digit >= radix {
            break;
        }
        v
            |= help_ckd_mul(
                &mut val as *mut libc::c_int as *mut libc::c_void,
                val as uintmax,
                radix as uintmax,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
                (((if 1 as libc::c_int != 0 { 0 as libc::c_int } else { val })
                    - 1 as libc::c_int) < 0 as libc::c_int) as libc::c_int,
            );
        v
            |= help_ckd_add(
                &mut val as *mut libc::c_int as *mut libc::c_void,
                val as uintmax,
                digit as uintmax,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
                (((if 1 as libc::c_int != 0 { 0 as libc::c_int } else { val })
                    - 1 as libc::c_int) < 0 as libc::c_int) as libc::c_int,
            );
        buf = buf.offset(1);
        buf;
    }
    if !ebuf.is_null() {
        *ebuf = buf;
    }
    return if v != 0 { -(1 as libc::c_int) } else { val };
}
pub unsafe extern "C" fn lstrtoul(
    mut buf: *mut libc::c_char,
    mut ebuf: *mut *mut libc::c_char,
    mut radix: libc::c_int,
) -> libc::c_ulong {
    let mut val: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut v: libc::c_int = 0 as libc::c_int;
    loop {
        let mut c: libc::c_char = *buf;
        let mut digit: libc::c_int = if c as libc::c_int >= '0' as i32
            && c as libc::c_int <= '9' as i32
        {
            c as libc::c_int - '0' as i32
        } else if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'f' as i32 {
            c as libc::c_int - 'a' as i32 + 10 as libc::c_int
        } else if c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'F' as i32 {
            c as libc::c_int - 'A' as i32 + 10 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
        if digit < 0 as libc::c_int || digit >= radix {
            break;
        }
        v
            |= help_ckd_mul(
                &mut val as *mut libc::c_ulong as *mut libc::c_void,
                val,
                radix as uintmax,
                ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong as libc::c_int,
                ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    val
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong) as libc::c_int,
            );
        v
            |= help_ckd_add(
                &mut val as *mut libc::c_ulong as *mut libc::c_void,
                val,
                digit as uintmax,
                ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong as libc::c_int,
                ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    val
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong) as libc::c_int,
            );
        buf = buf.offset(1);
        buf;
    }
    if !ebuf.is_null() {
        *ebuf = buf;
    }
    return if v != 0 { -(1 as libc::c_int) as libc::c_ulong } else { val };
}
unsafe extern "C" fn iprint_int(
    mut num: libc::c_int,
    mut radix: libc::c_int,
) -> libc::c_int {
    let mut buf: [libc::c_char; 11] = [0; 11];
    inttoa(num, buf.as_mut_ptr(), radix);
    putstr(buf.as_mut_ptr());
    return strlen(buf.as_mut_ptr()) as libc::c_int;
}
unsafe extern "C" fn iprint_linenum(
    mut num: LINENUM,
    mut radix: libc::c_int,
) -> libc::c_int {
    let mut buf: [libc::c_char; 21] = [0; 21];
    linenumtoa(num, buf.as_mut_ptr(), radix);
    putstr(buf.as_mut_ptr());
    return strlen(buf.as_mut_ptr()) as libc::c_int;
}
pub unsafe extern "C" fn less_printf(
    mut fmt: *mut libc::c_char,
    mut parg: *mut PARG,
) -> libc::c_int {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut col: libc::c_int = 0;
    col = 0 as libc::c_int;
    while *fmt as libc::c_int != '\0' as i32 {
        if *fmt as libc::c_int != '%' as i32 {
            let fresh2 = fmt;
            fmt = fmt.offset(1);
            putchr(*fresh2 as libc::c_int);
            col += 1;
            col;
        } else {
            fmt = fmt.offset(1);
            fmt;
            let fresh3 = fmt;
            fmt = fmt.offset(1);
            match *fresh3 as libc::c_int {
                115 => {
                    s = (*parg).p_string;
                    parg = parg.offset(1);
                    parg;
                    while *s as libc::c_int != '\0' as i32 {
                        let fresh4 = s;
                        s = s.offset(1);
                        putchr(*fresh4 as libc::c_int);
                        col += 1;
                        col;
                    }
                }
                100 => {
                    col += iprint_int((*parg).p_int, 10 as libc::c_int);
                    parg = parg.offset(1);
                    parg;
                }
                120 => {
                    col += iprint_int((*parg).p_int, 16 as libc::c_int);
                    parg = parg.offset(1);
                    parg;
                }
                110 => {
                    col += iprint_linenum((*parg).p_linenum, 10 as libc::c_int);
                    parg = parg.offset(1);
                    parg;
                }
                99 => {
                    s = prchar((*parg).p_char as LWCHAR);
                    parg = parg.offset(1);
                    parg;
                    while *s as libc::c_int != '\0' as i32 {
                        let fresh5 = s;
                        s = s.offset(1);
                        putchr(*fresh5 as libc::c_int);
                        col += 1;
                        col;
                    }
                }
                37 => {
                    putchr('%' as i32);
                }
                _ => {}
            }
        }
    }
    return col;
}
pub unsafe extern "C" fn get_return() {
    let mut c: libc::c_int = 0;
    c = getchr();
    if c != '\n' as i32 && c != '\r' as i32 && c != ' ' as i32
        && c != -(2 as libc::c_int)
    {
        ungetcc(c as LWCHAR);
    }
}
pub unsafe extern "C" fn error(mut fmt: *mut libc::c_char, mut parg: *mut PARG) {
    let mut col: libc::c_int = 0 as libc::c_int;
    static mut return_to_continue: [libc::c_char; 17] = unsafe {
        *::std::mem::transmute::<
            &[u8; 17],
            &mut [libc::c_char; 17],
        >(b"  (press RETURN)\0")
    };
    errmsgs += 1;
    errmsgs;
    if interactive() == 0 {
        less_printf(fmt, parg);
        putchr('\n' as i32);
        return;
    }
    if oldbot == 0 {
        squish_check();
    }
    at_exit();
    clear_bot();
    at_enter(
        (1 as libc::c_int) << 3 as libc::c_int | (4 as libc::c_int) << 8 as libc::c_int,
    );
    col += so_s_width;
    col += less_printf(fmt, parg);
    putstr(return_to_continue.as_mut_ptr());
    at_exit();
    col = (col as libc::c_ulong)
        .wrapping_add(
            (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                .wrapping_add(so_e_width as libc::c_ulong),
        ) as libc::c_int as libc::c_int;
    get_return();
    lower_left();
    clear_eol();
    if col >= sc_width {
        screen_trashed = 1 as libc::c_int;
    }
    flush();
}
unsafe extern "C" fn ierror_suffix(
    mut fmt: *mut libc::c_char,
    mut parg: *mut PARG,
    mut suffix1: *mut libc::c_char,
    mut suffix2: *mut libc::c_char,
    mut suffix3: *mut libc::c_char,
) {
    at_exit();
    clear_bot();
    at_enter(
        (1 as libc::c_int) << 3 as libc::c_int | (4 as libc::c_int) << 8 as libc::c_int,
    );
    less_printf(fmt, parg);
    putstr(suffix1);
    putstr(suffix2);
    putstr(suffix3);
    at_exit();
    flush();
    need_clr = 1 as libc::c_int;
}
pub unsafe extern "C" fn ierror(mut fmt: *mut libc::c_char, mut parg: *mut PARG) {
    ierror_suffix(
        fmt,
        parg,
        b"... (interrupt to abort)\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn ixerror(mut fmt: *mut libc::c_char, mut parg: *mut PARG) {
    if supports_ctrl_x() == 0 {
        ierror(fmt, parg);
    } else {
        ierror_suffix(
            fmt,
            parg,
            b"... (\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            prchar(intr_char as LWCHAR),
            b" or interrupt to abort)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    };
}
pub unsafe extern "C" fn query(
    mut fmt: *mut libc::c_char,
    mut parg: *mut PARG,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut col: libc::c_int = 0 as libc::c_int;
    if interactive() != 0 {
        clear_bot();
    }
    less_printf(fmt, parg);
    c = getchr();
    if interactive() != 0 {
        lower_left();
        if col >= sc_width {
            screen_trashed = 1 as libc::c_int;
        }
        flush();
    } else {
        putchr('\n' as i32);
    }
    if c == 'Q' as i32 {
        quit(0 as libc::c_int);
    }
    return c;
}
