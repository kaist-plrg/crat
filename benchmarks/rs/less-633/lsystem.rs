use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn free(__ptr: *mut libc::c_void);
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn save(s: *const libc::c_char) -> *mut libc::c_char;
    fn ecalloc(count: libc::c_int, size: libc::c_uint) -> *mut libc::c_void;
    fn raw_mode(on: libc::c_int);
    fn init();
    fn deinit();
    fn clear_bot();
    fn ch_seek(pos: POSITION) -> libc::c_int;
    fn ch_forw_get() -> libc::c_int;
    fn lgetenv(var: *mut libc::c_char) -> *mut libc::c_char;
    fn edit_ifile(ifile: *mut libc::c_void) -> libc::c_int;
    fn save_curr_ifile() -> *mut libc::c_void;
    fn reedit_ifile(save_ifile: *mut libc::c_void);
    fn shell_quote(s: *mut libc::c_char) -> *mut libc::c_char;
    fn shell_coption() -> *mut libc::c_char;
    fn winch(type_0: libc::c_int);
    fn flush();
    fn putchr(c: libc::c_int) -> libc::c_int;
    fn get_return();
    fn putstr(s: *const libc::c_char);
    fn init_signals(on: libc::c_int);
    fn open_tty() -> libc::c_int;
    fn error(fmt: *mut libc::c_char, parg: *mut PARG);
    fn position(sindex: libc::c_int) -> POSITION;
    fn markpos(c: LWCHAR) -> POSITION;
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    static mut screen_trashed: libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type off_t = __off_t;
pub type size_t = libc::c_ulong;
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
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub unsafe extern "C" fn lsystem(
    mut cmd: *mut libc::c_char,
    mut donemsg: *mut libc::c_char,
) {
    let mut inp: libc::c_int = 0;
    let mut shell: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut save_ifile: *mut libc::c_void = 0 as *mut libc::c_void;
    if *cmd.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
        cmd = cmd.offset(1);
        cmd;
    } else {
        clear_bot();
        putstr(b"!\0" as *const u8 as *const libc::c_char);
        putstr(cmd);
        putstr(b"\n\0" as *const u8 as *const libc::c_char);
    }
    save_ifile = save_curr_ifile();
    edit_ifile(0 as *mut libc::c_void);
    deinit();
    flush();
    raw_mode(0 as libc::c_int);
    init_signals(0 as libc::c_int);
    inp = dup(0 as libc::c_int);
    close(0 as libc::c_int);
    if open_tty() < 0 as libc::c_int {
        dup(inp);
    }
    p = 0 as *mut libc::c_char;
    shell = lgetenv(b"SHELL\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if !shell.is_null() && *shell as libc::c_int != '\0' as i32 {
        if *cmd as libc::c_int == '\0' as i32 {
            p = save(shell);
        } else {
            let mut esccmd: *mut libc::c_char = shell_quote(cmd);
            if !esccmd.is_null() {
                let mut len: libc::c_int = (strlen(shell))
                    .wrapping_add(strlen(esccmd))
                    .wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int;
                p = ecalloc(
                    len,
                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                        as libc::c_uint,
                ) as *mut libc::c_char;
                snprintf(
                    p,
                    len as libc::c_ulong,
                    b"%s %s %s\0" as *const u8 as *const libc::c_char,
                    shell,
                    shell_coption(),
                    esccmd,
                );
                free(esccmd as *mut libc::c_void);
            }
        }
    }
    if p.is_null() {
        if *cmd as libc::c_int == '\0' as i32 {
            p = save(b"sh\0" as *const u8 as *const libc::c_char);
        } else {
            p = save(cmd);
        }
    }
    system(p);
    free(p as *mut libc::c_void);
    close(0 as libc::c_int);
    dup(inp);
    close(inp);
    init_signals(1 as libc::c_int);
    raw_mode(1 as libc::c_int);
    if !donemsg.is_null() {
        putstr(donemsg);
        putstr(b"  (press RETURN)\0" as *const u8 as *const libc::c_char);
        get_return();
        putchr('\n' as i32);
        flush();
    }
    init();
    screen_trashed = 1 as libc::c_int;
    reedit_ifile(save_ifile);
    winch(0 as libc::c_int);
}
pub unsafe extern "C" fn pipe_mark(
    mut c: libc::c_int,
    mut cmd: *mut libc::c_char,
) -> libc::c_int {
    let mut mpos: POSITION = 0;
    let mut tpos: POSITION = 0;
    let mut bpos: POSITION = 0;
    mpos = markpos(c as LWCHAR);
    if mpos == -(1 as libc::c_int) as POSITION {
        return -(1 as libc::c_int);
    }
    tpos = position(0 as libc::c_int);
    if tpos == -(1 as libc::c_int) as POSITION {
        tpos = 0 as libc::c_int as POSITION;
    }
    bpos = position(-(1 as libc::c_int));
    if c == '.' as i32 {
        return pipe_data(cmd, tpos, bpos)
    } else if mpos <= tpos {
        return pipe_data(cmd, mpos, bpos)
    } else if bpos == -(1 as libc::c_int) as POSITION {
        return pipe_data(cmd, tpos, bpos)
    } else {
        return pipe_data(cmd, tpos, mpos)
    };
}
pub unsafe extern "C" fn pipe_data(
    mut cmd: *mut libc::c_char,
    mut spos: POSITION,
    mut epos: POSITION,
) -> libc::c_int {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut c: libc::c_int = 0;
    if ch_seek(spos) != 0 as libc::c_int {
        error(
            b"Cannot seek to start position\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *mut libc::c_void as *mut PARG,
        );
        return -(1 as libc::c_int);
    }
    f = popen(cmd, b"w\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        error(
            b"Cannot create pipe\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *mut libc::c_void as *mut PARG,
        );
        return -(1 as libc::c_int);
    }
    clear_bot();
    putstr(b"!\0" as *const u8 as *const libc::c_char);
    putstr(cmd);
    putstr(b"\n\0" as *const u8 as *const libc::c_char);
    deinit();
    flush();
    raw_mode(0 as libc::c_int);
    init_signals(0 as libc::c_int);
    signal(
        13 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    c = -(1 as libc::c_int);
    while epos == -(1 as libc::c_int) as POSITION
        || {
            let fresh0 = spos;
            spos = spos + 1;
            fresh0 <= epos
        }
    {
        c = ch_forw_get();
        if c == -(1 as libc::c_int) {
            break;
        }
        if putc(c, f) == -(1 as libc::c_int) {
            break;
        }
    }
    while c != '\n' as i32 && c != -(1 as libc::c_int) {
        c = ch_forw_get();
        if c == -(1 as libc::c_int) {
            break;
        }
        if putc(c, f) == -(1 as libc::c_int) {
            break;
        }
    }
    pclose(f);
    signal(13 as libc::c_int, None);
    init_signals(1 as libc::c_int);
    raw_mode(1 as libc::c_int);
    init();
    screen_trashed = 1 as libc::c_int;
    winch(0 as libc::c_int);
    return 0 as libc::c_int;
}
