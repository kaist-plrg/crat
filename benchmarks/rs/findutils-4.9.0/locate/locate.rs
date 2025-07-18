use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type re_dfa_t;
    pub type quoting_options;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    fn rpl_fcntl(fd: libc::c_int, action: libc::c_int, _: ...) -> libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn setgroups(__n: size_t, __groups: *const __gid_t) -> libc::c_int;
    fn rpl_re_set_syntax(__syntax: reg_syntax_t) -> reg_syntax_t;
    fn rpl_re_compile_pattern(
        __pattern: *const libc::c_char,
        __length: size_t,
        __buffer: *mut re_pattern_buffer,
    ) -> *const libc::c_char;
    fn rpl_re_search(
        __buffer: *mut re_pattern_buffer,
        __String: *const libc::c_char,
        __length: regoff_t,
        __start: regoff_t,
        __range: regoff_t,
        __regs: *mut re_registers,
    ) -> regoff_t;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn abort() -> !;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn rpl_free(ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcasestr(
        __haystack: *const libc::c_char,
        __needle: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn mbscasestr(
        haystack: *const libc::c_char,
        needle: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn mbsstr(
        haystack: *const libc::c_char,
        needle: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn __lxstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn difftime(__time1: time_t, __time0: time_t) -> libc::c_double;
    fn time(__timer: *mut time_t) -> time_t;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn setuid(__uid: __uid_t) -> libc::c_int;
    fn setgid(__gid: __gid_t) -> libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn xstrtoumax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut uintmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
    fn fnmatch(
        __pattern: *const libc::c_char,
        __name: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
    static mut program_name: *const libc::c_char;
    fn set_program_name(argv0: *const libc::c_char);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn x2nrealloc(p: *mut libc::c_void, pn: *mut size_t, s: size_t) -> *mut libc::c_void;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn open_safer(_: *const libc::c_char, _: libc::c_int, _: ...) -> libc::c_int;
    fn human_readable(
        _: uintmax_t,
        _: *mut libc::c_char,
        _: libc::c_int,
        _: uintmax_t,
        _: uintmax_t,
    ) -> *mut libc::c_char;
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
    fn close_stdout();
    fn clone_quoting_options(o: *mut quoting_options) -> *mut quoting_options;
    fn quotearg_n_style(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn get_regex_type(s: *const libc::c_char) -> libc::c_int;
    fn xstrtol_fatal(
        _: strtol_error,
        _: libc::c_int,
        _: libc::c_char,
        _: *const option,
        _: *const libc::c_char,
    );
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn dcngettext(
        __domainname: *const libc::c_char,
        __msgid1: *const libc::c_char,
        __msgid2: *const libc::c_char,
        __n: libc::c_ulong,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn explain_how_to_report_bugs(
        f: *mut FILE,
        program_name_0: *const libc::c_char,
    ) -> libc::c_int;
    fn display_findutils_version(official_name: *const libc::c_char);
    fn getword(
        fp: *mut FILE,
        filename: *const libc::c_char,
        maxvalue: size_t,
        endian_state_flag: *mut GetwordEndianState,
    ) -> libc::c_int;
    fn print_quoted(
        fp: *mut FILE,
        qopts: *const quoting_options,
        dest_is_tty: bool,
        format: *const libc::c_char,
        s: *const libc::c_char,
    ) -> libc::c_int;
    fn splitstring(
        s: *const libc::c_char,
        separators: *const libc::c_char,
        first: bool,
        pos: *mut size_t,
        len: *mut size_t,
    ) -> bool;
}
pub type __uintmax_t = libc::c_ulong;
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
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type uintmax_t = __uintmax_t;
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const human_B: C2RustUnnamed_0 = 256;
pub const human_SI: C2RustUnnamed_0 = 128;
pub const human_space_before_unit: C2RustUnnamed_0 = 64;
pub const human_base_1024: C2RustUnnamed_0 = 32;
pub const human_autoscale: C2RustUnnamed_0 = 16;
pub const human_suppress_point_zero: C2RustUnnamed_0 = 8;
pub const human_group_digits: C2RustUnnamed_0 = 4;
pub const human_floor: C2RustUnnamed_0 = 2;
pub const human_round_to_nearest: C2RustUnnamed_0 = 1;
pub const human_ceiling: C2RustUnnamed_0 = 0;
pub type quoting_style = libc::c_uint;
pub const custom_quoting_style: quoting_style = 10;
pub const clocale_quoting_style: quoting_style = 9;
pub const locale_quoting_style: quoting_style = 8;
pub const escape_quoting_style: quoting_style = 7;
pub const c_maybe_quoting_style: quoting_style = 6;
pub const c_quoting_style: quoting_style = 5;
pub const shell_escape_always_quoting_style: quoting_style = 4;
pub const shell_escape_quoting_style: quoting_style = 3;
pub const shell_always_quoting_style: quoting_style = 2;
pub const shell_quoting_style: quoting_style = 1;
pub const literal_quoting_style: quoting_style = 0;
pub type GetwordEndianState = libc::c_uint;
pub const GetwordEndianStateSwab: GetwordEndianState = 2;
pub const GetwordEndianStateNative: GetwordEndianState = 1;
pub const GetwordEndianStateInitial: GetwordEndianState = 0;
pub type visit_result = libc::c_uint;
pub const VISIT_ABORT: visit_result = 8;
pub const VISIT_REJECTED: visit_result = 4;
pub const VISIT_ACCEPTED: visit_result = 2;
pub const VISIT_CONTINUE: visit_result = 1;
pub type ExistenceCheckType = libc::c_uint;
pub const ACCEPT_NON_EXISTING: ExistenceCheckType = 2;
pub const ACCEPT_EXISTING: ExistenceCheckType = 1;
pub const ACCEPT_EITHER: ExistenceCheckType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct locate_limits {
    pub limit: uintmax_t,
    pub items_accepted: uintmax_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct process_data {
    pub c: libc::c_int,
    pub count: libc::c_int,
    pub len: libc::c_int,
    pub original_filename: *mut libc::c_char,
    pub pathsize: size_t,
    pub munged_filename: *mut libc::c_char,
    pub fp: *mut FILE,
    pub dbfile: *const libc::c_char,
    pub endian_state: GetwordEndianState,
    pub bigram1: [libc::c_char; 128],
    pub bigram2: [libc::c_char; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct locate_stats {
    pub compressed_bytes: uintmax_t,
    pub total_filename_count: uintmax_t,
    pub total_filename_length: uintmax_t,
    pub whitespace_count: uintmax_t,
    pub newline_count: uintmax_t,
    pub highbit_filename_count: uintmax_t,
}
pub const TIME_BUF_LEN: C2RustUnnamed_1 = 20;
pub type C2RustUnnamed_1 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub uch: [libc::c_uchar; 4],
    pub ui: libc::c_uint,
}
pub type processfunc = Option::<unsafe extern "C" fn(*mut process_data) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct visitor {
    pub inspector: visitfunc,
    pub context: *mut libc::c_void,
    pub next: *mut visitor,
}
pub type visitfunc = Option::<
    unsafe extern "C" fn(*mut process_data, *mut libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regular_expression {
    pub regex: re_pattern_buffer,
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
pub const MAX_DB_AGE: C2RustUnnamed_11 = 129;
pub const REGEXTYPE_OPTION: C2RustUnnamed_11 = 128;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub _gl_dummy: libc::c_int,
}
pub type C2RustUnnamed_11 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub _gl_dummy: libc::c_int,
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
#[inline]
unsafe extern "C" fn lstat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __lxstat(1 as libc::c_int, __path, __statbuf);
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
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
static mut warn_number_units: libc::c_uint = 8 as libc::c_int as libc::c_uint;
static mut warn_name_units: [libc::c_char; 5] = unsafe {
    *::std::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"days\0")
};
static mut check_existence: ExistenceCheckType = ACCEPT_EITHER;
static mut follow_symlinks: libc::c_int = 1 as libc::c_int;
static mut separator: libc::c_int = '\n' as i32;
static mut quote_opts: *mut quoting_options = 0 as *const quoting_options
    as *mut quoting_options;
static mut stdout_is_a_tty: bool = false;
static mut print_quoted_filename: bool = false;
static mut results_were_filtered: bool = false;
static mut selected_secure_db: *const libc::c_char = 0 as *const libc::c_char;
unsafe extern "C" fn set_max_db_age(mut s: *const libc::c_char) {
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: libc::c_ulong = 0;
    if 0 as libc::c_int == *s as libc::c_int {
        if ::std::mem::size_of::<C2RustUnnamed_9>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"The argument for option --max-database-age must not be empty\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"The argument for option --max-database-age must not be empty\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    *__errno_location() = 0 as libc::c_int;
    val = strtoul(s, &mut end, 10 as libc::c_int);
    if (9223372036854775807 as libc::c_long as libc::c_ulong)
        .wrapping_mul(2 as libc::c_ulong)
        .wrapping_add(1 as libc::c_ulong) == val
        && 34 as libc::c_int == *__errno_location()
        || 0 as libc::c_int as libc::c_ulong == val
            && 22 as libc::c_int == *__errno_location()
    {
        if ::std::mem::size_of::<C2RustUnnamed_8>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"Invalid argument %s for option --max-database-age\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style(0 as libc::c_int, locale_quoting_style, s),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"Invalid argument %s for option --max-database-age\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style(0 as libc::c_int, locale_quoting_style, s),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    } else if *end != 0 {
        if ::std::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Invalid argument %s for option --max-database-age\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style(0 as libc::c_int, locale_quoting_style, s),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Invalid argument %s for option --max-database-age\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style(0 as libc::c_int, locale_quoting_style, s),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    } else {
        warn_number_units = val as libc::c_uint;
    };
}
unsafe extern "C" fn get_short(mut fp: *mut FILE) -> libc::c_short {
    let mut x: libc::c_short = 0;
    x = ((fgetc(fp) as libc::c_schar as libc::c_int) << 8 as libc::c_int)
        as libc::c_short;
    x = (x as libc::c_int | fgetc(fp) & 0xff as libc::c_int) as libc::c_short;
    return x;
}
static mut metacharacters: *const libc::c_char = b"*?[]\\\0" as *const u8
    as *const libc::c_char;
unsafe extern "C" fn contains_metacharacter(mut s: *const libc::c_char) -> libc::c_int {
    if (strpbrk(s, metacharacters)).is_null() {
        return 0 as libc::c_int
    } else {
        return 1 as libc::c_int
    };
}
unsafe extern "C" fn locate_read_str(
    mut buf: *mut *mut libc::c_char,
    mut siz: *mut size_t,
    mut fp: *mut FILE,
    mut delimiter: libc::c_int,
    mut offs: libc::c_int,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sz: size_t = 0 as libc::c_int as size_t;
    let mut nread: libc::c_int = 0;
    let mut needed: size_t = 0;
    nread = getdelim(&mut p, &mut sz, delimiter, fp) as libc::c_int;
    if nread >= 0 as libc::c_int {
        if !p.is_null() {} else {
            __assert_fail(
                b"p != NULL\0" as *const u8 as *const libc::c_char,
                b"locate.c\0" as *const u8 as *const libc::c_char,
                245 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"int locate_read_str(char **, size_t *, FILE *, int, int)\0"))
                    .as_ptr(),
            );
        }
        'c_10056: {
            if !p.is_null() {} else {
                __assert_fail(
                    b"p != NULL\0" as *const u8 as *const libc::c_char,
                    b"locate.c\0" as *const u8 as *const libc::c_char,
                    245 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 57],
                        &[libc::c_char; 57],
                    >(b"int locate_read_str(char **, size_t *, FILE *, int, int)\0"))
                        .as_ptr(),
                );
            }
        };
        needed = ((offs + nread) as libc::c_uint).wrapping_add(1 as libc::c_uint)
            as size_t;
        if needed > *siz {
            let mut pnew: *mut libc::c_char = realloc(*buf as *mut libc::c_void, needed)
                as *mut libc::c_char;
            if pnew.is_null() {
                return -(1 as libc::c_int)
            } else {
                *siz = needed;
                *buf = pnew;
            }
        }
        memcpy(
            (*buf).offset(offs as isize) as *mut libc::c_void,
            p as *const libc::c_void,
            (nread + 1 as libc::c_int) as libc::c_ulong,
        );
        rpl_free(p as *mut libc::c_void);
    }
    return nread;
}
static mut limits: locate_limits = locate_limits {
    limit: 0,
    items_accepted: 0,
};
static mut statistics: locate_stats = locate_stats {
    compressed_bytes: 0,
    total_filename_count: 0,
    total_filename_length: 0,
    whitespace_count: 0,
    newline_count: 0,
    highbit_filename_count: 0,
};
static mut inspectors: *mut visitor = 0 as *const visitor as *mut visitor;
static mut lastinspector: *mut visitor = 0 as *const visitor as *mut visitor;
static mut past_pat_inspector: *mut visitor = 0 as *const visitor as *mut visitor;
#[inline]
unsafe extern "C" fn visit(
    mut p: *const visitor,
    mut accept_flags: libc::c_int,
    mut procdata: *mut process_data,
    stop: *const visitor,
) -> libc::c_int {
    let mut result: libc::c_int = accept_flags;
    while accept_flags & result != 0 && stop != p {
        result = ((*p).inspector).unwrap()(procdata, (*p).context);
        p = (*p).next;
    }
    return result;
}
unsafe extern "C" fn process_simple(mut procdata: *mut process_data) -> libc::c_int {
    return visit(
        inspectors,
        VISIT_CONTINUE as libc::c_int | VISIT_ACCEPTED as libc::c_int,
        procdata,
        0 as *const visitor,
    );
}
unsafe extern "C" fn process_or(mut procdata: *mut process_data) -> libc::c_int {
    let mut result: libc::c_int = 0;
    result = visit(
        inspectors,
        VISIT_CONTINUE as libc::c_int | VISIT_REJECTED as libc::c_int,
        procdata,
        past_pat_inspector,
    );
    if result == VISIT_CONTINUE as libc::c_int {
        result = VISIT_REJECTED as libc::c_int;
    }
    if result & (VISIT_ABORT as libc::c_int | VISIT_REJECTED as libc::c_int) != 0 {
        return result;
    }
    result = visit(
        past_pat_inspector,
        VISIT_CONTINUE as libc::c_int,
        procdata,
        0 as *const visitor,
    );
    if VISIT_CONTINUE as libc::c_int == result {
        return VISIT_ACCEPTED as libc::c_int
    } else {
        return result
    };
}
unsafe extern "C" fn process_and(mut procdata: *mut process_data) -> libc::c_int {
    let mut result: libc::c_int = 0;
    result = visit(
        inspectors,
        VISIT_CONTINUE as libc::c_int | VISIT_ACCEPTED as libc::c_int,
        procdata,
        past_pat_inspector,
    );
    if result == VISIT_CONTINUE as libc::c_int {
        result = VISIT_REJECTED as libc::c_int;
    }
    if result & (VISIT_ABORT as libc::c_int | VISIT_REJECTED as libc::c_int) != 0 {
        return result;
    }
    result = visit(
        past_pat_inspector,
        VISIT_CONTINUE as libc::c_int,
        procdata,
        0 as *const visitor,
    );
    if VISIT_CONTINUE as libc::c_int == result {
        return VISIT_ACCEPTED as libc::c_int
    } else {
        return result
    };
}
static mut mainprocessor: processfunc = None;
unsafe extern "C" fn add_visitor(mut fn_0: visitfunc, mut context: *mut libc::c_void) {
    let mut p: *mut visitor = xmalloc(::std::mem::size_of::<visitor>() as libc::c_ulong)
        as *mut visitor;
    (*p).inspector = fn_0;
    (*p).context = context;
    (*p).next = 0 as *mut visitor;
    if lastinspector.is_null() {
        inspectors = p;
        lastinspector = inspectors;
    } else {
        (*lastinspector).next = p;
        lastinspector = p;
    };
}
unsafe extern "C" fn visit_justprint_quoted(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> libc::c_int {
    print_quoted(
        stdout,
        quote_opts,
        stdout_is_a_tty,
        b"%s\0" as *const u8 as *const libc::c_char,
        (*procdata).original_filename,
    );
    putchar(separator);
    return VISIT_CONTINUE as libc::c_int;
}
unsafe extern "C" fn visit_justprint_unquoted(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> libc::c_int {
    fputs((*procdata).original_filename, stdout);
    putchar(separator);
    return VISIT_CONTINUE as libc::c_int;
}
unsafe extern "C" fn toolong(mut procdata: *mut process_data) {
    if ::std::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
        error(
            1 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"locate database %s contains a filename longer than locate can handle\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*procdata).dbfile,
        );
        if 0 as libc::c_int != 0 {} else {
            unreachable!();
        };
    } else {
        error(
            1 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"locate database %s contains a filename longer than locate can handle\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*procdata).dbfile,
        );
        if 0 as libc::c_int != 0 {} else {
            unreachable!();
        };
    };
}
unsafe extern "C" fn extend(
    mut procdata: *mut process_data,
    mut siz1: size_t,
    mut siz2: size_t,
) {
    if (18446744073709551615 as libc::c_ulong).wrapping_sub(siz1) < siz2 {
        toolong(procdata);
    } else if (*procdata).pathsize < siz1.wrapping_add(siz2) {
        (*procdata).pathsize = siz1.wrapping_add(siz2);
        (*procdata)
            .original_filename = x2nrealloc(
            (*procdata).original_filename as *mut libc::c_void,
            &mut (*procdata).pathsize,
            1 as libc::c_int as size_t,
        ) as *mut libc::c_char;
    }
}
unsafe extern "C" fn visit_old_format(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> libc::c_int {
    let mut i: size_t = 0;
    if -(1 as libc::c_int) == (*procdata).c {
        return VISIT_ABORT as libc::c_int;
    }
    if (*procdata).c == 30 as libc::c_int {
        let mut minval: libc::c_int = 0;
        let mut maxval: libc::c_int = 0;
        let mut word: libc::c_int = 0;
        (*procdata).count -= 14 as libc::c_int;
        minval = 0 as libc::c_int - (*procdata).count;
        if (*procdata).count >= 0 as libc::c_int {
            maxval = (*procdata).len - (*procdata).count;
        } else {
            maxval = (*procdata).len - 0 as libc::c_int;
        }
        word = getword(
            (*procdata).fp,
            (*procdata).dbfile,
            maxval as size_t,
            &mut (*procdata).endian_state,
        );
        if word >= minval {} else {
            __assert_fail(
                b"word >= minval\0" as *const u8 as *const libc::c_char,
                b"locate.c\0" as *const u8 as *const libc::c_char,
                478 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"int visit_old_format(struct process_data *, void *)\0"))
                    .as_ptr(),
            );
        }
        'c_9342: {
            if word >= minval {} else {
                __assert_fail(
                    b"word >= minval\0" as *const u8 as *const libc::c_char,
                    b"locate.c\0" as *const u8 as *const libc::c_char,
                    478 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 52],
                        &[libc::c_char; 52],
                    >(b"int visit_old_format(struct process_data *, void *)\0"))
                        .as_ptr(),
                );
            }
        };
        (*procdata).count += word;
        if (*procdata).count >= 0 as libc::c_int {} else {
            __assert_fail(
                b"procdata->count >= 0\0" as *const u8 as *const libc::c_char,
                b"locate.c\0" as *const u8 as *const libc::c_char,
                480 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"int visit_old_format(struct process_data *, void *)\0"))
                    .as_ptr(),
            );
        }
        'c_9295: {
            if (*procdata).count >= 0 as libc::c_int {} else {
                __assert_fail(
                    b"procdata->count >= 0\0" as *const u8 as *const libc::c_char,
                    b"locate.c\0" as *const u8 as *const libc::c_char,
                    480 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 52],
                        &[libc::c_char; 52],
                    >(b"int visit_old_format(struct process_data *, void *)\0"))
                        .as_ptr(),
                );
            }
        };
    } else {
        (*procdata).count += (*procdata).c - 14 as libc::c_int;
        if (*procdata).count >= 0 as libc::c_int {} else {
            __assert_fail(
                b"procdata->count >= 0\0" as *const u8 as *const libc::c_char,
                b"locate.c\0" as *const u8 as *const libc::c_char,
                485 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"int visit_old_format(struct process_data *, void *)\0"))
                    .as_ptr(),
            );
        }
        'c_9233: {
            if (*procdata).count >= 0 as libc::c_int {} else {
                __assert_fail(
                    b"procdata->count >= 0\0" as *const u8 as *const libc::c_char,
                    b"locate.c\0" as *const u8 as *const libc::c_char,
                    485 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 52],
                        &[libc::c_char; 52],
                    >(b"int visit_old_format(struct process_data *, void *)\0"))
                        .as_ptr(),
                );
            }
        };
    }
    i = (*procdata).count as size_t;
    loop {
        (*procdata).c = getc((*procdata).fp);
        if !((*procdata).c > 30 as libc::c_int) {
            break;
        }
        if -(1 as libc::c_int) == (*procdata).c {
            break;
        }
        if (*procdata).c < 0o200 as libc::c_int {
            extend(procdata, i, 1 as libc::c_uint as size_t);
            let fresh0 = i;
            i = i.wrapping_add(1);
            *((*procdata).original_filename)
                .offset(fresh0 as isize) = (*procdata).c as libc::c_char;
        } else {
            extend(procdata, i, 2 as libc::c_uint as size_t);
            (*procdata).c &= 0o177 as libc::c_int;
            let fresh1 = i;
            i = i.wrapping_add(1);
            *((*procdata).original_filename)
                .offset(fresh1 as isize) = (*procdata).bigram1[(*procdata).c as usize];
            let fresh2 = i;
            i = i.wrapping_add(1);
            *((*procdata).original_filename)
                .offset(fresh2 as isize) = (*procdata).bigram2[(*procdata).c as usize];
        }
    }
    extend(procdata, i, 1 as libc::c_uint as size_t);
    *((*procdata).original_filename)
        .offset(i as isize) = 0 as libc::c_int as libc::c_char;
    (*procdata).len = i as libc::c_int;
    (*procdata).munged_filename = (*procdata).original_filename;
    return VISIT_CONTINUE as libc::c_int;
}
unsafe extern "C" fn visit_locate02_format(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> libc::c_int {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nread: libc::c_int = 0;
    if (*procdata).c == 0x80 as libc::c_int {
        (*procdata).count += get_short((*procdata).fp) as libc::c_int;
    } else if (*procdata).c > 127 as libc::c_int {
        (*procdata).count += (*procdata).c - 256 as libc::c_int;
    } else {
        (*procdata).count += (*procdata).c;
    }
    if (*procdata).count > (*procdata).len || (*procdata).count < 0 as libc::c_int {
        if ::std::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"locate database %s is corrupt or invalid\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style(
                    0 as libc::c_int,
                    locale_quoting_style,
                    (*procdata).dbfile,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"locate database %s is corrupt or invalid\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style(
                    0 as libc::c_int,
                    locale_quoting_style,
                    (*procdata).dbfile,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    nread = locate_read_str(
        &mut (*procdata).original_filename,
        &mut (*procdata).pathsize,
        (*procdata).fp,
        0 as libc::c_int,
        (*procdata).count,
    );
    if nread < 1 as libc::c_int {
        return VISIT_ABORT as libc::c_int;
    }
    (*procdata).c = getc((*procdata).fp);
    (*procdata).len = (*procdata).count + nread - 1 as libc::c_int;
    if (*procdata).len < 1 as libc::c_int {
        error(
            1 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"locate database %s is corrupt or invalid\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_n_style(0 as libc::c_int, locale_quoting_style, (*procdata).dbfile),
        );
    }
    s = ((*procdata).original_filename)
        .offset((*procdata).len as isize)
        .offset(-(1 as libc::c_int as isize));
    if *s.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {} else {
        __assert_fail(
            b"s[0] != '\\0'\0" as *const u8 as *const libc::c_char,
            b"locate.c\0" as *const u8 as *const libc::c_char,
            568 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"int visit_locate02_format(struct process_data *, void *)\0"))
                .as_ptr(),
        );
    }
    'c_9792: {
        if *s.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {} else {
            __assert_fail(
                b"s[0] != '\\0'\0" as *const u8 as *const libc::c_char,
                b"locate.c\0" as *const u8 as *const libc::c_char,
                568 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"int visit_locate02_format(struct process_data *, void *)\0"))
                    .as_ptr(),
            );
        }
    };
    if *s.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32 {} else {
        __assert_fail(
            b"s[1] == '\\0'\0" as *const u8 as *const libc::c_char,
            b"locate.c\0" as *const u8 as *const libc::c_char,
            569 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"int visit_locate02_format(struct process_data *, void *)\0"))
                .as_ptr(),
        );
    }
    'c_9748: {
        if *s.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32 {} else {
            __assert_fail(
                b"s[1] == '\\0'\0" as *const u8 as *const libc::c_char,
                b"locate.c\0" as *const u8 as *const libc::c_char,
                569 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"int visit_locate02_format(struct process_data *, void *)\0"))
                    .as_ptr(),
            );
        }
    };
    if *s.offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32 {} else {
        __assert_fail(
            b"s[2] == '\\0'\0" as *const u8 as *const libc::c_char,
            b"locate.c\0" as *const u8 as *const libc::c_char,
            570 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"int visit_locate02_format(struct process_data *, void *)\0"))
                .as_ptr(),
        );
    }
    'c_9702: {
        if *s.offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32 {} else {
            __assert_fail(
                b"s[2] == '\\0'\0" as *const u8 as *const libc::c_char,
                b"locate.c\0" as *const u8 as *const libc::c_char,
                570 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"int visit_locate02_format(struct process_data *, void *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*procdata).munged_filename = (*procdata).original_filename;
    return VISIT_CONTINUE as libc::c_int;
}
unsafe extern "C" fn visit_basename(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> libc::c_int {
    (*procdata).munged_filename = last_component((*procdata).original_filename);
    return VISIT_CONTINUE as libc::c_int;
}
unsafe extern "C" fn visit_existing_follow(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> libc::c_int {
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
    if stat((*procdata).original_filename, &mut st) != 0 as libc::c_int {
        return VISIT_REJECTED as libc::c_int
    } else {
        return VISIT_CONTINUE as libc::c_int
    };
}
unsafe extern "C" fn visit_non_existing_follow(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> libc::c_int {
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
    if stat((*procdata).original_filename, &mut st) == 0 as libc::c_int {
        return VISIT_REJECTED as libc::c_int
    } else {
        return VISIT_CONTINUE as libc::c_int
    };
}
unsafe extern "C" fn visit_existing_nofollow(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> libc::c_int {
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
    if lstat((*procdata).original_filename, &mut st) != 0 as libc::c_int {
        return VISIT_REJECTED as libc::c_int
    } else {
        return VISIT_CONTINUE as libc::c_int
    };
}
unsafe extern "C" fn visit_non_existing_nofollow(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> libc::c_int {
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
    if lstat((*procdata).original_filename, &mut st) == 0 as libc::c_int {
        return VISIT_REJECTED as libc::c_int
    } else {
        return VISIT_CONTINUE as libc::c_int
    };
}
unsafe extern "C" fn visit_substring_match_nocasefold_wide(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> libc::c_int {
    let mut pattern: *const libc::c_char = context as *const libc::c_char;
    if !(mbsstr((*procdata).munged_filename, pattern)).is_null() {
        return VISIT_ACCEPTED as libc::c_int
    } else {
        return VISIT_REJECTED as libc::c_int
    };
}
unsafe extern "C" fn visit_substring_match_nocasefold_narrow(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> libc::c_int {
    let mut pattern: *const libc::c_char = context as *const libc::c_char;
    if __ctype_get_mb_cur_max() == 1 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"MB_CUR_MAX == 1\0" as *const u8 as *const libc::c_char,
            b"locate.c\0" as *const u8 as *const libc::c_char,
            690 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"int visit_substring_match_nocasefold_narrow(struct process_data *, void *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_8339: {
        if __ctype_get_mb_cur_max() == 1 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"MB_CUR_MAX == 1\0" as *const u8 as *const libc::c_char,
                b"locate.c\0" as *const u8 as *const libc::c_char,
                690 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 75],
                    &[libc::c_char; 75],
                >(
                    b"int visit_substring_match_nocasefold_narrow(struct process_data *, void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !(strstr((*procdata).munged_filename, pattern)).is_null() {
        return VISIT_ACCEPTED as libc::c_int
    } else {
        return VISIT_REJECTED as libc::c_int
    };
}
unsafe extern "C" fn visit_substring_match_casefold_wide(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> libc::c_int {
    let mut pattern: *const libc::c_char = context as *const libc::c_char;
    if !(mbscasestr((*procdata).munged_filename, pattern)).is_null() {
        return VISIT_ACCEPTED as libc::c_int
    } else {
        return VISIT_REJECTED as libc::c_int
    };
}
unsafe extern "C" fn visit_substring_match_casefold_narrow(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> libc::c_int {
    let mut pattern: *const libc::c_char = context as *const libc::c_char;
    if __ctype_get_mb_cur_max() == 1 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"MB_CUR_MAX == 1\0" as *const u8 as *const libc::c_char,
            b"locate.c\0" as *const u8 as *const libc::c_char,
            714 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int visit_substring_match_casefold_narrow(struct process_data *, void *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_8415: {
        if __ctype_get_mb_cur_max() == 1 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"MB_CUR_MAX == 1\0" as *const u8 as *const libc::c_char,
                b"locate.c\0" as *const u8 as *const libc::c_char,
                714 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 73],
                    &[libc::c_char; 73],
                >(
                    b"int visit_substring_match_casefold_narrow(struct process_data *, void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !(strcasestr((*procdata).munged_filename, pattern)).is_null() {
        return VISIT_ACCEPTED as libc::c_int
    } else {
        return VISIT_REJECTED as libc::c_int
    };
}
unsafe extern "C" fn visit_globmatch_nofold(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> libc::c_int {
    let mut glob: *const libc::c_char = context as *const libc::c_char;
    if fnmatch(glob, (*procdata).munged_filename, 0 as libc::c_int) != 0 as libc::c_int {
        return VISIT_REJECTED as libc::c_int
    } else {
        return VISIT_ACCEPTED as libc::c_int
    };
}
unsafe extern "C" fn visit_globmatch_casefold(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> libc::c_int {
    let mut glob: *const libc::c_char = context as *const libc::c_char;
    if fnmatch(glob, (*procdata).munged_filename, (1 as libc::c_int) << 4 as libc::c_int)
        != 0 as libc::c_int
    {
        return VISIT_REJECTED as libc::c_int
    } else {
        return VISIT_ACCEPTED as libc::c_int
    };
}
unsafe extern "C" fn visit_regex(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> libc::c_int {
    let mut p: *mut regular_expression = context as *mut regular_expression;
    let len: size_t = strlen((*procdata).munged_filename);
    let mut rv: libc::c_int = rpl_re_search(
        &mut (*p).regex,
        (*procdata).munged_filename,
        len as regoff_t,
        0 as libc::c_int as regoff_t,
        len as regoff_t,
        0 as *mut libc::c_void as *mut re_registers,
    ) as libc::c_int;
    if rv < 0 as libc::c_int {
        return VISIT_REJECTED as libc::c_int
    } else {
        return VISIT_ACCEPTED as libc::c_int
    };
}
unsafe extern "C" fn visit_stats(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> libc::c_int {
    let mut p: *mut locate_stats = context as *mut locate_stats;
    let mut len: size_t = strlen((*procdata).original_filename);
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut highbit: libc::c_int = 0;
    let mut whitespace: libc::c_int = 0;
    let mut newline: libc::c_int = 0;
    (*p).total_filename_count = ((*p).total_filename_count).wrapping_add(1);
    (*p).total_filename_count;
    (*p)
        .total_filename_length = ((*p).total_filename_length as libc::c_ulong)
        .wrapping_add(len) as uintmax_t as uintmax_t;
    newline = 0 as libc::c_int;
    whitespace = newline;
    highbit = whitespace;
    s = (*procdata).original_filename;
    while *s != 0 {
        if *s as libc::c_int & 128 as libc::c_int != 0 {
            highbit = 1 as libc::c_int;
        }
        if '\n' as i32 == *s as libc::c_int {
            whitespace = 1 as libc::c_int;
            newline = whitespace;
        } else if *(*__ctype_b_loc()).offset(*s as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            whitespace = 1 as libc::c_int;
        }
        s = s.offset(1);
        s;
    }
    if highbit != 0 {
        (*p).highbit_filename_count = ((*p).highbit_filename_count).wrapping_add(1);
        (*p).highbit_filename_count;
    }
    if whitespace != 0 {
        (*p).whitespace_count = ((*p).whitespace_count).wrapping_add(1);
        (*p).whitespace_count;
    }
    if newline != 0 {
        (*p).newline_count = ((*p).newline_count).wrapping_add(1);
        (*p).newline_count;
    }
    return VISIT_CONTINUE as libc::c_int;
}
unsafe extern "C" fn visit_limit(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> libc::c_int {
    let mut p: *mut locate_limits = context as *mut locate_limits;
    (*p).items_accepted = ((*p).items_accepted).wrapping_add(1);
    if (*p).items_accepted >= (*p).limit {
        return VISIT_ABORT as libc::c_int
    } else {
        return VISIT_CONTINUE as libc::c_int
    };
}
unsafe extern "C" fn visit_count(
    mut procdata: *mut process_data,
    mut context: *mut libc::c_void,
) -> libc::c_int {
    let mut p: *mut locate_limits = context as *mut locate_limits;
    (*p).items_accepted = ((*p).items_accepted).wrapping_add(1);
    (*p).items_accepted;
    return VISIT_CONTINUE as libc::c_int;
}
unsafe extern "C" fn print_stats(
    mut argc: libc::c_int,
    mut database_file_size: size_t,
    mut database_mtime: *const timespec,
) {
    let mut hbuf1: [libc::c_char; 652] = [0; 652];
    let mut hbuf2: [libc::c_char; 652] = [0; 652];
    let mut hbuf3: [libc::c_char; 652] = [0; 652];
    let mut hbuf4: [libc::c_char; 652] = [0; 652];
    if !database_mtime.is_null() {
        let mut ptm: *const tm = localtime(&(*database_mtime).tv_sec);
        if !ptm.is_null() {
            let mut whenbuf: [libc::c_char; 20] = [0; 20];
            let mut printed: size_t = strftime(
                whenbuf.as_mut_ptr(),
                TIME_BUF_LEN as libc::c_int as size_t,
                b"%Y:%m:%d %H:%M:%S\0" as *const u8 as *const libc::c_char,
                ptm,
            );
            if printed
                == (TIME_BUF_LEN as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            {} else {
                __assert_fail(
                    b"printed == TIME_BUF_LEN-1\0" as *const u8 as *const libc::c_char,
                    b"locate.c\0" as *const u8 as *const libc::c_char,
                    845 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 55],
                        &[libc::c_char; 55],
                    >(b"void print_stats(int, size_t, const struct timespec *)\0"))
                        .as_ptr(),
                );
            }
            'c_7053: {
                if printed
                    == (TIME_BUF_LEN as libc::c_int - 1 as libc::c_int) as libc::c_ulong
                {} else {
                    __assert_fail(
                        b"printed == TIME_BUF_LEN-1\0" as *const u8
                            as *const libc::c_char,
                        b"locate.c\0" as *const u8 as *const libc::c_char,
                        845 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 55],
                            &[libc::c_char; 55],
                        >(b"void print_stats(int, size_t, const struct timespec *)\0"))
                            .as_ptr(),
                    );
                }
            };
            if whenbuf[(TIME_BUF_LEN as libc::c_int - 1 as libc::c_int) as usize]
                as libc::c_int == 0 as libc::c_int
            {} else {
                __assert_fail(
                    b"whenbuf[TIME_BUF_LEN-1] == 0\0" as *const u8
                        as *const libc::c_char,
                    b"locate.c\0" as *const u8 as *const libc::c_char,
                    846 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 55],
                        &[libc::c_char; 55],
                    >(b"void print_stats(int, size_t, const struct timespec *)\0"))
                        .as_ptr(),
                );
            }
            'c_7005: {
                if whenbuf[(TIME_BUF_LEN as libc::c_int - 1 as libc::c_int) as usize]
                    as libc::c_int == 0 as libc::c_int
                {} else {
                    __assert_fail(
                        b"whenbuf[TIME_BUF_LEN-1] == 0\0" as *const u8
                            as *const libc::c_char,
                        b"locate.c\0" as *const u8 as *const libc::c_char,
                        846 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 55],
                            &[libc::c_char; 55],
                        >(b"void print_stats(int, size_t, const struct timespec *)\0"))
                            .as_ptr(),
                    );
                }
            };
            if whenbuf[(TIME_BUF_LEN as libc::c_int - 2 as libc::c_int) as usize]
                as libc::c_int != 0 as libc::c_int
            {} else {
                __assert_fail(
                    b"whenbuf[TIME_BUF_LEN-2] != 0\0" as *const u8
                        as *const libc::c_char,
                    b"locate.c\0" as *const u8 as *const libc::c_char,
                    847 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 55],
                        &[libc::c_char; 55],
                    >(b"void print_stats(int, size_t, const struct timespec *)\0"))
                        .as_ptr(),
                );
            }
            'c_6956: {
                if whenbuf[(TIME_BUF_LEN as libc::c_int - 2 as libc::c_int) as usize]
                    as libc::c_int != 0 as libc::c_int
                {} else {
                    __assert_fail(
                        b"whenbuf[TIME_BUF_LEN-2] != 0\0" as *const u8
                            as *const libc::c_char,
                        b"locate.c\0" as *const u8 as *const libc::c_char,
                        847 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 55],
                            &[libc::c_char; 55],
                        >(b"void print_stats(int, size_t, const struct timespec *)\0"))
                            .as_ptr(),
                    );
                }
            };
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Database was last modified at %s.%09ld\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                whenbuf.as_mut_ptr(),
                (*database_mtime).tv_nsec,
            );
            printed = strftime(
                whenbuf.as_mut_ptr(),
                TIME_BUF_LEN as libc::c_int as size_t,
                b"%z\0" as *const u8 as *const libc::c_char,
                ptm,
            );
            if printed == 5 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"printed == 5\0" as *const u8 as *const libc::c_char,
                    b"locate.c\0" as *const u8 as *const libc::c_char,
                    851 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 55],
                        &[libc::c_char; 55],
                    >(b"void print_stats(int, size_t, const struct timespec *)\0"))
                        .as_ptr(),
                );
            }
            'c_6851: {
                if printed == 5 as libc::c_int as libc::c_ulong {} else {
                    __assert_fail(
                        b"printed == 5\0" as *const u8 as *const libc::c_char,
                        b"locate.c\0" as *const u8 as *const libc::c_char,
                        851 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 55],
                            &[libc::c_char; 55],
                        >(b"void print_stats(int, size_t, const struct timespec *)\0"))
                            .as_ptr(),
                    );
                }
            };
            printf(b" %s\n\0" as *const u8 as *const libc::c_char, whenbuf.as_mut_ptr());
        }
    }
    printf(
        dcngettext(
            0 as *const libc::c_char,
            b"Locate database size: %s byte\n\0" as *const u8 as *const libc::c_char,
            b"Locate database size: %s bytes\n\0" as *const u8 as *const libc::c_char,
            database_file_size,
            5 as libc::c_int,
        ),
        human_readable(
            database_file_size,
            hbuf1.as_mut_ptr(),
            human_ceiling as libc::c_int,
            1 as libc::c_int as uintmax_t,
            1 as libc::c_int as uintmax_t,
        ),
    );
    printf(
        if results_were_filtered as libc::c_int != 0 {
            dcgettext(
                0 as *const libc::c_char,
                b"Matching Filenames: %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            )
        } else {
            dcgettext(
                0 as *const libc::c_char,
                b"All Filenames: %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            )
        },
        human_readable(
            statistics.total_filename_count,
            hbuf1.as_mut_ptr(),
            human_ceiling as libc::c_int,
            1 as libc::c_int as uintmax_t,
            1 as libc::c_int as uintmax_t,
        ),
    );
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"File names have a cumulative length of %s bytes.\nOf those file names,\n\n\t%s contain whitespace, \n\t%s contain newline characters, \n\tand %s contain characters with the high bit set.\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        human_readable(
            statistics.total_filename_length,
            hbuf1.as_mut_ptr(),
            human_ceiling as libc::c_int,
            1 as libc::c_int as uintmax_t,
            1 as libc::c_int as uintmax_t,
        ),
        human_readable(
            statistics.whitespace_count,
            hbuf2.as_mut_ptr(),
            human_ceiling as libc::c_int,
            1 as libc::c_int as uintmax_t,
            1 as libc::c_int as uintmax_t,
        ),
        human_readable(
            statistics.newline_count,
            hbuf3.as_mut_ptr(),
            human_ceiling as libc::c_int,
            1 as libc::c_int as uintmax_t,
            1 as libc::c_int as uintmax_t,
        ),
        human_readable(
            statistics.highbit_filename_count,
            hbuf4.as_mut_ptr(),
            human_ceiling as libc::c_int,
            1 as libc::c_int as uintmax_t,
            1 as libc::c_int as uintmax_t,
        ),
    );
    if argc == 0 {
        if results_were_filtered {
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Some filenames may have been filtered out, so we cannot compute the compression ratio.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        } else if statistics.total_filename_length != 0 {
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Compression ratio %4.2f%% (higher is better)\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                100.0f64
                    * (statistics.total_filename_length as libc::c_double
                        - database_file_size as libc::c_double)
                    / statistics.total_filename_length as libc::c_double,
            );
        } else {
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Compression ratio is undefined\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn looking_at_gnu_locatedb(
    mut data: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    if len < ::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong {
        return 0 as libc::c_int
    } else if 0 as libc::c_int
        == memcmp(
            data as *const libc::c_void,
            b"\0LOCATE02\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
        )
    {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn looking_at_slocate_locatedb(
    mut filename: *const libc::c_char,
    mut data: *const libc::c_char,
    mut len: size_t,
    mut seclevel: *mut libc::c_int,
) -> libc::c_int {
    if len <= 2 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"len <= 2\0" as *const u8 as *const libc::c_char,
            b"locate.c\0" as *const u8 as *const libc::c_char,
            935 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"int looking_at_slocate_locatedb(const char *, const char *, size_t, int *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_10667: {
        if len <= 2 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"len <= 2\0" as *const u8 as *const libc::c_char,
                b"locate.c\0" as *const u8 as *const libc::c_char,
                935 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 75],
                    &[libc::c_char; 75],
                >(
                    b"int looking_at_slocate_locatedb(const char *, const char *, size_t, int *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if len < 2 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int
    } else if 0 as libc::c_int == *data.offset(1 as libc::c_int as isize) as libc::c_int
    {
        if *(*__ctype_b_loc())
            .offset(
                *data.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                    as isize,
            ) as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            *seclevel = *data.offset(0 as libc::c_int as isize) as libc::c_int
                - '0' as i32;
            if *seclevel > 1 as libc::c_int {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"locate database %s looks like an slocate database but it seems to have security level %c, which GNU findutils does not currently support\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_n_style(0 as libc::c_int, locale_quoting_style, filename),
                    *data.offset(1 as libc::c_int as isize) as libc::c_int,
                );
                return 1 as libc::c_int;
            } else {
                return 1 as libc::c_int
            }
        } else {
            return 0 as libc::c_int
        }
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn i_am_little_endian() -> libc::c_int {
    let mut u: C2RustUnnamed_2 = C2RustUnnamed_2 { uch: [0; 4] };
    u.ui = 0 as libc::c_uint;
    u.uch[0 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    u.uch[3 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    u.uch[2 as libc::c_int as usize] = u.uch[3 as libc::c_int as usize];
    u.uch[1 as libc::c_int as usize] = u.uch[2 as libc::c_int as usize];
    return (u.ui == 1 as libc::c_int as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn search_one_database(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut dbfile: *const libc::c_char,
    mut fp: *mut FILE,
    mut filesize: off_t,
    mut database_mtime: *const timespec,
    mut ignore_case: libc::c_int,
    mut enable_print: libc::c_int,
    mut basename_only: libc::c_int,
    mut use_limit: libc::c_int,
    mut plimit: *mut locate_limits,
    mut stats: libc::c_int,
    mut op_and: libc::c_int,
    mut regex: libc::c_int,
    mut regex_options: libc::c_int,
) -> libc::c_ulong {
    let mut pathpart: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut argn: libc::c_int = 0;
    let mut nread: libc::c_int = 0;
    let mut procdata: process_data = process_data {
        c: 0,
        count: 0,
        len: 0,
        original_filename: 0 as *mut libc::c_char,
        pathsize: 0,
        munged_filename: 0 as *mut libc::c_char,
        fp: 0 as *mut FILE,
        dbfile: 0 as *const libc::c_char,
        endian_state: GetwordEndianStateInitial,
        bigram1: [0; 128],
        bigram2: [0; 128],
    };
    let mut slocate_seclevel: libc::c_int = 0;
    let mut oldformat: libc::c_int = 0;
    let mut slocatedb_format: libc::c_int = 0;
    let mut pvis: *mut visitor = 0 as *mut visitor;
    let mut format_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut do_check_existence: ExistenceCheckType = ACCEPT_EITHER;
    do_check_existence = check_existence;
    if ignore_case != 0 {
        regex_options = (regex_options as libc::c_ulong
            | ((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) as libc::c_int;
    }
    oldformat = 0 as libc::c_int;
    procdata.endian_state = GetwordEndianStateInitial;
    procdata.count = 0 as libc::c_int;
    procdata.len = procdata.count;
    procdata.dbfile = dbfile;
    procdata.fp = fp;
    inspectors = 0 as *mut visitor;
    lastinspector = 0 as *mut visitor;
    past_pat_inspector = 0 as *mut visitor;
    results_were_filtered = 0 as libc::c_int != 0;
    procdata.pathsize = 128 as libc::c_int as size_t;
    procdata.original_filename = xmalloc(procdata.pathsize) as *mut libc::c_char;
    nread = fread(
        procdata.original_filename as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        2 as libc::c_int as libc::c_ulong,
        procdata.fp,
    ) as libc::c_int;
    slocate_seclevel = 0 as libc::c_int;
    if looking_at_slocate_locatedb(
        procdata.dbfile,
        procdata.original_filename,
        nread as size_t,
        &mut slocate_seclevel,
    ) != 0
    {
        if slocate_seclevel > 1 as libc::c_int {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is an slocate database of unsupported security level %d; skipping it.\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style(
                    0 as libc::c_int,
                    locale_quoting_style,
                    procdata.dbfile,
                ),
                slocate_seclevel,
            );
            return 0 as libc::c_int as libc::c_ulong;
        } else if slocate_seclevel > 0 as libc::c_int {
            if ACCEPT_NON_EXISTING as libc::c_int as libc::c_uint
                == check_existence as libc::c_uint
            {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"You specified the -E option, but that option cannot be used with slocate-format databases with a non-zero security level.  No results will be generated for this database.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                return 0 as libc::c_int as libc::c_ulong;
            }
            if ACCEPT_EXISTING as libc::c_int as libc::c_uint
                != do_check_existence as libc::c_uint
            {
                if enable_print != 0 || stats != 0 {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s is an slocate database.  Turning on the '-e' option.\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_n_style(
                            0 as libc::c_int,
                            locale_quoting_style,
                            procdata.dbfile,
                        ),
                    );
                }
                do_check_existence = ACCEPT_EXISTING;
            }
        }
        add_visitor(
            Some(
                visit_locate02_format
                    as unsafe extern "C" fn(
                        *mut process_data,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            0 as *mut libc::c_void,
        );
        format_name = b"slocate\0" as *const u8 as *const libc::c_char;
        slocatedb_format = 1 as libc::c_int;
    } else {
        let mut nread2: libc::c_int = 0;
        slocatedb_format = 0 as libc::c_int;
        extend(
            &mut procdata,
            ::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
            0 as libc::c_uint as size_t,
        );
        nread2 = fread(
            (procdata.original_filename).offset(nread as isize) as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                .wrapping_sub(nread as libc::c_ulong),
            procdata.fp,
        ) as libc::c_int;
        if looking_at_gnu_locatedb(
            procdata.original_filename,
            (nread + nread2) as size_t,
        ) != 0
        {
            add_visitor(
                Some(
                    visit_locate02_format
                        as unsafe extern "C" fn(
                            *mut process_data,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
                0 as *mut libc::c_void,
            );
            format_name = b"GNU LOCATE02\0" as *const u8 as *const libc::c_char;
        } else {
            let mut i: libc::c_int = 0;
            nread += nread2;
            extend(
                &mut procdata,
                256 as libc::c_uint as size_t,
                0 as libc::c_uint as size_t,
            );
            if nread < 256 as libc::c_int {
                let mut more_read: libc::c_int = fread(
                    (procdata.original_filename).offset(nread as isize)
                        as *mut libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    (256 as libc::c_int - nread) as libc::c_ulong,
                    procdata.fp,
                ) as libc::c_int;
                if more_read + nread != 256 as libc::c_int {
                    if ::std::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Old-format locate database %s is too short to be valid\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quotearg_n_style(
                                0 as libc::c_int,
                                locale_quoting_style,
                                dbfile,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Old-format locate database %s is too short to be valid\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quotearg_n_style(
                                0 as libc::c_int,
                                locale_quoting_style,
                                dbfile,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
            }
            i = 0 as libc::c_int;
            while i < 128 as libc::c_int {
                procdata
                    .bigram1[i
                    as usize] = *(procdata.original_filename)
                    .offset((i << 1 as libc::c_int) as isize);
                procdata
                    .bigram2[i
                    as usize] = *(procdata.original_filename)
                    .offset(((i << 1 as libc::c_int) + 1 as libc::c_int) as isize);
                i += 1;
                i;
            }
            format_name = b"old\0" as *const u8 as *const libc::c_char;
            oldformat = 1 as libc::c_int;
            add_visitor(
                Some(
                    visit_old_format
                        as unsafe extern "C" fn(
                            *mut process_data,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
                0 as *mut libc::c_void,
            );
        }
    }
    if basename_only != 0 {
        add_visitor(
            Some(
                visit_basename
                    as unsafe extern "C" fn(
                        *mut process_data,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            0 as *mut libc::c_void,
        );
    }
    argn = 0 as libc::c_int;
    while argn < argc {
        results_were_filtered = 1 as libc::c_int != 0;
        pathpart = *argv.offset(argn as isize);
        if regex != 0 {
            let mut p: *mut regular_expression = xmalloc(
                ::std::mem::size_of::<regular_expression>() as libc::c_ulong,
            ) as *mut regular_expression;
            let mut error_message: *const libc::c_char = 0 as *const libc::c_char;
            memset(
                &mut (*p).regex as *mut re_pattern_buffer as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<re_pattern_buffer>() as libc::c_ulong,
            );
            rpl_re_set_syntax(regex_options as reg_syntax_t);
            (*p).regex.allocated = 100 as libc::c_int as __re_long_size_t;
            (*p).regex.buffer = xmalloc((*p).regex.allocated) as *mut re_dfa_t;
            (*p).regex.fastmap = 0 as *mut libc::c_char;
            (*p).regex.syntax = regex_options as reg_syntax_t;
            (*p).regex.translate = 0 as *mut libc::c_uchar;
            error_message = rpl_re_compile_pattern(
                pathpart,
                strlen(pathpart),
                &mut (*p).regex,
            );
            if !error_message.is_null() {
                if ::std::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        error_message,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        error_message,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            } else {
                add_visitor(
                    Some(
                        visit_regex
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                    ),
                    p as *mut libc::c_void,
                );
            }
        } else if contains_metacharacter(pathpart) != 0 {
            if ignore_case != 0 {
                add_visitor(
                    Some(
                        visit_globmatch_casefold
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                    ),
                    pathpart as *mut libc::c_void,
                );
            } else {
                add_visitor(
                    Some(
                        visit_globmatch_nofold
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                    ),
                    pathpart as *mut libc::c_void,
                );
            }
        } else {
            let mut matcher: visitfunc = None;
            if 1 as libc::c_int as libc::c_ulong == __ctype_get_mb_cur_max() {
                matcher = if ignore_case != 0 {
                    Some(
                        visit_substring_match_casefold_narrow
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                    )
                } else {
                    Some(
                        visit_substring_match_nocasefold_narrow
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                    )
                };
            } else {
                matcher = if ignore_case != 0 {
                    Some(
                        visit_substring_match_casefold_wide
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                    )
                } else {
                    Some(
                        visit_substring_match_nocasefold_wide
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                    )
                };
            }
            add_visitor(matcher, pathpart as *mut libc::c_void);
        }
        argn += 1;
        argn;
    }
    pvis = lastinspector;
    match do_check_existence as libc::c_uint {
        1 => {
            results_were_filtered = 1 as libc::c_int != 0;
            if follow_symlinks != 0 {
                add_visitor(
                    Some(
                        visit_existing_follow
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                    ),
                    0 as *mut libc::c_void,
                );
            } else {
                add_visitor(
                    Some(
                        visit_existing_nofollow
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                    ),
                    0 as *mut libc::c_void,
                );
            }
        }
        2 => {
            results_were_filtered = 1 as libc::c_int != 0;
            if follow_symlinks != 0 {
                add_visitor(
                    Some(
                        visit_non_existing_follow
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                    ),
                    0 as *mut libc::c_void,
                );
            } else {
                add_visitor(
                    Some(
                        visit_non_existing_nofollow
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                    ),
                    0 as *mut libc::c_void,
                );
            }
        }
        0 | _ => {}
    }
    if stats != 0 {
        add_visitor(
            Some(
                visit_stats
                    as unsafe extern "C" fn(
                        *mut process_data,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            &mut statistics as *mut locate_stats as *mut libc::c_void,
        );
    }
    if enable_print != 0 {
        if print_quoted_filename {
            add_visitor(
                Some(
                    visit_justprint_quoted
                        as unsafe extern "C" fn(
                            *mut process_data,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
                0 as *mut libc::c_void,
            );
        } else {
            add_visitor(
                Some(
                    visit_justprint_unquoted
                        as unsafe extern "C" fn(
                            *mut process_data,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
                0 as *mut libc::c_void,
            );
        }
    }
    if use_limit != 0 {
        add_visitor(
            Some(
                visit_limit
                    as unsafe extern "C" fn(
                        *mut process_data,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            plimit as *mut libc::c_void,
        );
    } else {
        add_visitor(
            Some(
                visit_count
                    as unsafe extern "C" fn(
                        *mut process_data,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            plimit as *mut libc::c_void,
        );
    }
    if argc > 1 as libc::c_int {
        past_pat_inspector = (*pvis).next;
        if op_and != 0 {
            mainprocessor = Some(
                process_and as unsafe extern "C" fn(*mut process_data) -> libc::c_int,
            );
        } else {
            mainprocessor = Some(
                process_or as unsafe extern "C" fn(*mut process_data) -> libc::c_int,
            );
        }
    } else {
        mainprocessor = Some(
            process_simple as unsafe extern "C" fn(*mut process_data) -> libc::c_int,
        );
    }
    if stats != 0 {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Database %s is in the %s format.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            procdata.dbfile,
            format_name,
        );
    }
    procdata.c = getc(procdata.fp);
    if slocatedb_format != 0 && procdata.c != -(1 as libc::c_int) {
        ungetc(procdata.c, procdata.fp);
        procdata.c = 0 as libc::c_int;
    }
    while procdata.c != -(1 as libc::c_int)
        && VISIT_ABORT as libc::c_int != mainprocessor.unwrap()(&mut procdata)
    {}
    if stats != 0 {
        if oldformat != 0 {
            let mut host_little_endian: libc::c_int = i_am_little_endian();
            let mut little: *const libc::c_char = dcgettext(
                0 as *const libc::c_char,
                b"The database has little-endian machine-word encoding.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            );
            let mut big: *const libc::c_char = dcgettext(
                0 as *const libc::c_char,
                b"The database has big-endian machine-word encoding.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            );
            if GetwordEndianStateNative as libc::c_int as libc::c_uint
                == procdata.endian_state as libc::c_uint
            {
                printf(
                    b"%s\0" as *const u8 as *const libc::c_char,
                    if host_little_endian != 0 { little } else { big },
                );
            } else if GetwordEndianStateSwab as libc::c_int as libc::c_uint
                == procdata.endian_state as libc::c_uint
            {
                printf(
                    b"%s\0" as *const u8 as *const libc::c_char,
                    if host_little_endian != 0 { big } else { little },
                );
            } else {
                printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"The database machine-word encoding order is not obvious.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
        }
        if filesize != 0 || !database_mtime.is_null() {
            print_stats(argc, filesize as size_t, database_mtime);
        }
    }
    if ferror(procdata.fp) != 0 {
        error(
            0 as libc::c_int,
            *__errno_location(),
            b"%s\0" as *const u8 as *const libc::c_char,
            quotearg_n_style(0 as libc::c_int, locale_quoting_style, procdata.dbfile),
        );
        return 0 as libc::c_int as libc::c_ulong;
    }
    return (*plimit).items_accepted;
}
unsafe extern "C" fn usage(mut status: libc::c_int) -> ! {
    if status != 0 as libc::c_int {
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
        exit(status);
    }
    fprintf(
        stdout,
        dcgettext(
            0 as *const libc::c_char,
            b"Usage: %s [-d path | --database=path] [-e | -E | --[non-]existing]\n      [-i | --ignore-case] [-w | --wholename] [-b | --basename] \n      [--limit=N | -l N] [-S | --statistics] [-0 | --null] [-c | --count]\n      [-P | -H | --nofollow] [-L | --follow] [-m | --mmap] [-s | --stdio]\n      [-A | --all] [-p | --print] [-r | --regex] [--regextype=TYPE]\n      [--max-database-age D] [--version] [--help]\n      pattern...\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        program_name,
    );
    explain_how_to_report_bugs(stdout, program_name);
    exit(status);
}
static mut longopts: [option; 23] = [
    {
        let mut init = option {
            name: b"database\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"existing\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'e' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"non-existing\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'E' as i32,
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
            name: b"all\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'A' as i32,
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
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"null\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: '0' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"count\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"wholename\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'w' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"wholepath\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'w' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"basename\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"print\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"stdio\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"mmap\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'm' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"limit\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"regex\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"regextype\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: REGEXTYPE_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"statistics\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'S' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"follow\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'L' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"nofollow\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'P' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"max-database-age\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: MAX_DB_AGE as libc::c_int,
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
unsafe extern "C" fn drop_privs() -> libc::c_int {
    let mut current_block: u64;
    let mut what: *const libc::c_char = b"failed\0" as *const u8 as *const libc::c_char;
    let orig_euid: uid_t = geteuid();
    let uid: uid_t = getuid();
    let gid: gid_t = getgid();
    if 0 as libc::c_int as libc::c_uint == orig_euid {
        let mut groups: [gid_t; 1] = [0; 1];
        groups[0 as libc::c_int as usize] = gid;
        if 0 as libc::c_int
            != setgroups(1 as libc::c_uint as size_t, groups.as_mut_ptr())
        {
            what = dcgettext(
                0 as *const libc::c_char,
                b"failed to drop group privileges\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            current_block = 190573666211694471;
        } else {
            current_block = 6873731126896040597;
        }
    } else {
        current_block = 6873731126896040597;
    }
    match current_block {
        6873731126896040597 => {
            if uid != orig_euid {
                if 0 as libc::c_int as libc::c_uint == uid {
                    current_block = 17407779659766490442;
                } else {
                    *__errno_location() = 0 as libc::c_int;
                    if 0 as libc::c_int != setuid(getuid()) {
                        what = dcgettext(
                            0 as *const libc::c_char,
                            b"failed to drop setuid privileges\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        );
                        current_block = 190573666211694471;
                    } else if 0 as libc::c_int == setuid(0 as libc::c_int as __uid_t) {
                        what = dcgettext(
                            0 as *const libc::c_char,
                            b"Failed to fully drop privileges\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        );
                        *__errno_location() = 0 as libc::c_int;
                        current_block = 190573666211694471;
                    } else {
                        current_block = 17407779659766490442;
                    }
                }
            } else {
                current_block = 17407779659766490442;
            }
            match current_block {
                190573666211694471 => {}
                _ => {
                    *__errno_location() = 0 as libc::c_int;
                    if 0 as libc::c_int != setgid(gid) {
                        what = dcgettext(
                            0 as *const libc::c_char,
                            b"failed to drop setgid privileges\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        );
                    } else {
                        return 0 as libc::c_int
                    }
                }
            }
        }
        _ => {}
    }
    if ::std::mem::size_of::<C2RustUnnamed_12>() as libc::c_ulong != 0 {
        error(
            1 as libc::c_int,
            *__errno_location(),
            b"%s\0" as *const u8 as *const libc::c_char,
            quotearg_n_style(0 as libc::c_int, locale_quoting_style, what),
        );
        if 0 as libc::c_int != 0 {} else {
            unreachable!();
        };
    } else {
        error(
            1 as libc::c_int,
            *__errno_location(),
            b"%s\0" as *const u8 as *const libc::c_char,
            quotearg_n_style(0 as libc::c_int, locale_quoting_style, what),
        );
        if 0 as libc::c_int != 0 {} else {
            unreachable!();
        };
    };
    abort();
}
unsafe extern "C" fn opendb(mut name: *const libc::c_char) -> libc::c_int {
    let mut fd: libc::c_int = open_safer(name, 0 as libc::c_int | 0 as libc::c_int);
    if fd >= 0 as libc::c_int {
        if 0 as libc::c_int != rpl_fcntl(fd, 2 as libc::c_int, 1 as libc::c_int) {
            close(fd);
            fd = -(1 as libc::c_int);
        }
    }
    return fd;
}
unsafe extern "C" fn cleanup_quote_opts() {
    rpl_free(quote_opts as *mut libc::c_void);
}
unsafe extern "C" fn dolocate(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut secure_db_fd: libc::c_int,
) -> libc::c_int {
    let mut path_element: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path_element_pos: size_t = 0;
    let mut path_element_len: size_t = 0;
    let mut user_selected_locate_path: *const libc::c_char = 0 as *const libc::c_char;
    let mut db_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut path_separators: *const libc::c_char = b":\0" as *const u8
        as *const libc::c_char;
    let mut found: libc::c_ulong = 0 as libc::c_ulong;
    let mut ignore_case: libc::c_int = 0 as libc::c_int;
    let mut print: libc::c_int = 0 as libc::c_int;
    let mut just_count: libc::c_int = 0 as libc::c_int;
    let mut basename_only: libc::c_int = 0 as libc::c_int;
    let mut use_limit: libc::c_int = 0 as libc::c_int;
    let mut regex: libc::c_int = 0 as libc::c_int;
    let mut regex_options: libc::c_int = 0 as libc::c_int;
    let mut stats: libc::c_int = 0 as libc::c_int;
    let mut op_and: libc::c_int = 0 as libc::c_int;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut did_stdin: bool = 0 as libc::c_int != 0;
    if !(*argv.offset(0 as libc::c_int as isize)).is_null() {
        set_program_name(*argv.offset(0 as libc::c_int as isize));
    } else {
        set_program_name(b"locate\0" as *const u8 as *const libc::c_char);
    }
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"findutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"findutils\0" as *const u8 as *const libc::c_char);
    quote_opts = clone_quoting_options(0 as *mut quoting_options);
    if atexit(Some(close_stdout as unsafe extern "C" fn() -> ())) != 0
        || atexit(Some(cleanup_quote_opts as unsafe extern "C" fn() -> ())) != 0
    {
        if ::std::mem::size_of::<C2RustUnnamed_10>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"The atexit library function failed\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"The atexit library function failed\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    limits.limit = 0 as libc::c_int as uintmax_t;
    limits.items_accepted = 0 as libc::c_int as uintmax_t;
    print_quoted_filename = 1 as libc::c_int != 0;
    user_selected_locate_path = getenv(
        b"LOCATE_PATH\0" as *const u8 as *const libc::c_char,
    );
    check_existence = ACCEPT_EITHER;
    loop {
        let mut opti: libc::c_int = -(1 as libc::c_int);
        let mut optc: libc::c_int = getopt_long(
            argc,
            argv,
            b"Abcd:eEil:prsm0SwHPL\0" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            &mut opti,
        );
        if optc == -(1 as libc::c_int) {
            break;
        }
        match optc {
            48 => {
                separator = 0 as libc::c_int;
                print_quoted_filename = 0 as libc::c_int != 0;
            }
            65 => {
                op_and = 1 as libc::c_int;
            }
            98 => {
                basename_only = 1 as libc::c_int;
            }
            99 => {
                just_count = 1 as libc::c_int;
            }
            100 => {
                user_selected_locate_path = optarg;
                if !optarg.is_null() {} else {
                    __assert_fail(
                        b"optarg != NULL\0" as *const u8 as *const libc::c_char,
                        b"locate.c\0" as *const u8 as *const libc::c_char,
                        1613 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 32],
                            &[libc::c_char; 32],
                        >(b"int dolocate(int, char **, int)\0"))
                            .as_ptr(),
                    );
                }
                'c_12375: {
                    if !optarg.is_null() {} else {
                        __assert_fail(
                            b"optarg != NULL\0" as *const u8 as *const libc::c_char,
                            b"locate.c\0" as *const u8 as *const libc::c_char,
                            1613 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 32],
                                &[libc::c_char; 32],
                            >(b"int dolocate(int, char **, int)\0"))
                                .as_ptr(),
                        );
                    }
                };
            }
            101 => {
                check_existence = ACCEPT_EXISTING;
            }
            69 => {
                check_existence = ACCEPT_NON_EXISTING;
            }
            105 => {
                ignore_case = 1 as libc::c_int;
            }
            104 => {
                usage(0 as libc::c_int);
            }
            129 => {
                set_max_db_age(optarg);
            }
            112 => {
                print = 1 as libc::c_int;
            }
            118 => {
                display_findutils_version(
                    b"locate\0" as *const u8 as *const libc::c_char,
                );
                return 0 as libc::c_int;
            }
            119 => {
                basename_only = 0 as libc::c_int;
            }
            114 => {
                regex = 1 as libc::c_int;
            }
            128 => {
                regex_options = get_regex_type(optarg);
            }
            83 => {
                stats = 1 as libc::c_int;
            }
            76 => {
                follow_symlinks = 1 as libc::c_int;
            }
            80 | 72 => {
                follow_symlinks = 0 as libc::c_int;
            }
            108 => {
                let mut end: *mut libc::c_char = optarg;
                let mut err: strtol_error = xstrtoumax(
                    optarg,
                    &mut end,
                    10 as libc::c_int,
                    &mut limits.limit,
                    0 as *const libc::c_char,
                );
                if LONGINT_OK as libc::c_int as libc::c_uint != err as libc::c_uint {
                    xstrtol_fatal(
                        err,
                        opti,
                        optc as libc::c_char,
                        longopts.as_ptr(),
                        optarg,
                    );
                }
                use_limit = 1 as libc::c_int;
            }
            115 => {}
            109 => {}
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    if !user_selected_locate_path.is_null() {
        if secure_db_fd >= 0 as libc::c_int {
            close(secure_db_fd);
            secure_db_fd = -(1 as libc::c_int);
        }
    }
    if just_count == 0 && stats == 0 {
        print = 1 as libc::c_int;
    }
    if stats != 0 {
        if optind == argc {
            use_limit = 0 as libc::c_int;
        }
    } else if just_count == 0 && optind == argc {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"pattern argument expected\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(1 as libc::c_int);
    }
    if 1 as libc::c_int == isatty(1 as libc::c_int) {
        stdout_is_a_tty = 1 as libc::c_int != 0;
    } else {
        stdout_is_a_tty = 0 as libc::c_int != 0;
    }
    if !user_selected_locate_path.is_null() {
        splitstring(
            user_selected_locate_path,
            path_separators,
            1 as libc::c_int != 0,
            &mut path_element_pos,
            &mut path_element_len,
        );
    }
    while use_limit == 0 || limits.limit > limits.items_accepted {
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
        let mut database_mtime: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
        let mut have_mtime: libc::c_int = 0;
        let mut fd: libc::c_int = 0;
        let mut filesize: off_t = 0;
        statistics.highbit_filename_count = 0 as libc::c_uint as uintmax_t;
        statistics.newline_count = statistics.highbit_filename_count;
        statistics.whitespace_count = statistics.newline_count;
        statistics.total_filename_length = statistics.whitespace_count;
        statistics.total_filename_count = statistics.total_filename_length;
        statistics.compressed_bytes = statistics.total_filename_count;
        if !user_selected_locate_path.is_null() {
            if 1 as libc::c_int as libc::c_ulong == path_element_len
                && '-' as i32
                    == *user_selected_locate_path.offset(path_element_pos as isize)
                        as libc::c_int
            {
                if did_stdin {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"warning: the locate database can only be read from stdin once.\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    return 0 as libc::c_int;
                } else {
                    db_name = b"<stdin>\0" as *const u8 as *const libc::c_char;
                    fd = 0 as libc::c_int;
                    did_stdin = 1 as libc::c_int != 0;
                }
            } else {
                if 0 as libc::c_int as libc::c_ulong == path_element_len
                    || 1 as libc::c_int as libc::c_ulong == path_element_len
                        && '.' as i32
                            == *user_selected_locate_path
                                .offset(path_element_pos as isize) as libc::c_int
                {
                    db_name = b"/usr/local/var/locatedb\0" as *const u8
                        as *const libc::c_char;
                } else {
                    path_element = strndup(
                        &*user_selected_locate_path.offset(path_element_pos as isize),
                        path_element_len,
                    );
                    db_name = path_element;
                }
                fd = opendb(db_name);
                if fd < 0 as libc::c_int {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style(0 as libc::c_int, locale_quoting_style, db_name),
                    );
                    return 0 as libc::c_int;
                }
            }
        } else if -(1 as libc::c_int) == secure_db_fd {
            break;
        } else {
            db_name = selected_secure_db;
            fd = secure_db_fd;
            secure_db_fd = -(1 as libc::c_int);
        }
        if fstat(fd, &mut st) != 0 {
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style(0 as libc::c_int, locale_quoting_style, db_name),
            );
            filesize = 0 as libc::c_int as off_t;
            have_mtime = 0 as libc::c_int;
        } else {
            let mut now: time_t = 0;
            filesize = st.st_size;
            database_mtime = get_stat_mtime(&mut st);
            have_mtime = 1 as libc::c_int;
            if -(1 as libc::c_int) as time_t == time(&mut now) {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"time system call failed\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            } else {
                let mut age: libc::c_double = difftime(now, st.st_mtim.tv_sec);
                let mut warn_seconds: libc::c_double = ((60 as libc::c_int
                    * 60 as libc::c_int * 24 as libc::c_int) as libc::c_uint)
                    .wrapping_mul(warn_number_units) as libc::c_double;
                if age > warn_seconds {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"warning: database %s is more than %u %s old (actual age is %.1f %s)\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_n_style(
                            0 as libc::c_int,
                            locale_quoting_style,
                            db_name,
                        ),
                        warn_number_units,
                        dcgettext(
                            0 as *const libc::c_char,
                            warn_name_units.as_ptr(),
                            5 as libc::c_int,
                        ),
                        age
                            / (60 as libc::c_int * 60 as libc::c_int * 24 as libc::c_int)
                                as libc::c_double,
                        dcgettext(
                            0 as *const libc::c_char,
                            warn_name_units.as_ptr(),
                            5 as libc::c_int,
                        ),
                    );
                }
            }
        }
        fp = fdopen(fd, b"r\0" as *const u8 as *const libc::c_char);
        if fp.is_null() {
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style(0 as libc::c_int, locale_quoting_style, db_name),
            );
            return 0 as libc::c_int;
        }
        found = search_one_database(
            argc - optind,
            &mut *argv.offset(optind as isize),
            db_name,
            fp,
            filesize,
            if have_mtime != 0 { &mut database_mtime } else { 0 as *mut timespec },
            ignore_case,
            print,
            basename_only,
            use_limit,
            &mut limits,
            stats,
            op_and,
            regex,
            regex_options,
        );
        if rpl_fclose(fp) == -(1 as libc::c_int) {
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style(0 as libc::c_int, locale_quoting_style, db_name),
            );
            return 0 as libc::c_int;
        }
        if !path_element.is_null() {
            rpl_free(path_element as *mut libc::c_void);
            path_element = 0 as *mut libc::c_char;
        }
        if user_selected_locate_path.is_null() {
            break;
        }
        if !splitstring(
            user_selected_locate_path,
            path_separators,
            0 as libc::c_int != 0,
            &mut path_element_pos,
            &mut path_element_len,
        ) {
            break;
        }
    }
    if just_count != 0 {
        printf(b"%lu\n\0" as *const u8 as *const libc::c_char, found);
    }
    if found != 0 || use_limit != 0 && limits.limit == 0 as libc::c_int as libc::c_ulong
        || stats != 0
    {
        return 0 as libc::c_int
    } else {
        return 1 as libc::c_int
    };
}
unsafe extern "C" fn open_secure_db() -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut secure_db_list: [*const libc::c_char; 3] = [
        b"/usr/local/var/locatedb\0" as *const u8 as *const libc::c_char,
        b"/var/lib/slocate/slocate.db\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    i = 0 as libc::c_int;
    while !(secure_db_list[i as usize]).is_null() {
        fd = opendb(secure_db_list[i as usize]);
        if fd >= 0 as libc::c_int {
            selected_secure_db = secure_db_list[i as usize];
            return fd;
        }
        i += 1;
        i;
    }
    return -(1 as libc::c_int);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut dbfd: libc::c_int = open_secure_db();
    drop_privs();
    return dolocate(argc, argv, dbfd);
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
