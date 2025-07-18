use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn abort() -> !;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn execv(
        __path: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn fork() -> __pid_t;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn raise(__sig: libc::c_int) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigismember(__set: *const sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
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
    fn rpl_free(ptr: *mut libc::c_void);
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __errno_location() -> *mut libc::c_int;
    fn rpl_re_search(
        __buffer: *mut re_pattern_buffer,
        __String: *const libc::c_char,
        __length: regoff_t,
        __start: regoff_t,
        __range: regoff_t,
        __regs: *mut re_registers,
    ) -> regoff_t;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    static mut output_style: output_style;
    static mut colors_style: colors_style;
    static mut ignore_white_space: DIFF_white_space;
    static mut ignore_blank_lines: bool;
    static mut ignore_case: bool;
    static mut ignore_regexp: re_pattern_buffer;
    static mut expand_tabs: bool;
    static mut tabsize: size_t;
    static mut initial_tab: bool;
    static mut suppress_blank_empty: bool;
    static mut paginate: bool;
    static mut sdiff_merge_assist: bool;
    static mut switch_string: *mut libc::c_char;
    static mut files: [file_data; 2];
    static mut outfile: *mut FILE;
    fn print_context_header(_: *mut file_data, _: *const *const libc::c_char, _: bool);
    static mut presume_output_tty: bool;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
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
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
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
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
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
pub type sig_atomic_t = __sig_atomic_t;
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
pub type __re_size_t = size_t;
pub type __re_long_size_t = size_t;
pub type reg_syntax_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut libc::c_char,
    pub translate: *mut libc::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regoff_t = ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub num_regs: __re_size_t,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
}
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
pub type changes = libc::c_uint;
pub const CHANGED: changes = 3;
pub const NEW: changes = 2;
pub const OLD: changes = 1;
pub const UNCHANGED: changes = 0;
pub type colors_style = libc::c_uint;
pub const ALWAYS: colors_style = 2;
pub const AUTO: colors_style = 1;
pub const NEVER: colors_style = 0;
pub type output_style = libc::c_uint;
pub const OUTPUT_SDIFF: output_style = 8;
pub const OUTPUT_IFDEF: output_style = 7;
pub const OUTPUT_RCS: output_style = 6;
pub const OUTPUT_FORWARD_ED: output_style = 5;
pub const OUTPUT_ED: output_style = 4;
pub const OUTPUT_UNIFIED: output_style = 3;
pub const OUTPUT_CONTEXT: output_style = 2;
pub const OUTPUT_NORMAL: output_style = 1;
pub const OUTPUT_UNSPECIFIED: output_style = 0;
pub type DIFF_white_space = libc::c_uint;
pub const IGNORE_ALL_SPACE: DIFF_white_space = 5;
pub const IGNORE_SPACE_CHANGE: DIFF_white_space = 4;
pub const IGNORE_TAB_EXPANSION_AND_TRAILING_SPACE: DIFF_white_space = 3;
pub const IGNORE_TRAILING_SPACE: DIFF_white_space = 2;
pub const IGNORE_TAB_EXPANSION: DIFF_white_space = 1;
pub const IGNORE_NO_WHITE_SPACE: DIFF_white_space = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct change {
    pub link: *mut change,
    pub inserted: lin,
    pub deleted: lin,
    pub line0: lin,
    pub line1: lin,
    pub ignore: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_data {
    pub desc: libc::c_int,
    pub name: *const libc::c_char,
    pub stat: stat,
    pub buffer: *mut size_t,
    pub bufsize: size_t,
    pub buffered: size_t,
    pub linbuf: *mut *const libc::c_char,
    pub linbuf_base: lin,
    pub buffered_lines: lin,
    pub valid_lines: lin,
    pub alloc_lines: lin,
    pub prefix_end: *const libc::c_char,
    pub prefix_lines: lin,
    pub suffix_begin: *const libc::c_char,
    pub equivs: *mut lin,
    pub undiscarded: *mut lin,
    pub realindexes: *mut lin,
    pub nondiscarded_lines: lin,
    pub changed: *mut libc::c_char,
    pub missing_newline: bool,
    pub eof: bool,
    pub equiv_max: lin,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msg {
    pub next: *mut msg,
    pub msgid: *const libc::c_char,
    pub argbytes: size_t,
    pub args: [libc::c_char; 0],
}
pub const nsigs: C2RustUnnamed_15 = 12;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct color_ext_type {
    pub ext: bin_str,
    pub seq: bin_str,
    pub next: *mut color_ext_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bin_str {
    pub len: size_t,
    pub string: *const libc::c_char,
}
pub const PS_FAIL: parse_state = 6;
pub type parse_state = libc::c_uint;
pub const PS_DONE: parse_state = 5;
pub const PS_4: parse_state = 4;
pub const PS_3: parse_state = 3;
pub const PS_2: parse_state = 2;
pub const PS_START: parse_state = 1;
pub const ST_ERROR: C2RustUnnamed_12 = 6;
pub type C2RustUnnamed_12 = libc::c_uint;
pub const ST_END: C2RustUnnamed_12 = 5;
pub const ST_CARET: C2RustUnnamed_12 = 4;
pub const ST_HEX: C2RustUnnamed_12 = 3;
pub const ST_OCTAL: C2RustUnnamed_12 = 2;
pub const ST_BACKSLASH: C2RustUnnamed_12 = 1;
pub const ST_GND: C2RustUnnamed_12 = 0;
pub type signal_handler = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type color_context = libc::c_uint;
pub const LINE_NUMBER_CONTEXT: color_context = 4;
pub const RESET_CONTEXT: color_context = 3;
pub const DELETE_CONTEXT: color_context = 2;
pub const ADD_CONTEXT: color_context = 1;
pub const HEADER_CONTEXT: color_context = 0;
pub const C_RIGHT: indicator_no = 1;
pub const C_RESET: indicator_no = 3;
pub const C_DELETE: indicator_no = 6;
pub const C_ADD: indicator_no = 5;
pub const C_LINE: indicator_no = 7;
pub const C_HEADER: indicator_no = 4;
pub const C_LEFT: indicator_no = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub _gl_dummy: libc::c_int,
}
pub type C2RustUnnamed_15 = libc::c_uint;
pub type indicator_no = libc::c_uint;
pub const C_END: indicator_no = 2;
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn vprintf(
    mut __fmt: *const libc::c_char,
    mut __arg: ::std::ffi::VaList,
) -> libc::c_int {
    return vfprintf(stdout, __fmt, __arg.as_va_list());
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
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x20 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
pub static mut pr_program: [libc::c_char; 12] = unsafe {
    *::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"/usr/bin/pr\0")
};
static mut msg_chain: *mut msg = 0 as *const msg as *mut msg;
static mut msg_chain_end: *mut *mut msg = unsafe {
    &msg_chain as *const *mut msg as *mut *mut msg
};
pub unsafe extern "C" fn perror_with_name(mut name: *const libc::c_char) {
    error(
        0 as libc::c_int,
        *__errno_location(),
        b"%s\0" as *const u8 as *const libc::c_char,
        name,
    );
}
pub unsafe extern "C" fn pfatal_with_name(mut name: *const libc::c_char) {
    let mut e: libc::c_int = *__errno_location();
    print_message_queue();
    if ::std::mem::size_of::<C2RustUnnamed_11>() as libc::c_ulong != 0 {
        error(2 as libc::c_int, e, b"%s\0" as *const u8 as *const libc::c_char, name);
        if 0 as libc::c_int != 0 {} else {
            unreachable!();
        };
    } else {
        error(2 as libc::c_int, e, b"%s\0" as *const u8 as *const libc::c_char, name);
        if 0 as libc::c_int != 0 {} else {
            unreachable!();
        };
    };
}
pub unsafe extern "C" fn fatal(mut msgid: *const libc::c_char) {
    print_message_queue();
    if ::std::mem::size_of::<C2RustUnnamed_13>() as libc::c_ulong != 0 {
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
pub unsafe extern "C" fn message(mut format_msgid: *const libc::c_char, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    if paginate {
        let mut argbytes: size_t = 0 as libc::c_int as size_t;
        let mut m: *const libc::c_char = format_msgid;
        while *m != 0 {
            if *m as libc::c_int == '%' as i32 {
                if *m.offset(1 as libc::c_int as isize) as libc::c_int == '%' as i32 {
                    m = m.offset(1);
                    m;
                } else {
                    argbytes = (argbytes as libc::c_ulong)
                        .wrapping_add(
                            (strlen(ap.arg::<*const libc::c_char>()))
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as size_t as size_t;
                }
            }
            m = m.offset(1);
            m;
        }
        let mut new: *mut msg = xmalloc(
            (24 as libc::c_ulong)
                .wrapping_add(::std::mem::align_of::<msg>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(argbytes)
                & !(::std::mem::align_of::<msg>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) as *mut msg;
        (*new).msgid = format_msgid;
        (*new).argbytes = argbytes;
        ap = args.clone();
        let mut p: *mut libc::c_char = ((*new).args).as_mut_ptr();
        let mut m_0: *const libc::c_char = format_msgid;
        while *m_0 != 0 {
            if *m_0 as libc::c_int == '%' as i32 {
                if *m_0.offset(1 as libc::c_int as isize) as libc::c_int == '%' as i32 {
                    m_0 = m_0.offset(1);
                    m_0;
                } else {
                    p = (stpcpy(p, ap.arg::<*const libc::c_char>()))
                        .offset(1 as libc::c_int as isize);
                }
            }
            m_0 = m_0.offset(1);
            m_0;
        }
        *msg_chain_end = new;
        (*new).next = 0 as *mut msg;
        msg_chain_end = &mut (*new).next;
    } else {
        if sdiff_merge_assist {
            putchar_unlocked(' ' as i32);
        }
        vprintf(
            dcgettext(0 as *const libc::c_char, format_msgid, 5 as libc::c_int),
            ap.as_va_list(),
        );
    };
}
pub unsafe extern "C" fn print_message_queue() {
    let mut m: *mut msg = msg_chain;
    while !m.is_null() {
        let mut p: *const libc::c_char = ((*m).args).as_mut_ptr();
        let mut plim: *const libc::c_char = p.offset((*m).argbytes as isize);
        let mut arg0: *const libc::c_char = p;
        p = p
            .offset(
                (if p < plim {
                    (strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                }) as isize,
            );
        let mut arg1: *const libc::c_char = p;
        p = p
            .offset(
                (if p < plim {
                    (strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                }) as isize,
            );
        let mut arg2: *const libc::c_char = p;
        p = p
            .offset(
                (if p < plim {
                    (strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                }) as isize,
            );
        let mut arg3: *const libc::c_char = p;
        p = p
            .offset(
                (if p < plim {
                    (strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                }) as isize,
            );
        printf(
            dcgettext(0 as *const libc::c_char, (*m).msgid, 5 as libc::c_int),
            arg0,
            arg1,
            arg2,
            arg3,
        );
        if p < plim {
            abort();
        }
        let mut next: *mut msg = (*m).next;
        rpl_free(m as *mut libc::c_void);
        m = next;
    }
}
unsafe extern "C" fn xsigaddset(mut set: *mut sigset_t, mut sig_0: libc::c_int) {
    if sigaddset(set, sig_0) != 0 as libc::c_int {
        pfatal_with_name(b"sigaddset\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn xsigismember(
    mut set: *const sigset_t,
    mut sig_0: libc::c_int,
) -> bool {
    let mut mem: libc::c_int = sigismember(set, sig_0);
    if mem < 0 as libc::c_int {
        pfatal_with_name(b"sigismember\0" as *const u8 as *const libc::c_char);
    }
    if mem <= 1 as libc::c_int {} else {
        unreachable!();
    };
    return mem != 0;
}
unsafe extern "C" fn xsignal(
    mut sig_0: libc::c_int,
    mut func: signal_handler,
) -> signal_handler {
    let mut h: signal_handler = signal(sig_0, func);
    if h
        == ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(-(1 as libc::c_int) as libc::intptr_t)
    {
        pfatal_with_name(b"signal\0" as *const u8 as *const libc::c_char);
    }
    return h;
}
unsafe extern "C" fn xsigprocmask(
    mut how: libc::c_int,
    mut set: *const sigset_t,
    mut oset: *mut sigset_t,
) {
    if sigprocmask(how, set, oset) != 0 as libc::c_int {
        pfatal_with_name(b"sigprocmask\0" as *const u8 as *const libc::c_char);
    }
}
static mut some_signals_caught: bool = false;
static mut caught_signals: sigset_t = sigset_t { __val: [0; 16] };
static mut interrupt_signal: sig_atomic_t = 0;
static mut stop_signal_count: sig_atomic_t = 0;
unsafe extern "C" fn sighandler(mut sig_0: libc::c_int) {
    if 1 as libc::c_int == 0 {
        signal(
            sig_0,
            ::std::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        );
    }
    if interrupt_signal == 0 {
        ::std::ptr::write_volatile(&mut interrupt_signal as *mut sig_atomic_t, sig_0);
    }
}
unsafe extern "C" fn stophandler(mut sig_0: libc::c_int) {
    if 1 as libc::c_int == 0 {
        signal(sig_0, Some(stophandler as unsafe extern "C" fn(libc::c_int) -> ()));
    }
    if interrupt_signal == 0 {
        ::std::ptr::write_volatile(
            &mut stop_signal_count as *mut sig_atomic_t,
            ::std::ptr::read_volatile::<
                sig_atomic_t,
            >(&stop_signal_count as *const sig_atomic_t) + 1,
        );
        ::std::ptr::read_volatile::<
            sig_atomic_t,
        >(&stop_signal_count as *const sig_atomic_t);
    }
}
unsafe extern "C" fn process_signals() {
    while interrupt_signal | stop_signal_count != 0 {
        set_color_context(RESET_CONTEXT);
        fflush_unlocked(stdout);
        let mut oldset: sigset_t = sigset_t { __val: [0; 16] };
        xsigprocmask(0 as libc::c_int, &mut caught_signals, &mut oldset);
        let mut stops: libc::c_int = stop_signal_count;
        let mut sig_0: libc::c_int = 0;
        if stops != 0 {
            ::std::ptr::write_volatile(
                &mut stop_signal_count as *mut sig_atomic_t,
                stops - 1 as libc::c_int,
            );
            sig_0 = 19 as libc::c_int;
        } else {
            sig_0 = interrupt_signal;
            xsignal(sig_0, None);
        }
        if raise(sig_0) != 0 as libc::c_int {
            pfatal_with_name(b"raise\0" as *const u8 as *const libc::c_char);
        }
        xsigprocmask(2 as libc::c_int, &mut oldset, 0 as *mut sigset_t);
    }
}
static mut sig: [libc::c_int; 12] = [
    20 as libc::c_int,
    14 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    13 as libc::c_int,
    3 as libc::c_int,
    15 as libc::c_int,
    29 as libc::c_int,
    27 as libc::c_int,
    26 as libc::c_int,
    24 as libc::c_int,
    25 as libc::c_int,
];
unsafe extern "C" fn is_tstp_index(mut j: libc::c_int) -> bool {
    return j == 0 as libc::c_int;
}
unsafe extern "C" fn install_signal_handlers() {
    if sigemptyset(&mut caught_signals) != 0 as libc::c_int {
        pfatal_with_name(b"sigemptyset\0" as *const u8 as *const libc::c_char);
    }
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < nsigs as libc::c_int {
        let mut actj: sigaction = sigaction {
            __sigaction_handler: C2RustUnnamed_9 {
                sa_handler: None,
            },
            sa_mask: sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };
        if sigaction(sig[j as usize], 0 as *const sigaction, &mut actj)
            == 0 as libc::c_int
            && actj.__sigaction_handler.sa_handler
                != ::std::mem::transmute::<
                    libc::intptr_t,
                    __sighandler_t,
                >(1 as libc::c_int as libc::intptr_t)
        {
            xsigaddset(&mut caught_signals, sig[j as usize]);
        }
        j += 1;
        j;
    }
    let mut act: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    act.sa_mask = caught_signals;
    act.sa_flags = 0x10000000 as libc::c_int;
    let mut j_0: libc::c_int = 0 as libc::c_int;
    while j_0 < nsigs as libc::c_int {
        if xsigismember(&mut caught_signals, sig[j_0 as usize]) {
            act
                .__sigaction_handler
                .sa_handler = if is_tstp_index(j_0) as libc::c_int != 0 {
                Some(stophandler as unsafe extern "C" fn(libc::c_int) -> ())
            } else {
                Some(sighandler as unsafe extern "C" fn(libc::c_int) -> ())
            };
            if sigaction(sig[j_0 as usize], &mut act, 0 as *mut sigaction)
                != 0 as libc::c_int
            {
                pfatal_with_name(b"sigaction\0" as *const u8 as *const libc::c_char);
            }
            some_signals_caught = 1 as libc::c_int != 0;
        }
        j_0 += 1;
        j_0;
    }
}
pub unsafe extern "C" fn cleanup_signal_handlers() {
    if some_signals_caught {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < nsigs as libc::c_int {
            if xsigismember(&mut caught_signals, sig[j as usize]) {
                xsignal(sig[j as usize], None);
            }
            j += 1;
            j;
        }
        process_signals();
    }
}
static mut current_name0: *const libc::c_char = 0 as *const libc::c_char;
static mut current_name1: *const libc::c_char = 0 as *const libc::c_char;
static mut currently_recursive: bool = false;
static mut colors_enabled: bool = false;
static mut color_ext_list: *mut color_ext_type = 0 as *const color_ext_type
    as *mut color_ext_type;
unsafe extern "C" fn get_funky_string(
    mut dest: *mut *mut libc::c_char,
    mut src: *mut *const libc::c_char,
    mut equals_end: bool,
    mut output_count: *mut size_t,
) -> bool {
    let mut num: libc::c_char = 0;
    let mut count: size_t = 0;
    let mut state: C2RustUnnamed_12 = ST_GND;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    p = *src;
    q = *dest;
    count = 0 as libc::c_int as size_t;
    num = 0 as libc::c_int as libc::c_char;
    state = ST_GND;
    while (state as libc::c_uint) < ST_END as libc::c_int as libc::c_uint {
        match state as libc::c_uint {
            0 => {
                let mut current_block_13: u64;
                match *p as libc::c_int {
                    58 | 0 => {
                        state = ST_END;
                        current_block_13 = 8457315219000651999;
                    }
                    92 => {
                        state = ST_BACKSLASH;
                        p = p.offset(1);
                        p;
                        current_block_13 = 8457315219000651999;
                    }
                    94 => {
                        state = ST_CARET;
                        p = p.offset(1);
                        p;
                        current_block_13 = 8457315219000651999;
                    }
                    61 => {
                        if equals_end {
                            state = ST_END;
                            current_block_13 = 8457315219000651999;
                        } else {
                            current_block_13 = 16601507626245803019;
                        }
                    }
                    _ => {
                        current_block_13 = 16601507626245803019;
                    }
                }
                match current_block_13 {
                    16601507626245803019 => {
                        let fresh2 = p;
                        p = p.offset(1);
                        let fresh3 = q;
                        q = q.offset(1);
                        *fresh3 = *fresh2;
                        count = count.wrapping_add(1);
                        count;
                    }
                    _ => {}
                }
            }
            1 => {
                match *p as libc::c_int {
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
                        state = ST_OCTAL;
                        num = (*p as libc::c_int - '0' as i32) as libc::c_char;
                    }
                    120 | 88 => {
                        state = ST_HEX;
                        num = 0 as libc::c_int as libc::c_char;
                    }
                    97 => {
                        num = '\u{7}' as i32 as libc::c_char;
                    }
                    98 => {
                        num = '\u{8}' as i32 as libc::c_char;
                    }
                    101 => {
                        num = 27 as libc::c_int as libc::c_char;
                    }
                    102 => {
                        num = '\u{c}' as i32 as libc::c_char;
                    }
                    110 => {
                        num = '\n' as i32 as libc::c_char;
                    }
                    114 => {
                        num = '\r' as i32 as libc::c_char;
                    }
                    116 => {
                        num = '\t' as i32 as libc::c_char;
                    }
                    118 => {
                        num = '\u{b}' as i32 as libc::c_char;
                    }
                    63 => {
                        num = 127 as libc::c_int as libc::c_char;
                    }
                    95 => {
                        num = ' ' as i32 as libc::c_char;
                    }
                    0 => {
                        state = ST_ERROR;
                    }
                    _ => {
                        num = *p;
                    }
                }
                if state as libc::c_uint == ST_BACKSLASH as libc::c_int as libc::c_uint {
                    let fresh4 = q;
                    q = q.offset(1);
                    *fresh4 = num;
                    count = count.wrapping_add(1);
                    count;
                    state = ST_GND;
                }
                p = p.offset(1);
                p;
            }
            2 => {
                if (*p as libc::c_int) < '0' as i32 || *p as libc::c_int > '7' as i32 {
                    let fresh5 = q;
                    q = q.offset(1);
                    *fresh5 = num;
                    count = count.wrapping_add(1);
                    count;
                    state = ST_GND;
                } else {
                    let fresh6 = p;
                    p = p.offset(1);
                    num = (((num as libc::c_int) << 3 as libc::c_int)
                        + (*fresh6 as libc::c_int - '0' as i32)) as libc::c_char;
                }
            }
            3 => {
                match *p as libc::c_int {
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                        let fresh7 = p;
                        p = p.offset(1);
                        num = (((num as libc::c_int) << 4 as libc::c_int)
                            + (*fresh7 as libc::c_int - '0' as i32)) as libc::c_char;
                    }
                    97 | 98 | 99 | 100 | 101 | 102 => {
                        let fresh8 = p;
                        p = p.offset(1);
                        num = (((num as libc::c_int) << 4 as libc::c_int)
                            + (*fresh8 as libc::c_int - 'a' as i32) + 10 as libc::c_int)
                            as libc::c_char;
                    }
                    65 | 66 | 67 | 68 | 69 | 70 => {
                        let fresh9 = p;
                        p = p.offset(1);
                        num = (((num as libc::c_int) << 4 as libc::c_int)
                            + (*fresh9 as libc::c_int - 'A' as i32) + 10 as libc::c_int)
                            as libc::c_char;
                    }
                    _ => {
                        let fresh10 = q;
                        q = q.offset(1);
                        *fresh10 = num;
                        count = count.wrapping_add(1);
                        count;
                        state = ST_GND;
                    }
                }
            }
            4 => {
                state = ST_GND;
                if *p as libc::c_int >= '@' as i32 && *p as libc::c_int <= '~' as i32 {
                    let fresh11 = p;
                    p = p.offset(1);
                    let fresh12 = q;
                    q = q.offset(1);
                    *fresh12 = (*fresh11 as libc::c_int & 0o37 as libc::c_int)
                        as libc::c_char;
                    count = count.wrapping_add(1);
                    count;
                } else if *p as libc::c_int == '?' as i32 {
                    let fresh13 = q;
                    q = q.offset(1);
                    *fresh13 = 127 as libc::c_int as libc::c_char;
                    count = count.wrapping_add(1);
                    count;
                } else {
                    state = ST_ERROR;
                }
            }
            _ => {
                abort();
            }
        }
    }
    *dest = q;
    *src = p;
    *output_count = count;
    return state as libc::c_uint != ST_ERROR as libc::c_int as libc::c_uint;
}
static mut color_indicator: [bin_str; 8] = [bin_str {
    len: 0,
    string: 0 as *const libc::c_char,
}; 8];
static mut indicator_name: [*const libc::c_char; 9] = [
    b"lc\0" as *const u8 as *const libc::c_char,
    b"rc\0" as *const u8 as *const libc::c_char,
    b"ec\0" as *const u8 as *const libc::c_char,
    b"rs\0" as *const u8 as *const libc::c_char,
    b"hd\0" as *const u8 as *const libc::c_char,
    b"ad\0" as *const u8 as *const libc::c_char,
    b"de\0" as *const u8 as *const libc::c_char,
    b"ln\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut color_palette: *const libc::c_char = 0 as *const libc::c_char;
pub unsafe extern "C" fn set_color_palette(mut palette: *const libc::c_char) {
    color_palette = palette;
}
unsafe extern "C" fn parse_diff_color() {
    let mut color_buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ind_no: libc::c_int = 0;
    let mut label: [libc::c_char; 3] = *::std::mem::transmute::<
        &[u8; 3],
        &mut [libc::c_char; 3],
    >(b"??\0");
    let mut ext: *mut color_ext_type = 0 as *mut color_ext_type;
    p = color_palette;
    if p.is_null() || *p as libc::c_int == '\0' as i32 {
        return;
    }
    ext = 0 as *mut color_ext_type;
    color_buf = xstrdup(p);
    buf = color_buf;
    let mut state: parse_state = PS_START;
    loop {
        match state as libc::c_uint {
            1 => {
                match *p as libc::c_int {
                    58 => {
                        p = p.offset(1);
                        p;
                    }
                    42 => {
                        ext = xmalloc(
                            ::std::mem::size_of::<color_ext_type>() as libc::c_ulong,
                        ) as *mut color_ext_type;
                        (*ext).next = color_ext_list;
                        color_ext_list = ext;
                        p = p.offset(1);
                        p;
                        (*ext).ext.string = buf;
                        state = (if get_funky_string(
                            &mut buf,
                            &mut p,
                            1 as libc::c_int != 0,
                            &mut (*ext).ext.len,
                        ) as libc::c_int != 0
                        {
                            PS_4 as libc::c_int
                        } else {
                            PS_FAIL as libc::c_int
                        }) as parse_state;
                    }
                    0 => {
                        state = PS_DONE;
                        break;
                    }
                    _ => {
                        let fresh14 = p;
                        p = p.offset(1);
                        label[0 as libc::c_int as usize] = *fresh14;
                        state = PS_2;
                    }
                }
            }
            2 => {
                if *p != 0 {
                    let fresh15 = p;
                    p = p.offset(1);
                    label[1 as libc::c_int as usize] = *fresh15;
                    state = PS_3;
                } else {
                    state = PS_FAIL;
                }
            }
            3 => {
                state = PS_FAIL;
                let fresh16 = p;
                p = p.offset(1);
                if *fresh16 as libc::c_int == '=' as i32 {
                    ind_no = 0 as libc::c_int;
                    while !(indicator_name[ind_no as usize]).is_null() {
                        if strcmp(label.as_mut_ptr(), indicator_name[ind_no as usize])
                            == 0 as libc::c_int
                        {
                            color_indicator[ind_no as usize].string = buf;
                            state = (if get_funky_string(
                                &mut buf,
                                &mut p,
                                0 as libc::c_int != 0,
                                &mut (*color_indicator.as_mut_ptr().offset(ind_no as isize))
                                    .len,
                            ) as libc::c_int != 0
                            {
                                PS_START as libc::c_int
                            } else {
                                PS_FAIL as libc::c_int
                            }) as parse_state;
                            break;
                        } else {
                            ind_no += 1;
                            ind_no;
                        }
                    }
                    if state as libc::c_uint == PS_FAIL as libc::c_int as libc::c_uint {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"unrecognized prefix: %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            label.as_mut_ptr(),
                        );
                    }
                }
            }
            4 => {
                let fresh17 = p;
                p = p.offset(1);
                if *fresh17 as libc::c_int == '=' as i32 {
                    (*ext).seq.string = buf;
                    state = (if get_funky_string(
                        &mut buf,
                        &mut p,
                        0 as libc::c_int != 0,
                        &mut (*ext).seq.len,
                    ) as libc::c_int != 0
                    {
                        PS_START as libc::c_int
                    } else {
                        PS_FAIL as libc::c_int
                    }) as parse_state;
                } else {
                    state = PS_FAIL;
                }
            }
            6 => {
                break;
            }
            _ => {
                abort();
            }
        }
    }
    if state as libc::c_uint == PS_FAIL as libc::c_int as libc::c_uint {
        let mut e: *mut color_ext_type = 0 as *mut color_ext_type;
        let mut e2: *mut color_ext_type = 0 as *mut color_ext_type;
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"unparsable value for --palette\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        rpl_free(color_buf as *mut libc::c_void);
        e = color_ext_list;
        while !e.is_null() {
            e2 = e;
            e = (*e).next;
            rpl_free(e2 as *mut libc::c_void);
        }
        colors_enabled = 0 as libc::c_int != 0;
    }
}
unsafe extern "C" fn check_color_output(mut is_pipe: bool) {
    let mut output_is_tty: bool = false;
    if outfile.is_null()
        || colors_style as libc::c_uint == NEVER as libc::c_int as libc::c_uint
    {
        return;
    }
    output_is_tty = presume_output_tty as libc::c_int != 0
        || !is_pipe && isatty(fileno(outfile)) != 0;
    colors_enabled = colors_style as libc::c_uint
        == ALWAYS as libc::c_int as libc::c_uint
        || colors_style as libc::c_uint == AUTO as libc::c_int as libc::c_uint
            && output_is_tty as libc::c_int != 0;
    if colors_enabled {
        parse_diff_color();
    }
    if output_is_tty {
        install_signal_handlers();
    }
}
pub unsafe extern "C" fn setup_output(
    mut name0: *const libc::c_char,
    mut name1: *const libc::c_char,
    mut recursive: bool,
) {
    current_name0 = name0;
    current_name1 = name1;
    currently_recursive = recursive;
    outfile = 0 as *mut FILE;
}
static mut pr_pid: pid_t = 0;
unsafe extern "C" fn c_escape_char(mut c: libc::c_char) -> libc::c_char {
    match c as libc::c_int {
        7 => return 'a' as i32 as libc::c_char,
        8 => return 'b' as i32 as libc::c_char,
        9 => return 't' as i32 as libc::c_char,
        10 => return 'n' as i32 as libc::c_char,
        11 => return 'v' as i32 as libc::c_char,
        12 => return 'f' as i32 as libc::c_char,
        13 => return 'r' as i32 as libc::c_char,
        34 => return '"' as i32 as libc::c_char,
        92 => return '\\' as i32 as libc::c_char,
        _ => {
            return ((c as libc::c_int) < 32 as libc::c_int) as libc::c_int
                as libc::c_char;
        }
    };
}
unsafe extern "C" fn c_escape(mut str: *const libc::c_char) -> *mut libc::c_char {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut plus: size_t = 0 as libc::c_int as size_t;
    let mut must_quote: bool = 0 as libc::c_int != 0;
    s = str;
    while *s != 0 {
        let mut c: libc::c_char = *s;
        if c as libc::c_int == ' ' as i32 {
            must_quote = 1 as libc::c_int != 0;
        } else {
            match c_escape_char(*s) as libc::c_int {
                1 => {
                    plus = (plus as libc::c_ulong)
                        .wrapping_add(3 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                }
                0 => {}
                _ => {
                    plus = plus.wrapping_add(1);
                    plus;
                }
            }
        }
        s = s.offset(1);
        s;
    }
    if must_quote as libc::c_int != 0 || plus != 0 {
        let mut s_len: size_t = s.offset_from(str) as libc::c_long as size_t;
        let mut buffer: *mut libc::c_char = xmalloc(
            s_len.wrapping_add(plus).wrapping_add(3 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        let mut b: *mut libc::c_char = buffer;
        let fresh18 = b;
        b = b.offset(1);
        *fresh18 = '"' as i32 as libc::c_char;
        s = str;
        while *s != 0 {
            let mut c_0: libc::c_char = *s;
            let mut escape: libc::c_char = c_escape_char(c_0);
            match escape as libc::c_int {
                0 => {
                    let fresh19 = b;
                    b = b.offset(1);
                    *fresh19 = c_0;
                }
                1 => {
                    let fresh20 = b;
                    b = b.offset(1);
                    *fresh20 = '\\' as i32 as libc::c_char;
                    let fresh21 = b;
                    b = b.offset(1);
                    *fresh21 = ((c_0 as libc::c_int >> 6 as libc::c_int
                        & 0o3 as libc::c_int) + '0' as i32) as libc::c_char;
                    let fresh22 = b;
                    b = b.offset(1);
                    *fresh22 = ((c_0 as libc::c_int >> 3 as libc::c_int
                        & 0o7 as libc::c_int) + '0' as i32) as libc::c_char;
                    let fresh23 = b;
                    b = b.offset(1);
                    *fresh23 = ((c_0 as libc::c_int >> 0 as libc::c_int
                        & 0o7 as libc::c_int) + '0' as i32) as libc::c_char;
                }
                _ => {
                    let fresh24 = b;
                    b = b.offset(1);
                    *fresh24 = '\\' as i32 as libc::c_char;
                    let fresh25 = b;
                    b = b.offset(1);
                    *fresh25 = escape;
                }
            }
            s = s.offset(1);
            s;
        }
        let fresh26 = b;
        b = b.offset(1);
        *fresh26 = '"' as i32 as libc::c_char;
        *b = 0 as libc::c_int as libc::c_char;
        return buffer;
    }
    return str as *mut libc::c_char;
}
pub unsafe extern "C" fn begin_output() {
    let mut names: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    if !outfile.is_null() {
        return;
    }
    names[0 as libc::c_int as usize] = c_escape(current_name0);
    names[1 as libc::c_int as usize] = c_escape(current_name1);
    name = xmalloc(
        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_add(strlen(switch_string))
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(strlen(names[0 as libc::c_int as usize]))
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(strlen(names[1 as libc::c_int as usize])),
    ) as *mut libc::c_char;
    let mut p: *mut libc::c_char = stpcpy(
        name,
        b"diff\0" as *const u8 as *const libc::c_char,
    );
    p = stpcpy(p, switch_string);
    let fresh27 = p;
    p = p.offset(1);
    *fresh27 = ' ' as i32 as libc::c_char;
    p = stpcpy(p, names[0 as libc::c_int as usize]);
    let fresh28 = p;
    p = p.offset(1);
    *fresh28 = ' ' as i32 as libc::c_char;
    strcpy(p, names[1 as libc::c_int as usize]);
    if paginate {
        let mut argv: [*const libc::c_char; 4] = [0 as *const libc::c_char; 4];
        if fflush_unlocked(stdout) != 0 as libc::c_int {
            pfatal_with_name(
                dcgettext(
                    0 as *const libc::c_char,
                    b"write failed\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        argv[0 as libc::c_int as usize] = pr_program.as_ptr();
        argv[1 as libc::c_int as usize] = b"-h\0" as *const u8 as *const libc::c_char;
        argv[2 as libc::c_int as usize] = name;
        argv[3 as libc::c_int as usize] = 0 as *const libc::c_char;
        let mut pipes: [libc::c_int; 2] = [0; 2];
        if pipe(pipes.as_mut_ptr()) != 0 as libc::c_int {
            pfatal_with_name(b"pipe\0" as *const u8 as *const libc::c_char);
        }
        pr_pid = fork();
        if pr_pid < 0 as libc::c_int {
            pfatal_with_name(b"fork\0" as *const u8 as *const libc::c_char);
        }
        if pr_pid == 0 as libc::c_int {
            close(pipes[1 as libc::c_int as usize]);
            if pipes[0 as libc::c_int as usize] != 0 as libc::c_int {
                if dup2(pipes[0 as libc::c_int as usize], 0 as libc::c_int)
                    < 0 as libc::c_int
                {
                    pfatal_with_name(b"dup2\0" as *const u8 as *const libc::c_char);
                }
                close(pipes[0 as libc::c_int as usize]);
            }
            execv(
                pr_program.as_ptr(),
                argv.as_mut_ptr() as *mut *mut libc::c_char as *const *mut libc::c_char,
            );
            _exit(
                if *__errno_location() == 2 as libc::c_int {
                    127 as libc::c_int
                } else {
                    126 as libc::c_int
                },
            );
        } else {
            close(pipes[0 as libc::c_int as usize]);
            outfile = fdopen(
                pipes[1 as libc::c_int as usize],
                b"w\0" as *const u8 as *const libc::c_char,
            );
            if outfile.is_null() {
                pfatal_with_name(b"fdopen\0" as *const u8 as *const libc::c_char);
            }
            check_color_output(1 as libc::c_int != 0);
        }
    } else {
        outfile = stdout;
        check_color_output(0 as libc::c_int != 0);
        if currently_recursive {
            printf(b"%s\n\0" as *const u8 as *const libc::c_char, name);
        }
    }
    rpl_free(name as *mut libc::c_void);
    match output_style as libc::c_uint {
        2 => {
            print_context_header(
                files.as_mut_ptr(),
                names.as_mut_ptr() as *const *const libc::c_char,
                0 as libc::c_int != 0,
            );
        }
        3 => {
            print_context_header(
                files.as_mut_ptr(),
                names.as_mut_ptr() as *const *const libc::c_char,
                1 as libc::c_int != 0,
            );
        }
        _ => {}
    }
    if names[0 as libc::c_int as usize] != current_name0 as *mut libc::c_char {
        rpl_free(names[0 as libc::c_int as usize] as *mut libc::c_void);
    }
    if names[1 as libc::c_int as usize] != current_name1 as *mut libc::c_char {
        rpl_free(names[1 as libc::c_int as usize] as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn finish_output() {
    if !outfile.is_null() && outfile != stdout {
        let mut status: libc::c_int = 0;
        let mut wstatus: libc::c_int = 0;
        let mut werrno: libc::c_int = 0 as libc::c_int;
        if ferror_unlocked(outfile) != 0 {
            fatal(b"write failed\0" as *const u8 as *const libc::c_char);
        }
        if fclose(outfile) != 0 as libc::c_int {
            pfatal_with_name(
                dcgettext(
                    0 as *const libc::c_char,
                    b"write failed\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        if waitpid(pr_pid, &mut wstatus, 0 as libc::c_int) < 0 as libc::c_int {
            pfatal_with_name(b"waitpid\0" as *const u8 as *const libc::c_char);
        }
        status = if werrno == 0 && wstatus & 0x7f as libc::c_int == 0 as libc::c_int {
            (wstatus & 0xff00 as libc::c_int) >> 8 as libc::c_int
        } else {
            2147483647 as libc::c_int
        };
        if status != 0 {
            if ::std::mem::size_of::<C2RustUnnamed_14>() as libc::c_ulong != 0 {
                error(
                    2 as libc::c_int,
                    werrno,
                    dcgettext(
                        0 as *const libc::c_char,
                        if status == 126 as libc::c_int {
                            b"subsidiary program '%s' could not be invoked\0"
                                as *const u8 as *const libc::c_char
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
                    pr_program.as_ptr(),
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
                            b"subsidiary program '%s' could not be invoked\0"
                                as *const u8 as *const libc::c_char
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
                    pr_program.as_ptr(),
                    status,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    outfile = 0 as *mut FILE;
}
pub unsafe extern "C" fn lines_differ(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
) -> bool {
    let mut t1: *const libc::c_char = s1;
    let mut t2: *const libc::c_char = s2;
    let mut column: size_t = 0 as libc::c_int as size_t;
    let mut current_block_40: u64;
    loop {
        let fresh29 = t1;
        t1 = t1.offset(1);
        let mut c1: libc::c_uchar = *fresh29 as libc::c_uchar;
        let fresh30 = t2;
        t2 = t2.offset(1);
        let mut c2: libc::c_uchar = *fresh30 as libc::c_uchar;
        if c1 as libc::c_int != c2 as libc::c_int {
            match ignore_white_space as libc::c_uint {
                5 => {
                    while *(*__ctype_b_loc()).offset(c1 as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                        && c1 as libc::c_int != '\n' as i32
                    {
                        let fresh31 = t1;
                        t1 = t1.offset(1);
                        c1 = *fresh31 as libc::c_uchar;
                    }
                    while *(*__ctype_b_loc()).offset(c2 as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                        && c2 as libc::c_int != '\n' as i32
                    {
                        let fresh32 = t2;
                        t2 = t2.offset(1);
                        c2 = *fresh32 as libc::c_uchar;
                    }
                    current_block_40 = 9241535491006583629;
                }
                4 => {
                    if *(*__ctype_b_loc()).offset(c1 as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                    {
                        while c1 as libc::c_int != '\n' as i32 {
                            let fresh33 = t1;
                            t1 = t1.offset(1);
                            c1 = *fresh33 as libc::c_uchar;
                            if !(*(*__ctype_b_loc()).offset(c1 as libc::c_int as isize)
                                as libc::c_int
                                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                == 0)
                            {
                                continue;
                            }
                            t1 = t1.offset(-1);
                            t1;
                            c1 = ' ' as i32 as libc::c_uchar;
                            break;
                        }
                    }
                    if *(*__ctype_b_loc()).offset(c2 as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                    {
                        while c2 as libc::c_int != '\n' as i32 {
                            let fresh34 = t2;
                            t2 = t2.offset(1);
                            c2 = *fresh34 as libc::c_uchar;
                            if !(*(*__ctype_b_loc()).offset(c2 as libc::c_int as isize)
                                as libc::c_int
                                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                == 0)
                            {
                                continue;
                            }
                            t2 = t2.offset(-1);
                            t2;
                            c2 = ' ' as i32 as libc::c_uchar;
                            break;
                        }
                    }
                    if c1 as libc::c_int != c2 as libc::c_int {
                        if c2 as libc::c_int == ' ' as i32
                            && c1 as libc::c_int != '\n' as i32
                            && s1.offset(1 as libc::c_int as isize) < t1
                            && *(*__ctype_b_loc())
                                .offset(
                                    *t1.offset(-(2 as libc::c_int) as isize) as libc::c_uchar
                                        as libc::c_int as isize,
                                ) as libc::c_int
                                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                        {
                            t1 = t1.offset(-1);
                            t1;
                            continue;
                        } else if c1 as libc::c_int == ' ' as i32
                            && c2 as libc::c_int != '\n' as i32
                            && s2.offset(1 as libc::c_int as isize) < t2
                            && *(*__ctype_b_loc())
                                .offset(
                                    *t2.offset(-(2 as libc::c_int) as isize) as libc::c_uchar
                                        as libc::c_int as isize,
                                ) as libc::c_int
                                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                        {
                            t2 = t2.offset(-1);
                            t2;
                            continue;
                        }
                        current_block_40 = 9241535491006583629;
                    } else {
                        current_block_40 = 9241535491006583629;
                    }
                }
                2 | 3 => {
                    if *(*__ctype_b_loc()).offset(c1 as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                        && *(*__ctype_b_loc()).offset(c2 as libc::c_int as isize)
                            as libc::c_int
                            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                    {
                        let mut c: libc::c_uchar = 0;
                        if c1 as libc::c_int != '\n' as i32 {
                            let mut p: *const libc::c_char = t1;
                            loop {
                                c = *p as libc::c_uchar;
                                if !(c as libc::c_int != '\n' as i32
                                    && *(*__ctype_b_loc()).offset(c as libc::c_int as isize)
                                        as libc::c_int
                                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                        != 0)
                                {
                                    break;
                                }
                                p = p.offset(1);
                                p;
                            }
                            if c as libc::c_int != '\n' as i32 {
                                current_block_40 = 9241535491006583629;
                            } else {
                                current_block_40 = 14136749492126903395;
                            }
                        } else {
                            current_block_40 = 14136749492126903395;
                        }
                        match current_block_40 {
                            9241535491006583629 => {}
                            _ => {
                                if c2 as libc::c_int != '\n' as i32 {
                                    let mut p_0: *const libc::c_char = t2;
                                    loop {
                                        c = *p_0 as libc::c_uchar;
                                        if !(c as libc::c_int != '\n' as i32
                                            && *(*__ctype_b_loc()).offset(c as libc::c_int as isize)
                                                as libc::c_int
                                                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                                != 0)
                                        {
                                            break;
                                        }
                                        p_0 = p_0.offset(1);
                                        p_0;
                                    }
                                    if c as libc::c_int != '\n' as i32 {
                                        current_block_40 = 9241535491006583629;
                                    } else {
                                        current_block_40 = 5529461102203738653;
                                    }
                                } else {
                                    current_block_40 = 5529461102203738653;
                                }
                                match current_block_40 {
                                    9241535491006583629 => {}
                                    _ => return 0 as libc::c_int != 0,
                                }
                            }
                        }
                    } else if ignore_white_space as libc::c_uint
                        == IGNORE_TRAILING_SPACE as libc::c_int as libc::c_uint
                    {
                        current_block_40 = 9241535491006583629;
                    } else {
                        current_block_40 = 7762385229115790276;
                    }
                }
                1 => {
                    current_block_40 = 7762385229115790276;
                }
                0 | _ => {
                    current_block_40 = 9241535491006583629;
                }
            }
            match current_block_40 {
                7762385229115790276 => {
                    if c1 as libc::c_int == ' ' as i32
                        && c2 as libc::c_int == '\t' as i32
                        || c1 as libc::c_int == '\t' as i32
                            && c2 as libc::c_int == ' ' as i32
                    {
                        let mut column2: size_t = column;
                        loop {
                            if c1 as libc::c_int == ' ' as i32 {
                                column = column.wrapping_add(1);
                                column;
                            } else {
                                if !(c1 as libc::c_int == '\t' as i32) {
                                    break;
                                }
                                column = (column as libc::c_ulong)
                                    .wrapping_add(
                                        tabsize.wrapping_sub(column.wrapping_rem(tabsize)),
                                    ) as size_t as size_t;
                            }
                            let fresh35 = t1;
                            t1 = t1.offset(1);
                            c1 = *fresh35 as libc::c_uchar;
                        }
                        loop {
                            if c2 as libc::c_int == ' ' as i32 {
                                column2 = column2.wrapping_add(1);
                                column2;
                            } else {
                                if !(c2 as libc::c_int == '\t' as i32) {
                                    break;
                                }
                                column2 = (column2 as libc::c_ulong)
                                    .wrapping_add(
                                        tabsize.wrapping_sub(column2.wrapping_rem(tabsize)),
                                    ) as size_t as size_t;
                            }
                            let fresh36 = t2;
                            t2 = t2.offset(1);
                            c2 = *fresh36 as libc::c_uchar;
                        }
                        if column != column2 {
                            return 1 as libc::c_int != 0;
                        }
                    }
                }
                _ => {}
            }
            if ignore_case {
                c1 = ({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = c1 as libc::c_int;
                            __res = if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = tolower(c1 as libc::c_int);
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc())
                            .offset(c1 as libc::c_int as isize);
                    }
                    __res
                }) as libc::c_uchar;
                c2 = ({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = c2 as libc::c_int;
                            __res = if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = tolower(c2 as libc::c_int);
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc())
                            .offset(c2 as libc::c_int as isize);
                    }
                    __res
                }) as libc::c_uchar;
            }
            if c1 as libc::c_int != c2 as libc::c_int {
                break;
            }
        }
        if c1 as libc::c_int == '\n' as i32 {
            return 0 as libc::c_int != 0;
        }
        column = (column as libc::c_ulong)
            .wrapping_add(
                if c1 as libc::c_int == '\t' as i32 {
                    tabsize.wrapping_sub(column.wrapping_rem(tabsize))
                } else {
                    1 as libc::c_int as libc::c_ulong
                },
            ) as size_t as size_t;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn find_change(mut start: *mut change) -> *mut change {
    return start;
}
pub unsafe extern "C" fn find_reverse_change(mut start: *mut change) -> *mut change {
    return start;
}
pub unsafe extern "C" fn print_script(
    mut script: *mut change,
    mut hunkfun: Option::<unsafe extern "C" fn(*mut change) -> *mut change>,
    mut printfun: Option::<unsafe extern "C" fn(*mut change) -> ()>,
) {
    let mut next: *mut change = script;
    while !next.is_null() {
        let mut this: *mut change = 0 as *mut change;
        let mut end: *mut change = 0 as *mut change;
        this = next;
        end = (Some(hunkfun.unwrap())).unwrap()(next);
        next = (*end).link;
        (*end).link = 0 as *mut change;
        (Some(printfun.unwrap())).unwrap()(this);
        (*end).link = next;
    }
}
pub unsafe extern "C" fn print_1_line(
    mut line_flag: *const libc::c_char,
    mut line: *const *const libc::c_char,
) {
    print_1_line_nl(line_flag, line, 0 as libc::c_int != 0);
}
pub unsafe extern "C" fn print_1_line_nl(
    mut line_flag: *const libc::c_char,
    mut line: *const *const libc::c_char,
    mut skip_nl: bool,
) {
    let mut base: *const libc::c_char = *line.offset(0 as libc::c_int as isize);
    let mut limit: *const libc::c_char = *line.offset(1 as libc::c_int as isize);
    let mut out: *mut FILE = outfile;
    let mut flag_format: *const libc::c_char = 0 as *const libc::c_char;
    if !line_flag.is_null() && *line_flag as libc::c_int != 0 {
        flag_format = if initial_tab as libc::c_int != 0 {
            b"%s\t\0" as *const u8 as *const libc::c_char
        } else {
            b"%s \0" as *const u8 as *const libc::c_char
        };
        let mut flag_format_1: *const libc::c_char = flag_format;
        let mut line_flag_1: *const libc::c_char = line_flag;
        if suppress_blank_empty as libc::c_int != 0
            && **line as libc::c_int == '\n' as i32
        {
            flag_format_1 = b"%s\0" as *const u8 as *const libc::c_char;
            line_flag_1 = line_flag_1
                .offset(
                    (*line_flag_1 as libc::c_int == ' ' as i32) as libc::c_int as isize,
                );
        }
        fprintf(out, flag_format_1, line_flag_1);
    }
    output_1_line(
        base,
        limit
            .offset(
                -((skip_nl as libc::c_int != 0
                    && *limit.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == '\n' as i32) as libc::c_int as isize),
            ),
        flag_format,
        line_flag,
    );
    if (line_flag.is_null()
        || *line_flag.offset(0 as libc::c_int as isize) as libc::c_int != 0)
        && *limit.offset(-(1 as libc::c_int) as isize) as libc::c_int != '\n' as i32
    {
        set_color_context(RESET_CONTEXT);
        fprintf(
            out,
            b"\n\\ %s\n\0" as *const u8 as *const libc::c_char,
            dcgettext(
                0 as *const libc::c_char,
                b"No newline at end of file\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
}
pub unsafe extern "C" fn output_1_line(
    mut base: *const libc::c_char,
    mut limit: *const libc::c_char,
    mut flag_format: *const libc::c_char,
    mut line_flag: *const libc::c_char,
) {
    let MAX_CHUNK: size_t = 1024 as libc::c_int as size_t;
    if !expand_tabs {
        let mut left: size_t = limit.offset_from(base) as libc::c_long as size_t;
        while left != 0 {
            let mut to_write: size_t = if left <= MAX_CHUNK { left } else { MAX_CHUNK };
            let mut written: size_t = if 0 != 0 && 0 != 0
                && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul(to_write) <= 8 as libc::c_int as libc::c_ulong
                && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    != 0 as libc::c_int as libc::c_ulong
            {
                {
                    let mut __ptr: *const libc::c_char = base;
                    let mut __stream: *mut FILE = outfile;
                    let mut __cnt: size_t = 0;
                    __cnt = (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                        .wrapping_mul(to_write);
                    while __cnt > 0 as libc::c_int as libc::c_ulong {
                        let fresh37 = __ptr;
                        __ptr = __ptr.offset(1);
                        if putc_unlocked(*fresh37 as libc::c_int, __stream)
                            == -(1 as libc::c_int)
                        {
                            break;
                        }
                        __cnt = __cnt.wrapping_sub(1);
                        __cnt;
                    }
                    (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                        .wrapping_mul(to_write)
                        .wrapping_sub(__cnt)
                        .wrapping_div(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        )
                }
            } else if 0 != 0
                && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    == 0 as libc::c_int as libc::c_ulong
                || 0 != 0 && to_write == 0 as libc::c_int as libc::c_ulong
            {
                0 as libc::c_int as size_t
            } else {
                fwrite_unlocked(
                    base as *const libc::c_void,
                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    to_write,
                    outfile,
                )
            };
            if written < to_write {
                return;
            }
            base = base.offset(written as isize);
            left = (left as libc::c_ulong).wrapping_sub(written) as size_t as size_t;
            process_signals();
        }
    } else {
        let mut out: *mut FILE = outfile;
        let mut c: libc::c_uchar = 0;
        let mut t: *const libc::c_char = base;
        let mut column: size_t = 0 as libc::c_int as size_t;
        let mut tab_size: size_t = tabsize;
        let mut counter_proc_signals: size_t = 0 as libc::c_int as size_t;
        while t < limit {
            counter_proc_signals = counter_proc_signals.wrapping_add(1);
            counter_proc_signals;
            if counter_proc_signals == MAX_CHUNK {
                process_signals();
                counter_proc_signals = 0 as libc::c_int as size_t;
            }
            let fresh38 = t;
            t = t.offset(1);
            c = *fresh38 as libc::c_uchar;
            match c as libc::c_int {
                9 => {
                    let mut spaces: size_t = tab_size
                        .wrapping_sub(column.wrapping_rem(tab_size));
                    column = (column as libc::c_ulong).wrapping_add(spaces) as size_t
                        as size_t;
                    loop {
                        putc_unlocked(' ' as i32, out);
                        spaces = spaces.wrapping_sub(1);
                        if !(spaces != 0) {
                            break;
                        }
                    }
                }
                13 => {
                    putc_unlocked(c as libc::c_int, out);
                    if !flag_format.is_null() && t < limit
                        && *t as libc::c_int != '\n' as i32
                    {
                        fprintf(out, flag_format, line_flag);
                    }
                    column = 0 as libc::c_int as size_t;
                }
                8 => {
                    if column == 0 as libc::c_int as libc::c_ulong {
                        continue;
                    }
                    column = column.wrapping_sub(1);
                    column;
                    putc_unlocked(c as libc::c_int, out);
                }
                _ => {
                    column = (column as libc::c_ulong)
                        .wrapping_add(
                            (*(*__ctype_b_loc()).offset(c as libc::c_int as isize)
                                as libc::c_int
                                & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
                                != 0 as libc::c_int) as libc::c_int as libc::c_ulong,
                        ) as size_t as size_t;
                    putc_unlocked(c as libc::c_int, out);
                }
            }
        }
    };
}
unsafe extern "C" fn put_indicator(mut ind: *const bin_str) {
    if 0 != 0 && 0 != 0
        && ((*ind).len).wrapping_mul(1 as libc::c_int as size_t)
            <= 8 as libc::c_int as libc::c_ulong
        && (*ind).len != 0 as libc::c_int as libc::c_ulong
    {
        ({
            let mut __ptr: *const libc::c_char = (*ind).string;
            let mut __stream: *mut FILE = outfile;
            let mut __cnt: size_t = 0;
            __cnt = ((*ind).len).wrapping_mul(1 as libc::c_int as size_t);
            while __cnt > 0 as libc::c_int as libc::c_ulong {
                let fresh39 = __ptr;
                __ptr = __ptr.offset(1);
                if putc_unlocked(*fresh39 as libc::c_int, __stream)
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
        if 0 != 0 && (*ind).len == 0 as libc::c_int as libc::c_ulong
            || 0 != 0 && 1 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
        {} else {
            fwrite_unlocked(
                (*ind).string as *const libc::c_void,
                (*ind).len,
                1 as libc::c_int as size_t,
                outfile,
            );
        };
    };
    0u8;
}
static mut last_context: color_context = RESET_CONTEXT;
pub unsafe extern "C" fn set_color_context(mut color_context: color_context) {
    if color_context as libc::c_uint != RESET_CONTEXT as libc::c_int as libc::c_uint {
        process_signals();
    }
    if colors_enabled as libc::c_int != 0
        && last_context as libc::c_uint != color_context as libc::c_uint
    {
        put_indicator(
            &mut *color_indicator.as_mut_ptr().offset(C_LEFT as libc::c_int as isize),
        );
        match color_context as libc::c_uint {
            0 => {
                put_indicator(
                    &mut *color_indicator
                        .as_mut_ptr()
                        .offset(C_HEADER as libc::c_int as isize),
                );
            }
            4 => {
                put_indicator(
                    &mut *color_indicator
                        .as_mut_ptr()
                        .offset(C_LINE as libc::c_int as isize),
                );
            }
            1 => {
                put_indicator(
                    &mut *color_indicator
                        .as_mut_ptr()
                        .offset(C_ADD as libc::c_int as isize),
                );
            }
            2 => {
                put_indicator(
                    &mut *color_indicator
                        .as_mut_ptr()
                        .offset(C_DELETE as libc::c_int as isize),
                );
            }
            3 => {
                put_indicator(
                    &mut *color_indicator
                        .as_mut_ptr()
                        .offset(C_RESET as libc::c_int as isize),
                );
            }
            _ => {
                abort();
            }
        }
        put_indicator(
            &mut *color_indicator.as_mut_ptr().offset(C_RIGHT as libc::c_int as isize),
        );
        last_context = color_context;
    }
}
pub static mut change_letter: [libc::c_char; 4] = [
    0 as libc::c_int as libc::c_char,
    'd' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
];
pub unsafe extern "C" fn translate_line_number(
    mut file: *const file_data,
    mut i: lin,
) -> lin {
    return i + (*file).prefix_lines + 1 as libc::c_int as libc::c_long;
}
pub unsafe extern "C" fn translate_range(
    mut file: *const file_data,
    mut a: lin,
    mut b: lin,
    mut aptr: *mut lin,
    mut bptr: *mut lin,
) {
    *aptr = translate_line_number(file, a - 1 as libc::c_int as libc::c_long)
        + 1 as libc::c_int as libc::c_long;
    *bptr = translate_line_number(file, b + 1 as libc::c_int as libc::c_long)
        - 1 as libc::c_int as libc::c_long;
}
pub unsafe extern "C" fn print_number_range(
    mut sepchar: libc::c_char,
    mut file: *mut file_data,
    mut a: lin,
    mut b: lin,
) {
    let mut trans_a: lin = 0;
    let mut trans_b: lin = 0;
    translate_range(file, a, b, &mut trans_a, &mut trans_b);
    if trans_b > trans_a {
        fprintf(
            outfile,
            b"%td%c%td\0" as *const u8 as *const libc::c_char,
            trans_a,
            sepchar as libc::c_int,
            trans_b,
        );
    } else {
        fprintf(outfile, b"%td\0" as *const u8 as *const libc::c_char, trans_b);
    };
}
pub unsafe extern "C" fn analyze_hunk(
    mut hunk: *mut change,
    mut first0: *mut lin,
    mut last0: *mut lin,
    mut first1: *mut lin,
    mut last1: *mut lin,
) -> changes {
    let mut next: *mut change = 0 as *mut change;
    let mut l0: lin = 0;
    let mut l1: lin = 0;
    let mut show_from: lin = 0;
    let mut show_to: lin = 0;
    let mut i: lin = 0;
    let mut trivial: bool = ignore_blank_lines as libc::c_int != 0
        || !(ignore_regexp.fastmap).is_null();
    let mut trivial_length: size_t = (ignore_blank_lines as libc::c_int
        - 1 as libc::c_int) as size_t;
    let mut skip_white_space: bool = ignore_blank_lines as libc::c_int != 0
        && IGNORE_TRAILING_SPACE as libc::c_int as libc::c_uint
            <= ignore_white_space as libc::c_uint;
    let mut skip_leading_white_space: bool = skip_white_space as libc::c_int != 0
        && IGNORE_SPACE_CHANGE as libc::c_int as libc::c_uint
            <= ignore_white_space as libc::c_uint;
    let mut linbuf0: *const *const libc::c_char = files[0 as libc::c_int as usize]
        .linbuf;
    let mut linbuf1: *const *const libc::c_char = files[1 as libc::c_int as usize]
        .linbuf;
    show_to = 0 as libc::c_int as lin;
    show_from = show_to;
    *first0 = (*hunk).line0;
    *first1 = (*hunk).line1;
    next = hunk;
    loop {
        l0 = (*next).line0 + (*next).deleted - 1 as libc::c_int as libc::c_long;
        l1 = (*next).line1 + (*next).inserted - 1 as libc::c_int as libc::c_long;
        show_from += (*next).deleted;
        show_to += (*next).inserted;
        i = (*next).line0;
        while i <= l0 && trivial as libc::c_int != 0 {
            let mut line: *const libc::c_char = *linbuf0.offset(i as isize);
            let mut lastbyte: *const libc::c_char = (*linbuf0
                .offset((i + 1 as libc::c_int as libc::c_long) as isize))
                .offset(-(1 as libc::c_int as isize));
            let mut newline: *const libc::c_char = lastbyte
                .offset(
                    (*lastbyte as libc::c_int != '\n' as i32) as libc::c_int as isize,
                );
            let mut len: size_t = newline.offset_from(line) as libc::c_long as size_t;
            let mut p: *const libc::c_char = line;
            if skip_white_space {
                while *p as libc::c_int != '\n' as i32 {
                    if *(*__ctype_b_loc())
                        .offset(*p as libc::c_uchar as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
                    {
                        if !skip_leading_white_space {
                            p = line;
                        }
                        break;
                    } else {
                        p = p.offset(1);
                        p;
                    }
                }
            }
            if newline.offset_from(p) as libc::c_long as libc::c_ulong != trivial_length
                && ((ignore_regexp.fastmap).is_null()
                    || rpl_re_search(
                        &mut ignore_regexp,
                        line,
                        len as regoff_t,
                        0 as libc::c_int as regoff_t,
                        len as regoff_t,
                        0 as *mut re_registers,
                    ) < 0 as libc::c_int as libc::c_long)
            {
                trivial = 0 as libc::c_int != 0;
            }
            i += 1;
            i;
        }
        i = (*next).line1;
        while i <= l1 && trivial as libc::c_int != 0 {
            let mut line_0: *const libc::c_char = *linbuf1.offset(i as isize);
            let mut lastbyte_0: *const libc::c_char = (*linbuf1
                .offset((i + 1 as libc::c_int as libc::c_long) as isize))
                .offset(-(1 as libc::c_int as isize));
            let mut newline_0: *const libc::c_char = lastbyte_0
                .offset(
                    (*lastbyte_0 as libc::c_int != '\n' as i32) as libc::c_int as isize,
                );
            let mut len_0: size_t = newline_0.offset_from(line_0) as libc::c_long
                as size_t;
            let mut p_0: *const libc::c_char = line_0;
            if skip_white_space {
                while *p_0 as libc::c_int != '\n' as i32 {
                    if *(*__ctype_b_loc())
                        .offset(*p_0 as libc::c_uchar as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
                    {
                        if !skip_leading_white_space {
                            p_0 = line_0;
                        }
                        break;
                    } else {
                        p_0 = p_0.offset(1);
                        p_0;
                    }
                }
            }
            if newline_0.offset_from(p_0) as libc::c_long as libc::c_ulong
                != trivial_length
                && ((ignore_regexp.fastmap).is_null()
                    || rpl_re_search(
                        &mut ignore_regexp,
                        line_0,
                        len_0 as regoff_t,
                        0 as libc::c_int as regoff_t,
                        len_0 as regoff_t,
                        0 as *mut re_registers,
                    ) < 0 as libc::c_int as libc::c_long)
            {
                trivial = 0 as libc::c_int != 0;
            }
            i += 1;
            i;
        }
        next = (*next).link;
        if next.is_null() {
            break;
        }
    }
    *last0 = l0;
    *last1 = l1;
    if trivial {
        return UNCHANGED;
    }
    return ((if show_from != 0 { OLD as libc::c_int } else { UNCHANGED as libc::c_int })
        | (if show_to != 0 { NEW as libc::c_int } else { UNCHANGED as libc::c_int }))
        as changes;
}
unsafe extern "C" fn run_static_initializers() {
    color_indicator = [
        {
            let mut init = bin_str {
                len: (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                string: b"\x1B[\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                string: b"m\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: 0 as libc::c_int as size_t,
                string: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                string: b"0\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                string: b"1\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                string: b"32\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                string: b"31\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = bin_str {
                len: (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                string: b"36\0" as *const u8 as *const libc::c_char,
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
