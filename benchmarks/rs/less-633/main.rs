use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn close_getchr();
    fn raw_mode(on: libc::c_int);
    fn save_cmdhist();
    fn edit(filename: *mut libc::c_char) -> libc::c_int;
    fn flush();
    fn repaint();
    fn deinit();
    fn clear_bot();
    fn interactive() -> libc::c_int;
    fn check_altpipe_error();
    fn error(fmt: *mut libc::c_char, parg: *mut PARG);
    fn get_term();
    fn init();
    fn init_charset();
    fn init_cmdhist();
    fn commands();
    fn expand_cmd_tables();
    fn init_cmds();
    fn lgetenv(var: *mut libc::c_char) -> *mut libc::c_char;
    fn isnullenv(s: *mut libc::c_char) -> libc::c_int;
    fn edit_first() -> libc::c_int;
    fn edit_next(n: libc::c_int) -> libc::c_int;
    fn cat_file();
    fn last_component(name: *mut libc::c_char) -> *mut libc::c_char;
    fn get_one_screen() -> libc::c_int;
    fn prev_ifile(h: *mut libc::c_void) -> *mut libc::c_void;
    fn nifile() -> libc::c_int;
    fn get_ifile(
        filename: *mut libc::c_char,
        prev: *mut libc::c_void,
    ) -> *mut libc::c_void;
    fn init_line();
    fn init_mark();
    fn scan_option(s: *mut libc::c_char);
    fn isoptpending() -> libc::c_int;
    fn nopendopt();
    fn init_option();
    fn init_poll();
    fn set_output(fd: libc::c_int);
    fn putchr(c: libc::c_int) -> libc::c_int;
    fn less_printf(fmt: *mut libc::c_char, parg: *mut PARG) -> libc::c_int;
    fn get_return();
    fn init_prompt();
    fn init_search();
    fn init_signals(on: libc::c_int);
    fn findtag(tag: *mut libc::c_char);
    fn tagsearch() -> POSITION;
    fn edit_tagfile() -> libc::c_int;
    fn open_getchr();
    static mut tags: *mut libc::c_char;
    static mut tagoption: *mut libc::c_char;
    static mut jump_sline: libc::c_int;
    static mut less_is_more: libc::c_int;
    static mut missing_cap: libc::c_int;
    static mut know_dumb: libc::c_int;
    static mut quit_if_one_screen: libc::c_int;
    static mut no_init: libc::c_int;
    static mut errmsgs: libc::c_int;
    static mut redraw_on_quit: libc::c_int;
    static mut term_init_done: libc::c_int;
    static mut first_time: libc::c_int;
}
pub type __off_t = libc::c_long;
pub type off_t = __off_t;
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
pub static mut every_first_cmd: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut new_file: libc::c_int = 0;
pub static mut is_tty: libc::c_int = 0;
pub static mut curr_ifile: *mut libc::c_void = 0 as *const libc::c_void
    as *mut libc::c_void;
pub static mut old_ifile: *mut libc::c_void = 0 as *const libc::c_void
    as *mut libc::c_void;
