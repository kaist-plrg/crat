use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn rpl_strtol(
        string: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
        base: libc::c_int,
    ) -> libc::c_long;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn rpl_free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn __uflow(_: *mut FILE) -> libc::c_int;
    fn __getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn fread_unlocked(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn __strtoul_internal(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
        __group: libc::c_int,
    ) -> libc::c_ulong;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn argp_parse(
        __argp: *const argp,
        _: libc::c_int,
        _: *mut *mut libc::c_char,
        __flags: libc::c_uint,
        __arg_index: *mut libc::c_int,
        __input: *mut libc::c_void,
    ) -> error_t;
    fn argp_version_setup(
        name: *const libc::c_char,
        authors: *const *const libc::c_char,
    );
    fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> size_t;
    fn set_program_name(argv0: *const libc::c_char);
    fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> size_t;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __uintmax_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __daddr_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type error_t = libc::c_int;
pub type va_list = __builtin_va_list;
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
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtop {
    pub mt_op: libc::c_short,
    pub mt_count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtget {
    pub mt_type: libc::c_long,
    pub mt_resid: libc::c_long,
    pub mt_dsreg: libc::c_long,
    pub mt_gstat: libc::c_long,
    pub mt_erreg: libc::c_long,
    pub mt_fileno: __daddr_t,
    pub mt_blkno: __daddr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_option {
    pub name: *const libc::c_char,
    pub key: libc::c_int,
    pub arg: *const libc::c_char,
    pub flags: libc::c_int,
    pub doc: *const libc::c_char,
    pub group: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp {
    pub options: *const argp_option,
    pub parser: argp_parser_t,
    pub args_doc: *const libc::c_char,
    pub doc: *const libc::c_char,
    pub children: *const argp_child,
    pub help_filter: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_char,
            *mut libc::c_void,
        ) -> *mut libc::c_char,
    >,
    pub argp_domain: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_child {
    pub argp: *const argp,
    pub flags: libc::c_int,
    pub header: *const libc::c_char,
    pub group: libc::c_int,
}
pub type argp_parser_t = Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_char, *mut argp_state) -> error_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_state {
    pub root_argp: *const argp,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub next: libc::c_int,
    pub flags: libc::c_uint,
    pub arg_num: libc::c_uint,
    pub quoted: libc::c_int,
    pub input: *mut libc::c_void,
    pub child_inputs: *mut *mut libc::c_void,
    pub hook: *mut libc::c_void,
    pub name: *mut libc::c_char,
    pub err_stream: *mut FILE,
    pub out_stream: *mut FILE,
    pub pstate: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rmt_kw {
    pub name: *const libc::c_char,
    pub len: size_t,
    pub value: libc::c_int,
}
pub type C2RustUnnamed = libc::c_uint;
pub const DEBUG_FILE_OPTION: C2RustUnnamed = 256;
#[inline]
unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE) -> libc::c_int {
    return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as libc::c_int
        as libc::c_long != 0
    {
        __uflow(__fp)
    } else {
        let fresh0 = (*__fp)._IO_read_ptr;
        (*__fp)._IO_read_ptr = ((*__fp)._IO_read_ptr).offset(1);
        *(fresh0 as *mut libc::c_uchar) as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut libc::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
#[inline]
unsafe extern "C" fn feof_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x10 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn strtoumax(
    mut nptr: *const libc::c_char,
    mut endptr: *mut *mut libc::c_char,
    mut base: libc::c_int,
) -> uintmax_t {
    return __strtoul_internal(nptr, endptr, base, 0 as libc::c_int);
}
#[inline]
unsafe extern "C" fn c_isalnum(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101 | 102
        | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115
        | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72
        | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88
        | 89 | 90 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isblank(mut c: libc::c_int) -> bool {
    return c == ' ' as i32 || c == '\t' as i32;
}
#[inline]
unsafe extern "C" fn c_isdigit(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
pub static mut dbglev: libc::c_int = 0;
pub static mut dbgout: *mut FILE = 0 as *const FILE as *mut FILE;
unsafe extern "C" fn trimnl(mut str: *mut libc::c_char) {
    if !str.is_null() {
        let mut len: size_t = strlen(str);
        if len > 1 as libc::c_int as libc::c_ulong
            && *str.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int == '\n' as i32
        {
            *str
                .offset(
                    len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) = 0 as libc::c_int as libc::c_char;
        }
    }
}
pub static mut input_buf_ptr: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut input_buf_size: size_t = 0 as libc::c_int as size_t;
unsafe extern "C" fn rmt_read() -> *mut libc::c_char {
    let mut rc: ssize_t = getline(&mut input_buf_ptr, &mut input_buf_size, stdin);
    if rc > 0 as libc::c_int as libc::c_long {
        if !dbgout.is_null() && 10 as libc::c_int <= dbglev {
            fprintf(
                dbgout,
                b"C: %s\0" as *const u8 as *const libc::c_char,
                input_buf_ptr,
            );
        }
        trimnl(input_buf_ptr);
        return input_buf_ptr;
    }
    if !dbgout.is_null() && 10 as libc::c_int <= dbglev {
        fprintf(
            dbgout,
            b"%s\0" as *const u8 as *const libc::c_char,
            b"reached EOF\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn rmt_write(mut fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    vfprintf(stdout, fmt, ap.as_va_list());
    fflush_unlocked(stdout);
    if !dbgout.is_null() && 10 as libc::c_int <= dbglev {
        let mut aptr: ::std::ffi::VaListImpl;
        aptr = args.clone();
        fprintf(
            dbgout,
            b"%s\0" as *const u8 as *const libc::c_char,
            b"S: \0" as *const u8 as *const libc::c_char,
        );
        vfprintf(dbgout, fmt, aptr.as_va_list());
    }
}
unsafe extern "C" fn rmt_reply(mut code: uintmax_t) {
    rmt_write(b"A%ju\n\0" as *const u8 as *const libc::c_char, code);
}
unsafe extern "C" fn rmt_error_message(
    mut code: libc::c_int,
    mut msg: *const libc::c_char,
) {
    if !dbgout.is_null() && 10 as libc::c_int <= dbglev {
        fprintf(dbgout, b"S: E%d\n\0" as *const u8 as *const libc::c_char, code);
    }
    if !dbgout.is_null() && 10 as libc::c_int <= dbglev {
        fprintf(dbgout, b"S: %s\n\0" as *const u8 as *const libc::c_char, msg);
    }
    if !dbgout.is_null() && 1 as libc::c_int <= dbglev {
        fprintf(dbgout, b"error: %s\n\0" as *const u8 as *const libc::c_char, msg);
    }
    fprintf(stdout, b"E%d\n%s\n\0" as *const u8 as *const libc::c_char, code, msg);
    fflush_unlocked(stdout);
}
unsafe extern "C" fn rmt_error(mut code: libc::c_int) {
    rmt_error_message(code, strerror(code));
}
pub static mut record_buffer_ptr: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut record_buffer_size: size_t = 0;
unsafe extern "C" fn prepare_record_buffer(mut size: size_t) {
    if size > record_buffer_size {
        record_buffer_ptr = xrealloc(record_buffer_ptr as *mut libc::c_void, size)
            as *mut libc::c_char;
        record_buffer_size = size;
    }
}
pub static mut device_fd: libc::c_int = -(1 as libc::c_int);
unsafe extern "C" fn xlat_kw(
    mut s: *const libc::c_char,
    mut pfx: *const libc::c_char,
    mut kw: *const rmt_kw,
    mut valp: *mut libc::c_int,
    mut endp: *mut *const libc::c_char,
) -> libc::c_int {
    let mut slen: size_t = strlen(s);
    if !pfx.is_null() {
        let mut pfxlen: size_t = strlen(pfx);
        if slen > pfxlen
            && memcmp(s as *const libc::c_void, pfx as *const libc::c_void, pfxlen)
                == 0 as libc::c_int
        {
            s = s.offset(pfxlen as isize);
            slen = (slen as libc::c_ulong).wrapping_sub(pfxlen) as size_t as size_t;
        }
    }
    while !((*kw).name).is_null() {
        if slen >= (*kw).len
            && memcmp(
                (*kw).name as *const libc::c_void,
                s as *const libc::c_void,
                (*kw).len,
            ) == 0 as libc::c_int
            && !(*s.offset((*kw).len as isize) as libc::c_int != 0
                && c_isalnum(*s.offset((*kw).len as isize) as libc::c_int) as libc::c_int
                    != 0)
        {
            *valp = (*kw).value;
            *endp = s.offset((*kw).len as isize);
            return 0 as libc::c_int;
        }
        kw = kw.offset(1);
        kw;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn skip_ws(mut s: *const libc::c_char) -> *const libc::c_char {
    while *s as libc::c_int != 0 && c_isblank(*s as libc::c_int) as libc::c_int != 0 {
        s = s.offset(1);
        s;
    }
    return s;
}
static mut open_flag_kw: [rmt_kw; 14] = [rmt_kw {
    name: 0 as *const libc::c_char,
    len: 0,
    value: 0,
}; 14];
unsafe extern "C" fn decode_open_flag(
    mut mstr: *const libc::c_char,
    mut pmode: *mut libc::c_int,
) -> libc::c_int {
    let mut numeric_mode: libc::c_int = 0 as libc::c_int;
    let mut mode: libc::c_int = 0 as libc::c_int;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    mstr = skip_ws(mstr);
    if c_isdigit(*mstr as libc::c_int) {
        numeric_mode = rpl_strtol(
            mstr,
            &mut p as *mut *const libc::c_char as *mut *mut libc::c_char,
            10 as libc::c_int,
        ) as libc::c_int;
        mstr = skip_ws(p);
    }
    if *mstr != 0 {
        while !mstr.is_null() {
            let mut v: libc::c_int = 0;
            mstr = skip_ws(mstr);
            if *mstr as libc::c_int == 0 as libc::c_int {
                break;
            }
            if c_isdigit(*mstr as libc::c_int) {
                v = rpl_strtol(
                    mstr,
                    &mut p as *mut *const libc::c_char as *mut *mut libc::c_char,
                    10 as libc::c_int,
                ) as libc::c_int;
            } else if xlat_kw(
                mstr,
                b"O_\0" as *const u8 as *const libc::c_char,
                open_flag_kw.as_ptr(),
                &mut v,
                &mut p,
            ) != 0
            {
                rmt_error_message(
                    22 as libc::c_int,
                    b"invalid open mode\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            mode |= v;
            if *p as libc::c_int != 0 && c_isblank(*p as libc::c_int) as libc::c_int != 0
            {
                p = skip_ws(p);
            }
            if *p as libc::c_int == 0 as libc::c_int {
                break;
            }
            if *p as libc::c_int == '|' as i32 {
                mstr = p.offset(1 as libc::c_int as isize);
            } else {
                rmt_error_message(
                    22 as libc::c_int,
                    b"invalid open mode\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
        }
    } else {
        mode = numeric_mode;
    }
    *pmode = mode;
    return 0 as libc::c_int;
}
unsafe extern "C" fn open_device(mut str: *mut libc::c_char) {
    let mut device: *mut libc::c_char = xstrdup(str);
    let mut flag_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut flag: libc::c_int = 0;
    flag_str = rmt_read();
    if flag_str.is_null() {
        if !dbgout.is_null() && 1 as libc::c_int <= dbglev {
            fprintf(
                dbgout,
                b"%s\0" as *const u8 as *const libc::c_char,
                b"unexpected EOF\0" as *const u8 as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    }
    if decode_open_flag(flag_str, &mut flag) == 0 as libc::c_int {
        if device_fd >= 0 as libc::c_int {
            close(device_fd);
        }
        device_fd = open(
            device,
            flag,
            0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                | (0o400 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
                    | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int),
        );
        if device_fd < 0 as libc::c_int {
            rmt_error(*__errno_location());
        } else {
            rmt_reply(0 as libc::c_int as uintmax_t);
        }
    }
    rpl_free(device as *mut libc::c_void);
}
unsafe extern "C" fn close_device() {
    if close(device_fd) < 0 as libc::c_int {
        rmt_error(*__errno_location());
    } else {
        device_fd = -(1 as libc::c_int);
        rmt_reply(0 as libc::c_int as uintmax_t);
    };
}
static mut seek_whence_kw: [rmt_kw; 4] = [rmt_kw {
    name: 0 as *const libc::c_char,
    len: 0,
    value: 0,
}; 4];
unsafe extern "C" fn lseek_device(mut str: *const libc::c_char) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut whence: libc::c_int = 0;
    let mut off: off_t = 0;
    let mut n: uintmax_t = 0;
    if *str.offset(0 as libc::c_int as isize) as libc::c_int != 0
        && *str.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
    {
        match *str.offset(0 as libc::c_int as isize) as libc::c_int {
            48 => {
                whence = 0 as libc::c_int;
            }
            49 => {
                whence = 1 as libc::c_int;
            }
            50 => {
                whence = 2 as libc::c_int;
            }
            _ => {
                rmt_error_message(
                    22 as libc::c_int,
                    b"Seek direction out of range\0" as *const u8 as *const libc::c_char,
                );
                return;
            }
        }
    } else if xlat_kw(
        str,
        b"SEEK_\0" as *const u8 as *const libc::c_char,
        seek_whence_kw.as_ptr(),
        &mut whence,
        &mut p as *mut *mut libc::c_char as *mut *const libc::c_char,
    ) != 0
    {
        rmt_error_message(
            22 as libc::c_int,
            b"Invalid seek direction\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    str = rmt_read();
    off = strtoumax(str, &mut p, 10 as libc::c_int) as off_t;
    n = off as uintmax_t;
    if *p != 0 {
        rmt_error_message(
            22 as libc::c_int,
            b"Invalid seek offset\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if n != off as libc::c_ulong || *__errno_location() == 34 as libc::c_int {
        rmt_error_message(
            22 as libc::c_int,
            b"Seek offset out of range\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    off = lseek(device_fd, off, whence);
    if off < 0 as libc::c_int as libc::c_long {
        rmt_error(*__errno_location());
    } else {
        rmt_reply(off as uintmax_t);
    };
}
unsafe extern "C" fn read_device(mut str: *const libc::c_char) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: size_t = 0;
    let mut n: uintmax_t = 0;
    let mut status: size_t = 0;
    size = strtoumax(str, &mut p, 10 as libc::c_int);
    n = size;
    if *p != 0 {
        rmt_error_message(
            22 as libc::c_int,
            b"Invalid byte count\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if n != size || *__errno_location() == 34 as libc::c_int {
        rmt_error_message(
            22 as libc::c_int,
            b"Byte count out of range\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    prepare_record_buffer(size);
    status = safe_read(device_fd, record_buffer_ptr as *mut libc::c_void, size);
    if status == -(1 as libc::c_int) as size_t {
        rmt_error(*__errno_location());
    } else {
        rmt_reply(status);
        full_write(1 as libc::c_int, record_buffer_ptr as *const libc::c_void, status);
    };
}
unsafe extern "C" fn write_device(mut str: *const libc::c_char) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: size_t = 0;
    let mut n: uintmax_t = 0;
    let mut status: size_t = 0;
    size = strtoumax(str, &mut p, 10 as libc::c_int);
    n = size;
    if *p != 0 {
        rmt_error_message(
            22 as libc::c_int,
            b"Invalid byte count\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if n != size || *__errno_location() == 34 as libc::c_int {
        rmt_error_message(
            22 as libc::c_int,
            b"Byte count out of range\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    prepare_record_buffer(size);
    if (if 0 != 0 && 0 != 0
        && size.wrapping_mul(1 as libc::c_int as size_t)
            <= 8 as libc::c_int as libc::c_ulong
        && size != 0 as libc::c_int as libc::c_ulong
    {
        {
            let mut __ptr: *mut libc::c_char = record_buffer_ptr;
            let mut __stream: *mut FILE = stdin;
            let mut __cnt: size_t = 0;
            __cnt = size.wrapping_mul(1 as libc::c_int as size_t);
            while __cnt > 0 as libc::c_int as libc::c_ulong {
                let mut __c: libc::c_int = getc_unlocked(__stream);
                if __c == -(1 as libc::c_int) {
                    break;
                }
                let fresh1 = __ptr;
                __ptr = __ptr.offset(1);
                *fresh1 = __c as libc::c_char;
                __cnt = __cnt.wrapping_sub(1);
                __cnt;
            }
            size.wrapping_mul(1 as libc::c_int as size_t)
                .wrapping_sub(__cnt)
                .wrapping_div(size)
        }
    } else {
        if 0 != 0 && size == 0 as libc::c_int as libc::c_ulong
            || 0 != 0 && 1 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
        {
            0 as libc::c_int as size_t
        } else {
            fread_unlocked(
                record_buffer_ptr as *mut libc::c_void,
                size,
                1 as libc::c_int as size_t,
                stdin,
            )
        }
    }) != 1 as libc::c_int as libc::c_ulong
    {
        if feof_unlocked(stdin) != 0 {
            rmt_error_message(
                5 as libc::c_int,
                b"Premature eof\0" as *const u8 as *const libc::c_char,
            );
        } else {
            rmt_error(*__errno_location());
        }
        return;
    }
    status = full_write(device_fd, record_buffer_ptr as *const libc::c_void, size);
    if status != size {
        rmt_error(*__errno_location());
    } else {
        rmt_reply(status);
    };
}
unsafe extern "C" fn iocop_device(mut str: *const libc::c_char) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut opcode: libc::c_long = 0;
    let mut count: off_t = 0;
    let mut n: uintmax_t = 0;
    opcode = rpl_strtol(str, &mut p, 10 as libc::c_int);
    if *p != 0 {
        rmt_error_message(
            22 as libc::c_int,
            b"Invalid operation code\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    str = rmt_read();
    count = strtoumax(str, &mut p, 10 as libc::c_int) as off_t;
    n = count as uintmax_t;
    if *p != 0 {
        rmt_error_message(
            22 as libc::c_int,
            b"Invalid byte count\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if n != count as libc::c_ulong || *__errno_location() == 34 as libc::c_int {
        rmt_error_message(
            22 as libc::c_int,
            b"Byte count out of range\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    let mut mtop: mtop = mtop { mt_op: 0, mt_count: 0 };
    mtop.mt_count = count as libc::c_int;
    if mtop.mt_count as libc::c_long != count {
        rmt_error_message(
            22 as libc::c_int,
            b"Byte count out of range\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    mtop.mt_op = opcode as libc::c_short;
    if ioctl(
        device_fd,
        ((1 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('m' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<mtop>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut mtop as *mut mtop as *mut libc::c_char,
    ) < 0 as libc::c_int
    {
        rmt_error(*__errno_location());
    } else {
        rmt_reply(0 as libc::c_int as uintmax_t);
    };
}
unsafe extern "C" fn status_device(mut str: *const libc::c_char) {
    if *str != 0 {
        rmt_error_message(
            22 as libc::c_int,
            b"Unexpected arguments\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    let mut mtget: mtget = mtget {
        mt_type: 0,
        mt_resid: 0,
        mt_dsreg: 0,
        mt_gstat: 0,
        mt_erreg: 0,
        mt_fileno: 0,
        mt_blkno: 0,
    };
    if ioctl(
        device_fd,
        ((2 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('m' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((2 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<mtget>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut mtget as *mut mtget as *mut libc::c_char,
    ) < 0 as libc::c_int
    {
        rmt_error(*__errno_location());
    } else {
        rmt_reply(::std::mem::size_of::<mtget>() as libc::c_ulong);
        full_write(
            1 as libc::c_int,
            &mut mtget as *mut mtget as *mut libc::c_char as *const libc::c_void,
            ::std::mem::size_of::<mtget>() as libc::c_ulong,
        );
    };
}
pub static mut argp_program_version: *const libc::c_char = b"rmt (GNU cpio) 2.14\0"
    as *const u8 as *const libc::c_char;
pub static mut argp_program_bug_address: *const libc::c_char = b"<bug-cpio@gnu.org>\0"
    as *const u8 as *const libc::c_char;
static mut doc: [libc::c_char; 66] = unsafe {
    *::std::mem::transmute::<
        &[u8; 66],
        &[libc::c_char; 66],
    >(b"Manipulate a tape drive, accepting commands from a remote process\0")
};
static mut options: [argp_option; 3] = [
    {
        let mut init = argp_option {
            name: b"debug\0" as *const u8 as *const libc::c_char,
            key: 'd' as i32,
            arg: b"NUMBER\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"set debug level\0" as *const u8 as *const libc::c_char,
            group: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"debug-file\0" as *const u8 as *const libc::c_char,
            key: DEBUG_FILE_OPTION as libc::c_int,
            arg: b"FILE\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"set debug output file name\0" as *const u8 as *const libc::c_char,
            group: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0,
            arg: 0 as *const libc::c_char,
            flags: 0,
            doc: 0 as *const libc::c_char,
            group: 0,
        };
        init
    },
];
unsafe extern "C" fn parse_opt(
    mut key: libc::c_int,
    mut arg: *mut libc::c_char,
    mut state: *mut argp_state,
) -> error_t {
    match key {
        100 => {
            dbglev = rpl_strtol(arg, 0 as *mut *mut libc::c_char, 0 as libc::c_int)
                as libc::c_int;
        }
        256 => {
            dbgout = fopen(arg, b"w\0" as *const u8 as *const libc::c_char);
            if dbgout.is_null() {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot open %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    arg,
                );
            }
        }
        16777223 => {
            if dbglev != 0 {
                if dbgout.is_null() {
                    dbgout = stderr;
                }
            } else if !dbgout.is_null() {
                dbglev = 1 as libc::c_int;
            }
        }
        _ => return 7 as libc::c_int,
    }
    return 0 as libc::c_int;
}
static mut argp: argp = unsafe {
    {
        let mut init = argp {
            options: options.as_ptr() as *mut _,
            parser: Some(
                parse_opt
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *mut libc::c_char,
                        *mut argp_state,
                    ) -> error_t,
            ),
            args_doc: 0 as *const libc::c_char,
            doc: doc.as_ptr(),
            children: 0 as *const argp_child,
            help_filter: None,
            argp_domain: 0 as *const libc::c_char,
        };
        init
    }
};
static mut rmt_authors: [*const libc::c_char; 2] = [
    b"Sergey Poznyakoff\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
pub unsafe extern "C" fn xalloc_die() {
    rmt_error(12 as libc::c_int);
    exit(1 as libc::c_int);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut idx: libc::c_int = 0;
    let mut stop: libc::c_int = 0 as libc::c_int;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    argp_version_setup(
        b"rmt\0" as *const u8 as *const libc::c_char,
        rmt_authors.as_mut_ptr(),
    );
    if isatty(1 as libc::c_int) != 0 {
        setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
        bindtextdomain(
            b"cpio\0" as *const u8 as *const libc::c_char,
            b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
        );
        textdomain(b"cpio\0" as *const u8 as *const libc::c_char);
    }
    if argp_parse(
        &mut argp,
        argc,
        argv,
        0x8 as libc::c_int as libc::c_uint,
        &mut idx,
        0 as *mut libc::c_void,
    ) != 0
    {
        exit(1 as libc::c_int);
    }
    if idx != argc {
        if idx != argc - 1 as libc::c_int {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"too many arguments\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        dbgout = fopen(
            *argv.offset(idx as isize),
            b"w\0" as *const u8 as *const libc::c_char,
        );
        if dbgout.is_null() {
            error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot open %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                *argv.offset(idx as isize),
            );
        }
        dbglev = 1 as libc::c_int;
    }
    while stop == 0
        && {
            buf = rmt_read();
            !buf.is_null()
        }
    {
        match *buf.offset(0 as libc::c_int as isize) as libc::c_int {
            67 => {
                close_device();
                stop = 1 as libc::c_int;
            }
            73 => {
                iocop_device(buf.offset(1 as libc::c_int as isize));
            }
            76 => {
                lseek_device(buf.offset(1 as libc::c_int as isize));
            }
            79 => {
                open_device(buf.offset(1 as libc::c_int as isize));
            }
            82 => {
                read_device(buf.offset(1 as libc::c_int as isize));
            }
            83 => {
                status_device(buf.offset(1 as libc::c_int as isize));
            }
            87 => {
                write_device(buf.offset(1 as libc::c_int as isize));
            }
            _ => {
                if !dbgout.is_null() && 1 as libc::c_int <= dbglev {
                    fprintf(
                        dbgout,
                        b"garbage input %s\n\0" as *const u8 as *const libc::c_char,
                        buf,
                    );
                }
                rmt_error_message(
                    22 as libc::c_int,
                    b"Garbage command\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
        }
    }
    if device_fd >= 0 as libc::c_int {
        close_device();
    }
    rpl_free(input_buf_ptr as *mut libc::c_void);
    rpl_free(record_buffer_ptr as *mut libc::c_void);
    return 0 as libc::c_int;
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
unsafe extern "C" fn run_static_initializers() {
    open_flag_kw = [
        {
            let mut init = rmt_kw {
                name: b"APPEND\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                value: 0o2000 as libc::c_int,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: b"CREAT\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                value: 0o100 as libc::c_int,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: b"DSYNC\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                value: 0o10000 as libc::c_int,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: b"EXCL\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                value: 0o200 as libc::c_int,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: b"LARGEFILE\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: b"NOCTTY\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                value: 0o400 as libc::c_int,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: b"NONBLOCK\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                value: 0o4000 as libc::c_int,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: b"RDONLY\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: b"RDWR\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                value: 0o2 as libc::c_int,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: b"RSYNC\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                value: 0o4010000 as libc::c_int,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: b"SYNC\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                value: 0o4010000 as libc::c_int,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: b"TRUNC\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                value: 0o1000 as libc::c_int,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: b"WRONLY\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                value: 0o1 as libc::c_int,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: 0 as *const libc::c_char,
                len: 0,
                value: 0,
            };
            init
        },
    ];
    seek_whence_kw = [
        {
            let mut init = rmt_kw {
                name: b"SET\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: b"CUR\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: b"END\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                value: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = rmt_kw {
                name: 0 as *const libc::c_char,
                len: 0,
                value: 0,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
