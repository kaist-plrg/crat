use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type re_dfa_t;
    pub type dfa;
    pub type break_point;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn make_bool_node(value: bool) -> *mut NODE;
    static mut btowc_cache: [wint_t; 0];
    fn flags2str(_: libc::c_int) -> *const libc::c_char;
    fn nodetype2str(type_0: NODETYPE) -> *const libc::c_char;
    fn get_numbase(
        str: *const libc::c_char,
        len: size_t,
        use_locale: bool,
    ) -> libc::c_int;
    fn adjust_uint(n: uintmax_t) -> uintmax_t;
    fn do_split(nargs: libc::c_int) -> *mut NODE;
    fn do_patsplit(nargs: libc::c_int) -> *mut NODE;
    fn r_get_field(
        n: *mut NODE,
        assign: *mut Func_ptr,
        reference: bool,
    ) -> *mut *mut NODE;
    fn grow_stack() -> *mut STACK_ITEM;
    fn reset_record();
    fn refree(rp: *mut Regexp);
    fn make_typed_regex(re: *const libc::c_char, len: size_t) -> *mut NODE;
    static mut symbol_table: *mut NODE;
    static mut func_table: *mut NODE;
    fn re_update(t: *mut NODE) -> *mut Regexp;
    fn research(
        rp: *mut Regexp,
        str: *mut libc::c_char,
        start: libc::c_int,
        len: size_t,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn wstr2str(n: *mut NODE) -> *mut NODE;
    fn get_field(num: libc::c_long, assign: *mut Func_ptr) -> *mut *mut NODE;
    fn os_restore_mode(fd: libc::c_int);
    fn os_setbinmode(fd: libc::c_int, mode: libc::c_int) -> libc::c_int;
    fn redirect(
        redir_exp: *mut NODE,
        redirtype: libc::c_int,
        errflg: *mut libc::c_int,
        failure_fatal: bool,
    ) -> *mut redirect;
    fn close_rp(rp: *mut redirect, how: two_way_close_type) -> libc::c_int;
    fn gawk_fwrite(
        buf: *const libc::c_void,
        size: size_t,
        count: size_t,
        fp: *mut FILE,
        opaque: *mut libc::c_void,
    ) -> size_t;
    fn msg(mesg: *const libc::c_char, _: ...);
    fn make_str_node(
        s: *const libc::c_char,
        len: size_t,
        flags: libc::c_int,
    ) -> *mut NODE;
    fn gawk_exit(status: libc::c_int);
    fn r_dupnode(n: *mut NODE) -> *mut NODE;
    fn str2wstr(n: *mut NODE, ptr: *mut *mut size_t) -> *mut NODE;
    fn wcasestrstr(
        haystack: *const wchar_t,
        hs_len: size_t,
        needle: *const wchar_t,
        needle_len: size_t,
    ) -> *const wchar_t;
    fn wstrstr(
        haystack: *const wchar_t,
        hs_len: size_t,
        needle: *const wchar_t,
        needle_len: size_t,
    ) -> *const wchar_t;
    fn flush_io() -> libc::c_int;
    fn getredirect(str: *const libc::c_char, len: libc::c_int) -> *mut redirect;
    fn non_fatal_flush_std_file(fp: *mut FILE) -> bool;
    fn elem_new_to_scalar(n: *mut NODE) -> *mut NODE;
    static mut lintfunc: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
    fn r_warning(mesg: *const libc::c_char, _: ...);
    fn os_maybe_set_errno();
    fn is_non_fatal_redirect(str: *const libc::c_char, len: size_t) -> bool;
    fn is_non_fatal_std(fp: *mut FILE) -> bool;
    fn update_ERRNO_int(_: libc::c_int);
    fn set_loc(file: *const libc::c_char, line: libc::c_int);
    fn r_fatal(mesg: *const libc::c_char, _: ...);
    fn make_regnode(type_0: NODETYPE, exp_0: *mut NODE) -> *mut NODE;
    fn is_alpha(c: libc::c_int) -> bool;
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
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn mktime(__tp: *mut tm) -> time_t;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn gmtime(__timer: *const time_t) -> *mut tm;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn timegm(__tp: *mut tm) -> time_t;
    fn __errno_location() -> *mut libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn mbrtowc(
        __pwc: *mut wchar_t,
        __s: *const libc::c_char,
        __n: size_t,
        __p: *mut mbstate_t,
    ) -> size_t;
    fn wcrtomb(__s: *mut libc::c_char, __wc: wchar_t, __ps: *mut mbstate_t) -> size_t;
    fn __mbrlen(__s: *const libc::c_char, __n: size_t, __ps: *mut mbstate_t) -> size_t;
    fn iswlower(__wc: wint_t) -> libc::c_int;
    fn iswupper(__wc: wint_t) -> libc::c_int;
    fn towlower(__wc: wint_t) -> wint_t;
    fn towupper(__wc: wint_t) -> wint_t;
    fn getpid() -> __pid_t;
    static mut BINMODE: libc::c_int;
    static mut IGNORECASE: bool;
    static mut OFS: *mut libc::c_char;
    static mut OFSlen: libc::c_int;
    static mut ORS: *mut libc::c_char;
    static mut ORSlen: libc::c_int;
    static mut OFMT: *mut libc::c_char;
    static mut CONVFMT: *const libc::c_char;
    static mut CONVFMTidx: libc::c_int;
    static mut OFMTidx: libc::c_int;
    static mut TEXTDOMAIN: *mut libc::c_char;
    static mut FS_node: *mut NODE;
    static mut RLENGTH_node: *mut NODE;
    static mut RSTART_node: *mut NODE;
    static mut SUBSEP_node: *mut NODE;
    static mut PROCINFO_node: *mut NODE;
    static mut FPAT_node: *mut NODE;
    static mut Nnull_string: *mut NODE;
    static mut fields_arr: *mut *mut NODE;
    static mut make_number: Option::<unsafe extern "C" fn(libc::c_double) -> *mut NODE>;
    static mut str2number: Option::<unsafe extern "C" fn(*mut NODE) -> *mut NODE>;
    static mut format_val: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int, *mut NODE) -> *mut NODE,
    >;
    static mut nextfree: [block_header; 2];
    static mut field0_valid: bool;
    static mut do_flags: do_flag_values;
    static mut use_lc_numeric: libc::c_int;
    static mut gawk_mb_cur_max: libc::c_int;
    static mut loc: lconv;
    static def_strftime_format: [libc::c_char; 0];
    static mut casetable: [libc::c_char; 0];
    static mut stack_ptr: *mut STACK_ITEM;
    static mut stack_top: *mut STACK_ITEM;
    fn r_unref(tmp: *mut NODE);
    fn force_array(symbol: *mut NODE, canfatal: bool) -> *mut NODE;
    fn array_vname(symbol: *const NODE) -> *const libc::c_char;
    fn gawk_initstate(
        seed: libc::c_ulong,
        state_0: *mut libc::c_char,
        n: libc::c_long,
    ) -> *mut libc::c_char;
    fn gawk_setstate(state_0: *mut libc::c_char) -> *mut libc::c_char;
    fn gawk_random() -> libc::c_long;
    fn gawk_srandom(seed: libc::c_ulong);
    static mut args_array: *mut *mut NODE;
    static mut output_is_tty: bool;
    static mut output_fp: *mut FILE;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __intmax_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
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
pub type ssize_t = __ssize_t;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_0 = 8;
pub const _ISpunct: C2RustUnnamed_0 = 4;
pub const _IScntrl: C2RustUnnamed_0 = 2;
pub const _ISblank: C2RustUnnamed_0 = 1;
pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISprint: C2RustUnnamed_0 = 16384;
pub const _ISspace: C2RustUnnamed_0 = 8192;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISdigit: C2RustUnnamed_0 = 2048;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
pub const _ISlower: C2RustUnnamed_0 = 512;
pub const _ISupper: C2RustUnnamed_0 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lconv {
    pub decimal_point: *mut libc::c_char,
    pub thousands_sep: *mut libc::c_char,
    pub grouping: *mut libc::c_char,
    pub int_curr_symbol: *mut libc::c_char,
    pub currency_symbol: *mut libc::c_char,
    pub mon_decimal_point: *mut libc::c_char,
    pub mon_thousands_sep: *mut libc::c_char,
    pub mon_grouping: *mut libc::c_char,
    pub positive_sign: *mut libc::c_char,
    pub negative_sign: *mut libc::c_char,
    pub int_frac_digits: libc::c_char,
    pub frac_digits: libc::c_char,
    pub p_cs_precedes: libc::c_char,
    pub p_sep_by_space: libc::c_char,
    pub n_cs_precedes: libc::c_char,
    pub n_sep_by_space: libc::c_char,
    pub p_sign_posn: libc::c_char,
    pub n_sign_posn: libc::c_char,
    pub int_p_cs_precedes: libc::c_char,
    pub int_p_sep_by_space: libc::c_char,
    pub int_n_cs_precedes: libc::c_char,
    pub int_n_sep_by_space: libc::c_char,
    pub int_p_sign_posn: libc::c_char,
    pub int_n_sign_posn: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type time_t = __time_t;
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
pub type wchar_t = libc::c_int;
pub type wint_t = libc::c_uint;
pub type mbstate_t = __mbstate_t;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
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
pub type __re_size_t = libc::c_uint;
pub type __re_long_size_t = libc::c_ulong;
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
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub num_regs: __re_size_t,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Regexp {
    pub pat: re_pattern_buffer,
    pub regs: re_registers,
    pub dfareg: *mut dfa,
    pub has_meta: bool,
    pub maybe_long: bool,
}
pub type awk_bool = libc::c_uint;
pub const awk_true: awk_bool = 1;
pub const awk_false: awk_bool = 0;
pub type awk_bool_t = awk_bool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_fieldwidth_info_t {
    pub use_chars: awk_bool_t,
    pub nf: size_t,
    pub fields: [awk_field_info; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_field_info {
    pub skip: size_t,
    pub len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_input {
    pub name: *const libc::c_char,
    pub fd: libc::c_int,
    pub opaque: *mut libc::c_void,
    pub get_record: Option::<
        unsafe extern "C" fn(
            *mut *mut libc::c_char,
            *mut awk_input,
            *mut libc::c_int,
            *mut *mut libc::c_char,
            *mut size_t,
            *mut *const awk_fieldwidth_info_t,
        ) -> libc::c_int,
    >,
    pub read_func: Option::<
        unsafe extern "C" fn(libc::c_int, *mut libc::c_void, size_t) -> ssize_t,
    >,
    pub close_func: Option::<unsafe extern "C" fn(*mut awk_input) -> ()>,
    pub sbuf: stat,
}
pub type awk_input_buf_t = awk_input;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_output_buf {
    pub name: *const libc::c_char,
    pub mode: *const libc::c_char,
    pub fp: *mut FILE,
    pub redirected: awk_bool_t,
    pub opaque: *mut libc::c_void,
    pub gawk_fwrite: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            size_t,
            size_t,
            *mut FILE,
            *mut libc::c_void,
        ) -> size_t,
    >,
    pub gawk_fflush: Option::<
        unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> libc::c_int,
    >,
    pub gawk_ferror: Option::<
        unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> libc::c_int,
    >,
    pub gawk_fclose: Option::<
        unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> libc::c_int,
    >,
}
pub type awk_output_buf_t = awk_output_buf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_string {
    pub str_0: *mut libc::c_char,
    pub len: size_t,
}
pub type awk_string_t = awk_string;
pub type AWK_NUMBER_TYPE = libc::c_uint;
pub const AWK_NUMBER_TYPE_MPZ: AWK_NUMBER_TYPE = 2;
pub const AWK_NUMBER_TYPE_MPFR: AWK_NUMBER_TYPE = 1;
pub const AWK_NUMBER_TYPE_DOUBLE: AWK_NUMBER_TYPE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_number {
    pub d: libc::c_double,
    pub type_0: AWK_NUMBER_TYPE,
    pub ptr: *mut libc::c_void,
}
pub type awk_number_t = awk_number;
pub type awk_array_t = *mut libc::c_void;
pub type awk_scalar_t = *mut libc::c_void;
pub type awk_value_cookie_t = *mut libc::c_void;
pub type awk_valtype_t = libc::c_uint;
pub const AWK_BOOL: awk_valtype_t = 8;
pub const AWK_VALUE_COOKIE: awk_valtype_t = 7;
pub const AWK_SCALAR: awk_valtype_t = 6;
pub const AWK_ARRAY: awk_valtype_t = 5;
pub const AWK_STRNUM: awk_valtype_t = 4;
pub const AWK_REGEX: awk_valtype_t = 3;
pub const AWK_STRING: awk_valtype_t = 2;
pub const AWK_NUMBER: awk_valtype_t = 1;
pub const AWK_UNDEFINED: awk_valtype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_value {
    pub val_type: awk_valtype_t,
    pub u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub s: awk_string_t,
    pub n: awk_number_t,
    pub a: awk_array_t,
    pub scl: awk_scalar_t,
    pub vc: awk_value_cookie_t,
    pub b: awk_bool_t,
}
pub type awk_value_t = awk_value;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_ext_func {
    pub name: *const libc::c_char,
    pub function: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *mut awk_value_t,
            *mut awk_ext_func,
        ) -> *mut awk_value_t,
    >,
    pub max_expected_args: size_t,
    pub min_required_args: size_t,
    pub suppress_lint: awk_bool_t,
    pub data: *mut libc::c_void,
}
pub type awk_ext_func_t = awk_ext_func;
pub type nodevals = libc::c_uint;
pub const Node_final: nodevals = 19;
pub const Node_instruction: nodevals = 18;
pub const Node_frame: nodevals = 17;
pub const Node_arrayfor: nodevals = 16;
pub const Node_dump_array: nodevals = 15;
pub const Node_array_leaf: nodevals = 14;
pub const Node_array_tree: nodevals = 13;
pub const Node_array_ref: nodevals = 12;
pub const Node_builtin_func: nodevals = 11;
pub const Node_ext_func: nodevals = 10;
pub const Node_func: nodevals = 9;
pub const Node_param_list: nodevals = 8;
pub const Node_elem_new: nodevals = 7;
pub const Node_var_new: nodevals = 6;
pub const Node_var_array: nodevals = 5;
pub const Node_var: nodevals = 4;
pub const Node_dynregex: nodevals = 3;
pub const Node_regex: nodevals = 2;
pub const Node_val: nodevals = 1;
pub const Node_illegal: nodevals = 0;
pub type NODETYPE = nodevals;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exp_node {
    pub sub: C2RustUnnamed_2,
    pub type_0: NODETYPE,
    pub flags: flagvals,
    pub valref: libc::c_long,
}
pub type flagvals = libc::c_uint;
pub const REGEX: flagvals = 524288;
pub const NUMCONSTSTR: flagvals = 262144;
pub const XARRAY: flagvals = 131072;
pub const HALFHAT: flagvals = 65536;
pub const ARRAYMAXED: flagvals = 32768;
pub const NULL_FIELD: flagvals = 16384;
pub const NO_EXT_SET: flagvals = 8192;
pub const MPZN: flagvals = 4096;
pub const MPFN: flagvals = 2048;
pub const WSTRCUR: flagvals = 1024;
pub const INTIND: flagvals = 512;
pub const NUMINT: flagvals = 256;
pub const INTLSTR: flagvals = 128;
pub const BOOLVAL: flagvals = 64;
pub const USER_INPUT: flagvals = 32;
pub const NUMBER: flagvals = 16;
pub const NUMCUR: flagvals = 8;
pub const STRCUR: flagvals = 4;
pub const STRING: flagvals = 2;
pub const MALLOC: flagvals = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub nodep: C2RustUnnamed_4,
    pub val: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub fltnum: libc::c_double,
    pub sp: *mut libc::c_char,
    pub slen: size_t,
    pub idx: libc::c_int,
    pub wsp: *mut wchar_t,
    pub wslen: size_t,
    pub typre: *mut exp_node,
    pub comtype: commenttype,
}
pub type commenttype = libc::c_uint;
pub const FOR_COMMENT: commenttype = 3;
pub const BLOCK_COMMENT: commenttype = 2;
pub const EOL_COMMENT: commenttype = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub l: C2RustUnnamed_11,
    pub r: C2RustUnnamed_6,
    pub x: C2RustUnnamed_5,
    pub name: *mut libc::c_char,
    pub reserved: size_t,
    pub rn: *mut exp_node,
    pub cnt: libc::c_ulong,
    pub reflags: reflagvals,
}
pub type reflagvals = libc::c_uint;
pub const FS_DFLT: reflagvals = 2;
pub const CONSTANT: reflagvals = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub extra: *mut exp_node,
    pub aptr: Option::<unsafe extern "C" fn() -> ()>,
    pub xl: libc::c_long,
    pub cmnt: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub rptr: *mut exp_node,
    pub preg: [*mut Regexp; 2],
    pub av: *mut *mut exp_node,
    pub bv: *mut *mut BUCKET,
    pub uptr: Option::<unsafe extern "C" fn() -> ()>,
    pub iptr: *mut exp_instruction,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exp_instruction {
    pub nexti: *mut exp_instruction,
    pub d: C2RustUnnamed_8,
    pub x: C2RustUnnamed_7,
    pub comment: *mut exp_instruction,
    pub source_line: libc::c_short,
    pub pool_size: libc::c_short,
    pub opcode: OPCODE,
}
pub type OPCODE = opcodeval;
pub type opcodeval = libc::c_uint;
pub const Op_final: opcodeval = 122;
pub const Op_parens: opcodeval = 121;
pub const Op_cond_exp: opcodeval = 120;
pub const Op_K_function: opcodeval = 119;
pub const Op_K_else: opcodeval = 118;
pub const Op_K_if: opcodeval = 117;
pub const Op_K_switch: opcodeval = 116;
pub const Op_K_while: opcodeval = 115;
pub const Op_K_arrayfor: opcodeval = 114;
pub const Op_K_for: opcodeval = 113;
pub const Op_K_do: opcodeval = 112;
pub const Op_list: opcodeval = 111;
pub const Op_symbol: opcodeval = 110;
pub const Op_token: opcodeval = 109;
pub const Op_stop: opcodeval = 108;
pub const Op_atexit: opcodeval = 107;
pub const Op_lint_plus: opcodeval = 106;
pub const Op_lint: opcodeval = 105;
pub const Op_breakpoint: opcodeval = 104;
pub const Op_exec_count: opcodeval = 103;
pub const Op_comment: opcodeval = 102;
pub const Op_func: opcodeval = 101;
pub const Op_after_endfile: opcodeval = 100;
pub const Op_after_beginfile: opcodeval = 99;
pub const Op_subscript_assign: opcodeval = 98;
pub const Op_field_assign: opcodeval = 97;
pub const Op_var_assign: opcodeval = 96;
pub const Op_var_update: opcodeval = 95;
pub const Op_arrayfor_final: opcodeval = 94;
pub const Op_arrayfor_incr: opcodeval = 93;
pub const Op_arrayfor_init: opcodeval = 92;
pub const Op_newfile: opcodeval = 91;
pub const Op_get_record: opcodeval = 90;
pub const Op_jmp_false: opcodeval = 89;
pub const Op_jmp_true: opcodeval = 88;
pub const Op_jmp: opcodeval = 87;
pub const Op_pop: opcodeval = 86;
pub const Op_no_op: opcodeval = 85;
pub const Op_field_spec_lhs: opcodeval = 84;
pub const Op_subscript_lhs: opcodeval = 83;
pub const Op_push_lhs: opcodeval = 82;
pub const Op_push_param: opcodeval = 81;
pub const Op_push_array: opcodeval = 80;
pub const Op_push_re: opcodeval = 79;
pub const Op_push_i: opcodeval = 78;
pub const Op_push_arg_untyped: opcodeval = 77;
pub const Op_push_arg: opcodeval = 76;
pub const Op_push: opcodeval = 75;
pub const Op_indirect_func_call: opcodeval = 74;
pub const Op_func_call: opcodeval = 73;
pub const Op_in_array: opcodeval = 72;
pub const Op_ext_builtin: opcodeval = 71;
pub const Op_sub_builtin: opcodeval = 70;
pub const Op_builtin: opcodeval = 69;
pub const Op_K_namespace: opcodeval = 68;
pub const Op_K_nextfile: opcodeval = 67;
pub const Op_K_getline: opcodeval = 66;
pub const Op_K_getline_redir: opcodeval = 65;
pub const Op_K_delete_loop: opcodeval = 64;
pub const Op_K_delete: opcodeval = 63;
pub const Op_K_return_from_eval: opcodeval = 62;
pub const Op_K_return: opcodeval = 61;
pub const Op_K_exit: opcodeval = 60;
pub const Op_K_next: opcodeval = 59;
pub const Op_K_printf: opcodeval = 58;
pub const Op_K_print_rec: opcodeval = 57;
pub const Op_K_print: opcodeval = 56;
pub const Op_K_continue: opcodeval = 55;
pub const Op_K_break: opcodeval = 54;
pub const Op_K_default: opcodeval = 53;
pub const Op_K_case: opcodeval = 52;
pub const Op_rule: opcodeval = 51;
pub const Op_nomatch: opcodeval = 50;
pub const Op_match_rec: opcodeval = 49;
pub const Op_match: opcodeval = 48;
pub const Op_geq: opcodeval = 47;
pub const Op_leq: opcodeval = 46;
pub const Op_greater: opcodeval = 45;
pub const Op_less: opcodeval = 44;
pub const Op_notequal: opcodeval = 43;
pub const Op_equal: opcodeval = 42;
pub const Op_or_final: opcodeval = 41;
pub const Op_or: opcodeval = 40;
pub const Op_and_final: opcodeval = 39;
pub const Op_and: opcodeval = 38;
pub const Op_assign_concat: opcodeval = 37;
pub const Op_assign_exp: opcodeval = 36;
pub const Op_assign_minus: opcodeval = 35;
pub const Op_assign_plus: opcodeval = 34;
pub const Op_assign_mod: opcodeval = 33;
pub const Op_assign_quotient: opcodeval = 32;
pub const Op_assign_times: opcodeval = 31;
pub const Op_store_field_exp: opcodeval = 30;
pub const Op_store_field: opcodeval = 29;
pub const Op_store_sub: opcodeval = 28;
pub const Op_store_var: opcodeval = 27;
pub const Op_assign: opcodeval = 26;
pub const Op_not: opcodeval = 25;
pub const Op_field_spec: opcodeval = 24;
pub const Op_unary_plus: opcodeval = 23;
pub const Op_unary_minus: opcodeval = 22;
pub const Op_postdecrement: opcodeval = 21;
pub const Op_postincrement: opcodeval = 20;
pub const Op_predecrement: opcodeval = 19;
pub const Op_preincrement: opcodeval = 18;
pub const Op_sub_array: opcodeval = 17;
pub const Op_subscript: opcodeval = 16;
pub const Op_cond_pair: opcodeval = 15;
pub const Op_line_range: opcodeval = 14;
pub const Op_concat: opcodeval = 13;
pub const Op_exp_i: opcodeval = 12;
pub const Op_exp: opcodeval = 11;
pub const Op_minus_i: opcodeval = 10;
pub const Op_minus: opcodeval = 9;
pub const Op_plus_i: opcodeval = 8;
pub const Op_plus: opcodeval = 7;
pub const Op_mod_i: opcodeval = 6;
pub const Op_mod: opcodeval = 5;
pub const Op_quotient_i: opcodeval = 4;
pub const Op_quotient: opcodeval = 3;
pub const Op_times_i: opcodeval = 2;
pub const Op_times: opcodeval = 1;
pub const Op_illegal: opcodeval = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub xl: libc::c_long,
    pub xn: *mut NODE,
    pub aptr: Option::<unsafe extern "C" fn() -> ()>,
    pub xi: *mut exp_instruction,
    pub bpt: *mut break_point,
    pub exf: *mut awk_ext_func_t,
}
pub type NODE = exp_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub dn: *mut NODE,
    pub di: *mut exp_instruction,
    pub fptr: Option::<unsafe extern "C" fn(libc::c_int) -> *mut NODE>,
    pub efptr: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *mut awk_value_t,
            *mut awk_ext_func,
        ) -> *mut awk_value_t,
    >,
    pub dl: libc::c_long,
    pub ldl: exec_count_t,
    pub name: *mut libc::c_char,
}
pub type exec_count_t = libc::c_ulonglong;
pub type BUCKET = bucket_item;
#[derive(Copy, Clone)]
#[repr(C)]
pub union bucket_item {
    pub hs: C2RustUnnamed_10,
    pub hi: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub next: *mut bucket_item,
    pub li: [libc::c_long; 2],
    pub val: [*mut exp_node; 2],
    pub cnt: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub next: *mut bucket_item,
    pub str_0: *mut libc::c_char,
    pub len: size_t,
    pub code: size_t,
    pub name: *mut exp_node,
    pub val: *mut exp_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub lptr: *mut exp_node,
    pub li: *mut exp_instruction,
    pub ll: libc::c_long,
    pub lp: *const array_funcs_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array_funcs_t {
    pub name: *const libc::c_char,
    pub init: afunc_t,
    pub type_of: afunc_t,
    pub lookup: afunc_t,
    pub exists: afunc_t,
    pub clear: afunc_t,
    pub remove: afunc_t,
    pub list: afunc_t,
    pub copy: afunc_t,
    pub dump: afunc_t,
    pub store: afunc_t,
}
pub type afunc_t = Option::<
    unsafe extern "C" fn(*mut exp_node, *mut exp_node) -> *mut *mut exp_node,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iobuf {
    pub public: awk_input_buf_t,
    pub buf: *mut libc::c_char,
    pub off: *mut libc::c_char,
    pub dataend: *mut libc::c_char,
    pub end: *mut libc::c_char,
    pub readsize: size_t,
    pub size: size_t,
    pub count: ssize_t,
    pub scanoff: size_t,
    pub valid: bool,
    pub errcode: libc::c_int,
    pub flag: iobuf_flags,
}
pub type iobuf_flags = libc::c_uint;
pub const IOP_AT_START: iobuf_flags = 8;
pub const IOP_CLOSED: iobuf_flags = 4;
pub const IOP_AT_EOF: iobuf_flags = 2;
pub const IOP_IS_TTY: iobuf_flags = 1;
pub type IOBUF = iobuf;
pub type Func_ptr = Option::<unsafe extern "C" fn() -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redirect {
    pub flag: redirect_flags,
    pub value: *mut libc::c_char,
    pub ifp: *mut FILE,
    pub iop: *mut IOBUF,
    pub pid: libc::c_int,
    pub status: libc::c_int,
    pub prev: *mut redirect,
    pub next: *mut redirect,
    pub mode: *const libc::c_char,
    pub output: awk_output_buf_t,
}
pub type redirect_flags = libc::c_uint;
pub const RED_TCP: redirect_flags = 2048;
pub const RED_SOCKET: redirect_flags = 1024;
pub const RED_PTY: redirect_flags = 512;
pub const RED_TWOWAY: redirect_flags = 256;
pub const RED_EOF: redirect_flags = 128;
pub const RED_USED: redirect_flags = 64;
pub const RED_FLUSH: redirect_flags = 32;
pub const RED_APPEND: redirect_flags = 16;
pub const RED_WRITE: redirect_flags = 8;
pub const RED_READ: redirect_flags = 4;
pub const RED_PIPE: redirect_flags = 2;
pub const RED_FILE: redirect_flags = 1;
pub const RED_NONE: redirect_flags = 0;
pub type binmode_values = libc::c_uint;
pub const BINMODE_BOTH: binmode_values = 3;
pub const BINMODE_OUTPUT: binmode_values = 2;
pub const BINMODE_INPUT: binmode_values = 1;
pub const TEXT_TRANSLATE: binmode_values = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct block_item {
    pub freep: *mut block_item,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct block_header {
    pub freep: *mut block_item,
    pub size: size_t,
    pub name: *const libc::c_char,
    pub highwater: libc::c_long,
}
pub type block_id = libc::c_uint;
pub const BLOCK_MAX: block_id = 2;
pub const BLOCK_BUCKET: block_id = 1;
pub const BLOCK_NODE: block_id = 0;
pub type do_flag_values = libc::c_uint;
pub const DO_MPFR: do_flag_values = 32768;
pub const DO_DEBUG: do_flag_values = 16384;
pub const DO_PROFILE: do_flag_values = 8192;
pub const DO_SANDBOX: do_flag_values = 4096;
pub const DO_TIDY_MEM: do_flag_values = 2048;
pub const DO_DUMP_VARS: do_flag_values = 1024;
pub const DO_PRETTY_PRINT: do_flag_values = 512;
pub const DO_INTERVALS: do_flag_values = 256;
pub const DO_NON_DEC_DATA: do_flag_values = 128;
pub const DO_INTL: do_flag_values = 64;
pub const DO_POSIX: do_flag_values = 32;
pub const DO_TRADITIONAL: do_flag_values = 16;
pub const DO_LINT_OLD: do_flag_values = 8;
pub const DO_LINT_ALL: do_flag_values = 4;
pub const DO_LINT_EXTENSIONS: do_flag_values = 2;
pub const DO_LINT_INVALID: do_flag_values = 1;
pub const DO_FLAG_NONE: do_flag_values = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union stack_item {
    pub rptr: *mut NODE,
    pub lptr: *mut *mut NODE,
}
pub type STACK_ITEM = stack_item;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub buf: *mut libc::c_char,
    pub bufsize: size_t,
    pub stackbuf: [libc::c_char; 30],
}
pub type C2RustUnnamed_13 = libc::c_uint;
pub const MP_FLOAT: C2RustUnnamed_13 = 3;
pub const MP_INT_WITHOUT_PREC: C2RustUnnamed_13 = 2;
pub const MP_INT_WITH_PREC: C2RustUnnamed_13 = 1;
pub const MP_NONE: C2RustUnnamed_13 = 0;
pub type two_way_close_type = libc::c_uint;
pub const CLOSE_FROM: two_way_close_type = 2;
pub const CLOSE_TO: two_way_close_type = 1;
pub const CLOSE_ALL: two_way_close_type = 0;
pub type gawk_uint32_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct category_table {
    pub val: libc::c_int,
    pub name: *const libc::c_char,
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
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn POP_PARAM() -> *mut NODE {
    let fresh1 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    let mut t: *mut NODE = (*fresh1).rptr;
    return if (*t).type_0 as libc::c_uint
        == Node_var_array as libc::c_int as libc::c_uint
    {
        t
    } else {
        force_array(t, 0 as libc::c_int != 0)
    };
}
#[inline]
unsafe extern "C" fn assoc_set(
    mut array: *mut NODE,
    mut sub: *mut NODE,
    mut value: *mut NODE,
) {
    let mut lhs: *mut *mut NODE = ((*(*array).sub.nodep.l.lp).lookup)
        .unwrap()(array, sub);
    unref(*lhs);
    *lhs = value;
    if ((*(*array).sub.nodep.l.lp).store).is_some() {
        (Some(((*(*array).sub.nodep.l.lp).store).unwrap())).unwrap()(array, sub);
    }
    unref(sub);
}
#[inline]
unsafe extern "C" fn in_array(mut a: *mut NODE, mut s: *mut NODE) -> *mut NODE {
    let mut ret: *mut *mut NODE = 0 as *mut *mut NODE;
    ret = ((*(*a).sub.nodep.l.lp).exists).unwrap()(a, s);
    return if !ret.is_null() { *ret } else { 0 as *mut NODE };
}
#[inline]
unsafe extern "C" fn unref(mut r: *mut NODE) {
    if !r.is_null()
        && {
            (*r).valref -= 1;
            (*r).valref <= 0 as libc::c_int as libc::c_long
        }
    {
        r_unref(r);
    }
}
#[inline]
unsafe extern "C" fn str_terminate_f(mut n: *mut NODE, mut savep: *mut libc::c_char) {
    *savep = *((*n).sub.val.sp).offset((*n).sub.val.slen as isize);
    *((*n).sub.val.sp).offset((*n).sub.val.slen as isize) = '\0' as i32 as libc::c_char;
}
#[inline]
unsafe extern "C" fn emalloc_real(
    mut count: size_t,
    mut where_0: *const libc::c_char,
    mut var: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if count == 0 as libc::c_int as libc::c_ulong {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./awk.h\0" as *const u8 as *const libc::c_char,
            2052 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            b"%s:%d: emalloc called with zero bytes\0" as *const u8
                as *const libc::c_char,
            file,
            line,
        );
    }
    ret = malloc(count);
    if ret.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./awk.h\0" as *const u8 as *const libc::c_char,
            2056 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d:%s: %s: cannot allocate %ld bytes of memory: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            file,
            line,
            where_0,
            var,
            count as libc::c_long,
            strerror(*__errno_location()),
        );
    }
    return ret;
}
#[inline]
unsafe extern "C" fn erealloc_real(
    mut ptr: *mut libc::c_void,
    mut count: size_t,
    mut where_0: *const libc::c_char,
    mut var: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if count == 0 as libc::c_int as libc::c_ulong {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./awk.h\0" as *const u8 as *const libc::c_char,
            2088 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            b"%s:%d: erealloc called with zero bytes\0" as *const u8
                as *const libc::c_char,
            file,
            line,
        );
    }
    ret = realloc(ptr, count);
    if ret.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./awk.h\0" as *const u8 as *const libc::c_char,
            2092 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d:%s: %s: cannot reallocate %ld bytes of memory: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            file,
            line,
            where_0,
            var,
            count as libc::c_long,
            strerror(*__errno_location()),
        );
    }
    return ret;
}
#[inline]
unsafe extern "C" fn boolval(mut t: *mut NODE) -> bool {
    if (*t).type_0 as libc::c_uint == Node_var as libc::c_int as libc::c_uint {
        t = (*t).sub.nodep.l.lptr;
    }
    fixtype(t);
    if (*t).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        return !((*t).sub.val.fltnum == 0.0f64);
    }
    return (*t).sub.val.slen > 0 as libc::c_int as libc::c_ulong;
}
#[inline]
unsafe extern "C" fn dupnode(mut n: *mut NODE) -> *mut NODE {
    if (*n).flags as libc::c_uint & MALLOC as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        (*n).valref += 1;
        (*n).valref;
        return n;
    }
    return r_dupnode(n);
}
#[inline]
unsafe extern "C" fn POP_SCALAR() -> *mut NODE {
    let fresh2 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    let mut t: *mut NODE = (*fresh2).rptr;
    if (*t).type_0 as libc::c_uint == Node_var_array as libc::c_int as libc::c_uint {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./awk.h\0" as *const u8 as *const libc::c_char,
            1881 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"attempt to use array `%s' in a scalar context\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            array_vname(t),
        );
    } else if (*t).type_0 as libc::c_uint == Node_elem_new as libc::c_int as libc::c_uint
    {
        t = elem_new_to_scalar(t);
    }
    return t;
}
#[inline]
unsafe extern "C" fn force_string_fmt(
    mut s: *mut NODE,
    mut fmtstr: *const libc::c_char,
    mut fmtidx: libc::c_int,
) -> *mut NODE {
    if (*s).type_0 as libc::c_uint == Node_elem_new as libc::c_int as libc::c_uint {
        (*s).type_0 = Node_val;
        (*s)
            .flags = ::std::mem::transmute::<
            libc::c_uint,
            flagvals,
        >((*s).flags as libc::c_uint & !(NUMBER as libc::c_int) as libc::c_uint);
        return s;
    }
    if (*s).flags as libc::c_uint & STRCUR as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
        && ((*s).sub.val.idx == -(1 as libc::c_int) || (*s).sub.val.idx == fmtidx)
    {
        return s;
    }
    return format_val.unwrap()(fmtstr, fmtidx, s);
}
#[inline]
unsafe extern "C" fn fixtype(mut n: *mut NODE) -> *mut NODE {
    if (*n).flags as libc::c_uint
        & (NUMCUR as libc::c_int | USER_INPUT as libc::c_int) as libc::c_uint
        == USER_INPUT as libc::c_int as libc::c_uint
    {
        return force_number(n);
    }
    if (*n).flags as libc::c_uint & INTIND as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        return force_string_fmt(n, CONVFMT, CONVFMTidx);
    }
    return n;
}
#[inline]
unsafe extern "C" fn force_number(mut n: *mut NODE) -> *mut NODE {
    return if (*n).flags as libc::c_uint & NUMCUR as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        n
    } else {
        str2number.unwrap()(n)
    };
}
#[inline]
unsafe extern "C" fn mbrlen(
    mut __s: *const libc::c_char,
    mut __n: size_t,
    mut __ps: *mut mbstate_t,
) -> size_t {
    return if !__ps.is_null() {
        mbrtowc(0 as *mut wchar_t, __s, __n, __ps)
    } else {
        __mbrlen(__s, __n, 0 as *mut mbstate_t)
    };
}
#[inline]
unsafe extern "C" fn DEREF(mut r: *mut NODE) {
    (*r).valref -= 1;
    if (*r).valref > 0 as libc::c_int as libc::c_long {
        return;
    }
    r_unref(r);
}
pub unsafe extern "C" fn check_exact_args(
    mut nargs: libc::c_int,
    mut fname: *const libc::c_char,
    mut count: libc::c_int,
) {
    if nargs != count {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: called with %d arguments\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            fname,
            nargs,
        );
    }
}
pub unsafe extern "C" fn check_args_min_max(
    mut nargs: libc::c_int,
    mut fname: *const libc::c_char,
    mut min: libc::c_int,
    mut max: libc::c_int,
) {
    if nargs < min || nargs > max {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            105 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: called with %d arguments\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            fname,
            nargs,
        );
    }
}
unsafe extern "C" fn wrerror(
    mut fp: *mut FILE,
    mut from: *const libc::c_char,
    mut rp: *mut redirect,
) {
    os_maybe_set_errno();
    if fp == stdout && *__errno_location() == 32 as libc::c_int {
        signal(13 as libc::c_int, None);
        kill(getpid(), 13 as libc::c_int);
    }
    if if !rp.is_null() {
        is_non_fatal_redirect((*rp).value, strlen((*rp).value)) as libc::c_int
    } else {
        is_non_fatal_std(fp) as libc::c_int
    } != 0
    {
        update_ERRNO_int(*__errno_location());
    } else {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            130 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s to \"%s\" failed: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            from,
            if !rp.is_null() {
                (*rp).value
            } else if fp == stdout {
                dcgettext(
                    0 as *const libc::c_char,
                    b"standard output\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            } else {
                dcgettext(
                    0 as *const libc::c_char,
                    b"standard error\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            },
            if *__errno_location() != 0 {
                strerror(*__errno_location())
            } else {
                dcgettext(
                    0 as *const libc::c_char,
                    b"reason unknown\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            },
        );
    };
}
pub unsafe extern "C" fn efflush(
    mut fp: *mut FILE,
    mut from: *const libc::c_char,
    mut rp: *mut redirect,
) {
    *__errno_location() = 0 as libc::c_int;
    if !rp.is_null() {
        ((*rp).output.gawk_fflush).unwrap()(fp, (*rp).output.opaque);
        if ((*rp).output.gawk_ferror).unwrap()(fp, (*rp).output.opaque) != 0 {
            wrerror(fp, from, rp);
        }
    } else {
        fflush(fp);
        if ferror(fp) != 0 {
            wrerror(fp, from, rp);
        }
    };
}
unsafe extern "C" fn efwrite(
    mut ptr: *const libc::c_void,
    mut size: size_t,
    mut count: size_t,
    mut fp: *mut FILE,
    mut from: *const libc::c_char,
    mut rp: *mut redirect,
    mut flush: bool,
) {
    *__errno_location() = 0 as libc::c_int;
    if !rp.is_null() {
        let mut err_on_write: bool = ((*rp).output.gawk_fwrite)
            .unwrap()(ptr, size, count, fp, (*rp).output.opaque) != count;
        let mut err_on_fp: bool = (*rp).output.gawk_fwrite
            == Some(
                gawk_fwrite
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        size_t,
                        size_t,
                        *mut FILE,
                        *mut libc::c_void,
                    ) -> size_t,
            ) && ferror(fp) != 0;
        if err_on_write as libc::c_int != 0 || err_on_fp as libc::c_int != 0 {
            wrerror(fp, from, rp);
            return;
        }
    } else if (if 0 != 0 && 0 != 0
        && size.wrapping_mul(count) <= 8 as libc::c_int as libc::c_ulong
        && size != 0 as libc::c_int as libc::c_ulong
    {
        {
            let mut __ptr: *const libc::c_char = ptr as *const libc::c_char;
            let mut __stream: *mut FILE = fp;
            let mut __cnt: size_t = 0;
            __cnt = size.wrapping_mul(count);
            while __cnt > 0 as libc::c_int as libc::c_ulong {
                let fresh3 = __ptr;
                __ptr = __ptr.offset(1);
                if putc_unlocked(*fresh3 as libc::c_int, __stream) == -(1 as libc::c_int)
                {
                    break;
                }
                __cnt = __cnt.wrapping_sub(1);
                __cnt;
            }
            size.wrapping_mul(count).wrapping_sub(__cnt).wrapping_div(size)
        }
    } else {
        if 0 != 0 && size == 0 as libc::c_int as libc::c_ulong
            || 0 != 0 && count == 0 as libc::c_int as libc::c_ulong
        {
            0 as libc::c_int as size_t
        } else {
            fwrite_unlocked(ptr, size, count, fp)
        }
    }) != count || ferror(fp) != 0
    {
        wrerror(fp, from, rp);
        return;
    }
    if flush as libc::c_int != 0
        && (fp == stdout && output_is_tty as libc::c_int != 0
            || !rp.is_null()
                && (*rp).flag as libc::c_uint & RED_FLUSH as libc::c_int as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint)
    {
        efflush(fp, from, rp);
    }
}
pub unsafe extern "C" fn do_exp(mut nargs: libc::c_int) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut d: libc::c_double = 0.;
    let mut res: libc::c_double = 0.;
    check_exact_args(
        nargs,
        b"exp\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    tmp = POP_SCALAR();
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
        && (*fixtype(tmp)).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            211 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: received non-numeric argument\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"exp\0" as *const u8 as *const libc::c_char,
        );
    }
    d = (*force_number(tmp)).sub.val.fltnum;
    DEREF(tmp);
    *__errno_location() = 0 as libc::c_int;
    res = exp(d);
    if *__errno_location() == 34 as libc::c_int {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            217 as libc::c_int,
        );
        (Some(
            (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"exp: argument %g is out of range\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            d,
        );
    }
    return make_number.unwrap()(res);
}
unsafe extern "C" fn stdfile(
    mut name: *const libc::c_char,
    mut len: size_t,
) -> *mut FILE {
    if len == 11 as libc::c_int as libc::c_ulong {
        if strncmp(
            name,
            b"/dev/stderr\0" as *const u8 as *const libc::c_char,
            11 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            return stderr
        } else if strncmp(
            name,
            b"/dev/stdout\0" as *const u8 as *const libc::c_char,
            11 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            return stdout
        }
    }
    return 0 as *mut FILE;
}
pub unsafe extern "C" fn do_fflush(mut nargs: libc::c_int) -> *mut NODE {
    let mut rp: *mut redirect = 0 as *mut redirect;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut status: libc::c_int = 0 as libc::c_int;
    let mut file: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: libc::c_int = 0;
    check_args_min_max(
        nargs,
        b"fflush\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        1 as libc::c_int,
    );
    if nargs == 0 as libc::c_int {
        status = flush_io();
        return make_number.unwrap()(status as libc::c_double);
    }
    tmp = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
        && (*fixtype(tmp)).flags as libc::c_uint & STRING as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            281 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: received non-string argument\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"fflush\0" as *const u8 as *const libc::c_char,
        );
    }
    file = (*tmp).sub.val.sp;
    len = (*tmp).sub.val.slen as libc::c_int;
    if (*tmp).sub.val.slen == 0 as libc::c_int as libc::c_ulong {
        status = flush_io();
        DEREF(tmp);
        return make_number.unwrap()(status as libc::c_double);
    }
    rp = getredirect((*tmp).sub.val.sp, (*tmp).sub.val.slen as libc::c_int);
    status = -(1 as libc::c_int);
    if !rp.is_null() {
        if (*rp).flag as libc::c_uint
            & (RED_WRITE as libc::c_int | RED_APPEND as libc::c_int) as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        {
            if (*rp).flag as libc::c_uint & RED_PIPE as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"builtin.c\0" as *const u8 as *const libc::c_char,
                    298 as libc::c_int,
                );
                (Some(
                    (Some(
                        r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                    ))
                        .unwrap(),
                ))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"fflush: cannot flush: pipe `%.*s' opened for reading, not writing\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    len,
                    file,
                );
            } else {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"builtin.c\0" as *const u8 as *const libc::c_char,
                    301 as libc::c_int,
                );
                (Some(
                    (Some(
                        r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                    ))
                        .unwrap(),
                ))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"fflush: cannot flush: file `%.*s' opened for reading, not writing\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    len,
                    file,
                );
            }
            DEREF(tmp);
            return make_number.unwrap()(status as libc::c_double);
        }
        fp = (*rp).output.fp;
        if !fp.is_null() {
            status = ((*rp).output.gawk_fflush).unwrap()(fp, (*rp).output.opaque);
            if status != 0 as libc::c_int {
                if !is_non_fatal_redirect((*tmp).sub.val.sp, (*tmp).sub.val.slen) {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"builtin.c\0" as *const u8 as *const libc::c_char,
                        312 as libc::c_int,
                    );
                    (Some(
                        (Some(
                            r_fatal
                                as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                        ))
                            .unwrap(),
                    ))
                        .unwrap()(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"fflush: cannot flush file `%.*s': %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        len,
                        file,
                        strerror(*__errno_location()),
                    );
                }
                update_ERRNO_int(*__errno_location());
            }
        } else if (*rp).flag as libc::c_uint & RED_TWOWAY as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                317 as libc::c_int,
            );
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"fflush: cannot flush: two-way pipe `%.*s' has closed write end\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                len,
                file,
            );
        }
    } else {
        fp = stdfile((*tmp).sub.val.sp, (*tmp).sub.val.slen);
        if !fp.is_null() {
            status = (non_fatal_flush_std_file(fp) as libc::c_int == 0 as libc::c_int)
                as libc::c_int;
        } else {
            status = -(1 as libc::c_int);
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                323 as libc::c_int,
            );
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"fflush: `%.*s' is not an open file, pipe or co-process\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                len,
                file,
            );
        }
    }
    DEREF(tmp);
    return make_number.unwrap()(status as libc::c_double);
}
pub unsafe extern "C" fn strncasecmpmbs(
    mut s1: *const libc::c_uchar,
    mut s2: *const libc::c_uchar,
    mut n: size_t,
) -> libc::c_int {
    let mut i1: size_t = 0;
    let mut i2: size_t = 0;
    let mut mbclen1: size_t = 0;
    let mut mbclen2: size_t = 0;
    let mut gap: size_t = 0;
    let mut wc1: wchar_t = 0;
    let mut wc2: wchar_t = 0;
    let mut mbs1: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    let mut mbs2: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    memset(
        &mut mbs1 as *mut mbstate_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    memset(
        &mut mbs2 as *mut mbstate_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    i2 = 0 as libc::c_int as size_t;
    i1 = i2;
    while i1 < n && i2 < n {
        if *btowc_cache
            .as_mut_ptr()
            .offset(
                (*s1.offset(i1 as isize) as libc::c_int & 0xff as libc::c_int) as isize,
            ) != 0xffffffff as libc::c_uint
        {
            mbclen1 = 1 as libc::c_int as size_t;
            wc1 = *btowc_cache
                .as_mut_ptr()
                .offset(
                    (*s1.offset(i1 as isize) as libc::c_int & 0xff as libc::c_int)
                        as isize,
                ) as wchar_t;
        } else {
            mbclen1 = mbrtowc(
                &mut wc1,
                (s1 as *const libc::c_char).offset(i1 as isize),
                n.wrapping_sub(i1),
                &mut mbs1,
            );
            if mbclen1 == -(1 as libc::c_int) as size_t
                || mbclen1 == -(2 as libc::c_int) as size_t
                || mbclen1 == 0 as libc::c_int as libc::c_ulong
            {
                mbclen1 = 1 as libc::c_int as size_t;
                wc1 = *btowc_cache
                    .as_mut_ptr()
                    .offset(
                        (*s1.offset(i1 as isize) as libc::c_int & 0xff as libc::c_int)
                            as isize,
                    ) as wchar_t;
            }
        }
        if *btowc_cache
            .as_mut_ptr()
            .offset(
                (*s2.offset(i2 as isize) as libc::c_int & 0xff as libc::c_int) as isize,
            ) != 0xffffffff as libc::c_uint
        {
            mbclen2 = 1 as libc::c_int as size_t;
            wc2 = *btowc_cache
                .as_mut_ptr()
                .offset(
                    (*s2.offset(i2 as isize) as libc::c_int & 0xff as libc::c_int)
                        as isize,
                ) as wchar_t;
        } else {
            mbclen2 = mbrtowc(
                &mut wc2,
                (s2 as *const libc::c_char).offset(i2 as isize),
                n.wrapping_sub(i2),
                &mut mbs2,
            );
            if mbclen2 == -(1 as libc::c_int) as size_t
                || mbclen2 == -(2 as libc::c_int) as size_t
                || mbclen2 == 0 as libc::c_int as libc::c_ulong
            {
                mbclen2 = 1 as libc::c_int as size_t;
                wc2 = *btowc_cache
                    .as_mut_ptr()
                    .offset(
                        (*s2.offset(i2 as isize) as libc::c_int & 0xff as libc::c_int)
                            as isize,
                    ) as wchar_t;
            }
        }
        gap = (towlower(wc1 as wint_t)).wrapping_sub(towlower(wc2 as wint_t)) as size_t;
        if gap != 0 as libc::c_int as libc::c_ulong {
            return gap as libc::c_int;
        }
        i1 = (i1 as libc::c_ulong).wrapping_add(mbclen1) as size_t as size_t;
        i2 = (i2 as libc::c_ulong).wrapping_add(mbclen2) as size_t as size_t;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn index_multibyte_buffer(
    mut src: *mut libc::c_char,
    mut dest: *mut libc::c_char,
    mut len: libc::c_int,
) {
    let mut idx: libc::c_int = 0;
    let mut prev_idx: libc::c_int = 0;
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    let mut prevs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    memset(
        &mut prevs as *mut mbstate_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    prev_idx = 0 as libc::c_int;
    idx = prev_idx;
    while idx < len {
        let mut mbclen: size_t = 0;
        mbs = prevs;
        mbclen = mbrlen(
            src.offset(prev_idx as isize),
            (idx - prev_idx + 1 as libc::c_int) as size_t,
            &mut mbs,
        );
        if mbclen == -(1 as libc::c_int) as size_t
            || mbclen == 1 as libc::c_int as libc::c_ulong
            || mbclen == 0 as libc::c_int as libc::c_ulong
        {
            mbclen = 1 as libc::c_int as size_t;
            prev_idx = idx + 1 as libc::c_int;
        } else if mbclen == -(2 as libc::c_int) as size_t {
            mbclen = (idx - prev_idx + 1 as libc::c_int) as size_t;
        } else if mbclen > 1 as libc::c_int as libc::c_ulong {
            prev_idx = idx + 1 as libc::c_int;
            prevs = mbs;
        }
        *dest.offset(idx as isize) = mbclen as libc::c_char;
        idx += 1;
        idx;
    }
}
pub unsafe extern "C" fn do_index(mut nargs: libc::c_int) -> *mut NODE {
    let mut s1: *mut NODE = 0 as *mut NODE;
    let mut s2: *mut NODE = 0 as *mut NODE;
    let mut p1: *const libc::c_char = 0 as *const libc::c_char;
    let mut p2: *const libc::c_char = 0 as *const libc::c_char;
    let mut l1: size_t = 0;
    let mut l2: size_t = 0;
    let mut ret: libc::c_long = 0;
    let mut do_single_byte: bool = 0 as libc::c_int != 0;
    let mut mbs1: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    let mut mbs2: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    check_exact_args(
        nargs,
        b"index\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int,
    );
    if gawk_mb_cur_max > 1 as libc::c_int {
        memset(
            &mut mbs1 as *mut mbstate_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
        );
        memset(
            &mut mbs2 as *mut mbstate_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
        );
    }
    s2 = POP_SCALAR();
    let fresh4 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    s1 = (*fresh4).rptr;
    if (*s1).type_0 as libc::c_uint == Node_var_array as libc::c_int as libc::c_uint {
        DEREF(s2);
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            428 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"attempt to use array `%s' in a scalar context\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            array_vname(s1),
        );
    }
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
    {
        if (*fixtype(s1)).flags as libc::c_uint
            & (STRING as libc::c_int | USER_INPUT as libc::c_int) as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                432 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: received non-string first argument\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"index\0" as *const u8 as *const libc::c_char,
            );
        }
        if (*fixtype(s2)).flags as libc::c_uint
            & (STRING as libc::c_int | USER_INPUT as libc::c_int) as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                434 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: received non-string second argument\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"index\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    s1 = force_string_fmt(s1, CONVFMT, CONVFMTidx);
    s2 = force_string_fmt(s2, CONVFMT, CONVFMTidx);
    p1 = (*s1).sub.val.sp;
    p2 = (*s2).sub.val.sp;
    l1 = (*s1).sub.val.slen;
    l2 = (*s2).sub.val.slen;
    ret = 0 as libc::c_int as libc::c_long;
    if l2 == 0 as libc::c_int as libc::c_ulong {
        ret = 1 as libc::c_int as libc::c_long;
    } else {
        if gawk_mb_cur_max > 1 as libc::c_int {
            s1 = str2wstr(s1, 0 as *mut *mut size_t);
            s2 = str2wstr(s2, 0 as *mut *mut size_t);
            do_single_byte = (*s1).sub.val.wslen == 0 as libc::c_int as libc::c_ulong
                && (*s1).sub.val.slen > 0 as libc::c_int as libc::c_ulong
                || (*s2).sub.val.wslen == 0 as libc::c_int as libc::c_ulong
                    && (*s2).sub.val.slen > 0 as libc::c_int as libc::c_ulong;
        }
        if IGNORECASE {
            while l1 > 0 as libc::c_int as libc::c_ulong {
                if l2 > l1 {
                    break;
                }
                if !do_single_byte && gawk_mb_cur_max > 1 as libc::c_int {
                    let mut pos: *const wchar_t = 0 as *const wchar_t;
                    pos = wcasestrstr(
                        (*s1).sub.val.wsp,
                        (*s1).sub.val.wslen,
                        (*s2).sub.val.wsp,
                        (*s2).sub.val.wslen,
                    );
                    if pos.is_null() {
                        ret = 0 as libc::c_int as libc::c_long;
                    } else {
                        ret = pos.offset_from((*s1).sub.val.wsp) as libc::c_long
                            + 1 as libc::c_int as libc::c_long;
                    }
                    break;
                } else if *casetable.as_mut_ptr().offset(*p1 as libc::c_uchar as isize)
                    as libc::c_int
                    == *casetable.as_mut_ptr().offset(*p2 as libc::c_uchar as isize)
                        as libc::c_int
                    && (l2 == 1 as libc::c_int as libc::c_ulong
                        || strncasecmp(p1, p2, l2) == 0 as libc::c_int)
                {
                    ret = (1 as libc::c_int as libc::c_ulong)
                        .wrapping_add((*s1).sub.val.slen)
                        .wrapping_sub(l1) as libc::c_long;
                    break;
                } else {
                    l1 = l1.wrapping_sub(1);
                    l1;
                    p1 = p1.offset(1);
                    p1;
                }
            }
        } else {
            while l1 > 0 as libc::c_int as libc::c_ulong {
                if l2 > l1 {
                    break;
                }
                if *p1 as libc::c_int == *p2 as libc::c_int
                    && (l2 == 1 as libc::c_int as libc::c_ulong
                        || l2 > 0 as libc::c_int as libc::c_ulong
                            && memcmp(
                                p1 as *const libc::c_void,
                                p2 as *const libc::c_void,
                                l2,
                            ) == 0 as libc::c_int)
                {
                    ret = (1 as libc::c_int as libc::c_ulong)
                        .wrapping_add((*s1).sub.val.slen)
                        .wrapping_sub(l1) as libc::c_long;
                    break;
                } else if !do_single_byte && gawk_mb_cur_max > 1 as libc::c_int {
                    let mut pos_0: *const wchar_t = 0 as *const wchar_t;
                    pos_0 = wstrstr(
                        (*s1).sub.val.wsp,
                        (*s1).sub.val.wslen,
                        (*s2).sub.val.wsp,
                        (*s2).sub.val.wslen,
                    );
                    if pos_0.is_null() {
                        ret = 0 as libc::c_int as libc::c_long;
                    } else {
                        ret = pos_0.offset_from((*s1).sub.val.wsp) as libc::c_long
                            + 1 as libc::c_int as libc::c_long;
                    }
                    break;
                } else {
                    l1 = l1.wrapping_sub(1);
                    l1;
                    p1 = p1.offset(1);
                    p1;
                }
            }
        }
    }
    DEREF(s1);
    DEREF(s2);
    return make_number.unwrap()(ret as libc::c_double);
}
pub unsafe extern "C" fn double_to_int(mut d: libc::c_double) -> libc::c_double {
    if d >= 0 as libc::c_int as libc::c_double {
        d = floor(d);
    } else {
        d = ceil(d);
    }
    return d;
}
pub unsafe extern "C" fn do_int(mut nargs: libc::c_int) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut d: libc::c_double = 0.;
    check_exact_args(
        nargs,
        b"int\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    tmp = POP_SCALAR();
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
        && (*fixtype(tmp)).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            549 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: received non-numeric argument\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"int\0" as *const u8 as *const libc::c_char,
        );
    }
    d = (*force_number(tmp)).sub.val.fltnum;
    d = double_to_int(d);
    DEREF(tmp);
    return make_number.unwrap()(d);
}
pub unsafe extern "C" fn do_isarray(mut nargs: libc::c_int) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut ret: libc::c_int = 1 as libc::c_int;
    check_exact_args(
        nargs,
        b"isarray\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    let fresh5 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    tmp = (*fresh5).rptr;
    if (*tmp).type_0 as libc::c_uint != Node_var_array as libc::c_int as libc::c_uint {
        ret = 0 as libc::c_int;
        if (*tmp).type_0 as libc::c_uint == Node_val as libc::c_int as libc::c_uint {
            DEREF(tmp);
        }
    }
    return make_number.unwrap()(ret as libc::c_double);
}
pub unsafe extern "C" fn do_length(mut nargs: libc::c_int) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut len: size_t = 0;
    check_exact_args(
        nargs,
        b"length\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    let fresh6 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    tmp = (*fresh6).rptr;
    if (*tmp).type_0 as libc::c_uint == Node_var_array as libc::c_int as libc::c_uint {
        static mut warned: bool = 0 as libc::c_int != 0;
        let mut size: libc::c_ulong = 0;
        if do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint != 0 {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                592 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"length: received array argument\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        if do_flags as libc::c_uint & DO_LINT_EXTENSIONS as libc::c_int as libc::c_uint
            != 0 && !warned
        {
            warned = 1 as libc::c_int != 0;
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                595 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"`length(array)' is a gawk extension\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        size = (*tmp).sub.nodep.reflags as libc::c_ulong;
        return make_number.unwrap()(size as libc::c_double);
    } else if (*tmp).type_0 as libc::c_uint
        == Node_var_new as libc::c_int as libc::c_uint
        || (*tmp).type_0 as libc::c_uint == Node_elem_new as libc::c_int as libc::c_uint
    {
        DEREF(tmp);
        tmp = dupnode(Nnull_string);
    }
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
        && (*fixtype(tmp)).flags as libc::c_uint
            & (STRING as libc::c_int | USER_INPUT as libc::c_int) as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            618 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: received non-string argument\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"length\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp = force_string_fmt(tmp, CONVFMT, CONVFMTidx);
    if gawk_mb_cur_max > 1 as libc::c_int {
        tmp = str2wstr(tmp, 0 as *mut *mut size_t);
        len = (*tmp).sub.val.wslen;
        if len == 0 as libc::c_int as libc::c_ulong
            && (*tmp).sub.val.slen > 0 as libc::c_int as libc::c_ulong
        {
            len = (*tmp).sub.val.slen;
        }
    } else {
        len = (*tmp).sub.val.slen;
    }
    DEREF(tmp);
    return make_number.unwrap()(len as libc::c_double);
}
pub unsafe extern "C" fn do_log(mut nargs: libc::c_int) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut d: libc::c_double = 0.;
    let mut arg: libc::c_double = 0.;
    check_exact_args(
        nargs,
        b"log\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    tmp = POP_SCALAR();
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
        && (*fixtype(tmp)).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            649 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: received non-numeric argument\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"log\0" as *const u8 as *const libc::c_char,
        );
    }
    arg = (*force_number(tmp)).sub.val.fltnum;
    if arg < 0.0f64 {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            652 as libc::c_int,
        );
        (Some(
            (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: received negative argument %g\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"log\0" as *const u8 as *const libc::c_char,
            arg,
        );
    }
    d = log(arg);
    DEREF(tmp);
    return make_number.unwrap()(d);
}
pub unsafe extern "C" fn format_tree(
    mut fmt_string: *const libc::c_char,
    mut n0: size_t,
    mut the_args: *mut *mut NODE,
    mut num_args: libc::c_long,
) -> *mut NODE {
    let mut need_to_add_thousands: bool = false;
    let mut current_block: u64;
    let mut cur_arg: size_t = 0 as libc::c_int as size_t;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut i: libc::c_int = 0;
    let mut nc: libc::c_int = 0;
    let mut toofew: bool = 0 as libc::c_int != 0;
    let mut obuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut obufout: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut osiz: size_t = 0;
    let mut ofre: size_t = 0;
    let mut olen_final: size_t = 0;
    let mut chbuf: *const libc::c_char = 0 as *const libc::c_char;
    let mut s0: *const libc::c_char = 0 as *const libc::c_char;
    let mut s1: *const libc::c_char = 0 as *const libc::c_char;
    let mut cs1: libc::c_int = 0;
    let mut arg: *mut NODE = 0 as *mut NODE;
    let mut fw: libc::c_long = 0;
    let mut prec: libc::c_long = 0;
    let mut argnum: libc::c_long = 0;
    let mut used_dollar: bool = false;
    let mut lj: bool = false;
    let mut alt: bool = false;
    let mut have_prec: bool = false;
    let mut need_format: bool = false;
    let mut cur: *mut libc::c_long = 0 as *mut libc::c_long;
    let mut uval: uintmax_t = 0;
    let mut sgn: bool = false;
    let mut base: libc::c_int = 0;
    let mut cpbufs: [C2RustUnnamed_12; 2] = [C2RustUnnamed_12 {
        buf: 0 as *mut libc::c_char,
        bufsize: 0,
        stackbuf: [0; 30],
    }; 2];
    let mut cend: *mut libc::c_char = &mut *((*cpbufs
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
        .stackbuf)
        .as_mut_ptr()
        .offset(::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong as isize)
        as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fill: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmpval: libc::c_double = 0.0f64;
    let mut signchar: libc::c_char = '\0' as i32 as libc::c_char;
    let mut len: size_t = 0;
    let mut zero_flag: bool = 0 as libc::c_int != 0;
    let mut quote_flag: bool = 0 as libc::c_int != 0;
    let mut ii: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut chp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut copy_count: size_t = 0;
    let mut char_count: size_t = 0;
    let mut nan_inf_val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut magic_posix_flag: bool = false;
    let mut fmt_type: C2RustUnnamed_13 = MP_NONE;
    static mut sp: [libc::c_char; 2] = unsafe {
        *::std::mem::transmute::<&[u8; 2], &[libc::c_char; 2]>(b" \0")
    };
    static mut zero_string: [libc::c_char; 2] = unsafe {
        *::std::mem::transmute::<&[u8; 2], &[libc::c_char; 2]>(b"0\0")
    };
    static mut lchbuf: [libc::c_char; 17] = unsafe {
        *::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"0123456789abcdef\0")
    };
    static mut Uchbuf: [libc::c_char; 17] = unsafe {
        *::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"0123456789ABCDEF\0")
    };
    static mut bad_modifiers: [libc::c_char; 7] = unsafe {
        *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"hjlLtz\0")
    };
    static mut warned: [bool; 6] = [false; 6];
    let mut modifier_seen: [bool; 6] = [false; 6];
    obuf = emalloc_real(
        64 as libc::c_int as size_t,
        b"format_tree\0" as *const u8 as *const libc::c_char,
        b"obuf\0" as *const u8 as *const libc::c_char,
        b"builtin.c\0" as *const u8 as *const libc::c_char,
        811 as libc::c_int,
    ) as *mut libc::c_char;
    obufout = obuf;
    osiz = 64 as libc::c_int as size_t;
    ofre = osiz.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    cur_arg = 1 as libc::c_int as size_t;
    let mut k: size_t = 0;
    k = 0 as libc::c_int as size_t;
    while k
        < (::std::mem::size_of::<[C2RustUnnamed_12; 2]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_12>() as libc::c_ulong)
    {
        cpbufs[k as usize]
            .bufsize = ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong;
        cpbufs[k as usize].buf = (cpbufs[k as usize].stackbuf).as_mut_ptr();
        k = k.wrapping_add(1);
        k;
    }
    need_format = 0 as libc::c_int != 0;
    used_dollar = 0 as libc::c_int != 0;
    s1 = fmt_string;
    s0 = s1;
    's_130: loop {
        let fresh7 = n0;
        n0 = n0.wrapping_sub(1);
        if !(fresh7 > 0 as libc::c_int as libc::c_ulong) {
            current_block = 2782926371273512654;
            break;
        }
        if *s1 as libc::c_int != '%' as i32 {
            s1 = s1.offset(1);
            s1;
        } else {
            need_format = 1 as libc::c_int != 0;
            if s1.offset_from(s0) as libc::c_long != 0 {
                while s1.offset_from(s0) as libc::c_long as libc::c_ulong > ofre {
                    let mut olen: size_t = obufout.offset_from(obuf) as libc::c_long
                        as size_t;
                    obuf = erealloc_real(
                        obuf as *mut libc::c_void,
                        osiz.wrapping_mul(2 as libc::c_int as libc::c_ulong),
                        b"format_tree\0" as *const u8 as *const libc::c_char,
                        b"obuf\0" as *const u8 as *const libc::c_char,
                        b"builtin.c\0" as *const u8 as *const libc::c_char,
                        885 as libc::c_int,
                    ) as *mut libc::c_char;
                    ofre = (ofre as libc::c_ulong).wrapping_add(osiz) as size_t
                        as size_t;
                    osiz = (osiz as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                    obufout = obuf.offset(olen as isize);
                }
                memcpy(
                    obufout as *mut libc::c_void,
                    s0 as *const libc::c_void,
                    s1.offset_from(s0) as libc::c_long as size_t,
                );
                obufout = obufout.offset(s1.offset_from(s0) as libc::c_long as isize);
                ofre = (ofre as libc::c_ulong)
                    .wrapping_sub(s1.offset_from(s0) as libc::c_long as libc::c_ulong)
                    as size_t as size_t;
            }
            s0 = s1;
            cur = &mut fw;
            fw = 0 as libc::c_int as libc::c_long;
            prec = 0 as libc::c_int as libc::c_long;
            base = 0 as libc::c_int;
            argnum = 0 as libc::c_int as libc::c_long;
            base = 0 as libc::c_int;
            have_prec = 0 as libc::c_int != 0;
            signchar = '\0' as i32 as libc::c_char;
            zero_flag = 0 as libc::c_int != 0;
            quote_flag = 0 as libc::c_int != 0;
            nan_inf_val = 0 as *mut libc::c_char;
            fmt_type = MP_NONE;
            alt = 0 as libc::c_int != 0;
            lj = alt;
            memset(
                modifier_seen.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<[bool; 6]>() as libc::c_ulong,
            );
            magic_posix_flag = 0 as libc::c_int != 0;
            fill = sp.as_ptr();
            cp = cend;
            chbuf = lchbuf.as_ptr();
            s1 = s1.offset(1);
            s1;
            loop {
                let fresh8 = n0;
                n0 = n0.wrapping_sub(1);
                if fresh8 == 0 as libc::c_int as libc::c_ulong {
                    current_block = 2782926371273512654;
                    break 's_130;
                }
                let fresh9 = s1;
                s1 = s1.offset(1);
                cs1 = *fresh9 as libc::c_int;
                match cs1 {
                    -1 => {
                        current_block = 1714667450202656664;
                    }
                    37 => {
                        need_format = 0 as libc::c_int != 0;
                        if do_flags as libc::c_uint
                            & (DO_LINT_INVALID as libc::c_int
                                | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                        {
                            let mut msg_0: *const libc::c_char = 0
                                as *const libc::c_char;
                            if fw != 0 && !have_prec {
                                msg_0 = dcgettext(
                                    0 as *const libc::c_char,
                                    b"field width is ignored for `%%' specifier\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                );
                            } else if fw == 0 as libc::c_int as libc::c_long
                                && have_prec as libc::c_int != 0
                            {
                                msg_0 = dcgettext(
                                    0 as *const libc::c_char,
                                    b"precision is ignored for `%%' specifier\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                );
                            } else if fw != 0 && have_prec as libc::c_int != 0 {
                                msg_0 = dcgettext(
                                    0 as *const libc::c_char,
                                    b"field width and precision are ignored for `%%' specifier\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                );
                            }
                            if !msg_0.is_null() {
                                (set_loc
                                    as unsafe extern "C" fn(
                                        *const libc::c_char,
                                        libc::c_int,
                                    ) -> ())(
                                    b"builtin.c\0" as *const u8 as *const libc::c_char,
                                    942 as libc::c_int,
                                );
                                (Some(lintfunc.unwrap()))
                                    .unwrap()(
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    msg_0,
                                );
                            }
                        }
                        if ofre < 1 as libc::c_int as libc::c_ulong {
                            let mut olen_0: size_t = obufout.offset_from(obuf)
                                as libc::c_long as size_t;
                            obuf = erealloc_real(
                                obuf as *mut libc::c_void,
                                osiz.wrapping_mul(2 as libc::c_int as libc::c_ulong),
                                b"format_tree\0" as *const u8 as *const libc::c_char,
                                b"obuf\0" as *const u8 as *const libc::c_char,
                                b"builtin.c\0" as *const u8 as *const libc::c_char,
                                944 as libc::c_int,
                            ) as *mut libc::c_char;
                            ofre = (ofre as libc::c_ulong).wrapping_add(osiz) as size_t
                                as size_t;
                            osiz = (osiz as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                                as size_t;
                            obufout = obuf.offset(olen_0 as isize);
                        }
                        let fresh10 = obufout;
                        obufout = obufout.offset(1);
                        *fresh10 = *(b"%\0" as *const u8 as *const libc::c_char);
                        ofre = ofre.wrapping_sub(1);
                        ofre;
                        s0 = s1;
                        current_block = 2749501120807187827;
                        break;
                    }
                    48 => {
                        if cur == &mut fw as *mut libc::c_long {
                            zero_flag = 1 as libc::c_int != 0;
                        }
                        if lj {
                            continue;
                        }
                        current_block = 5850232393697233855;
                    }
                    49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                        current_block = 5850232393697233855;
                    }
                    36 => {
                        if do_flags as libc::c_uint
                            & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0
                        {
                            msg(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"fatal: `$' is not permitted in awk formats\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                            current_block = 1242492412443577758;
                            break 's_130;
                        } else if cur == &mut fw as *mut libc::c_long {
                            argnum = fw;
                            fw = 0 as libc::c_int as libc::c_long;
                            used_dollar = 1 as libc::c_int != 0;
                            if argnum <= 0 as libc::c_int as libc::c_long {
                                msg(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"fatal: argument index with `$' must be > 0\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                current_block = 1242492412443577758;
                                break 's_130;
                            } else {
                                if !(argnum >= num_args) {
                                    continue;
                                }
                                msg(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"fatal: argument index %ld greater than total number of supplied arguments\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    argnum,
                                );
                                current_block = 1242492412443577758;
                                break 's_130;
                            }
                        } else {
                            msg(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"fatal: `$' not permitted after period in format\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                            current_block = 1242492412443577758;
                            break 's_130;
                        }
                    }
                    42 => {
                        if cur.is_null() {
                            current_block = 2749501120807187827;
                            break;
                        }
                        if do_flags as libc::c_uint
                            & DO_TRADITIONAL as libc::c_int as libc::c_uint == 0
                            && used_dollar as libc::c_int != 0
                            && *(*__ctype_b_loc())
                                .offset(*s1 as libc::c_uchar as libc::c_int as isize)
                                as libc::c_int
                                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                == 0
                        {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    libc::c_int,
                                ) -> ())(
                                b"builtin.c\0" as *const u8 as *const libc::c_char,
                                1016 as libc::c_int,
                            );
                            (Some(
                                (Some(
                                    r_fatal
                                        as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                                ))
                                    .unwrap(),
                            ))
                                .unwrap()(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"fatal: must use `count$' on all formats or none\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                            current_block = 2749501120807187827;
                            break;
                        } else {
                            if do_flags as libc::c_uint
                                & DO_TRADITIONAL as libc::c_int as libc::c_uint == 0
                                && *(*__ctype_b_loc())
                                    .offset(*s1 as libc::c_uchar as libc::c_int as isize)
                                    as libc::c_int
                                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0
                            {
                                let mut val: libc::c_int = 0 as libc::c_int;
                                while n0 > 0 as libc::c_int as libc::c_ulong
                                    && *s1 as libc::c_int != 0
                                    && *(*__ctype_b_loc())
                                        .offset(*s1 as libc::c_uchar as libc::c_int as isize)
                                        as libc::c_int
                                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                        != 0
                                {
                                    val *= 10 as libc::c_int;
                                    val += *s1 as libc::c_int - '0' as i32;
                                    s1 = s1.offset(1);
                                    s1;
                                    n0 = n0.wrapping_sub(1);
                                    n0;
                                }
                                if *s1 as libc::c_int != '$' as i32 {
                                    msg(
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"fatal: no `$' supplied for positional field width or precision\0"
                                                as *const u8 as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                    current_block = 1242492412443577758;
                                    break 's_130;
                                } else {
                                    s1 = s1.offset(1);
                                    s1;
                                    n0 = n0.wrapping_sub(1);
                                    n0;
                                    if val < 0 as libc::c_int || val as libc::c_long >= num_args
                                    {
                                        toofew = 1 as libc::c_int != 0;
                                        current_block = 2749501120807187827;
                                        break;
                                    } else {
                                        arg = *the_args.offset(val as isize);
                                    }
                                }
                            } else if argnum > 0 as libc::c_int as libc::c_long {
                                if cur_arg > 1 as libc::c_int as libc::c_ulong {
                                    msg(
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"fatal: must use `count$' on all formats or none\0"
                                                as *const u8 as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                    current_block = 1242492412443577758;
                                    break 's_130;
                                } else {
                                    arg = *the_args.offset(argnum as isize);
                                }
                            } else if used_dollar {
                                msg(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"fatal: must use `count$' on all formats or none\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                arg = 0 as *mut NODE;
                                current_block = 1242492412443577758;
                                break 's_130;
                            } else if cur_arg >= num_args as libc::c_ulong {
                                arg = 0 as *mut NODE;
                                toofew = 1 as libc::c_int != 0;
                                current_block = 2749501120807187827;
                                break;
                            } else {
                                arg = *the_args.offset(cur_arg as isize);
                                cur_arg = cur_arg.wrapping_add(1);
                                cur_arg;
                            }
                            force_number(arg);
                            *cur = (*arg).sub.val.fltnum as libc::c_long;
                            if *cur < 0 as libc::c_int as libc::c_long
                                && cur == &mut fw as *mut libc::c_long
                            {
                                *cur = -*cur;
                                lj = 1 as libc::c_int != 0;
                            }
                            if cur == &mut prec as *mut libc::c_long {
                                if *cur >= 0 as libc::c_int as libc::c_long {
                                    have_prec = 1 as libc::c_int != 0;
                                } else {
                                    have_prec = 0 as libc::c_int != 0;
                                }
                                cur = 0 as *mut libc::c_long;
                            }
                            continue;
                        }
                    }
                    32 => {
                        if signchar as libc::c_int != 0 as libc::c_int {
                            current_block = 1714667450202656664;
                        } else {
                            current_block = 134190185329934858;
                        }
                    }
                    43 => {
                        current_block = 134190185329934858;
                    }
                    45 => {
                        if prec < 0 as libc::c_int as libc::c_long {
                            current_block = 2749501120807187827;
                            break;
                        }
                        if cur == &mut prec as *mut libc::c_long {
                            prec = -(1 as libc::c_int) as libc::c_long;
                            continue;
                        } else {
                            fill = sp.as_ptr();
                            lj = 1 as libc::c_int != 0;
                        }
                        current_block = 1714667450202656664;
                    }
                    46 => {
                        if cur != &mut fw as *mut libc::c_long {
                            current_block = 2749501120807187827;
                            break;
                        }
                        cur = &mut prec;
                        have_prec = 1 as libc::c_int != 0;
                        continue;
                    }
                    35 => {
                        alt = 1 as libc::c_int != 0;
                        current_block = 1714667450202656664;
                    }
                    39 => {
                        quote_flag = 1 as libc::c_int != 0;
                        current_block = 1714667450202656664;
                    }
                    104 | 106 | 108 | 76 | 116 | 122 => {
                        if modifier_seen[(strchr(bad_modifiers.as_ptr(), cs1))
                            .offset_from(bad_modifiers.as_ptr()) as libc::c_long
                            as usize]
                        {
                            current_block = 2749501120807187827;
                            break;
                        }
                        let mut ind: libc::c_int = (strchr(bad_modifiers.as_ptr(), cs1))
                            .offset_from(bad_modifiers.as_ptr()) as libc::c_long
                            as libc::c_int;
                        if do_flags as libc::c_uint
                            & (DO_LINT_INVALID as libc::c_int
                                | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                            && !warned[ind as usize]
                        {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    libc::c_int,
                                ) -> ())(
                                b"builtin.c\0" as *const u8 as *const libc::c_char,
                                1104 as libc::c_int,
                            );
                            (Some(lintfunc.unwrap()))
                                .unwrap()(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"`%c' is meaningless in awk formats; ignored\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                cs1,
                            );
                            warned[ind as usize] = 1 as libc::c_int != 0;
                        }
                        if do_flags as libc::c_uint
                            & DO_POSIX as libc::c_int as libc::c_uint != 0
                        {
                            msg(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"fatal: `%c' is not permitted in POSIX awk formats\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                cs1,
                            );
                            current_block = 1242492412443577758;
                            break 's_130;
                        } else {
                            modifier_seen[(strchr(bad_modifiers.as_ptr(), cs1))
                                .offset_from(bad_modifiers.as_ptr()) as libc::c_long
                                as usize] = 1 as libc::c_int != 0;
                            continue;
                        }
                    }
                    80 => {
                        if magic_posix_flag {
                            current_block = 2749501120807187827;
                            break;
                        }
                        magic_posix_flag = 1 as libc::c_int != 0;
                        continue;
                    }
                    99 => {
                        need_format = 0 as libc::c_int != 0;
                        if argnum > 0 as libc::c_int as libc::c_long {
                            current_block = 9822987968968565122;
                            break;
                        } else {
                            current_block = 10282596542094995802;
                            break;
                        }
                    }
                    115 => {
                        need_format = 0 as libc::c_int != 0;
                        if argnum > 0 as libc::c_int as libc::c_long {
                            current_block = 1365401674109093360;
                            break;
                        } else {
                            current_block = 870304920170074103;
                            break;
                        }
                    }
                    100 | 105 => {
                        need_format = 0 as libc::c_int != 0;
                        if argnum > 0 as libc::c_int as libc::c_long {
                            current_block = 11364608634565542496;
                            break;
                        } else {
                            current_block = 17342404680090366851;
                            break;
                        }
                    }
                    88 => {
                        chbuf = Uchbuf.as_ptr();
                        current_block = 640215555276527818;
                        break;
                    }
                    120 => {
                        current_block = 640215555276527818;
                        break;
                    }
                    117 => {
                        current_block = 11847976846684641782;
                        break;
                    }
                    111 => {
                        current_block = 4644176752869242724;
                        break;
                    }
                    70 | 103 | 71 | 101 | 102 | 69 | 65 | 97 => {
                        static mut warned_0: bool = 0 as libc::c_int != 0;
                        if do_flags as libc::c_uint
                            & (DO_LINT_INVALID as libc::c_int
                                | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                            && ({
                                let mut __res: libc::c_int = 0;
                                if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    > 1 as libc::c_int as libc::c_ulong
                                {
                                    if 0 != 0 {
                                        let mut __c: libc::c_int = cs1;
                                        __res = if __c < -(128 as libc::c_int)
                                            || __c > 255 as libc::c_int
                                        {
                                            __c
                                        } else {
                                            *(*__ctype_tolower_loc()).offset(__c as isize)
                                        };
                                    } else {
                                        __res = tolower(cs1);
                                    }
                                } else {
                                    __res = *(*__ctype_tolower_loc()).offset(cs1 as isize);
                                }
                                __res
                            }) == 'a' as i32 && !warned_0
                        {
                            warned_0 = 1 as libc::c_int != 0;
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    libc::c_int,
                                ) -> ())(
                                b"builtin.c\0" as *const u8 as *const libc::c_char,
                                1577 as libc::c_int,
                            );
                            (Some(lintfunc.unwrap()))
                                .unwrap()(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"%%%c format is POSIX standard but not portable to other awks\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                cs1,
                            );
                        }
                        need_format = 0 as libc::c_int != 0;
                        if argnum > 0 as libc::c_int as libc::c_long {
                            current_block = 3889686208735362918;
                            break;
                        } else {
                            current_block = 7186168334825677637;
                            break;
                        }
                    }
                    _ => {
                        if do_flags as libc::c_uint
                            & (DO_LINT_INVALID as libc::c_int
                                | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                            && is_alpha(cs1) as libc::c_int != 0
                        {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    libc::c_int,
                                ) -> ())(
                                b"builtin.c\0" as *const u8 as *const libc::c_char,
                                1688 as libc::c_int,
                            );
                            (Some(lintfunc.unwrap()))
                                .unwrap()(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"ignoring unknown format specifier character `%c': no argument converted\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                cs1,
                            );
                        }
                        current_block = 2749501120807187827;
                        break;
                    }
                }
                match current_block {
                    5850232393697233855 => {
                        if cur.is_null() {
                            current_block = 2749501120807187827;
                            break;
                        }
                        if prec >= 0 as libc::c_int as libc::c_long {
                            *cur = (cs1 - '0' as i32) as libc::c_long;
                        }
                        while n0 > 0 as libc::c_int as libc::c_ulong
                            && *s1 as libc::c_int >= '0' as i32
                            && *s1 as libc::c_int <= '9' as i32
                        {
                            n0 = n0.wrapping_sub(1);
                            n0;
                            let fresh11 = s1;
                            s1 = s1.offset(1);
                            *cur = *cur * 10 as libc::c_int as libc::c_long
                                + *fresh11 as libc::c_long - '0' as i32 as libc::c_long;
                        }
                        if prec < 0 as libc::c_int as libc::c_long {
                            have_prec = 0 as libc::c_int != 0;
                        }
                        if cur == &mut prec as *mut libc::c_long {
                            cur = 0 as *mut libc::c_long;
                        }
                        if n0 == 0 as libc::c_int as libc::c_ulong {
                            continue 's_130;
                        } else {
                            continue;
                        }
                    }
                    134190185329934858 => {
                        signchar = cs1 as libc::c_char;
                    }
                    _ => {}
                }
                if cur != &mut fw as *mut libc::c_long {
                    current_block = 2749501120807187827;
                    break;
                }
            }
            match current_block {
                17342404680090366851 => {
                    if used_dollar {
                        msg(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"fatal: must use `count$' on all formats or none\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        arg = 0 as *mut NODE;
                        current_block = 1242492412443577758;
                        break;
                    } else if cur_arg >= num_args as libc::c_ulong {
                        arg = 0 as *mut NODE;
                        toofew = 1 as libc::c_int != 0;
                        current_block = 2749501120807187827;
                    } else {
                        arg = *the_args.offset(cur_arg as isize);
                        cur_arg = cur_arg.wrapping_add(1);
                        cur_arg;
                        current_block = 1761421285656443595;
                    }
                }
                11364608634565542496 => {
                    if cur_arg > 1 as libc::c_int as libc::c_ulong {
                        msg(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"fatal: must use `count$' on all formats or none\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        current_block = 1242492412443577758;
                        break;
                    } else {
                        arg = *the_args.offset(argnum as isize);
                    }
                    current_block = 1761421285656443595;
                }
                870304920170074103 => {
                    if used_dollar {
                        msg(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"fatal: must use `count$' on all formats or none\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        arg = 0 as *mut NODE;
                        current_block = 1242492412443577758;
                        break;
                    } else if cur_arg >= num_args as libc::c_ulong {
                        arg = 0 as *mut NODE;
                        toofew = 1 as libc::c_int != 0;
                        current_block = 2749501120807187827;
                    } else {
                        arg = *the_args.offset(cur_arg as isize);
                        cur_arg = cur_arg.wrapping_add(1);
                        cur_arg;
                        current_block = 12003943868717696208;
                    }
                }
                1365401674109093360 => {
                    if cur_arg > 1 as libc::c_int as libc::c_ulong {
                        msg(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"fatal: must use `count$' on all formats or none\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        current_block = 1242492412443577758;
                        break;
                    } else {
                        arg = *the_args.offset(argnum as isize);
                    }
                    current_block = 12003943868717696208;
                }
                10282596542094995802 => {
                    if used_dollar {
                        msg(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"fatal: must use `count$' on all formats or none\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        arg = 0 as *mut NODE;
                        current_block = 1242492412443577758;
                        break;
                    } else if cur_arg >= num_args as libc::c_ulong {
                        arg = 0 as *mut NODE;
                        toofew = 1 as libc::c_int != 0;
                        current_block = 2749501120807187827;
                    } else {
                        arg = *the_args.offset(cur_arg as isize);
                        cur_arg = cur_arg.wrapping_add(1);
                        cur_arg;
                        current_block = 2346768750020253347;
                    }
                }
                9822987968968565122 => {
                    if cur_arg > 1 as libc::c_int as libc::c_ulong {
                        msg(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"fatal: must use `count$' on all formats or none\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        current_block = 1242492412443577758;
                        break;
                    } else {
                        arg = *the_args.offset(argnum as isize);
                    }
                    current_block = 2346768750020253347;
                }
                7186168334825677637 => {
                    if used_dollar {
                        msg(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"fatal: must use `count$' on all formats or none\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        arg = 0 as *mut NODE;
                        current_block = 1242492412443577758;
                        break;
                    } else if cur_arg >= num_args as libc::c_ulong {
                        arg = 0 as *mut NODE;
                        toofew = 1 as libc::c_int != 0;
                        current_block = 2749501120807187827;
                    } else {
                        arg = *the_args.offset(cur_arg as isize);
                        cur_arg = cur_arg.wrapping_add(1);
                        cur_arg;
                        current_block = 2847482619380721777;
                    }
                }
                3889686208735362918 => {
                    if cur_arg > 1 as libc::c_int as libc::c_ulong {
                        msg(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"fatal: must use `count$' on all formats or none\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        current_block = 1242492412443577758;
                        break;
                    } else {
                        arg = *the_args.offset(argnum as isize);
                    }
                    current_block = 2847482619380721777;
                }
                640215555276527818 => {
                    base += 6 as libc::c_int;
                    current_block = 11847976846684641782;
                }
                _ => {}
            }
            match current_block {
                1761421285656443595 => {
                    force_number(arg);
                    if out_of_range(arg) {
                        current_block = 1906482790067100753;
                    } else {
                        tmpval = double_to_int((*arg).sub.val.fltnum);
                        if have_prec as libc::c_int != 0
                            && prec == 0 as libc::c_int as libc::c_long
                            && tmpval == 0 as libc::c_int as libc::c_double
                        {
                            current_block = 13813617645438890915;
                        } else {
                            if tmpval < 0 as libc::c_int as libc::c_double {
                                tmpval = -tmpval;
                                sgn = 1 as libc::c_int != 0;
                            } else {
                                if tmpval == -0.0f64 {
                                    tmpval = 0.0f64;
                                }
                                sgn = 0 as libc::c_int != 0;
                            }
                            loop {
                                i = snprintf(
                                    cpbufs[1 as libc::c_int as usize].buf,
                                    cpbufs[1 as libc::c_int as usize].bufsize,
                                    b"%.0f\0" as *const u8 as *const libc::c_char,
                                    tmpval,
                                );
                                if !(i as libc::c_ulong
                                    >= cpbufs[1 as libc::c_int as usize].bufsize)
                                {
                                    break;
                                }
                                if cpbufs[1 as libc::c_int as usize].buf
                                    == (cpbufs[1 as libc::c_int as usize].stackbuf).as_mut_ptr()
                                {
                                    cpbufs[1 as libc::c_int as usize]
                                        .buf = 0 as *mut libc::c_char;
                                }
                                if i > 0 as libc::c_int {
                                    cpbufs[1 as libc::c_int as usize]
                                        .bufsize = (cpbufs[1 as libc::c_int as usize].bufsize
                                        as libc::c_ulong)
                                        .wrapping_add(
                                            if i as libc::c_ulong
                                                > cpbufs[1 as libc::c_int as usize].bufsize
                                            {
                                                i as libc::c_ulong
                                            } else {
                                                cpbufs[1 as libc::c_int as usize].bufsize
                                            },
                                        ) as size_t as size_t;
                                } else {
                                    cpbufs[1 as libc::c_int as usize]
                                        .bufsize = (cpbufs[1 as libc::c_int as usize].bufsize
                                        as libc::c_ulong)
                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                                        as size_t;
                                }
                                cpbufs[1 as libc::c_int as usize]
                                    .buf = erealloc_real(
                                    cpbufs[1 as libc::c_int as usize].buf as *mut libc::c_void,
                                    cpbufs[1 as libc::c_int as usize].bufsize,
                                    b"format_tree\0" as *const u8 as *const libc::c_char,
                                    b"cpbufs[1].buf\0" as *const u8 as *const libc::c_char,
                                    b"builtin.c\0" as *const u8 as *const libc::c_char,
                                    1266 as libc::c_int,
                                ) as *mut libc::c_char;
                            }
                            if i < 1 as libc::c_int {
                                current_block = 1906482790067100753;
                            } else {
                                quote_flag = quote_flag as libc::c_int != 0
                                    && *(loc.thousands_sep).offset(0 as libc::c_int as isize)
                                        as libc::c_int != 0 as libc::c_int;
                                chp = &mut *((*cpbufs
                                    .as_mut_ptr()
                                    .offset(1 as libc::c_int as isize))
                                    .buf)
                                    .offset((i - 1 as libc::c_int) as isize)
                                    as *mut libc::c_char;
                                jj = 0 as libc::c_int;
                                ii = jj;
                                loop {
                                    if cp == cpbufs[0 as libc::c_int as usize].buf {
                                        let mut prev: *mut libc::c_char = cpbufs[0 as libc::c_int
                                                as usize]
                                            .buf;
                                        cpbufs[0 as libc::c_int as usize]
                                            .buf = emalloc_real(
                                            (2 as libc::c_int as libc::c_ulong)
                                                .wrapping_mul(cpbufs[0 as libc::c_int as usize].bufsize),
                                            b"format_tree\0" as *const u8 as *const libc::c_char,
                                            b"cpbufs[0].buf\0" as *const u8 as *const libc::c_char,
                                            b"builtin.c\0" as *const u8 as *const libc::c_char,
                                            1276 as libc::c_int,
                                        ) as *mut libc::c_char;
                                        cp = (cpbufs[0 as libc::c_int as usize].buf)
                                            .offset(cpbufs[0 as libc::c_int as usize].bufsize as isize);
                                        memcpy(
                                            cp as *mut libc::c_void,
                                            prev as *const libc::c_void,
                                            cpbufs[0 as libc::c_int as usize].bufsize,
                                        );
                                        cpbufs[0 as libc::c_int as usize]
                                            .bufsize = (cpbufs[0 as libc::c_int as usize].bufsize
                                            as libc::c_ulong)
                                            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                                            as size_t;
                                        if prev
                                            != (cpbufs[0 as libc::c_int as usize].stackbuf).as_mut_ptr()
                                        {
                                            free(prev as *mut libc::c_void);
                                        }
                                        cend = (cpbufs[0 as libc::c_int as usize].buf)
                                            .offset(cpbufs[0 as libc::c_int as usize].bufsize as isize);
                                    }
                                    cp = cp.offset(-1);
                                    *cp = *chp;
                                    chp = chp.offset(-1);
                                    chp;
                                    i -= 1;
                                    i;
                                    if quote_flag as libc::c_int != 0
                                        && *(loc.grouping).offset(ii as isize) as libc::c_int != 0
                                        && {
                                            jj += 1;
                                            jj == *(loc.grouping).offset(ii as isize) as libc::c_int
                                        }
                                    {
                                        if i != 0 {
                                            let mut k_0: libc::c_int = 0;
                                            let mut ts: *const libc::c_char = loc.thousands_sep;
                                            k_0 = (strlen(ts))
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as libc::c_int;
                                            while k_0 >= 0 as libc::c_int {
                                                if cp == cpbufs[0 as libc::c_int as usize].buf {
                                                    let mut prev_0: *mut libc::c_char = cpbufs[0 as libc::c_int
                                                            as usize]
                                                        .buf;
                                                    cpbufs[0 as libc::c_int as usize]
                                                        .buf = emalloc_real(
                                                        (2 as libc::c_int as libc::c_ulong)
                                                            .wrapping_mul(cpbufs[0 as libc::c_int as usize].bufsize),
                                                        b"format_tree\0" as *const u8 as *const libc::c_char,
                                                        b"cpbufs[0].buf\0" as *const u8 as *const libc::c_char,
                                                        b"builtin.c\0" as *const u8 as *const libc::c_char,
                                                        1285 as libc::c_int,
                                                    ) as *mut libc::c_char;
                                                    cp = (cpbufs[0 as libc::c_int as usize].buf)
                                                        .offset(cpbufs[0 as libc::c_int as usize].bufsize as isize);
                                                    memcpy(
                                                        cp as *mut libc::c_void,
                                                        prev_0 as *const libc::c_void,
                                                        cpbufs[0 as libc::c_int as usize].bufsize,
                                                    );
                                                    cpbufs[0 as libc::c_int as usize]
                                                        .bufsize = (cpbufs[0 as libc::c_int as usize].bufsize
                                                        as libc::c_ulong)
                                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                                                        as size_t;
                                                    if prev_0
                                                        != (cpbufs[0 as libc::c_int as usize].stackbuf).as_mut_ptr()
                                                    {
                                                        free(prev_0 as *mut libc::c_void);
                                                    }
                                                    cend = (cpbufs[0 as libc::c_int as usize].buf)
                                                        .offset(cpbufs[0 as libc::c_int as usize].bufsize as isize);
                                                }
                                                cp = cp.offset(-1);
                                                *cp = *ts.offset(k_0 as isize);
                                                k_0 -= 1;
                                                k_0;
                                            }
                                        }
                                        if *(loc.grouping).offset((ii + 1 as libc::c_int) as isize)
                                            as libc::c_int == 0 as libc::c_int
                                        {
                                            jj = 0 as libc::c_int;
                                        } else if *(loc.grouping)
                                            .offset((ii + 1 as libc::c_int) as isize) as libc::c_int
                                            == 127 as libc::c_int
                                        {
                                            quote_flag = 0 as libc::c_int != 0;
                                        } else {
                                            ii += 1;
                                            ii;
                                            jj = 0 as libc::c_int;
                                        }
                                    }
                                    if !(i > 0 as libc::c_int) {
                                        break;
                                    }
                                }
                                if have_prec {
                                    while (cend.offset_from(cp) as libc::c_long) < prec {
                                        if cp == cpbufs[0 as libc::c_int as usize].buf {
                                            let mut prev_1: *mut libc::c_char = cpbufs[0 as libc::c_int
                                                    as usize]
                                                .buf;
                                            cpbufs[0 as libc::c_int as usize]
                                                .buf = emalloc_real(
                                                (2 as libc::c_int as libc::c_ulong)
                                                    .wrapping_mul(cpbufs[0 as libc::c_int as usize].bufsize),
                                                b"format_tree\0" as *const u8 as *const libc::c_char,
                                                b"cpbufs[0].buf\0" as *const u8 as *const libc::c_char,
                                                b"builtin.c\0" as *const u8 as *const libc::c_char,
                                                1303 as libc::c_int,
                                            ) as *mut libc::c_char;
                                            cp = (cpbufs[0 as libc::c_int as usize].buf)
                                                .offset(cpbufs[0 as libc::c_int as usize].bufsize as isize);
                                            memcpy(
                                                cp as *mut libc::c_void,
                                                prev_1 as *const libc::c_void,
                                                cpbufs[0 as libc::c_int as usize].bufsize,
                                            );
                                            cpbufs[0 as libc::c_int as usize]
                                                .bufsize = (cpbufs[0 as libc::c_int as usize].bufsize
                                                as libc::c_ulong)
                                                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                                                as size_t;
                                            if prev_1
                                                != (cpbufs[0 as libc::c_int as usize].stackbuf).as_mut_ptr()
                                            {
                                                free(prev_1 as *mut libc::c_void);
                                            }
                                            cend = (cpbufs[0 as libc::c_int as usize].buf)
                                                .offset(cpbufs[0 as libc::c_int as usize].bufsize as isize);
                                        }
                                        cp = cp.offset(-1);
                                        *cp = '0' as i32 as libc::c_char;
                                    }
                                }
                                if sgn {
                                    if cp == cpbufs[0 as libc::c_int as usize].buf {
                                        let mut prev_2: *mut libc::c_char = cpbufs[0 as libc::c_int
                                                as usize]
                                            .buf;
                                        cpbufs[0 as libc::c_int as usize]
                                            .buf = emalloc_real(
                                            (2 as libc::c_int as libc::c_ulong)
                                                .wrapping_mul(cpbufs[0 as libc::c_int as usize].bufsize),
                                            b"format_tree\0" as *const u8 as *const libc::c_char,
                                            b"cpbufs[0].buf\0" as *const u8 as *const libc::c_char,
                                            b"builtin.c\0" as *const u8 as *const libc::c_char,
                                            1307 as libc::c_int,
                                        ) as *mut libc::c_char;
                                        cp = (cpbufs[0 as libc::c_int as usize].buf)
                                            .offset(cpbufs[0 as libc::c_int as usize].bufsize as isize);
                                        memcpy(
                                            cp as *mut libc::c_void,
                                            prev_2 as *const libc::c_void,
                                            cpbufs[0 as libc::c_int as usize].bufsize,
                                        );
                                        cpbufs[0 as libc::c_int as usize]
                                            .bufsize = (cpbufs[0 as libc::c_int as usize].bufsize
                                            as libc::c_ulong)
                                            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                                            as size_t;
                                        if prev_2
                                            != (cpbufs[0 as libc::c_int as usize].stackbuf).as_mut_ptr()
                                        {
                                            free(prev_2 as *mut libc::c_void);
                                        }
                                        cend = (cpbufs[0 as libc::c_int as usize].buf)
                                            .offset(cpbufs[0 as libc::c_int as usize].bufsize as isize);
                                    }
                                    cp = cp.offset(-1);
                                    *cp = '-' as i32 as libc::c_char;
                                } else if signchar != 0 {
                                    if cp == cpbufs[0 as libc::c_int as usize].buf {
                                        let mut prev_3: *mut libc::c_char = cpbufs[0 as libc::c_int
                                                as usize]
                                            .buf;
                                        cpbufs[0 as libc::c_int as usize]
                                            .buf = emalloc_real(
                                            (2 as libc::c_int as libc::c_ulong)
                                                .wrapping_mul(cpbufs[0 as libc::c_int as usize].bufsize),
                                            b"format_tree\0" as *const u8 as *const libc::c_char,
                                            b"cpbufs[0].buf\0" as *const u8 as *const libc::c_char,
                                            b"builtin.c\0" as *const u8 as *const libc::c_char,
                                            1309 as libc::c_int,
                                        ) as *mut libc::c_char;
                                        cp = (cpbufs[0 as libc::c_int as usize].buf)
                                            .offset(cpbufs[0 as libc::c_int as usize].bufsize as isize);
                                        memcpy(
                                            cp as *mut libc::c_void,
                                            prev_3 as *const libc::c_void,
                                            cpbufs[0 as libc::c_int as usize].bufsize,
                                        );
                                        cpbufs[0 as libc::c_int as usize]
                                            .bufsize = (cpbufs[0 as libc::c_int as usize].bufsize
                                            as libc::c_ulong)
                                            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                                            as size_t;
                                        if prev_3
                                            != (cpbufs[0 as libc::c_int as usize].stackbuf).as_mut_ptr()
                                        {
                                            free(prev_3 as *mut libc::c_void);
                                        }
                                        cend = (cpbufs[0 as libc::c_int as usize].buf)
                                            .offset(cpbufs[0 as libc::c_int as usize].bufsize as isize);
                                    }
                                    cp = cp.offset(-1);
                                    *cp = signchar;
                                }
                                if !lj
                                    && (zero_flag as libc::c_int != 0 && !have_prec
                                        || fw == 0 as libc::c_int as libc::c_long
                                            && have_prec as libc::c_int != 0)
                                {
                                    fill = zero_string.as_ptr();
                                }
                                if prec > fw {
                                    fw = prec;
                                }
                                prec = cend.offset_from(cp) as libc::c_long;
                                if fw > prec && !lj && fill != sp.as_ptr()
                                    && (*cp as libc::c_int == '-' as i32
                                        || signchar as libc::c_int != 0)
                                {
                                    if ofre < 1 as libc::c_int as libc::c_ulong {
                                        let mut olen_1: size_t = obufout.offset_from(obuf)
                                            as libc::c_long as size_t;
                                        obuf = erealloc_real(
                                            obuf as *mut libc::c_void,
                                            osiz.wrapping_mul(2 as libc::c_int as libc::c_ulong),
                                            b"format_tree\0" as *const u8 as *const libc::c_char,
                                            b"obuf\0" as *const u8 as *const libc::c_char,
                                            b"builtin.c\0" as *const u8 as *const libc::c_char,
                                            1327 as libc::c_int,
                                        ) as *mut libc::c_char;
                                        ofre = (ofre as libc::c_ulong).wrapping_add(osiz) as size_t
                                            as size_t;
                                        osiz = (osiz as libc::c_ulong)
                                            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                                            as size_t;
                                        obufout = obuf.offset(olen_1 as isize);
                                    }
                                    let fresh12 = obufout;
                                    obufout = obufout.offset(1);
                                    *fresh12 = *cp;
                                    ofre = ofre.wrapping_sub(1);
                                    ofre;
                                    cp = cp.offset(1);
                                    cp;
                                    prec -= 1;
                                    prec;
                                    fw -= 1;
                                    fw;
                                }
                                current_block = 13813617645438890915;
                            }
                        }
                    }
                }
                12003943868717696208 => {
                    arg = force_string_fmt(arg, CONVFMT, CONVFMTidx);
                    if fw == 0 as libc::c_int as libc::c_long && !have_prec {
                        prec = (*arg).sub.val.slen as libc::c_long;
                    } else {
                        char_count = mbc_char_count(
                            (*arg).sub.val.sp,
                            (*arg).sub.val.slen,
                        );
                        if !have_prec || prec as libc::c_ulong > char_count {
                            prec = char_count as libc::c_long;
                        }
                    }
                    cp = (*arg).sub.val.sp;
                    current_block = 13813617645438890915;
                }
                2346768750020253347 => {
                    fixtype(arg);
                    if (*arg).flags as libc::c_uint
                        & NUMBER as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint
                    {
                        uval = (*arg).sub.val.fltnum as uintmax_t;
                        if gawk_mb_cur_max > 1 as libc::c_int {
                            let mut buf: [libc::c_char; 100] = [0; 100];
                            let mut wc: wchar_t = 0;
                            let mut mbs: mbstate_t = mbstate_t {
                                __count: 0,
                                __value: C2RustUnnamed { __wch: 0 },
                            };
                            let mut count: size_t = 0;
                            memset(
                                &mut mbs as *mut mbstate_t as *mut libc::c_void,
                                0 as libc::c_int,
                                ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
                            );
                            if (::std::mem::size_of::<wchar_t>() as libc::c_ulong)
                                < 4 as libc::c_int as libc::c_ulong
                                && uval > 0xffff as libc::c_int as libc::c_ulong
                            {
                                if do_flags as libc::c_uint
                                    & (DO_LINT_INVALID as libc::c_int
                                        | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                {
                                    (set_loc
                                        as unsafe extern "C" fn(
                                            *const libc::c_char,
                                            libc::c_int,
                                        ) -> ())(
                                        b"builtin.c\0" as *const u8 as *const libc::c_char,
                                        1138 as libc::c_int,
                                    );
                                    (Some(lintfunc.unwrap()))
                                        .unwrap()(
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"[s]printf: value %g is too big for %%c format\0"
                                                as *const u8 as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                        (*arg).sub.val.fltnum,
                                    );
                                }
                                current_block = 12350242817162068077;
                            } else {
                                wc = uval as wchar_t;
                                count = wcrtomb(buf.as_mut_ptr(), wc, &mut mbs);
                                if count == 0 as libc::c_int as libc::c_ulong
                                    || count == -(1 as libc::c_int) as size_t
                                {
                                    if do_flags as libc::c_uint
                                        & (DO_LINT_INVALID as libc::c_int
                                            | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                    {
                                        (set_loc
                                            as unsafe extern "C" fn(
                                                *const libc::c_char,
                                                libc::c_int,
                                            ) -> ())(
                                            b"builtin.c\0" as *const u8 as *const libc::c_char,
                                            1151 as libc::c_int,
                                        );
                                        (Some(lintfunc.unwrap()))
                                            .unwrap()(
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"[s]printf: value %g is not a valid wide character\0"
                                                    as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            (*arg).sub.val.fltnum,
                                        );
                                    }
                                    current_block = 12350242817162068077;
                                } else {
                                    memcpy(
                                        cpbufs[0 as libc::c_int as usize].buf as *mut libc::c_void,
                                        buf.as_mut_ptr() as *const libc::c_void,
                                        count,
                                    );
                                    prec = count as libc::c_long;
                                    cp = cpbufs[0 as libc::c_int as usize].buf;
                                    current_block = 13813617645438890915;
                                }
                            }
                        } else {
                            current_block = 12350242817162068077;
                        }
                        match current_block {
                            13813617645438890915 => {}
                            _ => {
                                *(cpbufs[0 as libc::c_int as usize].buf)
                                    .offset(0 as libc::c_int as isize) = uval as libc::c_char;
                                prec = 1 as libc::c_int as libc::c_long;
                                cp = cpbufs[0 as libc::c_int as usize].buf;
                            }
                        }
                    } else {
                        cp = (*arg).sub.val.sp;
                        prec = 1 as libc::c_int as libc::c_long;
                        if gawk_mb_cur_max > 1 as libc::c_int {
                            let mut state_0: mbstate_t = mbstate_t {
                                __count: 0,
                                __value: C2RustUnnamed { __wch: 0 },
                            };
                            let mut count_0: size_t = 0;
                            memset(
                                &mut state_0 as *mut mbstate_t as *mut libc::c_void,
                                0 as libc::c_int,
                                ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
                            );
                            count_0 = mbrlen(cp, (*arg).sub.val.slen, &mut state_0);
                            if count_0 != -(1 as libc::c_int) as size_t
                                && count_0 != -(2 as libc::c_int) as size_t
                                && count_0 > 0 as libc::c_int as libc::c_ulong
                            {
                                prec = count_0 as libc::c_long;
                                if fw > 0 as libc::c_int as libc::c_long {
                                    fw = (fw as libc::c_ulong)
                                        .wrapping_add(
                                            count_0.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                        ) as libc::c_long as libc::c_long;
                                }
                            }
                        }
                    }
                    current_block = 13813617645438890915;
                }
                2847482619380721777 => {
                    force_number(arg);
                    if 0 as libc::c_int == 0 {
                        tmpval = (*arg).sub.val.fltnum;
                    }
                    if out_of_range(arg) {
                        current_block = 1906482790067100753;
                    } else {
                        current_block = 17406244059574714485;
                    }
                }
                11847976846684641782 => {
                    base += 2 as libc::c_int;
                    current_block = 4644176752869242724;
                }
                _ => {}
            }
            match current_block {
                4644176752869242724 => {
                    base += 8 as libc::c_int;
                    need_format = 0 as libc::c_int != 0;
                    if argnum > 0 as libc::c_int as libc::c_long {
                        if cur_arg > 1 as libc::c_int as libc::c_ulong {
                            msg(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"fatal: must use `count$' on all formats or none\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                            current_block = 1242492412443577758;
                            break;
                        } else {
                            arg = *the_args.offset(argnum as isize);
                        }
                        current_block = 2589719962955437281;
                    } else if used_dollar {
                        msg(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"fatal: must use `count$' on all formats or none\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        arg = 0 as *mut NODE;
                        current_block = 1242492412443577758;
                        break;
                    } else if cur_arg >= num_args as libc::c_ulong {
                        arg = 0 as *mut NODE;
                        toofew = 1 as libc::c_int != 0;
                        current_block = 2749501120807187827;
                    } else {
                        arg = *the_args.offset(cur_arg as isize);
                        cur_arg = cur_arg.wrapping_add(1);
                        cur_arg;
                        current_block = 2589719962955437281;
                    }
                    match current_block {
                        2749501120807187827 => {}
                        _ => {
                            force_number(arg);
                            if out_of_range(arg) {
                                current_block = 1906482790067100753;
                            } else {
                                tmpval = (*arg).sub.val.fltnum;
                                if !alt && have_prec as libc::c_int != 0
                                    && prec == 0 as libc::c_int as libc::c_long
                                    && tmpval == 0 as libc::c_int as libc::c_double
                                {
                                    current_block = 13813617645438890915;
                                } else {
                                    if tmpval < 0 as libc::c_int as libc::c_double {
                                        uval = tmpval as intmax_t as uintmax_t;
                                        if uval as intmax_t as libc::c_double
                                            != double_to_int(tmpval)
                                        {
                                            current_block = 1906482790067100753;
                                        } else {
                                            current_block = 11060486315799977401;
                                        }
                                    } else {
                                        uval = tmpval as uintmax_t;
                                        if uval as libc::c_double != double_to_int(tmpval) {
                                            current_block = 1906482790067100753;
                                        } else {
                                            current_block = 11060486315799977401;
                                        }
                                    }
                                    match current_block {
                                        1906482790067100753 => {}
                                        _ => {
                                            quote_flag = quote_flag as libc::c_int != 0
                                                && *(loc.thousands_sep).offset(0 as libc::c_int as isize)
                                                    as libc::c_int != 0 as libc::c_int;
                                            if !lj
                                                && (zero_flag as libc::c_int != 0 && !have_prec
                                                    || fw == 0 as libc::c_int as libc::c_long
                                                        && have_prec as libc::c_int != 0)
                                            {
                                                fill = zero_string.as_ptr();
                                            }
                                            jj = 0 as libc::c_int;
                                            ii = jj;
                                            loop {
                                                if cp == cpbufs[0 as libc::c_int as usize].buf {
                                                    let mut prev_4: *mut libc::c_char = cpbufs[0 as libc::c_int
                                                            as usize]
                                                        .buf;
                                                    cpbufs[0 as libc::c_int as usize]
                                                        .buf = emalloc_real(
                                                        (2 as libc::c_int as libc::c_ulong)
                                                            .wrapping_mul(cpbufs[0 as libc::c_int as usize].bufsize),
                                                        b"format_tree\0" as *const u8 as *const libc::c_char,
                                                        b"cpbufs[0].buf\0" as *const u8 as *const libc::c_char,
                                                        b"builtin.c\0" as *const u8 as *const libc::c_char,
                                                        1463 as libc::c_int,
                                                    ) as *mut libc::c_char;
                                                    cp = (cpbufs[0 as libc::c_int as usize].buf)
                                                        .offset(cpbufs[0 as libc::c_int as usize].bufsize as isize);
                                                    memcpy(
                                                        cp as *mut libc::c_void,
                                                        prev_4 as *const libc::c_void,
                                                        cpbufs[0 as libc::c_int as usize].bufsize,
                                                    );
                                                    cpbufs[0 as libc::c_int as usize]
                                                        .bufsize = (cpbufs[0 as libc::c_int as usize].bufsize
                                                        as libc::c_ulong)
                                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                                                        as size_t;
                                                    if prev_4
                                                        != (cpbufs[0 as libc::c_int as usize].stackbuf).as_mut_ptr()
                                                    {
                                                        free(prev_4 as *mut libc::c_void);
                                                    }
                                                    cend = (cpbufs[0 as libc::c_int as usize].buf)
                                                        .offset(cpbufs[0 as libc::c_int as usize].bufsize as isize);
                                                }
                                                cp = cp.offset(-1);
                                                *cp = *chbuf
                                                    .offset(uval.wrapping_rem(base as libc::c_ulong) as isize);
                                                uval = (uval as libc::c_ulong)
                                                    .wrapping_div(base as libc::c_ulong) as uintmax_t
                                                    as uintmax_t;
                                                if base == 10 as libc::c_int
                                                    && quote_flag as libc::c_int != 0
                                                    && *(loc.grouping).offset(ii as isize) as libc::c_int != 0
                                                    && {
                                                        jj += 1;
                                                        jj == *(loc.grouping).offset(ii as isize) as libc::c_int
                                                    }
                                                {
                                                    if uval != 0 {
                                                        let mut k_1: libc::c_int = 0;
                                                        let mut ts_0: *const libc::c_char = loc.thousands_sep;
                                                        k_1 = (strlen(ts_0))
                                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                            as libc::c_int;
                                                        while k_1 >= 0 as libc::c_int {
                                                            if cp == cpbufs[0 as libc::c_int as usize].buf {
                                                                let mut prev_5: *mut libc::c_char = cpbufs[0 as libc::c_int
                                                                        as usize]
                                                                    .buf;
                                                                cpbufs[0 as libc::c_int as usize]
                                                                    .buf = emalloc_real(
                                                                    (2 as libc::c_int as libc::c_ulong)
                                                                        .wrapping_mul(cpbufs[0 as libc::c_int as usize].bufsize),
                                                                    b"format_tree\0" as *const u8 as *const libc::c_char,
                                                                    b"cpbufs[0].buf\0" as *const u8 as *const libc::c_char,
                                                                    b"builtin.c\0" as *const u8 as *const libc::c_char,
                                                                    1472 as libc::c_int,
                                                                ) as *mut libc::c_char;
                                                                cp = (cpbufs[0 as libc::c_int as usize].buf)
                                                                    .offset(cpbufs[0 as libc::c_int as usize].bufsize as isize);
                                                                memcpy(
                                                                    cp as *mut libc::c_void,
                                                                    prev_5 as *const libc::c_void,
                                                                    cpbufs[0 as libc::c_int as usize].bufsize,
                                                                );
                                                                cpbufs[0 as libc::c_int as usize]
                                                                    .bufsize = (cpbufs[0 as libc::c_int as usize].bufsize
                                                                    as libc::c_ulong)
                                                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                                                                    as size_t;
                                                                if prev_5
                                                                    != (cpbufs[0 as libc::c_int as usize].stackbuf).as_mut_ptr()
                                                                {
                                                                    free(prev_5 as *mut libc::c_void);
                                                                }
                                                                cend = (cpbufs[0 as libc::c_int as usize].buf)
                                                                    .offset(cpbufs[0 as libc::c_int as usize].bufsize as isize);
                                                            }
                                                            cp = cp.offset(-1);
                                                            *cp = *ts_0.offset(k_1 as isize);
                                                            k_1 -= 1;
                                                            k_1;
                                                        }
                                                    }
                                                    if *(loc.grouping).offset((ii + 1 as libc::c_int) as isize)
                                                        as libc::c_int == 0 as libc::c_int
                                                    {
                                                        jj = 0 as libc::c_int;
                                                    } else if *(loc.grouping)
                                                        .offset((ii + 1 as libc::c_int) as isize) as libc::c_int
                                                        == 127 as libc::c_int
                                                    {
                                                        quote_flag = 0 as libc::c_int != 0;
                                                    } else {
                                                        ii += 1;
                                                        ii;
                                                        jj = 0 as libc::c_int;
                                                    }
                                                }
                                                if !(uval > 0 as libc::c_int as libc::c_ulong) {
                                                    break;
                                                }
                                            }
                                            if have_prec {
                                                while (cend.offset_from(cp) as libc::c_long) < prec {
                                                    if cp == cpbufs[0 as libc::c_int as usize].buf {
                                                        let mut prev_6: *mut libc::c_char = cpbufs[0 as libc::c_int
                                                                as usize]
                                                            .buf;
                                                        cpbufs[0 as libc::c_int as usize]
                                                            .buf = emalloc_real(
                                                            (2 as libc::c_int as libc::c_ulong)
                                                                .wrapping_mul(cpbufs[0 as libc::c_int as usize].bufsize),
                                                            b"format_tree\0" as *const u8 as *const libc::c_char,
                                                            b"cpbufs[0].buf\0" as *const u8 as *const libc::c_char,
                                                            b"builtin.c\0" as *const u8 as *const libc::c_char,
                                                            1490 as libc::c_int,
                                                        ) as *mut libc::c_char;
                                                        cp = (cpbufs[0 as libc::c_int as usize].buf)
                                                            .offset(cpbufs[0 as libc::c_int as usize].bufsize as isize);
                                                        memcpy(
                                                            cp as *mut libc::c_void,
                                                            prev_6 as *const libc::c_void,
                                                            cpbufs[0 as libc::c_int as usize].bufsize,
                                                        );
                                                        cpbufs[0 as libc::c_int as usize]
                                                            .bufsize = (cpbufs[0 as libc::c_int as usize].bufsize
                                                            as libc::c_ulong)
                                                            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                                                            as size_t;
                                                        if prev_6
                                                            != (cpbufs[0 as libc::c_int as usize].stackbuf).as_mut_ptr()
                                                        {
                                                            free(prev_6 as *mut libc::c_void);
                                                        }
                                                        cend = (cpbufs[0 as libc::c_int as usize].buf)
                                                            .offset(cpbufs[0 as libc::c_int as usize].bufsize as isize);
                                                    }
                                                    cp = cp.offset(-1);
                                                    *cp = '0' as i32 as libc::c_char;
                                                }
                                            }
                                            if alt as libc::c_int != 0
                                                && tmpval != 0 as libc::c_int as libc::c_double
                                            {
                                                if base == 16 as libc::c_int {
                                                    if cp == cpbufs[0 as libc::c_int as usize].buf {
                                                        let mut prev_7: *mut libc::c_char = cpbufs[0 as libc::c_int
                                                                as usize]
                                                            .buf;
                                                        cpbufs[0 as libc::c_int as usize]
                                                            .buf = emalloc_real(
                                                            (2 as libc::c_int as libc::c_ulong)
                                                                .wrapping_mul(cpbufs[0 as libc::c_int as usize].bufsize),
                                                            b"format_tree\0" as *const u8 as *const libc::c_char,
                                                            b"cpbufs[0].buf\0" as *const u8 as *const libc::c_char,
                                                            b"builtin.c\0" as *const u8 as *const libc::c_char,
                                                            1495 as libc::c_int,
                                                        ) as *mut libc::c_char;
                                                        cp = (cpbufs[0 as libc::c_int as usize].buf)
                                                            .offset(cpbufs[0 as libc::c_int as usize].bufsize as isize);
                                                        memcpy(
                                                            cp as *mut libc::c_void,
                                                            prev_7 as *const libc::c_void,
                                                            cpbufs[0 as libc::c_int as usize].bufsize,
                                                        );
                                                        cpbufs[0 as libc::c_int as usize]
                                                            .bufsize = (cpbufs[0 as libc::c_int as usize].bufsize
                                                            as libc::c_ulong)
                                                            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                                                            as size_t;
                                                        if prev_7
                                                            != (cpbufs[0 as libc::c_int as usize].stackbuf).as_mut_ptr()
                                                        {
                                                            free(prev_7 as *mut libc::c_void);
                                                        }
                                                        cend = (cpbufs[0 as libc::c_int as usize].buf)
                                                            .offset(cpbufs[0 as libc::c_int as usize].bufsize as isize);
                                                    }
                                                    cp = cp.offset(-1);
                                                    *cp = cs1 as libc::c_char;
                                                    if cp == cpbufs[0 as libc::c_int as usize].buf {
                                                        let mut prev_8: *mut libc::c_char = cpbufs[0 as libc::c_int
                                                                as usize]
                                                            .buf;
                                                        cpbufs[0 as libc::c_int as usize]
                                                            .buf = emalloc_real(
                                                            (2 as libc::c_int as libc::c_ulong)
                                                                .wrapping_mul(cpbufs[0 as libc::c_int as usize].bufsize),
                                                            b"format_tree\0" as *const u8 as *const libc::c_char,
                                                            b"cpbufs[0].buf\0" as *const u8 as *const libc::c_char,
                                                            b"builtin.c\0" as *const u8 as *const libc::c_char,
                                                            1496 as libc::c_int,
                                                        ) as *mut libc::c_char;
                                                        cp = (cpbufs[0 as libc::c_int as usize].buf)
                                                            .offset(cpbufs[0 as libc::c_int as usize].bufsize as isize);
                                                        memcpy(
                                                            cp as *mut libc::c_void,
                                                            prev_8 as *const libc::c_void,
                                                            cpbufs[0 as libc::c_int as usize].bufsize,
                                                        );
                                                        cpbufs[0 as libc::c_int as usize]
                                                            .bufsize = (cpbufs[0 as libc::c_int as usize].bufsize
                                                            as libc::c_ulong)
                                                            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                                                            as size_t;
                                                        if prev_8
                                                            != (cpbufs[0 as libc::c_int as usize].stackbuf).as_mut_ptr()
                                                        {
                                                            free(prev_8 as *mut libc::c_void);
                                                        }
                                                        cend = (cpbufs[0 as libc::c_int as usize].buf)
                                                            .offset(cpbufs[0 as libc::c_int as usize].bufsize as isize);
                                                    }
                                                    cp = cp.offset(-1);
                                                    *cp = '0' as i32 as libc::c_char;
                                                    if fill != sp.as_ptr() {
                                                        while 2 as libc::c_int as libc::c_ulong > ofre {
                                                            let mut olen_2: size_t = obufout.offset_from(obuf)
                                                                as libc::c_long as size_t;
                                                            obuf = erealloc_real(
                                                                obuf as *mut libc::c_void,
                                                                osiz.wrapping_mul(2 as libc::c_int as libc::c_ulong),
                                                                b"format_tree\0" as *const u8 as *const libc::c_char,
                                                                b"obuf\0" as *const u8 as *const libc::c_char,
                                                                b"builtin.c\0" as *const u8 as *const libc::c_char,
                                                                1498 as libc::c_int,
                                                            ) as *mut libc::c_char;
                                                            ofre = (ofre as libc::c_ulong).wrapping_add(osiz) as size_t
                                                                as size_t;
                                                            osiz = (osiz as libc::c_ulong)
                                                                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                                                                as size_t;
                                                            obufout = obuf.offset(olen_2 as isize);
                                                        }
                                                        memcpy(
                                                            obufout as *mut libc::c_void,
                                                            cp as *const libc::c_void,
                                                            2 as libc::c_int as size_t,
                                                        );
                                                        obufout = obufout.offset(2 as libc::c_int as isize);
                                                        ofre = (ofre as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong) as size_t
                                                            as size_t;
                                                        cp = cp.offset(2 as libc::c_int as isize);
                                                        fw -= 2 as libc::c_int as libc::c_long;
                                                    }
                                                } else if base == 8 as libc::c_int {
                                                    if cp == cpbufs[0 as libc::c_int as usize].buf {
                                                        let mut prev_9: *mut libc::c_char = cpbufs[0 as libc::c_int
                                                                as usize]
                                                            .buf;
                                                        cpbufs[0 as libc::c_int as usize]
                                                            .buf = emalloc_real(
                                                            (2 as libc::c_int as libc::c_ulong)
                                                                .wrapping_mul(cpbufs[0 as libc::c_int as usize].bufsize),
                                                            b"format_tree\0" as *const u8 as *const libc::c_char,
                                                            b"cpbufs[0].buf\0" as *const u8 as *const libc::c_char,
                                                            b"builtin.c\0" as *const u8 as *const libc::c_char,
                                                            1503 as libc::c_int,
                                                        ) as *mut libc::c_char;
                                                        cp = (cpbufs[0 as libc::c_int as usize].buf)
                                                            .offset(cpbufs[0 as libc::c_int as usize].bufsize as isize);
                                                        memcpy(
                                                            cp as *mut libc::c_void,
                                                            prev_9 as *const libc::c_void,
                                                            cpbufs[0 as libc::c_int as usize].bufsize,
                                                        );
                                                        cpbufs[0 as libc::c_int as usize]
                                                            .bufsize = (cpbufs[0 as libc::c_int as usize].bufsize
                                                            as libc::c_ulong)
                                                            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                                                            as size_t;
                                                        if prev_9
                                                            != (cpbufs[0 as libc::c_int as usize].stackbuf).as_mut_ptr()
                                                        {
                                                            free(prev_9 as *mut libc::c_void);
                                                        }
                                                        cend = (cpbufs[0 as libc::c_int as usize].buf)
                                                            .offset(cpbufs[0 as libc::c_int as usize].bufsize as isize);
                                                    }
                                                    cp = cp.offset(-1);
                                                    *cp = '0' as i32 as libc::c_char;
                                                }
                                            }
                                            base = 0 as libc::c_int;
                                            if prec > fw {
                                                fw = prec;
                                            }
                                            prec = cend.offset_from(cp) as libc::c_long;
                                            current_block = 13813617645438890915;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            match current_block {
                13813617645438890915 => {
                    if !lj {
                        while fw > prec {
                            if ofre < 1 as libc::c_int as libc::c_ulong {
                                let mut olen_3: size_t = obufout.offset_from(obuf)
                                    as libc::c_long as size_t;
                                obuf = erealloc_real(
                                    obuf as *mut libc::c_void,
                                    osiz.wrapping_mul(2 as libc::c_int as libc::c_ulong),
                                    b"format_tree\0" as *const u8 as *const libc::c_char,
                                    b"obuf\0" as *const u8 as *const libc::c_char,
                                    b"builtin.c\0" as *const u8 as *const libc::c_char,
                                    1512 as libc::c_int,
                                ) as *mut libc::c_char;
                                ofre = (ofre as libc::c_ulong).wrapping_add(osiz) as size_t
                                    as size_t;
                                osiz = (osiz as libc::c_ulong)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                                    as size_t;
                                obufout = obuf.offset(olen_3 as isize);
                            }
                            let fresh13 = obufout;
                            obufout = obufout.offset(1);
                            *fresh13 = *fill;
                            ofre = ofre.wrapping_sub(1);
                            ofre;
                            fw -= 1;
                            fw;
                        }
                    }
                    copy_count = prec as size_t;
                    if !(fw == 0 as libc::c_int as libc::c_long && !have_prec) {
                        if gawk_mb_cur_max > 1 as libc::c_int {
                            if cs1 == 's' as i32 {
                                copy_count = mbc_byte_count(
                                    (*arg).sub.val.sp,
                                    prec as size_t,
                                );
                            }
                        }
                    }
                    if copy_count != 0 {
                        while copy_count > ofre {
                            let mut olen_4: size_t = obufout.offset_from(obuf)
                                as libc::c_long as size_t;
                            obuf = erealloc_real(
                                obuf as *mut libc::c_void,
                                osiz.wrapping_mul(2 as libc::c_int as libc::c_ulong),
                                b"format_tree\0" as *const u8 as *const libc::c_char,
                                b"obuf\0" as *const u8 as *const libc::c_char,
                                b"builtin.c\0" as *const u8 as *const libc::c_char,
                                1528 as libc::c_int,
                            ) as *mut libc::c_char;
                            ofre = (ofre as libc::c_ulong).wrapping_add(osiz) as size_t
                                as size_t;
                            osiz = (osiz as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                                as size_t;
                            obufout = obuf.offset(olen_4 as isize);
                        }
                        memcpy(
                            obufout as *mut libc::c_void,
                            cp as *const libc::c_void,
                            copy_count,
                        );
                        obufout = obufout.offset(copy_count as isize);
                        ofre = (ofre as libc::c_ulong).wrapping_sub(copy_count) as size_t
                            as size_t;
                    }
                    while fw > prec {
                        if ofre < 1 as libc::c_int as libc::c_ulong {
                            let mut olen_5: size_t = obufout.offset_from(obuf)
                                as libc::c_long as size_t;
                            obuf = erealloc_real(
                                obuf as *mut libc::c_void,
                                osiz.wrapping_mul(2 as libc::c_int as libc::c_ulong),
                                b"format_tree\0" as *const u8 as *const libc::c_char,
                                b"obuf\0" as *const u8 as *const libc::c_char,
                                b"builtin.c\0" as *const u8 as *const libc::c_char,
                                1530 as libc::c_int,
                            ) as *mut libc::c_char;
                            ofre = (ofre as libc::c_ulong).wrapping_add(osiz) as size_t
                                as size_t;
                            osiz = (osiz as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                                as size_t;
                            obufout = obuf.offset(olen_5 as isize);
                        }
                        let fresh14 = obufout;
                        obufout = obufout.offset(1);
                        *fresh14 = *fill;
                        ofre = ofre.wrapping_sub(1);
                        ofre;
                        fw -= 1;
                        fw;
                    }
                    s0 = s1;
                    current_block = 2749501120807187827;
                }
                1906482790067100753 => {
                    nan_inf_val = format_nan_inf(arg, cs1 as libc::c_char);
                    if do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint
                        != 0 || magic_posix_flag as libc::c_int != 0
                        || nan_inf_val.is_null()
                    {
                        if do_flags as libc::c_uint
                            & (DO_LINT_INVALID as libc::c_int
                                | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                            && do_flags as libc::c_uint
                                & DO_POSIX as libc::c_int as libc::c_uint == 0
                            && !magic_posix_flag
                        {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    libc::c_int,
                                ) -> ())(
                                b"builtin.c\0" as *const u8 as *const libc::c_char,
                                1544 as libc::c_int,
                            );
                            (Some(lintfunc.unwrap()))
                                .unwrap()(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"[s]printf: value %g is out of range for `%%%c' format\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                tmpval,
                                cs1,
                            );
                        }
                        tmpval = (*arg).sub.val.fltnum;
                        if (strchr(
                            b"aAeEfFgG\0" as *const u8 as *const libc::c_char,
                            cs1,
                        ))
                            .is_null()
                        {
                            cs1 = 'g' as i32;
                        }
                        current_block = 17406244059574714485;
                    } else {
                        if do_flags as libc::c_uint
                            & (DO_LINT_INVALID as libc::c_int
                                | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                        {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    libc::c_int,
                                ) -> ())(
                                b"builtin.c\0" as *const u8 as *const libc::c_char,
                                1552 as libc::c_int,
                            );
                            (Some(lintfunc.unwrap()))
                                .unwrap()(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"[s]printf: value %s is out of range for `%%%c' format\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                nan_inf_val,
                                cs1,
                            );
                        }
                        if strlen(nan_inf_val) != 0 {
                            while strlen(nan_inf_val) > ofre {
                                let mut olen_6: size_t = obufout.offset_from(obuf)
                                    as libc::c_long as size_t;
                                obuf = erealloc_real(
                                    obuf as *mut libc::c_void,
                                    osiz.wrapping_mul(2 as libc::c_int as libc::c_ulong),
                                    b"format_tree\0" as *const u8 as *const libc::c_char,
                                    b"obuf\0" as *const u8 as *const libc::c_char,
                                    b"builtin.c\0" as *const u8 as *const libc::c_char,
                                    1554 as libc::c_int,
                                ) as *mut libc::c_char;
                                ofre = (ofre as libc::c_ulong).wrapping_add(osiz) as size_t
                                    as size_t;
                                osiz = (osiz as libc::c_ulong)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                                    as size_t;
                                obufout = obuf.offset(olen_6 as isize);
                            }
                            memcpy(
                                obufout as *mut libc::c_void,
                                nan_inf_val as *const libc::c_void,
                                strlen(nan_inf_val),
                            );
                            obufout = obufout.offset(strlen(nan_inf_val) as isize);
                            ofre = (ofre as libc::c_ulong)
                                .wrapping_sub(strlen(nan_inf_val)) as size_t as size_t;
                        }
                        s0 = s1;
                        current_block = 2749501120807187827;
                    }
                }
                _ => {}
            }
            match current_block {
                17406244059574714485 => {
                    if !have_prec {
                        prec = 6 as libc::c_int as libc::c_long;
                    }
                    if (fw + prec + 11 as libc::c_int as libc::c_long) as libc::c_ulong
                        >= ofre
                    {
                        let mut olen_7: size_t = obufout.offset_from(obuf)
                            as libc::c_long as size_t;
                        let mut delta: size_t = osiz
                            .wrapping_add(fw as libc::c_ulong)
                            .wrapping_add(prec as libc::c_ulong)
                            .wrapping_add(11 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(ofre);
                        obuf = erealloc_real(
                            obuf as *mut libc::c_void,
                            osiz.wrapping_add(delta),
                            b"format_tree\0" as *const u8 as *const libc::c_char,
                            b"obuf\0" as *const u8 as *const libc::c_char,
                            b"builtin.c\0" as *const u8 as *const libc::c_char,
                            1607 as libc::c_int,
                        ) as *mut libc::c_char;
                        obufout = obuf.offset(olen_7 as isize);
                        ofre = (ofre as libc::c_ulong).wrapping_add(delta) as size_t
                            as size_t;
                        osiz = (osiz as libc::c_ulong).wrapping_add(delta) as size_t
                            as size_t;
                    }
                    cp = cpbufs[0 as libc::c_int as usize].buf;
                    let fresh15 = cp;
                    cp = cp.offset(1);
                    *fresh15 = '%' as i32 as libc::c_char;
                    if lj {
                        let fresh16 = cp;
                        cp = cp.offset(1);
                        *fresh16 = '-' as i32 as libc::c_char;
                    }
                    if signchar != 0 {
                        let fresh17 = cp;
                        cp = cp.offset(1);
                        *fresh17 = signchar;
                    }
                    if alt {
                        let fresh18 = cp;
                        cp = cp.offset(1);
                        *fresh18 = '#' as i32 as libc::c_char;
                    }
                    if zero_flag {
                        let fresh19 = cp;
                        cp = cp.offset(1);
                        *fresh19 = '0' as i32 as libc::c_char;
                    }
                    if quote_flag {
                        let fresh20 = cp;
                        cp = cp.offset(1);
                        *fresh20 = '\'' as i32 as libc::c_char;
                    }
                    if quote_flag as libc::c_int != 0 && use_lc_numeric == 0 {
                        setlocale(
                            1 as libc::c_int,
                            b"\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    need_to_add_thousands = 0 as libc::c_int != 0;
                    match fmt_type as libc::c_uint {
                        _ => {}
                    }
                    if have_prec as libc::c_int != 0
                        || ({
                            let mut __res: libc::c_int = 0;
                            if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                > 1 as libc::c_int as libc::c_ulong
                            {
                                if 0 != 0 {
                                    let mut __c: libc::c_int = cs1;
                                    __res = if __c < -(128 as libc::c_int)
                                        || __c > 255 as libc::c_int
                                    {
                                        __c
                                    } else {
                                        *(*__ctype_tolower_loc()).offset(__c as isize)
                                    };
                                } else {
                                    __res = tolower(cs1);
                                }
                            } else {
                                __res = *(*__ctype_tolower_loc()).offset(cs1 as isize);
                            }
                            __res
                        }) != 'a' as i32
                    {
                        sprintf(cp, b"*.*%c\0" as *const u8 as *const libc::c_char, cs1);
                        loop {
                            nc = snprintf(
                                obufout,
                                ofre,
                                cpbufs[0 as libc::c_int as usize].buf,
                                fw as libc::c_int,
                                prec as libc::c_int,
                                tmpval,
                            );
                            if !(nc >= ofre as libc::c_int) {
                                break;
                            }
                            if nc as libc::c_ulong >= ofre {
                                let mut olen_8: size_t = obufout.offset_from(obuf)
                                    as libc::c_long as size_t;
                                let mut delta_0: size_t = osiz
                                    .wrapping_add(nc as libc::c_ulong)
                                    .wrapping_sub(ofre);
                                obuf = erealloc_real(
                                    obuf as *mut libc::c_void,
                                    osiz.wrapping_add(delta_0),
                                    b"format_tree\0" as *const u8 as *const libc::c_char,
                                    b"obuf\0" as *const u8 as *const libc::c_char,
                                    b"builtin.c\0" as *const u8 as *const libc::c_char,
                                    1656 as libc::c_int,
                                ) as *mut libc::c_char;
                                obufout = obuf.offset(olen_8 as isize);
                                ofre = (ofre as libc::c_ulong).wrapping_add(delta_0)
                                    as size_t as size_t;
                                osiz = (osiz as libc::c_ulong).wrapping_add(delta_0)
                                    as size_t as size_t;
                            }
                        }
                    } else {
                        sprintf(cp, b"*%c\0" as *const u8 as *const libc::c_char, cs1);
                        loop {
                            nc = snprintf(
                                obufout,
                                ofre,
                                cpbufs[0 as libc::c_int as usize].buf,
                                fw as libc::c_int,
                                tmpval,
                            );
                            if !(nc >= ofre as libc::c_int) {
                                break;
                            }
                            if nc as libc::c_ulong >= ofre {
                                let mut olen_9: size_t = obufout.offset_from(obuf)
                                    as libc::c_long as size_t;
                                let mut delta_1: size_t = osiz
                                    .wrapping_add(nc as libc::c_ulong)
                                    .wrapping_sub(ofre);
                                obuf = erealloc_real(
                                    obuf as *mut libc::c_void,
                                    osiz.wrapping_add(delta_1),
                                    b"format_tree\0" as *const u8 as *const libc::c_char,
                                    b"obuf\0" as *const u8 as *const libc::c_char,
                                    b"builtin.c\0" as *const u8 as *const libc::c_char,
                                    1664 as libc::c_int,
                                ) as *mut libc::c_char;
                                obufout = obuf.offset(olen_9 as isize);
                                ofre = (ofre as libc::c_ulong).wrapping_add(delta_1)
                                    as size_t as size_t;
                                osiz = (osiz as libc::c_ulong).wrapping_add(delta_1)
                                    as size_t as size_t;
                            }
                        }
                    }
                    if quote_flag as libc::c_int != 0 && use_lc_numeric == 0 {
                        setlocale(
                            1 as libc::c_int,
                            b"C\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    len = strlen(obufout);
                    if quote_flag as libc::c_int != 0
                        && need_to_add_thousands as libc::c_int != 0
                    {
                        let mut new_text: *const libc::c_char = add_thousands(
                            obufout,
                            &mut loc,
                        );
                        len = strlen(new_text);
                        if len >= ofre {
                            let mut olen_10: size_t = obufout.offset_from(obuf)
                                as libc::c_long as size_t;
                            let mut delta_2: size_t = osiz
                                .wrapping_add(len)
                                .wrapping_sub(ofre);
                            obuf = erealloc_real(
                                obuf as *mut libc::c_void,
                                osiz.wrapping_add(delta_2),
                                b"format_tree\0" as *const u8 as *const libc::c_char,
                                b"obuf\0" as *const u8 as *const libc::c_char,
                                b"builtin.c\0" as *const u8 as *const libc::c_char,
                                1677 as libc::c_int,
                            ) as *mut libc::c_char;
                            obufout = obuf.offset(olen_10 as isize);
                            ofre = (ofre as libc::c_ulong).wrapping_add(delta_2)
                                as size_t as size_t;
                            osiz = (osiz as libc::c_ulong).wrapping_add(delta_2)
                                as size_t as size_t;
                        }
                        strcpy(obufout, new_text);
                        free(new_text as *mut libc::c_void);
                    }
                    ofre = (ofre as libc::c_ulong).wrapping_sub(len) as size_t as size_t;
                    obufout = obufout.offset(len as isize);
                    s0 = s1;
                }
                _ => {}
            }
            if !toofew {
                continue;
            }
            msg(
                b"%s\n\t`%s'\n\t%*s%s\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"fatal: not enough arguments to satisfy format string\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                fmt_string,
                (s1.offset_from(fmt_string) as libc::c_long
                    - 1 as libc::c_int as libc::c_long) as libc::c_int,
                b"\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"^ ran out for this one\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            current_block = 1242492412443577758;
            break;
        }
    }
    match current_block {
        2782926371273512654 => {
            if do_flags as libc::c_uint
                & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                    as libc::c_uint != 0
            {
                if need_format {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"builtin.c\0" as *const u8 as *const libc::c_char,
                        1701 as libc::c_int,
                    );
                    (Some(lintfunc.unwrap()))
                        .unwrap()(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"[s]printf: format specifier does not have control letter\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                if cur_arg < num_args as libc::c_ulong {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"builtin.c\0" as *const u8 as *const libc::c_char,
                        1704 as libc::c_int,
                    );
                    (Some(lintfunc.unwrap()))
                        .unwrap()(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"too many arguments supplied for format string\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
            }
            if s1.offset_from(s0) as libc::c_long != 0 {
                while s1.offset_from(s0) as libc::c_long as libc::c_ulong > ofre {
                    let mut olen_11: size_t = obufout.offset_from(obuf) as libc::c_long
                        as size_t;
                    obuf = erealloc_real(
                        obuf as *mut libc::c_void,
                        osiz.wrapping_mul(2 as libc::c_int as libc::c_ulong),
                        b"format_tree\0" as *const u8 as *const libc::c_char,
                        b"obuf\0" as *const u8 as *const libc::c_char,
                        b"builtin.c\0" as *const u8 as *const libc::c_char,
                        1707 as libc::c_int,
                    ) as *mut libc::c_char;
                    ofre = (ofre as libc::c_ulong).wrapping_add(osiz) as size_t
                        as size_t;
                    osiz = (osiz as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                    obufout = obuf.offset(olen_11 as isize);
                }
                memcpy(
                    obufout as *mut libc::c_void,
                    s0 as *const libc::c_void,
                    s1.offset_from(s0) as libc::c_long as size_t,
                );
                obufout = obufout.offset(s1.offset_from(s0) as libc::c_long as isize);
                ofre = (ofre as libc::c_ulong)
                    .wrapping_sub(s1.offset_from(s0) as libc::c_long as libc::c_ulong)
                    as size_t as size_t;
            }
            olen_final = obufout.offset_from(obuf) as libc::c_long as size_t;
            if ofre > (64 as libc::c_int * 2 as libc::c_int) as libc::c_ulong {
                obuf = erealloc_real(
                    obuf as *mut libc::c_void,
                    olen_final.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    b"format_tree\0" as *const u8 as *const libc::c_char,
                    b"obuf\0" as *const u8 as *const libc::c_char,
                    b"builtin.c\0" as *const u8 as *const libc::c_char,
                    1711 as libc::c_int,
                ) as *mut libc::c_char;
            }
            r = make_str_node(obuf, olen_final, 2 as libc::c_int);
            obuf = 0 as *mut libc::c_char;
        }
        _ => {}
    }
    let mut k_2: size_t = 0;
    let mut count_1: size_t = (::std::mem::size_of::<[C2RustUnnamed_12; 2]>()
        as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<C2RustUnnamed_12>() as libc::c_ulong);
    k_2 = 0 as libc::c_int as size_t;
    while k_2 < count_1 {
        if cpbufs[k_2 as usize].buf != (cpbufs[k_2 as usize].stackbuf).as_mut_ptr() {
            free(cpbufs[k_2 as usize].buf as *mut libc::c_void);
        }
        k_2 = k_2.wrapping_add(1);
        k_2;
    }
    if !obuf.is_null() {
        free(obuf as *mut libc::c_void);
    }
    if r.is_null() {
        gawk_exit(2 as libc::c_int);
    }
    return r;
}
unsafe extern "C" fn printf_common(mut nargs: libc::c_int) -> *mut NODE {
    let mut i: libc::c_int = 0;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    i = 1 as libc::c_int;
    while i <= nargs {
        let fresh21 = stack_ptr;
        stack_ptr = stack_ptr.offset(-1);
        let ref mut fresh22 = *args_array.offset((nargs - i) as isize);
        *fresh22 = (*fresh21).rptr;
        tmp = *fresh22;
        if (*tmp).type_0 as libc::c_uint == Node_var_array as libc::c_int as libc::c_uint
        {
            loop {
                i -= 1;
                if !(i > 0 as libc::c_int) {
                    break;
                }
                DEREF(*args_array.offset((nargs - i) as isize));
            }
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                1746 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"attempt to use array `%s' in a scalar context\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                array_vname(tmp),
            );
        }
        i += 1;
        i;
    }
    let ref mut fresh23 = *args_array.offset(0 as libc::c_int as isize);
    *fresh23 = force_string_fmt(
        *args_array.offset(0 as libc::c_int as isize),
        CONVFMT,
        CONVFMTidx,
    );
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
        && (*fixtype(*args_array.offset(0 as libc::c_int as isize))).flags
            as libc::c_uint & STRING as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            1752 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: received non-string format string argument\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"printf/sprintf\0" as *const u8 as *const libc::c_char,
        );
    }
    r = format_tree(
        (**args_array.offset(0 as libc::c_int as isize)).sub.val.sp,
        (**args_array.offset(0 as libc::c_int as isize)).sub.val.slen,
        args_array,
        nargs as libc::c_long,
    );
    i = 0 as libc::c_int;
    while i < nargs {
        DEREF(*args_array.offset(i as isize));
        i += 1;
        i;
    }
    return r;
}
pub unsafe extern "C" fn do_sprintf(mut nargs: libc::c_int) -> *mut NODE {
    let mut r: *mut NODE = 0 as *mut NODE;
    if nargs == 0 as libc::c_int {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            1767 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"sprintf: no arguments\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    r = printf_common(nargs);
    if r.is_null() {
        gawk_exit(2 as libc::c_int);
    }
    return r;
}
pub unsafe extern "C" fn do_printf(mut nargs: libc::c_int, mut redirtype: libc::c_int) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut rp: *mut redirect = 0 as *mut redirect;
    let mut errflg: libc::c_int = 0 as libc::c_int;
    let mut redir_exp: *mut NODE = 0 as *mut NODE;
    if nargs == 0 as libc::c_int {
        if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0
        {
            if do_flags as libc::c_uint
                & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                    as libc::c_uint != 0
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"builtin.c\0" as *const u8 as *const libc::c_char,
                    1790 as libc::c_int,
                );
                (Some(lintfunc.unwrap()))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"printf: no arguments\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            if redirtype != 0 as libc::c_int {
                redir_exp = (*stack_ptr).rptr;
                if (*redir_exp).type_0 as libc::c_uint
                    != Node_val as libc::c_int as libc::c_uint
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"builtin.c\0" as *const u8 as *const libc::c_char,
                        1794 as libc::c_int,
                    );
                    (Some(
                        (Some(
                            r_fatal
                                as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                        ))
                            .unwrap(),
                    ))
                        .unwrap()(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"attempt to use array `%s' in a scalar context\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        array_vname(redir_exp),
                    );
                }
                rp = redirect(redir_exp, redirtype, &mut errflg, 1 as libc::c_int != 0);
                DEREF(redir_exp);
                stack_ptr = stack_ptr.offset(-1);
                stack_ptr;
            }
            return;
        }
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            1801 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"printf: no arguments\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if redirtype != 0 as libc::c_int {
        redir_exp = (*stack_ptr.offset(-(nargs as isize))).rptr;
        if (*redir_exp).type_0 as libc::c_uint != Node_val as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                1807 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"attempt to use array `%s' in a scalar context\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                array_vname(redir_exp),
            );
        }
        rp = redirect(redir_exp, redirtype, &mut errflg, 1 as libc::c_int != 0);
        if !rp.is_null() {
            if (*rp).flag as libc::c_uint & RED_TWOWAY as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint && ((*rp).output.fp).is_null()
            {
                if is_non_fatal_redirect(
                    (*redir_exp).sub.val.sp,
                    (*redir_exp).sub.val.slen,
                ) {
                    update_ERRNO_int(9 as libc::c_int);
                    return;
                }
                close_rp(rp, CLOSE_ALL);
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"builtin.c\0" as *const u8 as *const libc::c_char,
                    1816 as libc::c_int,
                );
                (Some(
                    (Some(
                        r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                    ))
                        .unwrap(),
                ))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"printf: attempt to write to closed write end of two-way pipe\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            fp = (*rp).output.fp;
        } else if errflg != 0 {
            update_ERRNO_int(errflg);
            return;
        }
    } else if do_flags as libc::c_uint & DO_DEBUG as libc::c_int as libc::c_uint != 0 {
        fp = output_fp;
    } else {
        fp = stdout;
    }
    tmp = printf_common(nargs);
    if !redir_exp.is_null() {
        DEREF(redir_exp);
        stack_ptr = stack_ptr.offset(-1);
        stack_ptr;
    }
    if !tmp.is_null() {
        if fp.is_null() {
            DEREF(tmp);
            return;
        }
        efwrite(
            (*tmp).sub.val.sp as *const libc::c_void,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
            (*tmp).sub.val.slen,
            fp,
            b"printf\0" as *const u8 as *const libc::c_char,
            rp,
            1 as libc::c_int != 0,
        );
        if !rp.is_null()
            && (*rp).flag as libc::c_uint & RED_TWOWAY as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
        {
            ((*rp).output.gawk_fflush).unwrap()((*rp).output.fp, (*rp).output.opaque);
        }
        DEREF(tmp);
    } else {
        gawk_exit(2 as libc::c_int);
    };
}
pub unsafe extern "C" fn do_sqrt(mut nargs: libc::c_int) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut arg: libc::c_double = 0.;
    check_exact_args(
        nargs,
        b"sqrt\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    tmp = POP_SCALAR();
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
        && (*fixtype(tmp)).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            1859 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: received non-numeric argument\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"sqrt\0" as *const u8 as *const libc::c_char,
        );
    }
    arg = (*force_number(tmp)).sub.val.fltnum;
    DEREF(tmp);
    if arg < 0.0f64 {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            1863 as libc::c_int,
        );
        (Some(
            (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: received negative argument %g\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"sqrt\0" as *const u8 as *const libc::c_char,
            arg,
        );
    }
    return make_number.unwrap()(sqrt(arg));
}
pub unsafe extern "C" fn do_substr(mut nargs: libc::c_int) -> *mut NODE {
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut indx: size_t = 0;
    let mut length: size_t = 0 as libc::c_int as size_t;
    let mut d_index: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut d_length: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut src_len: size_t = 0;
    check_args_min_max(
        nargs,
        b"substr\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int,
        3 as libc::c_int,
    );
    if nargs == 3 as libc::c_int {
        t1 = force_number(POP_SCALAR());
        if do_flags as libc::c_uint
            & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                as libc::c_uint != 0
            && (*fixtype(t1)).flags as libc::c_uint
                & NUMBER as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                1884 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: received non-numeric third argument\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"substr\0" as *const u8 as *const libc::c_char,
            );
        }
        d_length = (*t1).sub.val.fltnum;
        DEREF(t1);
    }
    t1 = force_number(POP_SCALAR());
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
        && (*fixtype(t1)).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            1891 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: received non-numeric second argument\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"substr\0" as *const u8 as *const libc::c_char,
        );
    }
    d_index = (*t1).sub.val.fltnum;
    DEREF(t1);
    t1 = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
        && (*fixtype(t1)).flags as libc::c_uint
            & (STRING as libc::c_int | USER_INPUT as libc::c_int) as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            1897 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: received non-string first argument\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"substr\0" as *const u8 as *const libc::c_char,
        );
    }
    if nargs == 3 as libc::c_int {
        if !(d_length >= 1 as libc::c_int as libc::c_double) {
            if do_flags as libc::c_uint
                & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                    as libc::c_uint == DO_LINT_ALL as libc::c_int as libc::c_uint
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"builtin.c\0" as *const u8 as *const libc::c_char,
                    1902 as libc::c_int,
                );
                (Some(lintfunc.unwrap()))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"substr: length %g is not >= 1\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    d_length,
                );
            } else if do_flags as libc::c_uint
                & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                    as libc::c_uint == DO_LINT_INVALID as libc::c_int as libc::c_uint
                && !(d_length >= 0 as libc::c_int as libc::c_double)
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"builtin.c\0" as *const u8 as *const libc::c_char,
                    1904 as libc::c_int,
                );
                (Some(lintfunc.unwrap()))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"substr: length %g is not >= 0\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    d_length,
                );
            }
            DEREF(t1);
            return make_str_node(
                b"\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int as size_t,
                0 as libc::c_int,
            );
        }
        if do_flags as libc::c_uint
            & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                as libc::c_uint != 0
        {
            if double_to_int(d_length) != d_length {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"builtin.c\0" as *const u8 as *const libc::c_char,
                    1917 as libc::c_int,
                );
                (Some(lintfunc.unwrap()))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"substr: non-integer length %g will be truncated\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    d_length,
                );
            }
            if d_length > 18446744073709551615 as libc::c_ulong as libc::c_double {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"builtin.c\0" as *const u8 as *const libc::c_char,
                    1922 as libc::c_int,
                );
                (Some(lintfunc.unwrap()))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"substr: length %g too big for string indexing, truncating to %g\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    d_length,
                    18446744073709551615 as libc::c_ulong as libc::c_double,
                );
            }
        }
        if d_length < 18446744073709551615 as libc::c_ulong as libc::c_double {
            length = d_length as size_t;
        } else {
            length = 18446744073709551615 as libc::c_ulong;
        }
    }
    if !(d_index >= 1 as libc::c_int as libc::c_double) {
        if do_flags as libc::c_uint
            & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                as libc::c_uint != 0
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                1935 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"substr: start index %g is invalid, using 1\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                d_index,
            );
        }
        d_index = 1 as libc::c_int as libc::c_double;
    }
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0 && double_to_int(d_index) != d_index
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            1940 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"substr: non-integer start index %g will be truncated\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            d_index,
        );
    }
    if d_index <= 18446744073709551615 as libc::c_ulong as libc::c_double {
        indx = (d_index - 1 as libc::c_int as libc::c_double) as size_t;
    } else {
        indx = 18446744073709551615 as libc::c_ulong;
    }
    if nargs == 2 as libc::c_int {
        length = ((*t1).sub.val.slen).wrapping_sub(indx);
        if gawk_mb_cur_max > 1 as libc::c_int {
            t1 = str2wstr(t1, 0 as *mut *mut size_t);
            if (*t1).sub.val.wslen > 0 as libc::c_int as libc::c_ulong {
                length = ((*t1).sub.val.wslen).wrapping_sub(indx);
            }
        }
        d_length = length as libc::c_double;
    }
    if (*t1).sub.val.slen == 0 as libc::c_int as libc::c_ulong {
        if do_flags as libc::c_uint
            & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                as libc::c_uint != 0
            && (do_flags as libc::c_uint
                & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                    as libc::c_uint == DO_LINT_ALL as libc::c_int as libc::c_uint
                || indx | length != 0 as libc::c_int as libc::c_ulong)
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                1963 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"substr: source string is zero length\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        DEREF(t1);
        return make_str_node(
            b"\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
            0 as libc::c_int,
        );
    }
    if gawk_mb_cur_max > 1 as libc::c_int {
        t1 = str2wstr(t1, 0 as *mut *mut size_t);
        src_len = (*t1).sub.val.wslen;
    } else {
        src_len = (*t1).sub.val.slen;
    }
    if indx >= src_len {
        if do_flags as libc::c_uint
            & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                as libc::c_uint != 0
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                1977 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"substr: start index %g is past end of string\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                d_index,
            );
        }
        DEREF(t1);
        return make_str_node(
            b"\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
            0 as libc::c_int,
        );
    }
    if length > src_len.wrapping_sub(indx) {
        if do_flags as libc::c_uint
            & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                as libc::c_uint != 0
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                1984 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"substr: length %g at start index %g exceeds length of first argument (%lu)\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                d_length,
                d_index,
                src_len,
            );
        }
        length = src_len.wrapping_sub(indx);
    }
    if gawk_mb_cur_max == 1 as libc::c_int || (*t1).sub.val.wslen == (*t1).sub.val.slen {
        r = make_str_node(
            ((*t1).sub.val.sp).offset(indx as isize),
            length,
            0 as libc::c_int,
        );
    } else {
        let mut result: size_t = 0;
        let mut wp: *mut wchar_t = 0 as *mut wchar_t;
        let mut mbs: mbstate_t = mbstate_t {
            __count: 0,
            __value: C2RustUnnamed { __wch: 0 },
        };
        let mut substr: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
        memset(
            &mut mbs as *mut mbstate_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
        );
        substr = emalloc_real(
            length
                .wrapping_mul(gawk_mb_cur_max as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
            b"do_substr\0" as *const u8 as *const libc::c_char,
            b"substr\0" as *const u8 as *const libc::c_char,
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            2007 as libc::c_int,
        ) as *mut libc::c_char;
        wp = ((*t1).sub.val.wsp).offset(indx as isize);
        cp = substr;
        while length > 0 as libc::c_int as libc::c_ulong {
            result = wcrtomb(cp, *wp, &mut mbs);
            if result == -(1 as libc::c_int) as size_t {
                break;
            }
            cp = cp.offset(result as isize);
            wp = wp.offset(1);
            wp;
            length = length.wrapping_sub(1);
            length;
        }
        *cp = '\0' as i32 as libc::c_char;
        r = make_str_node(
            substr,
            cp.offset_from(substr) as libc::c_long as size_t,
            2 as libc::c_int,
        );
    }
    DEREF(t1);
    return r;
}
static mut time_t_min: time_t = 0;
static mut time_t_max: time_t = 0;
pub unsafe extern "C" fn do_strftime(mut nargs: libc::c_int) -> *mut NODE {
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut t2: *mut NODE = 0 as *mut NODE;
    let mut t3: *mut NODE = 0 as *mut NODE;
    let mut ret: *mut NODE = 0 as *mut NODE;
    let mut tm: *mut tm = 0 as *mut tm;
    let mut fclock: time_t = 0;
    let mut clock_val: libc::c_double = 0.;
    let mut bufp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buflen: size_t = 0;
    let mut bufsize: size_t = 0;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut format: *const libc::c_char = 0 as *const libc::c_char;
    let mut formatlen: libc::c_int = 0;
    let mut do_gmt: bool = false;
    let mut val: *mut NODE = 0 as *mut NODE;
    let mut sub: *mut NODE = 0 as *mut NODE;
    let mut save: libc::c_char = '\0' as i32 as libc::c_char;
    format = def_strftime_format.as_ptr();
    formatlen = strlen(format) as libc::c_int;
    time(&mut fclock);
    do_gmt = 0 as libc::c_int != 0;
    check_args_min_max(
        nargs,
        b"strftime\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        3 as libc::c_int,
    );
    if !PROCINFO_node.is_null() {
        sub = make_str_node(
            b"strftime\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as size_t,
            0 as libc::c_int,
        );
        val = in_array(PROCINFO_node, sub);
        unref(sub);
        if !val.is_null() {
            if do_flags as libc::c_uint
                & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                    as libc::c_uint != 0
                && (*fixtype(val)).flags as libc::c_uint
                    & (STRING as libc::c_int | USER_INPUT as libc::c_int) as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"builtin.c\0" as *const u8 as *const libc::c_char,
                    2060 as libc::c_int,
                );
                (Some(lintfunc.unwrap()))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"strftime: format value in PROCINFO[\"strftime\"] has numeric type\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            val = force_string_fmt(val, CONVFMT, CONVFMTidx);
            format = (*val).sub.val.sp;
            formatlen = (*val).sub.val.slen as libc::c_int;
        }
    }
    t3 = 0 as *mut NODE;
    t2 = t3;
    t1 = t2;
    if nargs > 0 as libc::c_int {
        let mut tmp: *mut NODE = 0 as *mut NODE;
        if nargs == 3 as libc::c_int {
            t3 = POP_SCALAR();
            do_gmt = boolval(t3);
            DEREF(t3);
        }
        if nargs >= 2 as libc::c_int {
            t2 = POP_SCALAR();
            if do_flags as libc::c_uint
                & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                    as libc::c_uint != 0
                && (*fixtype(t2)).flags as libc::c_uint
                    & NUMBER as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"builtin.c\0" as *const u8 as *const libc::c_char,
                    2081 as libc::c_int,
                );
                (Some(lintfunc.unwrap()))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: received non-numeric second argument\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    b"strftime\0" as *const u8 as *const libc::c_char,
                );
            }
            force_number(t2);
            clock_val = (*t2).sub.val.fltnum;
            fclock = clock_val as time_t;
            if clock_val < 0 as libc::c_int as libc::c_double
                && fclock > 0 as libc::c_int as libc::c_long
            {
                if do_flags as libc::c_uint
                    & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                        as libc::c_uint != 0
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"builtin.c\0" as *const u8 as *const libc::c_char,
                        2091 as libc::c_int,
                    );
                    (Some(lintfunc.unwrap()))
                        .unwrap()(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"strftime: second argument less than 0 or too big for time_t\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                return make_str_node(
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int as size_t,
                    0 as libc::c_int,
                );
            }
            if clock_val < time_t_min as libc::c_double
                || clock_val > time_t_max as libc::c_double
            {
                if do_flags as libc::c_uint
                    & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                        as libc::c_uint != 0
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"builtin.c\0" as *const u8 as *const libc::c_char,
                        2098 as libc::c_int,
                    );
                    (Some(lintfunc.unwrap()))
                        .unwrap()(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"strftime: second argument out of range for time_t\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                return make_str_node(
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int as size_t,
                    0 as libc::c_int,
                );
            }
            DEREF(t2);
        }
        tmp = POP_SCALAR();
        if do_flags as libc::c_uint
            & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                as libc::c_uint != 0
            && (*fixtype(tmp)).flags as libc::c_uint
                & STRING as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                2107 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: received non-string first argument\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"strftime\0" as *const u8 as *const libc::c_char,
            );
        }
        t1 = force_string_fmt(tmp, CONVFMT, CONVFMTidx);
        format = (*t1).sub.val.sp;
        formatlen = (*t1).sub.val.slen as libc::c_int;
        if formatlen == 0 as libc::c_int {
            if do_flags as libc::c_uint
                & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                    as libc::c_uint != 0
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"builtin.c\0" as *const u8 as *const libc::c_char,
                    2114 as libc::c_int,
                );
                (Some(lintfunc.unwrap()))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"strftime: received empty format string\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            DEREF(t1);
            return make_str_node(
                b"\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int as size_t,
                0 as libc::c_int,
            );
        }
        str_terminate_f(t1, &mut save);
    }
    if do_gmt {
        tm = gmtime(&mut fclock);
    } else {
        tm = localtime(&mut fclock);
    }
    if tm.is_null() {
        ret = make_str_node(
            b"\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
            0 as libc::c_int,
        );
    } else {
        bufp = buf.as_mut_ptr();
        bufsize = ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong;
        loop {
            *bufp = '\0' as i32 as libc::c_char;
            buflen = strftime(bufp, bufsize, format, tm);
            if buflen > 0 as libc::c_int as libc::c_ulong
                || bufsize >= (1024 as libc::c_int * formatlen) as libc::c_ulong
            {
                break;
            }
            bufsize = (bufsize as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            if bufp == buf.as_mut_ptr() {
                bufp = emalloc_real(
                    bufsize,
                    b"do_strftime\0" as *const u8 as *const libc::c_char,
                    b"bufp\0" as *const u8 as *const libc::c_char,
                    b"builtin.c\0" as *const u8 as *const libc::c_char,
                    2148 as libc::c_int,
                ) as *mut libc::c_char;
            } else {
                bufp = erealloc_real(
                    bufp as *mut libc::c_void,
                    bufsize,
                    b"do_strftime\0" as *const u8 as *const libc::c_char,
                    b"bufp\0" as *const u8 as *const libc::c_char,
                    b"builtin.c\0" as *const u8 as *const libc::c_char,
                    2150 as libc::c_int,
                ) as *mut libc::c_char;
            }
        }
        ret = make_str_node(bufp, buflen, 0 as libc::c_int);
        if bufp != buf.as_mut_ptr() {
            free(bufp as *mut libc::c_void);
        }
    }
    if !t1.is_null() {
        *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = save;
        DEREF(t1);
    }
    return ret;
}
pub unsafe extern "C" fn do_systime(mut nargs: libc::c_int) -> *mut NODE {
    let mut lclock: time_t = 0;
    check_exact_args(
        nargs,
        b"systime\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    time(&mut lclock);
    return make_number.unwrap()(lclock as libc::c_double);
}
pub unsafe extern "C" fn do_mktime(mut nargs: libc::c_int) -> *mut NODE {
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut t2: *mut NODE = 0 as *mut NODE;
    let mut then: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut year: libc::c_long = 0;
    let mut month: libc::c_int = 0;
    let mut day: libc::c_int = 0;
    let mut hour: libc::c_int = 0;
    let mut minute: libc::c_int = 0;
    let mut second: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut dst: libc::c_int = -(1 as libc::c_int);
    let mut then_stamp: time_t = 0;
    let mut save: libc::c_char = 0;
    let mut do_gmt: bool = false;
    check_args_min_max(
        nargs,
        b"mktime\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        2 as libc::c_int,
    );
    if nargs == 2 as libc::c_int {
        t2 = POP_SCALAR();
        do_gmt = boolval(t2);
        DEREF(t2);
    } else {
        do_gmt = 0 as libc::c_int != 0;
    }
    t1 = POP_SCALAR();
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
        && (*fixtype(t1)).flags as libc::c_uint & STRING as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            2201 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: received non-string argument\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"mktime\0" as *const u8 as *const libc::c_char,
        );
    }
    t1 = force_string_fmt(t1, CONVFMT, CONVFMTidx);
    save = *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize);
    *((*t1).sub.val.sp)
        .offset((*t1).sub.val.slen as isize) = '\0' as i32 as libc::c_char;
    count = sscanf(
        (*t1).sub.val.sp,
        b"%ld %d %d %d %d %d %d\0" as *const u8 as *const libc::c_char,
        &mut year as *mut libc::c_long,
        &mut month as *mut libc::c_int,
        &mut day as *mut libc::c_int,
        &mut hour as *mut libc::c_int,
        &mut minute as *mut libc::c_int,
        &mut second as *mut libc::c_int,
        &mut dst as *mut libc::c_int,
    );
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
        && (second < 0 as libc::c_int || second > 60 as libc::c_int
            || (minute < 0 as libc::c_int || minute > 59 as libc::c_int)
            || (hour < 0 as libc::c_int || hour > 23 as libc::c_int)
            || (day < 1 as libc::c_int || day > 31 as libc::c_int)
            || (month < 1 as libc::c_int || month > 12 as libc::c_int))
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            2220 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"mktime: at least one of the values is out of the default range\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = save;
    DEREF(t1);
    if count < 6 as libc::c_int
        || month == -(2147483647 as libc::c_int) - 1 as libc::c_int
        || year
            < (-(2147483647 as libc::c_int) - 1 as libc::c_int + 1900 as libc::c_int)
                as libc::c_long
        || year - 1900 as libc::c_int as libc::c_long
            > 2147483647 as libc::c_int as libc::c_long
    {
        return make_number.unwrap()(-(1 as libc::c_int) as libc::c_double);
    }
    memset(
        &mut then as *mut tm as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<tm>() as libc::c_ulong,
    );
    then.tm_sec = second;
    then.tm_min = minute;
    then.tm_hour = hour;
    then.tm_mday = day;
    then.tm_mon = month - 1 as libc::c_int;
    then.tm_year = (year - 1900 as libc::c_int as libc::c_long) as libc::c_int;
    then.tm_isdst = dst;
    then_stamp = if do_gmt as libc::c_int != 0 {
        timegm(&mut then)
    } else {
        mktime(&mut then)
    };
    return make_number.unwrap()(then_stamp as libc::c_double);
}
pub unsafe extern "C" fn do_system(mut nargs: libc::c_int) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut ret: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut save: libc::c_char = 0;
    let mut status: libc::c_int = 0;
    check_exact_args(
        nargs,
        b"system\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    if do_flags as libc::c_uint & DO_SANDBOX as libc::c_int as libc::c_uint != 0 {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            2258 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"'system' function not allowed in sandbox mode\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    flush_io();
    tmp = POP_SCALAR();
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
        && (*fixtype(tmp)).flags as libc::c_uint & STRING as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            2263 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: received non-string argument\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"system\0" as *const u8 as *const libc::c_char,
        );
    }
    cmd = (*force_string_fmt(tmp, CONVFMT, CONVFMTidx)).sub.val.sp;
    if !cmd.is_null() && *cmd as libc::c_int != 0 {
        save = *cmd.offset((*tmp).sub.val.slen as isize);
        *cmd.offset((*tmp).sub.val.slen as isize) = '\0' as i32 as libc::c_char;
        os_restore_mode(fileno(stdin));
        signal(13 as libc::c_int, None);
        status = system(cmd);
        ret = status as libc::c_double;
        if status != -(1 as libc::c_int) {
            if !(do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint != 0)
            {
                if do_flags as libc::c_uint
                    & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0
                {
                    ret = status as libc::c_double / 256.0f64;
                } else {
                    ret = sanitize_exit_status(status) as libc::c_double;
                }
            }
        }
        if BINMODE & BINMODE_INPUT as libc::c_int != 0 as libc::c_int {
            os_setbinmode(fileno(stdin), 0 as libc::c_int);
        }
        signal(
            13 as libc::c_int,
            ::std::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        );
        *cmd.offset((*tmp).sub.val.slen as isize) = save;
    }
    DEREF(tmp);
    return make_number.unwrap()(ret);
}
pub unsafe extern "C" fn do_print(mut nargs: libc::c_int, mut redirtype: libc::c_int) {
    let mut rp: *mut redirect = 0 as *mut redirect;
    let mut errflg: libc::c_int = 0 as libc::c_int;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut i: libc::c_int = 0;
    let mut redir_exp: *mut NODE = 0 as *mut NODE;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    if redirtype != 0 as libc::c_int {
        redir_exp = (*stack_ptr.offset(-(nargs as isize))).rptr;
        if (*redir_exp).type_0 as libc::c_uint != Node_val as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                2323 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"attempt to use array `%s' in a scalar context\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                array_vname(redir_exp),
            );
        }
        rp = redirect(redir_exp, redirtype, &mut errflg, 1 as libc::c_int != 0);
        if !rp.is_null() {
            if (*rp).flag as libc::c_uint & RED_TWOWAY as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint && ((*rp).output.fp).is_null()
            {
                if is_non_fatal_redirect(
                    (*redir_exp).sub.val.sp,
                    (*redir_exp).sub.val.slen,
                ) {
                    update_ERRNO_int(9 as libc::c_int);
                    return;
                }
                close_rp(rp, CLOSE_ALL);
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"builtin.c\0" as *const u8 as *const libc::c_char,
                    2332 as libc::c_int,
                );
                (Some(
                    (Some(
                        r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                    ))
                        .unwrap(),
                ))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"print: attempt to write to closed write end of two-way pipe\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            fp = (*rp).output.fp;
        } else if errflg != 0 {
            update_ERRNO_int(errflg);
            return;
        }
    } else if do_flags as libc::c_uint & DO_DEBUG as libc::c_int as libc::c_uint != 0 {
        fp = output_fp;
    } else {
        fp = stdout;
    }
    i = 1 as libc::c_int;
    while i <= nargs {
        let fresh24 = stack_ptr;
        stack_ptr = stack_ptr.offset(-1);
        let ref mut fresh25 = *args_array.offset(i as isize);
        *fresh25 = (*fresh24).rptr;
        tmp = *fresh25;
        if (*tmp).type_0 as libc::c_uint == Node_var_array as libc::c_int as libc::c_uint
        {
            loop {
                i -= 1;
                if !(i > 0 as libc::c_int) {
                    break;
                }
                DEREF(*args_array.offset(i as isize));
            }
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                2350 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"attempt to use array `%s' in a scalar context\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                array_vname(tmp),
            );
        }
        let ref mut fresh26 = *args_array.offset(i as isize);
        *fresh26 = force_string_fmt(tmp, OFMT, OFMTidx);
        if *args_array.offset(i as isize) != tmp {
            DEREF(tmp);
        }
        i += 1;
        i;
    }
    if !redir_exp.is_null() {
        DEREF(redir_exp);
        stack_ptr = stack_ptr.offset(-1);
        stack_ptr;
    }
    if fp.is_null() {
        i = nargs;
        while i > 0 as libc::c_int {
            DEREF(*args_array.offset(i as isize));
            i -= 1;
            i;
        }
        return;
    }
    i = nargs;
    while i > 0 as libc::c_int {
        efwrite(
            (**args_array.offset(i as isize)).sub.val.sp as *const libc::c_void,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
            (**args_array.offset(i as isize)).sub.val.slen,
            fp,
            b"print\0" as *const u8 as *const libc::c_char,
            rp,
            0 as libc::c_int != 0,
        );
        DEREF(*args_array.offset(i as isize));
        if i != 1 as libc::c_int && OFSlen > 0 as libc::c_int {
            efwrite(
                OFS as *const libc::c_void,
                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                OFSlen as size_t,
                fp,
                b"print\0" as *const u8 as *const libc::c_char,
                rp,
                0 as libc::c_int != 0,
            );
        }
        i -= 1;
        i;
    }
    if ORSlen > 0 as libc::c_int {
        efwrite(
            ORS as *const libc::c_void,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
            ORSlen as size_t,
            fp,
            b"print\0" as *const u8 as *const libc::c_char,
            rp,
            1 as libc::c_int != 0,
        );
    }
    if !rp.is_null()
        && (*rp).flag as libc::c_uint & RED_TWOWAY as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
    {
        ((*rp).output.gawk_fflush).unwrap()((*rp).output.fp, (*rp).output.opaque);
    }
}
pub unsafe extern "C" fn do_print_rec(
    mut nargs: libc::c_int,
    mut redirtype: libc::c_int,
) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut f0: *mut NODE = 0 as *mut NODE;
    let mut rp: *mut redirect = 0 as *mut redirect;
    let mut errflg: libc::c_int = 0 as libc::c_int;
    let mut redir_exp: *mut NODE = 0 as *mut NODE;
    if redirtype != 0 as libc::c_int {
        redir_exp = (*stack_ptr).rptr;
        rp = redirect(redir_exp, redirtype, &mut errflg, 1 as libc::c_int != 0);
        if !rp.is_null() {
            if (*rp).flag as libc::c_uint & RED_TWOWAY as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint && ((*rp).output.fp).is_null()
            {
                if is_non_fatal_redirect(
                    (*redir_exp).sub.val.sp,
                    (*redir_exp).sub.val.slen,
                ) {
                    update_ERRNO_int(9 as libc::c_int);
                    return;
                }
                close_rp(rp, CLOSE_ALL);
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"builtin.c\0" as *const u8 as *const libc::c_char,
                    2407 as libc::c_int,
                );
                (Some(
                    (Some(
                        r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                    ))
                        .unwrap(),
                ))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"print: attempt to write to closed write end of two-way pipe\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            fp = (*rp).output.fp;
        }
        DEREF(redir_exp);
        stack_ptr = stack_ptr.offset(-1);
        stack_ptr;
    } else {
        fp = output_fp;
    }
    if errflg != 0 {
        update_ERRNO_int(errflg);
        return;
    }
    if fp.is_null() {
        return;
    }
    if !field0_valid
        || do_flags as libc::c_uint
            & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                as libc::c_uint != 0
    {
        get_field(0 as libc::c_long, 0 as *mut Func_ptr);
    }
    f0 = *fields_arr.offset(0 as libc::c_int as isize);
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
        && (*f0).flags as libc::c_uint & NULL_FIELD as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            2430 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"reference to uninitialized field `$%d'\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            0 as libc::c_int,
        );
    }
    efwrite(
        (*f0).sub.val.sp as *const libc::c_void,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        (*f0).sub.val.slen,
        fp,
        b"print\0" as *const u8 as *const libc::c_char,
        rp,
        0 as libc::c_int != 0,
    );
    if ORSlen > 0 as libc::c_int {
        efwrite(
            ORS as *const libc::c_void,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
            ORSlen as size_t,
            fp,
            b"print\0" as *const u8 as *const libc::c_char,
            rp,
            1 as libc::c_int != 0,
        );
    }
    if !rp.is_null()
        && (*rp).flag as libc::c_uint & RED_TWOWAY as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
    {
        ((*rp).output.gawk_fflush).unwrap()((*rp).output.fp, (*rp).output.opaque);
    }
}
unsafe extern "C" fn is_wupper(mut c: wchar_t) -> libc::c_int {
    return iswupper(c as wint_t);
}
unsafe extern "C" fn is_wlower(mut c: wchar_t) -> libc::c_int {
    return iswlower(c as wint_t);
}
unsafe extern "C" fn to_wlower(mut c: wchar_t) -> libc::c_int {
    return towlower(c as wint_t) as libc::c_int;
}
unsafe extern "C" fn to_wupper(mut c: wchar_t) -> libc::c_int {
    return towupper(c as wint_t) as libc::c_int;
}
unsafe extern "C" fn wide_change_case(
    mut wstr: *mut wchar_t,
    mut wlen: size_t,
    mut is_x: Option::<unsafe extern "C" fn(wchar_t) -> libc::c_int>,
    mut to_y: Option::<unsafe extern "C" fn(wchar_t) -> libc::c_int>,
) {
    let mut i: size_t = 0;
    let mut wcp: *mut wchar_t = 0 as *mut wchar_t;
    i = 0 as libc::c_int as size_t;
    wcp = wstr;
    while i < wlen {
        if is_x.unwrap()(*wcp) != 0 {
            *wcp = to_y.unwrap()(*wcp);
        }
        i = i.wrapping_add(1);
        i;
        wcp = wcp.offset(1);
        wcp;
    }
}
unsafe extern "C" fn wide_toupper(mut wstr: *mut wchar_t, mut wlen: size_t) {
    wide_change_case(
        wstr,
        wlen,
        Some(is_wlower as unsafe extern "C" fn(wchar_t) -> libc::c_int),
        Some(to_wupper as unsafe extern "C" fn(wchar_t) -> libc::c_int),
    );
}
unsafe extern "C" fn wide_tolower(mut wstr: *mut wchar_t, mut wlen: size_t) {
    wide_change_case(
        wstr,
        wlen,
        Some(is_wupper as unsafe extern "C" fn(wchar_t) -> libc::c_int),
        Some(to_wlower as unsafe extern "C" fn(wchar_t) -> libc::c_int),
    );
}
pub unsafe extern "C" fn do_tolower(mut nargs: libc::c_int) -> *mut NODE {
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut t2: *mut NODE = 0 as *mut NODE;
    check_exact_args(
        nargs,
        b"tolower\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    t1 = POP_SCALAR();
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
        && (*fixtype(t1)).flags as libc::c_uint & STRING as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            2517 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: received non-string argument\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"tolower\0" as *const u8 as *const libc::c_char,
        );
    }
    t1 = force_string_fmt(t1, CONVFMT, CONVFMTidx);
    t2 = make_str_node((*t1).sub.val.sp, (*t1).sub.val.slen, 0 as libc::c_int);
    if gawk_mb_cur_max == 1 as libc::c_int {
        let mut cp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut cp2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        cp = (*t2).sub.val.sp as *mut libc::c_uchar;
        cp2 = ((*t2).sub.val.sp).offset((*t2).sub.val.slen as isize)
            as *mut libc::c_uchar;
        while cp < cp2 {
            if *(*__ctype_b_loc()).offset(*cp as libc::c_int as isize) as libc::c_int
                & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                *cp = ({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = *cp as libc::c_int;
                            __res = if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = tolower(*cp as libc::c_int);
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc())
                            .offset(*cp as libc::c_int as isize);
                    }
                    __res
                }) as libc::c_uchar;
            }
            cp = cp.offset(1);
            cp;
        }
    } else {
        str2wstr(t2, 0 as *mut *mut size_t);
        wide_tolower((*t2).sub.val.wsp, (*t2).sub.val.wslen);
        wstr2str(t2);
    }
    DEREF(t1);
    return t2;
}
pub unsafe extern "C" fn do_toupper(mut nargs: libc::c_int) -> *mut NODE {
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut t2: *mut NODE = 0 as *mut NODE;
    check_exact_args(
        nargs,
        b"toupper\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    t1 = POP_SCALAR();
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
        && (*fixtype(t1)).flags as libc::c_uint & STRING as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            2550 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: received non-string argument\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"toupper\0" as *const u8 as *const libc::c_char,
        );
    }
    t1 = force_string_fmt(t1, CONVFMT, CONVFMTidx);
    t2 = make_str_node((*t1).sub.val.sp, (*t1).sub.val.slen, 0 as libc::c_int);
    if gawk_mb_cur_max == 1 as libc::c_int {
        let mut cp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut cp2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        cp = (*t2).sub.val.sp as *mut libc::c_uchar;
        cp2 = ((*t2).sub.val.sp).offset((*t2).sub.val.slen as isize)
            as *mut libc::c_uchar;
        while cp < cp2 {
            if *(*__ctype_b_loc()).offset(*cp as libc::c_int as isize) as libc::c_int
                & _ISlower as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                *cp = ({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = *cp as libc::c_int;
                            __res = if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = toupper(*cp as libc::c_int);
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc())
                            .offset(*cp as libc::c_int as isize);
                    }
                    __res
                }) as libc::c_uchar;
            }
            cp = cp.offset(1);
            cp;
        }
    } else {
        str2wstr(t2, 0 as *mut *mut size_t);
        wide_toupper((*t2).sub.val.wsp, (*t2).sub.val.wslen);
        wstr2str(t2);
    }
    DEREF(t1);
    return t2;
}
pub unsafe extern "C" fn do_atan2(mut nargs: libc::c_int) -> *mut NODE {
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut t2: *mut NODE = 0 as *mut NODE;
    let mut d1: libc::c_double = 0.;
    let mut d2: libc::c_double = 0.;
    check_exact_args(
        nargs,
        b"atan2\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int,
    );
    t2 = POP_SCALAR();
    let fresh27 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    t1 = (*fresh27).rptr;
    if (*t1).type_0 as libc::c_uint == Node_var_array as libc::c_int as libc::c_uint {
        DEREF(t2);
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            2582 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"attempt to use array `%s' in a scalar context\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            array_vname(t1),
        );
    }
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
    {
        if (*fixtype(t1)).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                2585 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: received non-numeric first argument\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"atan2\0" as *const u8 as *const libc::c_char,
            );
        }
        if (*fixtype(t2)).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                2587 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: received non-numeric second argument\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"atan2\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    d1 = (*force_number(t1)).sub.val.fltnum;
    d2 = (*force_number(t2)).sub.val.fltnum;
    DEREF(t1);
    DEREF(t2);
    return make_number.unwrap()(atan2(d1, d2));
}
pub unsafe extern "C" fn do_sin(mut nargs: libc::c_int) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut d: libc::c_double = 0.;
    check_exact_args(
        nargs,
        b"sin\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    tmp = POP_SCALAR();
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
        && (*fixtype(tmp)).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            2608 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: received non-numeric argument\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"sin\0" as *const u8 as *const libc::c_char,
        );
    }
    d = sin((*force_number(tmp)).sub.val.fltnum);
    DEREF(tmp);
    return make_number.unwrap()(d);
}
pub unsafe extern "C" fn do_cos(mut nargs: libc::c_int) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut d: libc::c_double = 0.;
    check_exact_args(
        nargs,
        b"cos\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    tmp = POP_SCALAR();
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
        && (*fixtype(tmp)).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            2626 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: received non-numeric argument\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"cos\0" as *const u8 as *const libc::c_char,
        );
    }
    d = cos((*force_number(tmp)).sub.val.fltnum);
    DEREF(tmp);
    return make_number.unwrap()(d);
}
static mut firstrand: bool = 1 as libc::c_int != 0;
static mut istate: [gawk_uint32_t; 64] = [0; 64];
static mut state: *mut libc::c_char = unsafe {
    istate.as_ptr() as *mut libc::c_char
};
pub unsafe extern "C" fn do_rand(mut nargs: libc::c_int) -> *mut NODE {
    let mut tmprand: libc::c_double = 0.;
    check_exact_args(
        nargs,
        b"rand\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    if firstrand {
        gawk_initstate(
            1 as libc::c_int as libc::c_uint as libc::c_ulong,
            state,
            256 as libc::c_int as libc::c_long,
        );
        firstrand = 0 as libc::c_int != 0;
        gawk_setstate(state);
    }
    loop {
        let mut d1: libc::c_long = 0;
        let mut d2: libc::c_long = 0;
        d1 = gawk_random();
        d2 = gawk_random();
        tmprand = 0.5f64
            + (d1 as libc::c_double
                / (0x7fffffff as libc::c_long as libc::c_double + 1.0f64)
                + d2 as libc::c_double)
                / (0x7fffffff as libc::c_long as libc::c_double + 1.0f64);
        tmprand -= 0.5f64;
        if !(tmprand == 1.0f64) {
            break;
        }
    }
    return make_number.unwrap()(tmprand);
}
pub unsafe extern "C" fn do_srand(mut nargs: libc::c_int) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    static mut save_seed: libc::c_long = 1 as libc::c_int as libc::c_long;
    let mut ret: libc::c_long = save_seed;
    if firstrand {
        gawk_initstate(
            1 as libc::c_int as libc::c_uint as libc::c_ulong,
            state,
            256 as libc::c_int as libc::c_long,
        );
        firstrand = 0 as libc::c_int != 0;
        gawk_setstate(state);
    }
    check_args_min_max(
        nargs,
        b"srand\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        1 as libc::c_int,
    );
    if nargs == 0 as libc::c_int {
        save_seed = time(0 as *mut time_t);
        gawk_srandom(save_seed as libc::c_uint as libc::c_ulong);
    } else {
        tmp = POP_SCALAR();
        if do_flags as libc::c_uint
            & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                as libc::c_uint != 0
            && (*fixtype(tmp)).flags as libc::c_uint
                & NUMBER as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                2745 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: received non-numeric argument\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"srand\0" as *const u8 as *const libc::c_char,
            );
        }
        save_seed = (*force_number(tmp)).sub.val.fltnum as libc::c_long;
        gawk_srandom(save_seed as libc::c_uint as libc::c_ulong);
        DEREF(tmp);
    }
    return make_number.unwrap()(ret as libc::c_double);
}
pub unsafe extern "C" fn do_match(mut nargs: libc::c_int) -> *mut NODE {
    let mut tre: *mut NODE = 0 as *mut NODE;
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut dest: *mut NODE = 0 as *mut NODE;
    let mut it: *mut NODE = 0 as *mut NODE;
    let mut rstart: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut rlength: libc::c_int = 0;
    let mut rp: *mut Regexp = 0 as *mut Regexp;
    let mut s: regoff_t = 0;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buff: [libc::c_char; 100] = [0; 100];
    let mut amt: size_t = 0;
    let mut oldamt: size_t = 0 as libc::c_int as size_t;
    let mut ilen: size_t = 0;
    let mut slen: size_t = 0;
    let mut subsepstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut subseplen: size_t = 0;
    check_args_min_max(
        nargs,
        b"match\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int,
        3 as libc::c_int,
    );
    dest = 0 as *mut NODE;
    if nargs == 3 as libc::c_int {
        dest = POP_PARAM();
        if (*dest).type_0 as libc::c_uint
            != Node_var_array as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                2778 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"match: third argument is not an array\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        check_symtab_functab(
            dest,
            b"match\0" as *const u8 as *const libc::c_char,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: cannot use %s as third argument\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        ((*(*dest).sub.nodep.l.lp).clear).unwrap()(dest, 0 as *mut exp_node);
    }
    let fresh28 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    tre = (*fresh28).rptr;
    rp = re_update(tre);
    t1 = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
        && (*fixtype(t1)).flags as libc::c_uint
            & (STRING as libc::c_int | USER_INPUT as libc::c_int) as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            2787 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: received non-string first argument\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"match\0" as *const u8 as *const libc::c_char,
        );
    }
    rstart = research(
        rp,
        (*t1).sub.val.sp,
        0 as libc::c_int,
        (*t1).sub.val.slen,
        1 as libc::c_int,
    );
    if rstart >= 0 as libc::c_int {
        let mut wc_indices: *mut size_t = 0 as *mut size_t;
        rlength = *((*rp).regs.end).offset(0 as libc::c_int as isize)
            - *((*rp).regs.start).offset(0 as libc::c_int as isize);
        if rlength > 0 as libc::c_int && gawk_mb_cur_max > 1 as libc::c_int {
            t1 = str2wstr(t1, &mut wc_indices);
            rlength = (*wc_indices
                .offset((rstart + rlength - 1 as libc::c_int) as isize))
                .wrapping_sub(*wc_indices.offset(rstart as isize))
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
            rstart = *wc_indices.offset(rstart as isize) as libc::c_int;
        }
        rstart += 1;
        rstart;
        if !dest.is_null() {
            subsepstr = (*(*SUBSEP_node).sub.nodep.l.lptr).sub.val.sp;
            subseplen = (*(*SUBSEP_node).sub.nodep.l.lptr).sub.val.slen;
            ii = 0 as libc::c_int;
            while (ii as libc::c_uint) < (*rp).regs.num_regs {
                s = *((*rp).regs.start).offset(ii as isize);
                if s != -(1 as libc::c_int) {
                    let mut subpat_start: size_t = 0;
                    let mut subpat_len: size_t = 0;
                    start = ((*t1).sub.val.sp).offset(s as isize);
                    subpat_start = s as size_t;
                    len = *((*rp).regs.end).offset(ii as isize) - s;
                    subpat_len = len as size_t;
                    if len > 0 as libc::c_int && gawk_mb_cur_max > 1 as libc::c_int {
                        subpat_start = *wc_indices.offset(s as isize);
                        subpat_len = (*wc_indices
                            .offset((s + len - 1 as libc::c_int) as isize))
                            .wrapping_sub(subpat_start)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong);
                    }
                    it = make_str_node(start, len as size_t, 0 as libc::c_int);
                    (*it)
                        .flags = ::std::mem::transmute::<
                        libc::c_uint,
                        flagvals,
                    >(
                        (*it).flags as libc::c_uint
                            | USER_INPUT as libc::c_int as libc::c_uint,
                    );
                    assoc_set(dest, make_number.unwrap()(ii as libc::c_double), it);
                    sprintf(
                        buff.as_mut_ptr(),
                        b"%d\0" as *const u8 as *const libc::c_char,
                        ii,
                    );
                    ilen = strlen(buff.as_mut_ptr());
                    amt = ilen
                        .wrapping_add(subseplen)
                        .wrapping_add(
                            strlen(b"length\0" as *const u8 as *const libc::c_char),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong);
                    if oldamt == 0 as libc::c_int as libc::c_ulong {
                        buf = emalloc_real(
                            amt,
                            b"do_match\0" as *const u8 as *const libc::c_char,
                            b"buf\0" as *const u8 as *const libc::c_char,
                            b"builtin.c\0" as *const u8 as *const libc::c_char,
                            2833 as libc::c_int,
                        ) as *mut libc::c_char;
                    } else if amt > oldamt {
                        buf = erealloc_real(
                            buf as *mut libc::c_void,
                            amt,
                            b"do_match\0" as *const u8 as *const libc::c_char,
                            b"buf\0" as *const u8 as *const libc::c_char,
                            b"builtin.c\0" as *const u8 as *const libc::c_char,
                            2835 as libc::c_int,
                        ) as *mut libc::c_char;
                    }
                    oldamt = amt;
                    memcpy(
                        buf as *mut libc::c_void,
                        buff.as_mut_ptr() as *const libc::c_void,
                        ilen,
                    );
                    memcpy(
                        buf.offset(ilen as isize) as *mut libc::c_void,
                        subsepstr as *const libc::c_void,
                        subseplen,
                    );
                    memcpy(
                        buf.offset(ilen as isize).offset(subseplen as isize)
                            as *mut libc::c_void,
                        b"start\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        6 as libc::c_int as libc::c_ulong,
                    );
                    slen = ilen
                        .wrapping_add(subseplen)
                        .wrapping_add(5 as libc::c_int as libc::c_ulong);
                    assoc_set(
                        dest,
                        make_str_node(buf, slen, 0 as libc::c_int),
                        make_number
                            .unwrap()(
                            subpat_start as libc::c_double
                                + 1 as libc::c_int as libc::c_double,
                        ),
                    );
                    memcpy(
                        buf as *mut libc::c_void,
                        buff.as_mut_ptr() as *const libc::c_void,
                        ilen,
                    );
                    memcpy(
                        buf.offset(ilen as isize) as *mut libc::c_void,
                        subsepstr as *const libc::c_void,
                        subseplen,
                    );
                    memcpy(
                        buf.offset(ilen as isize).offset(subseplen as isize)
                            as *mut libc::c_void,
                        b"length\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        7 as libc::c_int as libc::c_ulong,
                    );
                    slen = ilen
                        .wrapping_add(subseplen)
                        .wrapping_add(6 as libc::c_int as libc::c_ulong);
                    assoc_set(
                        dest,
                        make_str_node(buf, slen, 0 as libc::c_int),
                        make_number.unwrap()(subpat_len as libc::c_double),
                    );
                }
                ii += 1;
                ii;
            }
            free(buf as *mut libc::c_void);
        }
        if !wc_indices.is_null() {
            free(wc_indices as *mut libc::c_void);
        }
    } else {
        rstart = 0 as libc::c_int;
        rlength = -(1 as libc::c_int);
    }
    DEREF(t1);
    unref((*RSTART_node).sub.nodep.l.lptr);
    (*RSTART_node).sub.nodep.l.lptr = make_number.unwrap()(rstart as libc::c_double);
    unref((*RLENGTH_node).sub.nodep.l.lptr);
    (*RLENGTH_node).sub.nodep.l.lptr = make_number.unwrap()(rlength as libc::c_double);
    return make_number.unwrap()(rstart as libc::c_double);
}
pub unsafe extern "C" fn do_sub(
    mut nargs: libc::c_int,
    mut flags: libc::c_uint,
) -> *mut NODE {
    let mut scan: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buflen: size_t = 0;
    let mut matchend: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut matchstart: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut textlen: size_t = 0 as libc::c_int as size_t;
    let mut repl: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut replend: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut repllen: size_t = 0;
    let mut sofar: libc::c_int = 0;
    let mut ampersands: libc::c_int = 0;
    let mut matches: libc::c_int = 0 as libc::c_int;
    let mut rp: *mut Regexp = 0 as *mut Regexp;
    let mut rep_node: *mut NODE = 0 as *mut NODE;
    let mut target: *mut NODE = 0 as *mut NODE;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut how_many: libc::c_long = 1 as libc::c_int as libc::c_long;
    let mut global: bool = false;
    let mut current: libc::c_long = 0;
    let mut lastmatchnonzero: bool = false;
    let mut mb_indices: *mut libc::c_char = 0 as *mut libc::c_char;
    if flags & 0x2 as libc::c_int as libc::c_uint != 0 as libc::c_int as libc::c_uint {
        let mut d: libc::c_double = 0.;
        let mut glob_flag: *mut NODE = 0 as *mut NODE;
        check_exact_args(
            nargs,
            b"gensub\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int,
        );
        tmp = (*stack_ptr.offset(-(3 as libc::c_int as isize))).rptr;
        rp = re_update(tmp);
        target = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
        glob_flag = POP_SCALAR();
        if (*glob_flag).flags as libc::c_uint & STRING as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
            && (*glob_flag).sub.val.slen > 0 as libc::c_int as libc::c_ulong
            && (*((*glob_flag).sub.val.sp).offset(0 as libc::c_int as isize)
                as libc::c_int == 'g' as i32
                || *((*glob_flag).sub.val.sp).offset(0 as libc::c_int as isize)
                    as libc::c_int == 'G' as i32)
        {
            how_many = -(1 as libc::c_int) as libc::c_long;
        } else {
            force_number(glob_flag);
            d = (*glob_flag).sub.val.fltnum;
            if d < 1 as libc::c_int as libc::c_double {
                how_many = 1 as libc::c_int as libc::c_long;
            } else if d < 9223372036854775807 as libc::c_long as libc::c_double {
                how_many = d as libc::c_long;
            } else {
                how_many = 9223372036854775807 as libc::c_long;
            }
            if d <= 0 as libc::c_int as libc::c_double {
                force_string_fmt(glob_flag, CONVFMT, CONVFMTidx);
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"builtin.c\0" as *const u8 as *const libc::c_char,
                    3027 as libc::c_int,
                );
                (Some(
                    (Some(
                        r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                    ))
                        .unwrap(),
                ))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"gensub: third argument `%.*s' treated as 1\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*glob_flag).sub.val.slen as libc::c_int,
                    (*glob_flag).sub.val.sp,
                );
            }
        }
        DEREF(glob_flag);
    } else {
        if flags & 0x1 as libc::c_int as libc::c_uint != 0 as libc::c_int as libc::c_uint
        {
            check_exact_args(
                nargs,
                b"gsub\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int,
            );
        } else {
            check_exact_args(
                nargs,
                b"sub\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int,
            );
        }
        tmp = (*stack_ptr.offset(-(2 as libc::c_int as isize))).rptr;
        rp = re_update(tmp);
        if flags & 0x1 as libc::c_int as libc::c_uint != 0 as libc::c_int as libc::c_uint
        {
            how_many = -(1 as libc::c_int) as libc::c_long;
        }
        if flags & 0x4 as libc::c_int as libc::c_uint != 0 as libc::c_int as libc::c_uint
        {
            target = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
        } else {
            let fresh29 = stack_ptr;
            stack_ptr = stack_ptr.offset(-1);
            lhs = (*fresh29).lptr;
            target = force_string_fmt(*lhs, CONVFMT, CONVFMTidx);
        }
    }
    global = how_many == -(1 as libc::c_int) as libc::c_long;
    rep_node = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
    stack_ptr = stack_ptr.offset(-1);
    stack_ptr;
    if !(research(
        rp,
        (*target).sub.val.sp,
        0 as libc::c_int,
        (*target).sub.val.slen,
        1 as libc::c_int,
    ) == -(1 as libc::c_int)
        || *((*rp).regs.start).offset(0 as libc::c_int as isize) as libc::c_ulong
            > (*target).sub.val.slen)
    {
        text = (*target).sub.val.sp;
        textlen = (*target).sub.val.slen;
        repl = (*rep_node).sub.val.sp;
        replend = repl.offset((*rep_node).sub.val.slen as isize);
        repllen = replend.offset_from(repl) as libc::c_long as size_t;
        ampersands = 0 as libc::c_int;
        if gawk_mb_cur_max > 1 as libc::c_int
            && repllen > 0 as libc::c_int as libc::c_ulong
        {
            mb_indices = emalloc_real(
                repllen
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    ),
                b"do_sub\0" as *const u8 as *const libc::c_char,
                b"mb_indices\0" as *const u8 as *const libc::c_char,
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                3086 as libc::c_int,
            ) as *mut libc::c_char;
            index_multibyte_buffer(repl, mb_indices, repllen as libc::c_int);
        }
        scan = repl;
        while scan < replend {
            if (gawk_mb_cur_max == 1 as libc::c_int
                || repllen > 0 as libc::c_int as libc::c_ulong
                    && *mb_indices
                        .offset(scan.offset_from(repl) as libc::c_long as isize)
                        as libc::c_int == 1 as libc::c_int)
                && *scan as libc::c_int == '&' as i32
            {
                repllen = repllen.wrapping_sub(1);
                repllen;
                ampersands += 1;
                ampersands;
            } else if *scan as libc::c_int == '\\' as i32 {
                if flags & 0x2 as libc::c_int as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint
                {
                    if *(*__ctype_b_loc())
                        .offset(
                            *scan.offset(1 as libc::c_int as isize) as libc::c_uchar
                                as libc::c_int as isize,
                        ) as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                    {
                        ampersands += 1;
                        ampersands;
                        scan = scan.offset(1);
                        scan;
                    } else {
                        repllen = repllen.wrapping_sub(1);
                        repllen;
                        scan = scan.offset(1);
                        scan;
                    }
                } else if do_flags as libc::c_uint
                    & DO_POSIX as libc::c_int as libc::c_uint != 0
                {
                    if *scan.offset(1 as libc::c_int as isize) as libc::c_int
                        == '&' as i32
                        || *scan.offset(1 as libc::c_int as isize) as libc::c_int
                            == '\\' as i32
                    {
                        repllen = repllen.wrapping_sub(1);
                        repllen;
                        scan = scan.offset(1);
                        scan;
                    }
                } else if strncmp(
                    scan,
                    b"\\\\\\&\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                    || strncmp(
                        scan,
                        b"\\\\\\\\\0" as *const u8 as *const libc::c_char,
                        4 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    repllen = (repllen as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                    scan = scan.offset(3 as libc::c_int as isize);
                } else if strncmp(
                    scan,
                    b"\\\\&\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    ampersands += 1;
                    ampersands;
                    repllen = repllen.wrapping_sub(1);
                    repllen;
                    scan = scan.offset(2 as libc::c_int as isize);
                } else if *scan.offset(1 as libc::c_int as isize) as libc::c_int
                    == '&' as i32
                {
                    repllen = repllen.wrapping_sub(1);
                    repllen;
                    scan = scan.offset(1);
                    scan;
                }
            }
            scan = scan.offset(1);
            scan;
        }
        lastmatchnonzero = 0 as libc::c_int != 0;
        buflen = textlen
            .wrapping_add(
                ((ampersands + 1 as libc::c_int) as libc::c_ulong).wrapping_mul(repllen),
            )
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        buf = emalloc_real(
            buflen.wrapping_add(1 as libc::c_int as libc::c_ulong),
            b"do_sub\0" as *const u8 as *const libc::c_char,
            b"buf\0" as *const u8 as *const libc::c_char,
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            3139 as libc::c_int,
        ) as *mut libc::c_char;
        *buf.offset(buflen as isize) = '\0' as i32 as libc::c_char;
        bp = buf;
        current = 1 as libc::c_int as libc::c_long;
        loop {
            matches += 1;
            matches;
            matchstart = ((*target).sub.val.sp)
                .offset(*((*rp).regs.start).offset(0 as libc::c_int as isize) as isize);
            matchend = ((*target).sub.val.sp)
                .offset(*((*rp).regs.end).offset(0 as libc::c_int as isize) as isize);
            len = (matchend.offset_from(text) as libc::c_long as libc::c_ulong)
                .wrapping_add(repllen)
                .wrapping_add(
                    (ampersands as libc::c_long
                        * matchend.offset_from(matchstart) as libc::c_long)
                        as libc::c_ulong,
                )
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            sofar = bp.offset_from(buf) as libc::c_long as libc::c_int;
            while buflen
                < (sofar as libc::c_ulong)
                    .wrapping_add(len)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            {
                buflen = (buflen as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
                buf = erealloc_real(
                    buf as *mut libc::c_void,
                    buflen,
                    b"sub_common\0" as *const u8 as *const libc::c_char,
                    b"buf\0" as *const u8 as *const libc::c_char,
                    b"builtin.c\0" as *const u8 as *const libc::c_char,
                    3166 as libc::c_int,
                ) as *mut libc::c_char;
                bp = buf.offset(sofar as isize);
            }
            scan = text;
            while scan < matchstart {
                let fresh30 = bp;
                bp = bp.offset(1);
                *fresh30 = *scan;
                scan = scan.offset(1);
                scan;
            }
            if global as libc::c_int != 0 || current == how_many {
                if matchstart == matchend && lastmatchnonzero as libc::c_int != 0
                    && matchstart == text
                {
                    lastmatchnonzero = 0 as libc::c_int != 0;
                    matches -= 1;
                    matches;
                } else {
                    scan = repl;
                    while scan < replend {
                        if *scan as libc::c_int == '&' as i32
                            && (gawk_mb_cur_max == 1 as libc::c_int
                                || *mb_indices
                                    .offset(scan.offset_from(repl) as libc::c_long as isize)
                                    as libc::c_int == 1 as libc::c_int)
                        {
                            cp = matchstart;
                            while cp < matchend {
                                let fresh31 = bp;
                                bp = bp.offset(1);
                                *fresh31 = *cp;
                                cp = cp.offset(1);
                                cp;
                            }
                        } else if *scan as libc::c_int == '\\' as i32
                            && (gawk_mb_cur_max == 1 as libc::c_int
                                || repllen > 0 as libc::c_int as libc::c_ulong
                                    && *mb_indices
                                        .offset(scan.offset_from(repl) as libc::c_long as isize)
                                        as libc::c_int == 1 as libc::c_int)
                        {
                            if flags & 0x2 as libc::c_int as libc::c_uint != 0 {
                                if *(*__ctype_b_loc())
                                    .offset(
                                        *scan.offset(1 as libc::c_int as isize) as libc::c_uchar
                                            as libc::c_int as isize,
                                    ) as libc::c_int
                                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0
                                {
                                    let mut dig: libc::c_int = *scan
                                        .offset(1 as libc::c_int as isize) as libc::c_int
                                        - '0' as i32;
                                    if (dig as libc::c_uint) < (*rp).regs.num_regs
                                        && *((*rp).regs.start).offset(dig as isize)
                                            != -(1 as libc::c_int)
                                    {
                                        let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
                                        let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
                                        start = ((*target).sub.val.sp)
                                            .offset(*((*rp).regs.start).offset(dig as isize) as isize);
                                        end = ((*target).sub.val.sp)
                                            .offset(*((*rp).regs.end).offset(dig as isize) as isize);
                                        cp = start;
                                        while cp < end {
                                            let fresh32 = bp;
                                            bp = bp.offset(1);
                                            *fresh32 = *cp;
                                            cp = cp.offset(1);
                                            cp;
                                        }
                                    }
                                    scan = scan.offset(1);
                                    scan;
                                } else {
                                    scan = scan.offset(1);
                                    let fresh33 = bp;
                                    bp = bp.offset(1);
                                    *fresh33 = *scan;
                                }
                            } else if do_flags as libc::c_uint
                                & DO_POSIX as libc::c_int as libc::c_uint != 0
                            {
                                if *scan.offset(1 as libc::c_int as isize) as libc::c_int
                                    == '&' as i32
                                    || *scan.offset(1 as libc::c_int as isize) as libc::c_int
                                        == '\\' as i32
                                {
                                    scan = scan.offset(1);
                                    scan;
                                }
                                let fresh34 = bp;
                                bp = bp.offset(1);
                                *fresh34 = *scan;
                            } else if strncmp(
                                scan,
                                b"\\\\\\&\0" as *const u8 as *const libc::c_char,
                                4 as libc::c_int as libc::c_ulong,
                            ) == 0 as libc::c_int
                                || strncmp(
                                    scan,
                                    b"\\\\\\\\\0" as *const u8 as *const libc::c_char,
                                    4 as libc::c_int as libc::c_ulong,
                                ) == 0 as libc::c_int
                            {
                                let fresh35 = bp;
                                bp = bp.offset(1);
                                *fresh35 = '\\' as i32 as libc::c_char;
                                let fresh36 = bp;
                                bp = bp.offset(1);
                                *fresh36 = *scan.offset(3 as libc::c_int as isize);
                                scan = scan.offset(3 as libc::c_int as isize);
                            } else if strncmp(
                                scan,
                                b"\\\\&\0" as *const u8 as *const libc::c_char,
                                3 as libc::c_int as libc::c_ulong,
                            ) == 0 as libc::c_int
                            {
                                let fresh37 = bp;
                                bp = bp.offset(1);
                                *fresh37 = '\\' as i32 as libc::c_char;
                                cp = matchstart;
                                while cp < matchend {
                                    let fresh38 = bp;
                                    bp = bp.offset(1);
                                    *fresh38 = *cp;
                                    cp = cp.offset(1);
                                    cp;
                                }
                                scan = scan.offset(2 as libc::c_int as isize);
                            } else if *scan.offset(1 as libc::c_int as isize)
                                as libc::c_int == '&' as i32
                            {
                                let fresh39 = bp;
                                bp = bp.offset(1);
                                *fresh39 = '&' as i32 as libc::c_char;
                                scan = scan.offset(1);
                                scan;
                            } else {
                                let fresh40 = bp;
                                bp = bp.offset(1);
                                *fresh40 = *scan;
                            }
                        } else {
                            let fresh41 = bp;
                            bp = bp.offset(1);
                            *fresh41 = *scan;
                        }
                        scan = scan.offset(1);
                        scan;
                    }
                    if matchstart != matchend {
                        lastmatchnonzero = 1 as libc::c_int != 0;
                    }
                }
            } else {
                cp = matchstart;
                while cp < matchend {
                    let fresh42 = bp;
                    bp = bp.offset(1);
                    *fresh42 = *cp;
                    cp = cp.offset(1);
                    cp;
                }
            }
            if matchstart == matchend && matchend < text.offset(textlen as isize) {
                let fresh43 = bp;
                bp = bp.offset(1);
                *fresh43 = *matchend;
                matchend = matchend.offset(1);
                matchend;
            }
            textlen = text.offset(textlen as isize).offset_from(matchend) as libc::c_long
                as size_t;
            text = matchend;
            if current >= how_many && !global
                || textlen as libc::c_long <= 0 as libc::c_int as libc::c_long
                    && matchstart == matchend
                || research(
                    rp,
                    (*target).sub.val.sp,
                    text.offset_from((*target).sub.val.sp) as libc::c_long
                        as libc::c_int,
                    textlen,
                    1 as libc::c_int,
                ) == -(1 as libc::c_int)
            {
                break;
            }
            current += 1;
            current;
        }
        sofar = bp.offset_from(buf) as libc::c_long as libc::c_int;
        if buflen
            < (sofar as libc::c_ulong)
                .wrapping_add(textlen)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            buflen = (sofar as libc::c_ulong)
                .wrapping_add(textlen)
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            buf = erealloc_real(
                buf as *mut libc::c_void,
                buflen,
                b"do_sub\0" as *const u8 as *const libc::c_char,
                b"buf\0" as *const u8 as *const libc::c_char,
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                3285 as libc::c_int,
            ) as *mut libc::c_char;
            bp = buf.offset(sofar as isize);
        }
        scan = text;
        while scan < text.offset(textlen as isize) {
            let fresh44 = bp;
            bp = bp.offset(1);
            *fresh44 = *scan;
            scan = scan.offset(1);
            scan;
        }
        *bp = '\0' as i32 as libc::c_char;
        textlen = bp.offset_from(buf) as libc::c_long as size_t;
        if !mb_indices.is_null() {
            free(mb_indices as *mut libc::c_void);
        }
    }
    DEREF(rep_node);
    if (matches == 0 as libc::c_int
        || flags & 0x4 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint) && !buf.is_null()
    {
        free(buf as *mut libc::c_void);
        buf = 0 as *mut libc::c_char;
    }
    if flags & 0x2 as libc::c_int as libc::c_uint != 0 {
        if matches > 0 as libc::c_int {
            DEREF(target);
            return make_str_node(buf, textlen, 2 as libc::c_int);
        } else if (*target).flags as libc::c_uint & STRING as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        {
            DEREF(target);
            return make_str_node(
                (*target).sub.val.sp,
                (*target).sub.val.slen,
                0 as libc::c_int,
            );
        }
        return target;
    }
    if flags & 0x4 as libc::c_int as libc::c_uint != 0 as libc::c_int as libc::c_uint {
        DEREF(target);
    } else if matches > 0 as libc::c_int {
        let mut is_regex: bool = 0 as libc::c_int != 0;
        let mut target_0: *mut NODE = *lhs;
        if (*target_0).flags as libc::c_uint & REGEX as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            is_regex = 1 as libc::c_int != 0;
            if (*target_0).valref == 1 as libc::c_int as libc::c_long {
                refree(
                    (*(*target_0).sub.val.typre)
                        .sub
                        .nodep
                        .r
                        .preg[0 as libc::c_int as usize],
                );
                if !((*(*target_0).sub.val.typre)
                    .sub
                    .nodep
                    .r
                    .preg[1 as libc::c_int as usize])
                    .is_null()
                {
                    refree(
                        (*(*target_0).sub.val.typre)
                            .sub
                            .nodep
                            .r
                            .preg[1 as libc::c_int as usize],
                    );
                }
                let ref mut fresh45 = (*((*target_0).sub.val.typre as *mut block_item))
                    .freep;
                *fresh45 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
                nextfree[BLOCK_NODE as libc::c_int as usize]
                    .freep = (*target_0).sub.val.typre as *mut block_item;
            }
        }
        unref(*lhs);
        if is_regex {
            *lhs = make_typed_regex(buf, textlen);
        } else {
            *lhs = make_str_node(buf, textlen, 2 as libc::c_int);
        }
    }
    return make_number.unwrap()(matches as libc::c_double);
}
pub unsafe extern "C" fn call_sub(
    mut name: *const libc::c_char,
    mut nargs: libc::c_int,
) -> *mut NODE {
    let mut flags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut regex: *mut NODE = 0 as *mut NODE;
    let mut replace: *mut NODE = 0 as *mut NODE;
    let mut glob_flag: *mut NODE = 0 as *mut NODE;
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut rhs: *mut NODE = 0 as *mut NODE;
    let mut zero: *mut NODE = make_number.unwrap()(0.0f64);
    let mut result: *mut NODE = 0 as *mut NODE;
    if *name.offset(0 as libc::c_int as isize) as libc::c_int == 'g' as i32 {
        if *name.offset(1 as libc::c_int as isize) as libc::c_int == 'e' as i32 {
            flags = 0x2 as libc::c_int as libc::c_uint;
        } else {
            flags = 0x1 as libc::c_int as libc::c_uint;
        }
    }
    let mut need_free: bool = 0 as libc::c_int != 0;
    if flags == 0 as libc::c_int as libc::c_uint
        || flags == 0x1 as libc::c_int as libc::c_uint
    {
        if nargs != 2 as libc::c_int {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                3386 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: can be called indirectly only with two arguments\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                name,
            );
        }
        replace = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
        let fresh46 = stack_ptr;
        stack_ptr = stack_ptr.offset(-1);
        regex = (*fresh46).rptr;
        if (*regex).flags as libc::c_uint & REGEX as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            regex = (*regex).sub.val.typre;
        } else {
            regex = make_regnode(Node_regex, regex);
            need_free = 1 as libc::c_int != 0;
        }
        let ref mut fresh47 = (*if stack_ptr < stack_top {
            stack_ptr = stack_ptr.offset(1);
            stack_ptr
        } else {
            grow_stack()
        })
            .rptr;
        *fresh47 = regex;
        let ref mut fresh48 = (*if stack_ptr < stack_top {
            stack_ptr = stack_ptr.offset(1);
            stack_ptr
        } else {
            grow_stack()
        })
            .rptr;
        *fresh48 = replace;
        lhs = r_get_field(zero, 0 as *mut Func_ptr, 1 as libc::c_int != 0);
        nargs += 1;
        nargs;
        let ref mut fresh49 = (*if stack_ptr < stack_top {
            stack_ptr = stack_ptr.offset(1);
            stack_ptr
        } else {
            grow_stack()
        })
            .lptr;
        *fresh49 = lhs;
    } else {
        if nargs < 3 as libc::c_int || nargs > 4 as libc::c_int {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                3409 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"indirect call to gensub requires three or four arguments\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        if nargs == 4 as libc::c_int {
            let fresh50 = stack_ptr;
            stack_ptr = stack_ptr.offset(-1);
            rhs = (*fresh50).rptr;
        } else {
            rhs = 0 as *mut NODE;
        }
        glob_flag = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
        replace = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
        let fresh51 = stack_ptr;
        stack_ptr = stack_ptr.offset(-1);
        regex = (*fresh51).rptr;
        if (*regex).flags as libc::c_uint & REGEX as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            regex = (*regex).sub.val.typre;
        } else {
            regex = make_regnode(Node_regex, regex);
            need_free = 1 as libc::c_int != 0;
        }
        let ref mut fresh52 = (*if stack_ptr < stack_top {
            stack_ptr = stack_ptr.offset(1);
            stack_ptr
        } else {
            grow_stack()
        })
            .rptr;
        *fresh52 = regex;
        let ref mut fresh53 = (*if stack_ptr < stack_top {
            stack_ptr = stack_ptr.offset(1);
            stack_ptr
        } else {
            grow_stack()
        })
            .rptr;
        *fresh53 = replace;
        let ref mut fresh54 = (*if stack_ptr < stack_top {
            stack_ptr = stack_ptr.offset(1);
            stack_ptr
        } else {
            grow_stack()
        })
            .rptr;
        *fresh54 = glob_flag;
        if rhs.is_null() {
            lhs = r_get_field(zero, 0 as *mut Func_ptr, 1 as libc::c_int != 0);
            rhs = *lhs;
            (*rhs).valref += 1;
            (*rhs).valref;
            let ref mut fresh55 = (*if stack_ptr < stack_top {
                stack_ptr = stack_ptr.offset(1);
                stack_ptr
            } else {
                grow_stack()
            })
                .rptr;
            *fresh55 = rhs;
            nargs += 1;
            nargs;
        } else {
            let ref mut fresh56 = (*if stack_ptr < stack_top {
                stack_ptr = stack_ptr.offset(1);
                stack_ptr
            } else {
                grow_stack()
            })
                .rptr;
            *fresh56 = rhs;
        }
    }
    unref(zero);
    result = do_sub(nargs, flags);
    if need_free {
        refree((*regex).sub.nodep.r.preg[0 as libc::c_int as usize]);
        if !((*regex).sub.nodep.r.preg[1 as libc::c_int as usize]).is_null() {
            refree((*regex).sub.nodep.r.preg[1 as libc::c_int as usize]);
        }
        let ref mut fresh57 = (*(regex as *mut block_item)).freep;
        *fresh57 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
        nextfree[BLOCK_NODE as libc::c_int as usize].freep = regex as *mut block_item;
    }
    if flags != 0x2 as libc::c_int as libc::c_uint {
        reset_record();
    }
    return result;
}
pub unsafe extern "C" fn call_match(mut nargs: libc::c_int) -> *mut NODE {
    let mut regex: *mut NODE = 0 as *mut NODE;
    let mut text: *mut NODE = 0 as *mut NODE;
    let mut array: *mut NODE = 0 as *mut NODE;
    let mut result: *mut NODE = 0 as *mut NODE;
    if nargs < 2 as libc::c_int || nargs > 3 as libc::c_int {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            3471 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"indirect call to match requires two or three arguments\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    array = 0 as *mut NODE;
    text = array;
    regex = text;
    if nargs == 3 as libc::c_int {
        let fresh58 = stack_ptr;
        stack_ptr = stack_ptr.offset(-1);
        array = (*fresh58).rptr;
    }
    let fresh59 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    regex = (*fresh59).rptr;
    let mut need_free: bool = 0 as libc::c_int != 0;
    if (*regex).flags as libc::c_uint & REGEX as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        regex = (*regex).sub.val.typre;
    } else {
        regex = make_regnode(Node_regex, regex);
        need_free = 1 as libc::c_int != 0;
    }
    let ref mut fresh60 = (*if stack_ptr < stack_top {
        stack_ptr = stack_ptr.offset(1);
        stack_ptr
    } else {
        grow_stack()
    })
        .rptr;
    *fresh60 = regex;
    if !array.is_null() {
        let ref mut fresh61 = (*if stack_ptr < stack_top {
            stack_ptr = stack_ptr.offset(1);
            stack_ptr
        } else {
            grow_stack()
        })
            .rptr;
        *fresh61 = array;
    }
    result = do_match(nargs);
    if need_free {
        refree((*regex).sub.nodep.r.preg[0 as libc::c_int as usize]);
        if !((*regex).sub.nodep.r.preg[1 as libc::c_int as usize]).is_null() {
            refree((*regex).sub.nodep.r.preg[1 as libc::c_int as usize]);
        }
        let ref mut fresh62 = (*(regex as *mut block_item)).freep;
        *fresh62 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
        nextfree[BLOCK_NODE as libc::c_int as usize].freep = regex as *mut block_item;
    }
    return result;
}
pub unsafe extern "C" fn call_split_func(
    mut name: *const libc::c_char,
    mut nargs: libc::c_int,
) -> *mut NODE {
    let mut regex: *mut NODE = 0 as *mut NODE;
    let mut seps: *mut NODE = 0 as *mut NODE;
    let mut result: *mut NODE = 0 as *mut NODE;
    seps = 0 as *mut NODE;
    regex = seps;
    if nargs < 2 as libc::c_int || nargs > 4 as libc::c_int {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            3515 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"indirect call to %s requires two to four arguments\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            name,
        );
    }
    if nargs == 4 as libc::c_int {
        let fresh63 = stack_ptr;
        stack_ptr = stack_ptr.offset(-1);
        seps = (*fresh63).rptr;
    }
    let mut need_free: bool = 0 as libc::c_int != 0;
    if nargs >= 3 as libc::c_int {
        regex = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
        if (*regex).flags as libc::c_uint & REGEX as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            regex = (*regex).sub.val.typre;
        } else {
            regex = make_regnode(Node_regex, regex);
            need_free = 1 as libc::c_int != 0;
        }
    } else {
        if *name.offset(0 as libc::c_int as isize) as libc::c_int == 's' as i32 {
            regex = make_regnode(Node_regex, (*FS_node).sub.nodep.l.lptr);
            (*regex)
                .sub
                .nodep
                .reflags = ::std::mem::transmute::<
                libc::c_uint,
                reflagvals,
            >(
                (*regex).sub.nodep.reflags as libc::c_uint
                    | FS_DFLT as libc::c_int as libc::c_uint,
            );
        } else {
            regex = make_regnode(Node_regex, (*FPAT_node).sub.nodep.l.lptr);
        }
        need_free = 1 as libc::c_int != 0;
        nargs += 1;
        nargs;
    }
    let ref mut fresh64 = (*if stack_ptr < stack_top {
        stack_ptr = stack_ptr.offset(1);
        stack_ptr
    } else {
        grow_stack()
    })
        .rptr;
    *fresh64 = regex;
    if !seps.is_null() {
        let ref mut fresh65 = (*if stack_ptr < stack_top {
            stack_ptr = stack_ptr.offset(1);
            stack_ptr
        } else {
            grow_stack()
        })
            .rptr;
        *fresh65 = seps;
    }
    result = if *name.offset(0 as libc::c_int as isize) as libc::c_int == 's' as i32 {
        do_split(nargs)
    } else {
        do_patsplit(nargs)
    };
    if need_free {
        refree((*regex).sub.nodep.r.preg[0 as libc::c_int as usize]);
        if !((*regex).sub.nodep.r.preg[1 as libc::c_int as usize]).is_null() {
            refree((*regex).sub.nodep.r.preg[1 as libc::c_int as usize]);
        }
        let ref mut fresh66 = (*(regex as *mut block_item)).freep;
        *fresh66 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
        nextfree[BLOCK_NODE as libc::c_int as usize].freep = regex as *mut block_item;
    }
    return result;
}
unsafe extern "C" fn make_integer(mut n: uintmax_t) -> *mut NODE {
    n = adjust_uint(n);
    return make_number.unwrap()(n as libc::c_double);
}
pub unsafe extern "C" fn do_lshift(mut nargs: libc::c_int) -> *mut NODE {
    let mut s1: *mut NODE = 0 as *mut NODE;
    let mut s2: *mut NODE = 0 as *mut NODE;
    let mut uval: uintmax_t = 0;
    let mut ushift: uintmax_t = 0;
    let mut res: uintmax_t = 0;
    let mut val: libc::c_double = 0.;
    let mut shift: libc::c_double = 0.;
    check_exact_args(
        nargs,
        b"lshift\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int,
    );
    s2 = POP_SCALAR();
    let fresh67 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    s1 = (*fresh67).rptr;
    if (*s1).type_0 as libc::c_uint == Node_var_array as libc::c_int as libc::c_uint {
        DEREF(s2);
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            3581 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"attempt to use array `%s' in a scalar context\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            array_vname(s1),
        );
    }
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
    {
        if (*fixtype(s1)).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                3584 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: received non-numeric first argument\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"lshift\0" as *const u8 as *const libc::c_char,
            );
        }
        if (*fixtype(s2)).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                3586 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: received non-numeric second argument\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"lshift\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    val = (*force_number(s1)).sub.val.fltnum;
    shift = (*force_number(s2)).sub.val.fltnum;
    if val < 0 as libc::c_int as libc::c_double
        || shift < 0 as libc::c_int as libc::c_double
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            3592 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"lshift(%f, %f): negative values are not allowed\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            val,
            shift,
        );
    }
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
    {
        if double_to_int(val) != val || double_to_int(shift) != shift {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                3596 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"lshift(%f, %f): fractional values will be truncated\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                val,
                shift,
            );
        }
        if shift
            >= (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_double
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                3598 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"lshift(%f, %f): too large shift value will give strange results\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                val,
                shift,
            );
        }
    }
    DEREF(s1);
    DEREF(s2);
    uval = val as uintmax_t;
    ushift = shift as uintmax_t;
    res = uval << ushift;
    return make_integer(res);
}
pub unsafe extern "C" fn do_rshift(mut nargs: libc::c_int) -> *mut NODE {
    let mut s1: *mut NODE = 0 as *mut NODE;
    let mut s2: *mut NODE = 0 as *mut NODE;
    let mut uval: uintmax_t = 0;
    let mut ushift: uintmax_t = 0;
    let mut res: uintmax_t = 0;
    let mut val: libc::c_double = 0.;
    let mut shift: libc::c_double = 0.;
    check_exact_args(
        nargs,
        b"rshift\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int,
    );
    s2 = POP_SCALAR();
    let fresh68 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    s1 = (*fresh68).rptr;
    if (*s1).type_0 as libc::c_uint == Node_var_array as libc::c_int as libc::c_uint {
        DEREF(s2);
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            3622 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"attempt to use array `%s' in a scalar context\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            array_vname(s1),
        );
    }
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
    {
        if (*fixtype(s1)).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                3625 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: received non-numeric first argument\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"rshift\0" as *const u8 as *const libc::c_char,
            );
        }
        if (*fixtype(s2)).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                3627 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: received non-numeric second argument\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"rshift\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    val = (*force_number(s1)).sub.val.fltnum;
    shift = (*force_number(s2)).sub.val.fltnum;
    if val < 0 as libc::c_int as libc::c_double
        || shift < 0 as libc::c_int as libc::c_double
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            3633 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"rshift(%f, %f): negative values are not allowed\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            val,
            shift,
        );
    }
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
    {
        if double_to_int(val) != val || double_to_int(shift) != shift {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                3637 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"rshift(%f, %f): fractional values will be truncated\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                val,
                shift,
            );
        }
        if shift
            >= (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_double
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                3639 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"rshift(%f, %f): too large shift value will give strange results\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                val,
                shift,
            );
        }
    }
    DEREF(s1);
    DEREF(s2);
    uval = val as uintmax_t;
    ushift = shift as uintmax_t;
    res = uval >> ushift;
    return make_integer(res);
}
pub unsafe extern "C" fn do_and(mut nargs: libc::c_int) -> *mut NODE {
    let mut s1: *mut NODE = 0 as *mut NODE;
    let mut res: uintmax_t = 0;
    let mut uval: uintmax_t = 0;
    let mut val: libc::c_double = 0.;
    res = !(0 as libc::c_int as uintmax_t);
    if nargs < 2 as libc::c_int {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            3663 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: called with less than two arguments\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"and\0" as *const u8 as *const libc::c_char,
        );
    }
    while nargs > 0 as libc::c_int {
        s1 = POP_SCALAR();
        if do_flags as libc::c_uint
            & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                as libc::c_uint != 0
            && (*fixtype(s1)).flags as libc::c_uint
                & NUMBER as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                3668 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: argument %d is non-numeric\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"and\0" as *const u8 as *const libc::c_char,
                nargs,
            );
        }
        val = (*force_number(s1)).sub.val.fltnum;
        if val < 0 as libc::c_int as libc::c_double {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                3672 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: argument %d negative value %g is not allowed\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"and\0" as *const u8 as *const libc::c_char,
                nargs,
                val,
            );
        }
        uval = val as uintmax_t;
        res &= uval;
        DEREF(s1);
        nargs -= 1;
        nargs;
    }
    return make_integer(res);
}
pub unsafe extern "C" fn do_or(mut nargs: libc::c_int) -> *mut NODE {
    let mut s1: *mut NODE = 0 as *mut NODE;
    let mut res: uintmax_t = 0;
    let mut uval: uintmax_t = 0;
    let mut val: libc::c_double = 0.;
    res = 0 as libc::c_int as uintmax_t;
    if nargs < 2 as libc::c_int {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            3694 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: called with less than two arguments\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"or\0" as *const u8 as *const libc::c_char,
        );
    }
    while nargs > 0 as libc::c_int {
        s1 = POP_SCALAR();
        if do_flags as libc::c_uint
            & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                as libc::c_uint != 0
            && (*fixtype(s1)).flags as libc::c_uint
                & NUMBER as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                3699 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: argument %d is non-numeric\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"or\0" as *const u8 as *const libc::c_char,
                nargs,
            );
        }
        val = (*force_number(s1)).sub.val.fltnum;
        if val < 0 as libc::c_int as libc::c_double {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                3703 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: argument %d negative value %g is not allowed\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"or\0" as *const u8 as *const libc::c_char,
                nargs,
                val,
            );
        }
        uval = val as uintmax_t;
        res |= uval;
        DEREF(s1);
        nargs -= 1;
        nargs;
    }
    return make_integer(res);
}
pub unsafe extern "C" fn do_xor(mut nargs: libc::c_int) -> *mut NODE {
    let mut s1: *mut NODE = 0 as *mut NODE;
    let mut res: uintmax_t = 0;
    let mut uval: uintmax_t = 0;
    let mut val: libc::c_double = 0.;
    if nargs < 2 as libc::c_int {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            3724 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: called with less than two arguments\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"xor\0" as *const u8 as *const libc::c_char,
        );
    }
    res = 0 as libc::c_int as uintmax_t;
    while nargs > 0 as libc::c_int {
        s1 = POP_SCALAR();
        if do_flags as libc::c_uint
            & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                as libc::c_uint != 0
            && (*fixtype(s1)).flags as libc::c_uint
                & NUMBER as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                3730 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: argument %d is non-numeric\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"xor\0" as *const u8 as *const libc::c_char,
                nargs,
            );
        }
        val = (*force_number(s1)).sub.val.fltnum;
        if val < 0 as libc::c_int as libc::c_double {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                3734 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: argument %d negative value %g is not allowed\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"xor\0" as *const u8 as *const libc::c_char,
                nargs,
                val,
            );
        }
        uval = val as uintmax_t;
        res ^= uval;
        DEREF(s1);
        nargs -= 1;
        nargs;
    }
    return make_integer(res);
}
pub unsafe extern "C" fn do_compl(mut nargs: libc::c_int) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut d: libc::c_double = 0.;
    let mut uval: uintmax_t = 0;
    check_exact_args(
        nargs,
        b"compl\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    tmp = POP_SCALAR();
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
        && (*fixtype(tmp)).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            3758 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: received non-numeric argument\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"compl\0" as *const u8 as *const libc::c_char,
        );
    }
    d = (*force_number(tmp)).sub.val.fltnum;
    DEREF(tmp);
    if d < 0 as libc::c_int as libc::c_double {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            3763 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"compl(%f): negative value is not allowed\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            d,
        );
    }
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0 && double_to_int(d) != d
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            3766 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"compl(%f): fractional value will be truncated\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            d,
        );
    }
    uval = d as uintmax_t;
    uval = !uval;
    return make_integer(uval);
}
pub unsafe extern "C" fn do_strtonum(mut nargs: libc::c_int) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut d: libc::c_double = 0.;
    check_exact_args(
        nargs,
        b"strtonum\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    tmp = fixtype(POP_SCALAR());
    if (*tmp).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        d = (*tmp).sub.val.fltnum;
    } else if get_numbase((*tmp).sub.val.sp, (*tmp).sub.val.slen, use_lc_numeric != 0)
        != 10 as libc::c_int
    {
        d = nondec2awknum(
            (*tmp).sub.val.sp,
            (*tmp).sub.val.slen,
            0 as *mut *mut libc::c_char,
        );
    } else {
        d = (*force_number(tmp)).sub.val.fltnum;
    }
    DEREF(tmp);
    return make_number.unwrap()(d);
}
pub unsafe extern "C" fn nondec2awknum(
    mut str: *mut libc::c_char,
    mut len: size_t,
    mut endptr: *mut *mut libc::c_char,
) -> libc::c_double {
    let mut current_block: u64;
    let mut retval: libc::c_double = 0.0f64;
    let mut save: libc::c_char = 0;
    let mut val: libc::c_short = 0;
    let mut start: *mut libc::c_char = str;
    if len >= 2 as libc::c_int as libc::c_ulong && *str as libc::c_int == '0' as i32
        && (*str.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
            || *str.offset(1 as libc::c_int as isize) as libc::c_int == 'X' as i32)
    {
        if len <= 2 as libc::c_int as libc::c_ulong {
            if !endptr.is_null() {
                *endptr = start;
            }
            return 0.0f64;
        }
        str = str.offset(2 as libc::c_int as isize);
        len = (len as libc::c_ulong).wrapping_sub(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        loop {
            if !(len > 0 as libc::c_int as libc::c_ulong) {
                current_block = 10043043949733653460;
                break;
            }
            match *str as libc::c_int {
                48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                    val = (*str as libc::c_int - '0' as i32) as libc::c_short;
                }
                97 | 98 | 99 | 100 | 101 | 102 => {
                    val = (*str as libc::c_int - 'a' as i32 + 10 as libc::c_int)
                        as libc::c_short;
                }
                65 | 66 | 67 | 68 | 69 | 70 => {
                    val = (*str as libc::c_int - 'A' as i32 + 10 as libc::c_int)
                        as libc::c_short;
                }
                _ => {
                    if !endptr.is_null() {
                        *endptr = str;
                    }
                    current_block = 12439970321294333735;
                    break;
                }
            }
            retval = retval * 16 as libc::c_int as libc::c_double
                + val as libc::c_int as libc::c_double;
            len = len.wrapping_sub(1);
            len;
            str = str.offset(1);
            str;
        }
        match current_block {
            12439970321294333735 => {}
            _ => {
                if !endptr.is_null() {
                    *endptr = str;
                }
            }
        }
    } else {
        if len >= 1 as libc::c_int as libc::c_ulong && *str as libc::c_int == '0' as i32
        {
            let mut l: libc::c_int = 0;
            l = len as libc::c_int;
            loop {
                if !(l > 0 as libc::c_int) {
                    current_block = 1836292691772056875;
                    break;
                }
                if *(*__ctype_b_loc())
                    .offset(*str as libc::c_uchar as libc::c_int as isize) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
                {
                    if !endptr.is_null() {
                        *endptr = str;
                    }
                    current_block = 12439970321294333735;
                    break;
                } else if *str as libc::c_int == '8' as i32
                    || *str as libc::c_int == '9' as i32
                {
                    str = start;
                    current_block = 12877157945064627277;
                    break;
                } else {
                    retval = retval * 8 as libc::c_int as libc::c_double
                        + (*str as libc::c_int - '0' as i32) as libc::c_double;
                    str = str.offset(1);
                    str;
                    l -= 1;
                    l;
                }
            }
            match current_block {
                12439970321294333735 => {}
                12877157945064627277 => {}
                _ => {
                    if !endptr.is_null() {
                        *endptr = str;
                    }
                    current_block = 12439970321294333735;
                }
            }
        } else {
            current_block = 12877157945064627277;
        }
        match current_block {
            12439970321294333735 => {}
            _ => {
                save = *str.offset(len as isize);
                *str.offset(len as isize) = '\0' as i32 as libc::c_char;
                retval = strtod(str, endptr);
                *str.offset(len as isize) = save;
            }
        }
    }
    return retval;
}
unsafe extern "C" fn localecategory_from_argument(mut t: *mut NODE) -> libc::c_int {
    static mut cat_tab: [category_table; 7] = [
        {
            let mut init = category_table {
                val: 6 as libc::c_int,
                name: b"LC_ALL\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = category_table {
                val: 3 as libc::c_int,
                name: b"LC_COLLATE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = category_table {
                val: 0 as libc::c_int,
                name: b"LC_CTYPE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = category_table {
                val: 5 as libc::c_int,
                name: b"LC_MESSAGES\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = category_table {
                val: 4 as libc::c_int,
                name: b"LC_MONETARY\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = category_table {
                val: 1 as libc::c_int,
                name: b"LC_NUMERIC\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = category_table {
                val: 2 as libc::c_int,
                name: b"LC_TIME\0" as *const u8 as *const libc::c_char,
            };
            init
        },
    ];
    if !t.is_null() {
        let mut low: libc::c_int = 0;
        let mut high: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut mid: libc::c_int = 0;
        let mut category: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut lc_cat: libc::c_int = -(1 as libc::c_int);
        let mut save: libc::c_char = *((*t).sub.val.sp)
            .offset((*t).sub.val.slen as isize);
        *((*t).sub.val.sp)
            .offset((*t).sub.val.slen as isize) = '\0' as i32 as libc::c_char;
        category = (*t).sub.val.sp;
        low = 0 as libc::c_int;
        high = (::std::mem::size_of::<[category_table; 7]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<category_table>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        while low <= high {
            mid = (low + high) / 2 as libc::c_int;
            i = strcmp(category, cat_tab[mid as usize].name);
            if i < 0 as libc::c_int {
                high = mid - 1 as libc::c_int;
            } else if i > 0 as libc::c_int {
                low = mid + 1 as libc::c_int;
            } else {
                lc_cat = cat_tab[mid as usize].val;
                break;
            }
        }
        *((*t).sub.val.sp).offset((*t).sub.val.slen as isize) = save;
        if lc_cat == -(1 as libc::c_int) {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                3954 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"dcgettext: `%s' is not a valid locale category\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                category,
            );
        }
        return lc_cat;
    } else {
        return 5 as libc::c_int
    };
}
pub unsafe extern "C" fn do_dcgettext(mut nargs: libc::c_int) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut t2: *mut NODE = 0 as *mut NODE;
    let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut the_result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut reslen: size_t = 0;
    let mut lc_cat: libc::c_int = 0;
    let mut domain: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut save1: libc::c_char = '\0' as i32 as libc::c_char;
    let mut save2: libc::c_char = '\0' as i32 as libc::c_char;
    check_args_min_max(
        nargs,
        b"dcgettext\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        3 as libc::c_int,
    );
    if nargs == 3 as libc::c_int {
        tmp = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
        if do_flags as libc::c_uint
            & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                as libc::c_uint != 0
            && (*fixtype(tmp)).flags as libc::c_uint
                & STRING as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                3989 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: received non-string third argument\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"dcgettext\0" as *const u8 as *const libc::c_char,
            );
        }
        lc_cat = localecategory_from_argument(tmp);
        DEREF(tmp);
    } else {
        lc_cat = 5 as libc::c_int;
    }
    if nargs >= 2 as libc::c_int {
        t2 = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
        if do_flags as libc::c_uint
            & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                as libc::c_uint != 0
            && (*fixtype(t2)).flags as libc::c_uint
                & STRING as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                3998 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: received non-string second argument\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"dcgettext\0" as *const u8 as *const libc::c_char,
            );
        }
        domain = (*t2).sub.val.sp;
        str_terminate_f(t2, &mut save2);
    } else {
        domain = TEXTDOMAIN;
    }
    t1 = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
        && (*fixtype(t1)).flags as libc::c_uint & STRING as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            4020 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: received non-string first argument\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"dcgettext\0" as *const u8 as *const libc::c_char,
        );
    }
    string = (*t1).sub.val.sp;
    str_terminate_f(t1, &mut save1);
    the_result = dcgettext(domain, string, lc_cat);
    *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = save1;
    if !t2.is_null() {
        *((*t2).sub.val.sp).offset((*t2).sub.val.slen as isize) = save2;
        DEREF(t2);
    }
    reslen = strlen(the_result);
    DEREF(t1);
    return make_str_node(the_result, reslen, 0 as libc::c_int);
}
pub unsafe extern "C" fn do_dcngettext(mut nargs: libc::c_int) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut t2: *mut NODE = 0 as *mut NODE;
    let mut t3: *mut NODE = 0 as *mut NODE;
    let mut string1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut string2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut number: libc::c_ulong = 0;
    let mut d: libc::c_double = 0.;
    let mut the_result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut reslen: size_t = 0;
    let mut lc_cat: libc::c_int = 0;
    let mut domain: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut save: libc::c_char = '\0' as i32 as libc::c_char;
    let mut save1: libc::c_char = '\0' as i32 as libc::c_char;
    let mut save2: libc::c_char = '\0' as i32 as libc::c_char;
    let mut saved_end: bool = 0 as libc::c_int != 0;
    check_args_min_max(
        nargs,
        b"dcngettext\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int,
        5 as libc::c_int,
    );
    if nargs == 5 as libc::c_int {
        tmp = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
        if do_flags as libc::c_uint
            & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                as libc::c_uint != 0
            && (*fixtype(tmp)).flags as libc::c_uint
                & STRING as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                4062 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: received non-string fifth argument\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"dcngettext\0" as *const u8 as *const libc::c_char,
            );
        }
        lc_cat = localecategory_from_argument(tmp);
        DEREF(tmp);
    } else {
        lc_cat = 5 as libc::c_int;
    }
    t3 = 0 as *mut NODE;
    if nargs >= 4 as libc::c_int {
        t3 = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
        if do_flags as libc::c_uint
            & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                as libc::c_uint != 0
            && (*fixtype(t3)).flags as libc::c_uint
                & STRING as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                4072 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: received non-string fourth argument\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"dcngettext\0" as *const u8 as *const libc::c_char,
            );
        }
        domain = (*t3).sub.val.sp;
        save = *domain.offset((*t3).sub.val.slen as isize);
        *domain.offset((*t3).sub.val.slen as isize) = '\0' as i32 as libc::c_char;
        saved_end = 1 as libc::c_int != 0;
    } else {
        domain = TEXTDOMAIN;
    }
    t2 = force_number(POP_SCALAR());
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
        && (*fixtype(t2)).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            4096 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: received non-numeric third argument\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"dcngettext\0" as *const u8 as *const libc::c_char,
        );
    }
    d = (*t2).sub.val.fltnum;
    DEREF(t2);
    number = double_to_int(d) as libc::c_ulong;
    t2 = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
        && (*fixtype(t2)).flags as libc::c_uint & STRING as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            4103 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: received non-string second argument\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"dcngettext\0" as *const u8 as *const libc::c_char,
        );
    }
    string2 = (*t2).sub.val.sp;
    t1 = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
        && (*fixtype(t1)).flags as libc::c_uint & STRING as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            4107 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: received non-string first argument\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"dcngettext\0" as *const u8 as *const libc::c_char,
        );
    }
    string1 = (*t1).sub.val.sp;
    str_terminate_f(t1, &mut save1);
    str_terminate_f(t2, &mut save2);
    the_result = dcngettext(domain, string1, string2, number, lc_cat);
    reslen = strlen(the_result);
    *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = save1;
    *((*t2).sub.val.sp).offset((*t2).sub.val.slen as isize) = save2;
    if saved_end {
        *domain.offset((*t3).sub.val.slen as isize) = save;
    }
    if !t3.is_null() {
        DEREF(t3);
    }
    DEREF(t1);
    DEREF(t2);
    return make_str_node(the_result, reslen, 0 as libc::c_int);
}
pub unsafe extern "C" fn do_bindtextdomain(mut nargs: libc::c_int) -> *mut NODE {
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut t2: *mut NODE = 0 as *mut NODE;
    let mut directory: *const libc::c_char = 0 as *const libc::c_char;
    let mut domain: *const libc::c_char = 0 as *const libc::c_char;
    let mut the_result: *const libc::c_char = 0 as *const libc::c_char;
    check_args_min_max(
        nargs,
        b"bindtextdomain\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        2 as libc::c_int,
    );
    t2 = 0 as *mut NODE;
    t1 = t2;
    directory = 0 as *const libc::c_char;
    domain = TEXTDOMAIN;
    let mut save: libc::c_char = '\0' as i32 as libc::c_char;
    let mut save1: libc::c_char = '\0' as i32 as libc::c_char;
    if nargs == 2 as libc::c_int {
        t2 = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
        if do_flags as libc::c_uint
            & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                as libc::c_uint != 0
            && (*fixtype(t2)).flags as libc::c_uint
                & STRING as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                4165 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: received non-string second argument\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"bindtextdomain\0" as *const u8 as *const libc::c_char,
            );
        }
        domain = (*t2).sub.val.sp as *const libc::c_char;
        save = *((*t2).sub.val.sp).offset((*t2).sub.val.slen as isize);
        *((*t2).sub.val.sp)
            .offset((*t2).sub.val.slen as isize) = '\0' as i32 as libc::c_char;
    }
    t1 = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
        && (*fixtype(t1)).flags as libc::c_uint & STRING as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            4174 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: received non-string first argument\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"bindtextdomain\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*t1).sub.val.slen > 0 as libc::c_int as libc::c_ulong {
        directory = (*t1).sub.val.sp as *const libc::c_char;
        str_terminate_f(t1, &mut save1);
    }
    the_result = bindtextdomain(domain, directory);
    if !directory.is_null() {
        *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = save1;
    }
    DEREF(t1);
    if !t2.is_null() {
        *((*t2).sub.val.sp).offset((*t2).sub.val.slen as isize) = save;
        DEREF(t2);
    }
    if the_result.is_null() {
        the_result = b"\0" as *const u8 as *const libc::c_char;
    }
    return make_str_node(the_result, strlen(the_result), 0 as libc::c_int);
}
pub unsafe extern "C" fn do_typeof(mut nargs: libc::c_int) -> *mut NODE {
    let mut arg: *mut NODE = 0 as *mut NODE;
    let mut res: *const libc::c_char = 0 as *const libc::c_char;
    let mut deref: bool = 1 as libc::c_int != 0;
    let mut dbg: *mut NODE = 0 as *mut NODE;
    check_args_min_max(
        nargs,
        b"typeof\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        2 as libc::c_int,
    );
    if nargs == 2 as libc::c_int {
        dbg = POP_PARAM();
        if (*dbg).type_0 as libc::c_uint != Node_var_array as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                4277 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"typeof: second argument is not an array\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        ((*(*dbg).sub.nodep.l.lp).clear).unwrap()(dbg, 0 as *mut exp_node);
    } else {
        dbg = 0 as *mut NODE;
    }
    let fresh69 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    arg = (*fresh69).rptr;
    let mut current_block_55: u64;
    match (*arg).type_0 as libc::c_uint {
        5 => {
            res = b"array\0" as *const u8 as *const libc::c_char;
            deref = 0 as libc::c_int != 0;
            if !dbg.is_null() {
                assoc_set(
                    dbg,
                    make_str_node(
                        b"array_type\0" as *const u8 as *const libc::c_char,
                        10 as libc::c_int as size_t,
                        0 as libc::c_int,
                    ),
                    make_str_node(
                        (*(*arg).sub.nodep.l.lp).name,
                        strlen((*(*arg).sub.nodep.l.lp).name),
                        0 as libc::c_int,
                    ),
                );
                if arg == PROCINFO_node {
                    let mut i: libc::c_int = 0;
                    i = 0 as libc::c_int;
                    while i < BLOCK_MAX as libc::c_int {
                        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
                        let mut nl: size_t = strlen(nextfree[i as usize].name);
                        let mut hw: libc::c_long = nextfree[i as usize].highwater;
                        let mut active: libc::c_long = 0;
                        active = hw;
                        let mut ip: *mut block_item = 0 as *mut block_item;
                        ip = nextfree[i as usize].freep;
                        while !ip.is_null() {
                            active -= 1;
                            active;
                            ip = (*ip).freep;
                        }
                        let mut l: size_t = nl
                            .wrapping_add(
                                ::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
                            );
                        p = emalloc_real(
                            l.wrapping_add(1 as libc::c_int as libc::c_ulong),
                            b"do_typeof\0" as *const u8 as *const libc::c_char,
                            b"p\0" as *const u8 as *const libc::c_char,
                            b"builtin.c\0" as *const u8 as *const libc::c_char,
                            4320 as libc::c_int,
                        ) as *mut libc::c_char;
                        sprintf(
                            p,
                            b"%s_highwater\0" as *const u8 as *const libc::c_char,
                            nextfree[i as usize].name,
                        );
                        assoc_set(
                            dbg,
                            make_str_node(p, l, 2 as libc::c_int),
                            make_number.unwrap()(hw as libc::c_double),
                        );
                        let mut l_0: size_t = nl
                            .wrapping_add(
                                ::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong,
                            );
                        p = emalloc_real(
                            l_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                            b"do_typeof\0" as *const u8 as *const libc::c_char,
                            b"p\0" as *const u8 as *const libc::c_char,
                            b"builtin.c\0" as *const u8 as *const libc::c_char,
                            4321 as libc::c_int,
                        ) as *mut libc::c_char;
                        sprintf(
                            p,
                            b"%s_active\0" as *const u8 as *const libc::c_char,
                            nextfree[i as usize].name,
                        );
                        assoc_set(
                            dbg,
                            make_str_node(p, l_0, 2 as libc::c_int),
                            make_number.unwrap()(active as libc::c_double),
                        );
                        i += 1;
                        i;
                    }
                }
            }
            current_block_55 = 6560072651652764009;
        }
        4 => {
            arg = (*arg).sub.nodep.l.lptr;
            current_block_55 = 6188017276694156487;
        }
        1 => {
            current_block_55 = 6188017276694156487;
        }
        6 | 7 => {
            res = b"untyped\0" as *const u8 as *const libc::c_char;
            deref = 0 as libc::c_int != 0;
            current_block_55 = 6560072651652764009;
        }
        12 => {
            if (*(*arg).sub.nodep.l.lptr).type_0 as libc::c_uint
                == Node_var as libc::c_int as libc::c_uint
                && ((*(*arg).sub.nodep.l.lptr).sub.nodep.l.lptr == Nnull_string
                    || (*(*(*arg).sub.nodep.l.lptr).sub.nodep.l.lptr).flags
                        as libc::c_uint & NULL_FIELD as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint)
            {
                res = b"unassigned\0" as *const u8 as *const libc::c_char;
            } else {
                res = b"untyped\0" as *const u8 as *const libc::c_char;
            }
            deref = 0 as libc::c_int != 0;
            current_block_55 = 6560072651652764009;
        }
        _ => {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"builtin.c\0" as *const u8 as *const libc::c_char,
                4405 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"typeof: unknown argument type `%s'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                nodetype2str((*arg).type_0),
            );
            current_block_55 = 6560072651652764009;
        }
    }
    match current_block_55 {
        6188017276694156487 => {
            let mut current_block_42: u64;
            match (*fixtype(arg)).flags as libc::c_uint
                & (STRING as libc::c_int | NUMBER as libc::c_int
                    | USER_INPUT as libc::c_int | REGEX as libc::c_int
                    | BOOLVAL as libc::c_int) as libc::c_uint
            {
                80 => {
                    res = b"number|bool\0" as *const u8 as *const libc::c_char;
                    current_block_42 = 9441801433784995173;
                }
                16 => {
                    res = b"number\0" as *const u8 as *const libc::c_char;
                    current_block_42 = 9441801433784995173;
                }
                48 => {
                    res = b"strnum\0" as *const u8 as *const libc::c_char;
                    current_block_42 = 9441801433784995173;
                }
                524288 => {
                    res = b"regexp\0" as *const u8 as *const libc::c_char;
                    current_block_42 = 9441801433784995173;
                }
                2 => {
                    res = b"string\0" as *const u8 as *const libc::c_char;
                    current_block_42 = 9601429597124571314;
                }
                18 => {
                    current_block_42 = 9601429597124571314;
                }
                _ => {
                    current_block_42 = 13184219624369782253;
                }
            }
            match current_block_42 {
                9601429597124571314 => {
                    if arg == Nnull_string
                        || (*arg).flags as libc::c_uint
                            & NULL_FIELD as libc::c_int as libc::c_uint
                            != 0 as libc::c_int as libc::c_uint
                    {
                        res = b"unassigned\0" as *const u8 as *const libc::c_char;
                        current_block_42 = 9441801433784995173;
                    } else {
                        current_block_42 = 13184219624369782253;
                    }
                }
                _ => {}
            }
            match current_block_42 {
                13184219624369782253 => {
                    if res.is_null() {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"builtin.c\0" as *const u8 as *const libc::c_char,
                            4367 as libc::c_int,
                        );
                        (Some(
                            (Some(
                                r_warning
                                    as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                            ))
                                .unwrap(),
                        ))
                            .unwrap()(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"typeof detected invalid flags combination `%s'; please file a bug report\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            flags2str((*arg).flags as libc::c_int),
                        );
                        res = b"unknown\0" as *const u8 as *const libc::c_char;
                    }
                }
                _ => {}
            }
            if !dbg.is_null() {
                let mut s: *const libc::c_char = flags2str((*arg).flags as libc::c_int);
                assoc_set(
                    dbg,
                    make_str_node(
                        b"flags\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int as size_t,
                        0 as libc::c_int,
                    ),
                    make_str_node(s, strlen(s), 0 as libc::c_int),
                );
            }
        }
        _ => {}
    }
    if deref {
        DEREF(arg);
    }
    return make_str_node(res, strlen(res), 0 as libc::c_int);
}
unsafe extern "C" fn mbc_byte_count(
    mut ptr: *const libc::c_char,
    mut numchars: size_t,
) -> size_t {
    let mut cur_state: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    let mut sum: size_t = 0 as libc::c_int as size_t;
    let mut mb_len: libc::c_int = 0;
    memset(
        &mut cur_state as *mut mbstate_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    mb_len = mbrlen(
        ptr,
        numchars.wrapping_mul(gawk_mb_cur_max as libc::c_ulong),
        &mut cur_state,
    ) as libc::c_int;
    if mb_len <= 0 as libc::c_int {
        return numchars;
    }
    while numchars > 0 as libc::c_int as libc::c_ulong {
        mb_len = mbrlen(
            ptr,
            numchars.wrapping_mul(gawk_mb_cur_max as libc::c_ulong),
            &mut cur_state,
        ) as libc::c_int;
        if mb_len <= 0 as libc::c_int {
            break;
        }
        sum = (sum as libc::c_ulong).wrapping_add(mb_len as libc::c_ulong) as size_t
            as size_t;
        ptr = ptr.offset(mb_len as isize);
        numchars = numchars.wrapping_sub(1);
        numchars;
    }
    return sum;
}
unsafe extern "C" fn mbc_char_count(
    mut ptr: *const libc::c_char,
    mut numbytes: size_t,
) -> size_t {
    let mut cur_state: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    let mut sum: size_t = 0 as libc::c_int as size_t;
    let mut mb_len: libc::c_int = 0;
    if gawk_mb_cur_max == 1 as libc::c_int {
        return numbytes;
    }
    memset(
        &mut cur_state as *mut mbstate_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    mb_len = mbrlen(ptr, numbytes, &mut cur_state) as libc::c_int;
    if mb_len <= 0 as libc::c_int {
        return numbytes;
    }
    while numbytes > 0 as libc::c_int as libc::c_ulong {
        mb_len = mbrlen(ptr, numbytes, &mut cur_state) as libc::c_int;
        if mb_len <= 0 as libc::c_int {
            break;
        }
        sum = sum.wrapping_add(1);
        sum;
        ptr = ptr.offset(mb_len as isize);
        numbytes = (numbytes as libc::c_ulong).wrapping_sub(mb_len as libc::c_ulong)
            as size_t as size_t;
    }
    return sum;
}
pub unsafe extern "C" fn sanitize_exit_status(mut status: libc::c_int) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    if status & 0x7f as libc::c_int == 0 as libc::c_int {
        ret = (status & 0xff00 as libc::c_int) >> 8 as libc::c_int;
    } else if ((status & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar
        as libc::c_int >> 1 as libc::c_int > 0 as libc::c_int
    {
        let mut coredumped: bool = 0 as libc::c_int != 0;
        coredumped = status & 0x80 as libc::c_int != 0;
        ret = (status & 0x7f as libc::c_int)
            + (if coredumped as libc::c_int != 0 {
                512 as libc::c_int
            } else {
                256 as libc::c_int
            });
    } else {
        ret = 0 as libc::c_int;
    }
    return ret;
}
pub unsafe extern "C" fn out_of_range(mut n: *mut NODE) -> bool {
    return ((*n).sub.val.fltnum).is_nan() as i32 != 0
        || if ((*n).sub.val.fltnum).is_infinite() {
            if ((*n).sub.val.fltnum).is_sign_positive() { 1 } else { -1 }
        } else {
            0
        } != 0;
}
pub unsafe extern "C" fn format_nan_inf(
    mut n: *mut NODE,
    mut format: libc::c_char,
) -> *mut libc::c_char {
    static mut buf: [libc::c_char; 100] = [0; 100];
    let mut val: libc::c_double = (*n).sub.val.fltnum;
    if val.is_nan() as i32 != 0 {
        strcpy(
            buf.as_mut_ptr(),
            if val.is_sign_negative() as libc::c_int != 0 as libc::c_int {
                b"-nan\0" as *const u8 as *const libc::c_char
            } else {
                b"+nan\0" as *const u8 as *const libc::c_char
            },
        );
    } else if if val.is_infinite() {
        if val.is_sign_positive() { 1 } else { -1 }
    } else {
        0
    } != 0
    {
        strcpy(
            buf.as_mut_ptr(),
            if val < 0 as libc::c_int as libc::c_double {
                b"-inf\0" as *const u8 as *const libc::c_char
            } else {
                b"+inf\0" as *const u8 as *const libc::c_char
            },
        );
    } else {
        return 0 as *mut libc::c_char
    }
    if *(*__ctype_b_loc()).offset(format as libc::c_int as isize) as libc::c_int
        & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while buf[i as usize] as libc::c_int != '\0' as i32 {
            buf[i
                as usize] = ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = buf[i as usize] as libc::c_int;
                        __res = if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = toupper(buf[i as usize] as libc::c_int);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc())
                        .offset(buf[i as usize] as libc::c_int as isize);
                }
                __res
            }) as libc::c_char;
            i += 1;
            i;
        }
    }
    return buf.as_mut_ptr();
}
pub unsafe extern "C" fn check_symtab_functab(
    mut dest: *mut NODE,
    mut fname: *const libc::c_char,
    mut msg_0: *const libc::c_char,
) {
    if dest == symbol_table {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            4566 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(msg_0, fname, b"SYMTAB\0" as *const u8 as *const libc::c_char);
    } else if dest == func_table {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"builtin.c\0" as *const u8 as *const libc::c_char,
            4568 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(msg_0, fname, b"FUNCTAB\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn do_mkbool(mut nargs: libc::c_int) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut result: bool = false;
    tmp = POP_SCALAR();
    result = boolval(tmp);
    DEREF(tmp);
    return make_bool_node(result);
}
unsafe extern "C" fn reverse(mut str: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tmp: libc::c_char = 0;
    i = 0 as libc::c_int;
    j = (strlen(str)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    while j > i {
        tmp = *str.offset(i as isize);
        *str.offset(i as isize) = *str.offset(j as isize);
        *str.offset(j as isize) = tmp;
        i += 1;
        i;
        j -= 1;
        j;
    }
}
unsafe extern "C" fn add_thousands(
    mut original: *const libc::c_char,
    mut loc_0: *mut lconv,
) -> *const libc::c_char {
    let mut orig_len: size_t = strlen(original);
    let mut new_len: size_t = orig_len
        .wrapping_add(orig_len.wrapping_mul(strlen((*loc_0).thousands_sep)))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut newbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut decimal_point: libc::c_char = '\0' as i32 as libc::c_char;
    let mut dec: *const libc::c_char = 0 as *const libc::c_char;
    let mut src: *const libc::c_char = 0 as *const libc::c_char;
    let mut dest: *mut libc::c_char = 0 as *mut libc::c_char;
    newbuf = emalloc_real(
        new_len,
        b"add_thousands\0" as *const u8 as *const libc::c_char,
        b"newbuf\0" as *const u8 as *const libc::c_char,
        b"builtin.c\0" as *const u8 as *const libc::c_char,
        4621 as libc::c_int,
    ) as *mut libc::c_char;
    memset(newbuf as *mut libc::c_void, '\0' as i32, new_len);
    src = original
        .offset(strlen(original) as isize)
        .offset(-(1 as libc::c_int as isize));
    dest = newbuf;
    if *((*loc_0).decimal_point).offset(0 as libc::c_int as isize) as libc::c_int
        != '\0' as i32
    {
        decimal_point = *((*loc_0).decimal_point).offset(0 as libc::c_int as isize);
        dec = strchr(original, decimal_point as libc::c_int);
        if !dec.is_null() {
            while src >= dec {
                let fresh70 = src;
                src = src.offset(-1);
                let fresh71 = dest;
                dest = dest.offset(1);
                *fresh71 = *fresh70;
            }
        }
    }
    let mut ii: libc::c_int = 0 as libc::c_int;
    let mut jj: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh72 = src;
        src = src.offset(-1);
        let fresh73 = dest;
        dest = dest.offset(1);
        *fresh73 = *fresh72;
        if *((*loc_0).grouping).offset(ii as isize) as libc::c_int != 0
            && {
                jj += 1;
                jj == *((*loc_0).grouping).offset(ii as isize) as libc::c_int
            }
        {
            if src >= original {
                let mut ts: *const libc::c_char = (*loc_0).thousands_sep;
                let mut k: libc::c_int = 0;
                k = (strlen(ts)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as libc::c_int;
                while k >= 0 as libc::c_int {
                    let fresh74 = dest;
                    dest = dest.offset(1);
                    *fresh74 = *ts.offset(k as isize);
                    k -= 1;
                    k;
                }
            }
            if *((*loc_0).grouping).offset((ii + 1 as libc::c_int) as isize)
                as libc::c_int == 0 as libc::c_int
            {
                jj = 0 as libc::c_int;
            } else if *((*loc_0).grouping).offset((ii + 1 as libc::c_int) as isize)
                as libc::c_int == 127 as libc::c_int
            {
                while src >= original {
                    let fresh75 = src;
                    src = src.offset(-1);
                    let fresh76 = dest;
                    dest = dest.offset(1);
                    *fresh76 = *fresh75;
                }
                break;
            } else {
                ii += 1;
                ii;
                jj = 0 as libc::c_int;
            }
        }
        if !(src >= original) {
            break;
        }
    }
    let fresh77 = dest;
    dest = dest.offset(1);
    *fresh77 = '\0' as i32 as libc::c_char;
    reverse(newbuf);
    return newbuf;
}
unsafe extern "C" fn run_static_initializers() {
    time_t_min = (if !((0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t) {
        !(0 as libc::c_int as uintmax_t)
            << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    } else {
        0 as libc::c_int as libc::c_ulong
    }) as time_t;
    time_t_max = !(0 as libc::c_int as time_t)
        - (if !((0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t) {
            !(0 as libc::c_int as uintmax_t)
                << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        } else {
            0 as libc::c_int as libc::c_ulong
        }) as time_t;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
