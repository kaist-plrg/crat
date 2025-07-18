use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn perror(__s: *const libc::c_char);
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn parse_lesskey(
        infile_0: *mut libc::c_char,
        tables: *mut lesskey_tables,
    ) -> libc::c_int;
    static mut version: [libc::c_char; 0];
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub struct xbuffer {
    pub data: *mut libc::c_uchar,
    pub end: libc::c_int,
    pub size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lesskey_cmdname {
    pub cn_name: *mut libc::c_char,
    pub cn_action: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lesskey_table {
    pub names: *mut lesskey_cmdname,
    pub buf: xbuffer,
    pub is_var: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lesskey_tables {
    pub currtable: *mut lesskey_table,
    pub cmdtable: lesskey_table,
    pub edittable: lesskey_table,
    pub vartable: lesskey_table,
}
pub static mut fileheader: [libc::c_char; 4] = [
    '\0' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
];
pub static mut filetrailer: [libc::c_char; 3] = [
    'E' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
];
pub static mut cmdsection: [libc::c_char; 1] = ['c' as i32 as libc::c_char];
pub static mut editsection: [libc::c_char; 1] = ['e' as i32 as libc::c_char];
pub static mut varsection: [libc::c_char; 1] = ['v' as i32 as libc::c_char];
pub static mut endsection: [libc::c_char; 1] = ['x' as i32 as libc::c_char];
pub static mut infile: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut outfile: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
unsafe extern "C" fn usage() {
    fprintf(
        stderr,
        b"usage: lesskey [-o output] [input]\n\0" as *const u8 as *const libc::c_char,
    );
    exit(1 as libc::c_int);
}
pub unsafe extern "C" fn lesskey_parse_error(mut s: *mut libc::c_char) {
    fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, s);
}
pub unsafe extern "C" fn lstrtoi(
    mut buf: *mut libc::c_char,
    mut ebuf: *mut *mut libc::c_char,
    mut radix: libc::c_int,
) -> libc::c_int {
    return strtol(buf, ebuf, radix) as libc::c_int;
}
pub unsafe extern "C" fn out_of_memory() {
    fprintf(
        stderr,
        b"lesskey: cannot allocate memory\n\0" as *const u8 as *const libc::c_char,
    );
    exit(1 as libc::c_int);
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
unsafe extern "C" fn mkpathname(
    mut dirname: *mut libc::c_char,
    mut filename: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut pathname: *mut libc::c_char = 0 as *mut libc::c_char;
    pathname = ecalloc(
        (strlen(dirname))
            .wrapping_add(strlen(filename))
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_uint,
    ) as *mut libc::c_char;
    strcpy(pathname, dirname);
    strcat(pathname, b"/\0" as *const u8 as *const libc::c_char);
    strcat(pathname, filename);
    return pathname;
}
pub unsafe extern "C" fn homefile(mut filename: *mut libc::c_char) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pathname: *mut libc::c_char = 0 as *mut libc::c_char;
    p = getenv(b"HOME\0" as *const u8 as *const libc::c_char);
    if !p.is_null() && *p as libc::c_int != '\0' as i32 {
        pathname = mkpathname(p, filename);
    } else {
        fprintf(
            stderr,
            b"cannot find $HOME - using current directory\n\0" as *const u8
                as *const libc::c_char,
        );
        pathname = mkpathname(
            b".\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            filename,
        );
    }
    return pathname;
}
unsafe extern "C" fn parse_args(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    outfile = 0 as *mut libc::c_char;
    loop {
        argc -= 1;
        if !(argc > 0 as libc::c_int) {
            break;
        }
        argv = argv.offset(1);
        arg = *argv;
        if *arg.offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32 {
            break;
        } else {
            if *arg.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
                break;
            }
            if *arg.offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32
                && *arg.offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32
            {
                argc -= 1;
                argc;
                argv = argv.offset(1);
                argv;
                break;
            } else {
                let mut current_block_18: u64;
                match *arg.offset(1 as libc::c_int as isize) as libc::c_int {
                    45 => {
                        if strncmp(
                            arg,
                            b"--output\0" as *const u8 as *const libc::c_char,
                            8 as libc::c_int as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            if *arg.offset(8 as libc::c_int as isize) as libc::c_int
                                == '\0' as i32
                            {
                                outfile = &mut *arg.offset(8 as libc::c_int as isize)
                                    as *mut libc::c_char;
                            } else if *arg.offset(8 as libc::c_int as isize)
                                as libc::c_int == '=' as i32
                            {
                                outfile = &mut *arg.offset(9 as libc::c_int as isize)
                                    as *mut libc::c_char;
                            } else {
                                usage();
                            }
                            current_block_18 = 11067178304109865769;
                        } else if strcmp(
                            arg,
                            b"--version\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        {
                            current_block_18 = 5712543785632247906;
                        } else {
                            usage();
                            current_block_18 = 9828876828309294594;
                        }
                    }
                    111 => {
                        outfile = &mut *(*argv.offset(0 as libc::c_int as isize))
                            .offset(2 as libc::c_int as isize) as *mut libc::c_char;
                        current_block_18 = 11067178304109865769;
                    }
                    86 => {
                        current_block_18 = 5712543785632247906;
                    }
                    _ => {
                        usage();
                        current_block_18 = 9828876828309294594;
                    }
                }
                match current_block_18 {
                    5712543785632247906 => {
                        printf(
                            b"lesskey  version %s\n\0" as *const u8
                                as *const libc::c_char,
                            version.as_mut_ptr(),
                        );
                        exit(0 as libc::c_int);
                    }
                    11067178304109865769 => {
                        if *outfile as libc::c_int == '\0' as i32 {
                            argc -= 1;
                            if argc <= 0 as libc::c_int {
                                usage();
                            }
                            argv = argv.offset(1);
                            outfile = *argv;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    if argc > 1 as libc::c_int {
        usage();
    }
    if argc > 0 as libc::c_int {
        infile = *argv;
    }
}
unsafe extern "C" fn fputbytes(
    mut fd: *mut FILE,
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
) {
    loop {
        let fresh0 = len;
        len = len - 1;
        if !(fresh0 > 0 as libc::c_int) {
            break;
        }
        fwrite(
            buf as *const libc::c_void,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            fd,
        );
        buf = buf.offset(1);
        buf;
    };
}
unsafe extern "C" fn fputint(mut fd: *mut FILE, mut val: libc::c_uint) {
    let mut c: libc::c_char = 0;
    if val >= (64 as libc::c_int * 64 as libc::c_int) as libc::c_uint {
        fprintf(
            stderr,
            b"error: cannot write %d, max %d\n\0" as *const u8 as *const libc::c_char,
            val,
            64 as libc::c_int * 64 as libc::c_int,
        );
        exit(1 as libc::c_int);
    }
    c = val.wrapping_rem(64 as libc::c_int as libc::c_uint) as libc::c_char;
    fwrite(
        &mut c as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        fd,
    );
    c = val.wrapping_div(64 as libc::c_int as libc::c_uint) as libc::c_char;
    fwrite(
        &mut c as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        fd,
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut tables: lesskey_tables = lesskey_tables {
        currtable: 0 as *mut lesskey_table,
        cmdtable: lesskey_table {
            names: 0 as *mut lesskey_cmdname,
            buf: xbuffer {
                data: 0 as *mut libc::c_uchar,
                end: 0,
                size: 0,
            },
            is_var: 0,
        },
        edittable: lesskey_table {
            names: 0 as *mut lesskey_cmdname,
            buf: xbuffer {
                data: 0 as *mut libc::c_uchar,
                end: 0,
                size: 0,
            },
            is_var: 0,
        },
        vartable: lesskey_table {
            names: 0 as *mut lesskey_cmdname,
            buf: xbuffer {
                data: 0 as *mut libc::c_uchar,
                end: 0,
                size: 0,
            },
            is_var: 0,
        },
    };
    let mut out: *mut FILE = 0 as *mut FILE;
    let mut errors: libc::c_int = 0;
    parse_args(argc, argv);
    errors = parse_lesskey(infile, &mut tables);
    if errors != 0 {
        fprintf(
            stderr,
            b"%d errors; no output produced\n\0" as *const u8 as *const libc::c_char,
            errors,
        );
        return 1 as libc::c_int;
    }
    fprintf(
        stderr,
        b"NOTE: lesskey is deprecated.\n      It is no longer necessary to run lesskey,\n      when using less version 582 and later.\n\0"
            as *const u8 as *const libc::c_char,
    );
    if outfile.is_null() {
        outfile = getenv(b"LESSKEY\0" as *const u8 as *const libc::c_char);
    }
    if outfile.is_null() {
        outfile = homefile(
            b".less\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    out = fopen(outfile, b"wb\0" as *const u8 as *const libc::c_char);
    if out.is_null() {
        perror(outfile);
        return 1 as libc::c_int;
    }
    fputbytes(
        out,
        fileheader.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as libc::c_int,
    );
    fputbytes(
        out,
        cmdsection.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong as libc::c_int,
    );
    fputint(out, tables.cmdtable.buf.end as libc::c_uint);
    fputbytes(
        out,
        tables.cmdtable.buf.data as *mut libc::c_char,
        tables.cmdtable.buf.end,
    );
    fputbytes(
        out,
        editsection.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong as libc::c_int,
    );
    fputint(out, tables.edittable.buf.end as libc::c_uint);
    fputbytes(
        out,
        tables.edittable.buf.data as *mut libc::c_char,
        tables.edittable.buf.end,
    );
    fputbytes(
        out,
        varsection.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong as libc::c_int,
    );
    fputint(out, tables.vartable.buf.end as libc::c_uint);
    fputbytes(
        out,
        tables.vartable.buf.data as *mut libc::c_char,
        tables.vartable.buf.end,
    );
    fputbytes(
        out,
        endsection.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong as libc::c_int,
    );
    fputbytes(
        out,
        filetrailer.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as libc::c_int,
    );
    fclose(out);
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
