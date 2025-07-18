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
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn fork() -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn raise(__sig: libc::c_int) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
    fn rpl_free(ptr: *mut libc::c_void);
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn __strtol_internal(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
        __group: libc::c_int,
    ) -> libc::c_long;
    fn rawmemchr(__s: *const libc::c_void, __c: libc::c_int) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn mempcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
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
    static mut Version: *const libc::c_char;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn __uflow(_: *mut FILE) -> libc::c_int;
    fn rpl_fopen(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fread_unlocked(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn c_stack_action(
        _: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> libc::c_int;
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
    fn base_len(filename: *const libc::c_char) -> size_t;
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
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstdopen();
}
pub type __uint32_t = libc::c_uint;
pub type __intmax_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type pid_t = __pid_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
pub type intmax_t = __intmax_t;
pub type C2RustUnnamed_10 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_10 = 8;
pub const _ISpunct: C2RustUnnamed_10 = 4;
pub const _IScntrl: C2RustUnnamed_10 = 2;
pub const _ISblank: C2RustUnnamed_10 = 1;
pub const _ISgraph: C2RustUnnamed_10 = 32768;
pub const _ISprint: C2RustUnnamed_10 = 16384;
pub const _ISspace: C2RustUnnamed_10 = 8192;
pub const _ISxdigit: C2RustUnnamed_10 = 4096;
pub const _ISdigit: C2RustUnnamed_10 = 2048;
pub const _ISalpha: C2RustUnnamed_10 = 1024;
pub const _ISlower: C2RustUnnamed_10 = 512;
pub const _ISupper: C2RustUnnamed_10 = 256;
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
pub type C2RustUnnamed_11 = libc::c_uint;
pub const SDIFF_BUFSIZE: C2RustUnnamed_11 = 65536;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line_filter {
    pub infile: *mut FILE,
    pub bufpos: *mut libc::c_char,
    pub buffer: *mut libc::c_char,
    pub buflim: *mut libc::c_char,
}
pub const NUM_SIGS: C2RustUnnamed_12 = 7;
pub type C2RustUnnamed_12 = libc::c_uint;
pub const handler_index_of_SIGINT: C2RustUnnamed_12 = 6;
pub type C2RustUnnamed_13 = libc::c_uint;
pub const TABSIZE_OPTION: C2RustUnnamed_13 = 131;
pub const STRIP_TRAILING_CR_OPTION: C2RustUnnamed_13 = 130;
pub const HELP_OPTION: C2RustUnnamed_13 = 129;
pub const DIFF_PROGRAM_OPTION: C2RustUnnamed_13 = 128;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
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
unsafe extern "C" fn strtoimax(
    mut nptr: *const libc::c_char,
    mut endptr: *mut *mut libc::c_char,
    mut base: libc::c_int,
) -> intmax_t {
    return __strtol_internal(nptr, endptr, base, 0 as libc::c_int);
}
#[inline]
unsafe extern "C" fn feof_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x10 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
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
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x20 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn getchar_unlocked() -> libc::c_int {
    return if ((*stdin)._IO_read_ptr >= (*stdin)._IO_read_end) as libc::c_int
        as libc::c_long != 0
    {
        __uflow(stdin)
    } else {
        let fresh1 = (*stdin)._IO_read_ptr;
        (*stdin)._IO_read_ptr = ((*stdin)._IO_read_ptr).offset(1);
        *(fresh1 as *mut libc::c_uchar) as libc::c_int
    };
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
        let fresh2 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
        *fresh2 = __c as libc::c_char;
        *fresh2 as libc::c_uchar as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn putchar_unlocked(mut __c: libc::c_int) -> libc::c_int {
    return if ((*stdout)._IO_write_ptr >= (*stdout)._IO_write_end) as libc::c_int
        as libc::c_long != 0
    {
        __overflow(stdout, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh3 = (*stdout)._IO_write_ptr;
        (*stdout)._IO_write_ptr = ((*stdout)._IO_write_ptr).offset(1);
        *fresh3 = __c as libc::c_char;
        *fresh3 as libc::c_uchar as libc::c_int
    };
}
static mut PROGRAM_NAME: [libc::c_char; 6] = unsafe {
    *::std::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"sdiff\0")
};
static mut editor_program: *const libc::c_char = b"ed\0" as *const u8
    as *const libc::c_char;
static mut diffargv: *mut *const libc::c_char = 0 as *const *const libc::c_char
    as *mut *const libc::c_char;
static mut tmpname: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut tmp: *mut FILE = 0 as *const FILE as *mut FILE;
static mut diffpid: pid_t = 0;
static mut sigs: [libc::c_int; 7] = [
    1 as libc::c_int,
    3 as libc::c_int,
    15 as libc::c_int,
    24 as libc::c_int,
    25 as libc::c_int,
    13 as libc::c_int,
    2 as libc::c_int,
];
static mut initial_action: [sigaction; 7] = [sigaction {
    __sigaction_handler: C2RustUnnamed_9 {
        sa_handler: None,
    },
    sa_mask: __sigset_t { __val: [0; 16] },
    sa_flags: 0,
    sa_restorer: None,
}; 7];
static mut output: *const libc::c_char = 0 as *const libc::c_char;
static mut suppress_common_lines: bool = false;
static mut longopts: [option; 21] = [
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
            name: b"expand-tabs\0" as *const u8 as *const libc::c_char,
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
            val: HELP_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore-all-space\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'W' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore-blank-lines\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'B' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore-case\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore-matching-lines\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'I' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore-space-change\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore-tab-expansion\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'E' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore-trailing-space\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'Z' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"left-column\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"minimal\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"output\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'o' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"speed-large-files\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'H' as i32,
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
            name: b"suppress-common-lines\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"tabsize\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: TABSIZE_OPTION as libc::c_int,
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
            name: b"width\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'w' as i32,
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
    if ::std::mem::size_of::<C2RustUnnamed_14>() as libc::c_ulong != 0 {
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
        perror_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"standard output\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
}
static mut option_help_msgid: [*const libc::c_char; 26] = [
    b"-o, --output=FILE            operate interactively, sending output to FILE\0"
        as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"-i, --ignore-case            consider upper- and lower-case to be the same\0"
        as *const u8 as *const libc::c_char,
    b"-E, --ignore-tab-expansion   ignore changes due to tab expansion\0" as *const u8
        as *const libc::c_char,
    b"-Z, --ignore-trailing-space  ignore white space at line end\0" as *const u8
        as *const libc::c_char,
    b"-b, --ignore-space-change    ignore changes in the amount of white space\0"
        as *const u8 as *const libc::c_char,
    b"-W, --ignore-all-space       ignore all white space\0" as *const u8
        as *const libc::c_char,
    b"-B, --ignore-blank-lines     ignore changes whose lines are all blank\0"
        as *const u8 as *const libc::c_char,
    b"-I, --ignore-matching-lines=RE  ignore changes all whose lines match RE\0"
        as *const u8 as *const libc::c_char,
    b"    --strip-trailing-cr      strip trailing carriage return on input\0"
        as *const u8 as *const libc::c_char,
    b"-a, --text                   treat all files as text\0" as *const u8
        as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"-w, --width=NUM              output at most NUM (default 130) print columns\0"
        as *const u8 as *const libc::c_char,
    b"-l, --left-column            output only the left column of common lines\0"
        as *const u8 as *const libc::c_char,
    b"-s, --suppress-common-lines  do not output common lines\0" as *const u8
        as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"-t, --expand-tabs            expand tabs to spaces in output\0" as *const u8
        as *const libc::c_char,
    b"    --tabsize=NUM            tab stops at every NUM (default 8) print columns\0"
        as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"-d, --minimal                try hard to find a smaller set of changes\0"
        as *const u8 as *const libc::c_char,
    b"-H, --speed-large-files      assume large files, many scattered small changes\0"
        as *const u8 as *const libc::c_char,
    b"    --diff-program=PROGRAM   use PROGRAM to compare files\0" as *const u8
        as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"    --help                   display this help and exit\0" as *const u8
        as *const libc::c_char,
    b"-v, --version                output version information and exit\0" as *const u8
        as *const libc::c_char,
    0 as *const libc::c_char,
];
unsafe extern "C" fn usage() {
    let mut p: *const *const libc::c_char = 0 as *const *const libc::c_char;
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"Usage: %s [OPTION]... FILE1 FILE2\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        program_name,
    );
    printf(
        b"%s\n\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"Side-by-side merge of differences between FILE1 and FILE2.\0" as *const u8
                as *const libc::c_char,
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
            b"Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    emit_bug_reporting_address();
}
unsafe extern "C" fn cleanup(mut signo: libc::c_int) {
    if (0 as libc::c_int) < diffpid {
        kill(diffpid, 13 as libc::c_int);
    }
    if !tmpname.is_null() {
        unlink(tmpname);
    }
}
unsafe extern "C" fn exiterr() {
    cleanup(0 as libc::c_int);
    untrapsig(0 as libc::c_int);
    checksigs();
    exit(2 as libc::c_int);
}
unsafe extern "C" fn fatal(mut msgid: *const libc::c_char) {
    error(
        0 as libc::c_int,
        0 as libc::c_int,
        b"%s\0" as *const u8 as *const libc::c_char,
        dcgettext(0 as *const libc::c_char, msgid, 5 as libc::c_int),
    );
    exiterr();
}
unsafe extern "C" fn perror_fatal(mut msg: *const libc::c_char) {
    let mut e: libc::c_int = *__errno_location();
    checksigs();
    error(0 as libc::c_int, e, b"%s\0" as *const u8 as *const libc::c_char, msg);
    exiterr();
}
unsafe extern "C" fn check_child_status(
    mut werrno: libc::c_int,
    mut wstatus: libc::c_int,
    mut max_ok_status: libc::c_int,
    mut subsidiary_program: *const libc::c_char,
) {
    let mut status: libc::c_int = if werrno == 0
        && wstatus & 0x7f as libc::c_int == 0 as libc::c_int
    {
        (wstatus & 0xff00 as libc::c_int) >> 8 as libc::c_int
    } else {
        2147483647 as libc::c_int
    };
    if max_ok_status < status {
        error(
            0 as libc::c_int,
            werrno,
            dcgettext(
                0 as *const libc::c_char,
                if status == 126 as libc::c_int {
                    b"subsidiary program '%s' could not be invoked\0" as *const u8
                        as *const libc::c_char
                } else if status == 127 as libc::c_int {
                    b"subsidiary program '%s' not found\0" as *const u8
                        as *const libc::c_char
                } else if status == 2147483647 as libc::c_int {
                    b"subsidiary program '%s' failed\0" as *const u8
                        as *const libc::c_char
                } else {
                    b"subsidiary program '%s' failed (exit status %d)\0" as *const u8
                        as *const libc::c_char
                },
                5 as libc::c_int,
            ),
            subsidiary_program,
            status,
        );
        exiterr();
    }
}
unsafe extern "C" fn ck_fopen(
    mut fname: *const libc::c_char,
    mut type_0: *const libc::c_char,
) -> *mut FILE {
    let mut r: *mut FILE = rpl_fopen(fname, type_0);
    if r.is_null() {
        perror_fatal(fname);
    }
    return r;
}
unsafe extern "C" fn ck_fclose(mut f: *mut FILE) {
    if fclose(f) != 0 {
        perror_fatal(b"fclose\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn ck_fread(
    mut buf: *mut libc::c_char,
    mut size: size_t,
    mut f: *mut FILE,
) -> size_t {
    let mut r: size_t = if 0 != 0 && 0 != 0
        && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong).wrapping_mul(size)
            <= 8 as libc::c_int as libc::c_ulong
        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
    {
        {
            let mut __ptr: *mut libc::c_char = buf;
            let mut __stream: *mut FILE = f;
            let mut __cnt: size_t = 0;
            __cnt = (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(size);
            while __cnt > 0 as libc::c_int as libc::c_ulong {
                let mut __c: libc::c_int = getc_unlocked(__stream);
                if __c == -(1 as libc::c_int) {
                    break;
                }
                let fresh4 = __ptr;
                __ptr = __ptr.offset(1);
                *fresh4 = __c as libc::c_char;
                __cnt = __cnt.wrapping_sub(1);
                __cnt;
            }
            (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(size)
                .wrapping_sub(__cnt)
                .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        }
    } else if 0 != 0
        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
            == 0 as libc::c_int as libc::c_ulong
        || 0 != 0 && size == 0 as libc::c_int as libc::c_ulong
    {
        0 as libc::c_int as size_t
    } else {
        fread_unlocked(
            buf as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
            size,
            f,
        )
    };
    if r == 0 as libc::c_int as libc::c_ulong && ferror_unlocked(f) != 0 {
        perror_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"read failed\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    return r;
}
unsafe extern "C" fn ck_fwrite(
    mut buf: *const libc::c_char,
    mut size: size_t,
    mut f: *mut FILE,
) {
    if (if 0 != 0 && 0 != 0
        && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong).wrapping_mul(size)
            <= 8 as libc::c_int as libc::c_ulong
        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
    {
        {
            let mut __ptr: *const libc::c_char = buf;
            let mut __stream: *mut FILE = f;
            let mut __cnt: size_t = 0;
            __cnt = (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(size);
            while __cnt > 0 as libc::c_int as libc::c_ulong {
                let fresh5 = __ptr;
                __ptr = __ptr.offset(1);
                if putc_unlocked(*fresh5 as libc::c_int, __stream) == -(1 as libc::c_int)
                {
                    break;
                }
                __cnt = __cnt.wrapping_sub(1);
                __cnt;
            }
            (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(size)
                .wrapping_sub(__cnt)
                .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        }
    } else {
        if 0 != 0
            && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                == 0 as libc::c_int as libc::c_ulong
            || 0 != 0 && size == 0 as libc::c_int as libc::c_ulong
        {
            0 as libc::c_int as size_t
        } else {
            fwrite_unlocked(
                buf as *const libc::c_void,
                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                size,
                f,
            )
        }
    }) != size
    {
        perror_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"write failed\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
}
unsafe extern "C" fn ck_fflush(mut f: *mut FILE) {
    if fflush_unlocked(f) != 0 as libc::c_int {
        perror_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"write failed\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
}
unsafe extern "C" fn expand_name(
    mut name: *mut libc::c_char,
    mut is_dir: bool,
    mut other_name: *const libc::c_char,
) -> *const libc::c_char {
    if strcmp(name, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        fatal(
            b"cannot interactively merge standard input\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !is_dir {
        return name
    } else {
        let mut base: *const libc::c_char = last_component(other_name);
        let mut namelen: size_t = strlen(name);
        let mut baselen: size_t = base_len(base);
        let mut insert_slash: bool = *last_component(name) as libc::c_int != 0
            && *name
                .offset(namelen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int != '/' as i32;
        let mut r: *mut libc::c_char = xmalloc(
            namelen
                .wrapping_add(insert_slash as libc::c_ulong)
                .wrapping_add(baselen)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        let mut p: *mut libc::c_char = stpcpy(r, name);
        *p = '/' as i32 as libc::c_char;
        p = mempcpy(
            p.offset(insert_slash as libc::c_int as isize) as *mut libc::c_void,
            base as *const libc::c_void,
            baselen,
        ) as *mut libc::c_char;
        *p = '\0' as i32 as libc::c_char;
        return r;
    };
}
unsafe extern "C" fn lf_init(mut lf: *mut line_filter, mut infile: *mut FILE) {
    (*lf).infile = infile;
    (*lf)
        .buflim = xmalloc((SDIFF_BUFSIZE as libc::c_int + 1 as libc::c_int) as size_t)
        as *mut libc::c_char;
    (*lf).buffer = (*lf).buflim;
    (*lf).bufpos = (*lf).buffer;
    *((*lf).buflim).offset(0 as libc::c_int as isize) = '\n' as i32 as libc::c_char;
}
unsafe extern "C" fn lf_refill(mut lf: *mut line_filter) -> size_t {
    let mut s: size_t = ck_fread(
        (*lf).buffer,
        SDIFF_BUFSIZE as libc::c_int as size_t,
        (*lf).infile,
    );
    (*lf).bufpos = (*lf).buffer;
    (*lf).buflim = ((*lf).buffer).offset(s as isize);
    *((*lf).buflim).offset(0 as libc::c_int as isize) = '\n' as i32 as libc::c_char;
    checksigs();
    return s;
}
unsafe extern "C" fn lf_copy(
    mut lf: *mut line_filter,
    mut lines: lin,
    mut outfile: *mut FILE,
) {
    let mut start: *mut libc::c_char = (*lf).bufpos;
    while lines != 0 {
        (*lf)
            .bufpos = rawmemchr((*lf).bufpos as *const libc::c_void, '\n' as i32)
            as *mut libc::c_char;
        if (*lf).bufpos == (*lf).buflim {
            ck_fwrite(
                start,
                ((*lf).buflim).offset_from(start) as libc::c_long as size_t,
                outfile,
            );
            if lf_refill(lf) == 0 {
                return;
            }
            start = (*lf).bufpos;
        } else {
            lines -= 1;
            lines;
            (*lf).bufpos = ((*lf).bufpos).offset(1);
            (*lf).bufpos;
        }
    }
    ck_fwrite(
        start,
        ((*lf).bufpos).offset_from(start) as libc::c_long as size_t,
        outfile,
    );
}
unsafe extern "C" fn lf_skip(mut lf: *mut line_filter, mut lines: lin) {
    while lines != 0 {
        (*lf)
            .bufpos = rawmemchr((*lf).bufpos as *const libc::c_void, '\n' as i32)
            as *mut libc::c_char;
        if (*lf).bufpos == (*lf).buflim {
            if lf_refill(lf) == 0 {
                break;
            }
        } else {
            lines -= 1;
            lines;
            (*lf).bufpos = ((*lf).bufpos).offset(1);
            (*lf).bufpos;
        }
    }
}
unsafe extern "C" fn lf_snarf(
    mut lf: *mut line_filter,
    mut buffer: *mut libc::c_char,
    mut bufsize: size_t,
) -> libc::c_int {
    loop {
        let mut start: *mut libc::c_char = (*lf).bufpos;
        let mut next: *mut libc::c_char = rawmemchr(
            start as *const libc::c_void,
            '\n' as i32,
        ) as *mut libc::c_char;
        let mut s: size_t = next.offset_from(start) as libc::c_long as size_t;
        if bufsize <= s {
            return 0 as libc::c_int;
        }
        buffer = mempcpy(buffer as *mut libc::c_void, start as *const libc::c_void, s)
            as *mut libc::c_char;
        bufsize = (bufsize as libc::c_ulong).wrapping_sub(s) as size_t as size_t;
        if next < (*lf).buflim {
            *buffer = 0 as libc::c_int as libc::c_char;
            (*lf).bufpos = next.offset(1 as libc::c_int as isize);
            return 1 as libc::c_int;
        }
        if lf_refill(lf) == 0 {
            return if s != 0 { 0 as libc::c_int } else { -(1 as libc::c_int) };
        }
    };
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut opt: libc::c_int = 0;
    let mut prog: *const libc::c_char = 0 as *const libc::c_char;
    ::std::ptr::write_volatile(&mut exit_failure as *mut libc::c_int, 2 as libc::c_int);
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"diffutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"diffutils\0" as *const u8 as *const libc::c_char);
    c_stack_action(Some(cleanup as unsafe extern "C" fn(libc::c_int) -> ()));
    xstdopen();
    prog = getenv(b"EDITOR\0" as *const u8 as *const libc::c_char);
    if !prog.is_null() {
        editor_program = prog;
    }
    diffarg(b"diff\0" as *const u8 as *const libc::c_char);
    loop {
        opt = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            b"abBdEHiI:lo:stvw:WZ\0" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(opt != -(1 as libc::c_int)) {
            break;
        }
        match opt {
            97 => {
                diffarg(b"-a\0" as *const u8 as *const libc::c_char);
            }
            98 => {
                diffarg(b"-b\0" as *const u8 as *const libc::c_char);
            }
            66 => {
                diffarg(b"-B\0" as *const u8 as *const libc::c_char);
            }
            100 => {
                diffarg(b"-d\0" as *const u8 as *const libc::c_char);
            }
            69 => {
                diffarg(b"-E\0" as *const u8 as *const libc::c_char);
            }
            72 => {
                diffarg(b"-H\0" as *const u8 as *const libc::c_char);
            }
            105 => {
                diffarg(b"-i\0" as *const u8 as *const libc::c_char);
            }
            73 => {
                diffarg(b"-I\0" as *const u8 as *const libc::c_char);
                diffarg(optarg);
            }
            108 => {
                diffarg(b"--left-column\0" as *const u8 as *const libc::c_char);
            }
            111 => {
                output = optarg;
            }
            115 => {
                suppress_common_lines = 1 as libc::c_int != 0;
            }
            116 => {
                diffarg(b"-t\0" as *const u8 as *const libc::c_char);
            }
            118 => {
                version_etc(
                    stdout,
                    PROGRAM_NAME.as_ptr(),
                    b"GNU diffutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    proper_name(b"Thomas Lord\0" as *const u8 as *const libc::c_char),
                    0 as *mut libc::c_void,
                );
                check_stdout();
                return 0 as libc::c_int;
            }
            119 => {
                diffarg(b"-W\0" as *const u8 as *const libc::c_char);
                diffarg(optarg);
            }
            87 => {
                diffarg(b"-w\0" as *const u8 as *const libc::c_char);
            }
            90 => {
                diffarg(b"-Z\0" as *const u8 as *const libc::c_char);
            }
            128 => {
                let ref mut fresh6 = *diffargv.offset(0 as libc::c_int as isize);
                *fresh6 = optarg;
            }
            129 => {
                usage();
                check_stdout();
                return 0 as libc::c_int;
            }
            130 => {
                diffarg(b"--strip-trailing-cr\0" as *const u8 as *const libc::c_char);
            }
            131 => {
                diffarg(b"--tabsize\0" as *const u8 as *const libc::c_char);
                diffarg(optarg);
            }
            _ => {
                try_help(0 as *const libc::c_char, 0 as *const libc::c_char);
            }
        }
    }
    if argc - optind != 2 as libc::c_int {
        if argc - optind < 2 as libc::c_int {
            try_help(
                b"missing operand after '%s'\0" as *const u8 as *const libc::c_char,
                *argv.offset((argc - 1 as libc::c_int) as isize),
            );
        } else {
            try_help(
                b"extra operand '%s'\0" as *const u8 as *const libc::c_char,
                *argv.offset((optind + 2 as libc::c_int) as isize),
            );
        }
    }
    if output.is_null() {
        if suppress_common_lines {
            diffarg(b"--suppress-common-lines\0" as *const u8 as *const libc::c_char);
        }
        diffarg(b"-y\0" as *const u8 as *const libc::c_char);
        diffarg(b"--\0" as *const u8 as *const libc::c_char);
        diffarg(*argv.offset(optind as isize));
        diffarg(*argv.offset((optind + 1 as libc::c_int) as isize));
        diffarg(0 as *const libc::c_char);
        execvp(
            *diffargv.offset(0 as libc::c_int as isize),
            diffargv as *mut *mut libc::c_char as *const *mut libc::c_char,
        );
        perror_fatal(*diffargv.offset(0 as libc::c_int as isize));
    } else {
        let mut lname: *const libc::c_char = 0 as *const libc::c_char;
        let mut rname: *const libc::c_char = 0 as *const libc::c_char;
        let mut left: *mut FILE = 0 as *mut FILE;
        let mut right: *mut FILE = 0 as *mut FILE;
        let mut out: *mut FILE = 0 as *mut FILE;
        let mut diffout: *mut FILE = 0 as *mut FILE;
        let mut interact_ok: bool = false;
        let mut lfilt: line_filter = line_filter {
            infile: 0 as *mut FILE,
            bufpos: 0 as *mut libc::c_char,
            buffer: 0 as *mut libc::c_char,
            buflim: 0 as *mut libc::c_char,
        };
        let mut rfilt: line_filter = line_filter {
            infile: 0 as *mut FILE,
            bufpos: 0 as *mut libc::c_char,
            buffer: 0 as *mut libc::c_char,
            buflim: 0 as *mut libc::c_char,
        };
        let mut diff_filt: line_filter = line_filter {
            infile: 0 as *mut FILE,
            bufpos: 0 as *mut libc::c_char,
            buffer: 0 as *mut libc::c_char,
            buflim: 0 as *mut libc::c_char,
        };
        let mut leftdir: bool = diraccess(*argv.offset(optind as isize));
        let mut rightdir: bool = diraccess(
            *argv.offset((optind + 1 as libc::c_int) as isize),
        );
        if leftdir as libc::c_int & rightdir as libc::c_int != 0 {
            fatal(
                b"both files to be compared are directories\0" as *const u8
                    as *const libc::c_char,
            );
        }
        lname = expand_name(
            *argv.offset(optind as isize),
            leftdir,
            *argv.offset((optind + 1 as libc::c_int) as isize),
        );
        left = ck_fopen(lname, b"r\0" as *const u8 as *const libc::c_char);
        rname = expand_name(
            *argv.offset((optind + 1 as libc::c_int) as isize),
            rightdir,
            *argv.offset(optind as isize),
        );
        right = ck_fopen(rname, b"r\0" as *const u8 as *const libc::c_char);
        out = ck_fopen(output, b"w\0" as *const u8 as *const libc::c_char);
        diffarg(b"--sdiff-merge-assist\0" as *const u8 as *const libc::c_char);
        diffarg(b"--\0" as *const u8 as *const libc::c_char);
        diffarg(*argv.offset(optind as isize));
        diffarg(*argv.offset((optind + 1 as libc::c_int) as isize));
        diffarg(0 as *const libc::c_char);
        trapsigs();
        let mut diff_fds: [libc::c_int; 2] = [0; 2];
        if pipe(diff_fds.as_mut_ptr()) != 0 as libc::c_int {
            perror_fatal(b"pipe\0" as *const u8 as *const libc::c_char);
        }
        ::std::ptr::write_volatile(&mut diffpid as *mut pid_t, fork());
        if diffpid < 0 as libc::c_int {
            perror_fatal(b"fork\0" as *const u8 as *const libc::c_char);
        }
        if diffpid == 0 {
            if initial_action[handler_index_of_SIGINT as libc::c_int as usize]
                .__sigaction_handler
                .sa_handler
                != ::std::mem::transmute::<
                    libc::intptr_t,
                    __sighandler_t,
                >(1 as libc::c_int as libc::intptr_t)
            {
                signal_handler(
                    2 as libc::c_int,
                    ::std::mem::transmute::<
                        libc::intptr_t,
                        __sighandler_t,
                    >(1 as libc::c_int as libc::intptr_t),
                );
            }
            signal_handler(13 as libc::c_int, None);
            close(diff_fds[0 as libc::c_int as usize]);
            if diff_fds[1 as libc::c_int as usize] != 1 as libc::c_int {
                dup2(diff_fds[1 as libc::c_int as usize], 1 as libc::c_int);
                close(diff_fds[1 as libc::c_int as usize]);
            }
            execvp(
                *diffargv.offset(0 as libc::c_int as isize),
                diffargv as *mut *mut libc::c_char as *const *mut libc::c_char,
            );
            _exit(
                if *__errno_location() == 2 as libc::c_int {
                    127 as libc::c_int
                } else {
                    126 as libc::c_int
                },
            );
        }
        close(diff_fds[1 as libc::c_int as usize]);
        diffout = fdopen(
            diff_fds[0 as libc::c_int as usize],
            b"r\0" as *const u8 as *const libc::c_char,
        );
        if diffout.is_null() {
            perror_fatal(b"fdopen\0" as *const u8 as *const libc::c_char);
        }
        lf_init(&mut diff_filt, diffout);
        lf_init(&mut lfilt, left);
        lf_init(&mut rfilt, right);
        interact_ok = interact(
            &mut diff_filt,
            &mut lfilt,
            lname,
            &mut rfilt,
            rname,
            out,
        );
        ck_fclose(left);
        ck_fclose(right);
        ck_fclose(out);
        let mut wstatus: libc::c_int = 0;
        let mut werrno: libc::c_int = 0 as libc::c_int;
        ck_fclose(diffout);
        while waitpid(diffpid, &mut wstatus, 0 as libc::c_int) < 0 as libc::c_int {
            if *__errno_location() == 4 as libc::c_int {
                checksigs();
            } else {
                perror_fatal(b"waitpid\0" as *const u8 as *const libc::c_char);
            }
        }
        ::std::ptr::write_volatile(&mut diffpid as *mut pid_t, 0 as libc::c_int);
        if !tmpname.is_null() {
            unlink(tmpname);
            ::std::ptr::write_volatile(
                &mut tmpname as *mut *mut libc::c_char,
                0 as *mut libc::c_char,
            );
        }
        if !interact_ok {
            exiterr();
        }
        check_child_status(
            werrno,
            wstatus,
            1 as libc::c_int,
            *diffargv.offset(0 as libc::c_int as isize),
        );
        untrapsig(0 as libc::c_int);
        checksigs();
        exit((wstatus & 0xff00 as libc::c_int) >> 8 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn diffarg(mut a: *const libc::c_char) {
    static mut diffargs: size_t = 0;
    static mut diffarglim: size_t = 0;
    if diffargs == diffarglim {
        if diffarglim == 0 {
            diffarglim = 16 as libc::c_int as size_t;
        } else if (9223372036854775807 as libc::c_long as libc::c_ulong)
            .wrapping_div(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                    ),
            ) <= diffarglim
        {
            xalloc_die();
        } else {
            diffarglim = (diffarglim as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
        diffargv = xrealloc(
            diffargv as *mut libc::c_void,
            diffarglim
                .wrapping_mul(
                    ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *const libc::c_char;
    }
    let fresh7 = diffargs;
    diffargs = diffargs.wrapping_add(1);
    let ref mut fresh8 = *diffargv.offset(fresh7 as isize);
    *fresh8 = a;
}
static mut ignore_SIGINT: bool = false;
static mut signal_received: libc::c_int = 0;
static mut sigs_trapped: bool = false;
unsafe extern "C" fn catchsig(mut s: libc::c_int) {
    if !(s == 2 as libc::c_int && ignore_SIGINT as libc::c_int != 0) {
        ::std::ptr::write_volatile(&mut signal_received as *mut libc::c_int, s);
    }
}
static mut catchaction: sigaction = sigaction {
    __sigaction_handler: C2RustUnnamed_9 {
        sa_handler: None,
    },
    sa_mask: __sigset_t { __val: [0; 16] },
    sa_flags: 0,
    sa_restorer: None,
};
unsafe extern "C" fn signal_handler(
    mut sig: libc::c_int,
    mut handler: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
) {
    catchaction.__sigaction_handler.sa_handler = handler;
    sigaction(sig, &mut catchaction, 0 as *mut sigaction);
}
unsafe extern "C" fn trapsigs() {
    let mut i: libc::c_int = 0;
    catchaction.sa_flags = 0x10000000 as libc::c_int;
    sigemptyset(&mut catchaction.sa_mask);
    i = 0 as libc::c_int;
    while i < NUM_SIGS as libc::c_int {
        sigaddset(&mut catchaction.sa_mask, sigs[i as usize]);
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < NUM_SIGS as libc::c_int {
        sigaction(
            sigs[i as usize],
            0 as *const sigaction,
            &mut *initial_action.as_mut_ptr().offset(i as isize),
        );
        if initial_action[i as usize].__sigaction_handler.sa_handler
            != ::std::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t)
        {
            signal_handler(
                sigs[i as usize],
                Some(catchsig as unsafe extern "C" fn(libc::c_int) -> ()),
            );
        }
        i += 1;
        i;
    }
    signal(17 as libc::c_int, None);
    sigs_trapped = 1 as libc::c_int != 0;
}
unsafe extern "C" fn untrapsig(mut s: libc::c_int) {
    let mut i: libc::c_int = 0;
    if sigs_trapped {
        i = 0 as libc::c_int;
        while i < NUM_SIGS as libc::c_int {
            if (s == 0 || sigs[i as usize] == s)
                && initial_action[i as usize].__sigaction_handler.sa_handler
                    != ::std::mem::transmute::<
                        libc::intptr_t,
                        __sighandler_t,
                    >(1 as libc::c_int as libc::intptr_t)
            {
                sigaction(
                    sigs[i as usize],
                    &mut *initial_action.as_mut_ptr().offset(i as isize),
                    0 as *mut sigaction,
                );
            }
            i += 1;
            i;
        }
    }
}
unsafe extern "C" fn checksigs() {
    let mut s: libc::c_int = signal_received;
    if s != 0 {
        cleanup(0 as libc::c_int);
        untrapsig(s);
        raise(s);
        exit(2 as libc::c_int);
    }
}
unsafe extern "C" fn give_help() {
    fprintf(
        stderr,
        b"%s\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"ed:\tEdit then use both versions, each decorated with a header.\neb:\tEdit then use both versions.\nel or e1:\tEdit then use the left version.\ner or e2:\tEdit then use the right version.\ne:\tDiscard both versions then edit a new one.\nl or 1:\tUse the left version.\nr or 2:\tUse the right version.\ns:\tSilently include common lines.\nv:\tVerbosely include common lines.\nq:\tQuit.\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
}
unsafe extern "C" fn skip_white() -> libc::c_int {
    let mut c: libc::c_int = 0;
    loop {
        c = getchar_unlocked();
        if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
            || c == '\n' as i32
        {
            break;
        }
        checksigs();
    }
    if ferror_unlocked(stdin) != 0 {
        perror_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"read failed\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    return c;
}
unsafe extern "C" fn flush_line() {
    let mut c: libc::c_int = 0;
    loop {
        c = getchar_unlocked();
        if !(c != '\n' as i32 && c != -(1 as libc::c_int)) {
            break;
        }
    }
    if ferror_unlocked(stdin) != 0 {
        perror_fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"read failed\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
}
unsafe extern "C" fn edit(
    mut left: *mut line_filter,
    mut lname: *const libc::c_char,
    mut lline: lin,
    mut llen: lin,
    mut right: *mut line_filter,
    mut rname: *const libc::c_char,
    mut rline: lin,
    mut rlen: lin,
    mut outfile: *mut FILE,
) -> bool {
    loop {
        let mut cmd0: libc::c_int = 0;
        let mut cmd1: libc::c_int = 0;
        let mut gotcmd: bool = 0 as libc::c_int != 0;
        let mut current_block_19: u64;
        while !gotcmd {
            if putchar_unlocked('%' as i32) != '%' as i32 {
                perror_fatal(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"write failed\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            ck_fflush(stdout);
            cmd0 = skip_white();
            match cmd0 {
                49 | 50 | 108 | 114 | 115 | 118 | 113 => {
                    if skip_white() != '\n' as i32 {
                        give_help();
                        flush_line();
                        continue;
                    } else {
                        gotcmd = 1 as libc::c_int != 0;
                        continue;
                    }
                }
                101 => {
                    cmd1 = skip_white();
                    match cmd1 {
                        49 | 50 | 98 | 100 | 108 | 114 => {
                            if skip_white() != '\n' as i32 {
                                give_help();
                                flush_line();
                                continue;
                            } else {
                                gotcmd = 1 as libc::c_int != 0;
                                continue;
                            }
                        }
                        10 => {
                            gotcmd = 1 as libc::c_int != 0;
                            continue;
                        }
                        _ => {
                            give_help();
                            flush_line();
                            continue;
                        }
                    }
                }
                -1 => {
                    if feof_unlocked(stdin) != 0 {
                        gotcmd = 1 as libc::c_int != 0;
                        cmd0 = 'q' as i32;
                        continue;
                    } else {
                        current_block_19 = 3186595734287750477;
                    }
                }
                10 => {
                    current_block_19 = 9893100268966357831;
                }
                _ => {
                    current_block_19 = 3186595734287750477;
                }
            }
            match current_block_19 {
                3186595734287750477 => {
                    flush_line();
                }
                _ => {}
            }
            give_help();
        }
        match cmd0 {
            49 | 108 => {
                lf_copy(left, llen, outfile);
                lf_skip(right, rlen);
                return 1 as libc::c_int != 0;
            }
            50 | 114 => {
                lf_copy(right, rlen, outfile);
                lf_skip(left, llen);
                return 1 as libc::c_int != 0;
            }
            115 => {
                suppress_common_lines = 1 as libc::c_int != 0;
            }
            118 => {
                suppress_common_lines = 0 as libc::c_int != 0;
            }
            113 => return 0 as libc::c_int != 0,
            101 => {
                let mut fd: libc::c_int = 0;
                if !tmpname.is_null() {
                    tmp = rpl_fopen(tmpname, b"w\0" as *const u8 as *const libc::c_char);
                } else {
                    fd = temporary_file();
                    if fd < 0 as libc::c_int {
                        perror_fatal(b"mkstemp\0" as *const u8 as *const libc::c_char);
                    }
                    tmp = fdopen(fd, b"w\0" as *const u8 as *const libc::c_char);
                }
                if tmp.is_null() {
                    perror_fatal(tmpname);
                }
                let mut current_block_45: u64;
                match cmd1 {
                    100 => {
                        if llen != 0 {
                            if llen == 1 as libc::c_int as libc::c_long {
                                fprintf(
                                    tmp,
                                    b"--- %s %td\n\0" as *const u8 as *const libc::c_char,
                                    lname,
                                    lline,
                                );
                            } else {
                                fprintf(
                                    tmp,
                                    b"--- %s %td,%td\n\0" as *const u8 as *const libc::c_char,
                                    lname,
                                    lline,
                                    lline + llen - 1 as libc::c_int as libc::c_long,
                                );
                            }
                        }
                        current_block_45 = 2660319668005976262;
                    }
                    49 | 98 | 108 => {
                        current_block_45 = 2660319668005976262;
                    }
                    _ => {
                        lf_skip(left, llen);
                        current_block_45 = 9859671972921157070;
                    }
                }
                match current_block_45 {
                    2660319668005976262 => {
                        lf_copy(left, llen, tmp);
                    }
                    _ => {}
                }
                let mut current_block_54: u64;
                match cmd1 {
                    100 => {
                        if rlen != 0 {
                            if rlen == 1 as libc::c_int as libc::c_long {
                                fprintf(
                                    tmp,
                                    b"+++ %s %td\n\0" as *const u8 as *const libc::c_char,
                                    rname,
                                    rline,
                                );
                            } else {
                                fprintf(
                                    tmp,
                                    b"+++ %s %td,%td\n\0" as *const u8 as *const libc::c_char,
                                    rname,
                                    rline,
                                    rline + rlen - 1 as libc::c_int as libc::c_long,
                                );
                            }
                        }
                        current_block_54 = 2338606485997896488;
                    }
                    50 | 98 | 114 => {
                        current_block_54 = 2338606485997896488;
                    }
                    _ => {
                        lf_skip(right, rlen);
                        current_block_54 = 6281126495347172768;
                    }
                }
                match current_block_54 {
                    2338606485997896488 => {
                        lf_copy(right, rlen, tmp);
                    }
                    _ => {}
                }
                ck_fclose(tmp);
                let mut wstatus: libc::c_int = 0;
                let mut werrno: libc::c_int = 0 as libc::c_int;
                let mut argv: [*const libc::c_char; 3] = [0 as *const libc::c_char; 3];
                ::std::ptr::write_volatile(
                    &mut ignore_SIGINT as *mut bool,
                    1 as libc::c_int != 0,
                );
                checksigs();
                argv[0 as libc::c_int as usize] = editor_program;
                argv[1 as libc::c_int as usize] = tmpname;
                argv[2 as libc::c_int as usize] = 0 as *const libc::c_char;
                let mut pid: pid_t = 0;
                pid = fork();
                if pid == 0 as libc::c_int {
                    execvp(
                        editor_program,
                        argv.as_mut_ptr() as *mut *mut libc::c_char
                            as *const *mut libc::c_char,
                    );
                    _exit(
                        if *__errno_location() == 2 as libc::c_int {
                            127 as libc::c_int
                        } else {
                            126 as libc::c_int
                        },
                    );
                }
                if pid < 0 as libc::c_int {
                    perror_fatal(b"fork\0" as *const u8 as *const libc::c_char);
                }
                while waitpid(pid, &mut wstatus, 0 as libc::c_int) < 0 as libc::c_int {
                    if *__errno_location() == 4 as libc::c_int {
                        checksigs();
                    } else {
                        perror_fatal(b"waitpid\0" as *const u8 as *const libc::c_char);
                    }
                }
                ::std::ptr::write_volatile(
                    &mut ignore_SIGINT as *mut bool,
                    0 as libc::c_int != 0,
                );
                check_child_status(werrno, wstatus, 0 as libc::c_int, editor_program);
                let mut buf: [libc::c_char; 65536] = [0; 65536];
                let mut size: size_t = 0;
                tmp = ck_fopen(tmpname, b"r\0" as *const u8 as *const libc::c_char);
                loop {
                    size = ck_fread(
                        buf.as_mut_ptr(),
                        SDIFF_BUFSIZE as libc::c_int as size_t,
                        tmp,
                    );
                    if !(size != 0 as libc::c_int as libc::c_ulong) {
                        break;
                    }
                    checksigs();
                    ck_fwrite(buf.as_mut_ptr(), size, outfile);
                }
                ck_fclose(tmp);
                return 1 as libc::c_int != 0;
            }
            _ => {
                give_help();
            }
        }
    };
}
unsafe extern "C" fn interact(
    mut diff: *mut line_filter,
    mut left: *mut line_filter,
    mut lname: *const libc::c_char,
    mut right: *mut line_filter,
    mut rname: *const libc::c_char,
    mut outfile: *mut FILE,
) -> bool {
    let mut lline: lin = 1 as libc::c_int as lin;
    let mut rline: lin = 1 as libc::c_int as lin;
    loop {
        let mut diff_help: [libc::c_char; 256] = [0; 256];
        let mut snarfed: libc::c_int = lf_snarf(
            diff,
            diff_help.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
        if snarfed <= 0 as libc::c_int {
            return snarfed != 0 as libc::c_int;
        }
        checksigs();
        if diff_help[0 as libc::c_int as usize] as libc::c_int == ' ' as i32 {
            puts(diff_help.as_mut_ptr().offset(1 as libc::c_int as isize));
        } else {
            let mut numend: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut val: intmax_t = 0;
            let mut llen: lin = 0;
            let mut rlen: lin = 0;
            let mut lenmax: lin = 0;
            *__errno_location() = 0 as libc::c_int;
            val = strtoimax(
                diff_help.as_mut_ptr().offset(1 as libc::c_int as isize),
                &mut numend,
                10 as libc::c_int,
            );
            if !(0 as libc::c_int as libc::c_long <= val
                && val <= 9223372036854775807 as libc::c_long)
                || *__errno_location() != 0 || *numend as libc::c_int != ',' as i32
            {
                fatal(diff_help.as_mut_ptr());
            }
            llen = val;
            val = strtoimax(
                numend.offset(1 as libc::c_int as isize),
                &mut numend,
                10 as libc::c_int,
            );
            if !(0 as libc::c_int as libc::c_long <= val
                && val <= 9223372036854775807 as libc::c_long)
                || *__errno_location() != 0 || *numend as libc::c_int != 0
            {
                fatal(diff_help.as_mut_ptr());
            }
            rlen = val;
            lenmax = if llen >= rlen { llen } else { rlen };
            match diff_help[0 as libc::c_int as usize] as libc::c_int {
                105 => {
                    if suppress_common_lines {
                        lf_skip(diff, lenmax);
                    } else {
                        lf_copy(diff, lenmax, stdout);
                    }
                    lf_copy(left, llen, outfile);
                    lf_skip(right, rlen);
                }
                99 => {
                    lf_copy(diff, lenmax, stdout);
                    if !edit(
                        left,
                        lname,
                        lline,
                        llen,
                        right,
                        rname,
                        rline,
                        rlen,
                        outfile,
                    ) {
                        return 0 as libc::c_int != 0;
                    }
                }
                _ => {
                    fatal(diff_help.as_mut_ptr());
                }
            }
            lline += llen;
            rline += rlen;
        }
    };
}
unsafe extern "C" fn diraccess(mut dir: *const libc::c_char) -> bool {
    let mut buf: stat = stat {
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
    return stat(dir, &mut buf) == 0 as libc::c_int
        && buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn temporary_file() -> libc::c_int {
    let mut tmpdir: *const libc::c_char = getenv(
        b"TMPDIR\0" as *const u8 as *const libc::c_char,
    );
    let mut dir: *const libc::c_char = if !tmpdir.is_null() {
        tmpdir
    } else {
        b"/tmp\0" as *const u8 as *const libc::c_char
    };
    let mut buf: *mut libc::c_char = xmalloc(
        (strlen(dir))
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(5 as libc::c_int as libc::c_ulong)
            .wrapping_add(6 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    strcpy(stpcpy(buf, dir), b"/sdiffXXXXXX\0" as *const u8 as *const libc::c_char);
    let mut fd: libc::c_int = mkstemp(buf);
    if fd < 0 as libc::c_int {
        rpl_free(buf as *mut libc::c_void);
    } else {
        ::std::ptr::write_volatile(&mut tmpname as *mut *mut libc::c_char, buf);
    }
    return fd;
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
