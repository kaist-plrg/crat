use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type bc_struct;
    pub type dc_string;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn dc_array_init();
    fn dc_free_str(_: *mut dc_str);
    fn dc_math_init();
    fn dc_register_init();
    fn dc_string_init();
    fn dc_evalfile(_: *mut FILE) -> libc::c_int;
    fn dc_evalstr(_: *mut dc_data) -> libc::c_int;
    fn dc_makestring(_: *const libc::c_char, _: size_t) -> dc_data;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type dc_value_type = libc::c_uint;
pub const DC_STRING: dc_value_type = 2;
pub const DC_NUMBER: dc_value_type = 1;
pub const DC_UNINITIALIZED: dc_value_type = 0;
pub type dc_num = *mut bc_struct;
pub type dc_str = *mut dc_string;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dc_data {
    pub dc_type: dc_value_type,
    pub v: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub number: dc_num,
    pub string: dc_str,
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
pub static mut progname: *const libc::c_char = 0 as *const libc::c_char;
unsafe extern "C" fn bug_report_info() {
    printf(
        b"Email bug reports to:  bug-dc@gnu.org .\n\0" as *const u8
            as *const libc::c_char,
    );
}
unsafe extern "C" fn show_version() {
    printf(
        b"dc (GNU %s %s) %s\n\0" as *const u8 as *const libc::c_char,
        b"bc\0" as *const u8 as *const libc::c_char,
        b"1.07.1\0" as *const u8 as *const libc::c_char,
        b"1.4.1\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"\n%s\nThis is free software; see the source for copying conditions.  There is NO\nwarranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE,\nto the extent permitted by law.\n\0"
            as *const u8 as *const libc::c_char,
        b"Copyright 1994, 1997, 1998, 2000, 2001, 2003-2006, 2008, 2010, 2012-2017 Free Software Foundation, Inc.\0"
            as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn usage(mut f: *mut FILE) {
    fprintf(
        f,
        b"Usage: %s [OPTION] [file ...]\n  -e, --expression=EXPR    evaluate expression\n  -f, --file=FILE          evaluate contents of file\n  -h, --help               display this help and exit\n  -V, --version            output version information and exit\n\n\0"
            as *const u8 as *const libc::c_char,
        progname,
    );
    bug_report_info();
}
unsafe extern "C" fn r1bindex(
    mut s: *mut libc::c_char,
    mut c: libc::c_int,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = strrchr(s, c);
    if p.is_null() {
        return s;
    }
    return p.offset(1 as libc::c_int as isize);
}
unsafe extern "C" fn try_file(mut filename: *const libc::c_char) {
    let mut input: *mut FILE = 0 as *mut FILE;
    if strcmp(filename, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        input = stdin;
    } else {
        input = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
        if input.is_null() {
            fprintf(
                stderr,
                b"%s: Could not open file %s\n\0" as *const u8 as *const libc::c_char,
                progname,
                filename,
            );
            return;
        }
    }
    let mut s: stat = stat {
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
    if fstat(fileno(input), &mut s) == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"%s: Could not fstat file \0" as *const u8 as *const libc::c_char,
            progname,
        );
        perror(filename);
    } else if s.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint
    {
        fprintf(
            stderr,
            b"%s: Will not attempt to process directory %s\n\0" as *const u8
                as *const libc::c_char,
            progname,
            filename,
        );
    } else if s.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o60000 as libc::c_int as libc::c_uint
    {
        fprintf(
            stderr,
            b"%s: Will not attempt to process block-special file %s\n\0" as *const u8
                as *const libc::c_char,
            progname,
            filename,
        );
    } else if !(s.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint)
        && !(s.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o20000 as libc::c_int as libc::c_uint)
        && !(s.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o10000 as libc::c_int as libc::c_uint)
        && !(s.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o140000 as libc::c_int as libc::c_uint)
    {
        fprintf(
            stderr,
            b"%s: Will not attempt to process file of unusual type: %s\n\0" as *const u8
                as *const libc::c_char,
            progname,
            filename,
        );
    } else if dc_evalfile(input) != 0 as libc::c_int {
        exit(1 as libc::c_int);
    }
    if input != stdin {
        fclose(input);
    }
}
unsafe extern "C" fn flush_okay() -> libc::c_int {
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    let mut r: libc::c_int = 0 as libc::c_int;
    if ferror(stdout) != 0 {
        errmsg = b"error writing to stdout\0" as *const u8 as *const libc::c_char;
    } else if fflush(stdout) != 0 {
        errmsg = b"error flushing stdout\0" as *const u8 as *const libc::c_char;
    } else if fclose(stdout) != 0 {
        errmsg = b"error closing stdout\0" as *const u8 as *const libc::c_char;
    }
    if !errmsg.is_null() {
        fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, progname);
        perror(errmsg);
        r = 1 as libc::c_int;
    }
    if ferror(stderr) != 0 || fclose(stderr) != 0 {
        r = 1 as libc::c_int;
    }
    return r;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    static mut long_opts: [option; 5] = [
        {
            let mut init = option {
                name: b"expression\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'e' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"file\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'f' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"help\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'h' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"version\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'V' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: 0 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
    ];
    let mut did_eval: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_int = 0;
    progname = r1bindex(*argv, '/' as i32);
    dc_math_init();
    dc_string_init();
    dc_register_init();
    dc_array_init();
    loop {
        c = getopt_long(
            argc,
            argv,
            b"hVe:f:\0" as *const u8 as *const libc::c_char,
            long_opts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            101 => {
                let mut string: dc_data = dc_makestring(optarg, strlen(optarg));
                if dc_evalstr(&mut string) != 0 as libc::c_int {
                    return flush_okay();
                }
                dc_free_str(&mut string.v.string);
                did_eval = 1 as libc::c_int;
            }
            102 => {
                try_file(optarg);
                did_eval = 1 as libc::c_int;
            }
            104 => {
                usage(stdout);
                return flush_okay();
            }
            86 => {
                show_version();
                return flush_okay();
            }
            _ => {
                usage(stderr);
                return 1 as libc::c_int;
            }
        }
    }
    while optind < argc {
        try_file(*argv.offset(optind as isize));
        did_eval = 1 as libc::c_int;
        optind += 1;
        optind;
    }
    if did_eval == 0 as libc::c_int {
        if dc_evalfile(stdin) != 0 as libc::c_int {
            return 1 as libc::c_int;
        }
    }
    return flush_okay();
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
