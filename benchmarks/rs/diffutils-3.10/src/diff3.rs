use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn fork() -> __pid_t;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn exit(_: libc::c_int) -> !;
    fn rpl_free(ptr: *mut libc::c_void);
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
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
    fn __errno_location() -> *mut libc::c_int;
    fn proper_name(name: *const libc::c_char) -> *const libc::c_char;
    static mut Version: *const libc::c_char;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn __uflow(_: *mut FILE) -> libc::c_int;
    fn c_stack_action(
        _: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> libc::c_int;
    fn block_read(_: libc::c_int, _: *mut libc::c_char, _: size_t) -> size_t;
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
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xfreopen(filename: *const libc::c_char, mode: *const libc::c_char, fp: *mut FILE);
    fn xstdopen();
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type pid_t = __pid_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type ptrdiff_t = libc::c_long;
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
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type lin = ptrdiff_t;
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
pub type C2RustUnnamed = libc::c_uint;
pub const FILE2: C2RustUnnamed = 2;
pub const FILE1: C2RustUnnamed = 1;
pub const FILE0: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const FILEC: C2RustUnnamed_0 = 2;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const FC: C2RustUnnamed_1 = 1;
pub const FO: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const RANGE_END: C2RustUnnamed_2 = 1;
pub const RANGE_START: C2RustUnnamed_2 = 0;
pub type diff_type = libc::c_uint;
pub const DIFF_3RD: diff_type = 7;
pub const DIFF_2ND: diff_type = 6;
pub const DIFF_1ST: diff_type = 5;
pub const DIFF_ALL: diff_type = 4;
pub const DIFF_DELETE: diff_type = 3;
pub const DIFF_CHANGE: diff_type = 2;
pub const DIFF_ADD: diff_type = 1;
pub const DIFF_ERROR: diff_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct diff_block {
    pub ranges: [[lin; 2]; 2],
    pub lines: [*mut *mut libc::c_char; 2],
    pub lengths: [*mut size_t; 2],
    pub next: *mut diff_block,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct diff3_block {
    pub correspond: diff_type,
    pub ranges: [[lin; 2]; 3],
    pub lines: [*mut *mut libc::c_char; 3],
    pub lengths: [*mut size_t; 3],
    pub next: *mut diff3_block,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub _gl_dummy: libc::c_int,
}
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
pub type C2RustUnnamed_7 = libc::c_uint;
pub const STRIP_TRAILING_CR_OPTION: C2RustUnnamed_7 = 130;
pub const HELP_OPTION: C2RustUnnamed_7 = 129;
pub const DIFF_PROGRAM_OPTION: C2RustUnnamed_7 = 128;
pub const OPTION_e: C2RustUnnamed_8 = 4;
pub const OPTION_E: C2RustUnnamed_8 = 2;
pub const OPTION_X: C2RustUnnamed_8 = 3;
pub const OPTION_3: C2RustUnnamed_8 = 0;
pub const OPTION_x: C2RustUnnamed_8 = 5;
pub const OPTION_A: C2RustUnnamed_8 = 1;
pub type C2RustUnnamed_8 = libc::c_uint;
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn putc_unlocked(
    mut __c: libc::c_int,
    mut __stream: *mut FILE,
) -> libc::c_int {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as libc::c_int
        as libc::c_long != 0
    {
        __overflow(__stream, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh0 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
        *fresh0 = __c as libc::c_char;
        *fresh0 as libc::c_uchar as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn putchar_unlocked(mut __c: libc::c_int) -> libc::c_int {
    return if ((*stdout)._IO_write_ptr >= (*stdout)._IO_write_end) as libc::c_int
        as libc::c_long != 0
    {
        __overflow(stdout, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh1 = (*stdout)._IO_write_ptr;
        (*stdout)._IO_write_ptr = ((*stdout)._IO_write_ptr).offset(1);
        *fresh1 = __c as libc::c_char;
        *fresh1 as libc::c_uchar as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn feof_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x10 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x20 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fputc_unlocked(
    mut __c: libc::c_int,
    mut __stream: *mut FILE,
) -> libc::c_int {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as libc::c_int
        as libc::c_long != 0
    {
        __overflow(__stream, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh2 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
        *fresh2 = __c as libc::c_char;
        *fresh2 as libc::c_uchar as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE) -> libc::c_int {
    return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as libc::c_int
        as libc::c_long != 0
    {
        __uflow(__fp)
    } else {
        let fresh3 = (*__fp)._IO_read_ptr;
        (*__fp)._IO_read_ptr = ((*__fp)._IO_read_ptr).offset(1);
        *(fresh3 as *mut libc::c_uchar) as libc::c_int
    };
}
static mut PROGRAM_NAME: [libc::c_char; 6] = unsafe {
    *::std::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"diff3\0")
};
static mut text: bool = false;
static mut strip_trailing_cr: bool = false;
static mut edscript: bool = false;
static mut flagging: bool = false;
static mut initial_tab: bool = false;
static mut simple_only: bool = false;
static mut overlap_only: bool = false;
static mut show_2nd: bool = false;
static mut finalwrite: bool = false;
static mut merge: bool = false;
static mut diff_program: *const libc::c_char = b"diff\0" as *const u8
    as *const libc::c_char;
static mut longopts: [option; 14] = [
    {
        let mut init = option {
            name: b"diff-program\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: DIFF_PROGRAM_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"easy-only\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: '3' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ed\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'e' as i32,
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
            name: b"initial-tab\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'T' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"label\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'L' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"merge\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'm' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"overlap-only\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'x' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"show-all\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'A' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"show-overlap\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'E' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"strip-trailing-cr\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: STRIP_TRAILING_CR_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"text\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'a' as i32,
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
            name: 0 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
unsafe extern "C" fn free_diff_block(mut p: *mut diff_block) {}
unsafe extern "C" fn next_to_n2(mut p: *mut diff_block) {}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut common: libc::c_int = 0;
    let mut mapping: [libc::c_int; 3] = [0; 3];
    let mut rev_mapping: [libc::c_int; 3] = [0; 3];
    let mut incompat: libc::c_int = 0 as libc::c_int;
    let mut conflicts_found: bool = false;
    let mut thread0: *mut diff_block = 0 as *mut diff_block;
    let mut thread1: *mut diff_block = 0 as *mut diff_block;
    let mut diff3: *mut diff3_block = 0 as *mut diff3_block;
    let mut tag_count: libc::c_int = 0 as libc::c_int;
    let mut tag_strings: [*mut libc::c_char; 3] = [0 as *mut libc::c_char; 3];
    let mut commonname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut file: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
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
            b"aeimvx3AEL:TX\0" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        let mut current_block_34: u64;
        match c {
            97 => {
                text = 1 as libc::c_int != 0;
                current_block_34 = 3934796541983872331;
            }
            65 => {
                show_2nd = 1 as libc::c_int != 0;
                flagging = 1 as libc::c_int != 0;
                incompat |= (1 as libc::c_int) << OPTION_A as libc::c_int;
                current_block_34 = 3934796541983872331;
            }
            120 => {
                overlap_only = 1 as libc::c_int != 0;
                incompat |= (1 as libc::c_int) << OPTION_x as libc::c_int;
                current_block_34 = 3934796541983872331;
            }
            51 => {
                simple_only = 1 as libc::c_int != 0;
                incompat |= (1 as libc::c_int) << OPTION_3 as libc::c_int;
                current_block_34 = 3934796541983872331;
            }
            105 => {
                finalwrite = 1 as libc::c_int != 0;
                current_block_34 = 3934796541983872331;
            }
            109 => {
                merge = 1 as libc::c_int != 0;
                current_block_34 = 3934796541983872331;
            }
            88 => {
                overlap_only = 1 as libc::c_int != 0;
                incompat |= (1 as libc::c_int) << OPTION_X as libc::c_int;
                current_block_34 = 3934796541983872331;
            }
            69 => {
                flagging = 1 as libc::c_int != 0;
                incompat |= (1 as libc::c_int) << OPTION_E as libc::c_int;
                current_block_34 = 3934796541983872331;
            }
            101 => {
                incompat |= (1 as libc::c_int) << OPTION_e as libc::c_int;
                current_block_34 = 3934796541983872331;
            }
            84 => {
                initial_tab = 1 as libc::c_int != 0;
                current_block_34 = 3934796541983872331;
            }
            130 => {
                strip_trailing_cr = 1 as libc::c_int != 0;
                current_block_34 = 3934796541983872331;
            }
            118 => {
                version_etc(
                    stdout,
                    PROGRAM_NAME.as_ptr(),
                    b"GNU diffutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    proper_name(b"Randy Smith\0" as *const u8 as *const libc::c_char),
                    0 as *mut libc::c_void,
                );
                check_stdout();
                return 0 as libc::c_int;
            }
            128 => {
                diff_program = optarg;
                current_block_34 = 3934796541983872331;
            }
            129 => {
                usage();
                check_stdout();
                return 0 as libc::c_int;
            }
            76 => {
                if tag_count < 3 as libc::c_int {
                    let fresh4 = tag_count;
                    tag_count = tag_count + 1;
                    tag_strings[fresh4 as usize] = optarg;
                    current_block_34 = 3934796541983872331;
                } else {
                    try_help(
                        b"too many file label options\0" as *const u8
                            as *const libc::c_char,
                        0 as *const libc::c_char,
                    );
                    current_block_34 = 15601939255789058605;
                }
            }
            _ => {
                current_block_34 = 15601939255789058605;
            }
        }
        match current_block_34 {
            15601939255789058605 => {
                try_help(0 as *const libc::c_char, 0 as *const libc::c_char);
            }
            _ => {}
        }
    }
    edscript = (incompat != 0) as libc::c_int & !merge as libc::c_int != 0;
    show_2nd = (show_2nd as libc::c_int
        | (incompat == 0) as libc::c_int & merge as libc::c_int) != 0;
    flagging = (flagging as libc::c_int
        | (incompat == 0) as libc::c_int & merge as libc::c_int) != 0;
    if incompat & incompat - 1 as libc::c_int != 0
        || finalwrite as libc::c_int & merge as libc::c_int != 0
        || tag_count != 0 && !flagging
    {
        try_help(
            b"incompatible options\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
        );
    }
    if argc - optind != 3 as libc::c_int {
        if argc - optind < 3 as libc::c_int {
            try_help(
                b"missing operand after '%s'\0" as *const u8 as *const libc::c_char,
                *argv.offset((argc - 1 as libc::c_int) as isize),
            );
        } else {
            try_help(
                b"extra operand '%s'\0" as *const u8 as *const libc::c_char,
                *argv.offset((optind + 3 as libc::c_int) as isize),
            );
        }
    }
    file = &mut *argv.offset(optind as isize) as *mut *mut libc::c_char;
    i = tag_count;
    while i < 3 as libc::c_int {
        tag_strings[i as usize] = *file.offset(i as isize);
        i += 1;
        i;
    }
    common = 2 as libc::c_int - (edscript as libc::c_int | merge as libc::c_int);
    if strcmp(*file.offset(common as isize), b"-\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        common = 3 as libc::c_int - common;
        if strcmp(
            *file.offset(0 as libc::c_int as isize),
            b"-\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *file.offset(common as isize),
                b"-\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            fatal(
                b"'-' specified for more than one input file\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    mapping[0 as libc::c_int as usize] = 0 as libc::c_int;
    mapping[1 as libc::c_int as usize] = 3 as libc::c_int - common;
    mapping[2 as libc::c_int as usize] = common;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        rev_mapping[mapping[i as usize] as usize] = i;
        i += 1;
        i;
    }
    signal(17 as libc::c_int, None);
    let mut b0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut b1: *mut libc::c_char = 0 as *mut libc::c_char;
    commonname = *file.offset(rev_mapping[FILEC as libc::c_int as usize] as isize);
    thread1 = process_diff(
        *file.offset(rev_mapping[FILE1 as libc::c_int as usize] as isize),
        commonname,
        &mut b1,
    );
    thread0 = process_diff(
        *file.offset(rev_mapping[FILE0 as libc::c_int as usize] as isize),
        commonname,
        &mut b0,
    );
    next_to_n2(thread0);
    next_to_n2(thread1);
    diff3 = make_3way_diff(thread0, thread1);
    free_diff_block(thread0);
    free_diff_block(thread1);
    if edscript {
        conflicts_found = output_diff3_edscript(
            stdout,
            diff3,
            mapping.as_mut_ptr() as *const libc::c_int,
            rev_mapping.as_mut_ptr() as *const libc::c_int,
            tag_strings[0 as libc::c_int as usize],
            tag_strings[1 as libc::c_int as usize],
            tag_strings[2 as libc::c_int as usize],
        );
    } else if merge {
        xfreopen(
            *file.offset(rev_mapping[FILE0 as libc::c_int as usize] as isize),
            b"r\0" as *const u8 as *const libc::c_char,
            stdin,
        );
        conflicts_found = output_diff3_merge(
            stdin,
            stdout,
            diff3,
            mapping.as_mut_ptr() as *const libc::c_int,
            rev_mapping.as_mut_ptr() as *const libc::c_int,
            tag_strings[0 as libc::c_int as usize],
            tag_strings[1 as libc::c_int as usize],
            tag_strings[2 as libc::c_int as usize],
        );
        if ferror_unlocked(stdin) != 0 {
            fatal(b"read failed\0" as *const u8 as *const libc::c_char);
        }
    } else {
        output_diff3(
            stdout,
            diff3,
            mapping.as_mut_ptr() as *const libc::c_int,
            rev_mapping.as_mut_ptr() as *const libc::c_int,
        );
        conflicts_found = 0 as libc::c_int != 0;
    }
    rpl_free(b0 as *mut libc::c_void);
    rpl_free(b1 as *mut libc::c_void);
    check_stdout();
    exit(conflicts_found as libc::c_int);
}
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
    if ::std::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong != 0 {
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
unsafe extern "C" fn check_stdout() {
    if ferror_unlocked(stdout) != 0 {
        fatal(b"write failed\0" as *const u8 as *const libc::c_char);
    } else if fclose(stdout) != 0 as libc::c_int {
        perror_with_exit(
            dcgettext(
                0 as *const libc::c_char,
                b"standard output\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
}
static mut option_help_msgid: [*const libc::c_char; 20] = [
    b"-A, --show-all              output all changes, bracketing conflicts\0"
        as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"-e, --ed                    output ed script incorporating changes\n                                from OLDFILE to YOURFILE into MYFILE\0"
        as *const u8 as *const libc::c_char,
    b"-E, --show-overlap          like -e, but bracket conflicts\0" as *const u8
        as *const libc::c_char,
    b"-3, --easy-only             like -e, but incorporate only nonoverlapping changes\0"
        as *const u8 as *const libc::c_char,
    b"-x, --overlap-only          like -e, but incorporate only overlapping changes\0"
        as *const u8 as *const libc::c_char,
    b"-X                          like -x, but bracket conflicts\0" as *const u8
        as *const libc::c_char,
    b"-i                          append 'w' and 'q' commands to ed scripts\0"
        as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"-m, --merge                 output actual merged file, according to\n                                -A if no other options are given\0"
        as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"-a, --text                  treat all files as text\0" as *const u8
        as *const libc::c_char,
    b"    --strip-trailing-cr     strip trailing carriage return on input\0" as *const u8
        as *const libc::c_char,
    b"-T, --initial-tab           make tabs line up by prepending a tab\0" as *const u8
        as *const libc::c_char,
    b"    --diff-program=PROGRAM  use PROGRAM to compare files\0" as *const u8
        as *const libc::c_char,
    b"-L, --label=LABEL           use LABEL instead of file name\n                                (can be repeated up to three times)\0"
        as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"    --help                  display this help and exit\0" as *const u8
        as *const libc::c_char,
    b"-v, --version               output version information and exit\0" as *const u8
        as *const libc::c_char,
    0 as *const libc::c_char,
];
unsafe extern "C" fn usage() {
    let mut p: *const *const libc::c_char = 0 as *const *const libc::c_char;
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"Usage: %s [OPTION]... MYFILE OLDFILE YOURFILE\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        program_name,
    );
    printf(
        b"%s\n\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"Compare three files line by line.\0" as *const u8 as *const libc::c_char,
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
        if **p != 0 {
            printf(
                b"  %s\n\0" as *const u8 as *const libc::c_char,
                dcgettext(0 as *const libc::c_char, *p, 5 as libc::c_int),
            );
        } else {
            putchar_unlocked('\n' as i32);
        }
        p = p.offset(1);
        p;
    }
    fputs_unlocked(
        dcgettext(
            0 as *const libc::c_char,
            b"\nThe default output format is a somewhat human-readable representation of\nthe changes.\n\nThe -e, -E, -x, -X (and corresponding long) options cause an ed script\nto be output instead of the default.\n\nFinally, the -m (--merge) option causes diff3 to do the merge internally\nand output the actual merged file.  For unusual input, this is more\nrobust than using ed.\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    printf(
        b"\n%s\n%s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"If a FILE is '-', read standard input.\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        dcgettext(
            0 as *const libc::c_char,
            b"Exit status is 0 if successful, 1 if conflicts, 2 if trouble.\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    emit_bug_reporting_address();
}
unsafe extern "C" fn make_3way_diff(
    mut thread0: *mut diff_block,
    mut thread1: *mut diff_block,
) -> *mut diff3_block {
    let mut using: [*mut diff_block; 2] = [0 as *mut diff_block; 2];
    let mut last_using: [*mut diff_block; 2] = [0 as *mut diff_block; 2];
    let mut current: [*mut diff_block; 2] = [0 as *mut diff_block; 2];
    let mut high_water_mark: lin = 0;
    let mut high_water_thread: libc::c_int = 0;
    let mut base_water_thread: libc::c_int = 0;
    let mut other_thread: libc::c_int = 0;
    let mut high_water_diff: *mut diff_block = 0 as *mut diff_block;
    let mut other_diff: *mut diff_block = 0 as *mut diff_block;
    let mut result: *mut diff3_block = 0 as *mut diff3_block;
    let mut tmpblock: *mut diff3_block = 0 as *mut diff3_block;
    let mut result_end: *mut *mut diff3_block = 0 as *mut *mut diff3_block;
    let mut last_diff3: *const diff3_block = 0 as *const diff3_block;
    static mut zero_diff3: diff3_block = diff3_block {
        correspond: DIFF_ERROR,
        ranges: [[0; 2]; 3],
        lines: [0 as *const *mut libc::c_char as *mut *mut libc::c_char; 3],
        lengths: [0 as *const size_t as *mut size_t; 3],
        next: 0 as *const diff3_block as *mut diff3_block,
    };
    result = 0 as *mut diff3_block;
    result_end = &mut result;
    current[0 as libc::c_int as usize] = thread0;
    current[1 as libc::c_int as usize] = thread1;
    last_diff3 = &zero_diff3;
    while !(current[0 as libc::c_int as usize]).is_null()
        || !(current[1 as libc::c_int as usize]).is_null()
    {
        last_using[1 as libc::c_int as usize] = 0 as *mut diff_block;
        last_using[0 as libc::c_int as usize] = last_using[1 as libc::c_int as usize];
        using[1 as libc::c_int as usize] = last_using[0 as libc::c_int as usize];
        using[0 as libc::c_int as usize] = using[1 as libc::c_int as usize];
        if (current[0 as libc::c_int as usize]).is_null() {
            base_water_thread = 1 as libc::c_int;
        } else if (current[1 as libc::c_int as usize]).is_null() {
            base_water_thread = 0 as libc::c_int;
        } else {
            base_water_thread = ((*current[0 as libc::c_int as usize])
                .ranges[FC as libc::c_int as usize][RANGE_START as libc::c_int as usize]
                > (*current[1 as libc::c_int as usize])
                    .ranges[FC as libc::c_int
                    as usize][RANGE_START as libc::c_int as usize]) as libc::c_int;
        }
        high_water_thread = base_water_thread;
        high_water_diff = current[high_water_thread as usize];
        high_water_mark = (*high_water_diff)
            .ranges[FC as libc::c_int as usize][RANGE_END as libc::c_int as usize];
        last_using[high_water_thread as usize] = high_water_diff;
        using[high_water_thread as usize] = last_using[high_water_thread as usize];
        current[high_water_thread as usize] = (*high_water_diff).next;
        (*last_using[high_water_thread as usize]).next = 0 as *mut diff_block;
        other_thread = high_water_thread ^ 0x1 as libc::c_int;
        other_diff = current[other_thread as usize];
        while !other_diff.is_null()
            && (*other_diff)
                .ranges[FC as libc::c_int as usize][RANGE_START as libc::c_int as usize]
                <= high_water_mark + 1 as libc::c_int as libc::c_long
        {
            if !(using[other_thread as usize]).is_null() {
                (*last_using[other_thread as usize]).next = other_diff;
            } else {
                using[other_thread as usize] = other_diff;
            }
            last_using[other_thread as usize] = other_diff;
            current[other_thread as usize] = (*current[other_thread as usize]).next;
            (*other_diff).next = 0 as *mut diff_block;
            if high_water_mark
                < (*other_diff)
                    .ranges[FC as libc::c_int
                    as usize][RANGE_END as libc::c_int as usize]
            {
                high_water_thread ^= 1 as libc::c_int;
                high_water_mark = (*other_diff)
                    .ranges[FC as libc::c_int
                    as usize][RANGE_END as libc::c_int as usize];
            }
            other_thread = high_water_thread ^ 0x1 as libc::c_int;
            other_diff = current[other_thread as usize];
        }
        tmpblock = using_to_diff3_block(
            using.as_mut_ptr(),
            last_using.as_mut_ptr(),
            base_water_thread,
            high_water_thread,
            last_diff3,
        );
        if tmpblock.is_null() {
            fatal(
                b"internal error: screwup in format of diff blocks\0" as *const u8
                    as *const libc::c_char,
            );
        }
        *result_end = tmpblock;
        result_end = &mut (*tmpblock).next;
        last_diff3 = tmpblock;
    }
    return result;
}
unsafe extern "C" fn using_to_diff3_block(
    mut using: *mut *mut diff_block,
    mut last_using: *mut *mut diff_block,
    mut low_thread: libc::c_int,
    mut high_thread: libc::c_int,
    mut last_diff3: *const diff3_block,
) -> *mut diff3_block {
    let mut low: [lin; 2] = [0; 2];
    let mut high: [lin; 2] = [0; 2];
    let mut result: *mut diff3_block = 0 as *mut diff3_block;
    let mut ptr: *mut diff_block = 0 as *mut diff_block;
    let mut d: libc::c_int = 0;
    let mut i: lin = 0;
    let mut lowc: lin = (**using.offset(low_thread as isize))
        .ranges[FC as libc::c_int as usize][RANGE_START as libc::c_int as usize];
    let mut highc: lin = (**last_using.offset(high_thread as isize))
        .ranges[FC as libc::c_int as usize][RANGE_END as libc::c_int as usize];
    d = 0 as libc::c_int;
    while d < 2 as libc::c_int {
        if !(*using.offset(d as isize)).is_null() {
            low[d
                as usize] = lowc
                - (**using.offset(d as isize))
                    .ranges[FC as libc::c_int
                    as usize][RANGE_START as libc::c_int as usize]
                + (**using.offset(d as isize))
                    .ranges[FO as libc::c_int
                    as usize][RANGE_START as libc::c_int as usize];
            high[d
                as usize] = highc
                - (**last_using.offset(d as isize))
                    .ranges[FC as libc::c_int
                    as usize][RANGE_END as libc::c_int as usize]
                + (**last_using.offset(d as isize))
                    .ranges[FO as libc::c_int
                    as usize][RANGE_END as libc::c_int as usize];
        } else {
            low[d
                as usize] = lowc
                - (*last_diff3)
                    .ranges[FILEC as libc::c_int
                    as usize][RANGE_END as libc::c_int as usize]
                + (*last_diff3)
                    .ranges[(FILE0 as libc::c_int + d)
                    as usize][RANGE_END as libc::c_int as usize];
            high[d
                as usize] = highc
                - (*last_diff3)
                    .ranges[FILEC as libc::c_int
                    as usize][RANGE_END as libc::c_int as usize]
                + (*last_diff3)
                    .ranges[(FILE0 as libc::c_int + d)
                    as usize][RANGE_END as libc::c_int as usize];
        }
        d += 1;
        d;
    }
    result = create_diff3_block(
        low[0 as libc::c_int as usize],
        high[0 as libc::c_int as usize],
        low[1 as libc::c_int as usize],
        high[1 as libc::c_int as usize],
        lowc,
        highc,
    );
    d = 0 as libc::c_int;
    while d < 2 as libc::c_int {
        ptr = *using.offset(d as isize);
        while !ptr.is_null() {
            let mut result_offset: lin = (*ptr)
                .ranges[FC as libc::c_int as usize][RANGE_START as libc::c_int as usize]
                - lowc;
            if !copy_stringlist(
                (*ptr).lines[FC as libc::c_int as usize] as *const *mut libc::c_char,
                (*ptr).lengths[FC as libc::c_int as usize] as *const size_t,
                ((*result).lines[FILEC as libc::c_int as usize])
                    .offset(result_offset as isize),
                ((*result).lengths[FILEC as libc::c_int as usize])
                    .offset(result_offset as isize),
                (*ptr)
                    .ranges[FC as libc::c_int
                    as usize][RANGE_END as libc::c_int as usize]
                    - (*ptr)
                        .ranges[FC as libc::c_int
                        as usize][RANGE_START as libc::c_int as usize]
                    + 1 as libc::c_int as libc::c_long,
            ) {
                return 0 as *mut diff3_block;
            }
            ptr = (*ptr).next;
        }
        d += 1;
        d;
    }
    d = 0 as libc::c_int;
    while d < 2 as libc::c_int {
        let mut u: *mut diff_block = *using.offset(d as isize);
        let mut lo: lin = low[d as usize];
        let mut hi: lin = high[d as usize];
        i = 0 as libc::c_int as lin;
        while i + lo
            < (if !u.is_null() {
                (*u)
                    .ranges[FO as libc::c_int
                    as usize][RANGE_START as libc::c_int as usize]
            } else {
                hi + 1 as libc::c_int as libc::c_long
            })
        {
            let ref mut fresh5 = *((*result).lines[(FILE0 as libc::c_int + d) as usize])
                .offset(i as isize);
            *fresh5 = *((*result).lines[FILEC as libc::c_int as usize])
                .offset(i as isize);
            *((*result).lengths[(FILE0 as libc::c_int + d) as usize])
                .offset(
                    i as isize,
                ) = *((*result).lengths[FILEC as libc::c_int as usize])
                .offset(i as isize);
            i += 1;
            i;
        }
        ptr = u;
        while !ptr.is_null() {
            let mut result_offset_0: lin = (*ptr)
                .ranges[FO as libc::c_int as usize][RANGE_START as libc::c_int as usize]
                - lo;
            let mut linec: lin = 0;
            if !copy_stringlist(
                (*ptr).lines[FO as libc::c_int as usize] as *const *mut libc::c_char,
                (*ptr).lengths[FO as libc::c_int as usize] as *const size_t,
                ((*result).lines[(FILE0 as libc::c_int + d) as usize])
                    .offset(result_offset_0 as isize),
                ((*result).lengths[(FILE0 as libc::c_int + d) as usize])
                    .offset(result_offset_0 as isize),
                (*ptr)
                    .ranges[FO as libc::c_int
                    as usize][RANGE_END as libc::c_int as usize]
                    - (*ptr)
                        .ranges[FO as libc::c_int
                        as usize][RANGE_START as libc::c_int as usize]
                    + 1 as libc::c_int as libc::c_long,
            ) {
                return 0 as *mut diff3_block;
            }
            linec = (*ptr)
                .ranges[FC as libc::c_int as usize][RANGE_END as libc::c_int as usize]
                + 1 as libc::c_int as libc::c_long - lowc;
            i = (*ptr)
                .ranges[FO as libc::c_int as usize][RANGE_END as libc::c_int as usize]
                + 1 as libc::c_int as libc::c_long - lo;
            while i
                < (if !((*ptr).next).is_null() {
                    (*(*ptr).next)
                        .ranges[FO as libc::c_int
                        as usize][RANGE_START as libc::c_int as usize]
                } else {
                    hi + 1 as libc::c_int as libc::c_long
                }) - lo
            {
                let ref mut fresh6 = *((*result)
                    .lines[(FILE0 as libc::c_int + d) as usize])
                    .offset(i as isize);
                *fresh6 = *((*result).lines[FILEC as libc::c_int as usize])
                    .offset(linec as isize);
                *((*result).lengths[(FILE0 as libc::c_int + d) as usize])
                    .offset(
                        i as isize,
                    ) = *((*result).lengths[FILEC as libc::c_int as usize])
                    .offset(linec as isize);
                linec += 1;
                linec;
                i += 1;
                i;
            }
            ptr = (*ptr).next;
        }
        d += 1;
        d;
    }
    if (*using.offset(0 as libc::c_int as isize)).is_null() {
        (*result).correspond = DIFF_2ND;
    } else if (*using.offset(1 as libc::c_int as isize)).is_null() {
        (*result).correspond = DIFF_1ST;
    } else {
        let mut nl0: lin = (*result)
            .ranges[FILE0 as libc::c_int as usize][RANGE_END as libc::c_int as usize]
            - (*result)
                .ranges[FILE0 as libc::c_int
                as usize][RANGE_START as libc::c_int as usize]
            + 1 as libc::c_int as libc::c_long;
        let mut nl1: lin = (*result)
            .ranges[FILE1 as libc::c_int as usize][RANGE_END as libc::c_int as usize]
            - (*result)
                .ranges[FILE1 as libc::c_int
                as usize][RANGE_START as libc::c_int as usize]
            + 1 as libc::c_int as libc::c_long;
        if nl0 != nl1
            || !compare_line_list(
                (*result).lines[FILE0 as libc::c_int as usize]
                    as *const *mut libc::c_char,
                (*result).lengths[FILE0 as libc::c_int as usize] as *const size_t,
                (*result).lines[FILE1 as libc::c_int as usize]
                    as *const *mut libc::c_char,
                (*result).lengths[FILE1 as libc::c_int as usize] as *const size_t,
                nl0,
            )
        {
            (*result).correspond = DIFF_ALL;
        } else {
            (*result).correspond = DIFF_3RD;
        }
    }
    return result;
}
unsafe extern "C" fn copy_stringlist(
    mut fromptrs: *const *mut libc::c_char,
    mut fromlengths: *const size_t,
    mut toptrs: *mut *mut libc::c_char,
    mut tolengths: *mut size_t,
    mut copynum: lin,
) -> bool {
    let mut f: *const *mut libc::c_char = fromptrs;
    let mut t: *mut *mut libc::c_char = toptrs;
    let mut fl: *const size_t = fromlengths;
    let mut tl: *mut size_t = tolengths;
    loop {
        let fresh7 = copynum;
        copynum = copynum - 1;
        if !(fresh7 != 0) {
            break;
        }
        if !(*t).is_null() {
            if *fl != *tl
                || memcmp(*f as *const libc::c_void, *t as *const libc::c_void, *fl)
                    != 0 as libc::c_int
            {
                return 0 as libc::c_int != 0;
            }
        } else {
            *t = *f;
            *tl = *fl;
        }
        t = t.offset(1);
        t;
        f = f.offset(1);
        f;
        tl = tl.offset(1);
        tl;
        fl = fl.offset(1);
        fl;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn create_diff3_block(
    mut low0: lin,
    mut high0: lin,
    mut low1: lin,
    mut high1: lin,
    mut low2: lin,
    mut high2: lin,
) -> *mut diff3_block {
    let mut result: *mut diff3_block = xmalloc(
        ::std::mem::size_of::<diff3_block>() as libc::c_ulong,
    ) as *mut diff3_block;
    let mut numlines: lin = 0;
    (*result).correspond = DIFF_ERROR;
    (*result).next = 0 as *mut diff3_block;
    (*result)
        .ranges[FILE0 as libc::c_int
        as usize][RANGE_START as libc::c_int as usize] = low0;
    (*result)
        .ranges[FILE0 as libc::c_int
        as usize][RANGE_END as libc::c_int as usize] = high0;
    (*result)
        .ranges[FILE1 as libc::c_int
        as usize][RANGE_START as libc::c_int as usize] = low1;
    (*result)
        .ranges[FILE1 as libc::c_int
        as usize][RANGE_END as libc::c_int as usize] = high1;
    (*result)
        .ranges[FILE2 as libc::c_int
        as usize][RANGE_START as libc::c_int as usize] = low2;
    (*result)
        .ranges[FILE2 as libc::c_int
        as usize][RANGE_END as libc::c_int as usize] = high2;
    numlines = (*result)
        .ranges[FILE0 as libc::c_int as usize][RANGE_END as libc::c_int as usize]
        - (*result)
            .ranges[FILE0 as libc::c_int as usize][RANGE_START as libc::c_int as usize]
        + 1 as libc::c_int as libc::c_long;
    if numlines != 0 {
        (*result)
            .lines[FILE0 as libc::c_int
            as usize] = xcalloc(
            numlines as size_t,
            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        ) as *mut *mut libc::c_char;
        (*result)
            .lengths[FILE0 as libc::c_int
            as usize] = xcalloc(
            numlines as size_t,
            ::std::mem::size_of::<size_t>() as libc::c_ulong,
        ) as *mut size_t;
    } else {
        (*result).lines[FILE0 as libc::c_int as usize] = 0 as *mut *mut libc::c_char;
        (*result).lengths[FILE0 as libc::c_int as usize] = 0 as *mut size_t;
    }
    numlines = (*result)
        .ranges[FILE1 as libc::c_int as usize][RANGE_END as libc::c_int as usize]
        - (*result)
            .ranges[FILE1 as libc::c_int as usize][RANGE_START as libc::c_int as usize]
        + 1 as libc::c_int as libc::c_long;
    if numlines != 0 {
        (*result)
            .lines[FILE1 as libc::c_int
            as usize] = xcalloc(
            numlines as size_t,
            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        ) as *mut *mut libc::c_char;
        (*result)
            .lengths[FILE1 as libc::c_int
            as usize] = xcalloc(
            numlines as size_t,
            ::std::mem::size_of::<size_t>() as libc::c_ulong,
        ) as *mut size_t;
    } else {
        (*result).lines[FILE1 as libc::c_int as usize] = 0 as *mut *mut libc::c_char;
        (*result).lengths[FILE1 as libc::c_int as usize] = 0 as *mut size_t;
    }
    numlines = (*result)
        .ranges[FILE2 as libc::c_int as usize][RANGE_END as libc::c_int as usize]
        - (*result)
            .ranges[FILE2 as libc::c_int as usize][RANGE_START as libc::c_int as usize]
        + 1 as libc::c_int as libc::c_long;
    if numlines != 0 {
        (*result)
            .lines[FILE2 as libc::c_int
            as usize] = xcalloc(
            numlines as size_t,
            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        ) as *mut *mut libc::c_char;
        (*result)
            .lengths[FILE2 as libc::c_int
            as usize] = xcalloc(
            numlines as size_t,
            ::std::mem::size_of::<size_t>() as libc::c_ulong,
        ) as *mut size_t;
    } else {
        (*result).lines[FILE2 as libc::c_int as usize] = 0 as *mut *mut libc::c_char;
        (*result).lengths[FILE2 as libc::c_int as usize] = 0 as *mut size_t;
    }
    return result;
}
unsafe extern "C" fn compare_line_list(
    mut list1: *const *mut libc::c_char,
    mut lengths1: *const size_t,
    mut list2: *const *mut libc::c_char,
    mut lengths2: *const size_t,
    mut nl: lin,
) -> bool {
    let mut l1: *const *mut libc::c_char = list1;
    let mut l2: *const *mut libc::c_char = list2;
    let mut lgths1: *const size_t = lengths1;
    let mut lgths2: *const size_t = lengths2;
    loop {
        let fresh8 = nl;
        nl = nl - 1;
        if !(fresh8 != 0) {
            break;
        }
        if (*l1).is_null() || (*l2).is_null()
            || {
                let fresh9 = lgths2;
                lgths2 = lgths2.offset(1);
                *lgths1 != *fresh9
            }
            || {
                let fresh10 = l1;
                l1 = l1.offset(1);
                let fresh11 = l2;
                l2 = l2.offset(1);
                let fresh12 = lgths1;
                lgths1 = lgths1.offset(1);
                memcmp(
                    *fresh10 as *const libc::c_void,
                    *fresh11 as *const libc::c_void,
                    *fresh12,
                ) != 0 as libc::c_int
            }
        {
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn process_diff(
    mut filea: *const libc::c_char,
    mut fileb: *const libc::c_char,
    mut buf_to_free: *mut *mut libc::c_char,
) -> *mut diff_block {
    let mut diff_contents: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut diff_limit: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut scan_diff: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dt: diff_type = DIFF_ERROR;
    let mut i: lin = 0;
    let mut block_list: *mut diff_block = 0 as *mut diff_block;
    let mut block_list_end: *mut *mut diff_block = &mut block_list;
    diff_limit = read_diff(filea, fileb, &mut diff_contents);
    *buf_to_free = diff_contents;
    scan_diff = diff_contents;
    while scan_diff < diff_limit {
        let mut bptr: *mut diff_block = xmalloc(
            ::std::mem::size_of::<diff_block>() as libc::c_ulong,
        ) as *mut diff_block;
        (*bptr).lines[1 as libc::c_int as usize] = 0 as *mut *mut libc::c_char;
        (*bptr)
            .lines[0 as libc::c_int as usize] = (*bptr).lines[1 as libc::c_int as usize];
        (*bptr).lengths[1 as libc::c_int as usize] = 0 as *mut size_t;
        (*bptr)
            .lengths[0 as libc::c_int
            as usize] = (*bptr).lengths[1 as libc::c_int as usize];
        dt = process_diff_control(&mut scan_diff, bptr);
        if dt as libc::c_uint == DIFF_ERROR as libc::c_int as libc::c_uint
            || *scan_diff as libc::c_int != '\n' as i32
        {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: diff failed: \0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                program_name,
            );
            loop {
                putc_unlocked(*scan_diff as libc::c_int, stderr);
                let fresh13 = scan_diff;
                scan_diff = scan_diff.offset(1);
                if !(*fresh13 as libc::c_int != '\n' as i32) {
                    break;
                }
            }
            exit(2 as libc::c_int);
        }
        scan_diff = scan_diff.offset(1);
        scan_diff;
        match dt as libc::c_uint {
            1 => {
                (*bptr).ranges[0 as libc::c_int as usize][0 as libc::c_int as usize]
                    += 1;
                (*bptr).ranges[0 as libc::c_int as usize][0 as libc::c_int as usize];
            }
            3 => {
                (*bptr).ranges[1 as libc::c_int as usize][0 as libc::c_int as usize]
                    += 1;
                (*bptr).ranges[1 as libc::c_int as usize][0 as libc::c_int as usize];
            }
            2 => {}
            _ => {
                fatal(
                    b"internal error: invalid diff type in process_diff\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        if dt as libc::c_uint != DIFF_ADD as libc::c_int as libc::c_uint {
            let mut numlines: lin = (*bptr)
                .ranges[0 as libc::c_int as usize][RANGE_END as libc::c_int as usize]
                - (*bptr)
                    .ranges[0 as libc::c_int
                    as usize][RANGE_START as libc::c_int as usize]
                + 1 as libc::c_int as libc::c_long;
            (*bptr)
                .lines[0 as libc::c_int
                as usize] = xnmalloc(
                numlines as size_t,
                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            ) as *mut *mut libc::c_char;
            (*bptr)
                .lengths[0 as libc::c_int
                as usize] = xnmalloc(
                numlines as size_t,
                ::std::mem::size_of::<size_t>() as libc::c_ulong,
            ) as *mut size_t;
            i = 0 as libc::c_int as lin;
            while i < numlines {
                scan_diff = scan_diff_line(
                    scan_diff,
                    &mut *(*((*bptr).lines)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .offset(i as isize),
                    &mut *(*((*bptr).lengths)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .offset(i as isize),
                    diff_limit,
                    '<' as i32 as libc::c_char,
                );
                i += 1;
                i;
            }
        }
        if dt as libc::c_uint == DIFF_CHANGE as libc::c_int as libc::c_uint {
            if strncmp(
                scan_diff,
                b"---\n\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) != 0
            {
                fatal(
                    b"invalid diff format; invalid change separator\0" as *const u8
                        as *const libc::c_char,
                );
            }
            scan_diff = scan_diff.offset(4 as libc::c_int as isize);
        }
        if dt as libc::c_uint != DIFF_DELETE as libc::c_int as libc::c_uint {
            let mut numlines_0: lin = (*bptr)
                .ranges[1 as libc::c_int as usize][RANGE_END as libc::c_int as usize]
                - (*bptr)
                    .ranges[1 as libc::c_int
                    as usize][RANGE_START as libc::c_int as usize]
                + 1 as libc::c_int as libc::c_long;
            (*bptr)
                .lines[1 as libc::c_int
                as usize] = xnmalloc(
                numlines_0 as size_t,
                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            ) as *mut *mut libc::c_char;
            (*bptr)
                .lengths[1 as libc::c_int
                as usize] = xnmalloc(
                numlines_0 as size_t,
                ::std::mem::size_of::<size_t>() as libc::c_ulong,
            ) as *mut size_t;
            i = 0 as libc::c_int as lin;
            while i < numlines_0 {
                scan_diff = scan_diff_line(
                    scan_diff,
                    &mut *(*((*bptr).lines)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .offset(i as isize),
                    &mut *(*((*bptr).lengths)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .offset(i as isize),
                    diff_limit,
                    '>' as i32 as libc::c_char,
                );
                i += 1;
                i;
            }
        }
        *block_list_end = bptr;
        block_list_end = &mut (*bptr).next;
    }
    *block_list_end = 0 as *mut diff_block;
    return block_list;
}
unsafe extern "C" fn skipwhite(mut s: *mut libc::c_char) -> *mut libc::c_char {
    while *s as libc::c_int == ' ' as i32 || *s as libc::c_int == '\t' as i32 {
        s = s.offset(1);
        s;
    }
    return s;
}
unsafe extern "C" fn readnum(
    mut s: *mut libc::c_char,
    mut pnum: *mut lin,
) -> *mut libc::c_char {
    let mut c: libc::c_uchar = *s as libc::c_uchar;
    let mut num: lin = 0 as libc::c_int as lin;
    if !((c as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
        <= 9 as libc::c_int as libc::c_uint)
    {
        return 0 as *mut libc::c_char;
    }
    loop {
        num = (c as libc::c_int - '0' as i32) as libc::c_long
            + num * 10 as libc::c_int as libc::c_long;
        s = s.offset(1);
        c = *s as libc::c_uchar;
        if !((c as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
            <= 9 as libc::c_int as libc::c_uint)
        {
            break;
        }
    }
    *pnum = num;
    return s;
}
unsafe extern "C" fn process_diff_control(
    mut string: *mut *mut libc::c_char,
    mut db: *mut diff_block,
) -> diff_type {
    let mut s: *mut libc::c_char = *string;
    let mut type_0: diff_type = DIFF_ERROR;
    s = readnum(
        skipwhite(s),
        &mut *(*((*db).ranges).as_mut_ptr().offset(0 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(RANGE_START as libc::c_int as isize),
    );
    if s.is_null() {
        return DIFF_ERROR;
    }
    s = skipwhite(s);
    if *s as libc::c_int == ',' as i32 {
        s = readnum(
            s.offset(1 as libc::c_int as isize),
            &mut *(*((*db).ranges).as_mut_ptr().offset(0 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(RANGE_END as libc::c_int as isize),
        );
        if s.is_null() {
            return DIFF_ERROR;
        }
    } else {
        (*db)
            .ranges[0 as libc::c_int
            as usize][RANGE_END as libc::c_int
            as usize] = (*db)
            .ranges[0 as libc::c_int as usize][RANGE_START as libc::c_int as usize];
    }
    s = skipwhite(s);
    match *s as libc::c_int {
        97 => {
            type_0 = DIFF_ADD;
        }
        99 => {
            type_0 = DIFF_CHANGE;
        }
        100 => {
            type_0 = DIFF_DELETE;
        }
        _ => return DIFF_ERROR,
    }
    s = s.offset(1);
    s;
    s = readnum(
        skipwhite(s),
        &mut *(*((*db).ranges).as_mut_ptr().offset(1 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(RANGE_START as libc::c_int as isize),
    );
    if s.is_null() {
        return DIFF_ERROR;
    }
    s = skipwhite(s);
    if *s as libc::c_int == ',' as i32 {
        s = readnum(
            s.offset(1 as libc::c_int as isize),
            &mut *(*((*db).ranges).as_mut_ptr().offset(1 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(RANGE_END as libc::c_int as isize),
        );
        if s.is_null() {
            return DIFF_ERROR;
        }
        s = skipwhite(s);
    } else {
        (*db)
            .ranges[1 as libc::c_int
            as usize][RANGE_END as libc::c_int
            as usize] = (*db)
            .ranges[1 as libc::c_int as usize][RANGE_START as libc::c_int as usize];
    }
    *string = s;
    return type_0;
}
unsafe extern "C" fn read_diff(
    mut filea: *const libc::c_char,
    mut fileb: *const libc::c_char,
    mut output_placement: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    let mut diff_result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut current_chunk_size: size_t = 0;
    let mut total: size_t = 0;
    let mut fd: libc::c_int = 0;
    let mut wstatus: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut werrno: libc::c_int = 0 as libc::c_int;
    let mut pipestat: stat = stat {
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
    let mut argv: [*const libc::c_char; 10] = [0 as *const libc::c_char; 10];
    let mut ap: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut fds: [libc::c_int; 2] = [0; 2];
    let mut pid: pid_t = 0;
    ap = argv.as_mut_ptr();
    let fresh14 = ap;
    ap = ap.offset(1);
    *fresh14 = diff_program;
    if text {
        let fresh15 = ap;
        ap = ap.offset(1);
        *fresh15 = b"-a\0" as *const u8 as *const libc::c_char;
    }
    if strip_trailing_cr {
        let fresh16 = ap;
        ap = ap.offset(1);
        *fresh16 = b"--strip-trailing-cr\0" as *const u8 as *const libc::c_char;
    }
    let fresh17 = ap;
    ap = ap.offset(1);
    *fresh17 = b"--horizon-lines=100\0" as *const u8 as *const libc::c_char;
    let fresh18 = ap;
    ap = ap.offset(1);
    *fresh18 = b"---no-directory\0" as *const u8 as *const libc::c_char;
    let fresh19 = ap;
    ap = ap.offset(1);
    *fresh19 = b"--\0" as *const u8 as *const libc::c_char;
    let fresh20 = ap;
    ap = ap.offset(1);
    *fresh20 = filea;
    let fresh21 = ap;
    ap = ap.offset(1);
    *fresh21 = fileb;
    *ap = 0 as *const libc::c_char;
    if pipe(fds.as_mut_ptr()) != 0 as libc::c_int {
        perror_with_exit(b"pipe\0" as *const u8 as *const libc::c_char);
    }
    pid = fork();
    if pid == 0 as libc::c_int {
        close(fds[0 as libc::c_int as usize]);
        if fds[1 as libc::c_int as usize] != 1 as libc::c_int {
            dup2(fds[1 as libc::c_int as usize], 1 as libc::c_int);
            close(fds[1 as libc::c_int as usize]);
        }
        execvp(
            diff_program,
            argv.as_mut_ptr() as *mut *mut libc::c_char as *const *mut libc::c_char,
        );
        _exit(
            if *__errno_location() == 2 as libc::c_int {
                127 as libc::c_int
            } else {
                126 as libc::c_int
            },
        );
    }
    if pid == -(1 as libc::c_int) {
        perror_with_exit(b"fork\0" as *const u8 as *const libc::c_char);
    }
    close(fds[1 as libc::c_int as usize]);
    fd = fds[0 as libc::c_int as usize];
    current_chunk_size = (if fstat(fd, &mut pipestat) == 0 as libc::c_int {
        if 1 as libc::c_int as libc::c_long >= pipestat.st_blksize {
            1 as libc::c_int as libc::c_long
        } else {
            pipestat.st_blksize
        }
    } else {
        (8 as libc::c_int * 1024 as libc::c_int) as libc::c_long
    }) as size_t;
    diff_result = xmalloc(current_chunk_size) as *mut libc::c_char;
    total = 0 as libc::c_int as size_t;
    loop {
        let mut bytes_to_read: size_t = current_chunk_size.wrapping_sub(total);
        let mut bytes: size_t = block_read(
            fd,
            diff_result.offset(total as isize),
            bytes_to_read,
        );
        total = (total as libc::c_ulong).wrapping_add(bytes) as size_t as size_t;
        if bytes != bytes_to_read {
            if bytes == 18446744073709551615 as libc::c_ulong {
                perror_with_exit(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"read failed\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            break;
        } else {
            if (9223372036854775807 as libc::c_long / 2 as libc::c_int as libc::c_long)
                as libc::c_ulong <= current_chunk_size
            {
                xalloc_die();
            }
            current_chunk_size = (current_chunk_size as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            diff_result = xrealloc(diff_result as *mut libc::c_void, current_chunk_size)
                as *mut libc::c_char;
        }
    }
    if total != 0 as libc::c_int as libc::c_ulong
        && *diff_result
            .offset(total.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int != '\n' as i32
    {
        fatal(
            b"invalid diff format; incomplete last line\0" as *const u8
                as *const libc::c_char,
        );
    }
    *output_placement = diff_result;
    if close(fd) != 0 as libc::c_int {
        perror_with_exit(b"close\0" as *const u8 as *const libc::c_char);
    }
    if waitpid(pid, &mut wstatus, 0 as libc::c_int) < 0 as libc::c_int {
        perror_with_exit(b"waitpid\0" as *const u8 as *const libc::c_char);
    }
    status = if werrno == 0 && wstatus & 0x7f as libc::c_int == 0 as libc::c_int {
        (wstatus & 0xff00 as libc::c_int) >> 8 as libc::c_int
    } else {
        2147483647 as libc::c_int
    };
    if 2 as libc::c_int <= status {
        if ::std::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
            error(
                2 as libc::c_int,
                werrno,
                dcgettext(
                    0 as *const libc::c_char,
                    if status == 126 as libc::c_int {
                        b"subsidiary program '%s' could not be invoked\0" as *const u8
                            as *const libc::c_char
                    } else {
                        if status == 127 as libc::c_int {
                            b"subsidiary program '%s' not found\0" as *const u8
                                as *const libc::c_char
                        } else {
                            if status == 2147483647 as libc::c_int {
                                b"subsidiary program '%s' failed\0" as *const u8
                                    as *const libc::c_char
                            } else {
                                b"subsidiary program '%s' failed (exit status %d)\0"
                                    as *const u8 as *const libc::c_char
                            }
                        }
                    },
                    5 as libc::c_int,
                ),
                diff_program,
                status,
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                2 as libc::c_int,
                werrno,
                dcgettext(
                    0 as *const libc::c_char,
                    if status == 126 as libc::c_int {
                        b"subsidiary program '%s' could not be invoked\0" as *const u8
                            as *const libc::c_char
                    } else {
                        if status == 127 as libc::c_int {
                            b"subsidiary program '%s' not found\0" as *const u8
                                as *const libc::c_char
                        } else {
                            if status == 2147483647 as libc::c_int {
                                b"subsidiary program '%s' failed\0" as *const u8
                                    as *const libc::c_char
                            } else {
                                b"subsidiary program '%s' failed (exit status %d)\0"
                                    as *const u8 as *const libc::c_char
                            }
                        }
                    },
                    5 as libc::c_int,
                ),
                diff_program,
                status,
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    return diff_result.offset(total as isize);
}
unsafe extern "C" fn scan_diff_line(
    mut scan_ptr: *mut libc::c_char,
    mut set_start: *mut *mut libc::c_char,
    mut set_length: *mut size_t,
    mut limit: *mut libc::c_char,
    mut leadingchar: libc::c_char,
) -> *mut libc::c_char {
    let mut line_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(*scan_ptr.offset(0 as libc::c_int as isize) as libc::c_int
        == leadingchar as libc::c_int
        && *scan_ptr.offset(1 as libc::c_int as isize) as libc::c_int == ' ' as i32)
    {
        fatal(
            b"invalid diff format; incorrect leading line chars\0" as *const u8
                as *const libc::c_char,
        );
    }
    line_ptr = scan_ptr.offset(2 as libc::c_int as isize);
    *set_start = line_ptr;
    loop {
        let fresh22 = line_ptr;
        line_ptr = line_ptr.offset(1);
        if !(*fresh22 as libc::c_int != '\n' as i32) {
            break;
        }
    }
    *set_length = line_ptr.offset_from(*set_start) as libc::c_long as size_t;
    if line_ptr < limit && *line_ptr as libc::c_int == '\\' as i32 {
        if edscript {
            fprintf(stderr, b"%s:\0" as *const u8 as *const libc::c_char, program_name);
        } else {
            *set_length = (*set_length).wrapping_sub(1);
            *set_length;
        }
        line_ptr = line_ptr.offset(1);
        line_ptr;
        loop {
            if edscript {
                putc_unlocked(*line_ptr as libc::c_int, stderr);
            }
            let fresh23 = line_ptr;
            line_ptr = line_ptr.offset(1);
            if !(*fresh23 as libc::c_int != '\n' as i32) {
                break;
            }
        }
    }
    return line_ptr;
}
unsafe extern "C" fn output_diff3(
    mut outputfile: *mut FILE,
    mut diff: *mut diff3_block,
    mut mapping: *const libc::c_int,
    mut rev_mapping: *const libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut oddoneout: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut diff3_block = 0 as *mut diff3_block;
    let mut line: lin = 0;
    let mut length: size_t = 0;
    let mut dontprint: libc::c_int = 0;
    static mut skew_increment: [libc::c_int; 3] = [
        2 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
    ];
    let mut line_prefix: *const libc::c_char = if initial_tab as libc::c_int != 0 {
        b"\t\0" as *const u8 as *const libc::c_char
    } else {
        b"  \0" as *const u8 as *const libc::c_char
    };
    ptr = diff;
    while !ptr.is_null() {
        let mut x: [libc::c_char; 2] = [0; 2];
        match (*ptr).correspond as libc::c_uint {
            4 => {
                x[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                dontprint = 3 as libc::c_int;
                oddoneout = 3 as libc::c_int;
            }
            5 | 6 | 7 => {
                oddoneout = *rev_mapping
                    .offset(
                        ((*ptr).correspond as libc::c_uint)
                            .wrapping_sub(DIFF_1ST as libc::c_int as libc::c_uint)
                            as isize,
                    );
                x[0 as libc::c_int as usize] = (oddoneout + '1' as i32) as libc::c_char;
                x[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                dontprint = (oddoneout == 0 as libc::c_int) as libc::c_int;
            }
            _ => {
                fatal(
                    b"internal error: invalid diff type passed to output\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        fprintf(
            outputfile,
            b"====%s\n\0" as *const u8 as *const libc::c_char,
            x.as_mut_ptr(),
        );
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            let mut realfile: libc::c_int = *mapping.offset(i as isize);
            let mut lowt: lin = (*ptr)
                .ranges[realfile as usize][RANGE_START as libc::c_int as usize];
            let mut hight: lin = (*ptr)
                .ranges[realfile as usize][RANGE_END as libc::c_int as usize];
            fprintf(
                outputfile,
                b"%d:\0" as *const u8 as *const libc::c_char,
                i + 1 as libc::c_int,
            );
            match lowt - hight {
                1 => {
                    fprintf(
                        outputfile,
                        b"%tda\n\0" as *const u8 as *const libc::c_char,
                        lowt - 1 as libc::c_int as libc::c_long,
                    );
                }
                0 => {
                    fprintf(
                        outputfile,
                        b"%tdc\n\0" as *const u8 as *const libc::c_char,
                        lowt,
                    );
                }
                _ => {
                    fprintf(
                        outputfile,
                        b"%td,%tdc\n\0" as *const u8 as *const libc::c_char,
                        lowt,
                        hight,
                    );
                }
            }
            if !(i == dontprint) {
                if lowt <= hight {
                    line = 0 as libc::c_int as lin;
                    loop {
                        fputs_unlocked(line_prefix, outputfile);
                        cp = *((*ptr).lines[realfile as usize]).offset(line as isize);
                        length = *((*ptr).lengths[realfile as usize])
                            .offset(line as isize);
                        if 0 != 0 && 0 != 0
                            && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                .wrapping_mul(length) <= 8 as libc::c_int as libc::c_ulong
                            && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                != 0 as libc::c_int as libc::c_ulong
                        {
                            ({
                                let mut __ptr: *const libc::c_char = cp
                                    as *const libc::c_char;
                                let mut __stream: *mut FILE = outputfile;
                                let mut __cnt: size_t = 0;
                                __cnt = (::std::mem::size_of::<libc::c_char>()
                                    as libc::c_ulong)
                                    .wrapping_mul(length);
                                while __cnt > 0 as libc::c_int as libc::c_ulong {
                                    let fresh24 = __ptr;
                                    __ptr = __ptr.offset(1);
                                    if putc_unlocked(*fresh24 as libc::c_int, __stream)
                                        == -(1 as libc::c_int)
                                    {
                                        break;
                                    }
                                    __cnt = __cnt.wrapping_sub(1);
                                    __cnt;
                                }
                                0u8
                            });
                        } else {
                            if 0 != 0
                                && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                    == 0 as libc::c_int as libc::c_ulong
                                || 0 != 0 && length == 0 as libc::c_int as libc::c_ulong
                            {} else {
                                fwrite_unlocked(
                                    cp as *const libc::c_void,
                                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                    length,
                                    outputfile,
                                );
                            };
                        };
                        0u8;
                        line += 1;
                        if !(line < hight - lowt + 1 as libc::c_int as libc::c_long) {
                            break;
                        }
                    }
                    if *cp
                        .offset(
                            length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        ) as libc::c_int != '\n' as i32
                    {
                        fprintf(
                            outputfile,
                            b"\n\\ %s\n\0" as *const u8 as *const libc::c_char,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"No newline at end of file\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                }
            }
            i = if oddoneout == 1 as libc::c_int {
                skew_increment[i as usize]
            } else {
                i + 1 as libc::c_int
            };
        }
        ptr = (*ptr).next;
    }
}
unsafe extern "C" fn dotlines(
    mut outputfile: *mut FILE,
    mut b: *mut diff3_block,
    mut filenum: libc::c_int,
) -> bool {
    let mut i: lin = 0;
    let mut leading_dot: bool = 0 as libc::c_int != 0;
    i = 0 as libc::c_int as lin;
    while i
        < (*b).ranges[filenum as usize][RANGE_END as libc::c_int as usize]
            - (*b).ranges[filenum as usize][RANGE_START as libc::c_int as usize]
            + 1 as libc::c_int as libc::c_long
    {
        let mut line: *mut libc::c_char = *((*b).lines[filenum as usize])
            .offset(i as isize);
        if *line.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
            leading_dot = 1 as libc::c_int != 0;
            fputc_unlocked('.' as i32, outputfile);
        }
        if 0 != 0 && 0 != 0
            && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(*((*b).lengths[filenum as usize]).offset(i as isize))
                <= 8 as libc::c_int as libc::c_ulong
            && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                != 0 as libc::c_int as libc::c_ulong
        {
            ({
                let mut __ptr: *const libc::c_char = line as *const libc::c_char;
                let mut __stream: *mut FILE = outputfile;
                let mut __cnt: size_t = 0;
                __cnt = (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul(*((*b).lengths[filenum as usize]).offset(i as isize));
                while __cnt > 0 as libc::c_int as libc::c_ulong {
                    let fresh25 = __ptr;
                    __ptr = __ptr.offset(1);
                    if putc_unlocked(*fresh25 as libc::c_int, __stream)
                        == -(1 as libc::c_int)
                    {
                        break;
                    }
                    __cnt = __cnt.wrapping_sub(1);
                    __cnt;
                }
                0u8
            });
        } else {
            if 0 != 0
                && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    == 0 as libc::c_int as libc::c_ulong
                || 0 != 0
                    && *((*b).lengths[filenum as usize]).offset(i as isize)
                        == 0 as libc::c_int as libc::c_ulong
            {} else {
                fwrite_unlocked(
                    line as *const libc::c_void,
                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    *((*b).lengths[filenum as usize]).offset(i as isize),
                    outputfile,
                );
            };
        };
        0u8;
        i += 1;
        i;
    }
    return leading_dot;
}
unsafe extern "C" fn undotlines(
    mut outputfile: *mut FILE,
    mut leading_dot: bool,
    mut start: lin,
    mut num: lin,
) {
    fputs_unlocked(b".\n\0" as *const u8 as *const libc::c_char, outputfile);
    if leading_dot {
        if num == 1 as libc::c_int as libc::c_long {
            fprintf(
                outputfile,
                b"%tds/^\\.//\n\0" as *const u8 as *const libc::c_char,
                start,
            );
        } else {
            fprintf(
                outputfile,
                b"%td,%tds/^\\.//\n\0" as *const u8 as *const libc::c_char,
                start,
                start + num - 1 as libc::c_int as libc::c_long,
            );
        }
    }
}
unsafe extern "C" fn output_diff3_edscript(
    mut outputfile: *mut FILE,
    mut diff: *mut diff3_block,
    mut mapping: *const libc::c_int,
    mut rev_mapping: *const libc::c_int,
    mut file0: *const libc::c_char,
    mut file1: *const libc::c_char,
    mut file2: *const libc::c_char,
) -> bool {
    let mut leading_dot: bool = false;
    let mut conflicts_found: bool = 0 as libc::c_int != 0;
    let mut conflict: bool = false;
    let mut b: *mut diff3_block = 0 as *mut diff3_block;
    let mut current_block_36: u64;
    b = reverse_diff3_blocklist(diff);
    while !b.is_null() {
        let mut type_0: diff_type = (if (*b).correspond as libc::c_uint
            == DIFF_ALL as libc::c_int as libc::c_uint
        {
            DIFF_ALL as libc::c_int
        } else {
            DIFF_1ST as libc::c_int
                + *rev_mapping
                    .offset(
                        ((*b).correspond as libc::c_uint)
                            .wrapping_sub(DIFF_1ST as libc::c_int as libc::c_uint)
                            as isize,
                    )
        }) as diff_type;
        match type_0 as libc::c_uint {
            6 => {
                if !show_2nd {
                    current_block_36 = 8258075665625361029;
                } else {
                    conflict = 1 as libc::c_int != 0;
                    current_block_36 = 1841672684692190573;
                }
            }
            7 => {
                if overlap_only {
                    current_block_36 = 8258075665625361029;
                } else {
                    conflict = 0 as libc::c_int != 0;
                    current_block_36 = 1841672684692190573;
                }
            }
            4 => {
                if simple_only {
                    current_block_36 = 8258075665625361029;
                } else {
                    conflict = flagging;
                    current_block_36 = 1841672684692190573;
                }
            }
            _ => {
                current_block_36 = 8258075665625361029;
            }
        }
        match current_block_36 {
            1841672684692190573 => {
                let mut low0: lin = (*b)
                    .ranges[*mapping.offset(FILE0 as libc::c_int as isize)
                    as usize][RANGE_START as libc::c_int as usize];
                let mut high0: lin = (*b)
                    .ranges[*mapping.offset(FILE0 as libc::c_int as isize)
                    as usize][RANGE_END as libc::c_int as usize];
                if conflict {
                    conflicts_found = 1 as libc::c_int != 0;
                    fprintf(
                        outputfile,
                        b"%tda\n\0" as *const u8 as *const libc::c_char,
                        high0,
                    );
                    leading_dot = 0 as libc::c_int != 0;
                    if type_0 as libc::c_uint == DIFF_ALL as libc::c_int as libc::c_uint
                    {
                        if show_2nd {
                            fprintf(
                                outputfile,
                                b"||||||| %s\n\0" as *const u8 as *const libc::c_char,
                                file1,
                            );
                            leading_dot = dotlines(
                                outputfile,
                                b,
                                *mapping.offset(FILE1 as libc::c_int as isize),
                            );
                        }
                        fputs_unlocked(
                            b"=======\n\0" as *const u8 as *const libc::c_char,
                            outputfile,
                        );
                        leading_dot = (leading_dot as libc::c_int
                            | dotlines(
                                outputfile,
                                b,
                                *mapping.offset(FILE2 as libc::c_int as isize),
                            ) as libc::c_int) != 0;
                    }
                    fprintf(
                        outputfile,
                        b">>>>>>> %s\n\0" as *const u8 as *const libc::c_char,
                        file2,
                    );
                    undotlines(
                        outputfile,
                        leading_dot,
                        high0 + 2 as libc::c_int as libc::c_long,
                        (*b)
                            .ranges[*mapping.offset(FILE1 as libc::c_int as isize)
                            as usize][RANGE_END as libc::c_int as usize]
                            - (*b)
                                .ranges[*mapping.offset(FILE1 as libc::c_int as isize)
                                as usize][RANGE_START as libc::c_int as usize]
                            + 1 as libc::c_int as libc::c_long
                            + ((*b)
                                .ranges[*mapping.offset(FILE2 as libc::c_int as isize)
                                as usize][RANGE_END as libc::c_int as usize]
                                - (*b)
                                    .ranges[*mapping.offset(FILE2 as libc::c_int as isize)
                                    as usize][RANGE_START as libc::c_int as usize]
                                + 1 as libc::c_int as libc::c_long)
                            + 1 as libc::c_int as libc::c_long,
                    );
                    fprintf(
                        outputfile,
                        b"%tda\n<<<<<<< %s\n\0" as *const u8 as *const libc::c_char,
                        low0 - 1 as libc::c_int as libc::c_long,
                        if type_0 as libc::c_uint
                            == DIFF_ALL as libc::c_int as libc::c_uint
                        {
                            file0
                        } else {
                            file1
                        },
                    );
                    leading_dot = 0 as libc::c_int != 0;
                    if type_0 as libc::c_uint == DIFF_2ND as libc::c_int as libc::c_uint
                    {
                        leading_dot = dotlines(
                            outputfile,
                            b,
                            *mapping.offset(FILE1 as libc::c_int as isize),
                        );
                        fputs_unlocked(
                            b"=======\n\0" as *const u8 as *const libc::c_char,
                            outputfile,
                        );
                    }
                    undotlines(
                        outputfile,
                        leading_dot,
                        low0 + 1 as libc::c_int as libc::c_long,
                        (*b)
                            .ranges[*mapping.offset(FILE1 as libc::c_int as isize)
                            as usize][RANGE_END as libc::c_int as usize]
                            - (*b)
                                .ranges[*mapping.offset(FILE1 as libc::c_int as isize)
                                as usize][RANGE_START as libc::c_int as usize]
                            + 1 as libc::c_int as libc::c_long,
                    );
                } else if (*b)
                    .ranges[*mapping.offset(FILE2 as libc::c_int as isize)
                    as usize][RANGE_END as libc::c_int as usize]
                    - (*b)
                        .ranges[*mapping.offset(FILE2 as libc::c_int as isize)
                        as usize][RANGE_START as libc::c_int as usize]
                    + 1 as libc::c_int as libc::c_long
                    == 0 as libc::c_int as libc::c_long
                {
                    if low0 == high0 {
                        fprintf(
                            outputfile,
                            b"%tdd\n\0" as *const u8 as *const libc::c_char,
                            low0,
                        );
                    } else {
                        fprintf(
                            outputfile,
                            b"%td,%tdd\n\0" as *const u8 as *const libc::c_char,
                            low0,
                            high0,
                        );
                    }
                } else {
                    match high0 - low0 {
                        -1 => {
                            fprintf(
                                outputfile,
                                b"%tda\n\0" as *const u8 as *const libc::c_char,
                                high0,
                            );
                        }
                        0 => {
                            fprintf(
                                outputfile,
                                b"%tdc\n\0" as *const u8 as *const libc::c_char,
                                high0,
                            );
                        }
                        _ => {
                            fprintf(
                                outputfile,
                                b"%td,%tdc\n\0" as *const u8 as *const libc::c_char,
                                low0,
                                high0,
                            );
                        }
                    }
                    undotlines(
                        outputfile,
                        dotlines(
                            outputfile,
                            b,
                            *mapping.offset(FILE2 as libc::c_int as isize),
                        ),
                        low0,
                        (*b)
                            .ranges[*mapping.offset(FILE2 as libc::c_int as isize)
                            as usize][RANGE_END as libc::c_int as usize]
                            - (*b)
                                .ranges[*mapping.offset(FILE2 as libc::c_int as isize)
                                as usize][RANGE_START as libc::c_int as usize]
                            + 1 as libc::c_int as libc::c_long,
                    );
                }
            }
            _ => {}
        }
        b = (*b).next;
    }
    if finalwrite {
        fputs_unlocked(b"w\nq\n\0" as *const u8 as *const libc::c_char, outputfile);
    }
    return conflicts_found;
}
unsafe extern "C" fn output_diff3_merge(
    mut infile: *mut FILE,
    mut outputfile: *mut FILE,
    mut diff: *mut diff3_block,
    mut mapping: *const libc::c_int,
    mut rev_mapping: *const libc::c_int,
    mut file0: *const libc::c_char,
    mut file1: *const libc::c_char,
    mut file2: *const libc::c_char,
) -> bool {
    let mut c: libc::c_int = 0;
    let mut i: lin = 0;
    let mut conflicts_found: bool = 0 as libc::c_int != 0;
    let mut conflict: bool = false;
    let mut b: *mut diff3_block = 0 as *mut diff3_block;
    let mut linesread: lin = 0 as libc::c_int as lin;
    let mut current_block_51: u64;
    b = diff;
    while !b.is_null() {
        let mut type_0: diff_type = (if (*b).correspond as libc::c_uint
            == DIFF_ALL as libc::c_int as libc::c_uint
        {
            DIFF_ALL as libc::c_int
        } else {
            DIFF_1ST as libc::c_int
                + *rev_mapping
                    .offset(
                        ((*b).correspond as libc::c_uint)
                            .wrapping_sub(DIFF_1ST as libc::c_int as libc::c_uint)
                            as isize,
                    )
        }) as diff_type;
        let mut format_2nd: *const libc::c_char = b"<<<<<<< %s\n\0" as *const u8
            as *const libc::c_char;
        match type_0 as libc::c_uint {
            6 => {
                if !show_2nd {
                    current_block_51 = 12517898123489920830;
                } else {
                    conflict = 1 as libc::c_int != 0;
                    current_block_51 = 12599329904712511516;
                }
            }
            7 => {
                if overlap_only {
                    current_block_51 = 12517898123489920830;
                } else {
                    conflict = 0 as libc::c_int != 0;
                    current_block_51 = 12599329904712511516;
                }
            }
            4 => {
                if simple_only {
                    current_block_51 = 12517898123489920830;
                } else {
                    conflict = flagging;
                    format_2nd = b"||||||| %s\n\0" as *const u8 as *const libc::c_char;
                    current_block_51 = 12599329904712511516;
                }
            }
            _ => {
                current_block_51 = 12517898123489920830;
            }
        }
        match current_block_51 {
            12599329904712511516 => {
                i = (*b)
                    .ranges[FILE0 as libc::c_int
                    as usize][RANGE_START as libc::c_int as usize] - linesread
                    - 1 as libc::c_int as libc::c_long;
                linesread += i;
                loop {
                    i -= 1;
                    if !(0 as libc::c_int as libc::c_long <= i) {
                        break;
                    }
                    loop {
                        c = getc_unlocked(infile);
                        if c == -(1 as libc::c_int) {
                            if ferror_unlocked(infile) != 0 {
                                perror_with_exit(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"read failed\0" as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            } else if feof_unlocked(infile) != 0 {
                                fatal(
                                    b"input file shrank\0" as *const u8 as *const libc::c_char,
                                );
                            }
                        }
                        putc_unlocked(c, outputfile);
                        if !(c != '\n' as i32) {
                            break;
                        }
                    }
                }
                if conflict {
                    conflicts_found = 1 as libc::c_int != 0;
                    if type_0 as libc::c_uint == DIFF_ALL as libc::c_int as libc::c_uint
                    {
                        fprintf(
                            outputfile,
                            b"<<<<<<< %s\n\0" as *const u8 as *const libc::c_char,
                            file0,
                        );
                        i = 0 as libc::c_int as lin;
                        while i
                            < (*b)
                                .ranges[*mapping.offset(FILE0 as libc::c_int as isize)
                                as usize][RANGE_END as libc::c_int as usize]
                                - (*b)
                                    .ranges[*mapping.offset(FILE0 as libc::c_int as isize)
                                    as usize][RANGE_START as libc::c_int as usize]
                                + 1 as libc::c_int as libc::c_long
                        {
                            if 0 != 0 && 0 != 0
                                && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                    .wrapping_mul(
                                        *((*b)
                                            .lengths[*mapping.offset(FILE0 as libc::c_int as isize)
                                            as usize])
                                            .offset(i as isize),
                                    ) <= 8 as libc::c_int as libc::c_ulong
                                && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                    != 0 as libc::c_int as libc::c_ulong
                            {
                                ({
                                    let mut __ptr: *const libc::c_char = *((*b)
                                        .lines[*mapping.offset(FILE0 as libc::c_int as isize)
                                        as usize])
                                        .offset(i as isize) as *const libc::c_char;
                                    let mut __stream: *mut FILE = outputfile;
                                    let mut __cnt: size_t = 0;
                                    __cnt = (::std::mem::size_of::<libc::c_char>()
                                        as libc::c_ulong)
                                        .wrapping_mul(
                                            *((*b)
                                                .lengths[*mapping.offset(FILE0 as libc::c_int as isize)
                                                as usize])
                                                .offset(i as isize),
                                        );
                                    while __cnt > 0 as libc::c_int as libc::c_ulong {
                                        let fresh26 = __ptr;
                                        __ptr = __ptr.offset(1);
                                        if putc_unlocked(*fresh26 as libc::c_int, __stream)
                                            == -(1 as libc::c_int)
                                        {
                                            break;
                                        }
                                        __cnt = __cnt.wrapping_sub(1);
                                        __cnt;
                                    }
                                    0u8
                                });
                            } else {
                                if 0 != 0
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        == 0 as libc::c_int as libc::c_ulong
                                    || 0 != 0
                                        && *((*b)
                                            .lengths[*mapping.offset(FILE0 as libc::c_int as isize)
                                            as usize])
                                            .offset(i as isize) == 0 as libc::c_int as libc::c_ulong
                                {} else {
                                    fwrite_unlocked(
                                        *((*b)
                                            .lines[*mapping.offset(FILE0 as libc::c_int as isize)
                                            as usize])
                                            .offset(i as isize) as *const libc::c_void,
                                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                        *((*b)
                                            .lengths[*mapping.offset(FILE0 as libc::c_int as isize)
                                            as usize])
                                            .offset(i as isize),
                                        outputfile,
                                    );
                                };
                            };
                            0u8;
                            i += 1;
                            i;
                        }
                    }
                    if show_2nd {
                        fprintf(outputfile, format_2nd, file1);
                        i = 0 as libc::c_int as lin;
                        while i
                            < (*b)
                                .ranges[*mapping.offset(FILE1 as libc::c_int as isize)
                                as usize][RANGE_END as libc::c_int as usize]
                                - (*b)
                                    .ranges[*mapping.offset(FILE1 as libc::c_int as isize)
                                    as usize][RANGE_START as libc::c_int as usize]
                                + 1 as libc::c_int as libc::c_long
                        {
                            if 0 != 0 && 0 != 0
                                && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                    .wrapping_mul(
                                        *((*b)
                                            .lengths[*mapping.offset(FILE1 as libc::c_int as isize)
                                            as usize])
                                            .offset(i as isize),
                                    ) <= 8 as libc::c_int as libc::c_ulong
                                && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                    != 0 as libc::c_int as libc::c_ulong
                            {
                                ({
                                    let mut __ptr: *const libc::c_char = *((*b)
                                        .lines[*mapping.offset(FILE1 as libc::c_int as isize)
                                        as usize])
                                        .offset(i as isize) as *const libc::c_char;
                                    let mut __stream: *mut FILE = outputfile;
                                    let mut __cnt: size_t = 0;
                                    __cnt = (::std::mem::size_of::<libc::c_char>()
                                        as libc::c_ulong)
                                        .wrapping_mul(
                                            *((*b)
                                                .lengths[*mapping.offset(FILE1 as libc::c_int as isize)
                                                as usize])
                                                .offset(i as isize),
                                        );
                                    while __cnt > 0 as libc::c_int as libc::c_ulong {
                                        let fresh27 = __ptr;
                                        __ptr = __ptr.offset(1);
                                        if putc_unlocked(*fresh27 as libc::c_int, __stream)
                                            == -(1 as libc::c_int)
                                        {
                                            break;
                                        }
                                        __cnt = __cnt.wrapping_sub(1);
                                        __cnt;
                                    }
                                    0u8
                                });
                            } else {
                                if 0 != 0
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        == 0 as libc::c_int as libc::c_ulong
                                    || 0 != 0
                                        && *((*b)
                                            .lengths[*mapping.offset(FILE1 as libc::c_int as isize)
                                            as usize])
                                            .offset(i as isize) == 0 as libc::c_int as libc::c_ulong
                                {} else {
                                    fwrite_unlocked(
                                        *((*b)
                                            .lines[*mapping.offset(FILE1 as libc::c_int as isize)
                                            as usize])
                                            .offset(i as isize) as *const libc::c_void,
                                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                        *((*b)
                                            .lengths[*mapping.offset(FILE1 as libc::c_int as isize)
                                            as usize])
                                            .offset(i as isize),
                                        outputfile,
                                    );
                                };
                            };
                            0u8;
                            i += 1;
                            i;
                        }
                    }
                    fputs_unlocked(
                        b"=======\n\0" as *const u8 as *const libc::c_char,
                        outputfile,
                    );
                }
                i = 0 as libc::c_int as lin;
                while i
                    < (*b)
                        .ranges[*mapping.offset(FILE2 as libc::c_int as isize)
                        as usize][RANGE_END as libc::c_int as usize]
                        - (*b)
                            .ranges[*mapping.offset(FILE2 as libc::c_int as isize)
                            as usize][RANGE_START as libc::c_int as usize]
                        + 1 as libc::c_int as libc::c_long
                {
                    if 0 != 0 && 0 != 0
                        && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                            .wrapping_mul(
                                *((*b)
                                    .lengths[*mapping.offset(FILE2 as libc::c_int as isize)
                                    as usize])
                                    .offset(i as isize),
                            ) <= 8 as libc::c_int as libc::c_ulong
                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                            != 0 as libc::c_int as libc::c_ulong
                    {
                        ({
                            let mut __ptr: *const libc::c_char = *((*b)
                                .lines[*mapping.offset(FILE2 as libc::c_int as isize)
                                as usize])
                                .offset(i as isize) as *const libc::c_char;
                            let mut __stream: *mut FILE = outputfile;
                            let mut __cnt: size_t = 0;
                            __cnt = (::std::mem::size_of::<libc::c_char>()
                                as libc::c_ulong)
                                .wrapping_mul(
                                    *((*b)
                                        .lengths[*mapping.offset(FILE2 as libc::c_int as isize)
                                        as usize])
                                        .offset(i as isize),
                                );
                            while __cnt > 0 as libc::c_int as libc::c_ulong {
                                let fresh28 = __ptr;
                                __ptr = __ptr.offset(1);
                                if putc_unlocked(*fresh28 as libc::c_int, __stream)
                                    == -(1 as libc::c_int)
                                {
                                    break;
                                }
                                __cnt = __cnt.wrapping_sub(1);
                                __cnt;
                            }
                            0u8
                        });
                    } else {
                        if 0 != 0
                            && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                == 0 as libc::c_int as libc::c_ulong
                            || 0 != 0
                                && *((*b)
                                    .lengths[*mapping.offset(FILE2 as libc::c_int as isize)
                                    as usize])
                                    .offset(i as isize) == 0 as libc::c_int as libc::c_ulong
                        {} else {
                            fwrite_unlocked(
                                *((*b)
                                    .lines[*mapping.offset(FILE2 as libc::c_int as isize)
                                    as usize])
                                    .offset(i as isize) as *const libc::c_void,
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                *((*b)
                                    .lengths[*mapping.offset(FILE2 as libc::c_int as isize)
                                    as usize])
                                    .offset(i as isize),
                                outputfile,
                            );
                        };
                    };
                    0u8;
                    i += 1;
                    i;
                }
                if conflict {
                    fprintf(
                        outputfile,
                        b">>>>>>> %s\n\0" as *const u8 as *const libc::c_char,
                        file2,
                    );
                }
                i = (*b)
                    .ranges[FILE0 as libc::c_int
                    as usize][RANGE_END as libc::c_int as usize]
                    - (*b)
                        .ranges[FILE0 as libc::c_int
                        as usize][RANGE_START as libc::c_int as usize]
                    + 1 as libc::c_int as libc::c_long;
                linesread += i;
                loop {
                    i -= 1;
                    if !(0 as libc::c_int as libc::c_long <= i) {
                        break;
                    }
                    loop {
                        c = getc_unlocked(infile);
                        if !(c != '\n' as i32) {
                            break;
                        }
                        if c == -(1 as libc::c_int) {
                            if ferror_unlocked(infile) != 0 {
                                perror_with_exit(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"read failed\0" as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            } else if feof_unlocked(infile) != 0 {
                                if i != 0 || !((*b).next).is_null() {
                                    fatal(
                                        b"input file shrank\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                return conflicts_found;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        b = (*b).next;
    }
    loop {
        c = getc_unlocked(infile);
        if !(c != -(1 as libc::c_int)
            || ferror_unlocked(infile) | feof_unlocked(infile) == 0)
        {
            break;
        }
        putc_unlocked(c, outputfile);
    }
    return conflicts_found;
}
unsafe extern "C" fn reverse_diff3_blocklist(
    mut diff: *mut diff3_block,
) -> *mut diff3_block {
    let mut tmp: *mut diff3_block = 0 as *mut diff3_block;
    let mut next: *mut diff3_block = 0 as *mut diff3_block;
    let mut prev: *mut diff3_block = 0 as *mut diff3_block;
    tmp = diff;
    prev = 0 as *mut diff3_block;
    while !tmp.is_null() {
        next = (*tmp).next;
        (*tmp).next = prev;
        prev = tmp;
        tmp = next;
    }
    return prev;
}
unsafe extern "C" fn fatal(mut msgid: *const libc::c_char) {
    if ::std::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
        error(
            2 as libc::c_int,
            0 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            dcgettext(0 as *const libc::c_char, msgid, 5 as libc::c_int),
        );
        if 0 as libc::c_int != 0 {} else {
            unreachable!();
        };
    } else {
        error(
            2 as libc::c_int,
            0 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            dcgettext(0 as *const libc::c_char, msgid, 5 as libc::c_int),
        );
        if 0 as libc::c_int != 0 {} else {
            unreachable!();
        };
    };
}
unsafe extern "C" fn perror_with_exit(mut string: *const libc::c_char) {
    if ::std::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
        error(
            2 as libc::c_int,
            *__errno_location(),
            b"%s\0" as *const u8 as *const libc::c_char,
            string,
        );
        if 0 as libc::c_int != 0 {} else {
            unreachable!();
        };
    } else {
        error(
            2 as libc::c_int,
            *__errno_location(),
            b"%s\0" as *const u8 as *const libc::c_char,
            string,
        );
        if 0 as libc::c_int != 0 {} else {
            unreachable!();
        };
    };
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
