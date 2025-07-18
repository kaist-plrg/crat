use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type re_dfa_t;
    pub type dfa;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut libc::c_char,
        __modes: libc::c_int,
        __n: size_t,
    ) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn btowc(__c: libc::c_int) -> wint_t;
    fn wctob(__c: wint_t) -> libc::c_int;
    fn wcrtomb(__s: *mut libc::c_char, __wc: wchar_t, __ps: *mut mbstate_t) -> size_t;
    fn rpl_free(_: *mut libc::c_void);
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const libc::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn towlower(__wc: wint_t) -> wint_t;
    fn towupper(__wc: wint_t) -> wint_t;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn abort() -> !;
    static mut lcmd_out_line_len: countT;
    static mut separate_files: bool;
    static mut follow_symlinks: bool;
    static mut write_mode: *const libc::c_char;
    static mut posixicity: posixicity_types;
    static mut read_mode: *const libc::c_char;
    static mut is_utf8: bool;
    static mut mb_cur_max: libc::c_int;
    static mut debug: bool;
    static mut no_default_output: bool;
    static mut buffer_delimiter: libc::c_char;
    static mut unbuffered: bool;
    static mut in_place_extension: *mut libc::c_char;
    fn debug_print_char(c: libc::c_char);
    fn debug_print_command(program: *const vector, sc: *const sed_cmd);
    fn match_regex(
        regex: *mut regex,
        buf: *mut libc::c_char,
        buflen: size_t,
        buf_start_offset: size_t,
        regarray: *mut re_registers,
        regsize: libc::c_int,
    ) -> libc::c_int;
    fn rewind_read_files();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xzalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xreallocarray(p: *mut libc::c_void, n: size_t, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn __uflow(_: *mut FILE) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
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
    fn clearerr_unlocked(__stream: *mut FILE);
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn panic(str: *const libc::c_char, _: ...);
    fn ck_fopen(
        name: *const libc::c_char,
        mode: *const libc::c_char,
        fail: libc::c_int,
    ) -> *mut FILE;
    fn ck_fwrite(
        ptr: *const libc::c_void,
        size: size_t,
        nmemb: size_t,
        stream: *mut FILE,
    );
    fn ck_fread(
        ptr: *mut libc::c_void,
        size: size_t,
        nmemb: size_t,
        stream: *mut FILE,
    ) -> size_t;
    fn ck_fflush(stream: *mut FILE);
    fn ck_fclose(stream: *mut FILE);
    fn follow_symlink(path: *const libc::c_char) -> *const libc::c_char;
    fn ck_getdelim(
        text: *mut *mut libc::c_char,
        buflen: *mut size_t,
        buffer_delimiter_0: libc::c_char,
        stream: *mut FILE,
    ) -> size_t;
    fn ck_mkstemp(
        p_filename: *mut *mut libc::c_char,
        tmpdir: *const libc::c_char,
        base: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> *mut FILE;
    fn ck_rename(from: *const libc::c_char, to: *const libc::c_char);
    fn cancel_cleanup();
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn fchown(__fd: libc::c_int, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn is_selinux_enabled() -> libc::c_int;
    fn freecon(con: *mut libc::c_char);
    fn getfscreatecon(con: *mut *mut libc::c_char) -> libc::c_int;
    fn setfscreatecon(context: *const libc::c_char) -> libc::c_int;
    fn rpl_lgetfilecon(
        path: *const libc::c_char,
        con: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn copy_acl(
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: mode_t,
    ) -> libc::c_int;
    static mut program_name: *const libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type wint_t = libc::c_uint;
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
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type mode_t = __mode_t;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type countT = libc::c_ulong;
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
pub type regex_t = re_pattern_buffer;
pub type regoff_t = ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub num_regs: __re_size_t,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
}
pub type exit_codes = libc::c_uint;
pub const EXIT_PANIC: exit_codes = 4;
pub const EXIT_BAD_INPUT: exit_codes = 2;
pub const EXIT_BAD_USAGE: exit_codes = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vector {
    pub v: *mut sed_cmd,
    pub v_allocated: size_t,
    pub v_length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sed_cmd {
    pub a1: *mut addr,
    pub a2: *mut addr,
    pub range_state: addr_state,
    pub addr_bang: libc::c_char,
    pub cmd: libc::c_char,
    pub x: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub cmd_txt: text_buf,
    pub int_arg: libc::c_int,
    pub jump_index: countT,
    pub readcmd: readcmd,
    pub cmd_subst: *mut subst,
    pub outf: *mut output,
    pub inf: *mut output,
    pub translate: *mut libc::c_uchar,
    pub translatemb: *mut *mut libc::c_char,
    pub label_name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output {
    pub name: *mut libc::c_char,
    pub missing_newline: bool,
    pub fp: *mut FILE,
    pub link: *mut output,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct subst {
    pub regx: *mut regex,
    pub replacement: *mut replacement,
    pub numb: countT,
    pub outf: *mut output,
    #[bitfield(name = "global", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "print", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "eval", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "max_id", ty = "libc::c_uint", bits = "4..=7")]
    pub global_print_eval_max_id: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct replacement {
    pub prefix: *mut libc::c_char,
    pub prefix_length: size_t,
    pub subst_id: libc::c_int,
    pub repl_type: replacement_types,
    pub next: *mut replacement,
}
pub type replacement_types = libc::c_uint;
pub const REPL_LOWERCASE_LOWERCASE: replacement_types = 10;
pub const REPL_LOWERCASE_UPPERCASE: replacement_types = 9;
pub const REPL_UPPERCASE_LOWERCASE: replacement_types = 6;
pub const REPL_UPPERCASE_UPPERCASE: replacement_types = 5;
pub const REPL_MODIFIERS: replacement_types = 12;
pub const REPL_LOWERCASE_FIRST: replacement_types = 8;
pub const REPL_UPPERCASE_FIRST: replacement_types = 4;
pub const REPL_LOWERCASE: replacement_types = 2;
pub const REPL_UPPERCASE: replacement_types = 1;
pub const REPL_ASIS: replacement_types = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regex {
    pub pattern: regex_t,
    pub flags: libc::c_int,
    pub sz: size_t,
    pub dfa: *mut dfa,
    pub begline: bool,
    pub endline: bool,
    pub re: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct readcmd {
    pub fname: *mut libc::c_char,
    pub append: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct text_buf {
    pub text: *mut libc::c_char,
    pub text_length: size_t,
}
pub type addr_state = libc::c_uint;
pub const RANGE_CLOSED: addr_state = 2;
pub const RANGE_ACTIVE: addr_state = 1;
pub const RANGE_INACTIVE: addr_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addr {
    pub addr_type: addr_types,
    pub addr_number: countT,
    pub addr_step: countT,
    pub addr_regex: *mut regex,
}
pub type addr_types = libc::c_uint;
pub const ADDR_IS_LAST: addr_types = 6;
pub const ADDR_IS_STEP_MOD: addr_types = 5;
pub const ADDR_IS_STEP: addr_types = 4;
pub const ADDR_IS_NUM_MOD: addr_types = 3;
pub const ADDR_IS_NUM: addr_types = 2;
pub const ADDR_IS_REGEX: addr_types = 1;
pub const ADDR_IS_NULL: addr_types = 0;
pub type posixicity_types = libc::c_uint;
pub const POSIXLY_BASIC: posixicity_types = 2;
pub const POSIXLY_CORRECT: posixicity_types = 1;
pub const POSIXLY_EXTENDED: posixicity_types = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct input {
    pub file_list: *mut *mut libc::c_char,
    pub bad_count: countT,
    pub line_number: countT,
    pub reset_at_next_file: bool,
    pub read_fn: Option::<unsafe extern "C" fn(*mut input) -> bool>,
    pub out_file_name: *mut libc::c_char,
    pub in_file_name: *const libc::c_char,
    pub st: stat,
    pub fp: *mut FILE,
    pub no_buffering: bool,
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
pub struct line {
    pub text: *mut libc::c_char,
    pub active: *mut libc::c_char,
    pub length: size_t,
    pub alloc: size_t,
    pub chomped: bool,
    pub mbstate: mbstate_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct append_queue {
    pub fname: *const libc::c_char,
    pub text: *mut libc::c_char,
    pub textlen: size_t,
    pub next: *mut append_queue,
    pub rpl_free: bool,
}
pub const _ISprint: C2RustUnnamed_1 = 16384;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_1 = 8;
pub const _ISpunct: C2RustUnnamed_1 = 4;
pub const _IScntrl: C2RustUnnamed_1 = 2;
pub const _ISblank: C2RustUnnamed_1 = 1;
pub const _ISgraph: C2RustUnnamed_1 = 32768;
pub const _ISspace: C2RustUnnamed_1 = 8192;
pub const _ISxdigit: C2RustUnnamed_1 = 4096;
pub const _ISdigit: C2RustUnnamed_1 = 2048;
pub const _ISalpha: C2RustUnnamed_1 = 1024;
pub const _ISlower: C2RustUnnamed_1 = 512;
pub const _ISupper: C2RustUnnamed_1 = 256;
#[inline]
unsafe extern "C" fn btowc_unused(mut __c: libc::c_int) -> wint_t {
    return if 0 != 0 && __c >= '\0' as i32 && __c <= '\u{7f}' as i32 {
        __c as wint_t
    } else {
panic!()
    };
}
#[inline]
unsafe extern "C" fn wctob_unused(mut __wc: wint_t) -> libc::c_int {
    return if 0 != 0 && __wc >= '\0' as i32 as libc::c_uint
        && __wc <= '\u{7f}' as i32 as libc::c_uint
    {
        __wc as libc::c_int
    } else {
panic!()
    };
}
#[inline]
unsafe extern "C" fn xnrealloc(
    mut p: *mut libc::c_void,
    mut n: size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    return xreallocarray(p, n, s);
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
unsafe extern "C" fn putc_unlocked(
    mut __c: libc::c_int,
    mut __stream: *mut FILE,
) -> libc::c_int {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as libc::c_int
        as libc::c_long != 0
    {
        __overflow(__stream, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh1 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
        *fresh1 = __c as libc::c_char;
        *fresh1 as libc::c_uchar as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn putchar_unlocked(mut __c: libc::c_int) -> libc::c_int {
    return if ((*stdout)._IO_write_ptr >= (*stdout)._IO_write_end) as libc::c_int
        as libc::c_long != 0
    {
        __overflow(stdout, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh2 = (*stdout)._IO_write_ptr;
        (*stdout)._IO_write_ptr = ((*stdout)._IO_write_ptr).offset(1);
        *fresh2 = __c as libc::c_char;
        *fresh2 as libc::c_uchar as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn feof_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x10 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
static mut replaced: bool = 0 as libc::c_int != 0;
static mut output_file: output = output {
    name: 0 as *const libc::c_char as *mut libc::c_char,
    missing_newline: false,
    fp: 0 as *const FILE as *mut FILE,
    link: 0 as *const output as *mut output,
};
static mut line: line = line {
    text: 0 as *const libc::c_char as *mut libc::c_char,
    active: 0 as *const libc::c_char as *mut libc::c_char,
    length: 0,
    alloc: 0,
    chomped: false,
    mbstate: mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    },
};
static mut s_accum: line = line {
    text: 0 as *const libc::c_char as *mut libc::c_char,
    active: 0 as *const libc::c_char as *mut libc::c_char,
    length: 0,
    alloc: 0,
    chomped: false,
    mbstate: mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    },
};
static mut hold: line = line {
    text: 0 as *const libc::c_char as *mut libc::c_char,
    active: 0 as *const libc::c_char as *mut libc::c_char,
    length: 0,
    alloc: 0,
    chomped: false,
    mbstate: mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    },
};
static mut buffer: line = line {
    text: 0 as *const libc::c_char as *mut libc::c_char,
    active: 0 as *const libc::c_char as *mut libc::c_char,
    length: 0,
    alloc: 0,
    chomped: false,
    mbstate: mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    },
};
static mut append_head: *mut append_queue = 0 as *const append_queue
    as *mut append_queue;
static mut append_tail: *mut append_queue = 0 as *const append_queue
    as *mut append_queue;
unsafe extern "C" fn resize_line(mut lb: *mut line, mut len: size_t) {
    let mut inactive: libc::c_int = 0;
    inactive = ((*lb).active).offset_from((*lb).text) as libc::c_long as libc::c_int;
    if inactive as libc::c_ulong
        > ((*lb).alloc).wrapping_mul(2 as libc::c_int as libc::c_ulong)
    {
        memmove(
            (*lb).text as *mut libc::c_void,
            (*lb).active as *const libc::c_void,
            (*lb).length,
        );
        (*lb)
            .alloc = ((*lb).alloc as libc::c_ulong)
            .wrapping_add(
                ((*lb).active).offset_from((*lb).text) as libc::c_long as libc::c_ulong,
            ) as size_t as size_t;
        (*lb).active = (*lb).text;
        inactive = 0 as libc::c_int;
        if (*lb).alloc > len {
            return;
        }
    }
    (*lb)
        .alloc = ((*lb).alloc as libc::c_ulong)
        .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
    if (*lb).alloc < len {
        (*lb).alloc = len;
    }
    if (*lb).alloc < 50 as libc::c_int as libc::c_ulong {
        (*lb).alloc = 50 as libc::c_int as size_t;
    }
    (*lb)
        .text = xnrealloc(
        (*lb).text as *mut libc::c_void,
        (inactive as libc::c_ulong)
            .wrapping_add((*lb).alloc)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
    ) as *mut libc::c_char;
    (*lb).active = ((*lb).text).offset(inactive as isize);
}
unsafe extern "C" fn str_append(
    mut to: *mut line,
    mut string: *const libc::c_char,
    mut length: size_t,
) {
    let mut new_length: size_t = ((*to).length).wrapping_add(length);
    if (*to).alloc < new_length {
        resize_line(to, new_length);
    }
    memcpy(
        ((*to).active).offset((*to).length as isize) as *mut libc::c_void,
        string as *const libc::c_void,
        length,
    );
    (*to).length = new_length;
    if mb_cur_max > 1 as libc::c_int && !is_utf8 {
        while length != 0 {
            let mut n: size_t = if mb_cur_max == 1 as libc::c_int {
                1 as libc::c_int as libc::c_ulong
            } else {
                rpl_mbrtowc(0 as *mut wchar_t, string, length, &mut (*to).mbstate)
            };
            if n == -(1 as libc::c_int) as size_t || n == -(2 as libc::c_int) as size_t {
                memset(
                    &mut (*to).mbstate as *mut mbstate_t as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
                );
                n = 1 as libc::c_int as size_t;
            }
            if n == 0 as libc::c_int as libc::c_ulong {
                break;
            }
            string = string.offset(n as isize);
            length = (length as libc::c_ulong).wrapping_sub(n) as size_t as size_t;
        }
    }
}
unsafe extern "C" fn str_append_modified(
    mut to: *mut line,
    mut string: *const libc::c_char,
    mut length: size_t,
    mut type_0: replacement_types,
) {
    let mut from_stat: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    if type_0 as libc::c_uint == REPL_ASIS as libc::c_int as libc::c_uint {
        str_append(to, string, length);
        return;
    }
    if ((*to).alloc).wrapping_sub((*to).length)
        < length.wrapping_mul(mb_cur_max as libc::c_ulong)
    {
        resize_line(
            to,
            ((*to).length).wrapping_add(length.wrapping_mul(mb_cur_max as libc::c_ulong)),
        );
    }
    memcpy(
        &mut from_stat as *mut mbstate_t as *mut libc::c_void,
        &mut (*to).mbstate as *mut mbstate_t as *const libc::c_void,
        ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    while length != 0 {
        let mut wc: wchar_t = 0;
        let mut n: size_t = if mb_cur_max == 1 as libc::c_int {
            wc = btowc(*(string as *mut libc::c_uchar) as libc::c_int) as wchar_t;
            1 as libc::c_int as libc::c_ulong
        } else {
            rpl_mbrtowc(&mut wc, string, length, &mut from_stat)
        };
        if n == -(1 as libc::c_int) as size_t {
            type_0 = ::std::mem::transmute::<
                libc::c_uint,
                replacement_types,
            >(
                type_0 as libc::c_uint
                    & !(REPL_LOWERCASE_FIRST as libc::c_int
                        | REPL_UPPERCASE_FIRST as libc::c_int) as libc::c_uint,
            );
            if type_0 as libc::c_uint == REPL_ASIS as libc::c_int as libc::c_uint {
                str_append(to, string, length);
                return;
            }
            str_append(to, string, 1 as libc::c_int as size_t);
            memset(
                &mut (*to).mbstate as *mut mbstate_t as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
            );
            n = 1 as libc::c_int as size_t;
            string = string.offset(n as isize);
            length = (length as libc::c_ulong).wrapping_sub(n) as size_t as size_t;
        } else {
            if n == 0 as libc::c_int as libc::c_ulong
                || n == -(2 as libc::c_int) as size_t
            {
                str_append(to, string, length);
                return;
            }
            string = string.offset(n as isize);
            length = (length as libc::c_ulong).wrapping_sub(n) as size_t as size_t;
            if type_0 as libc::c_uint
                & (REPL_UPPERCASE_FIRST as libc::c_int
                    | REPL_LOWERCASE_FIRST as libc::c_int) as libc::c_uint != 0
            {
                if type_0 as libc::c_uint
                    & REPL_UPPERCASE_FIRST as libc::c_int as libc::c_uint != 0
                {
                    wc = towupper(wc as wint_t) as wchar_t;
                } else {
                    wc = towlower(wc as wint_t) as wchar_t;
                }
                type_0 = ::std::mem::transmute::<
                    libc::c_uint,
                    replacement_types,
                >(
                    type_0 as libc::c_uint
                        & !(REPL_LOWERCASE_FIRST as libc::c_int
                            | REPL_UPPERCASE_FIRST as libc::c_int) as libc::c_uint,
                );
                if type_0 as libc::c_uint == REPL_ASIS as libc::c_int as libc::c_uint {
                    n = if mb_cur_max == 1 as libc::c_int {
                        *((*to).active)
                            .offset(
                                (*to).length as isize,
                            ) = wctob(wc as wint_t) as libc::c_char;
                        1 as libc::c_int as libc::c_ulong
                    } else {
                        wcrtomb(
                            ((*to).active).offset((*to).length as isize),
                            wc,
                            &mut (*to).mbstate,
                        )
                    };
                    (*to)
                        .length = ((*to).length as libc::c_ulong).wrapping_add(n)
                        as size_t as size_t;
                    if n == -(1 as libc::c_int) as size_t
                        || n == -(2 as libc::c_int) as size_t
                    {
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"case conversion produced an invalid character\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        abort();
                    }
                    str_append(to, string, length);
                    return;
                }
            } else if type_0 as libc::c_uint
                & REPL_UPPERCASE as libc::c_int as libc::c_uint != 0
            {
                wc = towupper(wc as wint_t) as wchar_t;
            } else {
                wc = towlower(wc as wint_t) as wchar_t;
            }
            n = if mb_cur_max == 1 as libc::c_int {
                *((*to).active)
                    .offset((*to).length as isize) = wctob(wc as wint_t) as libc::c_char;
                1 as libc::c_int as libc::c_ulong
            } else {
                wcrtomb(
                    ((*to).active).offset((*to).length as isize),
                    wc,
                    &mut (*to).mbstate,
                )
            };
            (*to)
                .length = ((*to).length as libc::c_ulong).wrapping_add(n) as size_t
                as size_t;
            if n == -(1 as libc::c_int) as libc::c_ulong
                || n == -(2 as libc::c_int) as libc::c_ulong
            {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"case conversion produced an invalid character\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                abort();
            }
        }
    }
}
unsafe extern "C" fn line_init(
    mut buf: *mut line,
    mut state: *mut line,
    mut initial_size: size_t,
) {
    (*buf)
        .text = (if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
        == 1 as libc::c_int as libc::c_ulong
    {
        xzalloc(initial_size.wrapping_add(1 as libc::c_int as libc::c_ulong))
    } else {
        xcalloc(
            initial_size.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        )
    }) as *mut libc::c_char;
    (*buf).active = (*buf).text;
    (*buf).alloc = initial_size;
    (*buf).length = 0 as libc::c_int as size_t;
    (*buf).chomped = 1 as libc::c_int != 0;
    if !state.is_null() {
        memcpy(
            &mut (*buf).mbstate as *mut mbstate_t as *mut libc::c_void,
            &mut (*state).mbstate as *mut mbstate_t as *const libc::c_void,
            ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
        );
    } else {
        memset(
            &mut (*buf).mbstate as *mut mbstate_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
        );
    };
}
unsafe extern "C" fn line_reset(mut buf: *mut line, mut state: *mut line) {
    if (*buf).alloc == 0 as libc::c_int as libc::c_ulong {
        line_init(buf, state, 50 as libc::c_int as size_t);
    } else {
        (*buf).length = 0 as libc::c_int as size_t;
        if !state.is_null() {
            memcpy(
                &mut (*buf).mbstate as *mut mbstate_t as *mut libc::c_void,
                &mut (*state).mbstate as *mut mbstate_t as *const libc::c_void,
                ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
            );
        } else {
            memset(
                &mut (*buf).mbstate as *mut mbstate_t as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
            );
        }
    };
}
unsafe extern "C" fn line_copy(
    mut from: *mut line,
    mut to: *mut line,
    mut state: libc::c_int,
) {
    (*to)
        .alloc = ((*to).alloc as libc::c_ulong)
        .wrapping_add(
            ((*to).active).offset_from((*to).text) as libc::c_long as libc::c_ulong,
        ) as size_t as size_t;
    if (*to).alloc < (*from).length {
        (*to)
            .alloc = ((*to).alloc as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        if (*to).alloc < (*from).length {
            (*to).alloc = (*from).length;
        }
        if (*to).alloc < 50 as libc::c_int as libc::c_ulong {
            (*to).alloc = 50 as libc::c_int as size_t;
        }
        rpl_free((*to).text as *mut libc::c_void);
        (*to)
            .text = (if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
            == 1 as libc::c_int as libc::c_ulong
        {
            xzalloc(((*to).alloc).wrapping_add(1 as libc::c_int as libc::c_ulong))
        } else {
            xcalloc(
                ((*to).alloc).wrapping_add(1 as libc::c_int as libc::c_ulong),
                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
            )
        }) as *mut libc::c_char;
    }
    (*to).active = (*to).text;
    (*to).length = (*from).length;
    (*to).chomped = (*from).chomped;
    memcpy(
        (*to).active as *mut libc::c_void,
        (*from).active as *const libc::c_void,
        (*from).length,
    );
    if state != 0 {
        memcpy(
            &mut (*to).mbstate as *mut mbstate_t as *mut libc::c_void,
            &mut (*from).mbstate as *mut mbstate_t as *const libc::c_void,
            ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
        );
    }
}
unsafe extern "C" fn line_append(
    mut from: *mut line,
    mut to: *mut line,
    mut state: libc::c_int,
) {
    str_append(to, &mut buffer_delimiter, 1 as libc::c_int as size_t);
    str_append(to, (*from).active, (*from).length);
    (*to).chomped = (*from).chomped;
    if state != 0 {
        memcpy(
            &mut (*to).mbstate as *mut mbstate_t as *mut libc::c_void,
            &mut (*from).mbstate as *mut mbstate_t as *const libc::c_void,
            ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
        );
    }
}
unsafe extern "C" fn line_exchange(
    mut a: *mut line,
    mut b: *mut line,
    mut state: libc::c_int,
) {
    let mut t: line = line {
        text: 0 as *const libc::c_char as *mut libc::c_char,
        active: 0 as *const libc::c_char as *mut libc::c_char,
        length: 0,
        alloc: 0,
        chomped: false,
        mbstate: mbstate_t {
            __count: 0,
            __value: C2RustUnnamed { __wch: 0 },
        },
    };
    if state != 0 {
        memcpy(
            &mut t as *mut line as *mut libc::c_void,
            a as *const libc::c_void,
            ::std::mem::size_of::<line>() as libc::c_ulong,
        );
        memcpy(
            a as *mut libc::c_void,
            b as *const libc::c_void,
            ::std::mem::size_of::<line>() as libc::c_ulong,
        );
        memcpy(
            b as *mut libc::c_void,
            &mut t as *mut line as *const libc::c_void,
            ::std::mem::size_of::<line>() as libc::c_ulong,
        );
    } else {
        memcpy(
            &mut t as *mut line as *mut libc::c_void,
            a as *const libc::c_void,
            36 as libc::c_ulong,
        );
        memcpy(a as *mut libc::c_void, b as *const libc::c_void, 36 as libc::c_ulong);
        memcpy(
            b as *mut libc::c_void,
            &mut t as *mut line as *const libc::c_void,
            36 as libc::c_ulong,
        );
    };
}
unsafe extern "C" fn read_always_fail(mut input: *mut input) -> bool {
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn read_file_line(mut input: *mut input) -> bool {
    static mut b: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut blen: size_t = 0;
    let mut result: libc::c_long = ck_getdelim(
        &mut b,
        &mut blen,
        buffer_delimiter,
        (*input).fp,
    ) as libc::c_long;
    if result <= 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int != 0;
    }
    if *b.offset((result - 1 as libc::c_int as libc::c_long) as isize) as libc::c_int
        == buffer_delimiter as libc::c_int
    {
        result -= 1;
        result;
    } else {
        line.chomped = 0 as libc::c_int != 0;
    }
    str_append(&mut line, b, result as size_t);
    return 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn output_missing_newline(mut outf: *mut output) {
    if (*outf).missing_newline {
        ck_fwrite(
            &mut buffer_delimiter as *mut libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
            1 as libc::c_int as size_t,
            (*outf).fp,
        );
        (*outf).missing_newline = 0 as libc::c_int != 0;
    }
}
#[inline]
unsafe extern "C" fn flush_output(mut fp: *mut FILE) {
    if unbuffered {
        ck_fflush(fp);
    }
}
unsafe extern "C" fn output_line(
    mut text: *const libc::c_char,
    mut length: size_t,
    mut nl: libc::c_int,
    mut outf: *mut output,
) {
    if text.is_null() {
        return;
    }
    output_missing_newline(outf);
    if length != 0 {
        ck_fwrite(
            text as *const libc::c_void,
            1 as libc::c_int as size_t,
            length,
            (*outf).fp,
        );
    }
    if nl != 0 {
        ck_fwrite(
            &mut buffer_delimiter as *mut libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
            1 as libc::c_int as size_t,
            (*outf).fp,
        );
    } else {
        (*outf).missing_newline = 1 as libc::c_int != 0;
    }
    flush_output((*outf).fp);
}
unsafe extern "C" fn next_append_slot() -> *mut append_queue {
    let mut n: *mut append_queue = (if ::std::mem::size_of::<append_queue>()
        as libc::c_ulong == 1 as libc::c_int as libc::c_ulong
    {
        xzalloc(1 as libc::c_int as size_t)
    } else {
        xcalloc(
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<append_queue>() as libc::c_ulong,
        )
    }) as *mut append_queue;
    (*n).fname = 0 as *const libc::c_char;
    (*n).text = 0 as *mut libc::c_char;
    (*n).textlen = 0 as libc::c_int as size_t;
    (*n).next = 0 as *mut append_queue;
    (*n).rpl_free = 0 as libc::c_int != 0;
    if !append_tail.is_null() {
        (*append_tail).next = n;
    } else {
        append_head = n;
    }
    append_tail = n;
    return append_tail;
}
unsafe extern "C" fn release_append_queue() {
    let mut p: *mut append_queue = 0 as *mut append_queue;
    let mut q: *mut append_queue = 0 as *mut append_queue;
    p = append_head;
    while !p.is_null() {
        if (*p).rpl_free {
            rpl_free((*p).text as *mut libc::c_void);
        }
        q = (*p).next;
        rpl_free(p as *mut libc::c_void);
        p = q;
    }
    append_tail = 0 as *mut append_queue;
    append_head = append_tail;
}
unsafe extern "C" fn print_file(mut infname: *const libc::c_char, mut outf: *mut FILE) {
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut cnt: size_t = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = ck_fopen(infname, read_mode, 0 as libc::c_int);
    if !fp.is_null() {
        loop {
            cnt = ck_fread(
                buf.as_mut_ptr() as *mut libc::c_void,
                1 as libc::c_int as size_t,
                ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
                fp,
            );
            if !(cnt > 0 as libc::c_int as libc::c_ulong) {
                break;
            }
            ck_fwrite(
                buf.as_mut_ptr() as *const libc::c_void,
                1 as libc::c_int as size_t,
                cnt,
                outf,
            );
        }
        ck_fclose(fp);
    }
}
unsafe extern "C" fn dump_append_queue() {
    let mut p: *mut append_queue = 0 as *mut append_queue;
    output_missing_newline(&mut output_file);
    p = append_head;
    while !p.is_null() {
        if !((*p).text).is_null() {
            ck_fwrite(
                (*p).text as *const libc::c_void,
                1 as libc::c_int as size_t,
                (*p).textlen,
                output_file.fp,
            );
        }
        if !((*p).fname).is_null() {
            print_file((*p).fname, output_file.fp);
        }
        p = (*p).next;
    }
    flush_output(output_file.fp);
    release_append_queue();
}
unsafe extern "C" fn get_backup_file_name(
    mut name: *const libc::c_char,
) -> *mut libc::c_char {
    let mut old_asterisk: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut asterisk: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut backup: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name_length: libc::c_int = strlen(name) as libc::c_int;
    let mut backup_length: libc::c_int = strlen(in_place_extension) as libc::c_int;
    asterisk = in_place_extension.offset(-(1 as libc::c_int as isize));
    old_asterisk = asterisk.offset(1 as libc::c_int as isize);
    loop {
        asterisk = strchr(old_asterisk, '*' as i32);
        if asterisk.is_null() {
            break;
        }
        backup_length += name_length - 1 as libc::c_int;
        old_asterisk = asterisk.offset(1 as libc::c_int as isize);
    }
    backup = xmalloc((backup_length + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    p = backup;
    asterisk = in_place_extension.offset(-(1 as libc::c_int as isize));
    old_asterisk = asterisk.offset(1 as libc::c_int as isize);
    loop {
        asterisk = strchr(old_asterisk, '*' as i32);
        if asterisk.is_null() {
            break;
        }
        memcpy(
            p as *mut libc::c_void,
            old_asterisk as *const libc::c_void,
            asterisk.offset_from(old_asterisk) as libc::c_long as libc::c_ulong,
        );
        p = p.offset(asterisk.offset_from(old_asterisk) as libc::c_long as isize);
        strcpy(p, name);
        p = p.offset(name_length as isize);
        old_asterisk = asterisk.offset(1 as libc::c_int as isize);
    }
    strcpy(p, old_asterisk);
    return backup;
}
unsafe extern "C" fn open_next_file(
    mut name: *const libc::c_char,
    mut input: *mut input,
) {
    buffer.length = 0 as libc::c_int as size_t;
    (*input).in_file_name = name;
    if *name.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
        && *name.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
        && in_place_extension.is_null()
    {
        clearerr_unlocked(stdin);
        (*input).fp = stdin;
    } else {
        if follow_symlinks {
            (*input).in_file_name = follow_symlink(name);
        }
        (*input).fp = ck_fopen(name, read_mode, 0 as libc::c_int);
        if ((*input).fp).is_null() {
            let mut ptr: *const libc::c_char = strerror(*__errno_location());
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: can't read %s: %s\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                program_name,
                name,
                ptr,
            );
            (*input)
                .read_fn = Some(
                read_always_fail as unsafe extern "C" fn(*mut input) -> bool,
            );
            (*input).bad_count = ((*input).bad_count).wrapping_add(1);
            (*input).bad_count;
            return;
        }
    }
    (*input).read_fn = Some(read_file_line as unsafe extern "C" fn(*mut input) -> bool);
    if !in_place_extension.is_null() {
        let mut input_fd: libc::c_int = 0;
        let mut tmpdir: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut old_fscreatecon: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut reset_fscreatecon: libc::c_int = 0 as libc::c_int;
        memset(
            &mut old_fscreatecon as *mut *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        );
        tmpdir = xstrdup((*input).in_file_name);
        p = strrchr(tmpdir, '/' as i32);
        if !p.is_null() {
            *p = 0 as libc::c_int as libc::c_char;
        } else {
            strcpy(tmpdir, b".\0" as *const u8 as *const libc::c_char);
        }
        if isatty(fileno((*input).fp)) != 0 {
            panic(
                dcgettext(
                    0 as *const libc::c_char,
                    b"couldn't edit %s: is a terminal\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*input).in_file_name,
            );
        }
        input_fd = fileno((*input).fp);
        fstat(input_fd, &mut (*input).st);
        if !((*input).st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint)
        {
            panic(
                dcgettext(
                    0 as *const libc::c_char,
                    b"couldn't edit %s: not a regular file\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*input).in_file_name,
            );
        }
        if is_selinux_enabled() > 0 as libc::c_int {
            let mut con: *mut libc::c_char = 0 as *mut libc::c_char;
            if rpl_lgetfilecon((*input).in_file_name, &mut con) != -(1 as libc::c_int) {
                reset_fscreatecon = (getfscreatecon(&mut old_fscreatecon)
                    >= 0 as libc::c_int) as libc::c_int;
                if setfscreatecon(con) < 0 as libc::c_int {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: warning: failed to set default file creation context to %s: %s\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        program_name,
                        con,
                        strerror(*__errno_location()),
                    );
                }
                freecon(con);
            } else if *__errno_location() != 38 as libc::c_int {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: warning: failed to get security context of %s: %s\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    program_name,
                    (*input).in_file_name,
                    strerror(*__errno_location()),
                );
            }
        }
        output_file
            .fp = ck_mkstemp(
            &mut (*input).out_file_name,
            tmpdir,
            b"sed\0" as *const u8 as *const libc::c_char,
            write_mode,
        );
        output_file.missing_newline = 0 as libc::c_int != 0;
        rpl_free(tmpdir as *mut libc::c_void);
        if reset_fscreatecon != 0 {
            setfscreatecon(old_fscreatecon);
            freecon(old_fscreatecon);
        }
        if (output_file.fp).is_null() {
            panic(
                dcgettext(
                    0 as *const libc::c_char,
                    b"couldn't open temporary file %s: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*input).out_file_name,
                strerror(*__errno_location()),
            );
        }
    } else {
        if !((*input).fp).is_null() && unbuffered as libc::c_int != 0 {
            setvbuf(
                (*input).fp,
                0 as *mut libc::c_char,
                2 as libc::c_int,
                0 as libc::c_int as size_t,
            );
        }
        output_file.fp = stdout;
    };
}
unsafe extern "C" fn closedown(mut input: *mut input) {
    (*input)
        .read_fn = Some(read_always_fail as unsafe extern "C" fn(*mut input) -> bool);
    if ((*input).fp).is_null() {
        return;
    }
    if !in_place_extension.is_null() && !(output_file.fp).is_null() {
        let mut target_name: *const libc::c_char = 0 as *const libc::c_char;
        let mut input_fd: libc::c_int = 0;
        let mut output_fd: libc::c_int = 0;
        target_name = (*input).in_file_name;
        input_fd = fileno((*input).fp);
        output_fd = fileno(output_file.fp);
        if fchown(output_fd, (*input).st.st_uid, (*input).st.st_gid)
            == -(1 as libc::c_int)
        {
            fchown(output_fd, -(1 as libc::c_int) as __uid_t, (*input).st.st_gid);
        }
        copy_acl(
            (*input).in_file_name,
            input_fd,
            (*input).out_file_name,
            output_fd,
            (*input).st.st_mode,
        );
        ck_fclose((*input).fp);
        ck_fclose(output_file.fp);
        if strcmp(in_place_extension, b"*\0" as *const u8 as *const libc::c_char)
            != 0 as libc::c_int
        {
            let mut backup_file_name: *mut libc::c_char = get_backup_file_name(
                target_name,
            );
            ck_rename(target_name, backup_file_name);
            rpl_free(backup_file_name as *mut libc::c_void);
        }
        ck_rename((*input).out_file_name, target_name);
        cancel_cleanup();
        rpl_free((*input).out_file_name as *mut libc::c_void);
    } else {
        ck_fclose((*input).fp);
    }
    (*input).fp = 0 as *mut FILE;
}
unsafe extern "C" fn reset_addresses(mut vec: *mut vector) {
    let mut cur_cmd: *mut sed_cmd = 0 as *mut sed_cmd;
    let mut n: libc::c_int = 0;
    cur_cmd = (*vec).v;
    n = (*vec).v_length as libc::c_int;
    loop {
        let fresh3 = n;
        n = n - 1;
        if !(fresh3 != 0) {
            break;
        }
        if !((*cur_cmd).a1).is_null()
            && (*(*cur_cmd).a1).addr_type as libc::c_uint
                == ADDR_IS_NUM as libc::c_int as libc::c_uint
            && (*(*cur_cmd).a1).addr_number == 0 as libc::c_int as libc::c_ulong
        {
            (*cur_cmd).range_state = RANGE_ACTIVE;
        } else {
            (*cur_cmd).range_state = RANGE_INACTIVE;
        }
        cur_cmd = cur_cmd.offset(1);
        cur_cmd;
    };
}
unsafe extern "C" fn read_pattern_space(
    mut input: *mut input,
    mut the_program: *mut vector,
    mut append: libc::c_int,
) -> bool {
    if !append_head.is_null() {
        dump_append_queue();
    }
    replaced = 0 as libc::c_int != 0;
    if append == 0 {
        line.length = 0 as libc::c_int as size_t;
    }
    line.chomped = 1 as libc::c_int != 0;
    while !(Some(((*input).read_fn).unwrap())).unwrap()(input) {
        closedown(input);
        if (*(*input).file_list).is_null() {
            return 0 as libc::c_int != 0;
        }
        if (*input).reset_at_next_file {
            (*input).line_number = 0 as libc::c_int as countT;
            hold.length = 0 as libc::c_int as size_t;
            reset_addresses(the_program);
            rewind_read_files();
            if !in_place_extension.is_null() {
                output_file.missing_newline = 0 as libc::c_int != 0;
            }
            (*input).reset_at_next_file = separate_files;
        }
        let fresh4 = (*input).file_list;
        (*input).file_list = ((*input).file_list).offset(1);
        open_next_file(*fresh4, input);
    }
    (*input).line_number = ((*input).line_number).wrapping_add(1);
    (*input).line_number;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn last_file_with_data_p(mut input: *mut input) -> bool {
    loop {
        let mut ch: libc::c_int = 0;
        closedown(input);
        if (*(*input).file_list).is_null() {
            return 1 as libc::c_int != 0;
        }
        let fresh5 = (*input).file_list;
        (*input).file_list = ((*input).file_list).offset(1);
        open_next_file(*fresh5, input);
        if !((*input).fp).is_null() {
            ch = getc_unlocked((*input).fp);
            if ch != -(1 as libc::c_int) {
                ungetc(ch, (*input).fp);
                return 0 as libc::c_int != 0;
            }
        }
    };
}
unsafe extern "C" fn test_eof(mut input: *mut input) -> bool {
    let mut ch: libc::c_int = 0;
    if buffer.length != 0 {
        return 0 as libc::c_int != 0;
    }
    if ((*input).fp).is_null() {
        return separate_files as libc::c_int != 0
            || last_file_with_data_p(input) as libc::c_int != 0;
    }
    if feof_unlocked((*input).fp) != 0 {
        return separate_files as libc::c_int != 0
            || last_file_with_data_p(input) as libc::c_int != 0;
    }
    ch = getc_unlocked((*input).fp);
    if ch == -(1 as libc::c_int) {
        return separate_files as libc::c_int != 0
            || last_file_with_data_p(input) as libc::c_int != 0;
    }
    ungetc(ch, (*input).fp);
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn match_an_address_p(
    mut addr: *mut addr,
    mut input: *mut input,
) -> bool {
    match (*addr).addr_type as libc::c_uint {
        0 => return 1 as libc::c_int != 0,
        1 => {
            return match_regex(
                (*addr).addr_regex,
                line.active,
                line.length,
                0 as libc::c_int as size_t,
                0 as *mut re_registers,
                0 as libc::c_int,
            ) != 0;
        }
        3 => {
            return (*input).line_number >= (*addr).addr_number
                && ((*input).line_number)
                    .wrapping_sub((*addr).addr_number)
                    .wrapping_rem((*addr).addr_step) == 0 as libc::c_int as libc::c_ulong;
        }
        4 | 5 => return (*addr).addr_number <= (*input).line_number,
        6 => return test_eof(input),
        2 => return (*addr).addr_number == (*input).line_number,
        _ => {
            panic(
                b"INTERNAL ERROR: bad address type\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn match_address_p(
    mut cmd: *mut sed_cmd,
    mut input: *mut input,
) -> bool {
    if ((*cmd).a1).is_null() {
        return 1 as libc::c_int != 0;
    }
    if (*cmd).range_state as libc::c_uint != RANGE_ACTIVE as libc::c_int as libc::c_uint
    {
        if ((*cmd).a2).is_null() {
            return match_an_address_p((*cmd).a1, input);
        }
        if (*(*cmd).a1).addr_type as libc::c_uint
            == ADDR_IS_NUM as libc::c_int as libc::c_uint
        {
            if (*cmd).range_state as libc::c_uint
                == RANGE_CLOSED as libc::c_int as libc::c_uint
                || (*input).line_number < (*(*cmd).a1).addr_number
            {
                return 0 as libc::c_int != 0;
            }
        } else if !match_an_address_p((*cmd).a1, input) {
            return 0 as libc::c_int != 0
        }
        (*cmd).range_state = RANGE_ACTIVE;
        match (*(*cmd).a2).addr_type as libc::c_uint {
            1 => return 1 as libc::c_int != 0,
            2 => {
                if (*input).line_number >= (*(*cmd).a2).addr_number {
                    (*cmd).range_state = RANGE_CLOSED;
                }
                return (*input).line_number <= (*(*cmd).a2).addr_number
                    || match_an_address_p((*cmd).a1, input) as libc::c_int != 0;
            }
            4 => {
                (*(*cmd).a2)
                    .addr_number = ((*input).line_number)
                    .wrapping_add((*(*cmd).a2).addr_step);
                return 1 as libc::c_int != 0;
            }
            5 => {
                (*(*cmd).a2)
                    .addr_number = ((*input).line_number)
                    .wrapping_add((*(*cmd).a2).addr_step)
                    .wrapping_sub(
                        ((*input).line_number).wrapping_rem((*(*cmd).a2).addr_step),
                    );
                return 1 as libc::c_int != 0;
            }
            _ => {}
        }
    }
    if (*(*cmd).a2).addr_type as libc::c_uint
        == ADDR_IS_NUM as libc::c_int as libc::c_uint
    {
        if (*input).line_number >= (*(*cmd).a2).addr_number {
            (*cmd).range_state = RANGE_CLOSED;
        }
        return (*input).line_number <= (*(*cmd).a2).addr_number;
    }
    if match_an_address_p((*cmd).a2, input) {
        (*cmd).range_state = RANGE_CLOSED;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn do_list(mut line_len: libc::c_int) {
    let mut p: *mut libc::c_uchar = line.active as *mut libc::c_uchar;
    let mut len: countT = line.length;
    let mut width: countT = 0 as libc::c_int as countT;
    let mut obuf: [libc::c_char; 180] = [0; 180];
    let mut o: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut olen: size_t = 0;
    let mut fp: *mut FILE = output_file.fp;
    output_missing_newline(&mut output_file);
    loop {
        let fresh6 = len;
        len = len.wrapping_sub(1);
        if !(fresh6 != 0) {
            break;
        }
        o = obuf.as_mut_ptr();
        if *p as libc::c_int & !(0x7f as libc::c_int) == 0 as libc::c_int
            && (1 as libc::c_int != 0
                && *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
                    & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0)
        {
            let fresh7 = o;
            o = o.offset(1);
            *fresh7 = *p as libc::c_char;
            if *p as libc::c_int == '\\' as i32 {
                let fresh8 = o;
                o = o.offset(1);
                *fresh8 = '\\' as i32 as libc::c_char;
            }
        } else {
            let fresh9 = o;
            o = o.offset(1);
            *fresh9 = '\\' as i32 as libc::c_char;
            match *p as libc::c_int {
                7 => {
                    let fresh10 = o;
                    o = o.offset(1);
                    *fresh10 = 'a' as i32 as libc::c_char;
                }
                8 => {
                    let fresh11 = o;
                    o = o.offset(1);
                    *fresh11 = 'b' as i32 as libc::c_char;
                }
                12 => {
                    let fresh12 = o;
                    o = o.offset(1);
                    *fresh12 = 'f' as i32 as libc::c_char;
                }
                10 => {
                    let fresh13 = o;
                    o = o.offset(1);
                    *fresh13 = 'n' as i32 as libc::c_char;
                }
                13 => {
                    let fresh14 = o;
                    o = o.offset(1);
                    *fresh14 = 'r' as i32 as libc::c_char;
                }
                9 => {
                    let fresh15 = o;
                    o = o.offset(1);
                    *fresh15 = 't' as i32 as libc::c_char;
                }
                11 => {
                    let fresh16 = o;
                    o = o.offset(1);
                    *fresh16 = 'v' as i32 as libc::c_char;
                }
                _ => {
                    sprintf(
                        o,
                        b"%03o\0" as *const u8 as *const libc::c_char,
                        *p as libc::c_int,
                    );
                    o = o.offset(strlen(o) as isize);
                }
            }
        }
        olen = o.offset_from(obuf.as_mut_ptr()) as libc::c_long as size_t;
        if width.wrapping_add(olen) >= line_len as libc::c_ulong
            && line_len > 0 as libc::c_int
        {
            ck_fwrite(
                b"\\\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
                1 as libc::c_int as size_t,
                fp,
            );
            ck_fwrite(
                &mut buffer_delimiter as *mut libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
                1 as libc::c_int as size_t,
                fp,
            );
            width = 0 as libc::c_int as countT;
        }
        ck_fwrite(
            obuf.as_mut_ptr() as *const libc::c_void,
            1 as libc::c_int as size_t,
            olen,
            fp,
        );
        width = (width as libc::c_ulong).wrapping_add(olen) as countT as countT;
        p = p.offset(1);
        p;
    }
    ck_fwrite(
        b"$\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
        1 as libc::c_int as size_t,
        fp,
    );
    ck_fwrite(
        &mut buffer_delimiter as *mut libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
        1 as libc::c_int as size_t,
        fp,
    );
    flush_output(fp);
}
unsafe extern "C" fn append_replacement(
    mut buf: *mut line,
    mut p: *mut replacement,
    mut regs: *mut re_registers,
) {
    let mut repl_mod: replacement_types = REPL_ASIS;
    while !p.is_null() {
        let mut i: libc::c_int = (*p).subst_id;
        let mut curr_type: replacement_types = REPL_ASIS;
        curr_type = (if (*p).repl_type as libc::c_uint
            & REPL_MODIFIERS as libc::c_int as libc::c_uint != 0
        {
            (*p).repl_type as libc::c_uint
        } else {
            (*p).repl_type as libc::c_uint | repl_mod as libc::c_uint
        }) as replacement_types;
        repl_mod = REPL_ASIS;
        if (*p).prefix_length != 0 {
            str_append_modified(buf, (*p).prefix, (*p).prefix_length, curr_type);
            curr_type = ::std::mem::transmute::<
                libc::c_uint,
                replacement_types,
            >(
                curr_type as libc::c_uint
                    & !(REPL_MODIFIERS as libc::c_int) as libc::c_uint,
            );
        }
        if 0 as libc::c_int <= i && (i as libc::c_ulong) < (*regs).num_regs {
            if *((*regs).end).offset(i as isize) == *((*regs).start).offset(i as isize)
                && (*p).repl_type as libc::c_uint
                    & REPL_MODIFIERS as libc::c_int as libc::c_uint != 0
            {
                repl_mod = (curr_type as libc::c_uint
                    & REPL_MODIFIERS as libc::c_int as libc::c_uint)
                    as replacement_types;
            } else if *((*regs).end).offset(i as isize)
                != *((*regs).start).offset(i as isize)
            {
                str_append_modified(
                    buf,
                    (line.active).offset(*((*regs).start).offset(i as isize) as isize),
                    (*((*regs).end).offset(i as isize)
                        - *((*regs).start).offset(i as isize)) as size_t,
                    curr_type,
                );
            }
        }
        p = (*p).next;
    }
}
unsafe extern "C" fn do_subst(mut sub: *mut subst) {
    let mut current_block: u64;
    let mut start: size_t = 0 as libc::c_int as size_t;
    let mut last_end: size_t = 0 as libc::c_int as size_t;
    let mut count: countT = 0 as libc::c_int as countT;
    let mut again: bool = 1 as libc::c_int != 0;
    static mut regs: re_registers = re_registers {
        num_regs: 0,
        start: 0 as *const regoff_t as *mut regoff_t,
        end: 0 as *const regoff_t as *mut regoff_t,
    };
    line_reset(&mut s_accum, &mut line);
    if match_regex(
        (*sub).regx,
        line.active,
        line.length,
        start,
        &mut regs,
        (*sub).max_id() as libc::c_int + 1 as libc::c_int,
    ) == 0
    {
        return;
    }
    if debug {
        if regs.num_regs > 0 as libc::c_int as libc::c_ulong
            && *(regs.start).offset(0 as libc::c_int as isize)
                != -(1 as libc::c_int) as libc::c_long
        {
            puts(b"MATCHED REGEX REGISTERS\0" as *const u8 as *const libc::c_char);
        }
        let mut i: libc::c_int = 0 as libc::c_int;
        while (i as libc::c_ulong) < regs.num_regs {
            if *(regs.start).offset(i as isize) == -(1 as libc::c_int) as libc::c_long {
                break;
            }
            printf(
                b"  regex[%d] = %d-%d '\0" as *const u8 as *const libc::c_char,
                i,
                *(regs.start).offset(i as isize) as libc::c_int,
                *(regs.end).offset(i as isize) as libc::c_int,
            );
            if *(regs.start).offset(i as isize) != *(regs.end).offset(i as isize) {
                if 0 != 0 && 0 != 0
                    && ((*(regs.end).offset(i as isize)
                        - *(regs.start).offset(i as isize)) as size_t)
                        .wrapping_mul(1 as libc::c_int as size_t)
                        <= 8 as libc::c_int as libc::c_ulong
                    && (*(regs.end).offset(i as isize)
                        - *(regs.start).offset(i as isize)) as size_t
                        != 0 as libc::c_int as libc::c_ulong
                {
                    ({
                        let mut __ptr: *const libc::c_char = (line.active)
                            .offset(*(regs.start).offset(i as isize) as isize)
                            as *const libc::c_char;
                        let mut __stream: *mut FILE = stdout;
                        let mut __cnt: size_t = 0;
                        __cnt = ((*(regs.end).offset(i as isize)
                            - *(regs.start).offset(i as isize)) as size_t)
                            .wrapping_mul(1 as libc::c_int as size_t);
                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                            let fresh17 = __ptr;
                            __ptr = __ptr.offset(1);
                            if putc_unlocked(*fresh17 as libc::c_int, __stream)
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
                        && (*(regs.end).offset(i as isize)
                            - *(regs.start).offset(i as isize)) as size_t
                            == 0 as libc::c_int as libc::c_ulong
                        || 0 != 0
                            && 1 as libc::c_int as size_t
                                == 0 as libc::c_int as libc::c_ulong
                    {} else {
                        fwrite_unlocked(
                            (line.active)
                                .offset(*(regs.start).offset(i as isize) as isize)
                                as *const libc::c_void,
                            (*(regs.end).offset(i as isize)
                                - *(regs.start).offset(i as isize)) as size_t,
                            1 as libc::c_int as size_t,
                            stdout,
                        );
                    };
                };
                0u8;
            }
            puts(b"'\0" as *const u8 as *const libc::c_char);
            i += 1;
            i;
        }
    }
    if ((*sub).replacement).is_null() && (*sub).numb <= 1 as libc::c_int as libc::c_ulong
    {
        if *(regs.start).offset(0 as libc::c_int as isize)
            == 0 as libc::c_int as libc::c_long && (*sub).global() == 0
        {
            replaced = 1 as libc::c_int != 0;
            line
                .active = (line.active)
                .offset(*(regs.end).offset(0 as libc::c_int as isize) as isize);
            line
                .length = (line.length as libc::c_ulong)
                .wrapping_sub(
                    *(regs.end).offset(0 as libc::c_int as isize) as libc::c_ulong,
                ) as size_t as size_t;
            line
                .alloc = (line.alloc as libc::c_ulong)
                .wrapping_sub(
                    *(regs.end).offset(0 as libc::c_int as isize) as libc::c_ulong,
                ) as size_t as size_t;
            current_block = 12354906759110788807;
        } else if *(regs.end).offset(0 as libc::c_int as isize) as libc::c_ulong
            == line.length
        {
            replaced = 1 as libc::c_int != 0;
            line.length = *(regs.start).offset(0 as libc::c_int as isize) as size_t;
            current_block = 12354906759110788807;
        } else {
            current_block = 15089075282327824602;
        }
    } else {
        current_block = 15089075282327824602;
    }
    match current_block {
        15089075282327824602 => {
            loop {
                let mut offset: size_t = *(regs.start).offset(0 as libc::c_int as isize)
                    as size_t;
                let mut matched: size_t = (*(regs.end).offset(0 as libc::c_int as isize)
                    - *(regs.start).offset(0 as libc::c_int as isize)) as size_t;
                if start < offset {
                    str_append(
                        &mut s_accum,
                        (line.active).offset(start as isize),
                        offset.wrapping_sub(start),
                    );
                    start = offset;
                }
                if (matched > 0 as libc::c_int as libc::c_ulong
                    || count == 0 as libc::c_int as libc::c_ulong || offset > last_end)
                    && {
                        count = count.wrapping_add(1);
                        count >= (*sub).numb
                    }
                {
                    replaced = 1 as libc::c_int != 0;
                    append_replacement(&mut s_accum, (*sub).replacement, &mut regs);
                    again = (*sub).global() != 0;
                } else {
                    if matched == 0 as libc::c_int as libc::c_ulong {
                        if !(start < line.length) {
                            break;
                        }
                        matched = 1 as libc::c_int as size_t;
                    }
                    str_append(
                        &mut s_accum,
                        (line.active).offset(offset as isize),
                        matched,
                    );
                }
                start = offset.wrapping_add(matched);
                last_end = *(regs.end).offset(0 as libc::c_int as isize) as size_t;
                if !(again as libc::c_int != 0 && start <= line.length
                    && match_regex(
                        (*sub).regx,
                        line.active,
                        line.length,
                        start,
                        &mut regs,
                        (*sub).max_id() as libc::c_int + 1 as libc::c_int,
                    ) != 0)
                {
                    break;
                }
            }
            if start < line.length {
                str_append(
                    &mut s_accum,
                    (line.active).offset(start as isize),
                    (line.length).wrapping_sub(start),
                );
            }
            s_accum.chomped = line.chomped;
            line_exchange(&mut line, &mut s_accum, 0 as libc::c_int);
            if count < (*sub).numb {
                return;
            }
        }
        _ => {}
    }
    if (*sub).print() as libc::c_int & 1 as libc::c_int != 0 {
        output_line(
            line.active,
            line.length,
            line.chomped as libc::c_int,
            &mut output_file,
        );
    }
    if (*sub).eval() != 0 {
        let mut pipe_fp: *mut FILE = 0 as *mut FILE;
        line_reset(&mut s_accum, 0 as *mut line);
        str_append(
            &mut line,
            b"\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
        pipe_fp = popen(line.active, b"r\0" as *const u8 as *const libc::c_char);
        if !pipe_fp.is_null() {
            while feof_unlocked(pipe_fp) == 0 {
                let mut buf: [libc::c_char; 4096] = [0; 4096];
                let mut n: libc::c_int = (if 0 != 0 && 0 != 0
                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                        .wrapping_mul(4096 as libc::c_int as size_t)
                        <= 8 as libc::c_int as libc::c_ulong
                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                        != 0 as libc::c_int as libc::c_ulong
                {
                    {
                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                        let mut __stream: *mut FILE = pipe_fp;
                        let mut __cnt: size_t = 0;
                        __cnt = (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                            .wrapping_mul(4096 as libc::c_int as size_t);
                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                            let mut __c: libc::c_int = getc_unlocked(__stream);
                            if __c == -(1 as libc::c_int) {
                                break;
                            }
                            let fresh18 = __ptr;
                            __ptr = __ptr.offset(1);
                            *fresh18 = __c as libc::c_char;
                            __cnt = __cnt.wrapping_sub(1);
                            __cnt;
                        }
                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                            .wrapping_mul(4096 as libc::c_int as size_t)
                            .wrapping_sub(__cnt)
                            .wrapping_div(
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            )
                    }
                } else if 0 != 0
                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                        == 0 as libc::c_int as libc::c_ulong
                    || 0 != 0
                        && 4096 as libc::c_int as size_t
                            == 0 as libc::c_int as libc::c_ulong
                {
                    0 as libc::c_int as size_t
                } else {
                    fread_unlocked(
                        buf.as_mut_ptr() as *mut libc::c_void,
                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        4096 as libc::c_int as size_t,
                        pipe_fp,
                    )
                }) as libc::c_int;
                if n > 0 as libc::c_int {
                    str_append(&mut s_accum, buf.as_mut_ptr(), n as size_t);
                }
            }
            pclose(pipe_fp);
            line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
            if line.length != 0
                && *(line.active)
                    .offset(
                        (line.length).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as libc::c_int == buffer_delimiter as libc::c_int
            {
                line.length = (line.length).wrapping_sub(1);
                line.length;
            }
        } else {
            panic(
                dcgettext(
                    0 as *const libc::c_char,
                    b"error in subprocess\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    }
    if (*sub).print() as libc::c_int & 2 as libc::c_int != 0 {
        output_line(
            line.active,
            line.length,
            line.chomped as libc::c_int,
            &mut output_file,
        );
    }
    if !((*sub).outf).is_null() {
        output_line(line.active, line.length, line.chomped as libc::c_int, (*sub).outf);
    }
}
unsafe extern "C" fn translate_mb(mut trans: *const *mut libc::c_char) {
    let mut idx: size_t = 0;
    let mut mbstate: mbstate_t = {
        let mut init = __mbstate_t {
            __count: 0 as libc::c_int,
            __value: C2RustUnnamed { __wch: 0 },
        };
        init
    };
    idx = 0 as libc::c_int as size_t;
    while idx < line.length {
        let mut i: libc::c_uint = 0;
        let mut mbclen: size_t = if mb_cur_max == 1 as libc::c_int {
            1 as libc::c_int as libc::c_ulong
        } else {
            rpl_mbrtowc(
                0 as *mut wchar_t,
                (line.active).offset(idx as isize),
                (line.length).wrapping_sub(idx),
                &mut mbstate,
            )
        };
        if mbclen == -(1 as libc::c_int) as size_t
            || mbclen == -(2 as libc::c_int) as size_t
            || mbclen == 0 as libc::c_int as libc::c_ulong
        {
            mbclen = 1 as libc::c_int as size_t;
        }
        i = 0 as libc::c_int as libc::c_uint;
        while !(*trans
            .offset((2 as libc::c_int as libc::c_uint).wrapping_mul(i) as isize))
            .is_null()
        {
            if strncmp(
                (line.active).offset(idx as isize),
                *trans
                    .offset((2 as libc::c_int as libc::c_uint).wrapping_mul(i) as isize),
                mbclen,
            ) == 0 as libc::c_int
            {
                let mut move_remain_buffer: bool = 0 as libc::c_int != 0;
                let mut tr: *const libc::c_char = *trans
                    .offset(
                        (2 as libc::c_int as libc::c_uint)
                            .wrapping_mul(i)
                            .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                    );
                let mut trans_len: size_t = if *tr as libc::c_int == '\0' as i32 {
                    1 as libc::c_int as libc::c_ulong
                } else {
                    strlen(tr)
                };
                if mbclen < trans_len {
                    let mut new_len: size_t = (line.length)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(trans_len)
                        .wrapping_sub(mbclen);
                    if line.alloc < new_len {
                        resize_line(&mut line, new_len);
                    }
                    move_remain_buffer = 1 as libc::c_int != 0;
                } else if mbclen > trans_len {
                    move_remain_buffer = 1 as libc::c_int != 0;
                }
                let mut prev_idx: size_t = idx;
                if move_remain_buffer {
                    let mut move_from: *const libc::c_char = (line.active)
                        .offset(idx as isize)
                        .offset(mbclen as isize);
                    let mut move_to: *mut libc::c_char = (line.active)
                        .offset(idx as isize)
                        .offset(trans_len as isize);
                    let mut move_len: size_t = (line.length)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(idx)
                        .wrapping_sub(mbclen);
                    let mut move_offset: size_t = trans_len.wrapping_sub(mbclen);
                    memmove(
                        move_to as *mut libc::c_void,
                        move_from as *const libc::c_void,
                        move_len,
                    );
                    line
                        .length = (line.length as libc::c_ulong)
                        .wrapping_add(move_offset) as size_t as size_t;
                    idx = (idx as libc::c_ulong).wrapping_add(move_offset) as size_t
                        as size_t;
                }
                memcpy(
                    (line.active).offset(prev_idx as isize) as *mut libc::c_void,
                    *trans
                        .offset(
                            (2 as libc::c_int as libc::c_uint)
                                .wrapping_mul(i)
                                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) as *const libc::c_void,
                    trans_len,
                );
                break;
            } else {
                i = i.wrapping_add(1);
                i;
            }
        }
        idx = (idx as libc::c_ulong).wrapping_add(mbclen) as size_t as size_t;
    }
}
unsafe extern "C" fn debug_print_end_of_cycle() {
    puts(b"END-OF-CYCLE:\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn debug_print_input(mut input: *const input) {
    let mut is_stdin: bool = !((*input).fp).is_null()
        && fileno((*input).fp) == 0 as libc::c_int;
    printf(
        b"INPUT:   '%s' line %lu\n\0" as *const u8 as *const libc::c_char,
        if is_stdin as libc::c_int != 0 {
            b"STDIN\0" as *const u8 as *const libc::c_char
        } else {
            (*input).in_file_name
        },
        (*input).line_number,
    );
}
unsafe extern "C" fn debug_print_line(mut ln: *mut line) {
    let mut src: *const libc::c_char = if !((*ln).active).is_null() {
        (*ln).active
    } else {
        (*ln).text
    };
    let mut l: size_t = (*ln).length;
    let mut p: *const libc::c_char = src;
    fputs_unlocked(
        if ln == &mut hold as *mut line {
            b"HOLD:    \0" as *const u8 as *const libc::c_char
        } else {
            b"PATTERN: \0" as *const u8 as *const libc::c_char
        },
        stdout,
    );
    loop {
        let fresh19 = l;
        l = l.wrapping_sub(1);
        if !(fresh19 != 0) {
            break;
        }
        let fresh20 = p;
        p = p.offset(1);
        debug_print_char(*fresh20);
    }
    putchar_unlocked('\n' as i32);
}
unsafe extern "C" fn execute_program(
    mut vec: *mut vector,
    mut input: *mut input,
) -> libc::c_int {
    let mut cur_cmd: *mut sed_cmd = 0 as *mut sed_cmd;
    let mut end_cmd: *mut sed_cmd = 0 as *mut sed_cmd;
    cur_cmd = (*vec).v;
    end_cmd = ((*vec).v).offset((*vec).v_length as isize);
    let mut current_block_146: u64;
    while cur_cmd < end_cmd {
        if debug {
            fputs_unlocked(b"COMMAND: \0" as *const u8 as *const libc::c_char, stdout);
            debug_print_command(vec, cur_cmd);
        }
        if match_address_p(cur_cmd, input) as libc::c_int
            != (*cur_cmd).addr_bang as libc::c_int
        {
            match (*cur_cmd).cmd as libc::c_int {
                97 => {
                    current_block_146 = 9305221327605986830;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                123 | 98 => {
                    current_block_146 = 17910695824078092359;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                125 | 35 | 58 => {}
                99 => {
                    current_block_146 = 18384948527928192890;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                100 => {
                    current_block_146 = 13862184400901987378;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                68 => {
                    current_block_146 = 5689001924483802034;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                101 => {
                    current_block_146 = 7172762164747879670;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                103 => {
                    current_block_146 = 10333858659598215949;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                71 => {
                    current_block_146 = 7357930174397065698;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                104 => {
                    current_block_146 = 6748814308805609410;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                72 => {
                    current_block_146 = 15860131741026162785;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                105 => {
                    current_block_146 = 11707886979257187789;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                108 => {
                    current_block_146 = 5004202709687294834;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                110 => {
                    current_block_146 = 9708990415886069118;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                78 => {
                    current_block_146 = 16801834866831430556;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                112 => {
                    current_block_146 = 6466194886500373342;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                80 => {
                    current_block_146 = 8030768351463208789;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                113 => {
                    current_block_146 = 9426422318600939105;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                81 => {
                    current_block_146 = 17409469539518602963;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                114 => {
                    current_block_146 = 13815734542218783736;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                82 => {
                    current_block_146 = 9933222667374193556;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                115 => {
                    current_block_146 = 17279545118392682146;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                116 => {
                    current_block_146 = 7073359417487352857;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                84 => {
                    current_block_146 = 1294092529152747548;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                119 => {
                    current_block_146 = 15932501721594529961;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                87 => {
                    current_block_146 = 16625017989975992022;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                120 => {
                    current_block_146 = 11046353700707405348;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                121 => {
                    current_block_146 = 9441948780229972447;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                122 => {
                    current_block_146 = 14580458282906420739;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                61 => {
                    current_block_146 = 1091571308080461497;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                70 => {
                    current_block_146 = 7372943784920105432;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                _ => {
                    current_block_146 = 15751339901929288031;
                    match current_block_146 {
                        7372943784920105432 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                (*input).in_file_name,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1091571308080461497 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*input).line_number,
                                buffer_delimiter as libc::c_int,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        14580458282906420739 => {
                            line.length = 0 as libc::c_int as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9441948780229972447 => {
                            if mb_cur_max > 1 as libc::c_int {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                p_2 = line.active as *mut libc::c_uchar;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11046353700707405348 => {
                            line_exchange(&mut line, &mut hold, 0 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1294092529152747548 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as libc::c_int != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7073359417487352857 => {
                            if replaced {
                                replaced = 0 as libc::c_int != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        17279545118392682146 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as libc::c_int as size_t,
                            );
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 1 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as libc::c_uint
                                    == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as libc::c_int,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9708990415886069118 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as libc::c_int != 0
                                || !read_pattern_space(input, vec, 0 as libc::c_int)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as libc::c_int);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15860131741026162785 => {
                            line_append(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6748814308805609410 => {
                            line_copy(&mut line, &mut hold, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7357930174397065698 => {
                            line_append(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        10333858659598215949 => {
                            line_copy(&mut hold, &mut line, 1 as libc::c_int);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const libc::c_char,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"error in subprocess\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = if 0 != 0 && 0 != 0
                                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(4096 as libc::c_int as size_t)
                                        <= 8 as libc::c_int as libc::c_ulong
                                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        != 0 as libc::c_int as libc::c_ulong
                                {
                                    {
                                        let mut __ptr: *mut libc::c_char = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t);
                                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                                            let mut __c: libc::c_int = getc_unlocked(__stream);
                                            if __c == -(1 as libc::c_int) {
                                                break;
                                            }
                                            let fresh21 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh21 = __c as libc::c_char;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_mul(4096 as libc::c_int as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    }
                                } else {
                                    if 0 != 0
                                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            == 0 as libc::c_int as libc::c_ulong
                                        || 0 != 0
                                            && 4096 as libc::c_int as size_t
                                                == 0 as libc::c_int as libc::c_ulong
                                    {
                                        0 as libc::c_int as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            4096 as libc::c_int as size_t,
                                            pipe_fp,
                                        )
                                    }
                                };
                                if n > 0 as libc::c_int as libc::c_ulong {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as libc::c_int as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == buffer_delimiter as libc::c_int
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as libc::c_int);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            if p.is_null() {
                                return -(1 as libc::c_int);
                            }
                            p = p.offset(1);
                            p;
                            line
                                .alloc = (line.alloc as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .length = (line.length as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(line.active) as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            line
                                .active = (line.active)
                                .offset(
                                    p.offset_from(line.active) as libc::c_long as isize,
                                );
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        9426422318600939105 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 17409469539518602963;
                        }
                        9305221327605986830 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        17910695824078092359 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        18384948527928192890 => {
                            if (*cur_cmd).range_state as libc::c_uint
                                != RANGE_ACTIVE as libc::c_int as libc::c_uint
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    1 as libc::c_int,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 13862184400901987378;
                        }
                        11707886979257187789 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1 as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        5004202709687294834 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as libc::c_ulong
                                }) as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as libc::c_int,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            let mut p_0: *mut libc::c_char = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as libc::c_int,
                                line.length,
                            ) as *mut libc::c_char;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as libc::c_long
                                        as libc::c_ulong
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() {
                                    1 as libc::c_int
                                } else {
                                    line.chomped as libc::c_int
                                },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13815734542218783736 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as libc::c_int) as libc::c_ulong {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as libc::c_int != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15932501721594529961 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as libc::c_int,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut libc::c_char = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as libc::c_int,
                                    line.length,
                                ) as *mut libc::c_char;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as libc::c_long
                                            as libc::c_ulong
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() {
                                        1 as libc::c_int
                                    } else {
                                        line.chomped as libc::c_int
                                    },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        15751339901929288031 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8
                                    as *const libc::c_char,
                                (*cur_cmd).cmd as libc::c_int,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                17409469539518602963 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as libc::c_int) {
                                        0 as libc::c_int
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
            }
        }
        cur_cmd = cur_cmd.offset(1);
        cur_cmd;
    }
    if debug {
        debug_print_end_of_cycle();
    }
    if !no_default_output {
        output_line(
            line.active,
            line.length,
            line.chomped as libc::c_int,
            &mut output_file,
        );
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn process_files(
    mut the_program: *mut vector,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    static mut dash: [libc::c_char; 2] = unsafe {
        *::std::mem::transmute::<&[u8; 2], &mut [libc::c_char; 2]>(b"-\0")
    };
    static mut stdin_argv: [*mut libc::c_char; 2] = unsafe {
        [dash.as_ptr() as *mut _, 0 as *const libc::c_char as *mut libc::c_char]
    };
    let mut input: input = input {
        file_list: 0 as *mut *mut libc::c_char,
        bad_count: 0,
        line_number: 0,
        reset_at_next_file: false,
        read_fn: None,
        out_file_name: 0 as *mut libc::c_char,
        in_file_name: 0 as *const libc::c_char,
        st: stat {
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
        },
        fp: 0 as *mut FILE,
        no_buffering: false,
    };
    let mut status: libc::c_int = 0;
    line_init(&mut line, 0 as *mut line, 50 as libc::c_int as size_t);
    line_init(&mut hold, 0 as *mut line, 0 as libc::c_int as size_t);
    line_init(&mut buffer, 0 as *mut line, 0 as libc::c_int as size_t);
    input.reset_at_next_file = 1 as libc::c_int != 0;
    if !argv.is_null() && !(*argv).is_null() {
        input.file_list = argv;
    } else if !in_place_extension.is_null() {
        panic(
            dcgettext(
                0 as *const libc::c_char,
                b"no input files\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else {
        input.file_list = stdin_argv.as_mut_ptr();
    }
    input.bad_count = 0 as libc::c_int as countT;
    input.line_number = 0 as libc::c_int as countT;
    input.read_fn = Some(read_always_fail as unsafe extern "C" fn(*mut input) -> bool);
    input.fp = 0 as *mut FILE;
    status = 0 as libc::c_int;
    while read_pattern_space(&mut input, the_program, 0 as libc::c_int) {
        if debug {
            debug_print_input(&mut input);
            debug_print_line(&mut line);
        }
        status = execute_program(the_program, &mut input);
        if !(status == -(1 as libc::c_int)) {
            break;
        }
        status = 0 as libc::c_int;
    }
    closedown(&mut input);
    if input.bad_count != 0 {
        status = EXIT_BAD_INPUT as libc::c_int;
    }
    return status;
}
