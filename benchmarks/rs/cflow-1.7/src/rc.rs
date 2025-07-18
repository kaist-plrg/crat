use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type wordsplit_node;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn free(__ptr: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn wordsplit(
        s: *const libc::c_char,
        ws: *mut wordsplit_t,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn wordsplit_free(ws: *mut wordsplit_t);
    fn wordsplit_strerror(ws: *mut wordsplit_t) -> *const libc::c_char;
}
pub type size_t = libc::c_ulong;
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
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wordsplit {
    pub ws_wordc: size_t,
    pub ws_wordv: *mut *mut libc::c_char,
    pub ws_offs: size_t,
    pub ws_wordn: size_t,
    pub ws_flags: libc::c_int,
    pub ws_options: libc::c_int,
    pub ws_maxwords: size_t,
    pub ws_wordi: size_t,
    pub ws_delim: *const libc::c_char,
    pub ws_comment: *const libc::c_char,
    pub ws_escape: [*const libc::c_char; 2],
    pub ws_namechar: *const libc::c_char,
    pub ws_alloc_die: Option::<unsafe extern "C" fn(*mut wordsplit_t) -> ()>,
    pub ws_error: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>,
    pub ws_debug: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>,
    pub ws_env: *mut *const libc::c_char,
    pub ws_envbuf: *mut *mut libc::c_char,
    pub ws_envidx: size_t,
    pub ws_envsiz: size_t,
    pub ws_paramv: *mut *const libc::c_char,
    pub ws_paramc: size_t,
    pub ws_parambuf: *mut *mut libc::c_char,
    pub ws_paramidx: size_t,
    pub ws_paramsiz: size_t,
    pub ws_getvar: Option::<
        unsafe extern "C" fn(
            *mut *mut libc::c_char,
            *const libc::c_char,
            size_t,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub ws_closure: *mut libc::c_void,
    pub ws_command: Option::<
        unsafe extern "C" fn(
            *mut *mut libc::c_char,
            *const libc::c_char,
            size_t,
            *mut *mut libc::c_char,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub ws_input: *const libc::c_char,
    pub ws_len: size_t,
    pub ws_endp: size_t,
    pub ws_errno: libc::c_int,
    pub ws_usererr: *mut libc::c_char,
    pub ws_errctx: *mut libc::c_char,
    pub ws_head: *mut wordsplit_node,
    pub ws_tail: *mut wordsplit_node,
    pub ws_sep: [libc::c_char; 2],
    pub ws_lvl: libc::c_int,
}
pub type wordsplit_t = wordsplit;
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
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
unsafe extern "C" fn expand_argcv(
    mut argc_ptr: *mut libc::c_int,
    mut argv_ptr: *mut *mut *mut libc::c_char,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let mut i: libc::c_int = 0;
    *argv_ptr = xrealloc(
        *argv_ptr as *mut libc::c_void,
        ((*argc_ptr + argc + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < argc {
        let ref mut fresh0 = *(*argv_ptr).offset((*argc_ptr + i) as isize);
        *fresh0 = xstrdup(*argv.offset(i as isize));
        i += 1;
        i;
    }
    let ref mut fresh1 = *(*argv_ptr).offset((*argc_ptr + i) as isize);
    *fresh1 = 0 as *mut libc::c_char;
    *argc_ptr += argc;
}
pub unsafe extern "C" fn parse_rc(
    mut argc_ptr: *mut libc::c_int,
    mut argv_ptr: *mut *mut *mut libc::c_char,
    mut name: *mut libc::c_char,
) {
    let mut st: stat = stat {
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
    let mut rcfile: *mut FILE = 0 as *mut FILE;
    let mut size: libc::c_int = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ws: wordsplit = wordsplit {
        ws_wordc: 0,
        ws_wordv: 0 as *mut *mut libc::c_char,
        ws_offs: 0,
        ws_wordn: 0,
        ws_flags: 0,
        ws_options: 0,
        ws_maxwords: 0,
        ws_wordi: 0,
        ws_delim: 0 as *const libc::c_char,
        ws_comment: 0 as *const libc::c_char,
        ws_escape: [0 as *const libc::c_char; 2],
        ws_namechar: 0 as *const libc::c_char,
        ws_alloc_die: None,
        ws_error: None,
        ws_debug: None,
        ws_env: 0 as *mut *const libc::c_char,
        ws_envbuf: 0 as *mut *mut libc::c_char,
        ws_envidx: 0,
        ws_envsiz: 0,
        ws_paramv: 0 as *mut *const libc::c_char,
        ws_paramc: 0,
        ws_parambuf: 0 as *mut *mut libc::c_char,
        ws_paramidx: 0,
        ws_paramsiz: 0,
        ws_getvar: None,
        ws_closure: 0 as *mut libc::c_void,
        ws_command: None,
        ws_input: 0 as *const libc::c_char,
        ws_len: 0,
        ws_endp: 0,
        ws_errno: 0,
        ws_usererr: 0 as *mut libc::c_char,
        ws_errctx: 0 as *mut libc::c_char,
        ws_head: 0 as *mut wordsplit_node,
        ws_tail: 0 as *mut wordsplit_node,
        ws_sep: [0; 2],
        ws_lvl: 0,
    };
    let mut wsflags: libc::c_int = 0;
    let mut line: libc::c_int = 0;
    if stat(name, &mut st) != 0 {
        return;
    }
    buf = xmalloc((st.st_size + 1 as libc::c_int as libc::c_long) as size_t)
        as *mut libc::c_char;
    rcfile = fopen(name, b"r\0" as *const u8 as *const libc::c_char);
    if rcfile.is_null() {
        error(
            1 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"cannot open `%s'\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            name,
        );
        return;
    }
    size = fread(
        buf as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        st.st_size as libc::c_ulong,
        rcfile,
    ) as libc::c_int;
    *buf.offset(size as isize) = 0 as libc::c_int as libc::c_char;
    fclose(rcfile);
    ws.ws_comment = b"#\0" as *const u8 as *const libc::c_char;
    wsflags = 0x40 as libc::c_int | 0x4 as libc::c_int
        | (0x200 as libc::c_int | 0x400 as libc::c_int) | 0x800 as libc::c_int
        | 0x2000000 as libc::c_int | 0x8000 as libc::c_int;
    line = 0 as libc::c_int;
    p = strtok(buf, b"\n\0" as *const u8 as *const libc::c_char);
    while !p.is_null() {
        line += 1;
        line;
        if wordsplit(p, &mut ws, wsflags) != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                b"%s:%d: %s\0" as *const u8 as *const libc::c_char,
                name,
                line,
                wordsplit_strerror(&mut ws),
            );
        }
        wsflags |= 0x8 as libc::c_int;
        if ws.ws_wordc != 0 {
            expand_argcv(argc_ptr, argv_ptr, ws.ws_wordc as libc::c_int, ws.ws_wordv);
        }
        p = strtok(0 as *mut libc::c_char, b"\n\0" as *const u8 as *const libc::c_char);
    }
    if wsflags & 0x8 as libc::c_int != 0 {
        wordsplit_free(&mut ws);
    }
    free(buf as *mut libc::c_void);
}
pub unsafe extern "C" fn sourcerc(
    mut argc_ptr: *mut libc::c_int,
    mut argv_ptr: *mut *mut *mut libc::c_char,
) {
    let mut env: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut xargc: libc::c_int = 1 as libc::c_int;
    let mut xargv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    xargv = xmalloc(
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    let ref mut fresh2 = *xargv.offset(0 as libc::c_int as isize);
    *fresh2 = **argv_ptr;
    let ref mut fresh3 = *xargv.offset(1 as libc::c_int as isize);
    *fresh3 = 0 as *mut libc::c_char;
    env = getenv(b"CFLOW_OPTIONS\0" as *const u8 as *const libc::c_char);
    if !env.is_null() {
        let mut ws: wordsplit = wordsplit {
            ws_wordc: 0,
            ws_wordv: 0 as *mut *mut libc::c_char,
            ws_offs: 0,
            ws_wordn: 0,
            ws_flags: 0,
            ws_options: 0,
            ws_maxwords: 0,
            ws_wordi: 0,
            ws_delim: 0 as *const libc::c_char,
            ws_comment: 0 as *const libc::c_char,
            ws_escape: [0 as *const libc::c_char; 2],
            ws_namechar: 0 as *const libc::c_char,
            ws_alloc_die: None,
            ws_error: None,
            ws_debug: None,
            ws_env: 0 as *mut *const libc::c_char,
            ws_envbuf: 0 as *mut *mut libc::c_char,
            ws_envidx: 0,
            ws_envsiz: 0,
            ws_paramv: 0 as *mut *const libc::c_char,
            ws_paramc: 0,
            ws_parambuf: 0 as *mut *mut libc::c_char,
            ws_paramidx: 0,
            ws_paramsiz: 0,
            ws_getvar: None,
            ws_closure: 0 as *mut libc::c_void,
            ws_command: None,
            ws_input: 0 as *const libc::c_char,
            ws_len: 0,
            ws_endp: 0,
            ws_errno: 0,
            ws_usererr: 0 as *mut libc::c_char,
            ws_errctx: 0 as *mut libc::c_char,
            ws_head: 0 as *mut wordsplit_node,
            ws_tail: 0 as *mut wordsplit_node,
            ws_sep: [0; 2],
            ws_lvl: 0,
        };
        ws.ws_comment = b"#\0" as *const u8 as *const libc::c_char;
        if wordsplit(
            env,
            &mut ws,
            0x40 as libc::c_int | 0x4 as libc::c_int
                | (0x200 as libc::c_int | 0x400 as libc::c_int) | 0x800 as libc::c_int
                | 0x2000000 as libc::c_int | 0x8000 as libc::c_int,
        ) != 0
        {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                b"failed to parse CFLOW_OPTIONS: %s\0" as *const u8
                    as *const libc::c_char,
                wordsplit_strerror(&mut ws),
            );
        }
        if ws.ws_wordc != 0 {
            expand_argcv(
                &mut xargc,
                &mut xargv,
                ws.ws_wordc as libc::c_int,
                ws.ws_wordv,
            );
        }
        wordsplit_free(&mut ws);
    }
    env = getenv(b"CFLOWRC\0" as *const u8 as *const libc::c_char);
    if !env.is_null() {
        parse_rc(&mut xargc, &mut xargv, env);
    } else {
        let mut home: *mut libc::c_char = getenv(
            b"HOME\0" as *const u8 as *const libc::c_char,
        );
        if !home.is_null() {
            let mut len: libc::c_int = strlen(home) as libc::c_int;
            let mut buf: *mut libc::c_char = malloc(
                (len as libc::c_ulong)
                    .wrapping_add(
                        ::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
                    )
                    .wrapping_add(
                        (*home.offset((len - 1 as libc::c_int) as isize) as libc::c_int
                            != '/' as i32) as libc::c_int as libc::c_ulong,
                    ),
            ) as *mut libc::c_char;
            if buf.is_null() {
                return;
            }
            strcpy(buf, home);
            if *home.offset((len - 1 as libc::c_int) as isize) as libc::c_int
                != '/' as i32
            {
                let fresh4 = len;
                len = len + 1;
                *buf.offset(fresh4 as isize) = '/' as i32 as libc::c_char;
            }
            strcpy(
                buf.offset(len as isize),
                b".cflowrc\0" as *const u8 as *const libc::c_char,
            );
            parse_rc(&mut xargc, &mut xargv, buf);
            free(buf as *mut libc::c_void);
        }
    }
    if xargc > 1 as libc::c_int {
        expand_argcv(
            &mut xargc,
            &mut xargv,
            *argc_ptr - 1 as libc::c_int,
            (*argv_ptr).offset(1 as libc::c_int as isize),
        );
        *argc_ptr = xargc;
        *argv_ptr = xargv;
    }
}