pub static mut initial_scrpos: scrpos = scrpos { pos: 0, ln: 0 };
pub static mut start_attnpos: POSITION = -(1 as libc::c_int) as POSITION;
pub static mut end_attnpos: POSITION = -(1 as libc::c_int) as POSITION;
pub static mut wscroll: libc::c_int = 0;
pub static mut progname: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut quitting: libc::c_int = 0;
pub static mut secure: libc::c_int = 0;
pub static mut dohelp: libc::c_int = 0;
pub static mut logfile: libc::c_int = -(1 as libc::c_int);
pub static mut force_logfile: libc::c_int = 0 as libc::c_int;
pub static mut namelogfile: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut editor: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut editproto: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut one_screen: libc::c_int = 0;
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut ifile: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let fresh0 = argv;
    argv = argv.offset(1);
    progname = *fresh0;
    argc -= 1;
    argc;
    secure = 0 as libc::c_int;
    s = lgetenv(
        b"LESSSECURE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if isnullenv(s) == 0 {
        secure = 1 as libc::c_int;
    }
    is_tty = isatty(1 as libc::c_int);
    init_mark();
    init_cmds();
    init_poll();
    get_term();
    init_charset();
    init_line();
    init_cmdhist();
    init_option();
    init_search();
    s = last_component(progname);
    if strcmp(s, b"more\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        less_is_more = 1 as libc::c_int;
    }
    init_prompt();
    s = lgetenv(
        (if less_is_more != 0 {
            b"MORE\0" as *const u8 as *const libc::c_char
        } else {
            b"LESS\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
    );
    if !s.is_null() {
        scan_option(s);
    }
    while argc > 0 as libc::c_int
        && ((*(*argv).offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
            || *(*argv).offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32)
            && *(*argv).offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32
            || isoptpending() != 0)
    {
        let fresh1 = argv;
        argv = argv.offset(1);
        s = *fresh1;
        argc -= 1;
        argc;
        if strcmp(s, b"--\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            break;
        }
        scan_option(s);
    }
    if isoptpending() != 0 {
        nopendopt();
        quit(0 as libc::c_int);
    }
    expand_cmd_tables();
    editor = lgetenv(
        b"VISUAL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if editor.is_null() || *editor as libc::c_int == '\0' as i32 {
        editor = lgetenv(
            b"EDITOR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if isnullenv(editor) != 0 {
            editor = b"vi\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    }
    editproto = lgetenv(
        b"LESSEDIT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if isnullenv(editproto) != 0 {
        editproto = b"%E ?lm+%lm. %g\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    ifile = 0 as *mut libc::c_void;
    if dohelp != 0 {
        ifile = get_ifile(
            b"@/\\less/\\help/\\file/\\@\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ifile,
        );
    }
    loop {
        let fresh2 = argc;
        argc = argc - 1;
        if !(fresh2 > 0 as libc::c_int) {
            break;
        }
        let fresh3 = argv;
        argv = argv.offset(1);
        get_ifile(*fresh3, ifile);
        ifile = prev_ifile(0 as *mut libc::c_void);
    }
    if is_tty == 0 {
        set_output(1 as libc::c_int);
        if edit_first() == 0 as libc::c_int {
            loop {
                cat_file();
                if !(edit_next(1 as libc::c_int) == 0 as libc::c_int) {
                    break;
                }
            }
        }
        quit(0 as libc::c_int);
    }
    if missing_cap != 0 && know_dumb == 0 {
        error(
            b"WARNING: terminal is not fully functional\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_void as *mut PARG,
        );
    }
    open_getchr();
    raw_mode(1 as libc::c_int);
    init_signals(1 as libc::c_int);
    if !tagoption.is_null()
        || strcmp(tags, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        if nifile() > 0 as libc::c_int {
            error(
                b"No filenames allowed with -t option\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                0 as *mut libc::c_void as *mut PARG,
            );
            quit(1 as libc::c_int);
        }
        findtag(tagoption);
        if edit_tagfile() != 0 {
            quit(1 as libc::c_int);
        }
        initial_scrpos.pos = tagsearch();
        if initial_scrpos.pos == -(1 as libc::c_int) as POSITION {
            quit(1 as libc::c_int);
        }
        initial_scrpos.ln = jump_sline;
    } else {
        if edit_first() != 0 {
            quit(1 as libc::c_int);
        }
        if quit_if_one_screen != 0 {
            if nifile() > 1 as libc::c_int {
                quit_if_one_screen = 0 as libc::c_int;
            } else if no_init == 0 {
                one_screen = get_one_screen();
            }
        }
    }
    if errmsgs > 0 as libc::c_int {
        less_printf(
            b"Press RETURN to continue \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *mut libc::c_void as *mut PARG,
        );
        get_return();
        putchr('\n' as i32);
    }
    set_output(1 as libc::c_int);
    init();
    commands();
    quit(0 as libc::c_int);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn save(mut s: *const libc::c_char) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = ecalloc(
        (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_uint,
    ) as *mut libc::c_char;
    strcpy(p, s);
    return p;
}
pub unsafe extern "C" fn out_of_memory() {
    error(
        b"Cannot allocate memory\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        0 as *mut libc::c_void as *mut PARG,
    );
    quit(1 as libc::c_int);
}
pub unsafe extern "C" fn ecalloc(
    mut count: libc::c_int,
    mut size: libc::c_uint,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    p = calloc(count as libc::c_ulong, size as libc::c_ulong);
    if p.is_null() {
        out_of_memory();
    }
    return p;
}
pub unsafe extern "C" fn skipsp(mut s: *mut libc::c_char) -> *mut libc::c_char {
    while *s as libc::c_int == ' ' as i32 || *s as libc::c_int == '\t' as i32 {
        s = s.offset(1);
        s;
    }
    return s;
}
pub unsafe extern "C" fn sprefix(
    mut ps: *mut libc::c_char,
    mut s: *mut libc::c_char,
    mut uppercase: libc::c_int,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut sc: libc::c_int = 0;
    let mut len: libc::c_int = 0 as libc::c_int;
    while *s as libc::c_int != '\0' as i32 {
        c = *ps as libc::c_int;
        if uppercase != 0 {
            if len == 0 as libc::c_int && (c >= 'a' as i32 && c <= 'z' as i32) {
                return -(1 as libc::c_int);
            }
            if c >= 'A' as i32 && c <= 'Z' as i32 {
                c = c - 'A' as i32 + 'a' as i32;
            }
        }
        sc = *s as libc::c_int;
        if len > 0 as libc::c_int && (sc >= 'A' as i32 && sc <= 'Z' as i32) {
            sc = sc - 'A' as i32 + 'a' as i32;
        }
        if c != sc {
            break;
        }
        len += 1;
        len;
        s = s.offset(1);
        s;
        ps = ps.offset(1);
        ps;
    }
    return len;
}
pub unsafe extern "C" fn quit(mut status: libc::c_int) {
    static mut save_status: libc::c_int = 0;
    if status < 0 as libc::c_int {
        status = save_status;
    } else {
        save_status = status;
    }
    quitting = 1 as libc::c_int;
    check_altpipe_error();
    if interactive() != 0 {
        clear_bot();
    }
    deinit();
    flush();
    if redraw_on_quit != 0 && term_init_done != 0 {
        first_time = 1 as libc::c_int;
        repaint();
        flush();
    }
    edit(0 as *mut libc::c_void as *mut libc::c_char);
    save_cmdhist();
    raw_mode(0 as libc::c_int);
    close_getchr();
    exit(status);
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
