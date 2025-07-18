use ::libc;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn save(s: *const libc::c_char) -> *mut libc::c_char;
    fn skipsp(s: *mut libc::c_char) -> *mut libc::c_char;
    fn quit(status: libc::c_int);
    fn init_mouse();
    fn deinit_mouse();
    fn sync_logfile();
    fn ch_length() -> POSITION;
    fn ch_setbufspace(bufspace_0: libc::c_int);
    fn ch_getflags() -> libc::c_int;
    fn setfmt(
        s: *mut libc::c_char,
        fmtvarptr: *mut *mut libc::c_char,
        attrptr: *mut libc::c_int,
        default_fmt: *mut libc::c_char,
        for_printf: libc::c_int,
    );
    fn prchar(c: LWCHAR) -> *mut libc::c_char;
    fn norm_search_type(st: libc::c_int) -> libc::c_int;
    fn dispversion();
    fn ungetcc_back(c: LWCHAR);
    fn ungetsc(s: *mut libc::c_char);
    fn lesskey(filename: *mut libc::c_char, sysvar: libc::c_int) -> libc::c_int;
    fn lesskey_src(filename: *mut libc::c_char, sysvar: libc::c_int) -> libc::c_int;
    fn save_curr_ifile() -> *mut libc::c_void;
    fn unsave_ifile(save_ifile: *mut libc::c_void);
    fn reedit_ifile(save_ifile: *mut libc::c_void);
    fn use_logfile(filename: *mut libc::c_char);
    fn shell_unquote(str: *mut libc::c_char) -> *mut libc::c_char;
    fn lglob(filename: *mut libc::c_char) -> *mut libc::c_char;
    fn jump_loc(pos: POSITION, sline: libc::c_int);
    fn muldiv(val: uintmax, num: uintmax, den: uintmax) -> uintmax;
    fn getfraction(
        sp: *mut *mut libc::c_char,
        printopt: *mut libc::c_char,
        errp: *mut libc::c_int,
    ) -> libc::c_long;
    fn putstr(s: *const libc::c_char);
    fn pattern_lib_name() -> *mut libc::c_char;
    fn set_output(fd: libc::c_int);
    fn error(fmt: *mut libc::c_char, parg: *mut PARG);
    fn scan_eof();
    fn set_color_map(attr: libc::c_int, colorstr: *mut libc::c_char) -> libc::c_int;
    fn getnum(
        sp: *mut *mut libc::c_char,
        printopt: *mut libc::c_char,
        errp: *mut libc::c_int,
    ) -> libc::c_int;
    fn tagsearch() -> POSITION;
    fn edit_tagfile() -> libc::c_int;
    fn chg_caseless();
    fn findtag(tag: *mut libc::c_char);
    fn default_wheel_lines() -> libc::c_int;
    fn help_ckd_add(
        r: *mut libc::c_void,
        a: uintmax,
        b: uintmax,
        rsize: libc::c_int,
        rsigned: libc::c_int,
    ) -> libc::c_int;
    fn help_ckd_mul(
        r: *mut libc::c_void,
        a: uintmax,
        b: uintmax,
        rsize: libc::c_int,
        rsigned: libc::c_int,
    ) -> libc::c_int;
    static mut bufspace: libc::c_int;
    static mut pr_type: libc::c_int;
    static mut plusoption: libc::c_int;
    static mut swindow: libc::c_int;
    static mut sc_width: libc::c_int;
    static mut sc_height: libc::c_int;
    static mut secure: libc::c_int;
    static mut dohelp: libc::c_int;
    static mut openquote: libc::c_char;
    static mut closequote: libc::c_char;
    static mut prproto: [*mut libc::c_char; 0];
    static mut eqproto: *mut libc::c_char;
    static mut hproto: *mut libc::c_char;
    static mut wproto: *mut libc::c_char;
    static mut every_first_cmd: *mut libc::c_char;
    static mut curr_ifile: *mut libc::c_void;
    static mut version: [libc::c_char; 0];
    static mut jump_sline: libc::c_int;
    static mut jump_sline_fraction: libc::c_long;
    static mut shift_count: libc::c_int;
    static mut shift_count_fraction: libc::c_long;
    static mut rscroll_char: libc::c_char;
    static mut rscroll_attr: libc::c_int;
    static mut mousecap: libc::c_int;
    static mut wheel_lines: libc::c_int;
    static mut less_is_more: libc::c_int;
    static mut linenum_width: libc::c_int;
    static mut status_col_width: libc::c_int;
    static mut use_color: libc::c_int;
    static mut want_filesize: libc::c_int;
    static mut header_lines: libc::c_int;
    static mut header_cols: libc::c_int;
    static mut def_search_type: libc::c_int;
    static mut chopline: libc::c_int;
    static mut tabstops: [libc::c_int; 0];
    static mut ntabstops: libc::c_int;
    static mut tabdefault: libc::c_int;
    static mut intr_char: libc::c_char;
    static mut namelogfile: *mut libc::c_char;
    static mut force_logfile: libc::c_int;
    static mut logfile: libc::c_int;
    static mut tags: *mut libc::c_char;
    static mut ztags: [libc::c_char; 0];
}
pub type __uintmax_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type off_t = __off_t;
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
pub static mut tagoption: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub unsafe extern "C" fn opt_o(mut type_0: libc::c_int, mut s: *mut libc::c_char) {
    let mut parg: PARG = parg {
        p_string: 0 as *mut libc::c_char,
    };
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    if secure != 0 {
        error(
            b"log file support is not available\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *mut libc::c_void as *mut PARG,
        );
        return;
    }
    match type_0 {
        0 => {
            namelogfile = save(s);
        }
        2 => {
            if ch_getflags() & 0o1 as libc::c_int != 0 {
                error(
                    b"Input is not a pipe\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *mut libc::c_void as *mut PARG,
                );
                return;
            }
            if logfile >= 0 as libc::c_int {
                error(
                    b"Log file is already in use\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *mut libc::c_void as *mut PARG,
                );
                return;
            }
            s = skipsp(s);
            if !namelogfile.is_null() {
                free(namelogfile as *mut libc::c_void);
            }
            filename = lglob(s);
            namelogfile = shell_unquote(filename);
            free(filename as *mut libc::c_void);
            use_logfile(namelogfile);
            sync_logfile();
        }
        1 => {
            if logfile < 0 as libc::c_int {
                error(
                    b"No log file\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *mut libc::c_void as *mut PARG,
                );
            } else {
                parg.p_string = namelogfile;
                error(
                    b"Log file \"%s\"\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    &mut parg,
                );
            }
        }
        _ => {}
    };
}
pub unsafe extern "C" fn opt__O(mut type_0: libc::c_int, mut s: *mut libc::c_char) {
    force_logfile = 1 as libc::c_int;
    opt_o(type_0, s);
}
pub unsafe extern "C" fn opt_j(mut type_0: libc::c_int, mut s: *mut libc::c_char) {
    let mut parg: PARG = parg {
        p_string: 0 as *mut libc::c_char,
    };
    let mut len: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    match type_0 {
        0 | 2 => {
            if *s as libc::c_int == '.' as i32 {
                s = s.offset(1);
                s;
                jump_sline_fraction = getfraction(
                    &mut s,
                    b"j\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    &mut err,
                );
                if err != 0 {
                    error(
                        b"Invalid line fraction\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        0 as *mut libc::c_void as *mut PARG,
                    );
                } else {
                    calc_jump_sline();
                }
            } else {
                let mut sline: libc::c_int = getnum(
                    &mut s,
                    b"j\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    &mut err,
                );
                if err != 0 {
                    error(
                        b"Invalid line number\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        0 as *mut libc::c_void as *mut PARG,
                    );
                } else {
                    jump_sline = sline;
                    jump_sline_fraction = -(1 as libc::c_int) as libc::c_long;
                }
            }
        }
        1 => {
            if jump_sline_fraction < 0 as libc::c_int as libc::c_long {
                parg.p_int = jump_sline;
                error(
                    b"Position target at screen line %d\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    &mut parg,
                );
            } else {
                let mut buf: [libc::c_char; 24] = [0; 24];
                snprintf(
                    buf.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong,
                    b".%06ld\0" as *const u8 as *const libc::c_char,
                    jump_sline_fraction,
                );
                len = strlen(buf.as_mut_ptr()) as libc::c_int;
                while len > 2 as libc::c_int
                    && buf[(len - 1 as libc::c_int) as usize] as libc::c_int
                        == '0' as i32
                {
                    len -= 1;
                    len;
                }
                buf[len as usize] = '\0' as i32 as libc::c_char;
                parg.p_string = buf.as_mut_ptr();
                error(
                    b"Position target at screen position %s\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    &mut parg,
                );
            }
        }
        _ => {}
    };
}
pub unsafe extern "C" fn calc_jump_sline() {
    if jump_sline_fraction < 0 as libc::c_int as libc::c_long {
        return;
    }
    jump_sline = muldiv(
        sc_height as uintmax,
        jump_sline_fraction as uintmax,
        1000000 as libc::c_int as uintmax,
    ) as libc::c_int;
}
pub unsafe extern "C" fn opt_shift(mut type_0: libc::c_int, mut s: *mut libc::c_char) {
    let mut parg: PARG = parg {
        p_string: 0 as *mut libc::c_char,
    };
    let mut len: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    match type_0 {
        0 | 2 => {
            if *s as libc::c_int == '.' as i32 {
                s = s.offset(1);
                s;
                shift_count_fraction = getfraction(
                    &mut s,
                    b"#\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    &mut err,
                );
                if err != 0 {
                    error(
                        b"Invalid column fraction\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        0 as *mut libc::c_void as *mut PARG,
                    );
                } else {
                    calc_shift_count();
                }
            } else {
                let mut hs: libc::c_int = getnum(
                    &mut s,
                    b"#\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    &mut err,
                );
                if err != 0 {
                    error(
                        b"Invalid column number\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        0 as *mut libc::c_void as *mut PARG,
                    );
                } else {
                    shift_count = hs;
                    shift_count_fraction = -(1 as libc::c_int) as libc::c_long;
                }
            }
        }
        1 => {
            if shift_count_fraction < 0 as libc::c_int as libc::c_long {
                parg.p_int = shift_count;
                error(
                    b"Horizontal shift %d columns\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    &mut parg,
                );
            } else {
                let mut buf: [libc::c_char; 24] = [0; 24];
                snprintf(
                    buf.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong,
                    b".%06ld\0" as *const u8 as *const libc::c_char,
                    shift_count_fraction,
                );
                len = strlen(buf.as_mut_ptr()) as libc::c_int;
                while len > 2 as libc::c_int
                    && buf[(len - 1 as libc::c_int) as usize] as libc::c_int
                        == '0' as i32
                {
                    len -= 1;
                    len;
                }
                buf[len as usize] = '\0' as i32 as libc::c_char;
                parg.p_string = buf.as_mut_ptr();
                error(
                    b"Horizontal shift %s of screen width\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    &mut parg,
                );
            }
        }
        _ => {}
    };
}
pub unsafe extern "C" fn calc_shift_count() {
    if shift_count_fraction < 0 as libc::c_int as libc::c_long {
        return;
    }
    shift_count = muldiv(
        sc_width as uintmax,
        shift_count_fraction as uintmax,
        1000000 as libc::c_int as uintmax,
    ) as libc::c_int;
}
pub unsafe extern "C" fn opt_k(mut type_0: libc::c_int, mut s: *mut libc::c_char) {
    let mut parg: PARG = parg {
        p_string: 0 as *mut libc::c_char,
    };
    match type_0 {
        0 => {
            if lesskey(s, 0 as libc::c_int) != 0 {
                parg.p_string = s;
                error(
                    b"Cannot use lesskey file \"%s\"\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    &mut parg,
                );
            }
        }
        _ => {}
    };
}
pub unsafe extern "C" fn opt_ks(mut type_0: libc::c_int, mut s: *mut libc::c_char) {
    let mut parg: PARG = parg {
        p_string: 0 as *mut libc::c_char,
    };
    match type_0 {
        0 => {
            if lesskey_src(s, 0 as libc::c_int) != 0 {
                parg.p_string = s;
                error(
                    b"Cannot use lesskey source file \"%s\"\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    &mut parg,
                );
            }
        }
        _ => {}
    };
}
pub unsafe extern "C" fn opt_t(mut type_0: libc::c_int, mut s: *mut libc::c_char) {
    let mut save_ifile: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut pos: POSITION = 0;
    match type_0 {
        0 => {
            tagoption = save(s);
        }
        2 => {
            if secure != 0 {
                error(
                    b"tags support is not available\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    0 as *mut libc::c_void as *mut PARG,
                );
            } else {
                findtag(skipsp(s));
                save_ifile = save_curr_ifile();
                if edit_tagfile() != 0
                    || {
                        pos = tagsearch();
                        pos == -(1 as libc::c_int) as POSITION
                    }
                {
                    reedit_ifile(save_ifile);
                } else {
                    unsave_ifile(save_ifile);
                    jump_loc(pos, jump_sline);
                }
            }
        }
        _ => {}
    };
}
pub unsafe extern "C" fn opt__T(mut type_0: libc::c_int, mut s: *mut libc::c_char) {
    let mut parg: PARG = parg {
        p_string: 0 as *mut libc::c_char,
    };
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    match type_0 {
        0 => {
            tags = save(s);
        }
        2 => {
            s = skipsp(s);
            if !tags.is_null() && tags != ztags.as_mut_ptr() {
                free(tags as *mut libc::c_void);
            }
            filename = lglob(s);
            tags = shell_unquote(filename);
            free(filename as *mut libc::c_void);
        }
        1 => {
            parg.p_string = tags;
            error(
                b"Tags file \"%s\"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                &mut parg,
            );
        }
        _ => {}
    };
}
pub unsafe extern "C" fn opt_p(mut type_0: libc::c_int, mut s: *mut libc::c_char) {
    match type_0 {
        0 => {
            if less_is_more != 0 {
                every_first_cmd = save(s);
            } else {
                plusoption = 1 as libc::c_int;
                ungetsc(b"/\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
                ungetsc(s);
                ungetcc_back(0x40000000 as libc::c_int as LWCHAR);
            }
        }
        _ => {}
    };
}
pub unsafe extern "C" fn opt__P(mut type_0: libc::c_int, mut s: *mut libc::c_char) {
    let mut proto: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut parg: PARG = parg {
        p_string: 0 as *mut libc::c_char,
    };
    match type_0 {
        0 | 2 => {
            match *s as libc::c_int {
                115 => {
                    proto = &mut *prproto.as_mut_ptr().offset(0 as libc::c_int as isize)
                        as *mut *mut libc::c_char;
                    s = s.offset(1);
                    s;
                }
                109 => {
                    proto = &mut *prproto.as_mut_ptr().offset(1 as libc::c_int as isize)
                        as *mut *mut libc::c_char;
                    s = s.offset(1);
                    s;
                }
                77 => {
                    proto = &mut *prproto.as_mut_ptr().offset(2 as libc::c_int as isize)
                        as *mut *mut libc::c_char;
                    s = s.offset(1);
                    s;
                }
                61 => {
                    proto = &mut eqproto;
                    s = s.offset(1);
                    s;
                }
                104 => {
                    proto = &mut hproto;
                    s = s.offset(1);
                    s;
                }
                119 => {
                    proto = &mut wproto;
                    s = s.offset(1);
                    s;
                }
                _ => {
                    proto = &mut *prproto.as_mut_ptr().offset(0 as libc::c_int as isize)
                        as *mut *mut libc::c_char;
                }
            }
            free(*proto as *mut libc::c_void);
            *proto = save(s);
        }
        1 => {
            parg.p_string = *prproto.as_mut_ptr().offset(pr_type as isize);
            error(
                b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                &mut parg,
            );
        }
        _ => {}
    };
}
pub unsafe extern "C" fn opt_b(mut type_0: libc::c_int, mut s: *mut libc::c_char) {
    match type_0 {
        0 | 2 => {
            ch_setbufspace(bufspace);
        }
        1 | _ => {}
    };
}
pub unsafe extern "C" fn opt_i(mut type_0: libc::c_int, mut s: *mut libc::c_char) {
    match type_0 {
        2 => {
            chg_caseless();
        }
        1 | 0 | _ => {}
    };
}
pub unsafe extern "C" fn opt__V(mut type_0: libc::c_int, mut s: *mut libc::c_char) {
    match type_0 {
        2 | 1 => {
            dispversion();
        }
        0 => {
            set_output(1 as libc::c_int);
            putstr(b"less \0" as *const u8 as *const libc::c_char);
            putstr(version.as_mut_ptr());
            putstr(b" (\0" as *const u8 as *const libc::c_char);
            putstr(pattern_lib_name());
            putstr(b" regular expressions)\n\0" as *const u8 as *const libc::c_char);
            let mut copyright: *const libc::c_char = b"Copyright (C) 1984-2023  Mark Nudelman\n\n\0"
                as *const u8 as *const libc::c_char;
            putstr(copyright);
            if *version
                .as_mut_ptr()
                .offset(
                    (strlen(version.as_mut_ptr()))
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) as libc::c_int == 'x' as i32
            {
                putstr(
                    b"** This is an EXPERIMENTAL build of the 'less' software,\n\0"
                        as *const u8 as *const libc::c_char,
                );
                putstr(
                    b"** and may not function correctly.\n\0" as *const u8
                        as *const libc::c_char,
                );
                putstr(
                    b"** Obtain release builds from the web page below.\n\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            putstr(
                b"less comes with NO WARRANTY, to the extent permitted by law.\n\0"
                    as *const u8 as *const libc::c_char,
            );
            putstr(
                b"For information about the terms of redistribution,\n\0" as *const u8
                    as *const libc::c_char,
            );
            putstr(
                b"see the file named README in the less distribution.\n\0" as *const u8
                    as *const libc::c_char,
            );
            putstr(
                b"Home page: https://greenwoodsoftware.com/less\n\0" as *const u8
                    as *const libc::c_char,
            );
            quit(0 as libc::c_int);
        }
        _ => {}
    };
}
unsafe extern "C" fn color_from_namechar(mut namechar: libc::c_char) -> libc::c_int {
    match namechar as libc::c_int {
        66 => return (2 as libc::c_int) << 8 as libc::c_int,
        67 => return (3 as libc::c_int) << 8 as libc::c_int,
        69 => return (4 as libc::c_int) << 8 as libc::c_int,
        72 => return (9 as libc::c_int) << 8 as libc::c_int,
        77 => return (6 as libc::c_int) << 8 as libc::c_int,
        78 => return (5 as libc::c_int) << 8 as libc::c_int,
        80 => return (7 as libc::c_int) << 8 as libc::c_int,
        82 => return (8 as libc::c_int) << 8 as libc::c_int,
        83 => return (10 as libc::c_int) << 8 as libc::c_int,
        87 | 65 => return (1 as libc::c_int) << 8 as libc::c_int,
        110 => return 0 as libc::c_int,
        115 => return (1 as libc::c_int) << 3 as libc::c_int,
        100 => return (1 as libc::c_int) << 1 as libc::c_int,
        117 => return (1 as libc::c_int) << 0 as libc::c_int,
        107 => return (1 as libc::c_int) << 2 as libc::c_int,
        _ => {
            if namechar as libc::c_int >= '1' as i32
                && namechar as libc::c_int
                    <= '0' as i32
                        + (16 as libc::c_int - 10 as libc::c_int - 1 as libc::c_int)
            {
                return 10 as libc::c_int + (namechar as libc::c_int - '0' as i32)
                    << 8 as libc::c_int;
            }
            return -(1 as libc::c_int);
        }
    };
}
pub unsafe extern "C" fn opt_D(mut type_0: libc::c_int, mut s: *mut libc::c_char) {
    let mut p: PARG = parg {
        p_string: 0 as *mut libc::c_char,
    };
    let mut attr: libc::c_int = 0;
    match type_0 {
        0 | 2 => {
            attr = color_from_namechar(*s.offset(0 as libc::c_int as isize));
            if attr < 0 as libc::c_int {
                p.p_char = *s.offset(0 as libc::c_int as isize);
                error(
                    b"Invalid color specifier '%c'\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    &mut p,
                );
                return;
            }
            if use_color == 0
                && attr & (16 as libc::c_int - 1 as libc::c_int) << 8 as libc::c_int != 0
            {
                error(
                    b"Set --use-color before changing colors\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    0 as *mut libc::c_void as *mut PARG,
                );
                return;
            }
            s = s.offset(1);
            s;
            if set_color_map(attr, s) < 0 as libc::c_int {
                p.p_string = s;
                error(
                    b"Invalid color string \"%s\"\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    &mut p,
                );
                return;
            }
        }
        _ => {}
    };
}
pub unsafe extern "C" fn set_tabs(mut s: *mut libc::c_char, mut len: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut es: *mut libc::c_char = s.offset(len as isize);
    i = 1 as libc::c_int;
    while i < 128 as libc::c_int {
        let mut n: libc::c_int = 0 as libc::c_int;
        let mut v: libc::c_int = 0 as libc::c_int;
        while s < es && *s as libc::c_int == ' ' as i32 {
            s = s.offset(1);
            s;
        }
        while s < es && *s as libc::c_int >= '0' as i32
            && *s as libc::c_int <= '9' as i32
        {
            v
                |= help_ckd_mul(
                    &mut n as *mut libc::c_int as *mut libc::c_void,
                    n as uintmax,
                    10 as libc::c_int as uintmax,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
                    (((if 1 as libc::c_int != 0 { 0 as libc::c_int } else { n })
                        - 1 as libc::c_int) < 0 as libc::c_int) as libc::c_int,
                );
            v
                |= help_ckd_add(
                    &mut n as *mut libc::c_int as *mut libc::c_void,
                    n as uintmax,
                    (*s as libc::c_int - '0' as i32) as uintmax,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
                    (((if 1 as libc::c_int != 0 { 0 as libc::c_int } else { n })
                        - 1 as libc::c_int) < 0 as libc::c_int) as libc::c_int,
                );
            s = s.offset(1);
            s;
        }
        if v == 0 && n > *tabstops.as_mut_ptr().offset((i - 1 as libc::c_int) as isize) {
            let fresh0 = i;
            i = i + 1;
            *tabstops.as_mut_ptr().offset(fresh0 as isize) = n;
        }
        while s < es && *s as libc::c_int == ' ' as i32 {
            s = s.offset(1);
            s;
        }
        if s == es
            || {
                let fresh1 = s;
                s = s.offset(1);
                *fresh1 as libc::c_int != ',' as i32
            }
        {
            break;
        }
    }
    if i < 2 as libc::c_int {
        return;
    }
    ntabstops = i;
    tabdefault = *tabstops.as_mut_ptr().offset((ntabstops - 1 as libc::c_int) as isize)
        - *tabstops.as_mut_ptr().offset((ntabstops - 2 as libc::c_int) as isize);
}
pub unsafe extern "C" fn opt_x(mut type_0: libc::c_int, mut s: *mut libc::c_char) {
    let mut msg: [libc::c_char; 1596] = [0; 1596];
    let mut i: libc::c_int = 0;
    let mut p: PARG = parg {
        p_string: 0 as *mut libc::c_char,
    };
    match type_0 {
        0 | 2 => {
            set_tabs(s, strlen(s) as libc::c_int);
        }
        1 => {
            strcpy(
                msg.as_mut_ptr(),
                b"Tab stops \0" as *const u8 as *const libc::c_char,
            );
            if ntabstops > 2 as libc::c_int {
                i = 1 as libc::c_int;
                while i < ntabstops {
                    if i > 1 as libc::c_int {
                        strcat(
                            msg.as_mut_ptr(),
                            b",\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    sprintf(
                        msg.as_mut_ptr().offset(strlen(msg.as_mut_ptr()) as isize),
                        b"%d\0" as *const u8 as *const libc::c_char,
                        *tabstops.as_mut_ptr().offset(i as isize),
                    );
                    i += 1;
                    i;
                }
                sprintf(
                    msg.as_mut_ptr().offset(strlen(msg.as_mut_ptr()) as isize),
                    b" and then \0" as *const u8 as *const libc::c_char,
                );
            }
            sprintf(
                msg.as_mut_ptr().offset(strlen(msg.as_mut_ptr()) as isize),
                b"every %d spaces\0" as *const u8 as *const libc::c_char,
                tabdefault,
            );
            p.p_string = msg.as_mut_ptr();
            error(
                b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                &mut p,
            );
        }
        _ => {}
    };
}
pub unsafe extern "C" fn opt_quote(mut type_0: libc::c_int, mut s: *mut libc::c_char) {
    let mut buf: [libc::c_char; 3] = [0; 3];
    let mut parg: PARG = parg {
        p_string: 0 as *mut libc::c_char,
    };
    match type_0 {
        0 | 2 => {
            if *s.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
                closequote = '\0' as i32 as libc::c_char;
                openquote = closequote;
            } else {
                if *s.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32
                    && *s.offset(2 as libc::c_int as isize) as libc::c_int != '\0' as i32
                {
                    error(
                        b"-\" must be followed by 1 or 2 chars\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        0 as *mut libc::c_void as *mut PARG,
                    );
                    return;
                }
                openquote = *s.offset(0 as libc::c_int as isize);
                if *s.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
                    closequote = openquote;
                } else {
                    closequote = *s.offset(1 as libc::c_int as isize);
                }
            }
        }
        1 => {
            buf[0 as libc::c_int as usize] = openquote;
            buf[1 as libc::c_int as usize] = closequote;
            buf[2 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            parg.p_string = buf.as_mut_ptr();
            error(
                b"quotes %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                &mut parg,
            );
        }
        _ => {}
    };
}
pub unsafe extern "C" fn opt_rscroll(mut type_0: libc::c_int, mut s: *mut libc::c_char) {
    let mut p: PARG = parg {
        p_string: 0 as *mut libc::c_char,
    };
    match type_0 {
        0 | 2 => {
            let mut fmt: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut attr: libc::c_int = (1 as libc::c_int) << 3 as libc::c_int;
            setfmt(
                s,
                &mut fmt,
                &mut attr,
                b"*s>\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
            );
            if strcmp(fmt, b"-\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                rscroll_char = 0 as libc::c_int as libc::c_char;
            } else {
                rscroll_char = (if *fmt as libc::c_int != 0 {
                    *fmt as libc::c_int
                } else {
                    '>' as i32
                }) as libc::c_char;
                rscroll_attr = attr | (8 as libc::c_int) << 8 as libc::c_int;
            }
        }
        1 => {
            p
                .p_string = (if rscroll_char as libc::c_int != 0 {
                prchar(rscroll_char as LWCHAR) as *const libc::c_char
            } else {
                b"-\0" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char;
            error(
                b"rscroll character is %s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                &mut p,
            );
        }
        _ => {}
    };
}
pub unsafe extern "C" fn opt_query(mut type_0: libc::c_int, mut s: *mut libc::c_char) {
    match type_0 {
        1 | 2 => {
            error(
                b"Use \"h\" for help\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_void as *mut PARG,
            );
        }
        0 => {
            dohelp = 1 as libc::c_int;
        }
        _ => {}
    };
}
pub unsafe extern "C" fn opt_mousecap(
    mut type_0: libc::c_int,
    mut s: *mut libc::c_char,
) {
    match type_0 {
        2 => {
            if mousecap == 0 as libc::c_int {
                deinit_mouse();
            } else {
                init_mouse();
            }
        }
        0 | 1 | _ => {}
    };
}
pub unsafe extern "C" fn opt_wheel_lines(
    mut type_0: libc::c_int,
    mut s: *mut libc::c_char,
) {
    match type_0 {
        0 | 2 => {
            if wheel_lines <= 0 as libc::c_int {
                wheel_lines = default_wheel_lines();
            }
        }
        1 | _ => {}
    };
}
pub unsafe extern "C" fn opt_linenum_width(
    mut type_0: libc::c_int,
    mut s: *mut libc::c_char,
) {
    let mut parg: PARG = parg {
        p_string: 0 as *mut libc::c_char,
    };
    match type_0 {
        0 | 2 => {
            if linenum_width > 16 as libc::c_int {
                parg.p_int = 16 as libc::c_int;
                error(
                    b"Line number width must not be larger than %d\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    &mut parg,
                );
                linenum_width = 7 as libc::c_int;
            }
        }
        1 | _ => {}
    };
}
pub unsafe extern "C" fn opt_status_col_width(
    mut type_0: libc::c_int,
    mut s: *mut libc::c_char,
) {
    let mut parg: PARG = parg {
        p_string: 0 as *mut libc::c_char,
    };
    match type_0 {
        0 | 2 => {
            if status_col_width > 4 as libc::c_int {
                parg.p_int = 4 as libc::c_int;
                error(
                    b"Status column width must not be larger than %d\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    &mut parg,
                );
                status_col_width = 2 as libc::c_int;
            }
        }
        1 | _ => {}
    };
}
pub unsafe extern "C" fn opt_filesize(
    mut type_0: libc::c_int,
    mut s: *mut libc::c_char,
) {
    match type_0 {
        0 | 2 => {
            if want_filesize != 0 && !curr_ifile.is_null()
                && ch_length() == -(1 as libc::c_int) as POSITION
            {
                scan_eof();
            }
        }
        1 | _ => {}
    };
}
pub unsafe extern "C" fn opt_intr(mut type_0: libc::c_int, mut s: *mut libc::c_char) {
    let mut p: PARG = parg {
        p_string: 0 as *mut libc::c_char,
    };
    match type_0 {
        0 | 2 => {
            intr_char = *s;
            if intr_char as libc::c_int == '^' as i32
                && *s.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32
            {
                intr_char = (*s.offset(1 as libc::c_int as isize) as libc::c_int
                    & 0o37 as libc::c_int) as libc::c_char;
            }
        }
        1 => {
            p.p_string = prchar(intr_char as LWCHAR);
            error(
                b"interrupt character is %s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                &mut p,
            );
        }
        _ => {}
    };
}
pub unsafe extern "C" fn opt_header(mut type_0: libc::c_int, mut s: *mut libc::c_char) {
    let mut err: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    match type_0 {
        0 | 2 => {
            header_lines = 0 as libc::c_int;
            header_cols = 0 as libc::c_int;
            if *s as libc::c_int != ',' as i32 {
                n = getnum(
                    &mut s,
                    b"header\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    &mut err,
                );
                if err != 0 {
                    error(
                        b"invalid number of lines\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        0 as *mut libc::c_void as *mut PARG,
                    );
                    return;
                }
                header_lines = n;
            }
            if *s as libc::c_int == ',' as i32 {
                s = s.offset(1);
                s;
                n = getnum(
                    &mut s,
                    b"header\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    &mut err,
                );
                if err != 0 {
                    error(
                        b"invalid number of columns\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        0 as *mut libc::c_void as *mut PARG,
                    );
                } else {
                    header_cols = n;
                }
            }
        }
        1 => {
            let mut buf: [libc::c_char; 24] = [0; 24];
            let mut parg: PARG = parg {
                p_string: 0 as *mut libc::c_char,
            };
            snprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong,
                b"%d,%d\0" as *const u8 as *const libc::c_char,
                header_lines,
                header_cols,
            );
            parg.p_string = buf.as_mut_ptr();
            error(
                b"header (lines,columns) is %s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                &mut parg,
            );
        }
        _ => {}
    };
}
pub unsafe extern "C" fn opt_search_type(
    mut type_0: libc::c_int,
    mut s: *mut libc::c_char,
) {
    let mut st: libc::c_int = 0;
    let mut parg: PARG = parg {
        p_string: 0 as *mut libc::c_char,
    };
    let mut buf: [libc::c_char; 16] = [0; 16];
    let mut bp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    match type_0 {
        0 | 2 => {
            st = 0 as libc::c_int;
            while *s as libc::c_int != '\0' as i32 {
                match *s as libc::c_int {
                    69 | 101 | 5 => {
                        st |= (1 as libc::c_int) << 9 as libc::c_int;
                    }
                    70 | 102 | 6 => {
                        st |= (1 as libc::c_int) << 10 as libc::c_int;
                    }
                    75 | 107 | 11 => {
                        st |= (1 as libc::c_int) << 2 as libc::c_int;
                    }
                    78 | 110 | 14 => {
                        st |= (1 as libc::c_int) << 8 as libc::c_int;
                    }
                    82 | 114 | 18 => {
                        st |= (1 as libc::c_int) << 12 as libc::c_int;
                    }
                    87 | 119 | 23 => {
                        st |= (1 as libc::c_int) << 15 as libc::c_int;
                    }
                    45 => {
                        st = 0 as libc::c_int;
                    }
                    94 => {}
                    _ => {
                        if *s as libc::c_int >= '1' as i32
                            && *s as libc::c_int
                                <= '0' as i32
                                    + (16 as libc::c_int - 10 as libc::c_int - 1 as libc::c_int)
                        {
                            st
                                |= (1 as libc::c_int)
                                    << 16 as libc::c_int + (*s as libc::c_int - '0' as i32);
                        } else {
                            parg.p_char = *s;
                            error(
                                b"invalid search option '%c'\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                                &mut parg,
                            );
                            return;
                        }
                    }
                }
                s = s.offset(1);
                s;
            }
            def_search_type = norm_search_type(st);
        }
        1 => {
            bp = buf.as_mut_ptr();
            if def_search_type & (1 as libc::c_int) << 9 as libc::c_int != 0 {
                let fresh2 = bp;
                bp = bp.offset(1);
                *fresh2 = 'E' as i32 as libc::c_char;
            }
            if def_search_type & (1 as libc::c_int) << 10 as libc::c_int != 0 {
                let fresh3 = bp;
                bp = bp.offset(1);
                *fresh3 = 'F' as i32 as libc::c_char;
            }
            if def_search_type & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                let fresh4 = bp;
                bp = bp.offset(1);
                *fresh4 = 'K' as i32 as libc::c_char;
            }
            if def_search_type & (1 as libc::c_int) << 8 as libc::c_int != 0 {
                let fresh5 = bp;
                bp = bp.offset(1);
                *fresh5 = 'N' as i32 as libc::c_char;
            }
            if def_search_type & (1 as libc::c_int) << 12 as libc::c_int != 0 {
                let fresh6 = bp;
                bp = bp.offset(1);
                *fresh6 = 'R' as i32 as libc::c_char;
            }
            if def_search_type & (1 as libc::c_int) << 15 as libc::c_int != 0 {
                let fresh7 = bp;
                bp = bp.offset(1);
                *fresh7 = 'W' as i32 as libc::c_char;
            }
            i = 1 as libc::c_int;
            while i <= 16 as libc::c_int - 10 as libc::c_int - 1 as libc::c_int {
                if def_search_type & (1 as libc::c_int) << 16 as libc::c_int + i != 0 {
                    let fresh8 = bp;
                    bp = bp.offset(1);
                    *fresh8 = ('0' as i32 + i) as libc::c_char;
                }
                i += 1;
                i;
            }
            if bp == buf.as_mut_ptr() {
                let fresh9 = bp;
                bp = bp.offset(1);
                *fresh9 = '-' as i32 as libc::c_char;
            }
            *bp = '\0' as i32 as libc::c_char;
            parg.p_string = buf.as_mut_ptr();
            error(
                b"search options: %s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                &mut parg,
            );
        }
        _ => {}
    };
}
pub unsafe extern "C" fn chop_line() -> libc::c_int {
    return (chopline != 0 || header_cols > 0 as libc::c_int
        || header_lines > 0 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn get_swindow() -> libc::c_int {
    if swindow > 0 as libc::c_int {
        return swindow;
    }
    return sc_height - header_lines + swindow;
}
