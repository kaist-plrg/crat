use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn rawmemchr(__s: *const libc::c_void, __c: libc::c_int) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    fn proper_name(name: *const libc::c_char) -> *const libc::c_char;
    fn proper_name_utf8(
        name_ascii: *const libc::c_char,
        name_utf8: *const libc::c_char,
    ) -> *const libc::c_char;
    static mut Version: *const libc::c_char;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn c_stack_action(
        _: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> libc::c_int;
    fn block_read(_: libc::c_int, _: *mut libc::c_char, _: size_t) -> size_t;
    fn buffer_lcm(_: size_t, _: size_t, _: size_t) -> size_t;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    static mut exit_failure: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn hard_locale(category: libc::c_int) -> bool;
    fn offtostr(_: off_t, _: *mut libc::c_char) -> *mut libc::c_char;
    static mut program_name: *const libc::c_char;
    fn set_program_name(argv0: *const libc::c_char);
    fn version_etc(
        stream: *mut FILE,
        command_name: *const libc::c_char,
        package: *const libc::c_char,
        version: *const libc::c_char,
        _: ...
    );
    fn emit_bug_reporting_address();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstdopen();
    fn xstrtoimax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut intmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
}
pub type __intmax_t = libc::c_long;
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
pub type intmax_t = __intmax_t;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
pub const type_first_diff: comparison_type = 0;
pub type comparison_type = libc::c_uint;
pub const type_status: comparison_type = 3;
pub const type_no_stdout: comparison_type = 2;
pub const type_all_diffs: comparison_type = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub _gl_dummy: libc::c_int,
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const HELP_OPTION: C2RustUnnamed_3 = 128;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub _gl_dummy: libc::c_int,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x20 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn __gl_setmode(
    mut fd: libc::c_int,
    mut mode: libc::c_int,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn set_binary_mode(
    mut fd: libc::c_int,
    mut mode: libc::c_int,
) -> libc::c_int {
    return __gl_setmode(fd, mode);
}
static mut PROGRAM_NAME: [libc::c_char; 4] = unsafe {
    *::std::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"cmp\0")
};
unsafe extern "C" fn hard_locale_LC_MESSAGES() -> bool {
    return hard_locale(5 as libc::c_int);
}
static mut file: [*const libc::c_char; 2] = [0 as *const libc::c_char; 2];
static mut file_desc: [libc::c_int; 2] = [0; 2];
static mut stat_buf: [stat; 2] = [stat {
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
}; 2];
static mut buffer: [*mut size_t; 2] = [0 as *const size_t as *mut size_t; 2];
static mut buf_size: size_t = 0;
static mut ignore_initial: [off_t; 2] = [0; 2];
static mut bytes: intmax_t = -(1 as libc::c_int) as intmax_t;
static mut comparison_type: comparison_type = type_first_diff;
static mut opt_print_bytes: bool = false;
static mut long_options: [option; 10] = [
    {
        let mut init = option {
            name: b"print-bytes\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"print-chars\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore-initial\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"verbose\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"bytes\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"silent\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"quiet\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: HELP_OPTION as libc::c_int,
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
unsafe extern "C" fn try_help(
    mut reason_msgid: *const libc::c_char,
    mut operand: *const libc::c_char,
) {
    if !reason_msgid.is_null() {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(0 as *const libc::c_char, reason_msgid, 5 as libc::c_int),
            operand,
        );
    }
    if ::std::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
        error(
            2 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Try '%s --help' for more information.\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
        if 0 as libc::c_int != 0 {} else {
            unreachable!();
        };
    } else {
        error(
            2 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Try '%s --help' for more information.\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
        if 0 as libc::c_int != 0 {} else {
            unreachable!();
        };
    };
}
static mut valid_suffixes: [libc::c_char; 11] = unsafe {
    *::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"kKMGTPEZY0\0")
};
unsafe extern "C" fn specify_ignore_initial(
    mut f: libc::c_int,
    mut argptr: *mut *mut libc::c_char,
    mut delimiter: libc::c_char,
) {
    let mut val: intmax_t = 0;
    let mut arg: *const libc::c_char = *argptr;
    let mut e: strtol_error = xstrtoimax(
        arg,
        argptr,
        0 as libc::c_int,
        &mut val,
        valid_suffixes.as_ptr(),
    );
    if !((e as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint
        || e as libc::c_uint
            == LONGINT_INVALID_SUFFIX_CHAR as libc::c_int as libc::c_uint
            && **argptr as libc::c_int == delimiter as libc::c_int)
        && 0 as libc::c_int as libc::c_long <= val
        && val
            <= (if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
                -(1 as libc::c_int) as off_t
            } else {
                (((1 as libc::c_int as off_t)
                    << (::std::mem::size_of::<off_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            }))
    {
        try_help(
            b"invalid --ignore-initial value '%s'\0" as *const u8 as *const libc::c_char,
            arg,
        );
    }
    if ignore_initial[f as usize] < val {
        ignore_initial[f as usize] = val;
    }
}
unsafe extern "C" fn specify_comparison_type(mut t: comparison_type) {
    if comparison_type as libc::c_uint != 0
        && comparison_type as libc::c_uint != t as libc::c_uint
    {
        try_help(
            b"options -l and -s are incompatible\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
        );
    }
    comparison_type = t;
}
unsafe extern "C" fn check_stdout() {
    if ferror_unlocked(stdout) != 0 {
        if ::std::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong != 0 {
            error(
                2 as libc::c_int,
                0 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"write failed\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                2 as libc::c_int,
                0 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"write failed\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    } else if fclose(stdout) != 0 as libc::c_int {
        if ::std::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
            error(
                2 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"standard output\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                2 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"standard output\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
}
static mut option_help_msgid: [*const libc::c_char; 9] = [
    b"-b, --print-bytes          print differing bytes\0" as *const u8
        as *const libc::c_char,
    b"-i, --ignore-initial=SKIP         skip first SKIP bytes of both inputs\0"
        as *const u8 as *const libc::c_char,
    b"-i, --ignore-initial=SKIP1:SKIP2  skip first SKIP1 bytes of FILE1 and\n                                      first SKIP2 bytes of FILE2\0"
        as *const u8 as *const libc::c_char,
    b"-l, --verbose              output byte numbers and differing byte values\0"
        as *const u8 as *const libc::c_char,
    b"-n, --bytes=LIMIT          compare at most LIMIT bytes\0" as *const u8
        as *const libc::c_char,
    b"-s, --quiet, --silent      suppress all normal output\0" as *const u8
        as *const libc::c_char,
    b"    --help                 display this help and exit\0" as *const u8
        as *const libc::c_char,
    b"-v, --version              output version information and exit\0" as *const u8
        as *const libc::c_char,
    0 as *const libc::c_char,
];
unsafe extern "C" fn usage() {
    let mut p: *const *const libc::c_char = 0 as *const *const libc::c_char;
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"Usage: %s [OPTION]... FILE1 [FILE2 [SKIP1 [SKIP2]]]\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        program_name,
    );
    printf(
        b"%s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"Compare two files byte by byte.\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    printf(
        b"\n%s\n\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"The optional SKIP1 and SKIP2 specify the number of bytes to skip\nat the beginning of each file (zero by default).\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fputs_unlocked(
        dcgettext(
            0 as *const libc::c_char,
            b"Mandatory arguments to long options are mandatory for short options too.\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    p = option_help_msgid.as_ptr();
    while !(*p).is_null() {
        printf(
            b"  %s\n\0" as *const u8 as *const libc::c_char,
            dcgettext(0 as *const libc::c_char, *p, 5 as libc::c_int),
        );
        p = p.offset(1);
        p;
    }
    printf(
        b"\n%s\n\n%s\n%s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"SKIP values may be followed by the following multiplicative suffixes:\nkB 1000, K 1024, MB 1,000,000, M 1,048,576,\nGB 1,000,000,000, G 1,073,741,824, and so on for T, P, E, Z, Y.\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        dcgettext(
            0 as *const libc::c_char,
            b"If a FILE is '-' or missing, read standard input.\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        dcgettext(
            0 as *const libc::c_char,
            b"Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    emit_bug_reporting_address();
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut exit_status: libc::c_int = 0;
    let mut words_per_buffer: size_t = 0;
    ::std::ptr::write_volatile(&mut exit_failure as *mut libc::c_int, 2 as libc::c_int);
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"diffutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"diffutils\0" as *const u8 as *const libc::c_char);
    c_stack_action(None);
    xstdopen();
    loop {
        c = getopt_long(
            argc,
            argv,
            b"bci:ln:sv\0" as *const u8 as *const libc::c_char,
            long_options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            98 | 99 => {
                opt_print_bytes = 1 as libc::c_int != 0;
            }
            105 => {
                specify_ignore_initial(
                    0 as libc::c_int,
                    &mut optarg,
                    ':' as i32 as libc::c_char,
                );
                let fresh0 = optarg;
                optarg = optarg.offset(1);
                if *fresh0 as libc::c_int == ':' as i32 {
                    specify_ignore_initial(
                        1 as libc::c_int,
                        &mut optarg,
                        0 as libc::c_int as libc::c_char,
                    );
                } else if ignore_initial[1 as libc::c_int as usize]
                    < ignore_initial[0 as libc::c_int as usize]
                {
                    ignore_initial[1 as libc::c_int
                        as usize] = ignore_initial[0 as libc::c_int as usize];
                }
            }
            108 => {
                specify_comparison_type(type_all_diffs);
            }
            110 => {
                let mut n: intmax_t = 0;
                if xstrtoimax(
                    optarg,
                    0 as *mut *mut libc::c_char,
                    0 as libc::c_int,
                    &mut n,
                    valid_suffixes.as_ptr(),
                ) as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint
                    || n < 0 as libc::c_int as libc::c_long
                {
                    try_help(
                        b"invalid --bytes value '%s'\0" as *const u8
                            as *const libc::c_char,
                        optarg,
                    );
                }
                if !(0 as libc::c_int as libc::c_long <= bytes && bytes < n) {
                    bytes = n;
                }
            }
            115 => {
                specify_comparison_type(type_status);
            }
            118 => {
                version_etc(
                    stdout,
                    PROGRAM_NAME.as_ptr(),
                    b"GNU diffutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    proper_name_utf8(
                        b"Torbjorn Granlund\0" as *const u8 as *const libc::c_char,
                        b"Torbj\xC3\xB6rn Granlund\0" as *const u8 as *const libc::c_char,
                    ),
                    proper_name(
                        b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                    ),
                    0 as *mut libc::c_void,
                );
                check_stdout();
                return 0 as libc::c_int;
            }
            128 => {
                usage();
                check_stdout();
                return 0 as libc::c_int;
            }
            _ => {
                try_help(0 as *const libc::c_char, 0 as *const libc::c_char);
            }
        }
    }
    if optind == argc {
        try_help(
            b"missing operand after '%s'\0" as *const u8 as *const libc::c_char,
            *argv.offset((argc - 1 as libc::c_int) as isize),
        );
    }
    let fresh1 = optind;
    optind = optind + 1;
    file[0 as libc::c_int as usize] = *argv.offset(fresh1 as isize);
    file[1 as libc::c_int
        as usize] = if optind < argc {
        let fresh2 = optind;
        optind = optind + 1;
        *argv.offset(fresh2 as isize) as *const libc::c_char
    } else {
        b"-\0" as *const u8 as *const libc::c_char
    };
    let mut f: libc::c_int = 0 as libc::c_int;
    while f < 2 as libc::c_int && optind < argc {
        let fresh3 = optind;
        optind = optind + 1;
        let mut arg: *mut libc::c_char = *argv.offset(fresh3 as isize);
        specify_ignore_initial(f, &mut arg, 0 as libc::c_int as libc::c_char);
        f += 1;
        f;
    }
    if optind < argc {
        try_help(
            b"extra operand '%s'\0" as *const u8 as *const libc::c_char,
            *argv.offset(optind as isize),
        );
    }
    let mut f_0: libc::c_int = 0 as libc::c_int;
    while f_0 < 2 as libc::c_int {
        if f_0 != 0
            && ignore_initial[0 as libc::c_int as usize]
                == ignore_initial[1 as libc::c_int as usize]
            && strcmp(file[0 as libc::c_int as usize], file[1 as libc::c_int as usize])
                == 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        if strcmp(file[f_0 as usize], b"-\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            file_desc[f_0 as usize] = 0 as libc::c_int;
            if 0 as libc::c_int != 0 && isatty(0 as libc::c_int) == 0 {
                set_binary_mode(0 as libc::c_int, 0 as libc::c_int);
            }
        } else {
            file_desc[f_0
                as usize] = open(
                file[f_0 as usize],
                0 as libc::c_int | 0 as libc::c_int,
                0 as libc::c_int,
            );
            if file_desc[f_0 as usize] < 0 as libc::c_int {
                if comparison_type as libc::c_uint
                    != type_status as libc::c_int as libc::c_uint
                {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        file[f_0 as usize],
                    );
                }
                exit(2 as libc::c_int);
            }
        }
        if fstat(file_desc[f_0 as usize], stat_buf.as_mut_ptr().offset(f_0 as isize))
            < 0 as libc::c_int
        {
            stat_buf[f_0 as usize].st_size = -(1 as libc::c_int) as __off_t;
            stat_buf[f_0 as usize]
                .st_blksize = (8 as libc::c_int * 1024 as libc::c_int) as __blksize_t;
        }
        f_0 += 1;
        f_0;
    }
    if 0 as libc::c_int as libc::c_long <= stat_buf[0 as libc::c_int as usize].st_size
        && 0 as libc::c_int as libc::c_long
            <= stat_buf[1 as libc::c_int as usize].st_size
        && (0 as libc::c_int)
            < (stat_buf[0 as libc::c_int as usize].st_ino
                == stat_buf[1 as libc::c_int as usize].st_ino
                && stat_buf[0 as libc::c_int as usize].st_dev
                    == stat_buf[1 as libc::c_int as usize].st_dev
                || (stat_buf[0 as libc::c_int as usize].st_mode
                    & 0o170000 as libc::c_int as libc::c_uint
                    == 0o60000 as libc::c_int as libc::c_uint
                    && stat_buf[1 as libc::c_int as usize].st_mode
                        & 0o170000 as libc::c_int as libc::c_uint
                        == 0o60000 as libc::c_int as libc::c_uint
                    || stat_buf[0 as libc::c_int as usize].st_mode
                        & 0o170000 as libc::c_int as libc::c_uint
                        == 0o20000 as libc::c_int as libc::c_uint
                        && stat_buf[1 as libc::c_int as usize].st_mode
                            & 0o170000 as libc::c_int as libc::c_uint
                            == 0o20000 as libc::c_int as libc::c_uint)
                    && stat_buf[0 as libc::c_int as usize].st_rdev
                        == stat_buf[1 as libc::c_int as usize].st_rdev) as libc::c_int
        && (stat_buf[0 as libc::c_int as usize].st_mode
            == stat_buf[1 as libc::c_int as usize].st_mode
            && stat_buf[0 as libc::c_int as usize].st_nlink
                == stat_buf[1 as libc::c_int as usize].st_nlink
            && stat_buf[0 as libc::c_int as usize].st_uid
                == stat_buf[1 as libc::c_int as usize].st_uid
            && stat_buf[0 as libc::c_int as usize].st_gid
                == stat_buf[1 as libc::c_int as usize].st_gid
            && stat_buf[0 as libc::c_int as usize].st_size
                == stat_buf[1 as libc::c_int as usize].st_size
            && stat_buf[0 as libc::c_int as usize].st_mtim.tv_sec
                == stat_buf[1 as libc::c_int as usize].st_mtim.tv_sec
            && stat_buf[0 as libc::c_int as usize].st_ctim.tv_sec
                == stat_buf[1 as libc::c_int as usize].st_ctim.tv_sec)
        && file_position(0 as libc::c_int) == file_position(1 as libc::c_int)
    {
        return 0 as libc::c_int;
    }
    if comparison_type as libc::c_uint != type_status as libc::c_int as libc::c_uint {
        let mut outstat: stat = stat {
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
        let mut nullstat: stat = stat {
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
        if fstat(1 as libc::c_int, &mut outstat) == 0 as libc::c_int
            && stat(b"/dev/null\0" as *const u8 as *const libc::c_char, &mut nullstat)
                == 0 as libc::c_int
            && (0 as libc::c_int)
                < (outstat.st_ino == nullstat.st_ino && outstat.st_dev == nullstat.st_dev
                    || (outstat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o60000 as libc::c_int as libc::c_uint
                        && nullstat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                            == 0o60000 as libc::c_int as libc::c_uint
                        || outstat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                            == 0o20000 as libc::c_int as libc::c_uint
                            && nullstat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                                == 0o20000 as libc::c_int as libc::c_uint)
                        && outstat.st_rdev == nullstat.st_rdev) as libc::c_int
        {
            comparison_type = type_no_stdout;
        }
    }
    if comparison_type as libc::c_uint == type_status as libc::c_int as libc::c_uint
        && 0 as libc::c_int as libc::c_long
            <= stat_buf[0 as libc::c_int as usize].st_size
        && stat_buf[0 as libc::c_int as usize].st_mode
            & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint
        && 0 as libc::c_int as libc::c_long
            <= stat_buf[1 as libc::c_int as usize].st_size
        && stat_buf[1 as libc::c_int as usize].st_mode
            & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint
    {
        let mut s0: off_t = stat_buf[0 as libc::c_int as usize].st_size
            - file_position(0 as libc::c_int);
        let mut s1: off_t = stat_buf[1 as libc::c_int as usize].st_size
            - file_position(1 as libc::c_int);
        if s0 < 0 as libc::c_int as libc::c_long {
            s0 = 0 as libc::c_int as off_t;
        }
        if s1 < 0 as libc::c_int as libc::c_long {
            s1 = 0 as libc::c_int as off_t;
        }
        if s0 != s1
            && (bytes < 0 as libc::c_int as libc::c_long
                || (if s0 <= s1 { s0 } else { s1 }) < bytes)
        {
            exit(1 as libc::c_int);
        }
    }
    buf_size = buffer_lcm(
        stat_buf[0 as libc::c_int as usize].st_blksize as size_t,
        stat_buf[1 as libc::c_int as usize].st_blksize as size_t,
        (9223372036854775807 as libc::c_long as libc::c_ulong)
            .wrapping_sub(::std::mem::size_of::<size_t>() as libc::c_ulong),
    );
    words_per_buffer = buf_size
        .wrapping_add(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong),
        )
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<size_t>() as libc::c_ulong);
    buffer[0 as libc::c_int
        as usize] = xmalloc(
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_mul(words_per_buffer),
    ) as *mut size_t;
    buffer[1 as libc::c_int
        as usize] = (buffer[0 as libc::c_int as usize])
        .offset(words_per_buffer as isize);
    exit_status = cmp();
    let mut f_1: libc::c_int = 0 as libc::c_int;
    while f_1 < 2 as libc::c_int {
        if close(file_desc[f_1 as usize]) != 0 as libc::c_int {
            if ::std::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong != 0 {
                error(
                    2 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    file[f_1 as usize],
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    2 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    file[f_1 as usize],
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        f_1 += 1;
        f_1;
    }
    if exit_status != 0 as libc::c_int
        && (comparison_type as libc::c_uint)
            < type_no_stdout as libc::c_int as libc::c_uint
    {
        check_stdout();
    }
    exit(exit_status);
}
unsafe extern "C" fn cmp() -> libc::c_int {
    let mut at_line_start: bool = 1 as libc::c_int != 0;
    let mut line_number: off_t = 1 as libc::c_int as off_t;
    let mut byte_number: off_t = 1 as libc::c_int as off_t;
    let mut remaining: intmax_t = bytes;
    let mut read0: size_t = 0;
    let mut read1: size_t = 0;
    let mut first_diff: size_t = 0;
    let mut smaller: size_t = 0;
    let mut buffer0: *mut size_t = buffer[0 as libc::c_int as usize];
    let mut buffer1: *mut size_t = buffer[1 as libc::c_int as usize];
    let mut buf0: *mut libc::c_char = buffer0 as *mut libc::c_char;
    let mut buf1: *mut libc::c_char = buffer1 as *mut libc::c_char;
    let mut differing: libc::c_int = 0 as libc::c_int;
    let mut f: libc::c_int = 0;
    let mut offset_width: libc::c_int = 0;
    if comparison_type as libc::c_uint == type_all_diffs as libc::c_int as libc::c_uint {
        let mut byte_number_max: off_t = if 0 as libc::c_int as libc::c_long <= bytes
            && bytes
                <= (if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
                    -(1 as libc::c_int) as off_t
                } else {
                    (((1 as libc::c_int as off_t)
                        << (::std::mem::size_of::<off_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long
                })
        {
            bytes
        } else if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
            -(1 as libc::c_int) as off_t
        } else {
            (((1 as libc::c_int as off_t)
                << (::std::mem::size_of::<off_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        };
        f = 0 as libc::c_int;
        while f < 2 as libc::c_int {
            if 0 as libc::c_int as libc::c_long <= stat_buf[f as usize].st_size
                && stat_buf[f as usize].st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o100000 as libc::c_int as libc::c_uint
            {
                let mut file_bytes: off_t = stat_buf[f as usize].st_size
                    - file_position(f);
                if file_bytes < byte_number_max {
                    byte_number_max = file_bytes;
                }
            }
            f += 1;
            f;
        }
        offset_width = 1 as libc::c_int;
        loop {
            byte_number_max /= 10 as libc::c_int as libc::c_long;
            if !(byte_number_max != 0 as libc::c_int as libc::c_long) {
                break;
            }
            offset_width += 1;
            offset_width;
        }
    }
    f = 0 as libc::c_int;
    while f < 2 as libc::c_int {
        let mut ig: off_t = ignore_initial[f as usize];
        if ig != 0 && file_position(f) == -(1 as libc::c_int) as libc::c_long {
            loop {
                let mut bytes_to_read: size_t = if ig as libc::c_ulong <= buf_size {
                    ig as libc::c_ulong
                } else {
                    buf_size
                };
                let mut r: size_t = block_read(
                    file_desc[f as usize],
                    buf0,
                    bytes_to_read,
                );
                if r != bytes_to_read {
                    if r == 18446744073709551615 as libc::c_ulong {
                        if ::std::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0
                        {
                            error(
                                2 as libc::c_int,
                                *__errno_location(),
                                b"%s\0" as *const u8 as *const libc::c_char,
                                file[f as usize],
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        } else {
                            error(
                                2 as libc::c_int,
                                *__errno_location(),
                                b"%s\0" as *const u8 as *const libc::c_char,
                                file[f as usize],
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        };
                    }
                    break;
                } else {
                    ig = (ig as libc::c_ulong).wrapping_sub(r) as off_t as off_t;
                    if !(ig != 0) {
                        break;
                    }
                }
            }
        }
        f += 1;
        f;
    }
    loop {
        let mut bytes_to_read_0: size_t = buf_size;
        if 0 as libc::c_int as libc::c_long <= remaining {
            if (remaining as libc::c_ulong) < bytes_to_read_0 {
                bytes_to_read_0 = remaining as size_t;
            }
            remaining = (remaining as libc::c_ulong).wrapping_sub(bytes_to_read_0)
                as intmax_t as intmax_t;
        }
        read0 = block_read(file_desc[0 as libc::c_int as usize], buf0, bytes_to_read_0);
        if read0 == 18446744073709551615 as libc::c_ulong {
            if ::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong != 0 {
                error(
                    2 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    file[0 as libc::c_int as usize],
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    2 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    file[0 as libc::c_int as usize],
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        read1 = block_read(file_desc[1 as libc::c_int as usize], buf1, bytes_to_read_0);
        if read1 == 18446744073709551615 as libc::c_ulong {
            if ::std::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong != 0 {
                error(
                    2 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    file[1 as libc::c_int as usize],
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    2 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    file[1 as libc::c_int as usize],
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        smaller = if read0 <= read1 { read0 } else { read1 };
        if memcmp(buf0 as *const libc::c_void, buf1 as *const libc::c_void, smaller)
            == 0 as libc::c_int
        {
            first_diff = smaller;
        } else {
            if read0 >= read1 {
                *buf1.offset(read0 as isize) = 0x55 as libc::c_int as libc::c_char;
            }
            if read1 >= read0 {
                *buf0.offset(read1 as isize) = 0x79 as libc::c_int as libc::c_char;
            }
            *buf0
                .offset(
                    read0 as isize,
                ) = !(*buf1.offset(read0 as isize) as libc::c_int) as libc::c_char;
            *buf1
                .offset(
                    read1 as isize,
                ) = !(*buf0.offset(read1 as isize) as libc::c_int) as libc::c_char;
            memset(
                buf0.offset(read0 as isize).offset(1 as libc::c_int as isize)
                    as *mut libc::c_void,
                0 as libc::c_int,
                (::std::mem::size_of::<size_t>() as libc::c_ulong)
                    .wrapping_sub(
                        read0
                            .wrapping_rem(
                                ::std::mem::size_of::<size_t>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            memset(
                buf1.offset(read1 as isize).offset(1 as libc::c_int as isize)
                    as *mut libc::c_void,
                0 as libc::c_int,
                (::std::mem::size_of::<size_t>() as libc::c_ulong)
                    .wrapping_sub(
                        read1
                            .wrapping_rem(
                                ::std::mem::size_of::<size_t>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            first_diff = block_compare(buffer0, buffer1);
        }
        byte_number = (byte_number as libc::c_ulong).wrapping_add(first_diff) as off_t
            as off_t;
        if comparison_type as libc::c_uint
            == type_first_diff as libc::c_int as libc::c_uint
            && first_diff != 0 as libc::c_int as libc::c_ulong
        {
            line_number = (line_number as libc::c_ulong)
                .wrapping_add(count_newlines(buf0, first_diff)) as off_t as off_t;
            at_line_start = *buf0
                .offset(
                    first_diff.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) as libc::c_int == '\n' as i32;
        }
        if first_diff < smaller {
            let mut current_block_72: u64;
            match comparison_type as libc::c_uint {
                0 => {
                    let mut byte_buf: [libc::c_char; 21] = [0; 21];
                    let mut line_buf: [libc::c_char; 21] = [0; 21];
                    let mut byte_num: *const libc::c_char = offtostr(
                        byte_number,
                        byte_buf.as_mut_ptr(),
                    );
                    let mut line_num: *const libc::c_char = offtostr(
                        line_number,
                        line_buf.as_mut_ptr(),
                    );
                    if !opt_print_bytes {
                        static mut char_message: [libc::c_char; 32] = unsafe {
                            *::std::mem::transmute::<
                                &[u8; 32],
                                &[libc::c_char; 32],
                            >(b"%s %s differ: char %s, line %s\n\0")
                        };
                        static mut byte_msgid: [libc::c_char; 32] = unsafe {
                            *::std::mem::transmute::<
                                &[u8; 32],
                                &[libc::c_char; 32],
                            >(b"%s %s differ: byte %s, line %s\n\0")
                        };
                        let mut byte_message: *const libc::c_char = dcgettext(
                            0 as *const libc::c_char,
                            byte_msgid.as_ptr(),
                            5 as libc::c_int,
                        );
                        let mut use_byte_message: bool = byte_message
                            != byte_msgid.as_ptr()
                            || hard_locale_LC_MESSAGES() as libc::c_int != 0;
                        printf(
                            if use_byte_message as libc::c_int != 0 {
                                byte_message
                            } else {
                                char_message.as_ptr()
                            },
                            file[0 as libc::c_int as usize],
                            file[1 as libc::c_int as usize],
                            byte_num,
                            line_num,
                        );
                    } else {
                        let mut c0: libc::c_uchar = *buf0.offset(first_diff as isize)
                            as libc::c_uchar;
                        let mut c1: libc::c_uchar = *buf1.offset(first_diff as isize)
                            as libc::c_uchar;
                        let mut s0: [libc::c_char; 5] = [0; 5];
                        let mut s1: [libc::c_char; 5] = [0; 5];
                        sprintc(s0.as_mut_ptr(), c0);
                        sprintc(s1.as_mut_ptr(), c1);
                        printf(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s %s differ: byte %s, line %s is %3o %s %3o %s\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            file[0 as libc::c_int as usize],
                            file[1 as libc::c_int as usize],
                            byte_num,
                            line_num,
                            c0 as libc::c_int,
                            s0.as_mut_ptr(),
                            c1 as libc::c_int,
                            s1.as_mut_ptr(),
                        );
                    }
                    current_block_72 = 10348643357505364415;
                }
                3 => {
                    current_block_72 = 10348643357505364415;
                }
                1 => {
                    loop {
                        let mut c0_0: libc::c_uchar = *buf0.offset(first_diff as isize)
                            as libc::c_uchar;
                        let mut c1_0: libc::c_uchar = *buf1.offset(first_diff as isize)
                            as libc::c_uchar;
                        if c0_0 as libc::c_int != c1_0 as libc::c_int {
                            let mut byte_buf_0: [libc::c_char; 21] = [0; 21];
                            let mut byte_num_0: *const libc::c_char = offtostr(
                                byte_number,
                                byte_buf_0.as_mut_ptr(),
                            );
                            if !opt_print_bytes {
                                printf(
                                    b"%*s %3o %3o\n\0" as *const u8 as *const libc::c_char,
                                    offset_width,
                                    byte_num_0,
                                    c0_0 as libc::c_int,
                                    c1_0 as libc::c_int,
                                );
                            } else {
                                let mut s0_0: [libc::c_char; 5] = [0; 5];
                                let mut s1_0: [libc::c_char; 5] = [0; 5];
                                sprintc(s0_0.as_mut_ptr(), c0_0);
                                sprintc(s1_0.as_mut_ptr(), c1_0);
                                printf(
                                    b"%*s %3o %-4s %3o %s\n\0" as *const u8
                                        as *const libc::c_char,
                                    offset_width,
                                    byte_num_0,
                                    c0_0 as libc::c_int,
                                    s0_0.as_mut_ptr(),
                                    c1_0 as libc::c_int,
                                    s1_0.as_mut_ptr(),
                                );
                            }
                        }
                        byte_number += 1;
                        byte_number;
                        first_diff = first_diff.wrapping_add(1);
                        first_diff;
                        if !(first_diff < smaller) {
                            break;
                        }
                    }
                    differing = -(1 as libc::c_int);
                    current_block_72 = 851619935621435220;
                }
                2 => {
                    differing = 1 as libc::c_int;
                    current_block_72 = 851619935621435220;
                }
                _ => {
                    current_block_72 = 851619935621435220;
                }
            }
            match current_block_72 {
                851619935621435220 => {}
                _ => return 1 as libc::c_int,
            }
        }
        if read0 != read1 {
            if differing <= 0 as libc::c_int
                && comparison_type as libc::c_uint
                    != type_status as libc::c_int as libc::c_uint
            {
                let mut shorter_file: *const libc::c_char = file[(read1 < read0)
                    as libc::c_int as usize];
                if byte_number == 1 as libc::c_int as libc::c_long {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"cmp: EOF on %s which is empty\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        shorter_file,
                    );
                } else {
                    let mut byte_buf_1: [libc::c_char; 21] = [0; 21];
                    let mut byte_num_1: *const libc::c_char = offtostr(
                        byte_number - 1 as libc::c_int as libc::c_long,
                        byte_buf_1.as_mut_ptr(),
                    );
                    if comparison_type as libc::c_uint
                        == type_first_diff as libc::c_int as libc::c_uint
                    {
                        let mut line_buf_0: [libc::c_char; 21] = [0; 21];
                        let mut line_num_0: *const libc::c_char = offtostr(
                            line_number - at_line_start as libc::c_long,
                            line_buf_0.as_mut_ptr(),
                        );
                        fprintf(
                            stderr,
                            if at_line_start as libc::c_int != 0 {
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"cmp: EOF on %s after byte %s, line %s\n\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                )
                            } else {
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"cmp: EOF on %s after byte %s, in line %s\n\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                )
                            },
                            shorter_file,
                            byte_num_1,
                            line_num_0,
                        );
                    } else {
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"cmp: EOF on %s after byte %s\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            shorter_file,
                            byte_num_1,
                        );
                    }
                }
            }
            return 1 as libc::c_int;
        }
        if !(differing <= 0 as libc::c_int && read0 == buf_size) {
            break;
        }
    }
    return if differing == 0 as libc::c_int {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
}
unsafe extern "C" fn block_compare(
    mut p0: *const size_t,
    mut p1: *const size_t,
) -> size_t {
    let mut l0: *const size_t = 0 as *const size_t;
    let mut l1: *const size_t = 0 as *const size_t;
    let mut c0: *const libc::c_char = 0 as *const libc::c_char;
    let mut c1: *const libc::c_char = 0 as *const libc::c_char;
    l0 = p0;
    l1 = p1;
    while *l0 == *l1 {
        l0 = l0.offset(1);
        l0;
        l1 = l1.offset(1);
        l1;
    }
    c0 = l0 as *const libc::c_char;
    c1 = l1 as *const libc::c_char;
    while *c0 as libc::c_int == *c1 as libc::c_int {
        c0 = c0.offset(1);
        c0;
        c1 = c1.offset(1);
        c1;
    }
    return c0.offset_from(p0 as *const libc::c_char) as libc::c_long as size_t;
}
unsafe extern "C" fn count_newlines(
    mut buf: *mut libc::c_char,
    mut bufsize: size_t,
) -> size_t {
    let mut count: size_t = 0 as libc::c_int as size_t;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lim: *mut libc::c_char = buf.offset(bufsize as isize);
    let mut ch: libc::c_char = *lim;
    *lim = '\n' as i32 as libc::c_char;
    p = buf;
    loop {
        p = rawmemchr(p as *const libc::c_void, '\n' as i32) as *mut libc::c_char;
        if !(p != lim) {
            break;
        }
        count = count.wrapping_add(1);
        count;
        p = p.offset(1);
        p;
    }
    *lim = ch;
    return count;
}
unsafe extern "C" fn sprintc(mut buf: *mut libc::c_char, mut c: libc::c_uchar) {
    if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
        & _ISprint as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        if c as libc::c_int >= 128 as libc::c_int {
            let fresh4 = buf;
            buf = buf.offset(1);
            *fresh4 = 'M' as i32 as libc::c_char;
            let fresh5 = buf;
            buf = buf.offset(1);
            *fresh5 = '-' as i32 as libc::c_char;
            c = (c as libc::c_int - 128 as libc::c_int) as libc::c_uchar;
        }
        if (c as libc::c_int) < 32 as libc::c_int {
            let fresh6 = buf;
            buf = buf.offset(1);
            *fresh6 = '^' as i32 as libc::c_char;
            c = (c as libc::c_int + 64 as libc::c_int) as libc::c_uchar;
        } else if c as libc::c_int == 127 as libc::c_int {
            let fresh7 = buf;
            buf = buf.offset(1);
            *fresh7 = '^' as i32 as libc::c_char;
            c = '?' as i32 as libc::c_uchar;
        }
    }
    let fresh8 = buf;
    buf = buf.offset(1);
    *fresh8 = c as libc::c_char;
    *buf = 0 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn file_position(mut f: libc::c_int) -> off_t {
    static mut positioned: [bool; 2] = [false; 2];
    static mut position: [off_t; 2] = [0; 2];
    if !positioned[f as usize] {
        positioned[f as usize] = 1 as libc::c_int != 0;
        position[f
            as usize] = lseek(
            file_desc[f as usize],
            ignore_initial[f as usize],
            1 as libc::c_int,
        );
    }
    return position[f as usize];
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
