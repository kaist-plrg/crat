use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn ap_init(
        ap: *mut Arg_parser,
        argc: libc::c_int,
        argv: *const *const libc::c_char,
        options: *const ap_Option,
        in_order: libc::c_char,
    ) -> libc::c_char;
    fn ap_free(ap: *mut Arg_parser);
    fn ap_error(ap: *const Arg_parser) -> *const libc::c_char;
    fn ap_arguments(ap: *const Arg_parser) -> libc::c_int;
    fn ap_code(ap: *const Arg_parser, i: libc::c_int) -> libc::c_int;
    fn ap_argument(ap: *const Arg_parser, i: libc::c_int) -> *const libc::c_char;
    fn init_buffers() -> bool_0;
    fn read_file(filename: *const libc::c_char, addr: libc::c_int) -> libc::c_int;
    fn main_loop(initial_error: bool_0, loose: bool_0) -> libc::c_int;
    fn set_def_filename(s: *const libc::c_char) -> bool_0;
    fn set_error_msg(msg: *const libc::c_char);
    fn set_prompt(s: *const libc::c_char) -> bool_0;
    fn set_verbose();
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
pub type ap_Has_arg = libc::c_uint;
pub const ap_maybe: ap_Has_arg = 2;
pub const ap_yes: ap_Has_arg = 1;
pub const ap_no: ap_Has_arg = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ap_Option {
    pub code: libc::c_int,
    pub long_name: *const libc::c_char,
    pub has_arg: ap_Has_arg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ap_Record {
    pub code: libc::c_int,
    pub parsed_name: *mut libc::c_char,
    pub argument: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Arg_parser {
    pub data: *mut ap_Record,
    pub error: *mut libc::c_char,
    pub data_size: libc::c_int,
    pub error_size: libc::c_int,
}
pub type Bool = libc::c_uint;
pub const true_0: Bool = 1;
pub const false_0: Bool = 0;
pub type bool_0 = Bool;
pub const opt_cr: C2RustUnnamed = 256;
pub type C2RustUnnamed = libc::c_uint;
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
static mut program_name: *const libc::c_char = b"ed\0" as *const u8
    as *const libc::c_char;
static mut program_year: *const libc::c_char = b"2023\0" as *const u8
    as *const libc::c_char;
static mut invocation_name: *const libc::c_char = b"ed\0" as *const u8
    as *const libc::c_char;
static mut extended_regexp_: bool_0 = false_0;
static mut quiet_: bool_0 = false_0;
static mut restricted_: bool_0 = false_0;
static mut scripted_: bool_0 = false_0;
static mut strip_cr_: bool_0 = false_0;
static mut traditional_: bool_0 = false_0;
pub unsafe extern "C" fn extended_regexp() -> bool_0 {
    return extended_regexp_;
}
pub unsafe extern "C" fn restricted() -> bool_0 {
    return restricted_;
}
pub unsafe extern "C" fn scripted() -> bool_0 {
    return scripted_;
}
pub unsafe extern "C" fn strip_cr() -> bool_0 {
    return strip_cr_;
}
pub unsafe extern "C" fn traditional() -> bool_0 {
    return traditional_;
}
unsafe extern "C" fn show_help() {
    printf(
        b"GNU ed is a line-oriented text editor. It is used to create, display,\nmodify and otherwise manipulate text files, both interactively and via\nshell scripts. A restricted version of ed, red, can only edit files in\nthe current directory and cannot execute shell commands. Ed is the\n'standard' text editor in the sense that it is the original editor for\nUnix, and thus widely available. For most purposes, however, it is\nsuperseded by full-screen editors such as GNU Emacs or GNU Moe.\n\nUsage: %s [options] [file]\n\0"
            as *const u8 as *const libc::c_char,
        invocation_name,
    );
    printf(
        b"\nOptions:\n  -h, --help                 display this help and exit\n  -V, --version              output version information and exit\n  -E, --extended-regexp      use extended regular expressions\n  -G, --traditional          run in compatibility mode\n  -l, --loose-exit-status    exit with 0 status even if a command fails\n  -p, --prompt=STRING        use STRING as an interactive prompt\n  -q, --quiet, --silent      suppress diagnostics written to stderr\n  -r, --restricted           run in restricted mode\n  -s, --script               suppress byte counts and '!' prompt\n  -v, --verbose              be verbose; equivalent to the 'H' command\n      --strip-trailing-cr    strip carriage returns at end of text lines\n\nStart edit by reading in 'file' if given.\nIf 'file' begins with a '!', read output of shell command.\n\nExit status: 0 for a normal exit, 1 for environmental problems\n(file not found, invalid command line options, I/O errors, etc), 2 to\nindicate a corrupt or invalid input file, 3 for an internal consistency\nerror (e.g., bug) which caused ed to panic.\n\nReport bugs to bug-ed@gnu.org\nEd home page: http://www.gnu.org/software/ed/ed.html\nGeneral help using GNU software: http://www.gnu.org/gethelp\n\0"
            as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn show_version() {
    printf(
        b"GNU %s %s\n\0" as *const u8 as *const libc::c_char,
        program_name,
        b"1.19\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"Copyright (C) 1994 Andrew L. Moore.\nCopyright (C) %s Antonio Diaz Diaz.\n\0"
            as *const u8 as *const libc::c_char,
        program_year,
    );
    printf(
        b"License GPLv2+: GNU GPL version 2 or later <http://gnu.org/licenses/gpl.html>\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.\n\0"
            as *const u8 as *const libc::c_char,
    );
}
pub unsafe extern "C" fn show_strerror(
    filename: *const libc::c_char,
    errcode: libc::c_int,
) {
    if quiet_ as u64 == 0 {
        if !filename.is_null()
            && *filename.offset(0 as libc::c_int as isize) as libc::c_int != 0
        {
            fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, filename);
        }
        fprintf(
            stderr,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            strerror(errcode),
        );
    }
}
unsafe extern "C" fn show_error(
    msg: *const libc::c_char,
    errcode: libc::c_int,
    help: bool_0,
) {
    if !msg.is_null() && *msg.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        fprintf(
            stderr,
            b"%s: %s%s%s\n\0" as *const u8 as *const libc::c_char,
            program_name,
            msg,
            if errcode > 0 as libc::c_int {
                b": \0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            if errcode > 0 as libc::c_int {
                strerror(errcode) as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if help as u64 != 0 {
        fprintf(
            stderr,
            b"Try '%s --help' for more information.\n\0" as *const u8
                as *const libc::c_char,
            invocation_name,
        );
    }
}
pub unsafe extern "C" fn interactive() -> bool_0 {
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
    return (fstat(0 as libc::c_int, &mut st) == 0 as libc::c_int
        && !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint)) as libc::c_int as bool_0;
}
pub unsafe extern "C" fn may_access_filename(name: *const libc::c_char) -> bool_0 {
    if restricted_ as u64 != 0 {
        if *name.offset(0 as libc::c_int as isize) as libc::c_int == '!' as i32 {
            set_error_msg(
                b"Shell access restricted\0" as *const u8 as *const libc::c_char,
            );
            return false_0;
        }
        if strcmp(name, b"..\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            || !(strchr(name, '/' as i32)).is_null()
        {
            set_error_msg(
                b"Directory access restricted\0" as *const u8 as *const libc::c_char,
            );
            return false_0;
        }
    }
    return true_0;
}
unsafe fn main_0(
    argc: libc::c_int,
    mut argv: *const *const libc::c_char,
) -> libc::c_int {
    let mut argind: libc::c_int = 0;
    let mut initial_error: bool_0 = false_0;
    let mut loose: bool_0 = false_0;
    let options: [ap_Option; 13] = [
        {
            let mut init = ap_Option {
                code: 'E' as i32,
                long_name: b"extended-regexp\0" as *const u8 as *const libc::c_char,
                has_arg: ap_no,
            };
            init
        },
        {
            let mut init = ap_Option {
                code: 'G' as i32,
                long_name: b"traditional\0" as *const u8 as *const libc::c_char,
                has_arg: ap_no,
            };
            init
        },
        {
            let mut init = ap_Option {
                code: 'h' as i32,
                long_name: b"help\0" as *const u8 as *const libc::c_char,
                has_arg: ap_no,
            };
            init
        },
        {
            let mut init = ap_Option {
                code: 'l' as i32,
                long_name: b"loose-exit-status\0" as *const u8 as *const libc::c_char,
                has_arg: ap_no,
            };
            init
        },
        {
            let mut init = ap_Option {
                code: 'p' as i32,
                long_name: b"prompt\0" as *const u8 as *const libc::c_char,
                has_arg: ap_yes,
            };
            init
        },
        {
            let mut init = ap_Option {
                code: 'q' as i32,
                long_name: b"quiet\0" as *const u8 as *const libc::c_char,
                has_arg: ap_no,
            };
            init
        },
        {
            let mut init = ap_Option {
                code: 'q' as i32,
                long_name: b"silent\0" as *const u8 as *const libc::c_char,
                has_arg: ap_no,
            };
            init
        },
        {
            let mut init = ap_Option {
                code: 'r' as i32,
                long_name: b"restricted\0" as *const u8 as *const libc::c_char,
                has_arg: ap_no,
            };
            init
        },
        {
            let mut init = ap_Option {
                code: 's' as i32,
                long_name: b"script\0" as *const u8 as *const libc::c_char,
                has_arg: ap_no,
            };
            init
        },
        {
            let mut init = ap_Option {
                code: 'v' as i32,
                long_name: b"verbose\0" as *const u8 as *const libc::c_char,
                has_arg: ap_no,
            };
            init
        },
        {
            let mut init = ap_Option {
                code: 'V' as i32,
                long_name: b"version\0" as *const u8 as *const libc::c_char,
                has_arg: ap_no,
            };
            init
        },
        {
            let mut init = ap_Option {
                code: opt_cr as libc::c_int,
                long_name: b"strip-trailing-cr\0" as *const u8 as *const libc::c_char,
                has_arg: ap_no,
            };
            init
        },
        {
            let mut init = ap_Option {
                code: 0 as libc::c_int,
                long_name: 0 as *const libc::c_char,
                has_arg: ap_no,
            };
            init
        },
    ];
    let mut parser: Arg_parser = Arg_parser {
        data: 0 as *mut ap_Record,
        error: 0 as *mut libc::c_char,
        data_size: 0,
        error_size: 0,
    };
    if argc > 0 as libc::c_int {
        invocation_name = *argv.offset(0 as libc::c_int as isize);
    }
    if ap_init(
        &mut parser,
        argc,
        argv,
        options.as_ptr(),
        0 as libc::c_int as libc::c_char,
    ) == 0
    {
        show_error(
            b"Memory exhausted.\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            false_0,
        );
        return 1 as libc::c_int;
    }
    if !(ap_error(&mut parser)).is_null() {
        show_error(ap_error(&mut parser), 0 as libc::c_int, true_0);
        return 1 as libc::c_int;
    }
    argind = 0 as libc::c_int;
    while argind < ap_arguments(&mut parser) {
        let code: libc::c_int = ap_code(&mut parser, argind);
        let arg: *const libc::c_char = ap_argument(&mut parser, argind);
        if code == 0 {
            break;
        }
        match code {
            69 => {
                extended_regexp_ = true_0;
            }
            71 => {
                traditional_ = true_0;
            }
            104 => {
                show_help();
                return 0 as libc::c_int;
            }
            108 => {
                loose = true_0;
            }
            112 => {
                if !(set_prompt(arg) as u64 != 0) {
                    return 1 as libc::c_int;
                }
            }
            113 => {
                quiet_ = true_0;
            }
            114 => {
                restricted_ = true_0;
            }
            115 => {
                scripted_ = true_0;
            }
            118 => {
                set_verbose();
            }
            86 => {
                show_version();
                return 0 as libc::c_int;
            }
            256 => {
                strip_cr_ = true_0;
            }
            _ => {
                show_error(
                    b"internal error: uncaught option.\0" as *const u8
                        as *const libc::c_char,
                    0 as libc::c_int,
                    false_0,
                );
                return 3 as libc::c_int;
            }
        }
        argind += 1;
        argind;
    }
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    if init_buffers() as u64 == 0 {
        return 1 as libc::c_int;
    }
    while argind < ap_arguments(&mut parser) {
        let arg_0: *const libc::c_char = ap_argument(&mut parser, argind);
        if strcmp(arg_0, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            scripted_ = true_0;
            argind += 1;
            argind;
        } else {
            if may_access_filename(arg_0) as u64 != 0 {
                let ret: libc::c_int = read_file(arg_0, 0 as libc::c_int);
                if ret < 0 as libc::c_int && interactive() as u64 == 0 {
                    return 2 as libc::c_int;
                }
                if *arg_0.offset(0 as libc::c_int as isize) as libc::c_int != '!' as i32
                    && set_def_filename(arg_0) as u64 == 0
                {
                    return 1 as libc::c_int;
                }
                if ret == -(2 as libc::c_int) {
                    initial_error = true_0;
                }
            } else {
                if interactive() as u64 == 0 {
                    return 2 as libc::c_int;
                }
                initial_error = true_0;
            }
            break;
        }
    }
    ap_free(&mut parser);
    if initial_error as u64 != 0 {
        fputs(b"?\n\0" as *const u8 as *const libc::c_char, stdout);
    }
    return main_loop(initial_error, loose);
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
                args.as_mut_ptr() as *const *const libc::c_char,
            ) as i32,
        )
    }
}
