use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn mbsrtowcs(
        __dst: *mut wchar_t,
        __src: *mut *const libc::c_char,
        __len: size_t,
        __ps: *mut mbstate_t,
    ) -> size_t;
    fn wprintf(__format: *const wchar_t, _: ...) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
    fn close_stdout();
    fn __errno_location() -> *mut libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    static mut program_name: *const libc::c_char;
    fn set_program_name(argv0: *const libc::c_char);
    fn proper_name(name: *const libc::c_char) -> *const libc::c_char;
    fn version_etc(
        stream: *mut FILE,
        command_name: *const libc::c_char,
        package: *const libc::c_char,
        version: *const libc::c_char,
        _: ...
    );
    fn emit_bug_reporting_address();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
}
pub type __mbstate_t = mbstate_t;
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
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub type FILE = _IO_FILE;
pub const OPT_HELP: C2RustUnnamed_0 = 128;
pub const OPT_VERSION: C2RustUnnamed_0 = 129;
pub type C2RustUnnamed_0 = libc::c_uint;
unsafe extern "C" fn print_help(mut out: *mut FILE) {
    let mut lc_messages: *const libc::c_char = setlocale(
        5 as libc::c_int,
        0 as *const libc::c_char,
    );
    fprintf(
        out,
        dcgettext(
            0 as *const libc::c_char,
            b"Usage: %s [OPTION]...\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        program_name,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"Print a friendly, customizable greeting.\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, out);
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"  -t, --traditional       use traditional greeting\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"  -g, --greeting=TEXT     use TEXT as the greeting message\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, out);
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"      --help     display this help and exit\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"      --version  output version information and exit\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    emit_bug_reporting_address();
    if !lc_messages.is_null()
        && strncmp(
            lc_messages,
            b"en_\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) != 0
    {
        fprintf(
            out,
            dcgettext(
                0 as *const libc::c_char,
                b"Report %s translation bugs to <https://translationproject.org/team/>\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"GNU Hello\0" as *const u8 as *const libc::c_char,
        );
    }
    exit(if out == stderr { 1 as libc::c_int } else { 0 as libc::c_int });
}
unsafe extern "C" fn parse_options(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut greeting_msg: *mut *const libc::c_char,
) {
    let mut optc: libc::c_int = 0;
    let mut lose: libc::c_int = 0 as libc::c_int;
    static mut longopts: [option; 5] = [
        {
            let mut init = option {
                name: b"greeting\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'g' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"traditional\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 't' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"help\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPT_HELP as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"version\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPT_VERSION as libc::c_int,
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
    loop {
        optc = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            b"g:t\0" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(optc != -(1 as libc::c_int)) {
            break;
        }
        let mut current_block_5: u64;
        match optc {
            129 => {
                version_etc(
                    stdout,
                    b"hello\0" as *const u8 as *const libc::c_char,
                    b"GNU Hello\0" as *const u8 as *const libc::c_char,
                    b"2.12.1\0" as *const u8 as *const libc::c_char,
                    proper_name(b"Karl Berry\0" as *const u8 as *const libc::c_char),
                    proper_name(b"Sami Kerola\0" as *const u8 as *const libc::c_char),
                    proper_name(b"Jim Meyering\0" as *const u8 as *const libc::c_char),
                    proper_name(b"Reuben Thomas\0" as *const u8 as *const libc::c_char),
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            103 => {
                *greeting_msg = optarg;
                current_block_5 = 1394248824506584008;
            }
            128 => {
                print_help(stdout);
                current_block_5 = 10835925411765060935;
            }
            116 => {
                current_block_5 = 10835925411765060935;
            }
            _ => {
                lose = 1 as libc::c_int;
                current_block_5 = 1394248824506584008;
            }
        }
        match current_block_5 {
            10835925411765060935 => {
                *greeting_msg = dcgettext(
                    0 as *const libc::c_char,
                    b"hello, world\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                );
            }
            _ => {}
        }
    }
    if lose != 0 || optind < argc {
        if !(*argv.offset(optind as isize)).is_null() {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"extra operand\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                *argv.offset(optind as isize),
            );
        }
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Try '%s --help' for more information.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
        exit(1 as libc::c_int);
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut greeting_msg: *const libc::c_char = 0 as *const libc::c_char;
    let mut mb_greeting: *mut wchar_t = 0 as *mut wchar_t;
    let mut mbstate: mbstate_t = {
        let mut init = __mbstate_t {
            __count: 0 as libc::c_int,
            __value: C2RustUnnamed { __wch: 0 },
        };
        init
    };
    let mut len: size_t = 0;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"hello\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"hello\0" as *const u8 as *const libc::c_char);
    greeting_msg = dcgettext(
        0 as *const libc::c_char,
        b"Hello, world!\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int,
    );
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    parse_options(argc, argv, &mut greeting_msg);
    len = (strlen(greeting_msg)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    mb_greeting = xmalloc(
        len.wrapping_mul(::std::mem::size_of::<wchar_t>() as libc::c_ulong),
    ) as *mut wchar_t;
    len = mbsrtowcs(mb_greeting, &mut greeting_msg, len, &mut mbstate);
    if len == -(1 as libc::c_int) as size_t {
        error(
            1 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"conversion to a multibyte string failed\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    wprintf(
        (*::std::mem::transmute::<
            &[u8; 20],
            &[libc::c_int; 5],
        >(b"%\0\0\0l\0\0\0s\0\0\0\n\0\0\0\0\0\0\0"))
            .as_ptr(),
        mb_greeting,
    );
    free(mb_greeting as *mut libc::c_void);
    exit(0 as libc::c_int);
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
