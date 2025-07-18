use ::libc;
extern "C" {
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn save(s: *const libc::c_char) -> *mut libc::c_char;
    fn ecalloc(count: libc::c_int, size: libc::c_uint) -> *mut libc::c_void;
    fn quit(status: libc::c_int);
    fn check_winch();
    fn bell();
    fn clear();
    fn clear_eol();
    fn clear_bot();
    fn at_enter(attr: libc::c_int);
    fn at_exit();
    fn match_brac(
        obrac: libc::c_char,
        cbrac: libc::c_char,
        forwdir: libc::c_int,
        n: libc::c_int,
    );
    fn ch_length() -> POSITION;
    fn ch_flush();
    fn ch_set_eof();
    fn ch_getflags() -> libc::c_int;
    fn cmd_reset();
    fn clear_cmd();
    fn cmd_putstr(s: *const libc::c_char);
    fn len_cmdbuf() -> libc::c_int;
    fn cmd_repaint(old_cp: *const libc::c_char);
    fn set_mlist(mlist: *mut libc::c_void, cmdflags: libc::c_int);
    fn cmd_accept();
    fn cmd_char(c: libc::c_int) -> libc::c_int;
    fn cmd_int(frac: *mut libc::c_long) -> LINENUM;
    fn get_cmdbuf() -> *mut libc::c_char;
    fn error(fmt: *mut libc::c_char, parg: *mut PARG);
    fn getchr() -> libc::c_int;
    fn rrshift() -> libc::c_int;
    fn badmark(c: LWCHAR) -> libc::c_int;
    fn gomark(c: LWCHAR);
    fn flush();
    fn clear_attn();
    fn repaint();
    fn clrmark(c: LWCHAR);
    fn setmark(c: LWCHAR, where_0: libc::c_int);
    fn opt_toggle_disallowed(c: libc::c_int) -> *mut libc::c_char;
    fn del_ifile(h: *mut libc::c_void);
    fn reedit_ifile(save_ifile: *mut libc::c_void);
    fn edit_ifile(ifile: *mut libc::c_void) -> libc::c_int;
    fn getoff_ifile(ifile: *mut libc::c_void) -> *mut libc::c_void;
    fn edit_index(n: libc::c_int) -> libc::c_int;
    fn tagsearch() -> POSITION;
    fn jump_loc(pos: POSITION, sline: libc::c_int);
    fn edit(filename: *mut libc::c_char) -> libc::c_int;
    fn prevtag(n: libc::c_int) -> *mut libc::c_char;
    fn nexttag(n: libc::c_int) -> *mut libc::c_char;
    fn edit_prev(n: libc::c_int) -> libc::c_int;
    fn ntags() -> libc::c_int;
    fn eof_displayed() -> libc::c_int;
    fn get_quit_at_eof() -> libc::c_int;
    fn edit_next(n: libc::c_int) -> libc::c_int;
    fn pr_expand(proto: *const libc::c_char) -> *mut libc::c_char;
    fn lsystem(cmd: *mut libc::c_char, donemsg: *mut libc::c_char);
    fn jump_forw();
    fn reopen_curr_ifile();
    fn empty_screen() -> libc::c_int;
    fn get_altfilename(ifile: *mut libc::c_void) -> *mut libc::c_char;
    fn get_filename(ifile: *mut libc::c_void) -> *mut libc::c_char;
    fn undo_search(clear_0: libc::c_int);
    fn unsave_ifile(save_ifile: *mut libc::c_void);
    fn search(
        search_type_0: libc::c_int,
        pattern: *mut libc::c_char,
        n: libc::c_int,
    ) -> libc::c_int;
    fn edit_last() -> libc::c_int;
    fn edit_first() -> libc::c_int;
    fn save_curr_ifile() -> *mut libc::c_void;
    fn eq_message() -> *mut libc::c_char;
    fn jump_line_loc(pos: POSITION, sline: libc::c_int);
    fn jump_back(linenum: LINENUM);
    fn jump_forw_buffered();
    fn jump_percent(percent: libc::c_int, fraction_0: libc::c_long);
    fn clr_hilite();
    fn clr_linenum();
    fn backward(n: libc::c_int, force: libc::c_int, only_last: libc::c_int);
    fn forward(n: libc::c_int, force: libc::c_int, only_last: libc::c_int);
    fn set_attnpos(pos: POSITION);
    fn get_swindow() -> libc::c_int;
    fn fcmd_decode(cmd: *mut libc::c_char, sp: *mut *mut libc::c_char) -> libc::c_int;
    fn pipe_mark(c: libc::c_int, cmd: *mut libc::c_char) -> libc::c_int;
    fn fexpand(s: *mut libc::c_char) -> *mut libc::c_char;
    fn cleantags();
    fn edit_list(filelist: *mut libc::c_char) -> libc::c_int;
    fn toggle_option(
        o: *mut loption,
        lower: libc::c_int,
        s: *mut libc::c_char,
        how_toggle: libc::c_int,
    );
    fn psignals();
    fn set_filter_pattern(pattern: *mut libc::c_char, search_type_0: libc::c_int);
    fn position(sindex: libc::c_int) -> POSITION;
    fn opt_prompt(o: *mut loption) -> *mut libc::c_char;
    fn opt_has_param(o: *mut loption) -> libc::c_int;
    fn propt(c: libc::c_int) -> *mut libc::c_char;
    fn findopt(c: libc::c_int) -> *mut loption;
    fn entire_file_displayed() -> libc::c_int;
    fn next_ifile(h: *mut libc::c_void) -> *mut libc::c_void;
    fn findopt_name(
        p_optname: *mut *mut libc::c_char,
        p_oname: *mut *mut libc::c_char,
        p_err: *mut libc::c_int,
    ) -> *mut loption;
    fn pr_string() -> *mut libc::c_char;
    fn is_filtering() -> libc::c_int;
    fn putstr(s: *const libc::c_char);
    fn putchr(c: libc::c_int) -> libc::c_int;
    fn editchar(c: libc::c_int, flags: libc::c_int) -> libc::c_int;
    fn load_line(str: *const libc::c_char);
    fn put_line();
    fn free(__ptr: *mut libc::c_void);
    static mut erase_char: libc::c_int;
    static mut erase2_char: libc::c_int;
    static mut kill_char: libc::c_int;
    static mut sigs: libc::c_int;
    static mut quit_if_one_screen: libc::c_int;
    static mut one_screen: libc::c_int;
    static mut sc_width: libc::c_int;
    static mut sc_height: libc::c_int;
    static mut kent: *mut libc::c_char;
    static mut swindow: libc::c_int;
    static mut jump_sline: libc::c_int;
    static mut quitting: libc::c_int;
    static mut wscroll: libc::c_int;
    static mut top_scroll: libc::c_int;
    static mut ignore_eoi: libc::c_int;
    static mut secure: libc::c_int;
    static mut hshift: libc::c_int;
    static mut bs_mode: libc::c_int;
    static mut proc_backspace: libc::c_int;
    static mut show_attn: libc::c_int;
    static mut highest_hilite: POSITION;
    static mut every_first_cmd: *mut libc::c_char;
    static mut version: [libc::c_char; 0];
    static mut initial_scrpos: scrpos;
    static mut curr_ifile: *mut libc::c_void;
    static mut ml_search: *mut libc::c_void;
    static mut ml_examine: *mut libc::c_void;
    static mut wheel_lines: libc::c_int;
    static mut def_search_type: libc::c_int;
    static mut updown_match: libc::c_int;
    static mut ml_shell: *mut libc::c_void;
    static mut editproto: *mut libc::c_char;
    static mut screen_trashed: libc::c_int;
    static mut shift_count: libc::c_int;
    static mut forw_prompt: libc::c_int;
    static mut incr_search: libc::c_int;
    static mut full_screen: libc::c_int;
}
pub type __off_t = libc::c_long;
pub type off_t = __off_t;
pub type LWCHAR = libc::c_ulong;
pub type POSITION = off_t;
pub type LINENUM = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scrpos {
    pub pos: POSITION,
    pub ln: libc::c_int,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ungot {
    pub ug_next: *mut ungot,
    pub ug_char: LWCHAR,
}
static mut shellcmd: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut mca: libc::c_int = 0;
static mut search_type: libc::c_int = 0;
static mut last_search_type: libc::c_int = 0;
static mut number: LINENUM = 0;
static mut fraction: libc::c_long = 0;
static mut curropt: *mut loption = 0 as *const loption as *mut loption;
static mut opt_lower: libc::c_int = 0;
static mut optflag: libc::c_int = 0;
static mut optgetname: libc::c_int = 0;
static mut bottompos: POSITION = 0;
static mut save_hshift: libc::c_int = 0;
static mut save_bs_mode: libc::c_int = 0;
static mut save_proc_backspace: libc::c_int = 0;
static mut pipec: libc::c_char = 0;
static mut ungot: *mut ungot = 0 as *const ungot as *mut ungot;
unsafe extern "C" fn cmd_exec() {
    clear_attn();
    clear_bot();
    flush();
}
unsafe extern "C" fn set_mca(mut action: libc::c_int) {
    mca = action;
    clear_bot();
    clear_cmd();
}
unsafe extern "C" fn clear_mca() {
    if mca == 0 as libc::c_int {
        return;
    }
    mca = 0 as libc::c_int;
}
unsafe extern "C" fn start_mca(
    mut action: libc::c_int,
    mut prompt_0: *const libc::c_char,
    mut mlist: *mut libc::c_void,
    mut cmdflags: libc::c_int,
) {
    set_mca(action);
    cmd_putstr(prompt_0);
    set_mlist(mlist, cmdflags);
}
pub unsafe extern "C" fn in_mca() -> libc::c_int {
    return (mca != 0 as libc::c_int && mca != 105 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn mca_search1() {
    let mut i: libc::c_int = 0;
    if search_type & (1 as libc::c_int) << 13 as libc::c_int != 0 {
        set_mca(55 as libc::c_int);
    } else if search_type & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        set_mca(15 as libc::c_int);
    } else {
        set_mca(5 as libc::c_int);
    }
    if search_type & (1 as libc::c_int) << 8 as libc::c_int != 0 {
        cmd_putstr(b"Non-match \0" as *const u8 as *const libc::c_char);
    }
    if search_type & (1 as libc::c_int) << 10 as libc::c_int != 0 {
        cmd_putstr(b"First-file \0" as *const u8 as *const libc::c_char);
    }
    if search_type & (1 as libc::c_int) << 9 as libc::c_int != 0 {
        cmd_putstr(b"EOF-ignore \0" as *const u8 as *const libc::c_char);
    }
    if search_type & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        cmd_putstr(b"Keep-pos \0" as *const u8 as *const libc::c_char);
    }
    if search_type & (1 as libc::c_int) << 12 as libc::c_int != 0 {
        cmd_putstr(b"Regex-off \0" as *const u8 as *const libc::c_char);
    }
    if search_type & (1 as libc::c_int) << 15 as libc::c_int != 0 {
        cmd_putstr(b"Wrap \0" as *const u8 as *const libc::c_char);
    }
    i = 1 as libc::c_int;
    while i <= 16 as libc::c_int - 10 as libc::c_int - 1 as libc::c_int {
        if search_type & (1 as libc::c_int) << 16 as libc::c_int + i != 0 {
            let mut buf: [libc::c_char; 8] = [0; 8];
            snprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                b"Sub-%d \0" as *const u8 as *const libc::c_char,
                i,
            );
            cmd_putstr(buf.as_mut_ptr());
        }
        i += 1;
        i;
    }
    if search_type & (1 as libc::c_int) << 13 as libc::c_int != 0 {
        cmd_putstr(b"&/\0" as *const u8 as *const libc::c_char);
    } else if search_type & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        cmd_putstr(b"/\0" as *const u8 as *const libc::c_char);
    } else {
        cmd_putstr(b"?\0" as *const u8 as *const libc::c_char);
    }
    forw_prompt = 0 as libc::c_int;
}
unsafe extern "C" fn mca_search() {
    mca_search1();
    set_mlist(ml_search, 0 as libc::c_int);
}
unsafe extern "C" fn mca_opt_toggle() {
    let mut no_prompt: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    let mut dash: *mut libc::c_char = 0 as *mut libc::c_char;
    no_prompt = optflag & 0o100 as libc::c_int;
    flag = optflag & !(0o100 as libc::c_int);
    dash = (if flag == 0 as libc::c_int {
        b"_\0" as *const u8 as *const libc::c_char
    } else {
        b"-\0" as *const u8 as *const libc::c_char
    }) as *mut libc::c_char;
    set_mca(47 as libc::c_int);
    cmd_putstr(dash);
    if optgetname != 0 {
        cmd_putstr(dash);
    }
    if no_prompt != 0 {
        cmd_putstr(b"(P)\0" as *const u8 as *const libc::c_char);
    }
    match flag {
        2 => {
            cmd_putstr(b"+\0" as *const u8 as *const libc::c_char);
        }
        3 => {
            cmd_putstr(b"!\0" as *const u8 as *const libc::c_char);
        }
        _ => {}
    }
    forw_prompt = 0 as libc::c_int;
    set_mlist(0 as *mut libc::c_void, 0 as libc::c_int);
}
unsafe extern "C" fn exec_mca() {
    let mut cbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    cmd_exec();
    cbuf = get_cmdbuf();
    if cbuf.is_null() {
        return;
    }
    match mca {
        15 | 5 => {
            multi_search(cbuf, number as libc::c_int, 0 as libc::c_int);
        }
        55 => {
            search_type ^= (1 as libc::c_int) << 8 as libc::c_int;
            set_filter_pattern(cbuf, search_type);
        }
        10 => {
            while *cbuf as libc::c_int == '+' as i32
                || *cbuf as libc::c_int == ' ' as i32
            {
                cbuf = cbuf.offset(1);
                cbuf;
            }
            if !every_first_cmd.is_null() {
                free(every_first_cmd as *mut libc::c_void);
            }
            if *cbuf as libc::c_int == '\0' as i32 {
                every_first_cmd = 0 as *mut libc::c_char;
            } else {
                every_first_cmd = save(cbuf);
            }
        }
        47 => {
            toggle_option(curropt, opt_lower, cbuf, optflag);
            curropt = 0 as *mut loption;
        }
        35 => {
            match_brac(
                *cbuf.offset(0 as libc::c_int as isize),
                *cbuf.offset(1 as libc::c_int as isize),
                1 as libc::c_int,
                number as libc::c_int,
            );
        }
        36 => {
            match_brac(
                *cbuf.offset(1 as libc::c_int as isize),
                *cbuf.offset(0 as libc::c_int as isize),
                0 as libc::c_int,
                number as libc::c_int,
            );
        }
        9 => {
            if !(secure != 0) {
                edit_list(cbuf);
                cleantags();
            }
        }
        27 => {
            if *cbuf as libc::c_int != '!' as i32 {
                if !shellcmd.is_null() {
                    free(shellcmd as *mut libc::c_void);
                }
                shellcmd = fexpand(cbuf);
            }
            if !(secure != 0) {
                if shellcmd.is_null() {
                    lsystem(
                        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        b"!done\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                } else {
                    lsystem(
                        shellcmd,
                        b"!done\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                }
            }
        }
        69 => {
            if !(secure != 0) {
                lsystem(
                    pr_expand(cbuf),
                    b"#done\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
        }
        37 => {
            if !(secure != 0) {
                pipe_mark(pipec as libc::c_int, cbuf);
                error(
                    b"|done\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    0 as *mut libc::c_void as *mut PARG,
                );
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn is_erase_char(mut c: libc::c_int) -> libc::c_int {
    return (c == erase_char || c == erase2_char || c == kill_char) as libc::c_int;
}
unsafe extern "C" fn is_newline_char(mut c: libc::c_int) -> libc::c_int {
    return (c == '\n' as i32 || c == '\r' as i32) as libc::c_int;
}
unsafe extern "C" fn mca_opt_first_char(mut c: libc::c_int) -> libc::c_int {
    let mut no_prompt: libc::c_int = optflag & 0o100 as libc::c_int;
    let mut flag: libc::c_int = optflag & !(0o100 as libc::c_int);
    if flag == 0 as libc::c_int {
        match c {
            95 => {
                optgetname = 1 as libc::c_int;
                mca_opt_toggle();
                return 2 as libc::c_int;
            }
            _ => {}
        }
    } else {
        match c {
            43 => {
                optflag = no_prompt
                    | (if flag == 2 as libc::c_int {
                        1 as libc::c_int
                    } else {
                        2 as libc::c_int
                    });
                mca_opt_toggle();
                return 2 as libc::c_int;
            }
            33 => {
                optflag = no_prompt
                    | (if flag == 3 as libc::c_int {
                        1 as libc::c_int
                    } else {
                        3 as libc::c_int
                    });
                mca_opt_toggle();
                return 2 as libc::c_int;
            }
            16 => {
                optflag ^= 0o100 as libc::c_int;
                mca_opt_toggle();
                return 2 as libc::c_int;
            }
            45 => {
                optgetname = 1 as libc::c_int;
                mca_opt_toggle();
                return 2 as libc::c_int;
            }
            _ => {}
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mca_opt_nonfirst_char(mut c: libc::c_int) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut oname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut err: libc::c_int = 0;
    if !curropt.is_null() {
        if is_erase_char(c) != 0 {
            return 1 as libc::c_int;
        }
        return 2 as libc::c_int;
    }
    if cmd_char(c) == 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    p = get_cmdbuf();
    if p.is_null() {
        return 2 as libc::c_int;
    }
    opt_lower = (*p.offset(0 as libc::c_int as isize) as libc::c_int >= 'a' as i32
        && *p.offset(0 as libc::c_int as isize) as libc::c_int <= 'z' as i32)
        as libc::c_int;
    err = 0 as libc::c_int;
    curropt = findopt_name(&mut p, &mut oname, &mut err);
    if !curropt.is_null() {
        cmd_reset();
        mca_opt_toggle();
        p = oname;
        while *p as libc::c_int != '\0' as i32 {
            c = *p as libc::c_int;
            if opt_lower == 0 && (c >= 'a' as i32 && c <= 'z' as i32) {
                c = c - 'a' as i32 + 'A' as i32;
            }
            if cmd_char(c) != 0 as libc::c_int {
                return 1 as libc::c_int;
            }
            p = p.offset(1);
            p;
        }
    } else if err != 1 as libc::c_int {
        bell();
    }
    return 2 as libc::c_int;
}
unsafe extern "C" fn mca_opt_char(mut c: libc::c_int) -> libc::c_int {
    let mut parg: PARG = parg {
        p_string: 0 as *mut libc::c_char,
    };
    if curropt.is_null() && len_cmdbuf() == 0 as libc::c_int {
        let mut ret: libc::c_int = mca_opt_first_char(c);
        if ret != 0 as libc::c_int {
            return ret;
        }
    }
    if optgetname != 0 {
        if is_newline_char(c) == 0 && c != '=' as i32 {
            return mca_opt_nonfirst_char(c);
        }
        if curropt.is_null() {
            parg.p_string = get_cmdbuf();
            if (parg.p_string).is_null() {
                return 2 as libc::c_int;
            }
            error(
                b"There is no --%s option\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                &mut parg,
            );
            return 1 as libc::c_int;
        }
        optgetname = 0 as libc::c_int;
        cmd_reset();
    } else {
        if is_erase_char(c) != 0 {
            return 0 as libc::c_int;
        }
        if !curropt.is_null() {
            return 0 as libc::c_int;
        }
        curropt = findopt(c);
        if curropt.is_null() {
            parg.p_string = propt(c);
            error(
                b"There is no %s option\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                &mut parg,
            );
            return 1 as libc::c_int;
        }
        opt_lower = (c >= 'a' as i32 && c <= 'z' as i32) as libc::c_int;
    }
    if optflag & !(0o100 as libc::c_int) != 1 as libc::c_int
        || opt_has_param(curropt) == 0
    {
        toggle_option(
            curropt,
            opt_lower,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            optflag,
        );
        return 1 as libc::c_int;
    }
    start_mca(
        47 as libc::c_int,
        opt_prompt(curropt),
        0 as *mut libc::c_void,
        0 as libc::c_int,
    );
    return 2 as libc::c_int;
}
pub unsafe extern "C" fn norm_search_type(mut st: libc::c_int) -> libc::c_int {
    if st
        & ((1 as libc::c_int) << 9 as libc::c_int
            | (1 as libc::c_int) << 15 as libc::c_int)
        == (1 as libc::c_int) << 9 as libc::c_int
            | (1 as libc::c_int) << 15 as libc::c_int
    {
        st ^= (1 as libc::c_int) << 9 as libc::c_int;
    }
    return st;
}
unsafe extern "C" fn mca_search_char(mut c: libc::c_int) -> libc::c_int {
    let mut flag: libc::c_int = 0 as libc::c_int;
    if len_cmdbuf() > 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    match c {
        5 | 42 => {
            if mca != 55 as libc::c_int {
                flag = (1 as libc::c_int) << 9 as libc::c_int;
            }
        }
        6 | 64 => {
            if mca != 55 as libc::c_int {
                flag = (1 as libc::c_int) << 10 as libc::c_int;
            }
        }
        11 => {
            if mca != 55 as libc::c_int {
                flag = (1 as libc::c_int) << 2 as libc::c_int;
            }
        }
        19 => {
            let mut buf: [libc::c_char; 32] = [0; 32];
            snprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                b"Sub-pattern (1-%d):\0" as *const u8 as *const libc::c_char,
                16 as libc::c_int - 10 as libc::c_int - 1 as libc::c_int,
            );
            clear_bot();
            cmd_putstr(buf.as_mut_ptr());
            flush();
            c = getcc();
            if c >= '1' as i32
                && c
                    <= '0' as i32
                        + (16 as libc::c_int - 10 as libc::c_int - 1 as libc::c_int)
            {
                flag = (1 as libc::c_int) << 16 as libc::c_int + (c - '0' as i32);
            } else {
                flag = -(1 as libc::c_int);
            }
        }
        23 => {
            if mca != 55 as libc::c_int {
                flag = (1 as libc::c_int) << 15 as libc::c_int;
            }
        }
        18 => {
            flag = (1 as libc::c_int) << 12 as libc::c_int;
        }
        14 | 33 => {
            flag = (1 as libc::c_int) << 8 as libc::c_int;
        }
        _ => {}
    }
    if flag != 0 as libc::c_int {
        if flag != -(1 as libc::c_int) {
            search_type = norm_search_type(search_type ^ flag);
        }
        mca_search();
        return 2 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mca_char(mut c: libc::c_int) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    match mca {
        0 => return 0 as libc::c_int,
        105 => return 0 as libc::c_int,
        6 => {
            if !(c >= '0' as i32 && c <= '9' as i32 || c == '.' as i32) {
                match editchar(
                    c,
                    0o1 as libc::c_int | 0o2 as libc::c_int | 0o4 as libc::c_int
                        | 0o10 as libc::c_int,
                ) {
                    101 => return 2 as libc::c_int,
                    100 => {
                        number = cmd_int(&mut fraction);
                        clear_mca();
                        cmd_accept();
                        return 0 as libc::c_int;
                    }
                    _ => {}
                }
            }
        }
        47 => {
            ret = mca_opt_char(c);
            if ret != 0 as libc::c_int {
                return ret;
            }
        }
        15 | 5 | 55 => {
            ret = mca_search_char(c);
            if ret != 0 as libc::c_int {
                return ret;
            }
        }
        _ => {}
    }
    if is_newline_char(c) != 0 {
        exec_mca();
        return 1 as libc::c_int;
    }
    if cmd_char(c) == 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    match mca {
        35 | 36 => {
            if len_cmdbuf() >= 2 as libc::c_int {
                exec_mca();
                return 1 as libc::c_int;
            }
        }
        15 | 5 => {
            if incr_search != 0 {
                let mut st: libc::c_int = search_type
                    & ((1 as libc::c_int) << 0 as libc::c_int
                        | (1 as libc::c_int) << 1 as libc::c_int
                        | (1 as libc::c_int) << 8 as libc::c_int
                        | (1 as libc::c_int) << 12 as libc::c_int
                        | (1 as libc::c_int) << 2 as libc::c_int
                        | (1 as libc::c_int) << 15 as libc::c_int
                        | ((1 as libc::c_int) << 16 as libc::c_int + 1 as libc::c_int
                            | (1 as libc::c_int) << 16 as libc::c_int + 2 as libc::c_int
                            | (1 as libc::c_int) << 16 as libc::c_int + 3 as libc::c_int
                            | (1 as libc::c_int) << 16 as libc::c_int + 4 as libc::c_int
                            | (1 as libc::c_int)
                                << 16 as libc::c_int + 5 as libc::c_int));
                let mut pattern: *mut libc::c_char = get_cmdbuf();
                if pattern.is_null() {
                    return 2 as libc::c_int;
                }
                let mut save_updown_match: libc::c_int = updown_match;
                cmd_exec();
                if *pattern as libc::c_int == '\0' as i32 {
                    undo_search(1 as libc::c_int);
                } else if search(
                    st | (1 as libc::c_int) << 3 as libc::c_int,
                    pattern,
                    1 as libc::c_int,
                ) != 0 as libc::c_int
                {
                    undo_search(1 as libc::c_int);
                }
                if full_screen == 0 {
                    clear();
                    repaint();
                }
                mca_search1();
                updown_match = save_updown_match;
                cmd_repaint(0 as *const libc::c_char);
            }
        }
        _ => {}
    }
    return 2 as libc::c_int;
}
unsafe extern "C" fn clear_buffers() {
    if ch_getflags() & 0o1 as libc::c_int == 0 {
        return;
    }
    ch_flush();
    clr_linenum();
    clr_hilite();
}
unsafe extern "C" fn make_display() {
    if full_screen == 0 && !(quit_if_one_screen != 0 && one_screen != 0) {
        clear();
    }
    if empty_screen() != 0 {
        if initial_scrpos.pos == -(1 as libc::c_int) as POSITION {
            jump_loc(0 as libc::c_int as POSITION, 1 as libc::c_int);
        } else {
            jump_loc(initial_scrpos.pos, initial_scrpos.ln);
        }
    } else if screen_trashed != 0 || full_screen == 0 {
        let mut save_top_scroll: libc::c_int = top_scroll;
        let mut save_ignore_eoi: libc::c_int = ignore_eoi;
        top_scroll = 1 as libc::c_int;
        ignore_eoi = 0 as libc::c_int;
        if screen_trashed == 2 as libc::c_int {
            reopen_curr_ifile();
            jump_forw();
        }
        repaint();
        top_scroll = save_top_scroll;
        ignore_eoi = save_ignore_eoi;
    }
}
unsafe extern "C" fn prompt() {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if !ungot.is_null() && (*ungot).ug_char != 0x40000000 as libc::c_int as libc::c_ulong
    {
        return;
    }
    make_display();
    bottompos = position(-(2 as libc::c_int));
    if get_quit_at_eof() == 2 as libc::c_int && eof_displayed() != 0
        && ch_getflags() & 0o10 as libc::c_int == 0
        && next_ifile(curr_ifile) == 0 as *mut libc::c_void
    {
        quit(0 as libc::c_int);
    }
    if quit_if_one_screen != 0 && entire_file_displayed() != 0
        && ch_getflags() & 0o10 as libc::c_int == 0
        && next_ifile(curr_ifile) == 0 as *mut libc::c_void
    {
        quit(0 as libc::c_int);
    }
    quit_if_one_screen = 0 as libc::c_int;
    if forw_prompt == 0 {
        clear_bot();
    }
    clear_cmd();
    forw_prompt = 0 as libc::c_int;
    p = pr_string();
    if is_filtering() != 0 {
        putstr(b"& \0" as *const u8 as *const libc::c_char);
    }
    if p.is_null() || *p as libc::c_int == '\0' as i32 {
        at_enter(0 as libc::c_int | (7 as libc::c_int) << 8 as libc::c_int);
        putchr(':' as i32);
        at_exit();
    } else {
        load_line(p);
        put_line();
    }
    clear_eol();
}
pub unsafe extern "C" fn dispversion() {
    let mut parg: PARG = parg {
        p_string: 0 as *mut libc::c_char,
    };
    parg.p_string = version.as_mut_ptr();
    error(
        b"less %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut parg,
    );
}
unsafe extern "C" fn getcc_end_command() -> LWCHAR {
    match mca {
        6 => return 'g' as i32 as LWCHAR,
        15 | 5 | 55 => return '\n' as i32 as LWCHAR,
        _ => return (if ungot.is_null() { getchr() } else { 0 as libc::c_int }) as LWCHAR,
    };
}
unsafe extern "C" fn getccu() -> LWCHAR {
    let mut c: LWCHAR = 0 as libc::c_int as LWCHAR;
    while c == 0 as libc::c_int as libc::c_ulong {
        if ungot.is_null() {
            c = getchr() as LWCHAR;
        } else {
            let mut ug: *mut ungot = ungot;
            c = (*ug).ug_char;
            ungot = (*ug).ug_next;
            free(ug as *mut libc::c_void);
            if c == 0x40000000 as libc::c_int as libc::c_ulong {
                c = getcc_end_command();
            }
        }
    }
    return c;
}
unsafe extern "C" fn getcc_repl(
    mut orig: *const libc::c_char,
    mut repl: *const libc::c_char,
    mut gr_getc: Option::<unsafe extern "C" fn() -> LWCHAR>,
    mut gr_ungetc: Option::<unsafe extern "C" fn(LWCHAR) -> ()>,
) -> LWCHAR {
    let mut c: LWCHAR = 0;
    let mut keys: [LWCHAR; 16] = [0; 16];
    let mut ki: libc::c_int = 0 as libc::c_int;
    c = (Some(gr_getc.unwrap())).unwrap()();
    if orig.is_null()
        || *orig.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return c;
    }
    loop {
        keys[ki as usize] = c;
        if c != *orig.offset(ki as isize) as libc::c_ulong
            || ki as libc::c_ulong
                >= (::std::mem::size_of::<[LWCHAR; 16]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        {
            while ki > 0 as libc::c_int {
                let fresh0 = ki;
                ki = ki - 1;
                (Some(gr_ungetc.unwrap())).unwrap()(keys[fresh0 as usize]);
            }
            return keys[0 as libc::c_int as usize];
        }
        ki += 1;
        if *orig.offset(ki as isize) as libc::c_int == '\0' as i32 {
            ki = (strlen(repl)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                as libc::c_int;
            while ki > 0 as libc::c_int {
                let fresh1 = ki;
                ki = ki - 1;
                (Some(gr_ungetc.unwrap()))
                    .unwrap()(*repl.offset(fresh1 as isize) as LWCHAR);
            }
            return *repl.offset(0 as libc::c_int as isize) as LWCHAR;
        }
        c = (Some(gr_getc.unwrap())).unwrap()();
    };
}
pub unsafe extern "C" fn getcc() -> libc::c_int {
    return getcc_repl(
        kent,
        b"\n\0" as *const u8 as *const libc::c_char,
        Some(getccu as unsafe extern "C" fn() -> LWCHAR),
        Some(ungetcc as unsafe extern "C" fn(LWCHAR) -> ()),
    ) as libc::c_int;
}
pub unsafe extern "C" fn ungetcc(mut c: LWCHAR) {
    let mut ug: *mut ungot = ecalloc(
        1 as libc::c_int,
        ::std::mem::size_of::<ungot>() as libc::c_ulong as libc::c_uint,
    ) as *mut ungot;
    (*ug).ug_char = c;
    (*ug).ug_next = ungot;
    ungot = ug;
}
pub unsafe extern "C" fn ungetcc_back(mut c: LWCHAR) {
    let mut ug: *mut ungot = ecalloc(
        1 as libc::c_int,
        ::std::mem::size_of::<ungot>() as libc::c_ulong as libc::c_uint,
    ) as *mut ungot;
    (*ug).ug_char = c;
    (*ug).ug_next = 0 as *mut ungot;
    if ungot.is_null() {
        ungot = ug;
    } else {
        let mut pu: *mut ungot = 0 as *mut ungot;
        pu = ungot;
        while !((*pu).ug_next).is_null() {
            pu = (*pu).ug_next;
        }
        (*pu).ug_next = ug;
    };
}
pub unsafe extern "C" fn ungetsc(mut s: *mut libc::c_char) {
    while *s as libc::c_int != '\0' as i32 {
        let fresh2 = s;
        s = s.offset(1);
        ungetcc_back(*fresh2 as LWCHAR);
    }
}
pub unsafe extern "C" fn peekcc() -> LWCHAR {
    let mut c: LWCHAR = getcc() as LWCHAR;
    ungetcc(c);
    return c;
}
unsafe extern "C" fn multi_search(
    mut pattern: *mut libc::c_char,
    mut n: libc::c_int,
    mut silent: libc::c_int,
) {
    let mut nomore: libc::c_int = 0;
    let mut save_ifile: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut changed_file: libc::c_int = 0;
    changed_file = 0 as libc::c_int;
    save_ifile = save_curr_ifile();
    if search_type
        & ((1 as libc::c_int) << 0 as libc::c_int
            | (1 as libc::c_int) << 1 as libc::c_int) == 0 as libc::c_int
    {
        search_type |= (1 as libc::c_int) << 0 as libc::c_int;
    }
    if search_type & (1 as libc::c_int) << 10 as libc::c_int != 0 {
        if search_type & (1 as libc::c_int) << 0 as libc::c_int != 0 {
            nomore = edit_first();
        } else {
            nomore = edit_last();
        }
        if nomore != 0 {
            unsave_ifile(save_ifile);
            return;
        }
        changed_file = 1 as libc::c_int;
        search_type &= !((1 as libc::c_int) << 10 as libc::c_int);
    }
    loop {
        n = search(search_type, pattern, n);
        search_type &= !((1 as libc::c_int) << 2 as libc::c_int);
        last_search_type = search_type;
        if n == 0 as libc::c_int {
            unsave_ifile(save_ifile);
            return;
        }
        if n < 0 as libc::c_int {
            break;
        } else {
            if search_type & (1 as libc::c_int) << 9 as libc::c_int == 0 as libc::c_int {
                break;
            }
            if search_type & (1 as libc::c_int) << 0 as libc::c_int != 0 {
                nomore = edit_next(1 as libc::c_int);
            } else {
                nomore = edit_prev(1 as libc::c_int);
            }
            if nomore != 0 {
                break;
            }
            changed_file = 1 as libc::c_int;
        }
    }
    if n > 0 as libc::c_int && silent == 0 {
        error(
            b"Pattern not found\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *mut libc::c_void as *mut PARG,
        );
    }
    if changed_file != 0 {
        reedit_ifile(save_ifile);
    } else {
        unsave_ifile(save_ifile);
    };
}
unsafe extern "C" fn forw_loop(mut until_hilite: libc::c_int) -> libc::c_int {
    let mut curr_len: POSITION = 0;
    if ch_getflags() & 0o10 as libc::c_int != 0 {
        return 101 as libc::c_int;
    }
    cmd_exec();
    jump_forw_buffered();
    curr_len = ch_length();
    highest_hilite = if until_hilite != 0 {
        curr_len
    } else {
        -(1 as libc::c_int) as POSITION
    };
    ignore_eoi = 1 as libc::c_int;
    while sigs == 0 {
        if until_hilite != 0 && highest_hilite > curr_len {
            bell();
            break;
        } else {
            make_display();
            forward(1 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int);
        }
    }
    ignore_eoi = 0 as libc::c_int;
    ch_set_eof();
    if sigs != 0 && sigs & (0o1 as libc::c_int | 0o2 as libc::c_int) == 0 {
        return if until_hilite != 0 { 56 as libc::c_int } else { 50 as libc::c_int };
    }
    return 101 as libc::c_int;
}
pub unsafe extern "C" fn commands() {
    let mut current_block: u64;
    let mut c: libc::c_int = 0;
    let mut action: libc::c_int = 0;
    let mut cbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newaction: libc::c_int = 0;
    let mut save_jump_sline: libc::c_int = 0;
    let mut save_search_type: libc::c_int = 0;
    let mut extra: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tbuf: [libc::c_char; 2] = [0; 2];
    let mut parg: PARG = parg {
        p_string: 0 as *mut libc::c_char,
    };
    let mut old_ifile: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut new_ifile: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tagfile: *mut libc::c_char = 0 as *mut libc::c_char;
    search_type = (1 as libc::c_int) << 0 as libc::c_int;
    wscroll = (sc_height + 1 as libc::c_int) / 2 as libc::c_int;
    newaction = 101 as libc::c_int;
    's_39: loop {
        clear_mca();
        cmd_accept();
        number = 0 as libc::c_int as LINENUM;
        curropt = 0 as *mut loption;
        if sigs != 0 {
            psignals();
            if quitting != 0 {
                quit(-(1 as libc::c_int));
            }
        }
        check_winch();
        cmd_reset();
        prompt();
        if sigs != 0 {
            continue;
        }
        if newaction == 101 as libc::c_int {
            c = getcc();
        }
        loop {
            if sigs != 0 {
                continue 's_39;
            }
            if newaction != 101 as libc::c_int {
                action = newaction;
                newaction = 101 as libc::c_int;
            } else {
                if mca != 0 {
                    match mca_char(c) {
                        2 => {
                            current_block = 7873764389453924156;
                            match current_block {
                                7873764389453924156 => {
                                    c = getcc();
                                    continue;
                                }
                                _ => {}
                            }
                        }
                        1 => {
                            continue 's_39;
                        }
                        0 => {
                            current_block = 7146806494120673115;
                            match current_block {
                                7873764389453924156 => {
                                    c = getcc();
                                    continue;
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                if mca != 0 {
                    if cmd_char(c) == 1 as libc::c_int
                        || len_cmdbuf() == 0 as libc::c_int
                    {
                        continue 's_39;
                    }
                    cbuf = get_cmdbuf();
                    if cbuf.is_null() {
                        continue 's_39;
                    }
                } else {
                    tbuf[0 as libc::c_int as usize] = c as libc::c_char;
                    tbuf[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                    cbuf = tbuf.as_mut_ptr();
                }
                extra = 0 as *mut libc::c_char;
                action = fcmd_decode(cbuf, &mut extra);
                if !extra.is_null() {
                    ungetsc(extra);
                }
            }
            if action != 105 as libc::c_int {
                cmd_reset();
            }
            match action {
                6 => {
                    start_mca(
                        6 as libc::c_int,
                        b":\0" as *const u8 as *const libc::c_char,
                        0 as *mut libc::c_void,
                        0o1 as libc::c_int,
                    );
                }
                33 => {
                    if number > 0 as libc::c_int as libc::c_long {
                        swindow = number as libc::c_int;
                    }
                    current_block = 12176457224491674537;
                    break;
                }
                13 => {
                    current_block = 12176457224491674537;
                    break;
                }
                34 => {
                    if number > 0 as libc::c_int as libc::c_long {
                        swindow = number as libc::c_int;
                    }
                    current_block = 97719746403090554;
                    break;
                }
                3 => {
                    current_block = 97719746403090554;
                    break;
                }
                12 => {
                    if number <= 0 as libc::c_int as libc::c_long {
                        number = 1 as libc::c_int as LINENUM;
                    }
                    cmd_exec();
                    if show_attn == 2 as libc::c_int
                        && number > 1 as libc::c_int as libc::c_long
                    {
                        set_attnpos(bottompos);
                    }
                    forward(number as libc::c_int, 0 as libc::c_int, 0 as libc::c_int);
                    continue 's_39;
                }
                2 => {
                    if number <= 0 as libc::c_int as libc::c_long {
                        number = 1 as libc::c_int as LINENUM;
                    }
                    cmd_exec();
                    backward(number as libc::c_int, 0 as libc::c_int, 0 as libc::c_int);
                    continue 's_39;
                }
                66 => {
                    cmd_exec();
                    forward(wheel_lines, 0 as libc::c_int, 0 as libc::c_int);
                    continue 's_39;
                }
                67 => {
                    cmd_exec();
                    backward(wheel_lines, 0 as libc::c_int, 0 as libc::c_int);
                    continue 's_39;
                }
                29 => {
                    if number <= 0 as libc::c_int as libc::c_long {
                        number = 1 as libc::c_int as LINENUM;
                    }
                    cmd_exec();
                    if show_attn == 2 as libc::c_int
                        && number > 1 as libc::c_int as libc::c_long
                    {
                        set_attnpos(bottompos);
                    }
                    forward(number as libc::c_int, 1 as libc::c_int, 0 as libc::c_int);
                    continue 's_39;
                }
                30 => {
                    if number <= 0 as libc::c_int as libc::c_long {
                        number = 1 as libc::c_int as LINENUM;
                    }
                    cmd_exec();
                    backward(number as libc::c_int, 1 as libc::c_int, 0 as libc::c_int);
                    continue 's_39;
                }
                40 => {
                    if number <= 0 as libc::c_int as libc::c_long {
                        number = get_swindow() as LINENUM;
                    }
                    cmd_exec();
                    if show_attn == 2 as libc::c_int {
                        set_attnpos(bottompos);
                    }
                    forward(number as libc::c_int, 1 as libc::c_int, 0 as libc::c_int);
                    continue 's_39;
                }
                50 => {
                    if show_attn != 0 {
                        set_attnpos(bottompos);
                    }
                    newaction = forw_loop(0 as libc::c_int);
                    continue 's_39;
                }
                56 => {
                    newaction = forw_loop(1 as libc::c_int);
                    continue 's_39;
                }
                14 => {
                    if number > 0 as libc::c_int as libc::c_long {
                        wscroll = number as libc::c_int;
                    }
                    cmd_exec();
                    if show_attn == 2 as libc::c_int {
                        set_attnpos(bottompos);
                    }
                    forward(wscroll, 0 as libc::c_int, 0 as libc::c_int);
                    continue 's_39;
                }
                4 => {
                    if number > 0 as libc::c_int as libc::c_long {
                        wscroll = number as libc::c_int;
                    }
                    cmd_exec();
                    backward(wscroll, 0 as libc::c_int, 0 as libc::c_int);
                    continue 's_39;
                }
                11 => {
                    clear_buffers();
                    current_block = 7657999535899228935;
                    break;
                }
                25 => {
                    current_block = 7657999535899228935;
                    break;
                }
                17 => {
                    save_jump_sline = jump_sline;
                    if number <= 0 as libc::c_int as libc::c_long {
                        number = 1 as libc::c_int as LINENUM;
                        jump_sline = 0 as libc::c_int;
                    }
                    cmd_exec();
                    jump_back(number);
                    jump_sline = save_jump_sline;
                    continue 's_39;
                }
                21 => {
                    if number < 0 as libc::c_int as libc::c_long {
                        number = 0 as libc::c_int as LINENUM;
                        fraction = 0 as libc::c_int as libc::c_long;
                    }
                    if number > 100 as libc::c_int as libc::c_long
                        || number == 100 as libc::c_int as libc::c_long
                            && fraction != 0 as libc::c_int as libc::c_long
                    {
                        number = 100 as libc::c_int as LINENUM;
                        fraction = 0 as libc::c_int as libc::c_long;
                    }
                    cmd_exec();
                    jump_percent(number as libc::c_int, fraction);
                    continue 's_39;
                }
                16 => {
                    cmd_exec();
                    if number <= 0 as libc::c_int as libc::c_long {
                        jump_forw();
                    } else {
                        jump_back(number);
                    }
                    continue 's_39;
                }
                57 => {
                    cmd_exec();
                    if number <= 0 as libc::c_int as libc::c_long {
                        jump_forw_buffered();
                    } else {
                        jump_back(number);
                    }
                    continue 's_39;
                }
                51 => {
                    cmd_exec();
                    if number < 0 as libc::c_int as libc::c_long {
                        number = 0 as libc::c_int as LINENUM;
                    }
                    jump_line_loc(number, jump_sline);
                    continue 's_39;
                }
                28 => {
                    if ch_getflags() & 0o10 as libc::c_int != 0 {
                        continue 's_39;
                    }
                    cmd_exec();
                    parg.p_string = eq_message();
                    error(
                        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        &mut parg,
                    );
                    continue 's_39;
                }
                31 => {
                    cmd_exec();
                    dispversion();
                    continue 's_39;
                }
                24 => {
                    if curr_ifile != 0 as *mut libc::c_void
                        && ch_getflags() & 0o10 as libc::c_int != 0
                    {
                        current_block = 6988365858197790817;
                        break;
                    } else {
                        current_block = 10945915984064580713;
                        break;
                    }
                }
                15 => {
                    search_type = (1 as libc::c_int) << 0 as libc::c_int
                        | def_search_type;
                    if number <= 0 as libc::c_int as libc::c_long {
                        number = 1 as libc::c_int as LINENUM;
                    }
                    mca_search();
                    c = getcc();
                }
                5 => {
                    search_type = (1 as libc::c_int) << 1 as libc::c_int
                        | def_search_type;
                    if number <= 0 as libc::c_int as libc::c_long {
                        number = 1 as libc::c_int as LINENUM;
                    }
                    mca_search();
                    c = getcc();
                }
                55 => {
                    search_type = (1 as libc::c_int) << 0 as libc::c_int
                        | (1 as libc::c_int) << 13 as libc::c_int;
                    mca_search();
                    c = getcc();
                }
                43 => {
                    search_type = last_search_type;
                    if number <= 0 as libc::c_int as libc::c_long {
                        number = 1 as libc::c_int as LINENUM;
                    }
                    mca_search();
                    cmd_exec();
                    multi_search(
                        0 as *mut libc::c_void as *mut libc::c_char,
                        number as libc::c_int,
                        0 as libc::c_int,
                    );
                    continue 's_39;
                }
                44 => {
                    search_type = last_search_type
                        | (1 as libc::c_int) << 9 as libc::c_int;
                    if number <= 0 as libc::c_int as libc::c_long {
                        number = 1 as libc::c_int as LINENUM;
                    }
                    mca_search();
                    cmd_exec();
                    multi_search(
                        0 as *mut libc::c_void as *mut libc::c_char,
                        number as libc::c_int,
                        0 as libc::c_int,
                    );
                    continue 's_39;
                }
                45 => {
                    search_type = last_search_type;
                    save_search_type = search_type;
                    search_type = if search_type & (1 as libc::c_int) << 0 as libc::c_int
                        != 0
                    {
                        search_type & !((1 as libc::c_int) << 0 as libc::c_int)
                            | (1 as libc::c_int) << 1 as libc::c_int
                    } else {
                        search_type & !((1 as libc::c_int) << 1 as libc::c_int)
                            | (1 as libc::c_int) << 0 as libc::c_int
                    };
                    if number <= 0 as libc::c_int as libc::c_long {
                        number = 1 as libc::c_int as LINENUM;
                    }
                    mca_search();
                    cmd_exec();
                    multi_search(
                        0 as *mut libc::c_void as *mut libc::c_char,
                        number as libc::c_int,
                        0 as libc::c_int,
                    );
                    last_search_type = save_search_type;
                    continue 's_39;
                }
                46 => {
                    search_type = last_search_type;
                    save_search_type = search_type;
                    search_type = (if search_type
                        & (1 as libc::c_int) << 0 as libc::c_int != 0
                    {
                        search_type & !((1 as libc::c_int) << 0 as libc::c_int)
                            | (1 as libc::c_int) << 1 as libc::c_int
                    } else {
                        search_type & !((1 as libc::c_int) << 1 as libc::c_int)
                            | (1 as libc::c_int) << 0 as libc::c_int
                    }) | (1 as libc::c_int) << 9 as libc::c_int;
                    if number <= 0 as libc::c_int as libc::c_long {
                        number = 1 as libc::c_int as LINENUM;
                    }
                    mca_search();
                    cmd_exec();
                    multi_search(
                        0 as *mut libc::c_void as *mut libc::c_char,
                        number as libc::c_int,
                        0 as libc::c_int,
                    );
                    last_search_type = save_search_type;
                    continue 's_39;
                }
                39 | 70 => {
                    undo_search((action == 70 as libc::c_int) as libc::c_int);
                    continue 's_39;
                }
                19 => {
                    if ch_getflags() & 0o10 as libc::c_int != 0 {
                        continue 's_39;
                    }
                    cmd_exec();
                    save_hshift = hshift;
                    hshift = 0 as libc::c_int;
                    save_bs_mode = bs_mode;
                    bs_mode = 0 as libc::c_int;
                    save_proc_backspace = proc_backspace;
                    proc_backspace = 0 as libc::c_int;
                    edit(
                        b"@/\\less/\\help/\\file/\\@\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    continue 's_39;
                }
                9 => {
                    if secure == 0 {
                        start_mca(
                            9 as libc::c_int,
                            b"Examine: \0" as *const u8 as *const libc::c_char,
                            ml_examine,
                            0 as libc::c_int,
                        );
                        c = getcc();
                    } else {
                        error(
                            b"Command not available\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            0 as *mut libc::c_void as *mut PARG,
                        );
                        continue 's_39;
                    }
                }
                32 => {
                    if secure == 0 {
                        current_block = 16107425721173356396;
                        break;
                    } else {
                        current_block = 12717620301112128284;
                        break;
                    }
                }
                20 => {
                    if ntags() != 0 {
                        current_block = 4877859826192283278;
                        break;
                    } else {
                        current_block = 3705522161509601321;
                        break;
                    }
                }
                23 => {
                    if ntags() != 0 {
                        current_block = 11099343707781121639;
                        break;
                    } else {
                        current_block = 13434751124187322381;
                        break;
                    }
                }
                53 => {
                    if number <= 0 as libc::c_int as libc::c_long {
                        number = 1 as libc::c_int as LINENUM;
                    }
                    tagfile = nexttag(number as libc::c_int);
                    if tagfile.is_null() {
                        current_block = 5083741289379115417;
                        break;
                    } else {
                        current_block = 5331008529648877144;
                        break;
                    }
                }
                54 => {
                    if number <= 0 as libc::c_int as libc::c_long {
                        number = 1 as libc::c_int as LINENUM;
                    }
                    tagfile = prevtag(number as libc::c_int);
                    if tagfile.is_null() {
                        current_block = 10376002978872739088;
                        break;
                    } else {
                        current_block = 17664728594743454682;
                        break;
                    }
                }
                38 => {
                    if number <= 0 as libc::c_int as libc::c_long {
                        number = 1 as libc::c_int as LINENUM;
                    }
                    if edit_index(number as libc::c_int) != 0 {
                        error(
                            b"No such file\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            0 as *mut libc::c_void as *mut PARG,
                        );
                    }
                    continue 's_39;
                }
                52 => {
                    if ch_getflags() & 0o10 as libc::c_int != 0 {
                        continue 's_39;
                    }
                    old_ifile = curr_ifile;
                    new_ifile = getoff_ifile(curr_ifile);
                    if new_ifile == 0 as *mut libc::c_void {
                        current_block = 1997090137068572327;
                        break;
                    } else {
                        current_block = 12638391263490919476;
                        break;
                    }
                }
                47 => {
                    optflag = 1 as libc::c_int;
                    optgetname = 0 as libc::c_int;
                    mca_opt_toggle();
                    c = getcc();
                    cbuf = opt_toggle_disallowed(c);
                    if cbuf.is_null() {
                        continue;
                    }
                    error(cbuf, 0 as *mut libc::c_void as *mut PARG);
                    continue 's_39;
                }
                7 => {
                    optflag = 0 as libc::c_int;
                    optgetname = 0 as libc::c_int;
                    mca_opt_toggle();
                    c = getcc();
                }
                10 => {
                    start_mca(
                        10 as libc::c_int,
                        b"+\0" as *const u8 as *const libc::c_char,
                        0 as *mut libc::c_void,
                        0 as libc::c_int,
                    );
                    c = getcc();
                }
                27 | 69 => {
                    if secure == 0 {
                        start_mca(
                            action,
                            if action == 27 as libc::c_int {
                                b"!\0" as *const u8 as *const libc::c_char
                            } else {
                                b"#\0" as *const u8 as *const libc::c_char
                            },
                            ml_shell,
                            0 as libc::c_int,
                        );
                        c = getcc();
                    } else {
                        error(
                            b"Command not available\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            0 as *mut libc::c_void as *mut PARG,
                        );
                        continue 's_39;
                    }
                }
                26 | 63 => {
                    if ch_getflags() & 0o10 as libc::c_int != 0 {
                        continue 's_39;
                    }
                    start_mca(
                        26 as libc::c_int,
                        b"set mark: \0" as *const u8 as *const libc::c_char,
                        0 as *mut libc::c_void,
                        0 as libc::c_int,
                    );
                    c = getcc();
                    if is_erase_char(c) != 0 || is_newline_char(c) != 0 {
                        continue 's_39;
                    }
                    setmark(
                        c as LWCHAR,
                        if action == 63 as libc::c_int {
                            -(1 as libc::c_int)
                        } else {
                            0 as libc::c_int
                        },
                    );
                    repaint();
                    continue 's_39;
                }
                62 => {
                    start_mca(
                        62 as libc::c_int,
                        b"clear mark: \0" as *const u8 as *const libc::c_char,
                        0 as *mut libc::c_void,
                        0 as libc::c_int,
                    );
                    c = getcc();
                    if is_erase_char(c) != 0 || is_newline_char(c) != 0 {
                        continue 's_39;
                    }
                    clrmark(c as LWCHAR);
                    repaint();
                    continue 's_39;
                }
                18 => {
                    start_mca(
                        18 as libc::c_int,
                        b"goto mark: \0" as *const u8 as *const libc::c_char,
                        0 as *mut libc::c_void,
                        0 as libc::c_int,
                    );
                    c = getcc();
                    if is_erase_char(c) != 0 || is_newline_char(c) != 0 {
                        continue 's_39;
                    }
                    cmd_exec();
                    gomark(c as LWCHAR);
                    continue 's_39;
                }
                37 => {
                    if secure == 0 {
                        start_mca(
                            37 as libc::c_int,
                            b"|mark: \0" as *const u8 as *const libc::c_char,
                            0 as *mut libc::c_void,
                            0 as libc::c_int,
                        );
                        c = getcc();
                        if is_erase_char(c) != 0 {
                            continue 's_39;
                        }
                        if is_newline_char(c) != 0 {
                            c = '.' as i32;
                        }
                        if badmark(c as LWCHAR) != 0 {
                            continue 's_39;
                        }
                        pipec = c as libc::c_char;
                        start_mca(
                            37 as libc::c_int,
                            b"!\0" as *const u8 as *const libc::c_char,
                            ml_shell,
                            0 as libc::c_int,
                        );
                        c = getcc();
                    } else {
                        error(
                            b"Command not available\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            0 as *mut libc::c_void as *mut PARG,
                        );
                        continue 's_39;
                    }
                }
                36 | 35 => {
                    start_mca(
                        action,
                        b"Brackets: \0" as *const u8 as *const libc::c_char,
                        0 as *mut libc::c_void,
                        0 as libc::c_int,
                    );
                    c = getcc();
                }
                41 => {
                    if number > 0 as libc::c_int as libc::c_long {
                        shift_count = number as libc::c_int;
                    } else {
                        number = (if shift_count > 0 as libc::c_int {
                            shift_count
                        } else {
                            sc_width / 2 as libc::c_int
                        }) as LINENUM;
                    }
                    if number > hshift as libc::c_long {
                        number = hshift as LINENUM;
                    }
                    hshift = (hshift as libc::c_long - number) as libc::c_int;
                    screen_trashed = 1 as libc::c_int;
                    continue 's_39;
                }
                42 => {
                    if number > 0 as libc::c_int as libc::c_long {
                        shift_count = number as libc::c_int;
                    } else {
                        number = (if shift_count > 0 as libc::c_int {
                            shift_count
                        } else {
                            sc_width / 2 as libc::c_int
                        }) as LINENUM;
                    }
                    hshift = (hshift as libc::c_long + number) as libc::c_int;
                    screen_trashed = 1 as libc::c_int;
                    continue 's_39;
                }
                58 => {
                    hshift = 0 as libc::c_int;
                    screen_trashed = 1 as libc::c_int;
                    continue 's_39;
                }
                59 => {
                    hshift = rrshift();
                    screen_trashed = 1 as libc::c_int;
                    continue 's_39;
                }
                105 => {
                    if mca != 105 as libc::c_int {
                        cmd_reset();
                        start_mca(
                            105 as libc::c_int,
                            b" \0" as *const u8 as *const libc::c_char,
                            0 as *mut libc::c_void,
                            0o1 as libc::c_int,
                        );
                        cmd_char(c);
                    }
                    c = getcc();
                }
                101 => {
                    continue 's_39;
                }
                _ => {
                    bell();
                    continue 's_39;
                }
            }
        }
        match current_block {
            16107425721173356396 => {
                if ch_getflags() & 0o10 as libc::c_int != 0 {
                    continue;
                }
                if strcmp(
                    get_filename(curr_ifile),
                    b"-\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    error(
                        b"Cannot edit standard input\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        0 as *mut libc::c_void as *mut PARG,
                    );
                    continue;
                } else {
                    if !(get_altfilename(curr_ifile)).is_null() {
                        error(
                            b"WARNING: This file was viewed via LESSOPEN\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            0 as *mut libc::c_void as *mut PARG,
                        );
                    }
                    start_mca(
                        27 as libc::c_int,
                        b"!\0" as *const u8 as *const libc::c_char,
                        ml_shell,
                        0 as libc::c_int,
                    );
                    make_display();
                    cmd_exec();
                    lsystem(
                        pr_expand(editproto),
                        0 as *mut libc::c_void as *mut libc::c_char,
                    );
                    continue;
                }
            }
            3705522161509601321 => {
                if number <= 0 as libc::c_int as libc::c_long {
                    number = 1 as libc::c_int as LINENUM;
                }
                if edit_next(number as libc::c_int) != 0 {
                    if get_quit_at_eof() != 0 && eof_displayed() != 0
                        && ch_getflags() & 0o10 as libc::c_int == 0
                    {
                        quit(0 as libc::c_int);
                    }
                    parg
                        .p_string = (if number > 1 as libc::c_int as libc::c_long {
                        b"(N-th) \0" as *const u8 as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    }) as *mut libc::c_char;
                    error(
                        b"No %snext file\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        &mut parg,
                    );
                }
                continue;
            }
            13434751124187322381 => {
                if number <= 0 as libc::c_int as libc::c_long {
                    number = 1 as libc::c_int as LINENUM;
                }
                if edit_prev(number as libc::c_int) != 0 {
                    parg
                        .p_string = (if number > 1 as libc::c_int as libc::c_long {
                        b"(N-th) \0" as *const u8 as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    }) as *mut libc::c_char;
                    error(
                        b"No %sprevious file\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        &mut parg,
                    );
                }
                continue;
            }
            6988365858197790817 => {
                hshift = save_hshift;
                bs_mode = save_bs_mode;
                proc_backspace = save_proc_backspace;
                if edit_prev(1 as libc::c_int) == 0 as libc::c_int {
                    continue;
                }
            }
            97719746403090554 => {
                if number <= 0 as libc::c_int as libc::c_long {
                    number = get_swindow() as LINENUM;
                }
                cmd_exec();
                backward(number as libc::c_int, 0 as libc::c_int, 1 as libc::c_int);
                continue;
            }
            5331008529648877144 => {
                cmd_exec();
                if edit(tagfile) == 0 as libc::c_int {
                    let mut pos: POSITION = tagsearch();
                    if pos != -(1 as libc::c_int) as POSITION {
                        jump_loc(pos, jump_sline);
                    }
                }
                continue;
            }
            12176457224491674537 => {
                if number <= 0 as libc::c_int as libc::c_long {
                    number = get_swindow() as LINENUM;
                }
                cmd_exec();
                if show_attn != 0 {
                    set_attnpos(bottompos);
                }
                forward(number as libc::c_int, 0 as libc::c_int, 1 as libc::c_int);
                continue;
            }
            12638391263490919476 => {
                if edit_ifile(new_ifile) != 0 as libc::c_int {
                    reedit_ifile(old_ifile);
                    continue;
                } else {
                    del_ifile(old_ifile);
                    continue;
                }
            }
            7657999535899228935 => {
                cmd_exec();
                repaint();
                continue;
            }
            17664728594743454682 => {
                cmd_exec();
                if edit(tagfile) == 0 as libc::c_int {
                    let mut pos_0: POSITION = tagsearch();
                    if pos_0 != -(1 as libc::c_int) as POSITION {
                        jump_loc(pos_0, jump_sline);
                    }
                }
                continue;
            }
            12717620301112128284 => {
                error(
                    b"Command not available\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *mut libc::c_void as *mut PARG,
                );
                continue;
            }
            4877859826192283278 => {
                error(
                    b"No next file\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *mut libc::c_void as *mut PARG,
                );
                continue;
            }
            11099343707781121639 => {
                error(
                    b"No previous file\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *mut libc::c_void as *mut PARG,
                );
                continue;
            }
            5083741289379115417 => {
                error(
                    b"No next tag\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *mut libc::c_void as *mut PARG,
                );
                continue;
            }
            1997090137068572327 => {
                bell();
                continue;
            }
            10376002978872739088 => {
                error(
                    b"No previous tag\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *mut libc::c_void as *mut PARG,
                );
                continue;
            }
            _ => {}
        }
        if !extra.is_null() {
            quit(*extra as libc::c_int);
        }
        quit(0 as libc::c_int);
    };
}
