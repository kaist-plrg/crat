use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type mlist;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn creat(__file: *const libc::c_char, __mode: mode_t) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn save(s: *const libc::c_char) -> *mut libc::c_char;
    fn skipsp(s: *mut libc::c_char) -> *mut libc::c_char;
    fn quit(status: libc::c_int);
    fn end_logfile();
    fn ch_length() -> POSITION;
    fn ch_forw_get() -> libc::c_int;
    fn ch_init(f: libc::c_int, flags: libc::c_int);
    fn ch_close();
    fn ch_getflags() -> libc::c_int;
    fn cmd_addhist(mlist: *mut mlist, cmd: *const libc::c_char, modified: libc::c_int);
    fn ungetcc_back(c: LWCHAR);
    fn ungetsc(s: *mut libc::c_char);
    fn get_meta_escape() -> *mut libc::c_char;
    fn set_altpipe(ifile: *mut libc::c_void, p: *mut libc::c_void);
    fn get_altpipe(ifile: *mut libc::c_void) -> *mut libc::c_void;
    fn error(fmt: *mut libc::c_char, parg: *mut PARG);
    fn signal_message(sig: libc::c_int) -> *mut libc::c_char;
    fn errno_message(filename: *mut libc::c_char) -> *mut libc::c_char;
    fn get_altfilename(ifile: *mut libc::c_void) -> *mut libc::c_char;
    fn get_ifile(
        filename: *mut libc::c_char,
        prev: *mut libc::c_void,
    ) -> *mut libc::c_void;
    fn scan_eof();
    fn shell_quote(s: *mut libc::c_char) -> *mut libc::c_char;
    fn clr_hilite();
    fn clr_linenum();
    fn pos_clear();
    fn flush();
    fn query(fmt: *mut libc::c_char, parg: *mut PARG) -> libc::c_int;
    fn set_tabs(s: *mut libc::c_char, len: libc::c_int);
    fn forw_raw_line(
        curr_pos: POSITION,
        linep: *mut *mut libc::c_char,
        line_lenp: *mut libc::c_int,
    ) -> POSITION;
    fn get_pos(ifile: *mut libc::c_void, scrpos: *mut scrpos);
    fn set_open(ifile: *mut libc::c_void);
    fn set_altfilename(ifile: *mut libc::c_void, altfilename: *mut libc::c_char);
    fn hold_ifile(ifile: *mut libc::c_void, incr: libc::c_int);
    fn prev_ifile(h: *mut libc::c_void) -> *mut libc::c_void;
    fn next_ifile(h: *mut libc::c_void) -> *mut libc::c_void;
    fn del_ifile(h: *mut libc::c_void);
    fn close_altfile(altfilename: *mut libc::c_char, filename: *mut libc::c_char);
    fn bin_file(f: libc::c_int) -> libc::c_int;
    fn opened(ifile: *mut libc::c_void) -> libc::c_int;
    fn bad_file(filename: *mut libc::c_char) -> *mut libc::c_char;
    fn open_altfile(
        filename: *mut libc::c_char,
        pf: *mut libc::c_int,
        pfd: *mut *mut libc::c_void,
    ) -> *mut libc::c_char;
    fn get_filename(ifile: *mut libc::c_void) -> *mut libc::c_char;
    fn held_ifile(ifile: *mut libc::c_void) -> libc::c_int;
    fn lastmark();
    fn store_pos(ifile: *mut libc::c_void, scrpos: *mut scrpos);
    fn get_scrpos(scrpos: *mut scrpos, where_0: libc::c_int);
    fn shell_unquote(str: *mut libc::c_char) -> *mut libc::c_char;
    fn lglob(filename: *mut libc::c_char) -> *mut libc::c_char;
    fn nifile() -> libc::c_int;
    fn get_index(ifile: *mut libc::c_void) -> libc::c_int;
    fn putchr(c: libc::c_int) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    static mut new_file: libc::c_int;
    static mut every_first_cmd: *mut libc::c_char;
    static mut force_open: libc::c_int;
    static mut is_tty: libc::c_int;
    static mut sigs: libc::c_int;
    static mut hshift: libc::c_int;
    static mut want_filesize: libc::c_int;
    static mut consecutive_nulls: libc::c_int;
    static mut modelines: libc::c_int;
    static mut show_preproc_error: libc::c_int;
    static mut curr_ifile: *mut libc::c_void;
    static mut old_ifile: *mut libc::c_void;
    static mut initial_scrpos: scrpos;
    static mut ml_examine: *mut libc::c_void;
    static mut openquote: libc::c_char;
    static mut closequote: libc::c_char;
    static mut logfile: libc::c_int;
    static mut force_logfile: libc::c_int;
    static mut namelogfile: *mut libc::c_char;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type off_t = __off_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
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
pub struct textlist {
    pub string: *mut libc::c_char,
    pub endstring: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mloption {
    pub opt_name: *mut libc::c_char,
    pub opt_func: Option::<unsafe extern "C" fn(*mut libc::c_char, libc::c_int) -> ()>,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
pub static mut fd0: libc::c_int = 0 as libc::c_int;
pub static mut curr_dev: dev_t = 0;
pub static mut curr_ino: ino_t = 0;
pub unsafe extern "C" fn init_textlist(
    mut tlist: *mut textlist,
    mut str: *mut libc::c_char,
) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut meta_quoted: libc::c_int = 0 as libc::c_int;
    let mut delim_quoted: libc::c_int = 0 as libc::c_int;
    let mut esc: *mut libc::c_char = get_meta_escape();
    let mut esclen: libc::c_int = strlen(esc) as libc::c_int;
    (*tlist).string = skipsp(str);
    (*tlist).endstring = ((*tlist).string).offset(strlen((*tlist).string) as isize);
    s = str;
    while s < (*tlist).endstring {
        if meta_quoted != 0 {
            meta_quoted = 0 as libc::c_int;
        } else if esclen > 0 as libc::c_int
            && s.offset(esclen as isize) < (*tlist).endstring
            && strncmp(s, esc, esclen as libc::c_ulong) == 0 as libc::c_int
        {
            meta_quoted = 1 as libc::c_int;
            s = s.offset((esclen - 1 as libc::c_int) as isize);
        } else if delim_quoted != 0 {
            if *s as libc::c_int == closequote as libc::c_int {
                delim_quoted = 0 as libc::c_int;
            }
        } else if *s as libc::c_int == openquote as libc::c_int {
            delim_quoted = 1 as libc::c_int;
        } else if *s as libc::c_int == ' ' as i32 {
            *s = '\0' as i32 as libc::c_char;
        }
        s = s.offset(1);
        s;
    }
}
pub unsafe extern "C" fn forw_textlist(
    mut tlist: *mut textlist,
    mut prev: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if prev.is_null() {
        s = (*tlist).string;
    } else {
        s = prev.offset(strlen(prev) as isize);
    }
    if s >= (*tlist).endstring {
        return 0 as *mut libc::c_char;
    }
    while *s as libc::c_int == '\0' as i32 {
        s = s.offset(1);
        s;
    }
    if s >= (*tlist).endstring {
        return 0 as *mut libc::c_char;
    }
    return s;
}
pub unsafe extern "C" fn back_textlist(
    mut tlist: *mut textlist,
    mut prev: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if prev.is_null() {
        s = (*tlist).endstring;
    } else if prev <= (*tlist).string {
        return 0 as *mut libc::c_char
    } else {
        s = prev.offset(-(1 as libc::c_int as isize));
    }
    while *s as libc::c_int == '\0' as i32 {
        s = s.offset(-1);
        s;
    }
    if s <= (*tlist).string {
        return 0 as *mut libc::c_char;
    }
    while *s.offset(-(1 as libc::c_int) as isize) as libc::c_int != '\0' as i32
        && s > (*tlist).string
    {
        s = s.offset(-1);
        s;
    }
    return s;
}
unsafe extern "C" fn modeline_option(
    mut str: *mut libc::c_char,
    mut opt_len: libc::c_int,
) {
    let mut options: [mloption; 3] = [
        {
            let mut init = mloption {
                opt_name: b"ts=\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                opt_func: Some(
                    set_tabs
                        as unsafe extern "C" fn(*mut libc::c_char, libc::c_int) -> (),
                ),
            };
            init
        },
        {
            let mut init = mloption {
                opt_name: b"tabstop=\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                opt_func: Some(
                    set_tabs
                        as unsafe extern "C" fn(*mut libc::c_char, libc::c_int) -> (),
                ),
            };
            init
        },
        {
            let mut init = mloption {
                opt_name: 0 as *mut libc::c_char,
                opt_func: None,
            };
            init
        },
    ];
    let mut opt: *mut mloption = 0 as *mut mloption;
    opt = options.as_mut_ptr();
    while !((*opt).opt_name).is_null() {
        let mut name_len: libc::c_int = strlen((*opt).opt_name) as libc::c_int;
        if opt_len > name_len
            && strncmp(str, (*opt).opt_name, name_len as libc::c_ulong)
                == 0 as libc::c_int
        {
            (Some(((*opt).opt_func).unwrap()))
                .unwrap()(str.offset(name_len as isize), opt_len - name_len);
            break;
        } else {
            opt = opt.offset(1);
            opt;
        }
    }
}
unsafe extern "C" fn modeline_option_len(mut str: *mut libc::c_char) -> libc::c_int {
    let mut esc: libc::c_int = 0 as libc::c_int;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = str;
    while *s as libc::c_int != '\0' as i32 {
        if esc != 0 {
            esc = 0 as libc::c_int;
        } else if *s as libc::c_int == '\\' as i32 {
            esc = 1 as libc::c_int;
        } else if *s as libc::c_int == ' ' as i32 || *s as libc::c_int == ':' as i32 {
            break;
        }
        s = s.offset(1);
        s;
    }
    return s.offset_from(str) as libc::c_long as libc::c_int;
}
unsafe extern "C" fn modeline_options(
    mut str: *mut libc::c_char,
    mut end_char: libc::c_char,
) {
    loop {
        let mut opt_len: libc::c_int = 0;
        str = skipsp(str);
        if *str as libc::c_int == '\0' as i32
            || *str as libc::c_int == end_char as libc::c_int
        {
            break;
        }
        opt_len = modeline_option_len(str);
        modeline_option(str, opt_len);
        str = str.offset(opt_len as isize);
        if *str as libc::c_int != '\0' as i32 {
            str = str.offset(1 as libc::c_int as isize);
        }
    };
}
unsafe extern "C" fn check_modeline(mut line: *mut libc::c_char) {
    static mut pgms: [*mut libc::c_char; 5] = [
        b"less:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"vim:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"vi:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"ex:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
    ];
    let mut pgm: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    pgm = pgms.as_mut_ptr();
    while !(*pgm).is_null() {
        let mut pline: *mut libc::c_char = line;
        loop {
            let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
            pline = strstr(pline, *pgm);
            if pline.is_null() {
                break;
            }
            str = skipsp(pline.offset(strlen(*pgm) as isize));
            if pline == line
                || *pline.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    == ' ' as i32
            {
                if strncmp(
                    str,
                    b"set \0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    modeline_options(
                        str.offset(4 as libc::c_int as isize),
                        ':' as i32 as libc::c_char,
                    );
                } else if pgm
                    != &mut *pgms.as_mut_ptr().offset(0 as libc::c_int as isize)
                        as *mut *mut libc::c_char
                {
                    modeline_options(str, '\0' as i32 as libc::c_char);
                }
                break;
            } else {
                pline = str;
            }
        }
        pgm = pgm.offset(1);
        pgm;
    }
}
unsafe extern "C" fn check_modelines() {
    let mut pos: POSITION = 0 as libc::c_int as POSITION;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < modelines {
        let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut line_len: libc::c_int = 0;
        if sigs & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0 {
            return;
        }
        pos = forw_raw_line(pos, &mut line, &mut line_len);
        if pos == -(1 as libc::c_int) as POSITION {
            break;
        }
        check_modeline(line);
        i += 1;
        i;
    }
}
unsafe extern "C" fn close_pipe(mut pipefd: *mut FILE) {
    let mut status: libc::c_int = 0;
    let mut parg: PARG = parg {
        p_string: 0 as *mut libc::c_char,
    };
    if pipefd.is_null() {
        return;
    }
    status = pclose(pipefd);
    if status == -(1 as libc::c_int) {
        parg
            .p_string = errno_message(
            b"pclose\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        error(
            b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut parg,
        );
        free(parg.p_string as *mut libc::c_void);
        return;
    }
    if show_preproc_error == 0 {
        return;
    }
    if status & 0x7f as libc::c_int == 0 as libc::c_int {
        let mut s: libc::c_int = (status & 0xff00 as libc::c_int) >> 8 as libc::c_int;
        if s != 0 as libc::c_int {
            parg.p_int = s;
            error(
                b"Input preprocessor failed (status %d)\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                &mut parg,
            );
        }
        return;
    }
    if ((status & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar
        as libc::c_int >> 1 as libc::c_int > 0 as libc::c_int
    {
        let mut sig: libc::c_int = status & 0x7f as libc::c_int;
        if sig != 13 as libc::c_int || ch_length() != -(1 as libc::c_int) as POSITION {
            parg.p_string = signal_message(sig);
            error(
                b"Input preprocessor terminated: %s\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                &mut parg,
            );
        }
        return;
    }
    if status != 0 as libc::c_int {
        parg.p_int = status;
        error(
            b"Input preprocessor exited with status %x\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            &mut parg,
        );
    }
}
pub unsafe extern "C" fn close_altpipe(mut ifile: *mut libc::c_void) {
    let mut altpipe: *mut FILE = get_altpipe(ifile) as *mut FILE;
    if !altpipe.is_null() && ch_getflags() & 0o2 as libc::c_int == 0 {
        close_pipe(altpipe);
        set_altpipe(ifile, 0 as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn check_altpipe_error() {
    if show_preproc_error == 0 {
        return;
    }
    if curr_ifile != 0 as *mut libc::c_void && !(get_altfilename(curr_ifile)).is_null() {
        close_altpipe(curr_ifile);
    }
}
unsafe extern "C" fn close_file() {
    let mut scrpos: scrpos = scrpos { pos: 0, ln: 0 };
    let mut altfilename: *mut libc::c_char = 0 as *mut libc::c_char;
    if curr_ifile == 0 as *mut libc::c_void {
        return;
    }
    get_scrpos(&mut scrpos, 0 as libc::c_int);
    if scrpos.pos != -(1 as libc::c_int) as POSITION {
        store_pos(curr_ifile, &mut scrpos);
        lastmark();
    }
    ch_close();
    altfilename = get_altfilename(curr_ifile);
    if !altfilename.is_null() {
        close_altpipe(curr_ifile);
        close_altfile(altfilename, get_filename(curr_ifile));
        set_altfilename(curr_ifile, 0 as *mut libc::c_char);
    }
    curr_ifile = 0 as *mut libc::c_void;
    curr_dev = 0 as libc::c_int as dev_t;
    curr_ino = curr_dev;
}
pub unsafe extern "C" fn edit(mut filename: *mut libc::c_char) -> libc::c_int {
    if filename.is_null() {
        return edit_ifile(0 as *mut libc::c_void);
    }
    return edit_ifile(get_ifile(filename, curr_ifile));
}
unsafe extern "C" fn edit_error(
    mut filename: *mut libc::c_char,
    mut alt_filename: *mut libc::c_char,
    mut altpipe: *mut libc::c_void,
    mut ifile: *mut libc::c_void,
    mut was_curr_ifile: *mut libc::c_void,
) -> libc::c_int {
    if !alt_filename.is_null() {
        close_pipe(altpipe as *mut FILE);
        close_altfile(alt_filename, filename);
        free(alt_filename as *mut libc::c_void);
    }
    del_ifile(ifile);
    free(filename as *mut libc::c_void);
    if was_curr_ifile == ifile {
        quit(1 as libc::c_int);
    }
    reedit_ifile(was_curr_ifile);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn edit_ifile(mut ifile: *mut libc::c_void) -> libc::c_int {
    let mut f: libc::c_int = 0;
    let mut answer: libc::c_int = 0;
    let mut chflags: libc::c_int = 0;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut open_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut alt_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut altpipe: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut was_curr_ifile: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut parg: PARG = parg {
        p_string: 0 as *mut libc::c_char,
    };
    if ifile == curr_ifile {
        return 0 as libc::c_int;
    }
    end_logfile();
    was_curr_ifile = save_curr_ifile();
    if curr_ifile != 0 as *mut libc::c_void {
        chflags = ch_getflags();
        close_file();
        if chflags & 0o10 as libc::c_int != 0
            && held_ifile(was_curr_ifile) <= 1 as libc::c_int
        {
            del_ifile(was_curr_ifile);
            was_curr_ifile = old_ifile;
        }
    }
    if ifile == 0 as *mut libc::c_void {
        unsave_ifile(was_curr_ifile);
        return 0 as libc::c_int;
    }
    filename = save(get_filename(ifile));
    altpipe = get_altpipe(ifile);
    if !altpipe.is_null() {
        chflags = 0 as libc::c_int;
        f = -(1 as libc::c_int);
        alt_filename = get_altfilename(ifile);
        open_filename = if !alt_filename.is_null() { alt_filename } else { filename };
    } else {
        if strcmp(
            filename,
            b"@/\\less/\\help/\\file/\\@\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                filename,
                b"@/\\less/\\empty/\\file/\\@\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            alt_filename = 0 as *mut libc::c_char;
        } else {
            alt_filename = open_altfile(filename, &mut f, &mut altpipe);
        }
        open_filename = if !alt_filename.is_null() { alt_filename } else { filename };
        chflags = 0 as libc::c_int;
        if !altpipe.is_null() {
            chflags |= 0o4 as libc::c_int;
            if strcmp(filename, b"-\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                chflags |= 0o2 as libc::c_int;
            }
        } else if strcmp(filename, b"-\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            f = fd0;
            chflags |= 0o2 as libc::c_int;
        } else if strcmp(
            open_filename,
            b"@/\\less/\\empty/\\file/\\@\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            f = -(1 as libc::c_int);
            chflags |= 0o20 as libc::c_int;
        } else if strcmp(
            open_filename,
            b"@/\\less/\\help/\\file/\\@\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            f = -(1 as libc::c_int);
            chflags |= 0o10 as libc::c_int;
        } else {
            parg.p_string = bad_file(open_filename);
            if !(parg.p_string).is_null() {
                error(
                    b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    &mut parg,
                );
                free(parg.p_string as *mut libc::c_void);
                return edit_error(
                    filename,
                    alt_filename,
                    altpipe,
                    ifile,
                    was_curr_ifile,
                );
            } else {
                f = open(open_filename, 0 as libc::c_int);
                if f < 0 as libc::c_int {
                    parg.p_string = errno_message(filename);
                    error(
                        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        &mut parg,
                    );
                    free(parg.p_string as *mut libc::c_void);
                    return edit_error(
                        filename,
                        alt_filename,
                        altpipe,
                        ifile,
                        was_curr_ifile,
                    );
                } else {
                    chflags |= 0o1 as libc::c_int;
                    if force_open == 0 && opened(ifile) == 0 && bin_file(f) != 0 {
                        parg.p_string = filename;
                        answer = query(
                            b"\"%s\" may be a binary file.  See it anyway? \0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                            &mut parg,
                        );
                        if answer != 'y' as i32 && answer != 'Y' as i32 {
                            close(f);
                            return edit_error(
                                filename,
                                alt_filename,
                                altpipe,
                                ifile,
                                was_curr_ifile,
                            );
                        }
                    }
                }
            }
        }
    }
    if force_open == 0 && f >= 0 as libc::c_int && isatty(f) != 0 {
        let mut parg_0: PARG = parg {
            p_string: 0 as *mut libc::c_char,
        };
        parg_0.p_string = filename;
        error(
            b"%s is a terminal (use -f to open it)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            &mut parg_0,
        );
        return edit_error(filename, alt_filename, altpipe, ifile, was_curr_ifile);
    }
    if was_curr_ifile != 0 as *mut libc::c_void {
        old_ifile = was_curr_ifile;
        unsave_ifile(was_curr_ifile);
    }
    curr_ifile = ifile;
    set_altfilename(curr_ifile, alt_filename);
    set_altpipe(curr_ifile, altpipe);
    set_open(curr_ifile);
    get_pos(curr_ifile, &mut initial_scrpos);
    new_file = 1 as libc::c_int;
    ch_init(f, chflags);
    consecutive_nulls = 0 as libc::c_int;
    check_modelines();
    if chflags & 0o10 as libc::c_int == 0 {
        if !namelogfile.is_null() && is_tty != 0 {
            use_logfile(namelogfile);
        }
        if strcmp(open_filename, b"-\0" as *const u8 as *const libc::c_char)
            != 0 as libc::c_int
        {
            let mut statbuf: stat = stat {
                st_dev: 0,
                st_ino: 0,
                st_nlink: 0,
                st_mode: 0,
                st_uid: 0,
                st_gid: 0,
                __pad0: 0,
                st_rdev: 0,
                st_size: 0,
                st_blksize: 0,
                st_blocks: 0,
                st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
                st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
                st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
                __glibc_reserved: [0; 3],
            };
            let mut r: libc::c_int = stat(open_filename, &mut statbuf);
            if r == 0 as libc::c_int {
                curr_ino = statbuf.st_ino;
                curr_dev = statbuf.st_dev;
            }
        }
        if !every_first_cmd.is_null() {
            ungetsc(every_first_cmd);
            ungetcc_back(0x40000000 as libc::c_int as LWCHAR);
        }
    }
    flush();
    if is_tty != 0 {
        pos_clear();
        clr_linenum();
        clr_hilite();
        hshift = 0 as libc::c_int;
        if strcmp(
            filename,
            b"@/\\less/\\help/\\file/\\@\0" as *const u8 as *const libc::c_char,
        ) != 0
            && strcmp(
                filename,
                b"@/\\less/\\empty/\\file/\\@\0" as *const u8 as *const libc::c_char,
            ) != 0
        {
            let mut qfilename: *mut libc::c_char = shell_quote(filename);
            cmd_addhist(ml_examine as *mut mlist, qfilename, 1 as libc::c_int);
            free(qfilename as *mut libc::c_void);
        }
        if want_filesize != 0 {
            scan_eof();
        }
    }
    free(filename as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn edit_list(mut filelist: *mut libc::c_char) -> libc::c_int {
    let mut save_ifile: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut good_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut gfilelist: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut gfilename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut qfilename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tl_files: textlist = textlist {
        string: 0 as *mut libc::c_char,
        endstring: 0 as *mut libc::c_char,
    };
    let mut tl_gfiles: textlist = textlist {
        string: 0 as *mut libc::c_char,
        endstring: 0 as *mut libc::c_char,
    };
    save_ifile = save_curr_ifile();
    good_filename = 0 as *mut libc::c_char;
    init_textlist(&mut tl_files, filelist);
    filename = 0 as *mut libc::c_char;
    loop {
        filename = forw_textlist(&mut tl_files, filename);
        if filename.is_null() {
            break;
        }
        gfilelist = lglob(filename);
        init_textlist(&mut tl_gfiles, gfilelist);
        gfilename = 0 as *mut libc::c_char;
        loop {
            gfilename = forw_textlist(&mut tl_gfiles, gfilename);
            if gfilename.is_null() {
                break;
            }
            qfilename = shell_unquote(gfilename);
            if edit(qfilename) == 0 as libc::c_int && good_filename.is_null() {
                good_filename = get_filename(curr_ifile);
            }
            free(qfilename as *mut libc::c_void);
        }
        free(gfilelist as *mut libc::c_void);
    }
    if good_filename.is_null() {
        unsave_ifile(save_ifile);
        return 1 as libc::c_int;
    }
    if get_ifile(good_filename, curr_ifile) == curr_ifile {
        unsave_ifile(save_ifile);
        return 0 as libc::c_int;
    }
    reedit_ifile(save_ifile);
    return edit(good_filename);
}
pub unsafe extern "C" fn edit_first() -> libc::c_int {
    if nifile() == 0 as libc::c_int {
        return edit_stdin();
    }
    curr_ifile = 0 as *mut libc::c_void;
    return edit_next(1 as libc::c_int);
}
pub unsafe extern "C" fn edit_last() -> libc::c_int {
    curr_ifile = 0 as *mut libc::c_void;
    return edit_prev(1 as libc::c_int);
}
unsafe extern "C" fn edit_istep(
    mut h: *mut libc::c_void,
    mut n: libc::c_int,
    mut dir: libc::c_int,
) -> libc::c_int {
    let mut next: *mut libc::c_void = 0 as *mut libc::c_void;
    loop {
        next = if dir > 0 as libc::c_int { next_ifile(h) } else { prev_ifile(h) };
        n -= 1;
        if n < 0 as libc::c_int {
            if edit_ifile(h) == 0 as libc::c_int {
                break;
            }
        }
        if next == 0 as *mut libc::c_void {
            return 1 as libc::c_int;
        }
        if sigs & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0 {
            return 1 as libc::c_int;
        }
        h = next;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn edit_inext(
    mut h: *mut libc::c_void,
    mut n: libc::c_int,
) -> libc::c_int {
    return edit_istep(h, n, 1 as libc::c_int);
}
pub unsafe extern "C" fn edit_next(mut n: libc::c_int) -> libc::c_int {
    return edit_istep(curr_ifile, n, 1 as libc::c_int);
}
unsafe extern "C" fn edit_iprev(
    mut h: *mut libc::c_void,
    mut n: libc::c_int,
) -> libc::c_int {
    return edit_istep(h, n, -(1 as libc::c_int));
}
pub unsafe extern "C" fn edit_prev(mut n: libc::c_int) -> libc::c_int {
    return edit_istep(curr_ifile, n, -(1 as libc::c_int));
}
pub unsafe extern "C" fn edit_index(mut n: libc::c_int) -> libc::c_int {
    let mut h: *mut libc::c_void = 0 as *mut libc::c_void;
    h = 0 as *mut libc::c_void;
    loop {
        h = next_ifile(h);
        if h == 0 as *mut libc::c_void {
            return 1 as libc::c_int;
        }
        if !(get_index(h) != n) {
            break;
        }
    }
    return edit_ifile(h);
}
pub unsafe extern "C" fn save_curr_ifile() -> *mut libc::c_void {
    if curr_ifile != 0 as *mut libc::c_void {
        hold_ifile(curr_ifile, 1 as libc::c_int);
    }
    return curr_ifile;
}
pub unsafe extern "C" fn unsave_ifile(mut save_ifile: *mut libc::c_void) {
    if save_ifile != 0 as *mut libc::c_void {
        hold_ifile(save_ifile, -(1 as libc::c_int));
    }
}
pub unsafe extern "C" fn reedit_ifile(mut save_ifile: *mut libc::c_void) {
    let mut next: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut prev: *mut libc::c_void = 0 as *mut libc::c_void;
    unsave_ifile(save_ifile);
    next = next_ifile(save_ifile);
    prev = prev_ifile(save_ifile);
    if edit_ifile(save_ifile) == 0 as libc::c_int {
        return;
    }
    if next != 0 as *mut libc::c_void
        && edit_inext(next, 0 as libc::c_int) == 0 as libc::c_int
    {
        return;
    }
    if prev != 0 as *mut libc::c_void
        && edit_iprev(prev, 0 as libc::c_int) == 0 as libc::c_int
    {
        return;
    }
    quit(1 as libc::c_int);
}
pub unsafe extern "C" fn reopen_curr_ifile() {
    let mut save_ifile: *mut libc::c_void = save_curr_ifile();
    close_file();
    reedit_ifile(save_ifile);
}
pub unsafe extern "C" fn edit_stdin() -> libc::c_int {
    if isatty(fd0) != 0 {
        error(
            b"Missing filename (\"less --help\" for help)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_void as *mut PARG,
        );
        quit(0 as libc::c_int);
    }
    return edit(b"-\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
}
pub unsafe extern "C" fn cat_file() {
    let mut c: libc::c_int = 0;
    loop {
        c = ch_forw_get();
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        putchr(c);
    }
    flush();
}
pub unsafe extern "C" fn use_logfile(mut filename: *mut libc::c_char) {
    let mut exists: libc::c_int = 0;
    let mut answer: libc::c_int = 0;
    let mut parg: PARG = parg {
        p_string: 0 as *mut libc::c_char,
    };
    if ch_getflags() & 0o1 as libc::c_int != 0 {
        return;
    }
    exists = open(filename, 0 as libc::c_int);
    if exists >= 0 as libc::c_int {
        close(exists);
    }
    exists = (exists >= 0 as libc::c_int) as libc::c_int;
    if exists == 0 || force_logfile != 0 {
        answer = 'O' as i32;
    } else {
        parg.p_string = filename;
        answer = query(
            b"Warning: \"%s\" exists; Overwrite, Append, Don't log, or Quit? \0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut parg,
        );
    }
    loop {
        match answer {
            79 | 111 => {
                logfile = creat(filename, 0o644 as libc::c_int as mode_t);
                break;
            }
            65 | 97 => {
                logfile = open(filename, 0o2000 as libc::c_int | 0o1 as libc::c_int);
                if lseek(logfile, 0 as libc::c_int as off_t, 2 as libc::c_int)
                    == -(1 as libc::c_int) as off_t
                {
                    close(logfile);
                    logfile = -(1 as libc::c_int);
                }
                break;
            }
            68 | 100 => return,
            _ => {
                answer = query(
                    b"Overwrite, Append, Don't log, or Quit? (Type \"O\", \"A\", \"D\" or \"Q\") \0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                    0 as *mut libc::c_void as *mut PARG,
                );
            }
        }
    }
    if logfile < 0 as libc::c_int {
        parg.p_string = filename;
        error(
            b"Cannot write to \"%s\"\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            &mut parg,
        );
        return;
    }
}
