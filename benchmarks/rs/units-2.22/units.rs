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
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn raise(__sig: libc::c_int) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn ctime(__timer: *const time_t) -> *mut libc::c_char;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn mbsrtowcs(
        __dst: *mut wchar_t,
        __src: *mut *const libc::c_char,
        __len: size_t,
        __ps: *mut mbstate_t,
    ) -> size_t;
    fn wcswidth(__s: *const wchar_t, __n: size_t) -> libc::c_int;
    fn nl_langinfo(__item: nl_item) -> *mut libc::c_char;
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
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn readline(_: *const libc::c_char) -> *mut libc::c_char;
    fn rl_initialize() -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    static mut rl_completion_entry_function: Option::<rl_compentry_func_t>;
    static mut rl_basic_word_break_characters: *const libc::c_char;
    static mut rl_completion_suppress_append: libc::c_int;
    fn add_history(_: *const libc::c_char);
    fn read_history(_: *const libc::c_char) -> libc::c_int;
    fn write_history(_: *const libc::c_char) -> libc::c_int;
    fn append_history(_: libc::c_int, _: *const libc::c_char) -> libc::c_int;
    fn history_truncate_file(_: *const libc::c_char, _: libc::c_int) -> libc::c_int;
    static mut history_base: libc::c_int;
    static mut history_length: libc::c_int;
    static mut history_max_entries: libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        argc: libc::c_int,
        argv: *const *mut libc::c_char,
        shortopts: *const libc::c_char,
        longopts: *const option,
        longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn log2(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn round(_: libc::c_double) -> libc::c_double;
    fn modf(_: libc::c_double, _: *mut libc::c_double) -> libc::c_double;
    fn log10(_: libc::c_double) -> libc::c_double;
    fn __errno_location() -> *mut libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn abs(_: libc::c_int) -> libc::c_int;
    fn parseunit(
        output: *mut unittype,
        input: *const libc::c_char,
        errstr: *mut *mut libc::c_char,
        errloc: *mut libc::c_int,
    ) -> libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
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
pub type __syscall_ulong_t = libc::c_ulong;
pub type time_t = __time_t;
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
    pub st_atime: __time_t,
    pub st_atimensec: __syscall_ulong_t,
    pub st_mtime: __time_t,
    pub st_mtimensec: __syscall_ulong_t,
    pub st_ctime: __time_t,
    pub st_ctimensec: __syscall_ulong_t,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
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
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
pub type wchar_t = libc::c_int;
pub type mbstate_t = __mbstate_t;
pub type nl_item = libc::c_int;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const _NL_NUM: C2RustUnnamed_1 = 786449;
pub const _NL_NUM_LC_IDENTIFICATION: C2RustUnnamed_1 = 786448;
pub const _NL_IDENTIFICATION_CODESET: C2RustUnnamed_1 = 786447;
pub const _NL_IDENTIFICATION_CATEGORY: C2RustUnnamed_1 = 786446;
pub const _NL_IDENTIFICATION_DATE: C2RustUnnamed_1 = 786445;
pub const _NL_IDENTIFICATION_REVISION: C2RustUnnamed_1 = 786444;
pub const _NL_IDENTIFICATION_ABBREVIATION: C2RustUnnamed_1 = 786443;
pub const _NL_IDENTIFICATION_APPLICATION: C2RustUnnamed_1 = 786442;
pub const _NL_IDENTIFICATION_AUDIENCE: C2RustUnnamed_1 = 786441;
pub const _NL_IDENTIFICATION_TERRITORY: C2RustUnnamed_1 = 786440;
pub const _NL_IDENTIFICATION_LANGUAGE: C2RustUnnamed_1 = 786439;
pub const _NL_IDENTIFICATION_FAX: C2RustUnnamed_1 = 786438;
pub const _NL_IDENTIFICATION_TEL: C2RustUnnamed_1 = 786437;
pub const _NL_IDENTIFICATION_EMAIL: C2RustUnnamed_1 = 786436;
pub const _NL_IDENTIFICATION_CONTACT: C2RustUnnamed_1 = 786435;
pub const _NL_IDENTIFICATION_ADDRESS: C2RustUnnamed_1 = 786434;
pub const _NL_IDENTIFICATION_SOURCE: C2RustUnnamed_1 = 786433;
pub const _NL_IDENTIFICATION_TITLE: C2RustUnnamed_1 = 786432;
pub const _NL_NUM_LC_MEASUREMENT: C2RustUnnamed_1 = 720898;
pub const _NL_MEASUREMENT_CODESET: C2RustUnnamed_1 = 720897;
pub const _NL_MEASUREMENT_MEASUREMENT: C2RustUnnamed_1 = 720896;
pub const _NL_NUM_LC_TELEPHONE: C2RustUnnamed_1 = 655365;
pub const _NL_TELEPHONE_CODESET: C2RustUnnamed_1 = 655364;
pub const _NL_TELEPHONE_INT_PREFIX: C2RustUnnamed_1 = 655363;
pub const _NL_TELEPHONE_INT_SELECT: C2RustUnnamed_1 = 655362;
pub const _NL_TELEPHONE_TEL_DOM_FMT: C2RustUnnamed_1 = 655361;
pub const _NL_TELEPHONE_TEL_INT_FMT: C2RustUnnamed_1 = 655360;
pub const _NL_NUM_LC_ADDRESS: C2RustUnnamed_1 = 589837;
pub const _NL_ADDRESS_CODESET: C2RustUnnamed_1 = 589836;
pub const _NL_ADDRESS_LANG_LIB: C2RustUnnamed_1 = 589835;
pub const _NL_ADDRESS_LANG_TERM: C2RustUnnamed_1 = 589834;
pub const _NL_ADDRESS_LANG_AB: C2RustUnnamed_1 = 589833;
pub const _NL_ADDRESS_LANG_NAME: C2RustUnnamed_1 = 589832;
pub const _NL_ADDRESS_COUNTRY_ISBN: C2RustUnnamed_1 = 589831;
pub const _NL_ADDRESS_COUNTRY_NUM: C2RustUnnamed_1 = 589830;
pub const _NL_ADDRESS_COUNTRY_CAR: C2RustUnnamed_1 = 589829;
pub const _NL_ADDRESS_COUNTRY_AB3: C2RustUnnamed_1 = 589828;
pub const _NL_ADDRESS_COUNTRY_AB2: C2RustUnnamed_1 = 589827;
pub const _NL_ADDRESS_COUNTRY_POST: C2RustUnnamed_1 = 589826;
pub const _NL_ADDRESS_COUNTRY_NAME: C2RustUnnamed_1 = 589825;
pub const _NL_ADDRESS_POSTAL_FMT: C2RustUnnamed_1 = 589824;
pub const _NL_NUM_LC_NAME: C2RustUnnamed_1 = 524295;
pub const _NL_NAME_CODESET: C2RustUnnamed_1 = 524294;
pub const _NL_NAME_NAME_MS: C2RustUnnamed_1 = 524293;
pub const _NL_NAME_NAME_MISS: C2RustUnnamed_1 = 524292;
pub const _NL_NAME_NAME_MRS: C2RustUnnamed_1 = 524291;
pub const _NL_NAME_NAME_MR: C2RustUnnamed_1 = 524290;
pub const _NL_NAME_NAME_GEN: C2RustUnnamed_1 = 524289;
pub const _NL_NAME_NAME_FMT: C2RustUnnamed_1 = 524288;
pub const _NL_NUM_LC_PAPER: C2RustUnnamed_1 = 458755;
pub const _NL_PAPER_CODESET: C2RustUnnamed_1 = 458754;
pub const _NL_PAPER_WIDTH: C2RustUnnamed_1 = 458753;
pub const _NL_PAPER_HEIGHT: C2RustUnnamed_1 = 458752;
pub const _NL_NUM_LC_MESSAGES: C2RustUnnamed_1 = 327685;
pub const _NL_MESSAGES_CODESET: C2RustUnnamed_1 = 327684;
pub const __NOSTR: C2RustUnnamed_1 = 327683;
pub const __YESSTR: C2RustUnnamed_1 = 327682;
pub const __NOEXPR: C2RustUnnamed_1 = 327681;
pub const __YESEXPR: C2RustUnnamed_1 = 327680;
pub const _NL_NUM_LC_NUMERIC: C2RustUnnamed_1 = 65542;
pub const _NL_NUMERIC_CODESET: C2RustUnnamed_1 = 65541;
pub const _NL_NUMERIC_THOUSANDS_SEP_WC: C2RustUnnamed_1 = 65540;
pub const _NL_NUMERIC_DECIMAL_POINT_WC: C2RustUnnamed_1 = 65539;
pub const __GROUPING: C2RustUnnamed_1 = 65538;
pub const THOUSEP: C2RustUnnamed_1 = 65537;
pub const __THOUSANDS_SEP: C2RustUnnamed_1 = 65537;
pub const RADIXCHAR: C2RustUnnamed_1 = 65536;
pub const __DECIMAL_POINT: C2RustUnnamed_1 = 65536;
pub const _NL_NUM_LC_MONETARY: C2RustUnnamed_1 = 262190;
pub const _NL_MONETARY_CODESET: C2RustUnnamed_1 = 262189;
pub const _NL_MONETARY_THOUSANDS_SEP_WC: C2RustUnnamed_1 = 262188;
pub const _NL_MONETARY_DECIMAL_POINT_WC: C2RustUnnamed_1 = 262187;
pub const _NL_MONETARY_CONVERSION_RATE: C2RustUnnamed_1 = 262186;
pub const _NL_MONETARY_DUO_VALID_TO: C2RustUnnamed_1 = 262185;
pub const _NL_MONETARY_DUO_VALID_FROM: C2RustUnnamed_1 = 262184;
pub const _NL_MONETARY_UNO_VALID_TO: C2RustUnnamed_1 = 262183;
pub const _NL_MONETARY_UNO_VALID_FROM: C2RustUnnamed_1 = 262182;
pub const _NL_MONETARY_DUO_INT_N_SIGN_POSN: C2RustUnnamed_1 = 262181;
pub const _NL_MONETARY_DUO_INT_P_SIGN_POSN: C2RustUnnamed_1 = 262180;
pub const _NL_MONETARY_DUO_N_SIGN_POSN: C2RustUnnamed_1 = 262179;
pub const _NL_MONETARY_DUO_P_SIGN_POSN: C2RustUnnamed_1 = 262178;
pub const _NL_MONETARY_DUO_INT_N_SEP_BY_SPACE: C2RustUnnamed_1 = 262177;
pub const _NL_MONETARY_DUO_INT_N_CS_PRECEDES: C2RustUnnamed_1 = 262176;
pub const _NL_MONETARY_DUO_INT_P_SEP_BY_SPACE: C2RustUnnamed_1 = 262175;
pub const _NL_MONETARY_DUO_INT_P_CS_PRECEDES: C2RustUnnamed_1 = 262174;
pub const _NL_MONETARY_DUO_N_SEP_BY_SPACE: C2RustUnnamed_1 = 262173;
pub const _NL_MONETARY_DUO_N_CS_PRECEDES: C2RustUnnamed_1 = 262172;
pub const _NL_MONETARY_DUO_P_SEP_BY_SPACE: C2RustUnnamed_1 = 262171;
pub const _NL_MONETARY_DUO_P_CS_PRECEDES: C2RustUnnamed_1 = 262170;
pub const _NL_MONETARY_DUO_FRAC_DIGITS: C2RustUnnamed_1 = 262169;
pub const _NL_MONETARY_DUO_INT_FRAC_DIGITS: C2RustUnnamed_1 = 262168;
pub const _NL_MONETARY_DUO_CURRENCY_SYMBOL: C2RustUnnamed_1 = 262167;
pub const _NL_MONETARY_DUO_INT_CURR_SYMBOL: C2RustUnnamed_1 = 262166;
pub const __INT_N_SIGN_POSN: C2RustUnnamed_1 = 262165;
pub const __INT_P_SIGN_POSN: C2RustUnnamed_1 = 262164;
pub const __INT_N_SEP_BY_SPACE: C2RustUnnamed_1 = 262163;
pub const __INT_N_CS_PRECEDES: C2RustUnnamed_1 = 262162;
pub const __INT_P_SEP_BY_SPACE: C2RustUnnamed_1 = 262161;
pub const __INT_P_CS_PRECEDES: C2RustUnnamed_1 = 262160;
pub const _NL_MONETARY_CRNCYSTR: C2RustUnnamed_1 = 262159;
pub const __N_SIGN_POSN: C2RustUnnamed_1 = 262158;
pub const __P_SIGN_POSN: C2RustUnnamed_1 = 262157;
pub const __N_SEP_BY_SPACE: C2RustUnnamed_1 = 262156;
pub const __N_CS_PRECEDES: C2RustUnnamed_1 = 262155;
pub const __P_SEP_BY_SPACE: C2RustUnnamed_1 = 262154;
pub const __P_CS_PRECEDES: C2RustUnnamed_1 = 262153;
pub const __FRAC_DIGITS: C2RustUnnamed_1 = 262152;
pub const __INT_FRAC_DIGITS: C2RustUnnamed_1 = 262151;
pub const __NEGATIVE_SIGN: C2RustUnnamed_1 = 262150;
pub const __POSITIVE_SIGN: C2RustUnnamed_1 = 262149;
pub const __MON_GROUPING: C2RustUnnamed_1 = 262148;
pub const __MON_THOUSANDS_SEP: C2RustUnnamed_1 = 262147;
pub const __MON_DECIMAL_POINT: C2RustUnnamed_1 = 262146;
pub const __CURRENCY_SYMBOL: C2RustUnnamed_1 = 262145;
pub const __INT_CURR_SYMBOL: C2RustUnnamed_1 = 262144;
pub const _NL_NUM_LC_CTYPE: C2RustUnnamed_1 = 86;
pub const _NL_CTYPE_EXTRA_MAP_14: C2RustUnnamed_1 = 85;
pub const _NL_CTYPE_EXTRA_MAP_13: C2RustUnnamed_1 = 84;
pub const _NL_CTYPE_EXTRA_MAP_12: C2RustUnnamed_1 = 83;
pub const _NL_CTYPE_EXTRA_MAP_11: C2RustUnnamed_1 = 82;
pub const _NL_CTYPE_EXTRA_MAP_10: C2RustUnnamed_1 = 81;
pub const _NL_CTYPE_EXTRA_MAP_9: C2RustUnnamed_1 = 80;
pub const _NL_CTYPE_EXTRA_MAP_8: C2RustUnnamed_1 = 79;
pub const _NL_CTYPE_EXTRA_MAP_7: C2RustUnnamed_1 = 78;
pub const _NL_CTYPE_EXTRA_MAP_6: C2RustUnnamed_1 = 77;
pub const _NL_CTYPE_EXTRA_MAP_5: C2RustUnnamed_1 = 76;
pub const _NL_CTYPE_EXTRA_MAP_4: C2RustUnnamed_1 = 75;
pub const _NL_CTYPE_EXTRA_MAP_3: C2RustUnnamed_1 = 74;
pub const _NL_CTYPE_EXTRA_MAP_2: C2RustUnnamed_1 = 73;
pub const _NL_CTYPE_EXTRA_MAP_1: C2RustUnnamed_1 = 72;
pub const _NL_CTYPE_NONASCII_CASE: C2RustUnnamed_1 = 71;
pub const _NL_CTYPE_MAP_TO_NONASCII: C2RustUnnamed_1 = 70;
pub const _NL_CTYPE_TRANSLIT_IGNORE: C2RustUnnamed_1 = 69;
pub const _NL_CTYPE_TRANSLIT_IGNORE_LEN: C2RustUnnamed_1 = 68;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING: C2RustUnnamed_1 = 67;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN: C2RustUnnamed_1 = 66;
pub const _NL_CTYPE_TRANSLIT_TO_TBL: C2RustUnnamed_1 = 65;
pub const _NL_CTYPE_TRANSLIT_TO_IDX: C2RustUnnamed_1 = 64;
pub const _NL_CTYPE_TRANSLIT_FROM_TBL: C2RustUnnamed_1 = 63;
pub const _NL_CTYPE_TRANSLIT_FROM_IDX: C2RustUnnamed_1 = 62;
pub const _NL_CTYPE_TRANSLIT_TAB_SIZE: C2RustUnnamed_1 = 61;
pub const _NL_CTYPE_OUTDIGIT9_WC: C2RustUnnamed_1 = 60;
pub const _NL_CTYPE_OUTDIGIT8_WC: C2RustUnnamed_1 = 59;
pub const _NL_CTYPE_OUTDIGIT7_WC: C2RustUnnamed_1 = 58;
pub const _NL_CTYPE_OUTDIGIT6_WC: C2RustUnnamed_1 = 57;
pub const _NL_CTYPE_OUTDIGIT5_WC: C2RustUnnamed_1 = 56;
pub const _NL_CTYPE_OUTDIGIT4_WC: C2RustUnnamed_1 = 55;
pub const _NL_CTYPE_OUTDIGIT3_WC: C2RustUnnamed_1 = 54;
pub const _NL_CTYPE_OUTDIGIT2_WC: C2RustUnnamed_1 = 53;
pub const _NL_CTYPE_OUTDIGIT1_WC: C2RustUnnamed_1 = 52;
pub const _NL_CTYPE_OUTDIGIT0_WC: C2RustUnnamed_1 = 51;
pub const _NL_CTYPE_OUTDIGIT9_MB: C2RustUnnamed_1 = 50;
pub const _NL_CTYPE_OUTDIGIT8_MB: C2RustUnnamed_1 = 49;
pub const _NL_CTYPE_OUTDIGIT7_MB: C2RustUnnamed_1 = 48;
pub const _NL_CTYPE_OUTDIGIT6_MB: C2RustUnnamed_1 = 47;
pub const _NL_CTYPE_OUTDIGIT5_MB: C2RustUnnamed_1 = 46;
pub const _NL_CTYPE_OUTDIGIT4_MB: C2RustUnnamed_1 = 45;
pub const _NL_CTYPE_OUTDIGIT3_MB: C2RustUnnamed_1 = 44;
pub const _NL_CTYPE_OUTDIGIT2_MB: C2RustUnnamed_1 = 43;
pub const _NL_CTYPE_OUTDIGIT1_MB: C2RustUnnamed_1 = 42;
pub const _NL_CTYPE_OUTDIGIT0_MB: C2RustUnnamed_1 = 41;
pub const _NL_CTYPE_INDIGITS9_WC: C2RustUnnamed_1 = 40;
pub const _NL_CTYPE_INDIGITS8_WC: C2RustUnnamed_1 = 39;
pub const _NL_CTYPE_INDIGITS7_WC: C2RustUnnamed_1 = 38;
pub const _NL_CTYPE_INDIGITS6_WC: C2RustUnnamed_1 = 37;
pub const _NL_CTYPE_INDIGITS5_WC: C2RustUnnamed_1 = 36;
pub const _NL_CTYPE_INDIGITS4_WC: C2RustUnnamed_1 = 35;
pub const _NL_CTYPE_INDIGITS3_WC: C2RustUnnamed_1 = 34;
pub const _NL_CTYPE_INDIGITS2_WC: C2RustUnnamed_1 = 33;
pub const _NL_CTYPE_INDIGITS1_WC: C2RustUnnamed_1 = 32;
pub const _NL_CTYPE_INDIGITS0_WC: C2RustUnnamed_1 = 31;
pub const _NL_CTYPE_INDIGITS_WC_LEN: C2RustUnnamed_1 = 30;
pub const _NL_CTYPE_INDIGITS9_MB: C2RustUnnamed_1 = 29;
pub const _NL_CTYPE_INDIGITS8_MB: C2RustUnnamed_1 = 28;
pub const _NL_CTYPE_INDIGITS7_MB: C2RustUnnamed_1 = 27;
pub const _NL_CTYPE_INDIGITS6_MB: C2RustUnnamed_1 = 26;
pub const _NL_CTYPE_INDIGITS5_MB: C2RustUnnamed_1 = 25;
pub const _NL_CTYPE_INDIGITS4_MB: C2RustUnnamed_1 = 24;
pub const _NL_CTYPE_INDIGITS3_MB: C2RustUnnamed_1 = 23;
pub const _NL_CTYPE_INDIGITS2_MB: C2RustUnnamed_1 = 22;
pub const _NL_CTYPE_INDIGITS1_MB: C2RustUnnamed_1 = 21;
pub const _NL_CTYPE_INDIGITS0_MB: C2RustUnnamed_1 = 20;
pub const _NL_CTYPE_INDIGITS_MB_LEN: C2RustUnnamed_1 = 19;
pub const _NL_CTYPE_MAP_OFFSET: C2RustUnnamed_1 = 18;
pub const _NL_CTYPE_CLASS_OFFSET: C2RustUnnamed_1 = 17;
pub const _NL_CTYPE_TOLOWER32: C2RustUnnamed_1 = 16;
pub const _NL_CTYPE_TOUPPER32: C2RustUnnamed_1 = 15;
pub const CODESET: C2RustUnnamed_1 = 14;
pub const _NL_CTYPE_CODESET_NAME: C2RustUnnamed_1 = 14;
pub const _NL_CTYPE_MB_CUR_MAX: C2RustUnnamed_1 = 13;
pub const _NL_CTYPE_WIDTH: C2RustUnnamed_1 = 12;
pub const _NL_CTYPE_MAP_NAMES: C2RustUnnamed_1 = 11;
pub const _NL_CTYPE_CLASS_NAMES: C2RustUnnamed_1 = 10;
pub const _NL_CTYPE_GAP6: C2RustUnnamed_1 = 9;
pub const _NL_CTYPE_GAP5: C2RustUnnamed_1 = 8;
pub const _NL_CTYPE_GAP4: C2RustUnnamed_1 = 7;
pub const _NL_CTYPE_GAP3: C2RustUnnamed_1 = 6;
pub const _NL_CTYPE_CLASS32: C2RustUnnamed_1 = 5;
pub const _NL_CTYPE_GAP2: C2RustUnnamed_1 = 4;
pub const _NL_CTYPE_TOLOWER: C2RustUnnamed_1 = 3;
pub const _NL_CTYPE_GAP1: C2RustUnnamed_1 = 2;
pub const _NL_CTYPE_TOUPPER: C2RustUnnamed_1 = 1;
pub const _NL_CTYPE_CLASS: C2RustUnnamed_1 = 0;
pub const _NL_NUM_LC_COLLATE: C2RustUnnamed_1 = 196627;
pub const _NL_COLLATE_CODESET: C2RustUnnamed_1 = 196626;
pub const _NL_COLLATE_COLLSEQWC: C2RustUnnamed_1 = 196625;
pub const _NL_COLLATE_COLLSEQMB: C2RustUnnamed_1 = 196624;
pub const _NL_COLLATE_SYMB_EXTRAMB: C2RustUnnamed_1 = 196623;
pub const _NL_COLLATE_SYMB_TABLEMB: C2RustUnnamed_1 = 196622;
pub const _NL_COLLATE_SYMB_HASH_SIZEMB: C2RustUnnamed_1 = 196621;
pub const _NL_COLLATE_INDIRECTWC: C2RustUnnamed_1 = 196620;
pub const _NL_COLLATE_EXTRAWC: C2RustUnnamed_1 = 196619;
pub const _NL_COLLATE_WEIGHTWC: C2RustUnnamed_1 = 196618;
pub const _NL_COLLATE_TABLEWC: C2RustUnnamed_1 = 196617;
pub const _NL_COLLATE_GAP3: C2RustUnnamed_1 = 196616;
pub const _NL_COLLATE_GAP2: C2RustUnnamed_1 = 196615;
pub const _NL_COLLATE_GAP1: C2RustUnnamed_1 = 196614;
pub const _NL_COLLATE_INDIRECTMB: C2RustUnnamed_1 = 196613;
pub const _NL_COLLATE_EXTRAMB: C2RustUnnamed_1 = 196612;
pub const _NL_COLLATE_WEIGHTMB: C2RustUnnamed_1 = 196611;
pub const _NL_COLLATE_TABLEMB: C2RustUnnamed_1 = 196610;
pub const _NL_COLLATE_RULESETS: C2RustUnnamed_1 = 196609;
pub const _NL_COLLATE_NRULES: C2RustUnnamed_1 = 196608;
pub const _NL_NUM_LC_TIME: C2RustUnnamed_1 = 131231;
pub const _NL_WABALTMON_12: C2RustUnnamed_1 = 131230;
pub const _NL_WABALTMON_11: C2RustUnnamed_1 = 131229;
pub const _NL_WABALTMON_10: C2RustUnnamed_1 = 131228;
pub const _NL_WABALTMON_9: C2RustUnnamed_1 = 131227;
pub const _NL_WABALTMON_8: C2RustUnnamed_1 = 131226;
pub const _NL_WABALTMON_7: C2RustUnnamed_1 = 131225;
pub const _NL_WABALTMON_6: C2RustUnnamed_1 = 131224;
pub const _NL_WABALTMON_5: C2RustUnnamed_1 = 131223;
pub const _NL_WABALTMON_4: C2RustUnnamed_1 = 131222;
pub const _NL_WABALTMON_3: C2RustUnnamed_1 = 131221;
pub const _NL_WABALTMON_2: C2RustUnnamed_1 = 131220;
pub const _NL_WABALTMON_1: C2RustUnnamed_1 = 131219;
pub const _NL_ABALTMON_12: C2RustUnnamed_1 = 131218;
pub const _NL_ABALTMON_11: C2RustUnnamed_1 = 131217;
pub const _NL_ABALTMON_10: C2RustUnnamed_1 = 131216;
pub const _NL_ABALTMON_9: C2RustUnnamed_1 = 131215;
pub const _NL_ABALTMON_8: C2RustUnnamed_1 = 131214;
pub const _NL_ABALTMON_7: C2RustUnnamed_1 = 131213;
pub const _NL_ABALTMON_6: C2RustUnnamed_1 = 131212;
pub const _NL_ABALTMON_5: C2RustUnnamed_1 = 131211;
pub const _NL_ABALTMON_4: C2RustUnnamed_1 = 131210;
pub const _NL_ABALTMON_3: C2RustUnnamed_1 = 131209;
pub const _NL_ABALTMON_2: C2RustUnnamed_1 = 131208;
pub const _NL_ABALTMON_1: C2RustUnnamed_1 = 131207;
pub const _NL_WALTMON_12: C2RustUnnamed_1 = 131206;
pub const _NL_WALTMON_11: C2RustUnnamed_1 = 131205;
pub const _NL_WALTMON_10: C2RustUnnamed_1 = 131204;
pub const _NL_WALTMON_9: C2RustUnnamed_1 = 131203;
pub const _NL_WALTMON_8: C2RustUnnamed_1 = 131202;
pub const _NL_WALTMON_7: C2RustUnnamed_1 = 131201;
pub const _NL_WALTMON_6: C2RustUnnamed_1 = 131200;
pub const _NL_WALTMON_5: C2RustUnnamed_1 = 131199;
pub const _NL_WALTMON_4: C2RustUnnamed_1 = 131198;
pub const _NL_WALTMON_3: C2RustUnnamed_1 = 131197;
pub const _NL_WALTMON_2: C2RustUnnamed_1 = 131196;
pub const _NL_WALTMON_1: C2RustUnnamed_1 = 131195;
pub const __ALTMON_12: C2RustUnnamed_1 = 131194;
pub const __ALTMON_11: C2RustUnnamed_1 = 131193;
pub const __ALTMON_10: C2RustUnnamed_1 = 131192;
pub const __ALTMON_9: C2RustUnnamed_1 = 131191;
pub const __ALTMON_8: C2RustUnnamed_1 = 131190;
pub const __ALTMON_7: C2RustUnnamed_1 = 131189;
pub const __ALTMON_6: C2RustUnnamed_1 = 131188;
pub const __ALTMON_5: C2RustUnnamed_1 = 131187;
pub const __ALTMON_4: C2RustUnnamed_1 = 131186;
pub const __ALTMON_3: C2RustUnnamed_1 = 131185;
pub const __ALTMON_2: C2RustUnnamed_1 = 131184;
pub const __ALTMON_1: C2RustUnnamed_1 = 131183;
pub const _NL_TIME_CODESET: C2RustUnnamed_1 = 131182;
pub const _NL_W_DATE_FMT: C2RustUnnamed_1 = 131181;
pub const _DATE_FMT: C2RustUnnamed_1 = 131180;
pub const _NL_TIME_TIMEZONE: C2RustUnnamed_1 = 131179;
pub const _NL_TIME_CAL_DIRECTION: C2RustUnnamed_1 = 131178;
pub const _NL_TIME_FIRST_WORKDAY: C2RustUnnamed_1 = 131177;
pub const _NL_TIME_FIRST_WEEKDAY: C2RustUnnamed_1 = 131176;
pub const _NL_TIME_WEEK_1STWEEK: C2RustUnnamed_1 = 131175;
pub const _NL_TIME_WEEK_1STDAY: C2RustUnnamed_1 = 131174;
pub const _NL_TIME_WEEK_NDAYS: C2RustUnnamed_1 = 131173;
pub const _NL_WERA_T_FMT: C2RustUnnamed_1 = 131172;
pub const _NL_WERA_D_T_FMT: C2RustUnnamed_1 = 131171;
pub const _NL_WALT_DIGITS: C2RustUnnamed_1 = 131170;
pub const _NL_WERA_D_FMT: C2RustUnnamed_1 = 131169;
pub const _NL_WERA_YEAR: C2RustUnnamed_1 = 131168;
pub const _NL_WT_FMT_AMPM: C2RustUnnamed_1 = 131167;
pub const _NL_WT_FMT: C2RustUnnamed_1 = 131166;
pub const _NL_WD_FMT: C2RustUnnamed_1 = 131165;
pub const _NL_WD_T_FMT: C2RustUnnamed_1 = 131164;
pub const _NL_WPM_STR: C2RustUnnamed_1 = 131163;
pub const _NL_WAM_STR: C2RustUnnamed_1 = 131162;
pub const _NL_WMON_12: C2RustUnnamed_1 = 131161;
pub const _NL_WMON_11: C2RustUnnamed_1 = 131160;
pub const _NL_WMON_10: C2RustUnnamed_1 = 131159;
pub const _NL_WMON_9: C2RustUnnamed_1 = 131158;
pub const _NL_WMON_8: C2RustUnnamed_1 = 131157;
pub const _NL_WMON_7: C2RustUnnamed_1 = 131156;
pub const _NL_WMON_6: C2RustUnnamed_1 = 131155;
pub const _NL_WMON_5: C2RustUnnamed_1 = 131154;
pub const _NL_WMON_4: C2RustUnnamed_1 = 131153;
pub const _NL_WMON_3: C2RustUnnamed_1 = 131152;
pub const _NL_WMON_2: C2RustUnnamed_1 = 131151;
pub const _NL_WMON_1: C2RustUnnamed_1 = 131150;
pub const _NL_WABMON_12: C2RustUnnamed_1 = 131149;
pub const _NL_WABMON_11: C2RustUnnamed_1 = 131148;
pub const _NL_WABMON_10: C2RustUnnamed_1 = 131147;
pub const _NL_WABMON_9: C2RustUnnamed_1 = 131146;
pub const _NL_WABMON_8: C2RustUnnamed_1 = 131145;
pub const _NL_WABMON_7: C2RustUnnamed_1 = 131144;
pub const _NL_WABMON_6: C2RustUnnamed_1 = 131143;
pub const _NL_WABMON_5: C2RustUnnamed_1 = 131142;
pub const _NL_WABMON_4: C2RustUnnamed_1 = 131141;
pub const _NL_WABMON_3: C2RustUnnamed_1 = 131140;
pub const _NL_WABMON_2: C2RustUnnamed_1 = 131139;
pub const _NL_WABMON_1: C2RustUnnamed_1 = 131138;
pub const _NL_WDAY_7: C2RustUnnamed_1 = 131137;
pub const _NL_WDAY_6: C2RustUnnamed_1 = 131136;
pub const _NL_WDAY_5: C2RustUnnamed_1 = 131135;
pub const _NL_WDAY_4: C2RustUnnamed_1 = 131134;
pub const _NL_WDAY_3: C2RustUnnamed_1 = 131133;
pub const _NL_WDAY_2: C2RustUnnamed_1 = 131132;
pub const _NL_WDAY_1: C2RustUnnamed_1 = 131131;
pub const _NL_WABDAY_7: C2RustUnnamed_1 = 131130;
pub const _NL_WABDAY_6: C2RustUnnamed_1 = 131129;
pub const _NL_WABDAY_5: C2RustUnnamed_1 = 131128;
pub const _NL_WABDAY_4: C2RustUnnamed_1 = 131127;
pub const _NL_WABDAY_3: C2RustUnnamed_1 = 131126;
pub const _NL_WABDAY_2: C2RustUnnamed_1 = 131125;
pub const _NL_WABDAY_1: C2RustUnnamed_1 = 131124;
pub const _NL_TIME_ERA_ENTRIES: C2RustUnnamed_1 = 131123;
pub const _NL_TIME_ERA_NUM_ENTRIES: C2RustUnnamed_1 = 131122;
pub const ERA_T_FMT: C2RustUnnamed_1 = 131121;
pub const ERA_D_T_FMT: C2RustUnnamed_1 = 131120;
pub const ALT_DIGITS: C2RustUnnamed_1 = 131119;
pub const ERA_D_FMT: C2RustUnnamed_1 = 131118;
pub const __ERA_YEAR: C2RustUnnamed_1 = 131117;
pub const ERA: C2RustUnnamed_1 = 131116;
pub const T_FMT_AMPM: C2RustUnnamed_1 = 131115;
pub const T_FMT: C2RustUnnamed_1 = 131114;
pub const D_FMT: C2RustUnnamed_1 = 131113;
pub const D_T_FMT: C2RustUnnamed_1 = 131112;
pub const PM_STR: C2RustUnnamed_1 = 131111;
pub const AM_STR: C2RustUnnamed_1 = 131110;
pub const MON_12: C2RustUnnamed_1 = 131109;
pub const MON_11: C2RustUnnamed_1 = 131108;
pub const MON_10: C2RustUnnamed_1 = 131107;
pub const MON_9: C2RustUnnamed_1 = 131106;
pub const MON_8: C2RustUnnamed_1 = 131105;
pub const MON_7: C2RustUnnamed_1 = 131104;
pub const MON_6: C2RustUnnamed_1 = 131103;
pub const MON_5: C2RustUnnamed_1 = 131102;
pub const MON_4: C2RustUnnamed_1 = 131101;
pub const MON_3: C2RustUnnamed_1 = 131100;
pub const MON_2: C2RustUnnamed_1 = 131099;
pub const MON_1: C2RustUnnamed_1 = 131098;
pub const ABMON_12: C2RustUnnamed_1 = 131097;
pub const ABMON_11: C2RustUnnamed_1 = 131096;
pub const ABMON_10: C2RustUnnamed_1 = 131095;
pub const ABMON_9: C2RustUnnamed_1 = 131094;
pub const ABMON_8: C2RustUnnamed_1 = 131093;
pub const ABMON_7: C2RustUnnamed_1 = 131092;
pub const ABMON_6: C2RustUnnamed_1 = 131091;
pub const ABMON_5: C2RustUnnamed_1 = 131090;
pub const ABMON_4: C2RustUnnamed_1 = 131089;
pub const ABMON_3: C2RustUnnamed_1 = 131088;
pub const ABMON_2: C2RustUnnamed_1 = 131087;
pub const ABMON_1: C2RustUnnamed_1 = 131086;
pub const DAY_7: C2RustUnnamed_1 = 131085;
pub const DAY_6: C2RustUnnamed_1 = 131084;
pub const DAY_5: C2RustUnnamed_1 = 131083;
pub const DAY_4: C2RustUnnamed_1 = 131082;
pub const DAY_3: C2RustUnnamed_1 = 131081;
pub const DAY_2: C2RustUnnamed_1 = 131080;
pub const DAY_1: C2RustUnnamed_1 = 131079;
pub const ABDAY_7: C2RustUnnamed_1 = 131078;
pub const ABDAY_6: C2RustUnnamed_1 = 131077;
pub const ABDAY_5: C2RustUnnamed_1 = 131076;
pub const ABDAY_4: C2RustUnnamed_1 = 131075;
pub const ABDAY_3: C2RustUnnamed_1 = 131074;
pub const ABDAY_2: C2RustUnnamed_1 = 131073;
pub const ABDAY_1: C2RustUnnamed_1 = 131072;
pub type rl_compentry_func_t = unsafe extern "C" fn(
    *const libc::c_char,
    libc::c_int,
) -> *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unittype {
    pub numerator: [*mut libc::c_char; 100],
    pub denominator: [*mut libc::c_char; 100],
    pub factor: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct functype {
    pub param: *mut libc::c_char,
    pub def: *mut libc::c_char,
    pub dimen: *mut libc::c_char,
    pub domain_min: *mut libc::c_double,
    pub domain_max: *mut libc::c_double,
    pub domain_min_open: libc::c_int,
    pub domain_max_open: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pair {
    pub location: libc::c_double,
    pub value: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct func {
    pub name: *mut libc::c_char,
    pub forward: functype,
    pub inverse: functype,
    pub table: *mut pair,
    pub tablelen: libc::c_int,
    pub tableunit: *mut libc::c_char,
    pub next: *mut func,
    pub skip_error_check: libc::c_int,
    pub linenumber: libc::c_int,
    pub file: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parseflag {
    pub oldstar: libc::c_int,
    pub minusminus: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct prefixlist {
    pub len: libc::c_int,
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub linenumber: libc::c_int,
    pub file: *mut libc::c_char,
    pub next: *mut prefixlist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unitlist {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub linenumber: libc::c_int,
    pub file: *mut libc::c_char,
    pub next: *mut unitlist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub word: *mut libc::c_char,
    pub delimit: libc::c_char,
    pub checkopen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub format: *mut libc::c_char,
    pub width: libc::c_int,
    pub precision: libc::c_int,
    pub type_0: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub interactive: libc::c_int,
    pub unitlists: libc::c_int,
    pub oneline: libc::c_int,
    pub quiet: libc::c_int,
    pub round: libc::c_int,
    pub showconformable: libc::c_int,
    pub showfactor: libc::c_int,
    pub strictconvert: libc::c_int,
    pub unitcheck: libc::c_int,
    pub verbose: libc::c_int,
    pub readline: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wantalias {
    pub name: *mut libc::c_char,
    pub definition: *mut libc::c_char,
    pub next: *mut wantalias,
    pub linenumber: libc::c_int,
    pub file: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct namedef {
    pub name: *mut libc::c_char,
    pub def: *mut libc::c_char,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
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
unsafe extern "C" fn vprintf(
    mut __fmt: *const libc::c_char,
    mut __arg: ::std::ffi::VaList,
) -> libc::c_int {
    return vfprintf(stdout, __fmt, __arg.as_va_list());
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
pub static mut exit_commands: [*mut libc::c_char; 3] = [
    b"quit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"exit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
pub static mut all_commands: [*mut libc::c_char; 6] = [
    b"quit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"exit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"help\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"search\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"?\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
pub static mut fnkeywords: [C2RustUnnamed_2; 5] = [
    {
        let mut init = C2RustUnnamed_2 {
            word: b"units=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            delimit: ';' as i32 as libc::c_char,
            checkopen: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            word: b"domain=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            delimit: ',' as i32 as libc::c_char,
            checkopen: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            word: b"range=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            delimit: ',' as i32 as libc::c_char,
            checkopen: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            word: b"noerror \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            delimit: ' ' as i32 as libc::c_char,
            checkopen: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            word: 0 as *const libc::c_char as *mut libc::c_char,
            delimit: 0 as libc::c_int as libc::c_char,
            checkopen: 0,
        };
        init
    },
];
pub static mut builtins: [*mut libc::c_char; 19] = [
    b"sin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cos\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tan\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ln\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"log\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"exp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"acos\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"atan\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"asin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sqrt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cuberoot\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"per\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sinh\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cosh\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tanh\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"asinh\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"atanh\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"acosh\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
pub static mut num_format: C2RustUnnamed_3 = C2RustUnnamed_3 {
    format: 0 as *const libc::c_char as *mut libc::c_char,
    width: 0,
    precision: 0,
    type_0: 0,
};
pub static mut flags: C2RustUnnamed_4 = C2RustUnnamed_4 {
    interactive: 0,
    unitlists: 0,
    oneline: 0,
    quiet: 0,
    round: 0,
    showconformable: 0,
    showfactor: 0,
    strictconvert: 0,
    unitcheck: 0,
    verbose: 0,
    readline: 0,
};
pub static mut parserflags: parseflag = parseflag {
    oldstar: 0,
    minusminus: 0,
};
pub static mut homeunitsfile: *mut libc::c_char = b".units\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
pub static mut homedir: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut homedir_error: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut pager: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut mylocale: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut utf8mode: libc::c_int = 0;
pub static mut powerstring: *mut libc::c_char = b"^\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
pub static mut unitsfiles: [*mut libc::c_char; 26] = [0 as *const libc::c_char
    as *mut libc::c_char; 26];
pub static mut logfilename: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut logfile: *mut FILE = 0 as *const FILE as *mut FILE;
pub static mut promptprefix: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut progname: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut fullprogname: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut progdir: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut datadir: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut deftext: *mut libc::c_char = b"        Definition: \0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
pub static mut digits: *mut libc::c_char = b"0123456789.,\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
pub static mut errormsg: [*mut libc::c_char; 26] = [
    b"Successful completion\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Parse error\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Product overflow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Unit reduction error (bad unit definition)\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Invalid sum or difference of non-conformable units\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Unit not dimensionless\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Unit not a root\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Unknown unit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Bad argument\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Weird nonlinear unit type (bug in program)\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Function argument has wrong dimension\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Argument of function outside domain\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Nonlinear unit definition has unit error\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"No inverse defined\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Parser memory overflow (recursive function definition?)\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Argument wrong dimension or bad nonlinear unit definition\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Cannot open units file\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Units file contains errors\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Memory allocation error\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Malformed number\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Unit name ends with a digit other than 0 or 1 without preceding '_'\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"No previous result; '_' not set\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Base unit not dimensionless; rational exponent required\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Base unit not a root\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Exponent not dimensionless\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Unknown function name\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
pub static mut invalid_utf8: *mut libc::c_char = b"invalid/nonprinting UTF-8\0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
pub static mut irreducible: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut utab: [*mut unitlist; 101] = [0 as *const unitlist as *mut unitlist; 101];
pub static mut ptab: [*mut prefixlist; 128] = [0 as *const prefixlist
    as *mut prefixlist; 128];
pub static mut firstalias: *mut wantalias = 0 as *const wantalias as *mut wantalias;
pub static mut aliaslistend: *mut *mut wantalias = unsafe {
    &firstalias as *const *mut wantalias as *mut *mut wantalias
};
pub static mut ftab: [*mut func; 128] = [0 as *const func as *mut func; 128];
pub static mut function_parameter: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut parameter_value: *mut unittype = 0 as *const unittype as *mut unittype;
pub static mut lastunitset: libc::c_int = 0 as libc::c_int;
pub static mut lastunit: unittype = unittype {
    numerator: [0 as *const libc::c_char as *mut libc::c_char; 100],
    denominator: [0 as *const libc::c_char as *mut libc::c_char; 100],
    factor: 0.,
};
pub static mut NULLUNIT: *mut libc::c_char = b"\0" as *const u8 as *const libc::c_char
    as *mut libc::c_char;
pub static mut historyfile: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut init_history_length: libc::c_int = 0;
pub static mut init_history_base: libc::c_int = 0;
pub unsafe extern "C" fn save_history() {
    let mut newentries: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    newentries = history_length - init_history_length;
    if history_max_entries > 0 as libc::c_int {
        newentries += history_base - init_history_base;
        if newentries > history_max_entries {
            newentries = history_max_entries;
        }
    }
    err = append_history(newentries, historyfile);
    if err != 0 {
        if err == 2 as libc::c_int {
            err = write_history(historyfile);
        }
        if err != 0 {
            printf(
                b"Unable to write history to '%s': %s\n\0" as *const u8
                    as *const libc::c_char,
                historyfile,
                strerror(err),
            );
            return;
        }
    }
    history_truncate_file(historyfile, 5000 as libc::c_int);
}
pub unsafe extern "C" fn growbuffer(
    mut buf: *mut *mut libc::c_char,
    mut bufsize_0: *mut libc::c_int,
) {
    let mut usemalloc: libc::c_int = 0;
    usemalloc = ((*buf).is_null() || *bufsize_0 == 0) as libc::c_int;
    *bufsize_0 += 100 as libc::c_int;
    if usemalloc != 0 {
        *buf = malloc(*bufsize_0 as libc::c_ulong) as *mut libc::c_char;
    } else {
        *buf = realloc(*buf as *mut libc::c_void, *bufsize_0 as libc::c_ulong)
            as *mut libc::c_char;
    }
    if (*buf).is_null() {
        fprintf(
            stderr,
            b"%s: memory allocation error (growbuffer)\n\0" as *const u8
                as *const libc::c_char,
            progname,
        );
        exit(1 as libc::c_int);
    }
}
pub unsafe extern "C" fn openfile(
    mut file: *mut libc::c_char,
    mut mode: *mut libc::c_char,
) -> *mut FILE {
    let mut fileptr: *mut FILE = 0 as *mut FILE;
    let mut statbuf: stat = stat {
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
        st_atime: 0,
        st_atimensec: 0,
        st_mtime: 0,
        st_mtimensec: 0,
        st_ctime: 0,
        st_ctimensec: 0,
        __glibc_reserved: [0; 3],
    };
    if stat(file, &mut statbuf) == 0 as libc::c_int
        && statbuf.st_mode & 0o40000 as libc::c_int as libc::c_uint != 0
    {
        *__errno_location() = 21 as libc::c_int;
        return 0 as *mut FILE;
    }
    fileptr = fopen(file, mode);
    return fileptr;
}
pub unsafe extern "C" fn logprintf(mut format: *const libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    vprintf(format, args_0.as_va_list());
    if !logfile.is_null() {
        args_0 = args.clone();
        vfprintf(logfile, format, args_0.as_va_list());
    }
}
pub unsafe extern "C" fn logputchar(mut c: libc::c_char) {
    putchar(c as libc::c_int);
    if !logfile.is_null() {
        fputc(c as libc::c_int, logfile);
    }
}
pub unsafe extern "C" fn logputs(mut s: *const libc::c_char) {
    fputs(s, stdout);
    if !logfile.is_null() {
        fputs(s, logfile);
    }
}
pub unsafe extern "C" fn hassubscript(mut str: *const libc::c_char) -> libc::c_int {
    let mut ptr: *const libc::c_char = str
        .offset(strlen(str) as isize)
        .offset(-(1 as libc::c_int as isize));
    while ptr > str {
        if (strchr(digits, *ptr as libc::c_int)).is_null() {
            return 0 as libc::c_int;
        }
        ptr = ptr.offset(-1);
        ptr;
        if *ptr as libc::c_int == '_' as i32 {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn replace_minus(mut input: *mut libc::c_char) {
    let mut unicode_minus: [*mut libc::c_char; 4] = [
        b"\xE2\x80\x92\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xE2\x80\x93\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\xE2\x88\x92\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *mut libc::c_char,
    ];
    let mut inptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut minus: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    minus = unicode_minus.as_mut_ptr();
    while !(*minus).is_null() {
        outptr = input;
        inptr = outptr;
        loop {
            ptr = strstr(inptr, *minus);
            if !ptr.is_null() {
                while inptr < ptr {
                    let fresh0 = inptr;
                    inptr = inptr.offset(1);
                    let fresh1 = outptr;
                    outptr = outptr.offset(1);
                    *fresh1 = *fresh0;
                }
                let fresh2 = outptr;
                outptr = outptr.offset(1);
                *fresh2 = '-' as i32 as libc::c_char;
                inptr = ptr.offset(strlen(*minus) as isize);
            }
            if ptr.is_null() {
                break;
            }
        }
        if inptr > input {
            while *inptr != 0 {
                let fresh3 = inptr;
                inptr = inptr.offset(1);
                let fresh4 = outptr;
                outptr = outptr.offset(1);
                *fresh4 = *fresh3;
            }
            *outptr = '\0' as i32 as libc::c_char;
        }
        minus = minus.offset(1);
        minus;
    }
}
pub unsafe extern "C" fn replacectrlchars(mut string: *mut libc::c_char) {
    while *string != 0 {
        if *(*__ctype_b_loc()).offset(*string as libc::c_int as isize) as libc::c_int
            & _IScntrl as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            *string = ' ' as i32 as libc::c_char;
        }
        string = string.offset(1);
        string;
    }
}
pub unsafe extern "C" fn fgetscont(
    mut buf: *mut libc::c_char,
    mut size: libc::c_int,
    mut file: *mut FILE,
    mut count: *mut libc::c_int,
) -> *mut libc::c_char {
    if (fgets(buf, size, file)).is_null() {
        return 0 as *mut libc::c_char;
    }
    *count += 1;
    *count;
    while strlen(buf) >= 2 as libc::c_int as libc::c_ulong
        && 0 as libc::c_int
            == strcmp(
                buf.offset(strlen(buf) as isize).offset(-(2 as libc::c_int as isize)),
                b"\\\n\0" as *const u8 as *const libc::c_char,
            )
    {
        *count += 1;
        *count;
        *buf
            .offset(
                (strlen(buf)).wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize,
            ) = 0 as libc::c_int as libc::c_char;
        if strlen(buf) >= (size - 1 as libc::c_int) as libc::c_ulong {
            return buf;
        }
        if (fgets(
            buf.offset(strlen(buf) as isize),
            (size as libc::c_ulong).wrapping_sub(strlen(buf)) as libc::c_int,
            file,
        ))
            .is_null()
        {
            return buf;
        }
    }
    if *buf.offset(strlen(buf) as isize).offset(-(1 as libc::c_int as isize))
        as libc::c_int == '\\' as i32
    {
        ungetc('\\' as i32, file);
        *buf
            .offset(strlen(buf) as isize)
            .offset(-(1 as libc::c_int as isize)) = 0 as libc::c_int as libc::c_char;
    }
    return buf;
}
pub unsafe extern "C" fn fgetslong(
    mut buf: *mut *mut libc::c_char,
    mut bufsize_0: *mut libc::c_int,
    mut file: *mut FILE,
    mut count: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut dummy: libc::c_int = 0;
    if count.is_null() {
        count = &mut dummy;
    }
    if *bufsize_0 == 0 {
        growbuffer(buf, bufsize_0);
    }
    if (fgetscont(*buf, *bufsize_0, file, count)).is_null() {
        return 0 as *mut libc::c_char;
    }
    while *(*buf).offset(strlen(*buf) as isize).offset(-(1 as libc::c_int as isize))
        as libc::c_int != '\n' as i32 && feof(file) == 0
    {
        growbuffer(buf, bufsize_0);
        fgetscont(
            (*buf).offset(strlen(*buf) as isize),
            (*bufsize_0 as libc::c_ulong).wrapping_sub(strlen(*buf)) as libc::c_int,
            file,
            count,
        );
        *count -= 1;
        *count;
    }
    replacectrlchars(*buf);
    return *buf;
}
pub unsafe extern "C" fn mymalloc(
    mut bytes: libc::c_int,
    mut mesg: *const libc::c_char,
) -> *mut libc::c_void {
    let mut pointer: *mut libc::c_void = 0 as *mut libc::c_void;
    pointer = malloc(bytes as libc::c_ulong);
    if pointer.is_null() {
        fprintf(
            stderr,
            b"%s: memory allocation error %s\n\0" as *const u8 as *const libc::c_char,
            progname,
            mesg,
        );
        exit(1 as libc::c_int);
    }
    return pointer;
}
pub unsafe extern "C" fn dupstr(mut str: *const libc::c_char) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    ret = mymalloc(
        (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"(dupstr)\0" as *const u8 as *const libc::c_char,
    ) as *mut libc::c_char;
    strcpy(ret, str);
    return ret;
}
pub unsafe extern "C" fn dupnstr(
    mut string: *const libc::c_char,
    mut length: libc::c_int,
) -> *mut libc::c_char {
    let mut newstr: *mut libc::c_char = 0 as *mut libc::c_char;
    newstr = mymalloc(
        length + 1 as libc::c_int,
        b"(dupnstr)\0" as *const u8 as *const libc::c_char,
    ) as *mut libc::c_char;
    strncpy(newstr, string, length as libc::c_ulong);
    *newstr.offset(length as isize) = 0 as libc::c_int as libc::c_char;
    return newstr;
}
pub unsafe extern "C" fn strwidth(mut str: *const libc::c_char) -> libc::c_int {
    let mut widestr: *mut wchar_t = 0 as *mut wchar_t;
    let mut len: libc::c_int = 0;
    if utf8mode == 0 {
        return strlen(str) as libc::c_int;
    }
    len = (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    widestr = mymalloc(
        (::std::mem::size_of::<wchar_t>() as libc::c_ulong)
            .wrapping_mul(len as libc::c_ulong) as libc::c_int,
        b"(strwidth)\0" as *const u8 as *const libc::c_char,
    ) as *mut wchar_t;
    len = mbsrtowcs(widestr, &mut str, len as size_t, 0 as *mut mbstate_t)
        as libc::c_int;
    if len == -(1 as libc::c_int) {
        free(widestr as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    len = wcswidth(widestr, len as size_t);
    free(widestr as *mut libc::c_void);
    return len;
}
pub unsafe extern "C" fn uhash(mut str: *const libc::c_char) -> libc::c_uint {
    let mut hashval: libc::c_uint = 0;
    hashval = 0 as libc::c_int as libc::c_uint;
    while *str != 0 {
        hashval = (*str as libc::c_uint)
            .wrapping_add((31 as libc::c_int as libc::c_uint).wrapping_mul(hashval));
        str = str.offset(1);
        str;
    }
    return hashval.wrapping_rem(101 as libc::c_int as libc::c_uint);
}
pub unsafe extern "C" fn ulookup(mut str: *const libc::c_char) -> *mut unitlist {
    let mut uptr: *mut unitlist = 0 as *mut unitlist;
    uptr = utab[uhash(str) as usize];
    while !uptr.is_null() {
        if strcmp(str, (*uptr).name) == 0 as libc::c_int {
            return uptr;
        }
        uptr = (*uptr).next;
    }
    return 0 as *mut unitlist;
}
pub unsafe extern "C" fn plookup(mut str: *const libc::c_char) -> *mut prefixlist {
    let mut prefix: *mut prefixlist = 0 as *mut prefixlist;
    let mut bestprefix: *mut prefixlist = 0 as *mut prefixlist;
    let mut bestlength: libc::c_int = 0 as libc::c_int;
    prefix = ptab[(*str as libc::c_int & 127 as libc::c_int) as usize];
    while !prefix.is_null() {
        if (*prefix).len > bestlength
            && strncmp(str, (*prefix).name, (*prefix).len as libc::c_ulong) == 0
        {
            bestlength = (*prefix).len;
            bestprefix = prefix;
        }
        prefix = (*prefix).next;
    }
    return bestprefix;
}
pub unsafe extern "C" fn fnlookup(mut str: *const libc::c_char) -> *mut func {
    let mut funcptr: *mut func = 0 as *mut func;
    funcptr = ftab[(*str as libc::c_int & 127 as libc::c_int) as usize];
    while !funcptr.is_null() {
        if strcmp((*funcptr).name, str) == 0 {
            return funcptr;
        }
        funcptr = (*funcptr).next;
    }
    return 0 as *mut func;
}
pub unsafe extern "C" fn aliaslookup(mut str: *const libc::c_char) -> *mut wantalias {
    let mut aliasptr: *mut wantalias = 0 as *mut wantalias;
    aliasptr = firstalias;
    while !aliasptr.is_null() {
        if strcmp((*aliasptr).name, str) == 0 {
            return aliasptr;
        }
        aliasptr = (*aliasptr).next;
    }
    return 0 as *mut wantalias;
}
pub unsafe extern "C" fn addfunction(mut newfunc: *mut func) {
    let mut val: libc::c_int = 0;
    val = *(*newfunc).name as libc::c_int & 127 as libc::c_int;
    (*newfunc).next = ftab[val as usize];
    ftab[val as usize] = newfunc;
}
pub unsafe extern "C" fn freefunction(mut funcentry: *mut func) {
    if !((*funcentry).table).is_null() {
        free((*funcentry).table as *mut libc::c_void);
        free((*funcentry).tableunit as *mut libc::c_void);
    } else {
        free((*funcentry).forward.param as *mut libc::c_void);
        free((*funcentry).forward.def as *mut libc::c_void);
        if !((*funcentry).forward.domain_min).is_null() {
            free((*funcentry).forward.domain_min as *mut libc::c_void);
        }
        if !((*funcentry).forward.domain_max).is_null() {
            free((*funcentry).forward.domain_max as *mut libc::c_void);
        }
        if !((*funcentry).inverse.domain_min).is_null() {
            free((*funcentry).inverse.domain_min as *mut libc::c_void);
        }
        if !((*funcentry).inverse.domain_max).is_null() {
            free((*funcentry).inverse.domain_max as *mut libc::c_void);
        }
        if !((*funcentry).forward.dimen).is_null() {
            free((*funcentry).forward.dimen as *mut libc::c_void);
        }
        if !((*funcentry).inverse.dimen).is_null() {
            free((*funcentry).inverse.dimen as *mut libc::c_void);
        }
        if !((*funcentry).inverse.def).is_null() {
            free((*funcentry).inverse.def as *mut libc::c_void);
        }
        if !((*funcentry).inverse.param).is_null() {
            free((*funcentry).inverse.param as *mut libc::c_void);
        }
    };
}
pub unsafe extern "C" fn removespaces(mut in_0: *mut libc::c_char) {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if *in_0 != 0 {
        ptr = in_0.offset(strlen(in_0) as isize).offset(-(1 as libc::c_int as isize));
        while *ptr as libc::c_int == ' ' as i32 {
            ptr = ptr.offset(-1);
            ptr;
        }
        *ptr.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
        if *in_0 as libc::c_int == ' ' as i32 {
            ptr = in_0
                .offset(
                    strspn(in_0, b" \0" as *const u8 as *const libc::c_char) as isize,
                );
            memmove(
                in_0 as *mut libc::c_void,
                ptr as *const libc::c_void,
                (strlen(ptr)).wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        }
    }
}
pub unsafe extern "C" fn invfnlookup(mut str: *mut libc::c_char) -> *mut func {
    if *str as libc::c_int != '~' as i32 {
        return 0 as *mut func;
    }
    removespaces(str.offset(1 as libc::c_int as isize));
    return fnlookup(str.offset(1 as libc::c_int as isize));
}
pub unsafe extern "C" fn strip_comment(
    mut line: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut comment: *mut libc::c_char = 0 as *mut libc::c_char;
    line = strchr(line, '#' as i32);
    if !line.is_null() {
        comment = line.offset(1 as libc::c_int as isize);
        *line = 0 as libc::c_int as libc::c_char;
    }
    return comment;
}
pub unsafe extern "C" fn tightprint(
    mut outfile: *mut FILE,
    mut string: *mut libc::c_char,
) {
    while *string != 0 {
        fputc(*string as libc::c_int, outfile);
        if *string as libc::c_int != ' ' as i32 {
            string = string.offset(1);
            string;
        } else {
            while *string as libc::c_int == ' ' as i32 {
                string = string.offset(1);
                string;
            }
        }
    }
}
pub unsafe extern "C" fn splitline(
    mut line: *mut libc::c_char,
    mut first: *mut *mut libc::c_char,
    mut second: *mut *mut libc::c_char,
) {
    *second = 0 as *mut libc::c_char;
    *first = strtok(line, b" \0" as *const u8 as *const libc::c_char);
    if !(*first).is_null() {
        *second = strtok(
            0 as *mut libc::c_char,
            b"\n\0" as *const u8 as *const libc::c_char,
        );
        if !(*second).is_null() {
            removespaces(*second);
            if **second as libc::c_int == 0 as libc::c_int {
                *second = 0 as *mut libc::c_char;
            }
        }
    }
}
pub unsafe extern "C" fn isdecimal(mut c: libc::c_char) -> libc::c_int {
    return (strchr(digits, c as libc::c_int)
        != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
}
pub unsafe extern "C" fn checkunitname(
    mut name: *mut libc::c_char,
    mut linenum: libc::c_int,
    mut file: *mut libc::c_char,
    mut errfile: *mut FILE,
) -> libc::c_int {
    let mut nonunitchars: [libc::c_char; 10] = *::std::mem::transmute::<
        &[u8; 10],
        &mut [libc::c_char; 10],
    >(b"~;+-*/|^)\0");
    let mut ptr: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut cptr: *mut libc::c_char = 0 as *mut libc::c_char;
    cptr = strpbrk(name, nonunitchars.as_mut_ptr());
    if !cptr.is_null() {
        if !errfile.is_null() {
            fprintf(
                errfile,
                b"%s: unit '%s' in units file '%s' on line %d ignored.  It contains invalid character '%c'\n\0"
                    as *const u8 as *const libc::c_char,
                progname,
                name,
                file,
                linenum,
                *cptr as libc::c_int,
            );
        }
        return 1 as libc::c_int;
    }
    if !(strchr(digits, *name.offset(0 as libc::c_int as isize) as libc::c_int))
        .is_null()
    {
        if !errfile.is_null() {
            fprintf(
                errfile,
                b"%s: unit '%s' in units file '%s' on line %d ignored.  It starts with a digit\n\0"
                    as *const u8 as *const libc::c_char,
                progname,
                name,
                file,
                linenum,
            );
        }
        return 1 as libc::c_int;
    }
    ptr = builtins.as_mut_ptr();
    while !(*ptr).is_null() {
        if strcmp(name, *ptr) == 0 {
            if !errfile.is_null() {
                fprintf(
                    errfile,
                    b"%s: redefinition of built-in function '%s' in file '%s' on line %d ignored.\n\0"
                        as *const u8 as *const libc::c_char,
                    progname,
                    name,
                    file,
                    linenum,
                );
            }
            return 1 as libc::c_int;
        }
        ptr = ptr.offset(1);
        ptr;
    }
    ptr = all_commands.as_mut_ptr();
    while !(*ptr).is_null() {
        if strcmp(name, *ptr) == 0 {
            if !errfile.is_null() {
                fprintf(
                    errfile,
                    b"%s: unit name '%s' in file '%s' on line %d may be hidden by command with the same name.\n\0"
                        as *const u8 as *const libc::c_char,
                    progname,
                    name,
                    file,
                    linenum,
                );
            }
        }
        ptr = ptr.offset(1);
        ptr;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn newunit(
    mut unitname: *mut libc::c_char,
    mut unitdef: *mut libc::c_char,
    mut count: *mut libc::c_int,
    mut linenum: libc::c_int,
    mut file: *mut libc::c_char,
    mut errfile: *mut FILE,
    mut redefine: libc::c_int,
) -> libc::c_int {
    let mut uptr: *mut unitlist = 0 as *mut unitlist;
    let mut hashval: libc::c_uint = 0;
    if *unitname.offset(0 as libc::c_int as isize) as libc::c_int == '_' as i32
        || *unitname
            .offset(strlen(unitname) as isize)
            .offset(-(1 as libc::c_int as isize)) as libc::c_int == '_' as i32
    {
        if !errfile.is_null() {
            fprintf(
                errfile,
                b"%s: unit '%s' on line %d of '%s' ignored.  It starts or ends with '_'\n\0"
                    as *const u8 as *const libc::c_char,
                progname,
                unitname,
                linenum,
                file,
            );
        }
        return 17 as libc::c_int;
    }
    if !(strchr(
        b".,23456789\0" as *const u8 as *const libc::c_char,
        *unitname.offset(strlen(unitname) as isize).offset(-(1 as libc::c_int as isize))
            as libc::c_int,
    ))
        .is_null() && hassubscript(unitname) == 0
    {
        if !errfile.is_null() {
            fprintf(
                errfile,
                b"%s: unit '%s' on line %d of '%s' ignored.  %s\n\0" as *const u8
                    as *const libc::c_char,
                progname,
                unitname,
                linenum,
                file,
                errormsg[20 as libc::c_int as usize],
            );
        }
        return 17 as libc::c_int;
    }
    if checkunitname(unitname, linenum, file, errfile) != 0 {
        return 17 as libc::c_int;
    }
    uptr = ulookup(unitname);
    if !uptr.is_null() {
        if flags.unitcheck != 0 && !errfile.is_null() && redefine == 0 {
            fprintf(
                errfile,
                b"%s: unit '%s' defined on line %d of '%s' is redefined on line %d of '%s'.\n\0"
                    as *const u8 as *const libc::c_char,
                progname,
                unitname,
                (*uptr).linenumber,
                (*uptr).file,
                linenum,
                file,
            );
        }
        free((*uptr).value as *mut libc::c_void);
    } else {
        uptr = mymalloc(
            ::std::mem::size_of::<unitlist>() as libc::c_ulong as libc::c_int,
            b"(newunit)\0" as *const u8 as *const libc::c_char,
        ) as *mut unitlist;
        (*uptr).name = dupstr(unitname);
        hashval = uhash((*uptr).name);
        (*uptr).next = utab[hashval as usize];
        utab[hashval as usize] = uptr;
        *count += 1;
        *count;
    }
    (*uptr).value = dupstr(unitdef);
    (*uptr).linenumber = linenum;
    (*uptr).file = file;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn newprefix(
    mut unitname: *mut libc::c_char,
    mut unitdef: *mut libc::c_char,
    mut count: *mut libc::c_int,
    mut linenum: libc::c_int,
    mut file: *mut libc::c_char,
    mut errfile: *mut FILE,
    mut redefine: libc::c_int,
) -> libc::c_int {
    let mut pfxptr: *mut prefixlist = 0 as *mut prefixlist;
    let mut pval: libc::c_uint = 0;
    *unitname
        .offset(strlen(unitname) as isize)
        .offset(-(1 as libc::c_int as isize)) = 0 as libc::c_int as libc::c_char;
    if checkunitname(unitname, linenum, file, errfile) != 0 {
        return 17 as libc::c_int;
    }
    pfxptr = plookup(unitname);
    if !pfxptr.is_null() && strcmp((*pfxptr).name, unitname) == 0 {
        if flags.unitcheck != 0 && !errfile.is_null() && redefine == 0 {
            fprintf(
                errfile,
                b"%s: prefix '%s-' defined on line %d of '%s' is redefined on line %d of '%s'.\n\0"
                    as *const u8 as *const libc::c_char,
                progname,
                unitname,
                (*pfxptr).linenumber,
                (*pfxptr).file,
                linenum,
                file,
            );
        }
        free((*pfxptr).value as *mut libc::c_void);
    } else {
        pfxptr = mymalloc(
            ::std::mem::size_of::<prefixlist>() as libc::c_ulong as libc::c_int,
            b"(newprefix)\0" as *const u8 as *const libc::c_char,
        ) as *mut prefixlist;
        (*pfxptr).name = dupstr(unitname);
        (*pfxptr).len = strlen(unitname) as libc::c_int;
        pval = (*unitname as libc::c_int & 127 as libc::c_int) as libc::c_uint;
        (*pfxptr).next = ptab[pval as usize];
        ptab[pval as usize] = pfxptr;
        *count += 1;
        *count;
    }
    (*pfxptr).value = dupstr(unitdef);
    (*pfxptr).linenumber = linenum;
    (*pfxptr).file = file;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn parsepair(
    mut input: *mut libc::c_char,
    mut first: *mut *mut libc::c_char,
    mut second: *mut *mut libc::c_char,
    mut firstopen: *mut libc::c_int,
    mut secondopen: *mut libc::c_int,
    mut delimiter: libc::c_char,
    mut checkopen: libc::c_int,
    mut unitname: *mut libc::c_char,
    mut linenum: libc::c_int,
    mut file: *mut libc::c_char,
    mut errfile: *mut FILE,
) -> *mut libc::c_char {
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut middle: *mut libc::c_char = 0 as *mut libc::c_char;
    start = strpbrk(
        input,
        if checkopen != 0 {
            b"[(\0" as *const u8 as *const libc::c_char
        } else {
            b"[\0" as *const u8 as *const libc::c_char
        },
    );
    if start.is_null() {
        if !errfile.is_null() {
            fprintf(
                errfile,
                b"%s: expecting '[' %s in definition of '%s' in '%s' line %d\n\0"
                    as *const u8 as *const libc::c_char,
                progname,
                if checkopen != 0 {
                    b"or '('\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                unitname,
                file,
                linenum,
            );
        }
        return 0 as *mut libc::c_char;
    }
    if *start as libc::c_int == '(' as i32 {
        *firstopen = 1 as libc::c_int;
    } else {
        *firstopen = 0 as libc::c_int;
    }
    let fresh5 = start;
    start = start.offset(1);
    *fresh5 = 0 as libc::c_int as libc::c_char;
    removespaces(input);
    if !(*input as libc::c_int == 0 as libc::c_int) {
        if !errfile.is_null() {
            fprintf(
                errfile,
                b"%s: unexpected characters before '%c' in definition of '%s' in '%s' line %d\n\0"
                    as *const u8 as *const libc::c_char,
                progname,
                if *firstopen != 0 { '(' as i32 } else { '[' as i32 },
                unitname,
                file,
                linenum,
            );
        }
        return 0 as *mut libc::c_char;
    }
    end = strpbrk(
        start,
        if checkopen != 0 {
            b"])\0" as *const u8 as *const libc::c_char
        } else {
            b"]\0" as *const u8 as *const libc::c_char
        },
    );
    if end.is_null() {
        if !errfile.is_null() {
            fprintf(
                errfile,
                b"%s: expecting ']' %s in definition of '%s' in '%s' line %d\n\0"
                    as *const u8 as *const libc::c_char,
                progname,
                if checkopen != 0 {
                    b"or ')'\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                unitname,
                file,
                linenum,
            );
        }
        return 0 as *mut libc::c_char;
    }
    if *end as libc::c_int == ')' as i32 {
        *secondopen = 1 as libc::c_int;
    } else {
        *secondopen = 0 as libc::c_int;
    }
    let fresh6 = end;
    end = end.offset(1);
    *fresh6 = 0 as libc::c_int as libc::c_char;
    middle = strchr(start, delimiter as libc::c_int);
    if !middle.is_null() {
        let fresh7 = middle;
        middle = middle.offset(1);
        *fresh7 = 0 as libc::c_int as libc::c_char;
        removespaces(middle);
        *second = middle;
    } else {
        *second = 0 as *mut libc::c_char;
    }
    removespaces(start);
    *first = start;
    return end;
}
pub unsafe extern "C" fn extract_interval(
    mut first: *mut libc::c_char,
    mut second: *mut libc::c_char,
    mut firstout: *mut *mut libc::c_double,
    mut secondout: *mut *mut libc::c_double,
) -> libc::c_int {
    let mut val: libc::c_double = 0.;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(*first as libc::c_int == 0 as libc::c_int) {
        val = strtod(first, &mut end);
        if *end != 0 {
            return 2 as libc::c_int
        } else {
            *firstout = mymalloc(
                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
                b"(extract_interval)\0" as *const u8 as *const libc::c_char,
            ) as *mut libc::c_double;
            **firstout = val;
        }
    }
    if !second.is_null() && *second as libc::c_int != 0 {
        val = strtod(second, &mut end);
        if *end != 0 {
            return 2 as libc::c_int
        } else if !(*firstout).is_null() && **firstout >= val {
            return 1 as libc::c_int
        } else {
            *secondout = mymalloc(
                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
                b"(extract_interval)\0" as *const u8 as *const libc::c_char,
            ) as *mut libc::c_double;
            **secondout = val;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn copyfunctype(mut dest: *mut functype, mut src: *mut functype) {
    (*dest).domain_min_open = (*src).domain_min_open;
    (*dest).domain_max_open = (*src).domain_max_open;
    (*dest).dimen = 0 as *mut libc::c_char;
    (*dest).def = (*dest).dimen;
    (*dest).param = (*dest).def;
    (*dest).domain_max = 0 as *mut libc::c_double;
    (*dest).domain_min = (*dest).domain_max;
    if !((*src).param).is_null() {
        (*dest).param = dupstr((*src).param);
    }
    if !((*src).def).is_null() {
        (*dest).def = dupstr((*src).def);
    }
    if !((*src).dimen).is_null() {
        (*dest).dimen = dupstr((*src).dimen);
    }
    if !((*src).domain_min).is_null() {
        (*dest)
            .domain_min = mymalloc(
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
            b"(copyfunctype)\0" as *const u8 as *const libc::c_char,
        ) as *mut libc::c_double;
        *(*dest).domain_min = *(*src).domain_min;
    }
    if !((*src).domain_max).is_null() {
        (*dest)
            .domain_max = mymalloc(
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
            b"(copyfunctype)\0" as *const u8 as *const libc::c_char,
        ) as *mut libc::c_double;
        *(*dest).domain_max = *(*src).domain_max;
    }
}
pub unsafe extern "C" fn copyfunction(
    mut unitname: *mut libc::c_char,
    mut funcname: *mut libc::c_char,
    mut count: *mut libc::c_int,
    mut linenum: libc::c_int,
    mut file: *mut libc::c_char,
    mut errfile: *mut FILE,
) -> libc::c_int {
    let mut source: *mut func = 0 as *mut func;
    let mut funcentry: *mut func = 0 as *mut func;
    let mut i: libc::c_int = 0;
    if checkunitname(unitname, linenum, file, errfile) != 0 {
        return 17 as libc::c_int;
    }
    removespaces(funcname);
    i = (strlen(funcname)).wrapping_sub(2 as libc::c_int as libc::c_ulong)
        as libc::c_int;
    if i > 0 as libc::c_int
        && strcmp(
            funcname.offset(i as isize),
            b"()\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        *funcname.offset(i as isize) = 0 as libc::c_int as libc::c_char;
    }
    source = fnlookup(funcname);
    if source.is_null() {
        if !errfile.is_null() {
            if (strpbrk(funcname, b" ;][()+*/-^\0" as *const u8 as *const libc::c_char))
                .is_null()
            {
                fprintf(
                    errfile,
                    b"%s: bad definition for '%s' in '%s' line %d, function '%s' not defined\n\0"
                        as *const u8 as *const libc::c_char,
                    progname,
                    unitname,
                    file,
                    linenum,
                    funcname,
                );
            } else {
                fprintf(
                    errfile,
                    b"%s: bad function definition of '%s' in '%s' line %d\n\0"
                        as *const u8 as *const libc::c_char,
                    progname,
                    unitname,
                    file,
                    linenum,
                );
            }
        }
        return 17 as libc::c_int;
    }
    funcentry = fnlookup(unitname);
    if !funcentry.is_null() {
        if flags.unitcheck != 0 && !errfile.is_null() {
            fprintf(
                errfile,
                b"%s: function '%s' defined on line %d of '%s' is redefined on line %d of '%s'.\n\0"
                    as *const u8 as *const libc::c_char,
                progname,
                unitname,
                (*funcentry).linenumber,
                (*funcentry).file,
                linenum,
                file,
            );
        }
        freefunction(funcentry);
    } else {
        funcentry = mymalloc(
            ::std::mem::size_of::<func>() as libc::c_ulong as libc::c_int,
            b"(newfunction)\0" as *const u8 as *const libc::c_char,
        ) as *mut func;
        (*funcentry).name = dupstr(unitname);
        addfunction(funcentry);
        *count += 1;
        *count;
    }
    (*funcentry).linenumber = linenum;
    (*funcentry).file = file;
    (*funcentry).skip_error_check = (*source).skip_error_check;
    if !((*source).table).is_null() {
        (*funcentry).tablelen = (*source).tablelen;
        (*funcentry).tableunit = dupstr((*source).tableunit);
        (*funcentry)
            .table = mymalloc(
            (::std::mem::size_of::<pair>() as libc::c_ulong)
                .wrapping_mul((*funcentry).tablelen as libc::c_ulong) as libc::c_int,
            b"(copyfunction)\0" as *const u8 as *const libc::c_char,
        ) as *mut pair;
        i = 0 as libc::c_int;
        while i < (*funcentry).tablelen {
            (*((*funcentry).table).offset(i as isize))
                .location = (*((*source).table).offset(i as isize)).location;
            (*((*funcentry).table).offset(i as isize))
                .value = (*((*source).table).offset(i as isize)).value;
            i += 1;
            i;
        }
    } else {
        (*funcentry).table = 0 as *mut pair;
        copyfunctype(&mut (*funcentry).forward, &mut (*source).forward);
        copyfunctype(&mut (*funcentry).inverse, &mut (*source).inverse);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn newfunction(
    mut unitname: *mut libc::c_char,
    mut unitdef: *mut libc::c_char,
    mut count: *mut libc::c_int,
    mut linenum: libc::c_int,
    mut file: *mut libc::c_char,
    mut errfile: *mut FILE,
    mut redefine: libc::c_int,
) -> libc::c_int {
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut inv: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut forward_dim: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut inverse_dim: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut first: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut second: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut domain_min: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut domain_max: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut range_min: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut range_max: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut funcentry: *mut func = 0 as *mut func;
    let mut looking_for_keywords: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut firstopen: libc::c_int = 0;
    let mut secondopen: libc::c_int = 0;
    let mut domain_min_open: libc::c_int = 0;
    let mut domain_max_open: libc::c_int = 0;
    let mut range_min_open: libc::c_int = 0;
    let mut range_max_open: libc::c_int = 0;
    let mut noerror: libc::c_int = 0 as libc::c_int;
    if *unitname as libc::c_int == '(' as i32 {
        if !errfile.is_null() {
            fprintf(
                errfile,
                b"%s: unit '%s' on line %d of '%s' ignored.  It starts with a '('\n\0"
                    as *const u8 as *const libc::c_char,
                progname,
                unitname,
                linenum,
                file,
            );
        }
        return 17 as libc::c_int;
    }
    start = strchr(unitname, '(' as i32);
    end = strchr(unitname, ')' as i32);
    let fresh8 = start;
    start = start.offset(1);
    *fresh8 = 0 as libc::c_int as libc::c_char;
    if checkunitname(unitname, linenum, file, errfile) != 0 {
        return 17 as libc::c_int;
    }
    if start == end {
        return copyfunction(unitname, unitdef, count, linenum, file, errfile);
    }
    if end.is_null() || strlen(end) > 1 as libc::c_int as libc::c_ulong {
        if !errfile.is_null() {
            fprintf(
                errfile,
                b"%s: bad function definition of '%s' in '%s' line %d\n\0" as *const u8
                    as *const libc::c_char,
                progname,
                unitname,
                file,
                linenum,
            );
        }
        return 17 as libc::c_int;
    }
    *end = 0 as libc::c_int as libc::c_char;
    forward_dim = 0 as *mut libc::c_char;
    inverse_dim = 0 as *mut libc::c_char;
    domain_min = 0 as *mut libc::c_double;
    domain_max = 0 as *mut libc::c_double;
    range_min = 0 as *mut libc::c_double;
    range_max = 0 as *mut libc::c_double;
    domain_min_open = 0 as libc::c_int;
    domain_max_open = 0 as libc::c_int;
    range_min_open = 0 as libc::c_int;
    range_max_open = 0 as libc::c_int;
    looking_for_keywords = 1 as libc::c_int;
    while looking_for_keywords != 0 {
        looking_for_keywords = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while !(fnkeywords[i as usize].word).is_null() {
            if strncmp(
                unitdef,
                fnkeywords[i as usize].word,
                strlen(fnkeywords[i as usize].word),
            ) == 0
            {
                looking_for_keywords = 1 as libc::c_int;
                unitdef = unitdef.offset(strlen(fnkeywords[i as usize].word) as isize);
                if fnkeywords[i as usize].checkopen != -(1 as libc::c_int) {
                    unitdef = parsepair(
                        unitdef,
                        &mut first,
                        &mut second,
                        &mut firstopen,
                        &mut secondopen,
                        fnkeywords[i as usize].delimit,
                        fnkeywords[i as usize].checkopen,
                        unitname,
                        linenum,
                        file,
                        errfile,
                    );
                    if unitdef.is_null() {
                        if !forward_dim.is_null() {
                            free(forward_dim as *mut libc::c_void);
                        }
                        if !inverse_dim.is_null() {
                            free(inverse_dim as *mut libc::c_void);
                        }
                        if !domain_min.is_null() {
                            free(domain_min as *mut libc::c_void);
                        }
                        if !domain_max.is_null() {
                            free(domain_max as *mut libc::c_void);
                        }
                        if !range_min.is_null() {
                            free(range_min as *mut libc::c_void);
                        }
                        if !range_max.is_null() {
                            free(range_max as *mut libc::c_void);
                        }
                        return 17 as libc::c_int;
                    }
                    removespaces(unitdef);
                }
                if i == 3 as libc::c_int {
                    noerror = 1 as libc::c_int;
                }
                if i == 0 as libc::c_int {
                    if !forward_dim.is_null() || !inverse_dim.is_null() {
                        if !errfile.is_null() {
                            fprintf(
                                errfile,
                                b"%s: keyword '%s' repeated in definition of '%s' on line %d of '%s'.\n\0"
                                    as *const u8 as *const libc::c_char,
                                progname,
                                fnkeywords[i as usize].word,
                                unitname,
                                linenum,
                                file,
                            );
                        }
                        return 17 as libc::c_int;
                    }
                    forward_dim = dupstr(first);
                    if !second.is_null() {
                        inverse_dim = dupstr(second);
                    }
                }
                if i == 1 as libc::c_int {
                    let mut err: libc::c_int = 0 as libc::c_int;
                    if !domain_min.is_null() || !domain_max.is_null() {
                        if !errfile.is_null() {
                            fprintf(
                                errfile,
                                b"%s: keyword '%s' repeated in definition of '%s' on line %d of '%s'.\n\0"
                                    as *const u8 as *const libc::c_char,
                                progname,
                                fnkeywords[i as usize].word,
                                unitname,
                                linenum,
                                file,
                            );
                        }
                        return 17 as libc::c_int;
                    }
                    err = extract_interval(
                        first,
                        second,
                        &mut domain_min,
                        &mut domain_max,
                    );
                    domain_min_open = firstopen;
                    domain_max_open = secondopen;
                    if err != 0 {
                        if !forward_dim.is_null() {
                            free(forward_dim as *mut libc::c_void);
                        }
                        if !inverse_dim.is_null() {
                            free(inverse_dim as *mut libc::c_void);
                        }
                        if !domain_min.is_null() {
                            free(domain_min as *mut libc::c_void);
                        }
                        if !domain_max.is_null() {
                            free(domain_max as *mut libc::c_void);
                        }
                        if !range_min.is_null() {
                            free(range_min as *mut libc::c_void);
                        }
                        if !range_max.is_null() {
                            free(range_max as *mut libc::c_void);
                        }
                    }
                    if err == 1 as libc::c_int {
                        if !errfile.is_null() {
                            fprintf(
                                errfile,
                                b"%s: second endpoint for domain must be greater than the first\n       in definition of '%s' in '%s' line %d\n\0"
                                    as *const u8 as *const libc::c_char,
                                progname,
                                unitname,
                                file,
                                linenum,
                            );
                        }
                        return 17 as libc::c_int;
                    }
                    if err == 2 as libc::c_int {
                        if !errfile.is_null() {
                            fprintf(
                                errfile,
                                b"%s: malformed domain in definition of '%s' in '%s' line %d\n\0"
                                    as *const u8 as *const libc::c_char,
                                progname,
                                unitname,
                                file,
                                linenum,
                            );
                        }
                        return 17 as libc::c_int;
                    }
                }
                if i == 2 as libc::c_int {
                    let mut err_0: libc::c_int = 0 as libc::c_int;
                    if !range_min.is_null() || !range_max.is_null() {
                        if !errfile.is_null() {
                            fprintf(
                                errfile,
                                b"%s: keyword '%s' repeated in definition of '%s' on line %d of '%s'.\n\0"
                                    as *const u8 as *const libc::c_char,
                                progname,
                                fnkeywords[i as usize].word,
                                unitname,
                                linenum,
                                file,
                            );
                        }
                        if !forward_dim.is_null() {
                            free(forward_dim as *mut libc::c_void);
                        }
                        if !inverse_dim.is_null() {
                            free(inverse_dim as *mut libc::c_void);
                        }
                        if !domain_min.is_null() {
                            free(domain_min as *mut libc::c_void);
                        }
                        if !domain_max.is_null() {
                            free(domain_max as *mut libc::c_void);
                        }
                        if !range_min.is_null() {
                            free(range_min as *mut libc::c_void);
                        }
                        if !range_max.is_null() {
                            free(range_max as *mut libc::c_void);
                        }
                        return 17 as libc::c_int;
                    }
                    err_0 = extract_interval(
                        first,
                        second,
                        &mut range_min,
                        &mut range_max,
                    );
                    range_min_open = firstopen;
                    range_max_open = secondopen;
                    if err_0 != 0 {
                        if !forward_dim.is_null() {
                            free(forward_dim as *mut libc::c_void);
                        }
                        if !inverse_dim.is_null() {
                            free(inverse_dim as *mut libc::c_void);
                        }
                        if !domain_min.is_null() {
                            free(domain_min as *mut libc::c_void);
                        }
                        if !domain_max.is_null() {
                            free(domain_max as *mut libc::c_void);
                        }
                        if !range_min.is_null() {
                            free(range_min as *mut libc::c_void);
                        }
                        if !range_max.is_null() {
                            free(range_max as *mut libc::c_void);
                        }
                    }
                    if err_0 == 1 as libc::c_int {
                        if !errfile.is_null() {
                            fprintf(
                                errfile,
                                b"%s: second endpoint for range must be greater than the first\n       in definition of '%s' in '%s' line %d\n\0"
                                    as *const u8 as *const libc::c_char,
                                progname,
                                unitname,
                                file,
                                linenum,
                            );
                        }
                        return 17 as libc::c_int;
                    }
                    if err_0 == 2 as libc::c_int {
                        if !errfile.is_null() {
                            fprintf(
                                errfile,
                                b"%s: malformed range in definition of '%s' in '%s' line %d\n\0"
                                    as *const u8 as *const libc::c_char,
                                progname,
                                unitname,
                                file,
                                linenum,
                            );
                        }
                        return 17 as libc::c_int;
                    }
                }
            }
            i += 1;
            i;
        }
    }
    if *unitdef as libc::c_int == 0 as libc::c_int {
        if !errfile.is_null() {
            fprintf(
                errfile,
                b"%s: function '%s' lacks a definition at line %d of '%s'\n\0"
                    as *const u8 as *const libc::c_char,
                progname,
                unitname,
                linenum,
                file,
            );
        }
        if !forward_dim.is_null() {
            free(forward_dim as *mut libc::c_void);
        }
        if !inverse_dim.is_null() {
            free(inverse_dim as *mut libc::c_void);
        }
        if !domain_min.is_null() {
            free(domain_min as *mut libc::c_void);
        }
        if !domain_max.is_null() {
            free(domain_max as *mut libc::c_void);
        }
        if !range_min.is_null() {
            free(range_min as *mut libc::c_void);
        }
        if !range_max.is_null() {
            free(range_max as *mut libc::c_void);
        }
        return 17 as libc::c_int;
    }
    if *unitdef as libc::c_int == '[' as i32 {
        if !errfile.is_null() {
            fprintf(
                errfile,
                b"%s: function '%s' missing keyword before '[' on line %d of '%s'\n\0"
                    as *const u8 as *const libc::c_char,
                progname,
                unitname,
                linenum,
                file,
            );
        }
        if !forward_dim.is_null() {
            free(forward_dim as *mut libc::c_void);
        }
        if !inverse_dim.is_null() {
            free(inverse_dim as *mut libc::c_void);
        }
        if !domain_min.is_null() {
            free(domain_min as *mut libc::c_void);
        }
        if !domain_max.is_null() {
            free(domain_max as *mut libc::c_void);
        }
        if !range_min.is_null() {
            free(range_min as *mut libc::c_void);
        }
        if !range_max.is_null() {
            free(range_max as *mut libc::c_void);
        }
        return 17 as libc::c_int;
    }
    if forward_dim.is_null()
        && (!domain_min.is_null() && *domain_min != 0.
            || !domain_max.is_null() && *domain_max != 0.)
    {
        if !errfile.is_null() {
            fprintf(
                errfile,
                b"%s: function '%s' defined on line %d of '%s' has domain with no units.\n\0"
                    as *const u8 as *const libc::c_char,
                progname,
                unitname,
                linenum,
                file,
            );
        }
        if !forward_dim.is_null() {
            free(forward_dim as *mut libc::c_void);
        }
        if !inverse_dim.is_null() {
            free(inverse_dim as *mut libc::c_void);
        }
        if !domain_min.is_null() {
            free(domain_min as *mut libc::c_void);
        }
        if !domain_max.is_null() {
            free(domain_max as *mut libc::c_void);
        }
        if !range_min.is_null() {
            free(range_min as *mut libc::c_void);
        }
        if !range_max.is_null() {
            free(range_max as *mut libc::c_void);
        }
        return 17 as libc::c_int;
    }
    if inverse_dim.is_null()
        && (!range_min.is_null() && *range_min != 0.
            || !range_max.is_null() && *range_max != 0.)
    {
        if !errfile.is_null() {
            fprintf(
                errfile,
                b"%s: function '%s' defined on line %d of '%s' has range with no units.\n\0"
                    as *const u8 as *const libc::c_char,
                progname,
                unitname,
                linenum,
                file,
            );
        }
        if !forward_dim.is_null() {
            free(forward_dim as *mut libc::c_void);
        }
        if !inverse_dim.is_null() {
            free(inverse_dim as *mut libc::c_void);
        }
        if !domain_min.is_null() {
            free(domain_min as *mut libc::c_void);
        }
        if !domain_max.is_null() {
            free(domain_max as *mut libc::c_void);
        }
        if !range_min.is_null() {
            free(range_min as *mut libc::c_void);
        }
        if !range_max.is_null() {
            free(range_max as *mut libc::c_void);
        }
        return 17 as libc::c_int;
    }
    funcentry = fnlookup(unitname);
    if !funcentry.is_null() {
        if flags.unitcheck != 0 && !errfile.is_null() && redefine == 0 {
            fprintf(
                errfile,
                b"%s: function '%s' defined on line %d of '%s' is redefined on line %d of '%s'.\n\0"
                    as *const u8 as *const libc::c_char,
                progname,
                unitname,
                (*funcentry).linenumber,
                (*funcentry).file,
                linenum,
                file,
            );
        }
        freefunction(funcentry);
    } else {
        funcentry = mymalloc(
            ::std::mem::size_of::<func>() as libc::c_ulong as libc::c_int,
            b"(newfunction)\0" as *const u8 as *const libc::c_char,
        ) as *mut func;
        (*funcentry).name = dupstr(unitname);
        addfunction(funcentry);
        *count += 1;
        *count;
    }
    (*funcentry).table = 0 as *mut pair;
    (*funcentry).skip_error_check = noerror;
    (*funcentry).forward.dimen = forward_dim;
    (*funcentry).inverse.dimen = inverse_dim;
    (*funcentry).forward.domain_min = domain_min;
    (*funcentry).forward.domain_max = domain_max;
    (*funcentry).inverse.domain_min = range_min;
    (*funcentry).inverse.domain_max = range_max;
    (*funcentry).forward.domain_min_open = domain_min_open;
    (*funcentry).forward.domain_max_open = domain_max_open;
    (*funcentry).inverse.domain_min_open = range_min_open;
    (*funcentry).inverse.domain_max_open = range_max_open;
    inv = strchr(unitdef, ';' as i32);
    if !inv.is_null() {
        let fresh9 = inv;
        inv = inv.offset(1);
        *fresh9 = 0 as libc::c_int as libc::c_char;
    }
    (*funcentry).forward.param = dupstr(start);
    removespaces(unitdef);
    (*funcentry).forward.def = dupstr(unitdef);
    if !inv.is_null() {
        removespaces(inv);
        (*funcentry).inverse.def = dupstr(inv);
        (*funcentry).inverse.param = dupstr(unitname);
    } else {
        (*funcentry).inverse.def = 0 as *mut libc::c_char;
        (*funcentry).inverse.param = 0 as *mut libc::c_char;
    }
    (*funcentry).linenumber = linenum;
    (*funcentry).file = file;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn newtable(
    mut unitname: *mut libc::c_char,
    mut unitdef: *mut libc::c_char,
    mut count: *mut libc::c_int,
    mut linenum: libc::c_int,
    mut file: *mut libc::c_char,
    mut errfile: *mut FILE,
    mut redefine: libc::c_int,
) -> libc::c_int {
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tableunit: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tablealloc: libc::c_int = 0;
    let mut tabpt: libc::c_int = 0;
    let mut tab: *mut pair = 0 as *mut pair;
    let mut funcentry: *mut func = 0 as *mut func;
    let mut noerror: libc::c_int = 0 as libc::c_int;
    tableunit = strchr(unitname, '[' as i32);
    end = strchr(unitname, ']' as i32);
    let fresh10 = tableunit;
    tableunit = tableunit.offset(1);
    *fresh10 = 0 as libc::c_int as libc::c_char;
    if checkunitname(unitname, linenum, file, errfile) != 0 {
        return 17 as libc::c_int;
    }
    if end.is_null() {
        if !errfile.is_null() {
            fprintf(
                errfile,
                b"%s: missing ']' in units file '%s' line %d\n\0" as *const u8
                    as *const libc::c_char,
                progname,
                file,
                linenum,
            );
        }
        return 17 as libc::c_int;
    }
    if strlen(end) > 1 as libc::c_int as libc::c_ulong {
        if !errfile.is_null() {
            fprintf(
                errfile,
                b"%s: unexpected characters after ']' in units file '%s' line %d\n\0"
                    as *const u8 as *const libc::c_char,
                progname,
                file,
                linenum,
            );
        }
        return 17 as libc::c_int;
    }
    *end = 0 as libc::c_int as libc::c_char;
    tab = mymalloc(
        (::std::mem::size_of::<pair>() as libc::c_ulong)
            .wrapping_mul(20 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"(newtable)\0" as *const u8 as *const libc::c_char,
    ) as *mut pair;
    tablealloc = 20 as libc::c_int;
    tabpt = 0 as libc::c_int;
    start = unitdef;
    if strncmp(
        start,
        b"noerror \0" as *const u8 as *const libc::c_char,
        strlen(b"noerror \0" as *const u8 as *const libc::c_char),
    ) == 0
    {
        noerror = 1 as libc::c_int;
        start = start
            .offset(strlen(b"noerror \0" as *const u8 as *const libc::c_char) as isize);
        removespaces(start);
    }
    loop {
        if tabpt >= tablealloc {
            tablealloc += 20 as libc::c_int;
            tab = realloc(
                tab as *mut libc::c_void,
                (::std::mem::size_of::<pair>() as libc::c_ulong)
                    .wrapping_mul(tablealloc as libc::c_ulong),
            ) as *mut pair;
            if tab.is_null() {
                if !errfile.is_null() {
                    fprintf(
                        errfile,
                        b"%s: memory allocation error (newtable)\n\0" as *const u8
                            as *const libc::c_char,
                        progname,
                    );
                }
                return 18 as libc::c_int;
            }
        }
        (*tab.offset(tabpt as isize)).location = strtod(start, &mut end);
        if start == end
            || !(*end as libc::c_int == 0 as libc::c_int)
                && *end as libc::c_int != ' ' as i32
        {
            if !(*start as libc::c_int == 0 as libc::c_int) {
                if strlen(start) > 15 as libc::c_int as libc::c_ulong {
                    *start
                        .offset(
                            15 as libc::c_int as isize,
                        ) = 0 as libc::c_int as libc::c_char;
                }
                if !errfile.is_null() {
                    fprintf(
                        errfile,
                        b"%s: cannot parse table definition %s at '%s' on line %d of '%s'\n\0"
                            as *const u8 as *const libc::c_char,
                        progname,
                        unitname,
                        start,
                        linenum,
                        file,
                    );
                }
                free(tab as *mut libc::c_void);
                return 17 as libc::c_int;
            }
            break;
        } else {
            if tabpt > 0 as libc::c_int
                && (*tab.offset(tabpt as isize)).location
                    <= (*tab.offset((tabpt - 1 as libc::c_int) as isize)).location
            {
                if !errfile.is_null() {
                    fprintf(
                        errfile,
                        b"%s: points don't increase (%.8g to %.8g) in units file '%s' line %d\n\0"
                            as *const u8 as *const libc::c_char,
                        progname,
                        (*tab.offset((tabpt - 1 as libc::c_int) as isize)).location,
                        (*tab.offset(tabpt as isize)).location,
                        file,
                        linenum,
                    );
                }
                free(tab as *mut libc::c_void);
                return 17 as libc::c_int;
            }
            start = end
                .offset(
                    strspn(end, b" \0" as *const u8 as *const libc::c_char) as isize,
                );
            (*tab.offset(tabpt as isize)).value = strtod(start, &mut end);
            if start == end {
                if !errfile.is_null() {
                    fprintf(
                        errfile,
                        b"%s: missing value after %.8g in units file '%s' line %d\n\0"
                            as *const u8 as *const libc::c_char,
                        progname,
                        (*tab.offset(tabpt as isize)).location,
                        file,
                        linenum,
                    );
                }
                free(tab as *mut libc::c_void);
                return 17 as libc::c_int;
            }
            tabpt += 1;
            tabpt;
            start = end
                .offset(
                    strspn(end, b" ,\0" as *const u8 as *const libc::c_char) as isize,
                );
        }
    }
    funcentry = fnlookup(unitname);
    if !funcentry.is_null() {
        if flags.unitcheck != 0 && !errfile.is_null() && redefine == 0 {
            fprintf(
                errfile,
                b"%s: unit '%s' defined on line %d of '%s' is redefined on line %d of '%s'.\n\0"
                    as *const u8 as *const libc::c_char,
                progname,
                unitname,
                (*funcentry).linenumber,
                (*funcentry).file,
                linenum,
                file,
            );
        }
        freefunction(funcentry);
    } else {
        funcentry = mymalloc(
            ::std::mem::size_of::<func>() as libc::c_ulong as libc::c_int,
            b"(newtable)\0" as *const u8 as *const libc::c_char,
        ) as *mut func;
        (*funcentry).name = dupstr(unitname);
        addfunction(funcentry);
        *count += 1;
        *count;
    }
    (*funcentry).tableunit = dupstr(tableunit);
    (*funcentry).tablelen = tabpt;
    (*funcentry).table = tab;
    (*funcentry).skip_error_check = noerror;
    (*funcentry).linenumber = linenum;
    (*funcentry).file = file;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn newalias(
    mut unitname: *mut libc::c_char,
    mut unitdef: *mut libc::c_char,
    mut linenum: libc::c_int,
    mut file: *mut libc::c_char,
    mut errfile: *mut FILE,
) -> libc::c_int {
    let mut aliasentry: *mut wantalias = 0 as *mut wantalias;
    if (strchr(unitdef, ';' as i32)).is_null() {
        if !errfile.is_null() {
            fprintf(
                errfile,
                b"%s: unit list missing '%c' on line %d of '%s'\n\0" as *const u8
                    as *const libc::c_char,
                progname,
                ';' as i32,
                linenum,
                file,
            );
        }
        return 17 as libc::c_int;
    }
    aliasentry = aliaslookup(unitname);
    if !aliasentry.is_null() {
        if flags.unitcheck != 0 && !errfile.is_null() {
            fprintf(
                errfile,
                b"%s: unit list '%s' defined on line %d of '%s' is redefined on line %d of '%s'.\n\0"
                    as *const u8 as *const libc::c_char,
                progname,
                unitname,
                (*aliasentry).linenumber,
                (*aliasentry).file,
                linenum,
                file,
            );
        }
        free((*aliasentry).definition as *mut libc::c_void);
    } else {
        aliasentry = mymalloc(
            ::std::mem::size_of::<wantalias>() as libc::c_ulong as libc::c_int,
            b"(newalias)\0" as *const u8 as *const libc::c_char,
        ) as *mut wantalias;
        (*aliasentry).name = dupstr(unitname);
        (*aliasentry).next = 0 as *mut wantalias;
        *aliaslistend = aliasentry;
        aliaslistend = &mut (*aliasentry).next;
    }
    (*aliasentry).definition = dupstr(unitdef);
    (*aliasentry).linenumber = linenum;
    (*aliasentry).file = file;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn checkvar(
    mut name: *mut libc::c_char,
    mut list: *mut libc::c_char,
) -> libc::c_int {
    let mut listitem: *mut libc::c_char = 0 as *mut libc::c_char;
    name = getenv(name);
    if name.is_null() {
        return 2 as libc::c_int;
    }
    listitem = strtok(list, b" \0" as *const u8 as *const libc::c_char);
    while !listitem.is_null() {
        if strcmp(name, listitem) == 0 {
            return 1 as libc::c_int;
        }
        listitem = strtok(
            0 as *mut libc::c_char,
            b" \0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn pathend(mut filename: *mut libc::c_char) -> *mut libc::c_char {
    let mut pointer: *mut libc::c_char = 0 as *mut libc::c_char;
    pointer = filename.offset(strlen(filename) as isize);
    while pointer > filename {
        if *pointer as libc::c_int == '/' as i32 {
            pointer = pointer.offset(1);
            pointer;
            break;
        } else {
            pointer = pointer.offset(-1);
            pointer;
        }
    }
    return pointer;
}
pub unsafe extern "C" fn isfullpath(mut path: *mut libc::c_char) -> libc::c_int {
    return (*path as libc::c_int == '/' as i32) as libc::c_int;
}
pub unsafe extern "C" fn readunits(
    mut file: *mut libc::c_char,
    mut errfile: *mut FILE,
    mut unitcount: *mut libc::c_int,
    mut prefixcount: *mut libc::c_int,
    mut funccount: *mut libc::c_int,
    mut depth: libc::c_int,
) -> libc::c_int {
    let mut unitfile: *mut FILE = 0 as *mut FILE;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lineptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut unitdef: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut unitname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut permfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut linenum: libc::c_int = 0;
    let mut linebufsize: libc::c_int = 0;
    let mut goterr: libc::c_int = 0;
    let mut retcode: libc::c_int = 0;
    let mut locunitcount: libc::c_int = 0;
    let mut locprefixcount: libc::c_int = 0;
    let mut locfunccount: libc::c_int = 0;
    let mut redefinition: libc::c_int = 0;
    let mut wronglocale: libc::c_int = 0 as libc::c_int;
    let mut inlocale: libc::c_int = 0 as libc::c_int;
    let mut in_utf8: libc::c_int = 0 as libc::c_int;
    let mut invar: libc::c_int = 0 as libc::c_int;
    let mut wrongvar: libc::c_int = 0 as libc::c_int;
    locunitcount = 0 as libc::c_int;
    locprefixcount = 0 as libc::c_int;
    locfunccount = 0 as libc::c_int;
    linenum = 0 as libc::c_int;
    linebufsize = 0 as libc::c_int;
    goterr = 0 as libc::c_int;
    unitfile = openfile(
        file,
        b"rt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if unitfile.is_null() {
        if !errfile.is_null() {
            fprintf(
                errfile,
                b"%s: Unable to read units file '%s': %s\n\0" as *const u8
                    as *const libc::c_char,
                progname,
                file,
                strerror(*__errno_location()),
            );
        }
        return 16 as libc::c_int;
    }
    growbuffer(&mut line, &mut linebufsize);
    permfile = dupstr(file);
    while feof(unitfile) == 0 {
        if (fgetslong(&mut line, &mut linebufsize, unitfile, &mut linenum)).is_null() {
            break;
        }
        if linenum == 1 as libc::c_int
            && strncmp(
                line,
                b"\xEF\xBB\xBF\0" as *const u8 as *const libc::c_char,
                strlen(b"\xEF\xBB\xBF\0" as *const u8 as *const libc::c_char),
            ) == 0
        {
            let mut i: libc::c_int = 0;
            lineptr = line;
            i = 0 as libc::c_int;
            while (i as libc::c_ulong)
                < strlen(b"\xEF\xBB\xBF\0" as *const u8 as *const libc::c_char)
            {
                *lineptr = ' ' as i32 as libc::c_char;
                i += 1;
                i;
                lineptr = lineptr.offset(1);
                lineptr;
            }
        }
        strip_comment(line);
        if -(1 as libc::c_int) == strwidth(line) {
            goterr = 1 as libc::c_int;
            (goterr != 0 && !errfile.is_null()
                && fprintf(
                    errfile,
                    b"%s: %s on line %d of '%s'\n\0" as *const u8 as *const libc::c_char,
                    progname,
                    invalid_utf8,
                    linenum,
                    file,
                ) != 0) as libc::c_int;
        } else {
            replace_minus(line);
            if *line as libc::c_int == '!' as i32 {
                unitname = strtok(
                    line.offset(1 as libc::c_int as isize),
                    b" \0" as *const u8 as *const libc::c_char,
                );
                if unitname.is_null() {
                    goterr = 1 as libc::c_int;
                    (goterr != 0 && !errfile.is_null()
                        && fprintf(
                            errfile,
                            b"%s: error in units file '%s' line %d\n\0" as *const u8
                                as *const libc::c_char,
                            progname,
                            file,
                            linenum,
                        ) != 0) as libc::c_int;
                } else if strcmp(unitname, b"var\0" as *const u8 as *const libc::c_char)
                    == 0
                    || strcmp(unitname, b"varnot\0" as *const u8 as *const libc::c_char)
                        == 0
                {
                    let mut not: libc::c_int = 0 as libc::c_int;
                    if *unitname.offset(3 as libc::c_int as isize) as libc::c_int
                        == 'n' as i32
                    {
                        not = 1 as libc::c_int;
                    }
                    unitname = strtok(
                        0 as *mut libc::c_char,
                        b" \0" as *const u8 as *const libc::c_char,
                    );
                    unitdef = strtok(
                        0 as *mut libc::c_char,
                        b"\0" as *const u8 as *const libc::c_char,
                    );
                    if unitname.is_null() {
                        goterr = 1 as libc::c_int;
                        (goterr != 0 && !errfile.is_null()
                            && fprintf(
                                errfile,
                                b"%s: no variable name specified on line %d of '%s'\n\0"
                                    as *const u8 as *const libc::c_char,
                                progname,
                                linenum,
                                file,
                            ) != 0) as libc::c_int;
                    } else if unitdef.is_null() {
                        goterr = 1 as libc::c_int;
                        (goterr != 0 && !errfile.is_null()
                            && fprintf(
                                errfile,
                                b"%s: no value specified on line %d of '%s'\n\0"
                                    as *const u8 as *const libc::c_char,
                                progname,
                                linenum,
                                file,
                            ) != 0) as libc::c_int;
                    } else if invar != 0 {
                        goterr = 1 as libc::c_int;
                        (goterr != 0 && !errfile.is_null()
                            && fprintf(
                                errfile,
                                b"%s: nested var statements not allowed, line %d of '%s'\n\0"
                                    as *const u8 as *const libc::c_char,
                                progname,
                                linenum,
                                file,
                            ) != 0) as libc::c_int;
                    } else {
                        let mut check: libc::c_int = 0;
                        invar = 1 as libc::c_int;
                        check = checkvar(unitname, unitdef);
                        if check == 2 as libc::c_int {
                            goterr = 1 as libc::c_int;
                            (goterr != 0 && !errfile.is_null()
                                && fprintf(
                                    errfile,
                                    b"%s: environment variable %s not set at line %d of '%s'\n\0"
                                        as *const u8 as *const libc::c_char,
                                    progname,
                                    unitname,
                                    linenum,
                                    file,
                                ) != 0) as libc::c_int;
                            wrongvar = 1 as libc::c_int;
                        } else if not ^ check == 0 {
                            wrongvar = 1 as libc::c_int;
                        }
                    }
                } else if strcmp(
                    unitname,
                    b"endvar\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    if invar == 0 {
                        goterr = 1 as libc::c_int;
                        (goterr != 0 && !errfile.is_null()
                            && fprintf(
                                errfile,
                                b"%s: unmatched !endvar on line %d of '%s'\n\0" as *const u8
                                    as *const libc::c_char,
                                progname,
                                linenum,
                                file,
                            ) != 0) as libc::c_int;
                    }
                    wrongvar = 0 as libc::c_int;
                    invar = 0 as libc::c_int;
                } else if strcmp(
                    unitname,
                    b"locale\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    unitname = strtok(
                        0 as *mut libc::c_char,
                        b" \0" as *const u8 as *const libc::c_char,
                    );
                    if unitname.is_null() {
                        goterr = 1 as libc::c_int;
                        (goterr != 0 && !errfile.is_null()
                            && fprintf(
                                errfile,
                                b"%s: no locale specified on line %d of '%s'\n\0"
                                    as *const u8 as *const libc::c_char,
                                progname,
                                linenum,
                                file,
                            ) != 0) as libc::c_int;
                    } else if inlocale != 0 {
                        goterr = 1 as libc::c_int;
                        (goterr != 0 && !errfile.is_null()
                            && fprintf(
                                errfile,
                                b"%s: nested locales not allowed, line %d of '%s'\n\0"
                                    as *const u8 as *const libc::c_char,
                                progname,
                                linenum,
                                file,
                            ) != 0) as libc::c_int;
                    } else {
                        inlocale = 1 as libc::c_int;
                        if strcmp(unitname, mylocale) != 0 {
                            wronglocale = 1 as libc::c_int;
                        }
                    }
                } else if strcmp(
                    unitname,
                    b"endlocale\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    if inlocale == 0 {
                        goterr = 1 as libc::c_int;
                        (goterr != 0 && !errfile.is_null()
                            && fprintf(
                                errfile,
                                b"%s: unmatched !endlocale on line %d of '%s'\n\0"
                                    as *const u8 as *const libc::c_char,
                                progname,
                                linenum,
                                file,
                            ) != 0) as libc::c_int;
                    }
                    wronglocale = 0 as libc::c_int;
                    inlocale = 0 as libc::c_int;
                } else if strcmp(unitname, b"utf8\0" as *const u8 as *const libc::c_char)
                    == 0
                {
                    if in_utf8 != 0 {
                        goterr = 1 as libc::c_int;
                        (goterr != 0 && !errfile.is_null()
                            && fprintf(
                                errfile,
                                b"%s: nested utf8 not allowed, line %d of '%s'\n\0"
                                    as *const u8 as *const libc::c_char,
                                progname,
                                linenum,
                                file,
                            ) != 0) as libc::c_int;
                    } else {
                        in_utf8 = 1 as libc::c_int;
                    }
                } else if strcmp(
                    unitname,
                    b"endutf8\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    if in_utf8 == 0 {
                        goterr = 1 as libc::c_int;
                        (goterr != 0 && !errfile.is_null()
                            && fprintf(
                                errfile,
                                b"%s: unmatched !endutf8 on line %d of '%s'\n\0"
                                    as *const u8 as *const libc::c_char,
                                progname,
                                linenum,
                                file,
                            ) != 0) as libc::c_int;
                    }
                    in_utf8 = 0 as libc::c_int;
                } else {
                    if in_utf8 != 0 && utf8mode == 0 {
                        continue;
                    }
                    if wronglocale != 0 || wrongvar != 0 {
                        continue;
                    }
                    if strcmp(unitname, b"prompt\0" as *const u8 as *const libc::c_char)
                        == 0
                    {
                        unitname = strtok(
                            0 as *mut libc::c_char,
                            b"\0" as *const u8 as *const libc::c_char,
                        );
                        if !promptprefix.is_null() {
                            free(promptprefix as *mut libc::c_void);
                        }
                        if unitname.is_null() {
                            promptprefix = 0 as *mut libc::c_char;
                        } else {
                            promptprefix = dupstr(unitname);
                        }
                    } else if strcmp(
                        unitname,
                        b"message\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        unitname = strtok(
                            0 as *mut libc::c_char,
                            b"\0" as *const u8 as *const libc::c_char,
                        );
                        if flags.quiet == 0 {
                            if !unitname.is_null() {
                                logputs(unitname);
                            }
                            logputchar('\n' as i32 as libc::c_char);
                        }
                    } else if strcmp(
                        unitname,
                        b"set\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        unitname = strtok(
                            0 as *mut libc::c_char,
                            b" \0" as *const u8 as *const libc::c_char,
                        );
                        unitdef = strtok(
                            0 as *mut libc::c_char,
                            b" \0" as *const u8 as *const libc::c_char,
                        );
                        if unitname.is_null() {
                            goterr = 1 as libc::c_int;
                            (goterr != 0 && !errfile.is_null()
                                && fprintf(
                                    errfile,
                                    b"%s: no variable name specified on line %d of '%s'\n\0"
                                        as *const u8 as *const libc::c_char,
                                    progname,
                                    linenum,
                                    file,
                                ) != 0) as libc::c_int;
                        } else if unitdef.is_null() {
                            goterr = 1 as libc::c_int;
                            (goterr != 0 && !errfile.is_null()
                                && fprintf(
                                    errfile,
                                    b"%s: no value specified on line %d of '%s'\n\0"
                                        as *const u8 as *const libc::c_char,
                                    progname,
                                    linenum,
                                    file,
                                ) != 0) as libc::c_int;
                        } else {
                            setenv(unitname, unitdef, 0 as libc::c_int);
                        }
                    } else if strcmp(
                        unitname,
                        b"unitlist\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        splitline(0 as *mut libc::c_char, &mut unitname, &mut unitdef);
                        if unitname.is_null() || unitdef.is_null() {
                            goterr = 1 as libc::c_int;
                            (goterr != 0 && !errfile.is_null()
                                && fprintf(
                                    errfile,
                                    b"%s: error in units file '%s' line %d\n\0" as *const u8
                                        as *const libc::c_char,
                                    progname,
                                    file,
                                    linenum,
                                ) != 0) as libc::c_int;
                        } else if newalias(unitname, unitdef, linenum, permfile, errfile)
                            != 0
                        {
                            goterr = 1 as libc::c_int;
                        }
                    } else if strcmp(
                        unitname,
                        b"include\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        if depth > 5 as libc::c_int {
                            goterr = 1 as libc::c_int;
                            (goterr != 0 && !errfile.is_null()
                                && fprintf(
                                    errfile,
                                    b"%s: max include depth of %d exceeded in file '%s' line %d\n\0"
                                        as *const u8 as *const libc::c_char,
                                    progname,
                                    5 as libc::c_int,
                                    file,
                                    linenum,
                                ) != 0) as libc::c_int;
                        } else {
                            let mut readerr: libc::c_int = 0;
                            let mut includefile: *mut libc::c_char = 0
                                as *mut libc::c_char;
                            unitname = strtok(
                                0 as *mut libc::c_char,
                                b" \0" as *const u8 as *const libc::c_char,
                            );
                            if unitname.is_null() {
                                goterr = 1 as libc::c_int;
                                (goterr != 0 && !errfile.is_null()
                                    && fprintf(
                                        errfile,
                                        b"%s: missing include filename on line %d of '%s'\n\0"
                                            as *const u8 as *const libc::c_char,
                                        progname,
                                        linenum,
                                        file,
                                    ) != 0) as libc::c_int;
                            } else {
                                includefile = mymalloc(
                                    (strlen(file))
                                        .wrapping_add(strlen(unitname))
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        as libc::c_int,
                                    b"(readunits)\0" as *const u8 as *const libc::c_char,
                                ) as *mut libc::c_char;
                                if isfullpath(unitname) != 0 {
                                    strcpy(includefile, unitname);
                                } else {
                                    strcpy(includefile, file);
                                    strcpy(pathend(includefile), unitname);
                                }
                                readerr = readunits(
                                    includefile,
                                    errfile,
                                    unitcount,
                                    prefixcount,
                                    funccount,
                                    depth + 1 as libc::c_int,
                                );
                                if readerr == 18 as libc::c_int {
                                    fclose(unitfile);
                                    free(line as *mut libc::c_void);
                                    free(includefile as *mut libc::c_void);
                                    return readerr;
                                }
                                if readerr == 16 as libc::c_int {
                                    goterr = 1 as libc::c_int;
                                    (goterr != 0 && !errfile.is_null()
                                        && fprintf(
                                            errfile,
                                            b"%s:   file was included at line %d of file '%s'\n\0"
                                                as *const u8 as *const libc::c_char,
                                            progname,
                                            linenum,
                                            file,
                                        ) != 0) as libc::c_int;
                                }
                                if readerr != 0 {
                                    goterr = 1 as libc::c_int;
                                }
                                free(includefile as *mut libc::c_void);
                            }
                        }
                    } else {
                        goterr = 1 as libc::c_int;
                        (goterr != 0 && !errfile.is_null()
                            && fprintf(
                                errfile,
                                b"%s: error in units file '%s' line %d\n\0" as *const u8
                                    as *const libc::c_char,
                                progname,
                                file,
                                linenum,
                            ) != 0) as libc::c_int;
                    }
                }
            } else {
                if in_utf8 != 0 && utf8mode == 0 {
                    continue;
                }
                if wronglocale != 0 || wrongvar != 0 {
                    continue;
                }
                splitline(line, &mut unitname, &mut unitdef);
                if unitname.is_null() {
                    continue;
                }
                if *unitname as libc::c_int == '+' as i32 {
                    unitname = unitname.offset(1);
                    unitname;
                    redefinition = 1 as libc::c_int;
                    if strlen(unitname) == 0 as libc::c_int as libc::c_ulong {
                        goterr = 1 as libc::c_int;
                        (goterr != 0 && !errfile.is_null()
                            && fprintf(
                                errfile,
                                b"%s: expecting name of unit to redefine after '+' at line %d of '%s'\n\0"
                                    as *const u8 as *const libc::c_char,
                                progname,
                                linenum,
                                file,
                            ) != 0) as libc::c_int;
                        continue;
                    }
                } else {
                    redefinition = 0 as libc::c_int;
                }
                if strcmp(unitname, b"-\0" as *const u8 as *const libc::c_char) == 0 {
                    goterr = 1 as libc::c_int;
                    (goterr != 0 && !errfile.is_null()
                        && fprintf(
                            errfile,
                            b"%s: expecting prefix name before '-' at line %d of '%s'\n\0"
                                as *const u8 as *const libc::c_char,
                            progname,
                            linenum,
                            file,
                        ) != 0) as libc::c_int;
                } else if unitdef.is_null() {
                    goterr = 1 as libc::c_int;
                    (goterr != 0 && !errfile.is_null()
                        && fprintf(
                            errfile,
                            b"%s: unit '%s' lacks a definition at line %d of '%s'\n\0"
                                as *const u8 as *const libc::c_char,
                            progname,
                            unitname,
                            linenum,
                            file,
                        ) != 0) as libc::c_int;
                } else if *unitname
                    .offset(strlen(unitname) as isize)
                    .offset(-(1 as libc::c_int as isize)) as libc::c_int == '-' as i32
                {
                    if newprefix(
                        unitname,
                        unitdef,
                        &mut locprefixcount,
                        linenum,
                        permfile,
                        errfile,
                        redefinition,
                    ) != 0
                    {
                        goterr = 1 as libc::c_int;
                    }
                } else if !(strchr(unitname, '[' as i32)).is_null() {
                    retcode = newtable(
                        unitname,
                        unitdef,
                        &mut locfunccount,
                        linenum,
                        permfile,
                        errfile,
                        redefinition,
                    );
                    if retcode != 0 {
                        if retcode != 17 as libc::c_int {
                            fclose(unitfile);
                            free(line as *mut libc::c_void);
                            return retcode;
                        }
                        goterr = 1 as libc::c_int;
                    }
                } else if !(strchr(unitname, '(' as i32)).is_null() {
                    if newfunction(
                        unitname,
                        unitdef,
                        &mut locfunccount,
                        linenum,
                        permfile,
                        errfile,
                        redefinition,
                    ) != 0
                    {
                        goterr = 1 as libc::c_int;
                    }
                } else if newunit(
                    unitname,
                    unitdef,
                    &mut locunitcount,
                    linenum,
                    permfile,
                    errfile,
                    redefinition,
                ) != 0
                {
                    goterr = 1 as libc::c_int;
                }
            }
        }
    }
    fclose(unitfile);
    free(line as *mut libc::c_void);
    if !unitcount.is_null() {
        *unitcount += locunitcount;
    }
    if !prefixcount.is_null() {
        *prefixcount += locprefixcount;
    }
    if !funccount.is_null() {
        *funccount += locfunccount;
    }
    if goterr != 0 { return 17 as libc::c_int } else { return 0 as libc::c_int };
}
pub unsafe extern "C" fn initializeunit(mut theunit: *mut unittype) {
    (*theunit).factor = 1.0f64;
    (*theunit).denominator[0 as libc::c_int as usize] = 0 as *mut libc::c_char;
    (*theunit)
        .numerator[0 as libc::c_int
        as usize] = (*theunit).denominator[0 as libc::c_int as usize];
}
pub unsafe extern "C" fn freeunit(mut theunit: *mut unittype) {
    let mut ptr: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    ptr = ((*theunit).numerator).as_mut_ptr();
    while !(*ptr).is_null() {
        if *ptr != NULLUNIT {
            free(*ptr as *mut libc::c_void);
        }
        ptr = ptr.offset(1);
        ptr;
    }
    ptr = ((*theunit).denominator).as_mut_ptr();
    while !(*ptr).is_null() {
        if *ptr != NULLUNIT {
            free(*ptr as *mut libc::c_void);
        }
        ptr = ptr.offset(1);
        ptr;
    }
    (*theunit).numerator[0 as libc::c_int as usize] = 0 as *mut libc::c_char;
    (*theunit).denominator[0 as libc::c_int as usize] = 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn showunit(mut theunit: *mut unittype) {
    let mut ptr: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut printedslash: libc::c_int = 0;
    let mut counter: libc::c_int = 1 as libc::c_int;
    logprintf(num_format.format, (*theunit).factor);
    ptr = ((*theunit).numerator).as_mut_ptr();
    while !(*ptr).is_null() {
        if ptr > ((*theunit).numerator).as_mut_ptr() && **ptr as libc::c_int != 0
            && strcmp(*ptr, *ptr.offset(-(1 as libc::c_int as isize))) == 0
        {
            counter += 1;
            counter;
        } else {
            if counter > 1 as libc::c_int {
                logprintf(
                    b"%s%d\0" as *const u8 as *const libc::c_char,
                    powerstring,
                    counter,
                );
            }
            if **ptr != 0 {
                logprintf(b" %s\0" as *const u8 as *const libc::c_char, *ptr);
            }
            counter = 1 as libc::c_int;
        }
        ptr = ptr.offset(1);
        ptr;
    }
    if counter > 1 as libc::c_int {
        logprintf(b"%s%d\0" as *const u8 as *const libc::c_char, powerstring, counter);
    }
    counter = 1 as libc::c_int;
    printedslash = 0 as libc::c_int;
    ptr = ((*theunit).denominator).as_mut_ptr();
    while !(*ptr).is_null() {
        if ptr > ((*theunit).denominator).as_mut_ptr() && **ptr as libc::c_int != 0
            && strcmp(*ptr, *ptr.offset(-(1 as libc::c_int as isize))) == 0
        {
            counter += 1;
            counter;
        } else {
            if counter > 1 as libc::c_int {
                logprintf(
                    b"%s%d\0" as *const u8 as *const libc::c_char,
                    powerstring,
                    counter,
                );
            }
            if **ptr != 0 {
                if printedslash == 0 {
                    logprintf(b" /\0" as *const u8 as *const libc::c_char);
                }
                printedslash = 1 as libc::c_int;
                logprintf(b" %s\0" as *const u8 as *const libc::c_char, *ptr);
            }
            counter = 1 as libc::c_int;
        }
        ptr = ptr.offset(1);
        ptr;
    }
    if counter > 1 as libc::c_int {
        logprintf(b"%s%d\0" as *const u8 as *const libc::c_char, powerstring, counter);
    }
}
pub unsafe extern "C" fn compare(
    mut item1: *const libc::c_void,
    mut item2: *const libc::c_void,
) -> libc::c_int {
    return strcmp(
        *(item1 as *mut *mut libc::c_char),
        *(item2 as *mut *mut libc::c_char),
    );
}
pub unsafe extern "C" fn sortunit(mut theunit: *mut unittype) {
    let mut ptr: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut count: libc::c_int = 0;
    count = 0 as libc::c_int;
    ptr = ((*theunit).numerator).as_mut_ptr();
    while !(*ptr).is_null() {
        ptr = ptr.offset(1);
        ptr;
        count += 1;
        count;
    }
    qsort(
        ((*theunit).numerator).as_mut_ptr() as *mut libc::c_void,
        count as size_t,
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        Some(
            compare
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    count = 0 as libc::c_int;
    ptr = ((*theunit).denominator).as_mut_ptr();
    while !(*ptr).is_null() {
        ptr = ptr.offset(1);
        ptr;
        count += 1;
        count;
    }
    qsort(
        ((*theunit).denominator).as_mut_ptr() as *mut libc::c_void,
        count as size_t,
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        Some(
            compare
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
}
pub unsafe extern "C" fn cancelunit(mut theunit: *mut unittype) {
    let mut den: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut num: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut comp: libc::c_int = 0;
    den = ((*theunit).denominator).as_mut_ptr();
    num = ((*theunit).numerator).as_mut_ptr();
    while !(*num).is_null() && !(*den).is_null() {
        comp = strcmp(*den, *num);
        if comp == 0 {
            if *den != NULLUNIT {
                free(*den as *mut libc::c_void);
            }
            if *num != NULLUNIT {
                free(*num as *mut libc::c_void);
            }
            let fresh11 = den;
            den = den.offset(1);
            *fresh11 = NULLUNIT;
            let fresh12 = num;
            num = num.offset(1);
            *fresh12 = NULLUNIT;
        } else if comp < 0 as libc::c_int {
            den = den.offset(1);
            den;
        } else {
            num = num.offset(1);
            num;
        }
    }
}
static mut bufsize: libc::c_int = 0 as libc::c_int;
static mut buffer: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub unsafe extern "C" fn lookupunit(
    mut unit: *mut libc::c_char,
    mut prefixok: libc::c_int,
) -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pfxptr: *mut prefixlist = 0 as *mut prefixlist;
    let mut uptr: *mut unitlist = 0 as *mut unitlist;
    uptr = ulookup(unit);
    if !uptr.is_null() {
        return (*uptr).value;
    }
    if strwidth(unit) > 2 as libc::c_int
        && *unit.offset(strlen(unit) as isize).offset(-(1 as libc::c_int as isize))
            as libc::c_int == 's' as i32
    {
        copy = dupstr(unit);
        *copy
            .offset(strlen(copy) as isize)
            .offset(-(1 as libc::c_int as isize)) = 0 as libc::c_int as libc::c_char;
        if !(lookupunit(copy, prefixok)).is_null() {
            while (strlen(copy)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                > bufsize as libc::c_ulong
            {
                growbuffer(&mut buffer, &mut bufsize);
            }
            strcpy(buffer, copy);
            free(copy as *mut libc::c_void);
            return buffer;
        }
        if strlen(copy) > 2 as libc::c_int as libc::c_ulong
            && *copy.offset(strlen(copy) as isize).offset(-(1 as libc::c_int as isize))
                as libc::c_int == 'e' as i32
        {
            *copy
                .offset(strlen(copy) as isize)
                .offset(-(1 as libc::c_int as isize)) = 0 as libc::c_int as libc::c_char;
            if !(lookupunit(copy, prefixok)).is_null() {
                while (strlen(copy)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                    > bufsize as libc::c_ulong
                {
                    growbuffer(&mut buffer, &mut bufsize);
                }
                strcpy(buffer, copy);
                free(copy as *mut libc::c_void);
                return buffer;
            }
        }
        if strlen(copy) > 2 as libc::c_int as libc::c_ulong
            && *copy.offset(strlen(copy) as isize).offset(-(1 as libc::c_int as isize))
                as libc::c_int == 'i' as i32
        {
            *copy
                .offset(strlen(copy) as isize)
                .offset(-(1 as libc::c_int as isize)) = 'y' as i32 as libc::c_char;
            if !(lookupunit(copy, prefixok)).is_null() {
                while (strlen(copy)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                    > bufsize as libc::c_ulong
                {
                    growbuffer(&mut buffer, &mut bufsize);
                }
                strcpy(buffer, copy);
                free(copy as *mut libc::c_void);
                return buffer;
            }
        }
        free(copy as *mut libc::c_void);
    }
    if prefixok != 0
        && {
            pfxptr = plookup(unit);
            !pfxptr.is_null()
        }
    {
        copy = unit.offset((*pfxptr).len as isize);
        if *copy as libc::c_int == 0 as libc::c_int
            || !(lookupunit(copy, 0 as libc::c_int)).is_null()
        {
            let mut tempbuf: *mut libc::c_char = 0 as *mut libc::c_char;
            while (strlen((*pfxptr).value))
                .wrapping_add(strlen(copy))
                .wrapping_add(2 as libc::c_int as libc::c_ulong)
                > bufsize as libc::c_ulong
            {
                growbuffer(&mut buffer, &mut bufsize);
            }
            tempbuf = dupstr(copy);
            strcpy(buffer, (*pfxptr).value);
            strcat(buffer, b" \0" as *const u8 as *const libc::c_char);
            strcat(buffer, tempbuf);
            free(tempbuf as *mut libc::c_void);
            return buffer;
        }
    }
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn moveproduct(
    mut product: *mut *mut libc::c_char,
    mut tomove: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut dest: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut src: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    dest = product;
    src = tomove;
    while !(*src).is_null() {
        if !(*src == NULLUNIT) {
            while !(*dest).is_null() && *dest != NULLUNIT {
                dest = dest.offset(1);
                dest;
            }
            if dest.offset_from(product) as libc::c_long
                >= (100 as libc::c_int - 1 as libc::c_int) as libc::c_long
            {
                return 2 as libc::c_int;
            }
            if (*dest).is_null() {
                let ref mut fresh13 = *dest.offset(1 as libc::c_int as isize);
                *fresh13 = 0 as *mut libc::c_char;
            }
            *dest = *src;
            *src = NULLUNIT;
        }
        src = src.offset(1);
        src;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn copyproduct(
    mut dest: *mut *mut libc::c_char,
    mut source: *mut *mut libc::c_char,
) {
    while !(*source).is_null() {
        if *source == NULLUNIT {
            *dest = NULLUNIT;
        } else {
            *dest = dupstr(*source);
        }
        source = source.offset(1);
        source;
        dest = dest.offset(1);
        dest;
    }
    *dest = 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn unitcopy(mut dest: *mut unittype, mut source: *mut unittype) {
    (*dest).factor = (*source).factor;
    copyproduct(((*dest).numerator).as_mut_ptr(), ((*source).numerator).as_mut_ptr());
    copyproduct(
        ((*dest).denominator).as_mut_ptr(),
        ((*source).denominator).as_mut_ptr(),
    );
}
pub unsafe extern "C" fn multunit(
    mut left: *mut unittype,
    mut right: *mut unittype,
) -> libc::c_int {
    let mut myerr: libc::c_int = 0;
    (*left).factor *= (*right).factor;
    myerr = moveproduct(
        ((*left).numerator).as_mut_ptr(),
        ((*right).numerator).as_mut_ptr(),
    );
    if myerr == 0 {
        myerr = moveproduct(
            ((*left).denominator).as_mut_ptr(),
            ((*right).denominator).as_mut_ptr(),
        );
    }
    return myerr;
}
pub unsafe extern "C" fn divunit(
    mut left: *mut unittype,
    mut right: *mut unittype,
) -> libc::c_int {
    let mut myerr: libc::c_int = 0;
    (*left).factor /= (*right).factor;
    myerr = moveproduct(
        ((*left).numerator).as_mut_ptr(),
        ((*right).denominator).as_mut_ptr(),
    );
    if myerr == 0 {
        myerr = moveproduct(
            ((*left).denominator).as_mut_ptr(),
            ((*right).numerator).as_mut_ptr(),
        );
    }
    return myerr;
}
pub unsafe extern "C" fn reduceproduct(
    mut theunit: *mut unittype,
    mut flip: libc::c_int,
) -> libc::c_int {
    let mut toadd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut product: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut didsomething: libc::c_int = (1 as libc::c_int) << 1 as libc::c_int;
    let mut newunit_0: unittype = unittype {
        numerator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        denominator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        factor: 0.,
    };
    let mut ret: libc::c_int = 0;
    if flip != 0 {
        product = ((*theunit).denominator).as_mut_ptr();
    } else {
        product = ((*theunit).numerator).as_mut_ptr();
    }
    while !(*product).is_null() {
        while !(strlen(*product) == 0) {
            toadd = lookupunit(*product, 1 as libc::c_int);
            if toadd.is_null() {
                if irreducible.is_null() {
                    irreducible = dupstr(*product);
                }
                return (1 as libc::c_int) << 2 as libc::c_int;
            }
            if !(strchr(toadd, '!' as i32)).is_null() {
                break;
            }
            didsomething = (1 as libc::c_int) << 0 as libc::c_int;
            if *product != NULLUNIT {
                free(*product as *mut libc::c_void);
                *product = NULLUNIT;
            }
            if parseunit(
                &mut newunit_0,
                toadd,
                0 as *mut *mut libc::c_char,
                0 as *mut libc::c_int,
            ) != 0
            {
                return (1 as libc::c_int) << 2 as libc::c_int;
            }
            if flip != 0 {
                ret = divunit(theunit, &mut newunit_0);
            } else {
                ret = multunit(theunit, &mut newunit_0);
            }
            freeunit(&mut newunit_0);
            if ret != 0 {
                return (1 as libc::c_int) << 2 as libc::c_int;
            }
        }
        product = product.offset(1);
        product;
    }
    return didsomething;
}
pub unsafe extern "C" fn reduceunit(mut theunit: *mut unittype) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if !irreducible.is_null() {
        free(irreducible as *mut libc::c_void);
    }
    irreducible = 0 as *mut libc::c_char;
    ret = (1 as libc::c_int) << 0 as libc::c_int;
    while ret & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        ret = reduceproduct(theunit, 0 as libc::c_int);
        if ret & (1 as libc::c_int) << 2 as libc::c_int == 0 {
            ret |= reduceproduct(theunit, 1 as libc::c_int);
        }
        if ret & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            if !irreducible.is_null() {
                return 7 as libc::c_int
            } else {
                return 3 as libc::c_int
            }
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ignore_dimless(mut name: *mut libc::c_char) -> libc::c_int {
    let mut ul: *mut unitlist = 0 as *mut unitlist;
    if name.is_null() {
        return 0 as libc::c_int;
    }
    ul = ulookup(name);
    if !ul.is_null()
        && strcmp((*ul).value, b"!dimensionless\0" as *const u8 as *const libc::c_char)
            == 0
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ignore_nothing(mut name: *mut libc::c_char) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ignore_primitive(mut name: *mut libc::c_char) -> libc::c_int {
    let mut ul: *mut unitlist = 0 as *mut unitlist;
    if name.is_null() {
        return 0 as libc::c_int;
    }
    ul = ulookup(name);
    if !ul.is_null() && !(strchr((*ul).value, '!' as i32)).is_null() {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn compareproducts(
    mut one: *mut *mut libc::c_char,
    mut two: *mut *mut libc::c_char,
    mut isdimless: Option::<unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int>,
) -> libc::c_int {
    let mut oneblank: libc::c_int = 0;
    let mut twoblank: libc::c_int = 0;
    while !(*one).is_null() || !(*two).is_null() {
        oneblank = (*one == NULLUNIT || isdimless.unwrap()(*one) != 0) as libc::c_int;
        twoblank = (*two == NULLUNIT || isdimless.unwrap()(*two) != 0) as libc::c_int;
        if (*one).is_null() && twoblank == 0 {
            return 1 as libc::c_int;
        }
        if (*two).is_null() && oneblank == 0 {
            return 1 as libc::c_int;
        }
        if oneblank != 0 {
            one = one.offset(1);
            one;
        } else if twoblank != 0 {
            two = two.offset(1);
            two;
        } else if strcmp(*one, *two) != 0 {
            return 1 as libc::c_int
        } else {
            one = one.offset(1);
            one;
            two = two.offset(1);
            two;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn compareunits(
    mut first: *mut unittype,
    mut second: *mut unittype,
    mut isdimless: Option::<unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int>,
) -> libc::c_int {
    return (compareproducts(
        ((*first).numerator).as_mut_ptr(),
        ((*second).numerator).as_mut_ptr(),
        isdimless,
    ) != 0
        || compareproducts(
            ((*first).denominator).as_mut_ptr(),
            ((*second).denominator).as_mut_ptr(),
            isdimless,
        ) != 0) as libc::c_int;
}
pub unsafe extern "C" fn completereduce(mut unit: *mut unittype) -> libc::c_int {
    let mut err: libc::c_int = 0;
    err = reduceunit(unit);
    if err != 0 {
        return err;
    }
    sortunit(unit);
    cancelunit(unit);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn expunit(
    mut theunit: *mut unittype,
    mut power: libc::c_int,
) -> libc::c_int {
    let mut numptr: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut denptr: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut thefactor: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut uind: libc::c_int = 0;
    let mut denlen: libc::c_int = 0;
    let mut numlen: libc::c_int = 0;
    if power == 0 as libc::c_int {
        freeunit(theunit);
        initializeunit(theunit);
        return 0 as libc::c_int;
    }
    numlen = 0 as libc::c_int;
    numptr = ((*theunit).numerator).as_mut_ptr();
    while !(*numptr).is_null() {
        numlen += 1;
        numlen;
        numptr = numptr.offset(1);
        numptr;
    }
    denlen = 0 as libc::c_int;
    denptr = ((*theunit).denominator).as_mut_ptr();
    while !(*denptr).is_null() {
        denlen += 1;
        denlen;
        denptr = denptr.offset(1);
        denptr;
    }
    thefactor = (*theunit).factor;
    i = 1 as libc::c_int;
    while i < power {
        (*theunit).factor *= thefactor;
        uind = 0 as libc::c_int;
        while uind < numlen {
            if (*theunit).numerator[uind as usize] != NULLUNIT {
                if numptr.offset_from(((*theunit).numerator).as_mut_ptr())
                    as libc::c_long
                    >= (100 as libc::c_int - 1 as libc::c_int) as libc::c_long
                {
                    *denptr = 0 as *mut libc::c_char;
                    *numptr = *denptr;
                    return 2 as libc::c_int;
                }
                let fresh14 = numptr;
                numptr = numptr.offset(1);
                *fresh14 = dupstr((*theunit).numerator[uind as usize]);
            }
            uind += 1;
            uind;
        }
        uind = 0 as libc::c_int;
        while uind < denlen {
            if (*theunit).denominator[uind as usize] != NULLUNIT {
                let fresh15 = denptr;
                denptr = denptr.offset(1);
                *fresh15 = dupstr((*theunit).denominator[uind as usize]);
                if denptr.offset_from(((*theunit).denominator).as_mut_ptr())
                    as libc::c_long
                    >= (100 as libc::c_int - 1 as libc::c_int) as libc::c_long
                {
                    *denptr = 0 as *mut libc::c_char;
                    *numptr = *denptr;
                    return 2 as libc::c_int;
                }
            }
            uind += 1;
            uind;
        }
        i += 1;
        i;
    }
    *numptr = 0 as *mut libc::c_char;
    *denptr = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn unit2num(mut input: *mut unittype) -> libc::c_int {
    let mut one: unittype = unittype {
        numerator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        denominator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        factor: 0.,
    };
    let mut err: libc::c_int = 0;
    initializeunit(&mut one);
    err = completereduce(input);
    if err != 0 {
        return err;
    }
    if compareunits(
        input,
        &mut one,
        Some(ignore_nothing as unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int),
    ) != 0
    {
        return 5 as libc::c_int;
    }
    freeunit(input);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn unitdimless(mut input: *mut unittype) -> libc::c_int {
    let mut one: unittype = unittype {
        numerator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        denominator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        factor: 0.,
    };
    initializeunit(&mut one);
    if compareunits(
        input,
        &mut one,
        Some(ignore_dimless as unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int),
    ) != 0
    {
        return 0 as libc::c_int;
    }
    freeunit(input);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn subunitroot(
    mut n: libc::c_int,
    mut current: *mut *mut libc::c_char,
    mut out: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut ptr: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut count: libc::c_int = 0 as libc::c_int;
    while *current == NULLUNIT {
        current = current.offset(1);
        current;
    }
    ptr = current;
    while !(*ptr).is_null() {
        while !(*ptr).is_null() {
            if *ptr != NULLUNIT {
                if strcmp(*current, *ptr) != 0 {
                    break;
                }
                count += 1;
                count;
            }
            ptr = ptr.offset(1);
            ptr;
        }
        if count % n != 0 as libc::c_int {
            if ignore_dimless(*current) == 0 {
                return 6 as libc::c_int;
            }
        } else {
            count /= n;
            while count > 0 as libc::c_int {
                let fresh16 = out;
                out = out.offset(1);
                *fresh16 = dupstr(*current);
                count -= 1;
                count;
            }
        }
        current = ptr;
    }
    *out = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn rootunit(
    mut inunit: *mut unittype,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut outunit: unittype = unittype {
        numerator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        denominator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        factor: 0.,
    };
    let mut err: libc::c_int = 0;
    initializeunit(&mut outunit);
    err = completereduce(inunit);
    if err != 0 {
        return err;
    }
    if (*inunit).factor < 0 as libc::c_int as libc::c_double {
        return 6 as libc::c_int;
    }
    outunit.factor = pow((*inunit).factor, 1.0f64 / n as libc::c_double);
    err = subunitroot(
        n,
        ((*inunit).numerator).as_mut_ptr(),
        (outunit.numerator).as_mut_ptr(),
    );
    if err != 0 {
        return err;
    }
    err = subunitroot(
        n,
        ((*inunit).denominator).as_mut_ptr(),
        (outunit.denominator).as_mut_ptr(),
    );
    if err != 0 {
        return err;
    }
    freeunit(inunit);
    initializeunit(inunit);
    return multunit(inunit, &mut outunit);
}
pub unsafe extern "C" fn invertunit(mut theunit: *mut unittype) {
    let mut ptr: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut swap: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut numlen: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut ind: libc::c_int = 0;
    (*theunit).factor = 1.0f64 / (*theunit).factor;
    numlen = 0 as libc::c_int;
    length = numlen;
    ptr = ((*theunit).denominator).as_mut_ptr();
    while !(*ptr).is_null() {
        ptr = ptr.offset(1);
        ptr;
        length += 1;
        length;
    }
    ptr = ((*theunit).numerator).as_mut_ptr();
    while !(*ptr).is_null() {
        ptr = ptr.offset(1);
        ptr;
        numlen += 1;
        numlen;
    }
    if numlen > length {
        length = numlen;
    }
    ind = 0 as libc::c_int;
    while ind <= length {
        swap = (*theunit).numerator[ind as usize];
        (*theunit).numerator[ind as usize] = (*theunit).denominator[ind as usize];
        (*theunit).denominator[ind as usize] = swap;
        ind += 1;
        ind;
    }
}
pub unsafe extern "C" fn float2rat(
    mut y: libc::c_double,
    mut p: *mut libc::c_int,
    mut q: *mut libc::c_int,
) -> libc::c_int {
    let mut coef: [libc::c_int; 20] = [0; 20];
    let mut i: libc::c_int = 0;
    let mut termcount: libc::c_int = 0;
    let mut saveq: libc::c_int = 0;
    let mut fracpart: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    x = y;
    termcount = 0 as libc::c_int;
    loop {
        coef[termcount as usize] = floor(x) as libc::c_int;
        fracpart = x - coef[termcount as usize] as libc::c_double;
        if fracpart < 0.001f64 || termcount == 19 as libc::c_int {
            break;
        }
        x = 1 as libc::c_int as libc::c_double / fracpart;
        termcount += 1;
        termcount;
    }
    *p = 0 as libc::c_int;
    *q = 1 as libc::c_int;
    i = termcount;
    while i >= 1 as libc::c_int {
        saveq = *q;
        *q = coef[i as usize] * *q + *p;
        *p = saveq;
        i -= 1;
        i;
    }
    *p += *q * coef[0 as libc::c_int as usize];
    return (*q < 100 as libc::c_int
        && fabs(*p as libc::c_double / *q as libc::c_double - y)
            < 2.2204460492503131e-16f64) as libc::c_int;
}
pub unsafe extern "C" fn unitpower(
    mut base: *mut unittype,
    mut exponent: *mut unittype,
) -> libc::c_int {
    let mut errcode: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut q: libc::c_int = 0;
    errcode = unit2num(exponent);
    if errcode == 5 as libc::c_int {
        return 24 as libc::c_int;
    }
    if errcode != 0 {
        return errcode;
    }
    errcode = unit2num(base);
    if errcode == 0 {
        (*base).factor = pow((*base).factor, (*exponent).factor);
        if *__errno_location() != 0 {
            return 8 as libc::c_int;
        }
    } else if errcode == 5 as libc::c_int {
        if float2rat((*exponent).factor, &mut p, &mut q) == 0 {
            if unitdimless(base) != 0 {
                (*base).factor = pow((*base).factor, (*exponent).factor);
                if *__errno_location() != 0 {
                    return 8 as libc::c_int;
                }
            } else {
                return 22 as libc::c_int
            }
        } else {
            if q != 1 as libc::c_int {
                errcode = rootunit(base, q);
                if errcode == 6 as libc::c_int {
                    return 23 as libc::c_int;
                }
                if errcode != 0 {
                    return errcode;
                }
            }
            errcode = expunit(base, abs(p));
            if errcode != 0 {
                return errcode;
            }
            if p < 0 as libc::c_int {
                invertunit(base);
            }
        }
    } else {
        return errcode
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn addunit(
    mut unita: *mut unittype,
    mut unitb: *mut unittype,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    err = completereduce(unita);
    if err != 0 {
        return err;
    }
    err = completereduce(unitb);
    if err != 0 {
        return err;
    }
    if compareunits(
        unita,
        unitb,
        Some(ignore_nothing as unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int),
    ) != 0
    {
        return 4 as libc::c_int;
    }
    (*unita).factor += (*unitb).factor;
    freeunit(unitb);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn linearinterp(
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut aval: libc::c_double,
    mut bval: libc::c_double,
    mut c: libc::c_double,
) -> libc::c_double {
    let mut lambda: libc::c_double = 0.;
    lambda = (b - c) / (b - a);
    return lambda * aval + (1 as libc::c_int as libc::c_double - lambda) * bval;
}
pub unsafe extern "C" fn evalfunc(
    mut theunit: *mut unittype,
    mut infunc: *mut func,
    mut inverse: libc::c_int,
    mut allerrors: libc::c_int,
) -> libc::c_int {
    let mut result: unittype = unittype {
        numerator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        denominator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        factor: 0.,
    };
    let mut thefunc: *mut functype = 0 as *mut functype;
    let mut err: libc::c_int = 0;
    let mut value: libc::c_double = 0.;
    let mut foundit: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut save_value: *mut unittype = 0 as *mut unittype;
    let mut save_function: *mut libc::c_char = 0 as *mut libc::c_char;
    if !((*infunc).table).is_null() {
        err = parseunit(
            &mut result,
            (*infunc).tableunit,
            0 as *mut *mut libc::c_char,
            0 as *mut libc::c_int,
        );
        if err != 0 {
            return 12 as libc::c_int;
        }
        if inverse != 0 {
            err = divunit(theunit, &mut result);
            if err != 0 {
                return err;
            }
            err = unit2num(theunit);
            if err == 5 as libc::c_int {
                return 10 as libc::c_int;
            }
            if err != 0 {
                return err;
            }
            value = (*theunit).factor;
            foundit = 0 as libc::c_int;
            count = 0 as libc::c_int;
            while count < (*infunc).tablelen - 1 as libc::c_int {
                if (*((*infunc).table).offset(count as isize)).value <= value
                    && value
                        <= (*((*infunc).table)
                            .offset((count + 1 as libc::c_int) as isize))
                            .value
                    || (*((*infunc).table).offset((count + 1 as libc::c_int) as isize))
                        .value <= value
                        && value <= (*((*infunc).table).offset(count as isize)).value
                {
                    foundit = 1 as libc::c_int;
                    value = linearinterp(
                        (*((*infunc).table).offset(count as isize)).value,
                        (*((*infunc).table).offset((count + 1 as libc::c_int) as isize))
                            .value,
                        (*((*infunc).table).offset(count as isize)).location,
                        (*((*infunc).table).offset((count + 1 as libc::c_int) as isize))
                            .location,
                        value,
                    );
                    break;
                } else {
                    count += 1;
                    count;
                }
            }
            if foundit == 0 {
                return 11 as libc::c_int;
            }
            freeunit(&mut result);
            freeunit(theunit);
            (*theunit).factor = value;
            return 0 as libc::c_int;
        } else {
            err = unit2num(theunit);
            if err != 0 {
                return err;
            }
            value = (*theunit).factor;
            foundit = 0 as libc::c_int;
            count = 0 as libc::c_int;
            while count < (*infunc).tablelen - 1 as libc::c_int {
                if (*((*infunc).table).offset(count as isize)).location <= value
                    && value
                        <= (*((*infunc).table)
                            .offset((count + 1 as libc::c_int) as isize))
                            .location
                {
                    foundit = 1 as libc::c_int;
                    value = linearinterp(
                        (*((*infunc).table).offset(count as isize)).location,
                        (*((*infunc).table).offset((count + 1 as libc::c_int) as isize))
                            .location,
                        (*((*infunc).table).offset(count as isize)).value,
                        (*((*infunc).table).offset((count + 1 as libc::c_int) as isize))
                            .value,
                        value,
                    );
                    break;
                } else {
                    count += 1;
                    count;
                }
            }
            if foundit == 0 {
                return 11 as libc::c_int;
            }
            result.factor *= value;
        }
    } else {
        if inverse != 0 {
            thefunc = &mut (*infunc).inverse;
            if ((*thefunc).def).is_null() {
                return 13 as libc::c_int;
            }
        } else {
            thefunc = &mut (*infunc).forward;
        }
        err = completereduce(theunit);
        if err != 0 {
            return err;
        }
        if !((*thefunc).dimen).is_null() {
            err = parseunit(
                &mut result,
                (*thefunc).dimen,
                0 as *mut *mut libc::c_char,
                0 as *mut libc::c_int,
            );
            if err != 0 {
                return 12 as libc::c_int;
            }
            err = completereduce(&mut result);
            if err != 0 {
                return 12 as libc::c_int;
            }
            if compareunits(
                &mut result,
                theunit,
                Some(
                    ignore_nothing
                        as unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int,
                ),
            ) != 0
            {
                return 10 as libc::c_int;
            }
            value = (*theunit).factor / result.factor;
        } else {
            value = (*theunit).factor;
        }
        if !((*thefunc).domain_max).is_null()
            && (value > *(*thefunc).domain_max
                || (*thefunc).domain_max_open != 0 && value == *(*thefunc).domain_max)
        {
            return 11 as libc::c_int;
        }
        if !((*thefunc).domain_min).is_null()
            && (value < *(*thefunc).domain_min
                || (*thefunc).domain_min_open != 0 && value == *(*thefunc).domain_min)
        {
            return 11 as libc::c_int;
        }
        save_value = parameter_value;
        save_function = function_parameter;
        parameter_value = theunit;
        function_parameter = (*thefunc).param;
        err = parseunit(
            &mut result,
            (*thefunc).def,
            0 as *mut *mut libc::c_char,
            0 as *mut libc::c_int,
        );
        function_parameter = save_function;
        parameter_value = save_value;
        if err != 0
            && (allerrors == 1 as libc::c_int || err == 14 as libc::c_int
                || err == 2 as libc::c_int || err == 6 as libc::c_int
                || err == 9 as libc::c_int)
        {
            return err;
        }
        if err != 0 {
            return 15 as libc::c_int;
        }
    }
    freeunit(theunit);
    initializeunit(theunit);
    multunit(theunit, &mut result);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn showdefinition(
    mut unitstr: *mut libc::c_char,
    mut theunit: *mut unittype,
) {
    logputs(deftext);
    loop {
        unitstr = lookupunit(unitstr, 1 as libc::c_int);
        if !(!unitstr.is_null() && strspn(unitstr, digits) != strlen(unitstr)
            && (strchr(unitstr, '!' as i32)).is_null())
        {
            break;
        }
        tightprint(stdout, unitstr);
        if !logfile.is_null() {
            tightprint(logfile, unitstr);
        }
        logputs(b" = \0" as *const u8 as *const libc::c_char);
    }
    showunit(theunit);
    logputchar('\n' as i32 as libc::c_char);
}
pub unsafe extern "C" fn showfunction(mut func: *mut functype) {
    let mut unit: unittype = unittype {
        numerator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        denominator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        factor: 0.,
    };
    let mut not_dimensionless: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if ((*func).def).is_null() {
        logputs(b" is undefined\0" as *const u8 as *const libc::c_char);
        return;
    }
    if !((*func).dimen).is_null() {
        parseunit(
            &mut unit,
            (*func).dimen,
            0 as *mut *mut libc::c_char,
            0 as *mut libc::c_int,
        );
        not_dimensionless = unit2num(&mut unit);
    }
    logprintf(
        b"(%s) = %s\0" as *const u8 as *const libc::c_char,
        (*func).param,
        (*func).def,
    );
    if !((*func).domain_min).is_null() || !((*func).domain_max).is_null() {
        logputchar('\n' as i32 as libc::c_char);
        i = strwidth(deftext);
        while i != 0 {
            logputchar(' ' as i32 as libc::c_char);
            i -= 1;
            i;
        }
        logputs(b"defined for \0" as *const u8 as *const libc::c_char);
        if !((*func).domain_min).is_null() && !((*func).domain_max).is_null() {
            logprintf(num_format.format, *(*func).domain_min);
            if !((*func).dimen).is_null()
                && (not_dimensionless != 0
                    || unit.factor != 1 as libc::c_int as libc::c_double)
            {
                if isdecimal(*(*func).dimen) != 0 {
                    logputs(b" *\0" as *const u8 as *const libc::c_char);
                }
                logprintf(b" %s\0" as *const u8 as *const libc::c_char, (*func).dimen);
            }
            logputs(
                if (*func).domain_min_open != 0 {
                    b" < \0" as *const u8 as *const libc::c_char
                } else {
                    b" <= \0" as *const u8 as *const libc::c_char
                },
            );
        }
        logputs((*func).param);
        if !((*func).domain_max).is_null() {
            logputs(
                if (*func).domain_max_open != 0 {
                    b" < \0" as *const u8 as *const libc::c_char
                } else {
                    b" <= \0" as *const u8 as *const libc::c_char
                },
            );
            logprintf(num_format.format, *(*func).domain_max);
        } else {
            logputs(
                if (*func).domain_min_open != 0 {
                    b" > \0" as *const u8 as *const libc::c_char
                } else {
                    b" >= \0" as *const u8 as *const libc::c_char
                },
            );
            logprintf(num_format.format, *(*func).domain_min);
        }
        if !((*func).dimen).is_null()
            && (not_dimensionless != 0
                || unit.factor != 1 as libc::c_int as libc::c_double)
        {
            if isdecimal(*(*func).dimen) != 0 {
                logputs(b" *\0" as *const u8 as *const libc::c_char);
            }
            logprintf(b" %s\0" as *const u8 as *const libc::c_char, (*func).dimen);
        }
        if ((*func).dimen).is_null() {
            logputs(b" (any units)\0" as *const u8 as *const libc::c_char);
        }
    } else if !((*func).dimen).is_null() {
        logputchar('\n' as i32 as libc::c_char);
        i = strwidth(deftext);
        while i != 0 {
            logputchar(' ' as i32 as libc::c_char);
            i -= 1;
            i;
        }
        if not_dimensionless != 0 {
            logprintf(
                b"%s has units %s\0" as *const u8 as *const libc::c_char,
                (*func).param,
                (*func).dimen,
            );
        } else {
            logprintf(
                b"%s is dimensionless\0" as *const u8 as *const libc::c_char,
                (*func).param,
            );
        }
    }
    logputchar('\n' as i32 as libc::c_char);
}
pub unsafe extern "C" fn showtable(mut fun: *mut func, mut inverse: libc::c_int) {
    let mut i: libc::c_int = 0;
    logprintf(
        b"%sinterpolated table with points\n\0" as *const u8 as *const libc::c_char,
        deftext,
    );
    if inverse != 0 {
        let mut reverse: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        reverse = ((*((*fun).table).offset(0 as libc::c_int as isize)).value
            > (*((*fun).table).offset(((*fun).tablelen - 1 as libc::c_int) as isize))
                .value) as libc::c_int;
        i = 0 as libc::c_int;
        while i < (*fun).tablelen {
            if reverse != 0 {
                j = (*fun).tablelen - i - 1 as libc::c_int;
            } else {
                j = i;
            }
            if flags.verbose > 0 as libc::c_int {
                logputs(b"\t\t    \0" as *const u8 as *const libc::c_char);
            }
            logprintf(b"~%s(\0" as *const u8 as *const libc::c_char, (*fun).name);
            logprintf(num_format.format, (*((*fun).table).offset(j as isize)).value);
            if isdecimal(*((*fun).tableunit).offset(0 as libc::c_int as isize)) != 0 {
                logputs(b" *\0" as *const u8 as *const libc::c_char);
            }
            logprintf(b" %s\0" as *const u8 as *const libc::c_char, (*fun).tableunit);
            logputs(b") = \0" as *const u8 as *const libc::c_char);
            logprintf(num_format.format, (*((*fun).table).offset(j as isize)).location);
            logputchar('\n' as i32 as libc::c_char);
            i += 1;
            i;
        }
    } else {
        i = 0 as libc::c_int;
        while i < (*fun).tablelen {
            if flags.verbose > 0 as libc::c_int {
                logputs(b"\t\t    \0" as *const u8 as *const libc::c_char);
            }
            logprintf(b"%s(\0" as *const u8 as *const libc::c_char, (*fun).name);
            logprintf(num_format.format, (*((*fun).table).offset(i as isize)).location);
            logputs(b") = \0" as *const u8 as *const libc::c_char);
            logprintf(num_format.format, (*((*fun).table).offset(i as isize)).value);
            if isdecimal(*((*fun).tableunit).offset(0 as libc::c_int as isize)) != 0 {
                logputs(b" *\0" as *const u8 as *const libc::c_char);
            }
            logprintf(b" %s\n\0" as *const u8 as *const libc::c_char, (*fun).tableunit);
            i += 1;
            i;
        }
    };
}
pub unsafe extern "C" fn showfuncdefinition(
    mut fun: *mut func,
    mut inverse: libc::c_int,
) {
    if !((*fun).table).is_null() {
        showtable(fun, inverse);
    } else {
        logprintf(
            b"%s%s%s\0" as *const u8 as *const libc::c_char,
            deftext,
            if inverse != 0 {
                b"~\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            (*fun).name,
        );
        if inverse != 0 {
            showfunction(&mut (*fun).inverse);
        } else {
            showfunction(&mut (*fun).forward);
        }
    };
}
pub unsafe extern "C" fn showunitlistdef(mut alias: *mut wantalias) {
    logprintf(b"%sunit list, \0" as *const u8 as *const libc::c_char, deftext);
    tightprint(stdout, (*alias).definition);
    if !logfile.is_null() {
        tightprint(logfile, (*alias).definition);
    }
    logputchar('\n' as i32 as libc::c_char);
}
pub unsafe extern "C" fn showfunc(
    mut havestr: *mut libc::c_char,
    mut have: *mut unittype,
    mut fun: *mut func,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut dimen: *mut libc::c_char = 0 as *mut libc::c_char;
    err = evalfunc(have, fun, 1 as libc::c_int, 0 as libc::c_int);
    if err == 0 {
        err = completereduce(have);
    }
    if err != 0 {
        if err == 10 as libc::c_int {
            logputs(b"conformability error\0" as *const u8 as *const libc::c_char);
            if !((*fun).table).is_null() {
                dimen = (*fun).tableunit;
            } else if !((*fun).inverse.dimen).is_null() {
                dimen = (*fun).inverse.dimen;
            } else {
                dimen = 0 as *mut libc::c_char;
            }
            if dimen.is_null() {
                logputchar('\n' as i32 as libc::c_char);
            } else {
                let mut want: unittype = unittype {
                    numerator: [0 as *const libc::c_char as *mut libc::c_char; 100],
                    denominator: [0 as *const libc::c_char as *mut libc::c_char; 100],
                    factor: 0.,
                };
                if *dimen as libc::c_int == 0 as libc::c_int {
                    dimen = b"1\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                logprintf(
                    b": conversion requires dimensions of '%s'\n\0" as *const u8
                        as *const libc::c_char,
                    dimen,
                );
                if flags.verbose == 2 as libc::c_int {
                    logprintf(b"\t%s = \0" as *const u8 as *const libc::c_char, havestr);
                } else if flags.verbose == 1 as libc::c_int {
                    logputchar('\t' as i32 as libc::c_char);
                }
                showunit(have);
                if flags.verbose == 2 as libc::c_int {
                    logprintf(b"\n\t%s = \0" as *const u8 as *const libc::c_char, dimen);
                } else if flags.verbose == 1 as libc::c_int {
                    logprintf(b"\n\t\0" as *const u8 as *const libc::c_char);
                } else {
                    logputchar('\n' as i32 as libc::c_char);
                }
                parseunit(
                    &mut want,
                    dimen,
                    0 as *mut *mut libc::c_char,
                    0 as *mut libc::c_int,
                );
                completereduce(&mut want);
                showunit(&mut want);
                logputchar('\n' as i32 as libc::c_char);
            }
        } else if err == 11 as libc::c_int {
            logprintf(
                b"Value '%s' is not in the function's range\n\0" as *const u8
                    as *const libc::c_char,
                havestr,
            );
        } else if err == 13 as libc::c_int {
            logprintf(
                b"Inverse of the function '%s' is not defined\n\0" as *const u8
                    as *const libc::c_char,
                (*fun).name,
            );
        } else {
            logputs(
                b"Function evaluation error (bad function definition)\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return 1 as libc::c_int;
    }
    if flags.verbose == 2 as libc::c_int {
        logprintf(
            b"\t%s = %s(\0" as *const u8 as *const libc::c_char,
            havestr,
            (*fun).name,
        );
    } else if flags.verbose == 1 as libc::c_int {
        logputchar('\t' as i32 as libc::c_char);
    }
    showunit(have);
    if flags.verbose == 2 as libc::c_int {
        logputchar(')' as i32 as libc::c_char);
    }
    logputchar('\n' as i32 as libc::c_char);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn showconformabilityerr(
    mut havestr: *mut libc::c_char,
    mut have: *mut unittype,
    mut wantstr: *mut libc::c_char,
    mut want: *mut unittype,
) {
    logputs(b"conformability error\n\0" as *const u8 as *const libc::c_char);
    if flags.verbose == 2 as libc::c_int {
        logprintf(b"\t%s = \0" as *const u8 as *const libc::c_char, havestr);
    } else if flags.verbose == 1 as libc::c_int {
        logputchar('\t' as i32 as libc::c_char);
    }
    showunit(have);
    if flags.verbose == 2 as libc::c_int {
        logprintf(b"\n\t%s = \0" as *const u8 as *const libc::c_char, wantstr);
    } else if flags.verbose == 1 as libc::c_int {
        logputs(b"\n\t\0" as *const u8 as *const libc::c_char);
    } else {
        logputchar('\n' as i32 as libc::c_char);
    }
    showunit(want);
    logputchar('\n' as i32 as libc::c_char);
}
pub unsafe extern "C" fn isfract(mut unitstr: *const libc::c_char) -> libc::c_int {
    let mut enddouble: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut endlong: *mut libc::c_char = 0 as *mut libc::c_char;
    while *(*__ctype_b_loc()).offset(*unitstr as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        unitstr = unitstr.offset(1);
        unitstr;
    }
    let fresh17 = unitstr;
    unitstr = unitstr.offset(1);
    if *fresh17 as libc::c_int == '|' as i32 {
        strtod(unitstr, &mut enddouble);
        strtol(unitstr, &mut endlong, 10 as libc::c_int);
        if enddouble == endlong {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn checksigdigits(mut arg: *mut libc::c_char) -> libc::c_int {
    let mut errors: libc::c_int = 0;
    let mut ival: libc::c_int = 0;
    let mut nonum: *mut libc::c_char = 0 as *mut libc::c_char;
    errors = 0 as libc::c_int;
    if strcmp(arg, b"max\0" as *const u8 as *const libc::c_char) == 0 {
        num_format.precision = 15 as libc::c_int;
    } else {
        ival = strtol(arg, &mut nonum, 10 as libc::c_int) as libc::c_int;
        if !(*nonum as libc::c_int == 0 as libc::c_int) {
            fprintf(
                stderr,
                b"%s: invalid significant digits (%s)--integer value or 'max' required\n\0"
                    as *const u8 as *const libc::c_char,
                progname,
                arg,
            );
            errors += 1;
            errors;
        } else if ival <= 0 as libc::c_int {
            fprintf(
                stderr,
                b"%s: number of significant digits must be positive\n\0" as *const u8
                    as *const libc::c_char,
                progname,
            );
            errors += 1;
            errors;
        } else if ival > 15 as libc::c_int {
            fprintf(
                stderr,
                b"%s: too many significant digits (%d)--using maximum value (%d)\n\0"
                    as *const u8 as *const libc::c_char,
                progname,
                ival,
                15 as libc::c_int,
            );
            num_format.precision = 15 as libc::c_int;
        } else {
            num_format.precision = ival;
        }
    }
    if errors != 0 { return -(1 as libc::c_int) } else { return 0 as libc::c_int };
}
pub unsafe extern "C" fn setnumformat() -> libc::c_int {
    let mut len: size_t = 0;
    if !(strchr(
        b"Ee\0" as *const u8 as *const libc::c_char,
        num_format.type_0 as libc::c_int,
    ))
        .is_null()
    {
        num_format.precision -= 1;
        num_format.precision;
    }
    len = 4 as libc::c_int as size_t;
    if num_format.precision > 0 as libc::c_int {
        len = (len as libc::c_ulong)
            .wrapping_add(
                (floor(log10(num_format.precision as libc::c_double)) as size_t)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
    }
    num_format
        .format = mymalloc(
        len as libc::c_int,
        b"(setnumformat)\0" as *const u8 as *const libc::c_char,
    ) as *mut libc::c_char;
    sprintf(
        num_format.format,
        b"%%.%d%c\0" as *const u8 as *const libc::c_char,
        num_format.precision,
        num_format.type_0 as libc::c_int,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn parsenumformat() -> libc::c_int {
    static mut format_types: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    static mut format_flags: *mut libc::c_char = b"+-# 0'\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    static mut badflag: libc::c_char = 0;
    let mut two: *mut libc::c_char = b"0x1p+1\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut valid: *mut libc::c_char = b"ABCDEFGHIJKLMNOPQRSTUVWXYXabcdefghijklmnopqrstuvwxyx.01234567890\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    let mut dotptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nonum: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut testbuf: [libc::c_char; 80] = [0; 80];
    let mut errors: libc::c_int = 0;
    let mut ndx: libc::c_int = 0;
    if format_types.is_null() {
        format_types = mymalloc(
            (strlen(b"gGeEf\0" as *const u8 as *const libc::c_char))
                .wrapping_add(4 as libc::c_int as libc::c_ulong) as libc::c_int,
            b"(parsenumformat)\0" as *const u8 as *const libc::c_char,
        ) as *mut libc::c_char;
        strcpy(format_types, b"gGeEf\0" as *const u8 as *const libc::c_char);
        sprintf(
            testbuf.as_mut_ptr(),
            b"%.1F\0" as *const u8 as *const libc::c_char,
            1.2f64,
        );
        if strlen(testbuf.as_mut_ptr()) == 3 as libc::c_int as libc::c_ulong
            && testbuf[0 as libc::c_int as usize] as libc::c_int == '1' as i32
            && testbuf[2 as libc::c_int as usize] as libc::c_int == '2' as i32
        {
            strcat(format_types, b"F\0" as *const u8 as *const libc::c_char);
        }
        sprintf(
            testbuf.as_mut_ptr(),
            b"%.0a\0" as *const u8 as *const libc::c_char,
            2.0f64,
        );
        if strcmp(testbuf.as_mut_ptr(), two) == 0 {
            strcat(format_types, b"aA\0" as *const u8 as *const libc::c_char);
        }
        sprintf(
            testbuf.as_mut_ptr(),
            b"%'.0f\0" as *const u8 as *const libc::c_char,
            1234.0f64,
        );
        if strlen(testbuf.as_mut_ptr()) > 2 as libc::c_int as libc::c_ulong
            && testbuf[0 as libc::c_int as usize] as libc::c_int == '1' as i32
            && testbuf[2 as libc::c_int as usize] as libc::c_int == '2' as i32
        {
            badflag = '\0' as i32 as libc::c_char;
        } else {
            badflag = '\'' as i32 as libc::c_char;
        }
    }
    errors = 0 as libc::c_int;
    p = num_format.format;
    if *p as libc::c_int != '%' as i32 {
        fprintf(
            stderr,
            b"%s: number format specification must start with '%%'\n\0" as *const u8
                as *const libc::c_char,
            progname,
        );
        errors += 1;
        errors;
    } else if strrchr(num_format.format, '%' as i32) != num_format.format {
        fprintf(
            stderr,
            b"%s: only one '%%' allowed in number format specification\n\0" as *const u8
                as *const libc::c_char,
            progname,
        );
        errors += 1;
        errors;
        p = p.offset(1);
        p;
    } else {
        p = p.offset(1);
        p;
    }
    dotptr = strchr(num_format.format, '.' as i32);
    if !dotptr.is_null() && strrchr(num_format.format, '.' as i32) != dotptr {
        fprintf(
            stderr,
            b"%s: only one '.' allowed in number format specification\n\0" as *const u8
                as *const libc::c_char,
            progname,
        );
        errors += 1;
        errors;
    }
    while *p as libc::c_int != 0 && !(strchr(format_flags, *p as libc::c_int)).is_null()
    {
        if *p as libc::c_int == badflag as libc::c_int {
            fprintf(
                stderr,
                b"%s: digit-grouping flag (') not supported\n\0" as *const u8
                    as *const libc::c_char,
                progname,
            );
            errors += 1;
            errors;
        }
        p = p.offset(1);
        p;
    }
    lptr = strstr(num_format.format, b"hh\0" as *const u8 as *const libc::c_char);
    if !lptr.is_null()
        || {
            lptr = strstr(
                num_format.format,
                b"ll\0" as *const u8 as *const libc::c_char,
            );
            !lptr.is_null()
        }
    {
        fprintf(
            stderr,
            b"%s: type length modifier (%.2s) not supported\n\0" as *const u8
                as *const libc::c_char,
            progname,
            lptr,
        );
        errors += 1;
        errors;
    } else {
        lptr = strpbrk(
            num_format.format,
            b"hjLltz\0" as *const u8 as *const libc::c_char,
        );
        if !lptr.is_null() {
            fprintf(
                stderr,
                b"%s: type length modifier (%c) not supported\n\0" as *const u8
                    as *const libc::c_char,
                progname,
                *lptr.offset(0 as libc::c_int as isize) as libc::c_int,
            );
            errors += 1;
            errors;
        }
    }
    ndx = strspn(p, valid) as libc::c_int;
    if (ndx as libc::c_ulong) < strlen(p) {
        fprintf(
            stderr,
            b"%s: invalid character (%c) in width, precision, or type\n\0" as *const u8
                as *const libc::c_char,
            progname,
            *p.offset(ndx as isize) as libc::c_int,
        );
        errors += 1;
        errors;
    }
    if errors != 0 {
        fprintf(
            stderr,
            b"%s: invalid number format specification (%s)\n\0" as *const u8
                as *const libc::c_char,
            progname,
            num_format.format,
        );
        fprintf(
            stderr,
            b"%s: valid specification is %%[flags][width][.precision]type\n\0"
                as *const u8 as *const libc::c_char,
            progname,
        );
        return -(1 as libc::c_int);
    }
    num_format.width = strtol(p, &mut nonum, 10 as libc::c_int) as libc::c_int;
    if *nonum as libc::c_int == '.' as i32 {
        if *(*__ctype_b_loc())
            .offset(*nonum.offset(1 as libc::c_int as isize) as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            num_format
                .precision = strtol(
                nonum.offset(1 as libc::c_int as isize),
                &mut nonum,
                10 as libc::c_int,
            ) as libc::c_int;
        } else {
            num_format.precision = 0 as libc::c_int;
            nonum = nonum.offset(1);
            nonum;
        }
    } else {
        num_format.precision = 6 as libc::c_int;
    }
    if *nonum as libc::c_int == 0 as libc::c_int {
        fprintf(
            stderr,
            b"%s: missing format type\n\0" as *const u8 as *const libc::c_char,
            progname,
        );
        errors += 1;
        errors;
    } else if !(strchr(format_types, *nonum as libc::c_int)).is_null() {
        if *nonum.offset(1 as libc::c_int as isize) != 0 {
            fprintf(
                stderr,
                b"%s: invalid character(s) (%s) after format type\n\0" as *const u8
                    as *const libc::c_char,
                progname,
                nonum.offset(1 as libc::c_int as isize),
            );
            errors += 1;
            errors;
        } else {
            num_format.type_0 = *nonum;
        }
    } else {
        fprintf(
            stderr,
            b"%s: invalid format type (%c)--valid types are [%s]\n\0" as *const u8
                as *const libc::c_char,
            progname,
            *nonum as libc::c_int,
            format_types,
        );
        errors += 1;
        errors;
    }
    if num_format.precision == 0 as libc::c_int
        && (num_format.type_0 as libc::c_int == 'G' as i32
            || num_format.type_0 as libc::c_int == 'g' as i32)
    {
        num_format.precision = 1 as libc::c_int;
    }
    if errors != 0 {
        fprintf(
            stderr,
            b"%s: invalid number format specification (%s)\n\0" as *const u8
                as *const libc::c_char,
            progname,
            num_format.format,
        );
        fprintf(
            stderr,
            b"%s: valid specification is %%[flags][width][.precision]type\n\0"
                as *const u8 as *const libc::c_char,
            progname,
        );
        return -(1 as libc::c_int);
    } else {
        return 0 as libc::c_int
    };
}
pub unsafe extern "C" fn round_output(
    mut value: libc::c_double,
    mut sigdigits: libc::c_int,
    mut hasnondigits: *mut libc::c_int,
) -> libc::c_double {
    let mut buflen: libc::c_int = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rounded: libc::c_double = 0.;
    let mut mult_factor: libc::c_double = 0.;
    let mut rdigits: libc::c_double = 0.;
    let mut fmt_digits: libc::c_int = 0;
    if value.is_finite() as i32 == 0 {
        if !hasnondigits.is_null() {
            *hasnondigits = 1 as libc::c_int;
        }
        return value;
    }
    fmt_digits = num_format.precision;
    match num_format.type_0 as libc::c_int {
        65 | 97 => {
            sigdigits = round(
                sigdigits as libc::c_double * log2(10 as libc::c_int as libc::c_double)
                    / 4 as libc::c_int as libc::c_double,
            ) as libc::c_int;
            fmt_digits += 1;
            fmt_digits;
        }
        69 | 101 => {
            fmt_digits += 1;
            fmt_digits;
        }
        70 | 102 => {
            if fabs(value) > 0 as libc::c_int as libc::c_double {
                fmt_digits = (fmt_digits as libc::c_double + ceil(log10(fabs(value))))
                    as libc::c_int;
            }
        }
        _ => {}
    }
    if sigdigits < fmt_digits {
        rdigits = sigdigits as libc::c_double;
    } else {
        rdigits = fmt_digits as libc::c_double;
    }
    if value != 0 as libc::c_int as libc::c_double {
        rdigits -= ceil(log10(fabs(value)));
    }
    mult_factor = pow(10.0f64, rdigits);
    rounded = round(value * mult_factor) / mult_factor;
    buflen = num_format.precision + 9 as libc::c_int;
    if num_format.width > buflen {
        buflen = num_format.width;
    }
    if !(strchr(
        b"Ff\0" as *const u8 as *const libc::c_char,
        num_format.type_0 as libc::c_int,
    ))
        .is_null()
    {
        let mut len: libc::c_int = num_format.precision + 2 as libc::c_int;
        if fabs(value) > 1.0f64 {
            len += floor(log10(fabs(value))) as libc::c_int + 1 as libc::c_int;
        }
        if len > buflen {
            buflen = len;
        }
    }
    if !(strchr(num_format.format, '\'' as i32)).is_null()
        && !(strchr(
            b"FfGg\0" as *const u8 as *const libc::c_char,
            num_format.type_0 as libc::c_int,
        ))
            .is_null()
    {
        buflen = buflen * 3 as libc::c_int / 2 as libc::c_int;
    }
    buf = mymalloc(buflen, b"(round_output)\0" as *const u8 as *const libc::c_char)
        as *mut libc::c_char;
    sprintf(buf, num_format.format, value);
    if !hasnondigits.is_null() {
        if strspn(buf, b"1234567890\0" as *const u8 as *const libc::c_char)
            != strlen(buf)
        {
            *hasnondigits = 1 as libc::c_int;
        } else {
            *hasnondigits = 0 as libc::c_int;
        }
    }
    free(buf as *mut libc::c_void);
    return rounded;
}
pub unsafe extern "C" fn getsigdigits(
    mut original: libc::c_double,
    mut remainder: libc::c_double,
    mut base: libc::c_int,
) -> libc::c_int {
    let mut sigdigits: libc::c_int = 0;
    let mut maxdigits: libc::c_double = 0.;
    let mut logfunc: Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double> = None;
    if base == 2 as libc::c_int {
        maxdigits = 53 as libc::c_int as libc::c_double;
        logfunc = Some(log2 as unsafe extern "C" fn(libc::c_double) -> libc::c_double);
    } else {
        maxdigits = 53 as libc::c_int as libc::c_double * log10(2.0f64);
        logfunc = Some(log10 as unsafe extern "C" fn(libc::c_double) -> libc::c_double);
    }
    if original == 0 as libc::c_int as libc::c_double {
        return floor(maxdigits) as libc::c_int
    } else if remainder == 0 as libc::c_int as libc::c_double {
        return 0 as libc::c_int
    }
    sigdigits = floor(maxdigits - logfunc.unwrap()(fabs(original / remainder)))
        as libc::c_int;
    if sigdigits < 0 as libc::c_int {
        sigdigits = 0 as libc::c_int;
    }
    return sigdigits;
}
pub unsafe extern "C" fn round_digits(
    mut value: libc::c_double,
    mut digits_0: libc::c_int,
    mut base: libc::c_int,
) -> libc::c_double {
    let mut mult_factor: libc::c_double = 0.;
    let mut logfunc: Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double> = None;
    if digits_0 == 0 as libc::c_int {
        return 0.0f64;
    }
    if base == 2 as libc::c_int {
        logfunc = Some(log2 as unsafe extern "C" fn(libc::c_double) -> libc::c_double);
    } else {
        logfunc = Some(log10 as unsafe extern "C" fn(libc::c_double) -> libc::c_double);
    }
    if value != 0 as libc::c_int as libc::c_double {
        digits_0 = (digits_0 as libc::c_double - ceil(logfunc.unwrap()(fabs(value))))
            as libc::c_int;
    }
    mult_factor = pow(base as libc::c_double, digits_0 as libc::c_double);
    return round(value * mult_factor) / mult_factor;
}
pub unsafe extern "C" fn displays_as(
    mut reference: libc::c_double,
    mut value: libc::c_double,
    mut hasnondigits: *mut libc::c_int,
) -> libc::c_int {
    let mut buflen: libc::c_int = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rounded: libc::c_double = 0.;
    if value.is_finite() as i32 == 0 {
        if !hasnondigits.is_null() {
            *hasnondigits = 1 as libc::c_int;
        }
        return 0 as libc::c_int;
    }
    buflen = num_format.precision + 9 as libc::c_int;
    if num_format.width > buflen {
        buflen = num_format.width;
    }
    if !(strchr(
        b"Ff\0" as *const u8 as *const libc::c_char,
        num_format.type_0 as libc::c_int,
    ))
        .is_null()
    {
        let mut len: libc::c_int = num_format.precision + 2 as libc::c_int;
        if fabs(value) > 1.0f64 {
            len += floor(log10(fabs(value))) as libc::c_int + 1 as libc::c_int;
        }
        if len > buflen {
            buflen = len;
        }
    }
    if !(strchr(num_format.format, '\'' as i32)).is_null()
        && !(strchr(
            b"FfGg\0" as *const u8 as *const libc::c_char,
            num_format.type_0 as libc::c_int,
        ))
            .is_null()
    {
        buflen = buflen * 3 as libc::c_int / 2 as libc::c_int;
    }
    buf = mymalloc(buflen, b"(round_to_displayed)\0" as *const u8 as *const libc::c_char)
        as *mut libc::c_char;
    sprintf(buf, num_format.format, value);
    if !hasnondigits.is_null() {
        if strspn(buf, b"1234567890\0" as *const u8 as *const libc::c_char)
            != strlen(buf)
        {
            *hasnondigits = 1 as libc::c_int;
        } else {
            *hasnondigits = 0 as libc::c_int;
        }
    }
    rounded = strtod(buf, 0 as *mut *mut libc::c_char);
    free(buf as *mut libc::c_void);
    return (rounded == reference) as libc::c_int;
}
pub unsafe extern "C" fn showunitname(
    mut value: libc::c_double,
    mut unitstr: *mut libc::c_char,
    mut printnum: libc::c_int,
) {
    let mut hasnondigits: libc::c_int = 0;
    let mut is_one: libc::c_int = 0;
    is_one = displays_as(1 as libc::c_int as libc::c_double, value, &mut hasnondigits);
    if printnum != 0 && !(is_one != 0 && isdecimal(*unitstr) != 0) {
        logprintf(num_format.format, value);
    }
    if !(strpbrk(unitstr, b"+-\0" as *const u8 as *const libc::c_char)).is_null() {
        logprintf(b" (%s)\0" as *const u8 as *const libc::c_char, unitstr);
    } else if printnum != 0 && flags.showfactor == 0
        && strncmp(
            unitstr,
            b"1|\0" as *const u8 as *const libc::c_char,
            strlen(b"1|\0" as *const u8 as *const libc::c_char),
        ) == 0 && isfract(unitstr) != 0 && is_one == 0 && hasnondigits == 0
    {
        logputs(unitstr.offset(1 as libc::c_int as isize));
    } else if is_one != 0 && isdecimal(*unitstr) != 0 {
        logputs(unitstr);
    } else if isdecimal(*unitstr.offset(0 as libc::c_int as isize)) != 0 {
        logprintf(b" * %s\0" as *const u8 as *const libc::c_char, unitstr);
    } else {
        logprintf(b" %s\0" as *const u8 as *const libc::c_char, unitstr);
    };
}
pub unsafe extern "C" fn showanswer(
    mut havestr: *mut libc::c_char,
    mut have: *mut unittype,
    mut wantstr: *mut libc::c_char,
    mut want: *mut unittype,
) -> libc::c_int {
    let mut invhave: unittype = unittype {
        numerator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        denominator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        factor: 0.,
    };
    let mut doingrec: libc::c_int = 0;
    let mut right: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut left: *mut libc::c_char = 0 as *mut libc::c_char;
    doingrec = 0 as libc::c_int;
    if compareunits(
        have,
        want,
        Some(ignore_dimless as unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int),
    ) != 0
    {
        let mut src: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut dest: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        invhave.factor = 1 as libc::c_int as libc::c_double / (*have).factor;
        src = ((*have).numerator).as_mut_ptr();
        dest = (invhave.denominator).as_mut_ptr();
        while !(*src).is_null() {
            *dest = *src;
            src = src.offset(1);
            src;
            dest = dest.offset(1);
            dest;
        }
        *dest = 0 as *mut libc::c_char;
        src = ((*have).denominator).as_mut_ptr();
        dest = (invhave.numerator).as_mut_ptr();
        while !(*src).is_null() {
            *dest = *src;
            src = src.offset(1);
            src;
            dest = dest.offset(1);
            dest;
        }
        *dest = 0 as *mut libc::c_char;
        if flags.strictconvert != 0
            || compareunits(
                &mut invhave,
                want,
                Some(
                    ignore_dimless
                        as unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int,
                ),
            ) != 0
        {
            showconformabilityerr(havestr, have, wantstr, want);
            return -(1 as libc::c_int);
        }
        if flags.verbose > 0 as libc::c_int {
            logputchar('\t' as i32 as libc::c_char);
        }
        logputs(b"reciprocal conversion\n\0" as *const u8 as *const libc::c_char);
        have = &mut invhave;
        doingrec = 1 as libc::c_int;
    }
    if flags.verbose == 2 as libc::c_int {
        if doingrec == 0 {
            right = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            left = right;
        } else if !(strchr(havestr, '/' as i32)).is_null() {
            left = b"1 / (\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            right = b")\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else {
            left = b"1 / \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            right = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    }
    if flags.verbose == 2 as libc::c_int {
        logprintf(
            b"\t%s%s%s = \0" as *const u8 as *const libc::c_char,
            left,
            havestr,
            right,
        );
    } else if flags.verbose == 1 as libc::c_int {
        logputs(b"\t* \0" as *const u8 as *const libc::c_char);
    }
    if flags.verbose == 2 as libc::c_int {
        showunitname((*have).factor / (*want).factor, wantstr, 1 as libc::c_int);
    } else {
        logprintf(num_format.format, (*have).factor / (*want).factor);
    }
    if flags.oneline == 0 {
        if flags.verbose == 2 as libc::c_int {
            logprintf(
                b"\n\t%s%s%s = (1 / \0" as *const u8 as *const libc::c_char,
                left,
                havestr,
                right,
            );
        } else if flags.verbose == 1 as libc::c_int {
            logputs(b"\n\t/ \0" as *const u8 as *const libc::c_char);
        } else {
            logputchar('\n' as i32 as libc::c_char);
        }
        logprintf(num_format.format, (*want).factor / (*have).factor);
        if flags.verbose == 2 as libc::c_int {
            logputchar(')' as i32 as libc::c_char);
            showunitname(0 as libc::c_int as libc::c_double, wantstr, 0 as libc::c_int);
        }
    }
    logputchar('\n' as i32 as libc::c_char);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn checkfunc(mut infunc: *mut func, mut verbose: libc::c_int) {
    let mut theunit: unittype = unittype {
        numerator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        denominator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        factor: 0.,
    };
    let mut saveunit: unittype = unittype {
        numerator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        denominator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        factor: 0.,
    };
    let mut prefix: *mut prefixlist = 0 as *mut prefixlist;
    let mut err: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut direction: libc::c_double = 0.;
    if (*infunc).skip_error_check != 0 {
        if verbose != 0 {
            printf(
                b"skipped function '%s'\n\0" as *const u8 as *const libc::c_char,
                (*infunc).name,
            );
        }
        return;
    }
    if verbose != 0 {
        printf(
            b"doing function '%s'\n\0" as *const u8 as *const libc::c_char,
            (*infunc).name,
        );
    }
    prefix = plookup((*infunc).name);
    if !prefix.is_null() && strlen((*prefix).name) == strlen((*infunc).name) {
        printf(
            b"Warning: '%s' defined as prefix and function\n\0" as *const u8
                as *const libc::c_char,
            (*infunc).name,
        );
    }
    if !((*infunc).table).is_null() {
        if parseunit(
            &mut theunit,
            (*infunc).tableunit,
            0 as *mut *mut libc::c_char,
            0 as *mut libc::c_int,
        ) != 0 || completereduce(&mut theunit) != 0
        {
            printf(
                b"Table '%s' has invalid units '%s'\n\0" as *const u8
                    as *const libc::c_char,
                (*infunc).name,
                (*infunc).tableunit,
            );
        }
        freeunit(&mut theunit);
        if (*infunc).tablelen <= 1 as libc::c_int {
            printf(
                b"Table '%s' has only one data point\n\0" as *const u8
                    as *const libc::c_char,
                (*infunc).name,
            );
            return;
        }
        direction = (if (*((*infunc).table).offset(1 as libc::c_int as isize)).value
            - (*((*infunc).table).offset(0 as libc::c_int as isize)).value > 0.0f64
        {
            1 as libc::c_int
        } else if (*((*infunc).table).offset(1 as libc::c_int as isize)).value
            - (*((*infunc).table).offset(0 as libc::c_int as isize)).value < 0.0f64
        {
            -(1 as libc::c_int)
        } else {
            0 as libc::c_int
        }) as libc::c_double;
        i = 2 as libc::c_int;
        while i < (*infunc).tablelen {
            if (if (*((*infunc).table).offset(i as isize)).value
                - (*((*infunc).table).offset((i - 1 as libc::c_int) as isize)).value
                > 0.0f64
            {
                1 as libc::c_int
            } else {
                (if (*((*infunc).table).offset(i as isize)).value
                    - (*((*infunc).table).offset((i - 1 as libc::c_int) as isize)).value
                    < 0.0f64
                {
                    -(1 as libc::c_int)
                } else {
                    0 as libc::c_int
                })
            }) as libc::c_double != direction
            {
                printf(
                    b"Table '%s' lacks unique inverse around entry %.8g\n\0" as *const u8
                        as *const libc::c_char,
                    (*infunc).name,
                    (*((*infunc).table).offset(i as isize)).location,
                );
                return;
            }
            i += 1;
            i;
        }
        return;
    }
    if !((*infunc).forward.dimen).is_null() {
        if parseunit(
            &mut theunit,
            (*infunc).forward.dimen,
            0 as *mut *mut libc::c_char,
            0 as *mut libc::c_int,
        ) != 0 || completereduce(&mut theunit) != 0
        {
            printf(
                b"Function '%s' has invalid units '%s'\n\0" as *const u8
                    as *const libc::c_char,
                (*infunc).name,
                (*infunc).forward.dimen,
            );
            freeunit(&mut theunit);
            return;
        }
    } else {
        initializeunit(&mut theunit);
    }
    if !((*infunc).forward.domain_max).is_null()
        && !((*infunc).forward.domain_min).is_null()
    {
        theunit.factor
            *= (*(*infunc).forward.domain_max + *(*infunc).forward.domain_min)
                / 2 as libc::c_int as libc::c_double;
    } else if !((*infunc).forward.domain_max).is_null() {
        theunit
            .factor = theunit.factor * *(*infunc).forward.domain_max
            - 1 as libc::c_int as libc::c_double;
    } else if !((*infunc).forward.domain_min).is_null() {
        theunit
            .factor = theunit.factor * *(*infunc).forward.domain_min
            + 1 as libc::c_int as libc::c_double;
    } else {
        theunit.factor *= 7 as libc::c_int as libc::c_double;
    }
    if !((*infunc).forward.dimen).is_null() {
        unitcopy(&mut saveunit, &mut theunit);
        err = evalfunc(&mut theunit, infunc, 0 as libc::c_int, 1 as libc::c_int);
        if err != 0 {
            printf(
                b"Error in definition %s(%s) as '%s':\n\0" as *const u8
                    as *const libc::c_char,
                (*infunc).name,
                (*infunc).forward.param,
                (*infunc).forward.def,
            );
            printf(
                b"      %s\n\0" as *const u8 as *const libc::c_char,
                errormsg[err as usize],
            );
            freeunit(&mut theunit);
            freeunit(&mut saveunit);
            return;
        }
    } else {
        let mut resultunit: unittype = unittype {
            numerator: [0 as *const libc::c_char as *mut libc::c_char; 100],
            denominator: [0 as *const libc::c_char as *mut libc::c_char; 100],
            factor: 0.,
        };
        let mut arbunit: unittype = unittype {
            numerator: [0 as *const libc::c_char as *mut libc::c_char; 100],
            denominator: [0 as *const libc::c_char as *mut libc::c_char; 100],
            factor: 0.,
        };
        let mut unittext: [libc::c_char; 9] = [0; 9];
        let mut factor: libc::c_double = 0.;
        let mut errors: [libc::c_int; 4] = [0; 4];
        let mut errcount: libc::c_int = 0 as libc::c_int;
        let mut indent: *mut libc::c_char = 0 as *mut libc::c_char;
        strcpy(unittext.as_mut_ptr(), b"(kg K)^ \0" as *const u8 as *const libc::c_char);
        factor = theunit.factor;
        initializeunit(&mut saveunit);
        initializeunit(&mut resultunit);
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            *unittext
                .as_mut_ptr()
                .offset(strlen(unittext.as_mut_ptr()) as isize)
                .offset(-(1 as libc::c_int as isize)) = ('0' as i32 + i) as libc::c_char;
            err = parseunit(
                &mut arbunit,
                unittext.as_mut_ptr(),
                0 as *mut *mut libc::c_char,
                0 as *mut libc::c_int,
            );
            if err != 0 {
                initializeunit(&mut arbunit);
            }
            arbunit.factor = factor;
            unitcopy(&mut resultunit, &mut arbunit);
            errors[i
                as usize] = evalfunc(
                &mut resultunit,
                infunc,
                0 as libc::c_int,
                1 as libc::c_int,
            );
            if errors[i as usize] != 0 {
                errcount += 1;
                errcount;
            } else {
                freeunit(&mut saveunit);
                freeunit(&mut theunit);
                unitcopy(&mut saveunit, &mut arbunit);
                unitcopy(&mut theunit, &mut resultunit);
            }
            freeunit(&mut resultunit);
            freeunit(&mut arbunit);
            i += 1;
            i;
        }
        if errors[0 as libc::c_int as usize] == 0 && errcount == 3 as libc::c_int {
            printf(
                b"Warning: function '%s(%s)' defined as '%s'\n\0" as *const u8
                    as *const libc::c_char,
                (*infunc).name,
                (*infunc).forward.param,
                (*infunc).forward.def,
            );
            printf(
                b"         appears to require a dimensionless argument, 'units' keyword not given\n\0"
                    as *const u8 as *const libc::c_char,
            );
            indent = b"         \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        } else if errcount == 4 as libc::c_int {
            printf(
                b"Error or missing 'units' keyword in definion %s(%s) as '%s'\n\0"
                    as *const u8 as *const libc::c_char,
                (*infunc).name,
                (*infunc).forward.param,
                (*infunc).forward.def,
            );
            indent = b"      \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        } else if errcount != 0 {
            printf(
                b"Warning: function '%s(%s)' defined as '%s'\n\0" as *const u8
                    as *const libc::c_char,
                (*infunc).name,
                (*infunc).forward.param,
                (*infunc).forward.def,
            );
            printf(
                b"         failed for some test inputs:\n\0" as *const u8
                    as *const libc::c_char,
            );
            indent = b"         \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            if errors[i as usize] != 0 {
                *unittext
                    .as_mut_ptr()
                    .offset(strlen(unittext.as_mut_ptr()) as isize)
                    .offset(
                        -(1 as libc::c_int as isize),
                    ) = ('0' as i32 + i) as libc::c_char;
                printf(
                    b"%s%s(\0" as *const u8 as *const libc::c_char,
                    indent,
                    (*infunc).name,
                );
                printf(num_format.format, factor);
                printf(
                    b"%s): %s\n\0" as *const u8 as *const libc::c_char,
                    unittext.as_mut_ptr(),
                    errormsg[errors[i as usize] as usize],
                );
            }
            i += 1;
            i;
        }
    }
    if completereduce(&mut theunit) != 0 {
        printf(
            b"Definition %s(%s) as '%s' is irreducible\n\0" as *const u8
                as *const libc::c_char,
            (*infunc).name,
            (*infunc).forward.param,
            (*infunc).forward.def,
        );
        freeunit(&mut theunit);
        freeunit(&mut saveunit);
        return;
    }
    if ((*infunc).inverse.def).is_null() {
        printf(
            b"Warning: no inverse for function '%s'\n\0" as *const u8
                as *const libc::c_char,
            (*infunc).name,
        );
        freeunit(&mut theunit);
        freeunit(&mut saveunit);
        return;
    }
    err = evalfunc(&mut theunit, infunc, 1 as libc::c_int, 1 as libc::c_int);
    if err != 0 {
        printf(
            b"Error in inverse ~%s(%s) as '%s':\n\0" as *const u8 as *const libc::c_char,
            (*infunc).name,
            (*infunc).inverse.param,
            (*infunc).inverse.def,
        );
        printf(
            b"      %s\n\0" as *const u8 as *const libc::c_char,
            errormsg[err as usize],
        );
        freeunit(&mut theunit);
        freeunit(&mut saveunit);
        return;
    }
    divunit(&mut theunit, &mut saveunit);
    if unit2num(&mut theunit) != 0
        || fabs(theunit.factor - 1 as libc::c_int as libc::c_double) > 1e-12f64
    {
        printf(
            b"Inverse is not the inverse for function '%s'\n\0" as *const u8
                as *const libc::c_char,
            (*infunc).name,
        );
    }
    freeunit(&mut theunit);
}
pub unsafe extern "C" fn addtolist(
    mut have: *mut unittype,
    mut searchstring: *mut libc::c_char,
    mut rname: *mut libc::c_char,
    mut name: *mut libc::c_char,
    mut def: *mut libc::c_char,
    mut list: *mut *mut namedef,
    mut listsize: *mut libc::c_int,
    mut maxnamelen: *mut libc::c_int,
    mut count: *mut libc::c_int,
    mut searchtype: libc::c_int,
) {
    let mut want: unittype = unittype {
        numerator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        denominator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        factor: 0.,
    };
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut keepit: libc::c_int = 0 as libc::c_int;
    if name.is_null() {
        return;
    }
    if searchtype == 1 as libc::c_int {
        initializeunit(&mut want);
        if parseunit(&mut want, name, 0 as *mut *mut libc::c_char, 0 as *mut libc::c_int)
            == 0 && completereduce(&mut want) == 0
        {
            keepit = (compareunits(
                have,
                &mut want,
                Some(
                    ignore_dimless
                        as unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int,
                ),
            ) == 0) as libc::c_int;
        }
    } else if searchtype == 2 as libc::c_int {
        keepit = (strstr(rname, searchstring)
            != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
    }
    if keepit != 0 {
        if *count == *listsize {
            *listsize += 100 as libc::c_int;
            *list = realloc(
                *list as *mut libc::c_void,
                (*listsize as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<namedef>() as libc::c_ulong),
            ) as *mut namedef;
            if (*list).is_null() {
                fprintf(
                    stderr,
                    b"%s: memory allocation error (addtolist)\n\0" as *const u8
                        as *const libc::c_char,
                    progname,
                );
                exit(1 as libc::c_int);
            }
        }
        let ref mut fresh18 = (*(*list).offset(*count as isize)).name;
        *fresh18 = rname;
        if !(strchr(def, '!' as i32)).is_null() {
            let ref mut fresh19 = (*(*list).offset(*count as isize)).def;
            *fresh19 = b"<primitive unit>\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        } else {
            let ref mut fresh20 = (*(*list).offset(*count as isize)).def;
            *fresh20 = def;
        }
        *count += 1;
        *count;
        len = strwidth(name);
        if len > *maxnamelen {
            *maxnamelen = len;
        }
    }
    if searchtype == 1 as libc::c_int {
        freeunit(&mut want);
    }
}
pub unsafe extern "C" fn compnd(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    return strcmp((*(a as *mut namedef)).name, (*(b as *mut namedef)).name);
}
pub unsafe extern "C" fn screensize() -> libc::c_int {
    let mut lines: libc::c_int = 20 as libc::c_int;
    let mut ws: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let mut fd: libc::c_int = 0;
    fd = open(b"/dev/tty\0" as *const u8 as *const libc::c_char, 0o2 as libc::c_int);
    if fd >= 0 as libc::c_int
        && ioctl(fd, 0x5413 as libc::c_int as libc::c_ulong, &mut ws as *mut winsize)
            == 0 as libc::c_int
    {
        lines = ws.ws_row as libc::c_int;
    }
    return lines;
}
pub unsafe extern "C" fn get_output_fp(mut lines: libc::c_int) -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    if isatty(fileno(stdout)) != 0 && screensize() < lines {
        fp = popen(pager, b"w\0" as *const u8 as *const libc::c_char);
        if fp.is_null() {
            fprintf(
                stderr,
                b"%s: can't run pager '%s--'\0" as *const u8 as *const libc::c_char,
                progname,
                pager,
            );
            perror(0 as *mut libc::c_void as *mut libc::c_char);
        }
    }
    if fp.is_null() {
        fp = stdout;
    }
    return fp;
}
pub unsafe extern "C" fn countlines(mut msg: *mut libc::c_char) -> libc::c_int {
    let mut nlines: libc::c_int = 0 as libc::c_int;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = msg;
    while *p != 0 {
        if *p as libc::c_int == '\n' as i32 {
            nlines += 1;
            nlines;
        }
        p = p.offset(1);
        p;
    }
    return nlines;
}
pub unsafe extern "C" fn tryallunits(
    mut have: *mut unittype,
    mut searchstring: *mut libc::c_char,
) {
    let mut uptr: *mut unitlist = 0 as *mut unitlist;
    let mut list: *mut namedef = 0 as *mut namedef;
    let mut listsize: libc::c_int = 0;
    let mut maxnamelen: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut funcptr: *mut func = 0 as *mut func;
    let mut aliasptr: *mut wantalias = 0 as *mut wantalias;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut searchtype: libc::c_int = 0;
    let mut outfile: *mut FILE = 0 as *mut FILE;
    let mut seploc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut firstunit: *mut libc::c_char = 0 as *mut libc::c_char;
    list = mymalloc(
        (100 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<namedef>() as libc::c_ulong)
            as libc::c_int,
        b"(tryallunits)\0" as *const u8 as *const libc::c_char,
    ) as *mut namedef;
    listsize = 100 as libc::c_int;
    maxnamelen = 0 as libc::c_int;
    count = 0 as libc::c_int;
    if !have.is_null() {
        searchtype = 1 as libc::c_int;
    } else {
        if searchstring.is_null() {
            searchstring = b"\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        searchtype = 2 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < 101 as libc::c_int {
        uptr = utab[i as usize];
        while !uptr.is_null() {
            addtolist(
                have,
                searchstring,
                (*uptr).name,
                (*uptr).name,
                (*uptr).value,
                &mut list,
                &mut listsize,
                &mut maxnamelen,
                &mut count,
                searchtype,
            );
            uptr = (*uptr).next;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        funcptr = ftab[i as usize];
        while !funcptr.is_null() {
            if !((*funcptr).table).is_null() {
                addtolist(
                    have,
                    searchstring,
                    (*funcptr).name,
                    (*funcptr).tableunit,
                    b"<piecewise linear>\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    &mut list,
                    &mut listsize,
                    &mut maxnamelen,
                    &mut count,
                    searchtype,
                );
            } else {
                addtolist(
                    have,
                    searchstring,
                    (*funcptr).name,
                    (*funcptr).inverse.dimen,
                    b"<nonlinear>\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    &mut list,
                    &mut listsize,
                    &mut maxnamelen,
                    &mut count,
                    searchtype,
                );
            }
            funcptr = (*funcptr).next;
        }
        i += 1;
        i;
    }
    aliasptr = firstalias;
    while !aliasptr.is_null() {
        firstunit = dupstr((*aliasptr).definition);
        seploc = strchr(firstunit, ';' as i32);
        *seploc = 0 as libc::c_int as libc::c_char;
        addtolist(
            have,
            searchstring,
            (*aliasptr).name,
            firstunit,
            (*aliasptr).definition,
            &mut list,
            &mut listsize,
            &mut maxnamelen,
            &mut count,
            searchtype,
        );
        free(firstunit as *mut libc::c_void);
        aliasptr = (*aliasptr).next;
    }
    qsort(
        list as *mut libc::c_void,
        count as size_t,
        ::std::mem::size_of::<namedef>() as libc::c_ulong,
        Some(
            compnd
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    if count == 0 as libc::c_int {
        puts(b"No matching units found.\0" as *const u8 as *const libc::c_char);
    }
    signal(
        13 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    outfile = get_output_fp(count);
    i = 0 as libc::c_int;
    while i < count {
        fputs((*list.offset(i as isize)).name, outfile);
        if flags.verbose > 0 as libc::c_int || flags.interactive != 0 {
            j = strwidth((*list.offset(i as isize)).name);
            while j <= maxnamelen {
                putc(' ' as i32, outfile);
                j += 1;
                j;
            }
            tightprint(outfile, (*list.offset(i as isize)).def);
        }
        fputc('\n' as i32, outfile);
        i += 1;
        i;
    }
    if outfile != stdout {
        pclose(outfile);
    }
    signal(13 as libc::c_int, None);
}
pub unsafe extern "C" fn getuser_noreadline(
    mut buffer_0: *mut *mut libc::c_char,
    mut bufsize_0: *mut libc::c_int,
    mut query: *const libc::c_char,
) {
    let mut valid: libc::c_int = 0 as libc::c_int;
    while valid == 0 {
        fputs(query, stdout);
        if (fgetslong(buffer_0, bufsize_0, stdin, 0 as *mut libc::c_int)).is_null() {
            if flags.quiet == 0 {
                putchar('\n' as i32);
            }
            exit(0 as libc::c_int);
        }
        replacectrlchars(*buffer_0);
        valid = (strwidth(*buffer_0) >= 0 as libc::c_int) as libc::c_int;
        if valid == 0 {
            printf(b"Error: %s\n\0" as *const u8 as *const libc::c_char, invalid_utf8);
        }
    }
}
pub unsafe extern "C" fn getuser_readline(
    mut buffer_0: *mut *mut libc::c_char,
    mut bufsize_0: *mut libc::c_int,
    mut query: *const libc::c_char,
) {
    let mut valid: libc::c_int = 0 as libc::c_int;
    while valid == 0 {
        if !(*buffer_0).is_null() {
            free(*buffer_0 as *mut libc::c_void);
        }
        *buffer_0 = readline(query);
        if !(*buffer_0).is_null() {
            replacectrlchars(*buffer_0);
        }
        if (*buffer_0).is_null() || strwidth(*buffer_0) >= 0 as libc::c_int {
            valid = 1 as libc::c_int;
        } else {
            printf(b"Error: %s\n\0" as *const u8 as *const libc::c_char, invalid_utf8);
        }
    }
    if !(*buffer_0).is_null() && **buffer_0 as libc::c_int != 0 {
        add_history(*buffer_0);
    }
    if (*buffer_0).is_null() {
        if flags.quiet == 0 {
            putchar('\n' as i32);
        }
        exit(0 as libc::c_int);
    }
}
pub unsafe extern "C" fn getuser(
    mut buffer_0: *mut *mut libc::c_char,
    mut bufsize_0: *mut libc::c_int,
    mut query: *const libc::c_char,
) {
    if flags.readline != 0 {
        getuser_readline(buffer_0, bufsize_0, query);
    } else {
        getuser_noreadline(buffer_0, bufsize_0, query);
    };
}
pub unsafe extern "C" fn completeunits(
    mut text: *mut libc::c_char,
    mut state: libc::c_int,
) -> *mut libc::c_char {
    static mut uhash_0: libc::c_int = 0;
    static mut fhash: libc::c_int = 0;
    static mut phash: libc::c_int = 0;
    static mut checktype: libc::c_int = 0;
    static mut curprefix: *mut prefixlist = 0 as *const prefixlist as *mut prefixlist;
    static mut unitprefix: *mut prefixlist = 0 as *const prefixlist as *mut prefixlist;
    static mut curunit: *mut unitlist = 0 as *const unitlist as *mut unitlist;
    static mut curfunc: *mut func = 0 as *const func as *mut func;
    static mut curalias: *mut wantalias = 0 as *const wantalias as *mut wantalias;
    static mut curbuiltin: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
        as *mut *mut libc::c_char;
    let mut output: *mut libc::c_char = 0 as *mut libc::c_char;
    rl_completion_suppress_append = 1 as libc::c_int;
    if state == 0 {
        checktype = 0 as libc::c_int;
        phash = 0 as libc::c_int;
        uhash_0 = phash;
        fhash = uhash_0;
        unitprefix = 0 as *mut prefixlist;
        curfunc = ftab[fhash as usize];
        curunit = utab[uhash_0 as usize];
        curprefix = ptab[phash as usize];
        curbuiltin = builtins.as_mut_ptr();
        curalias = firstalias;
    }
    while checktype != 5 as libc::c_int {
        if checktype == 0 as libc::c_int {
            while !curalias.is_null() {
                if strncmp((*curalias).name, text, strlen(text)) == 0 {
                    output = dupstr((*curalias).name);
                }
                curalias = (*curalias).next;
                if !output.is_null() {
                    return output;
                }
            }
            checktype += 1;
            checktype;
        }
        if checktype == 1 as libc::c_int {
            while !(*curbuiltin).is_null() {
                if strncmp(*curbuiltin, text, strlen(text)) == 0 {
                    output = dupstr(*curbuiltin);
                }
                curbuiltin = curbuiltin.offset(1);
                curbuiltin;
                if !output.is_null() {
                    return output;
                }
            }
            checktype += 1;
            checktype;
        }
        while checktype == 2 as libc::c_int {
            while curfunc.is_null() && fhash < 128 as libc::c_int - 1 as libc::c_int {
                fhash += 1;
                fhash;
                curfunc = ftab[fhash as usize];
            }
            if curfunc.is_null() {
                checktype += 1;
                checktype;
            } else {
                if strncmp((*curfunc).name, text, strlen(text)) == 0 {
                    output = dupstr((*curfunc).name);
                }
                curfunc = (*curfunc).next;
                if !output.is_null() {
                    return output;
                }
            }
        }
        while checktype == 3 as libc::c_int {
            while curprefix.is_null() && phash < 128 as libc::c_int - 1 as libc::c_int {
                phash += 1;
                phash;
                curprefix = ptab[phash as usize];
            }
            if curprefix.is_null() {
                checktype += 1;
                checktype;
            } else {
                if strncmp((*curprefix).name, text, strlen(text)) == 0 {
                    output = dupstr((*curprefix).name);
                }
                curprefix = (*curprefix).next;
                if !output.is_null() {
                    return output;
                }
            }
        }
        while checktype == 4 as libc::c_int {
            while curunit.is_null() && uhash_0 < 101 as libc::c_int - 1 as libc::c_int {
                uhash_0 += 1;
                uhash_0;
                curunit = utab[uhash_0 as usize];
            }
            if curunit.is_null() && unitprefix.is_null()
                && {
                    unitprefix = plookup(text);
                    !unitprefix.is_null()
                } && strlen((*unitprefix).name) > 1 as libc::c_int as libc::c_ulong
            {
                uhash_0 = 0 as libc::c_int;
                curunit = utab[uhash_0 as usize];
            }
            if curunit.is_null() {
                checktype += 1;
                checktype;
                break;
            } else {
                if !unitprefix.is_null() {
                    if strncmp(
                        (*curunit).name,
                        text.offset((*unitprefix).len as isize),
                        strlen(text.offset((*unitprefix).len as isize)),
                    ) == 0
                    {
                        output = mymalloc(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(strlen((*curunit).name))
                                .wrapping_add((*unitprefix).len as libc::c_ulong)
                                as libc::c_int,
                            b"(completeunits)\0" as *const u8 as *const libc::c_char,
                        ) as *mut libc::c_char;
                        strcpy(output, (*unitprefix).name);
                        strcat(output, (*curunit).name);
                    }
                } else if strncmp((*curunit).name, text, strlen(text)) == 0 {
                    output = dupstr((*curunit).name);
                }
                curunit = (*curunit).next;
                if !output.is_null() {
                    return output;
                }
            }
        }
    }
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn checkcwd(mut file: *mut libc::c_char) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    fp = openfile(file, b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if !fp.is_null() {
        fclose(fp);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn getprogramname(
    mut path: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut proglen: size_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    path = pathend(path);
    proglen = strlen(path);
    p = strrchr(path, '.' as i32);
    if !p.is_null()
        && (strlen(p) == 4 as libc::c_int as libc::c_ulong
            && ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *p.offset(1 as libc::c_int as isize)
                            as libc::c_int;
                        __res = (if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        });
                    } else {
                        __res = tolower(
                            *p.offset(1 as libc::c_int as isize) as libc::c_int,
                        );
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(
                            *p.offset(1 as libc::c_int as isize) as libc::c_int as isize,
                        );
                }
                __res
            }) == 'e' as i32
            && ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *p.offset(2 as libc::c_int as isize)
                            as libc::c_int;
                        __res = (if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        });
                    } else {
                        __res = tolower(
                            *p.offset(2 as libc::c_int as isize) as libc::c_int,
                        );
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(
                            *p.offset(2 as libc::c_int as isize) as libc::c_int as isize,
                        );
                }
                __res
            }) == 'x' as i32
            && ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *p.offset(3 as libc::c_int as isize)
                            as libc::c_int;
                        __res = (if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        });
                    } else {
                        __res = tolower(
                            *p.offset(3 as libc::c_int as isize) as libc::c_int,
                        );
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(
                            *p.offset(3 as libc::c_int as isize) as libc::c_int as isize,
                        );
                }
                __res
            }) == 'e' as i32)
    {
        proglen = (proglen as libc::c_ulong)
            .wrapping_sub(4 as libc::c_int as libc::c_ulong) as size_t as size_t;
    }
    return dupnstr(path, proglen as libc::c_int);
}
pub unsafe extern "C" fn getprogdir(
    mut progname_0: *mut libc::c_char,
    mut fullprogname_0: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    let mut progdir_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if progdir_0.is_null()
        && (isfullpath(progname_0) != 0 || !(strchr(progname_0, '/' as i32)).is_null())
    {
        progdir_0 = dupstr(progname_0);
    }
    if progdir_0.is_null() {
        let mut env: *mut libc::c_char = 0 as *mut libc::c_char;
        env = getenv(b"PATH\0" as *const u8 as *const libc::c_char);
        if !env.is_null() {
            let mut direc: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut direc_end: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut pathname: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut len: libc::c_int = 0;
            let mut fp: *mut FILE = 0 as *mut FILE;
            pathname = mymalloc(
                (strlen(env))
                    .wrapping_add(strlen(progname_0))
                    .wrapping_add(strlen(b"\0" as *const u8 as *const libc::c_char))
                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int,
                b"(getprogdir)\0" as *const u8 as *const libc::c_char,
            ) as *mut libc::c_char;
            direc = env;
            while !direc.is_null() {
                direc_end = strchr(direc, ':' as i32);
                if direc_end.is_null() {
                    len = strlen(direc) as libc::c_int;
                } else {
                    len = direc_end.offset_from(direc) as libc::c_long as libc::c_int;
                }
                strncpy(pathname, direc, len as libc::c_ulong);
                if len > 0 as libc::c_int {
                    let fresh21 = len;
                    len = len + 1;
                    *pathname.offset(fresh21 as isize) = '/' as i32 as libc::c_char;
                }
                strcpy(pathname.offset(len as isize), progname_0);
                fp = openfile(
                    pathname,
                    b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                if !fp.is_null() {
                    progdir_0 = dupstr(pathname);
                    break;
                } else {
                    direc = direc_end;
                    if !direc.is_null() {
                        direc = direc.offset(1);
                        direc;
                    }
                }
            }
            free(pathname as *mut libc::c_void);
            if !fp.is_null() {
                fclose(fp);
            }
        }
    }
    if progdir_0.is_null() {
        fprintf(
            stderr,
            b"%s: cannot find program directory\n\0" as *const u8 as *const libc::c_char,
            progname_0,
        );
        exit(1 as libc::c_int);
    }
    *fullprogname_0 = dupstr(progdir_0);
    p = pathend(progdir_0);
    *p = '\0' as i32 as libc::c_char;
    return progdir_0;
}
pub unsafe extern "C" fn getdatadir() -> *mut libc::c_char {
    let mut progdirlen: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if isfullpath(
        b"../share/units\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        return b"../share/units\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    if progdir.is_null()
        || *(b"../share/units\0" as *const u8 as *const libc::c_char) as libc::c_int
            == 0 as libc::c_int
    {
        return 0 as *mut libc::c_char;
    }
    progdirlen = strlen(progdir) as libc::c_int;
    datadir = mymalloc(
        (progdirlen as libc::c_ulong)
            .wrapping_add(
                strlen(b"../share/units\0" as *const u8 as *const libc::c_char),
            )
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"(getdatadir)\0" as *const u8 as *const libc::c_char,
    ) as *mut libc::c_char;
    strcpy(datadir, progdir);
    if *progdir.offset((progdirlen - 1 as libc::c_int) as isize) as libc::c_int
        == '/' as i32
    {
        *datadir
            .offset(
                (progdirlen - 1 as libc::c_int) as isize,
            ) = '\0' as i32 as libc::c_char;
    }
    p = pathend(datadir);
    if strlen(p) == 3 as libc::c_int as libc::c_ulong
        && ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *p.offset(0 as libc::c_int as isize)
                        as libc::c_int;
                    __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    });
                } else {
                    __res = tolower(*p.offset(0 as libc::c_int as isize) as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(
                        *p.offset(0 as libc::c_int as isize) as libc::c_int as isize,
                    );
            }
            __res
        }) == 'b' as i32
        && ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *p.offset(1 as libc::c_int as isize)
                        as libc::c_int;
                    __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    });
                } else {
                    __res = tolower(*p.offset(1 as libc::c_int as isize) as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(
                        *p.offset(1 as libc::c_int as isize) as libc::c_int as isize,
                    );
            }
            __res
        }) == 'i' as i32
        && ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *p.offset(2 as libc::c_int as isize)
                        as libc::c_int;
                    __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    });
                } else {
                    __res = tolower(*p.offset(2 as libc::c_int as isize) as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(
                        *p.offset(2 as libc::c_int as isize) as libc::c_int as isize,
                    );
            }
            __res
        }) == 'n' as i32
    {
        p = b"../share/units\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        while *p as libc::c_int == '.' as i32 {
            p = p.offset(1);
            p;
        }
        if *p as libc::c_int == '/' as i32 {
            p = p.offset(1);
            p;
        }
        strcpy(pathend(datadir), p);
        return datadir;
    } else {
        return 0 as *mut libc::c_char
    };
}
pub unsafe extern "C" fn showfilecheck(
    mut errnum: libc::c_int,
    mut filename: *mut libc::c_char,
) {
    if errnum == 2 as libc::c_int {
        printf(b"  Checking %s\n\0" as *const u8 as *const libc::c_char, filename);
    } else {
        printf(
            b"  Checking %s: %s\n\0" as *const u8 as *const libc::c_char,
            filename,
            strerror(errnum),
        );
    };
}
pub unsafe extern "C" fn findunitsfile(mut print: libc::c_int) -> *mut libc::c_char {
    let mut testfile: *mut FILE = 0 as *mut FILE;
    let mut file: *mut libc::c_char = 0 as *mut libc::c_char;
    file = getenv(b"UNITSFILE\0" as *const u8 as *const libc::c_char);
    if !file.is_null() && *file as libc::c_int != 0 {
        testfile = openfile(
            file,
            b"rt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if testfile.is_null() {
            if print == -(3 as libc::c_int) {
                fprintf(
                    stderr,
                    b"%s: cannot open units file '%s' in environment variable UNITSFILE.  \0"
                        as *const u8 as *const libc::c_char,
                    progname,
                    file,
                );
                perror(0 as *mut libc::c_void as *mut libc::c_char);
            }
            return 0 as *mut libc::c_char;
        }
    }
    if testfile.is_null()
        && isfullpath(
            b"/usr/local/share/units/definitions.units\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        ) != 0
    {
        file = b"/usr/local/share/units/definitions.units\0" as *const u8
            as *const libc::c_char as *mut libc::c_char;
        testfile = openfile(
            file,
            b"rt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if testfile.is_null() {
            if print == -(3 as libc::c_int) {
                fprintf(
                    stderr,
                    b"%s: cannot open units data file '%s'.  \0" as *const u8
                        as *const libc::c_char,
                    progname,
                    b"/usr/local/share/units/definitions.units\0" as *const u8
                        as *const libc::c_char,
                );
                perror(0 as *mut libc::c_void as *mut libc::c_char);
            }
            return 0 as *mut libc::c_char;
        }
    }
    if testfile.is_null() && progdir.is_null() {
        if print == -(3 as libc::c_int) {
            fprintf(
                stderr,
                b"%s: cannot open units file '%s' and cannot find program directory.\n\0"
                    as *const u8 as *const libc::c_char,
                progname,
                b"/usr/local/share/units/definitions.units\0" as *const u8
                    as *const libc::c_char,
            );
            perror(0 as *mut libc::c_void as *mut libc::c_char);
        }
        return 0 as *mut libc::c_char;
    }
    if testfile.is_null() {
        file = mymalloc(
            (strlen(progdir))
                .wrapping_add(
                    strlen(
                        b"/usr/local/share/units/definitions.units\0" as *const u8
                            as *const libc::c_char,
                    ),
                )
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            b"(findunitsfile)\0" as *const u8 as *const libc::c_char,
        ) as *mut libc::c_char;
        strcpy(file, progdir);
        strcat(
            file,
            b"/usr/local/share/units/definitions.units\0" as *const u8
                as *const libc::c_char,
        );
        testfile = openfile(
            file,
            b"rt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if print == -(4 as libc::c_int) && testfile.is_null() {
            showfilecheck(*__errno_location(), file);
        }
        if testfile.is_null() {
            free(file as *mut libc::c_void);
        }
    }
    if testfile.is_null() && !datadir.is_null() {
        file = mymalloc(
            (strlen(datadir))
                .wrapping_add(
                    strlen(
                        b"/usr/local/share/units/definitions.units\0" as *const u8
                            as *const libc::c_char,
                    ),
                )
                .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int,
            b"(findunitsfile)\0" as *const u8 as *const libc::c_char,
        ) as *mut libc::c_char;
        strcpy(file, datadir);
        strcat(file, b"/\0" as *const u8 as *const libc::c_char);
        strcat(
            file,
            b"/usr/local/share/units/definitions.units\0" as *const u8
                as *const libc::c_char,
        );
        testfile = openfile(
            file,
            b"rt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if print == -(4 as libc::c_int) && testfile.is_null() {
            showfilecheck(*__errno_location(), file);
        }
        if testfile.is_null() {
            free(file as *mut libc::c_void);
        }
    }
    if testfile.is_null() {
        if print == -(3 as libc::c_int) {
            fprintf(
                stderr,
                b"%s: cannot find units file '%s'\n\0" as *const u8
                    as *const libc::c_char,
                progname,
                b"/usr/local/share/units/definitions.units\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return 0 as *mut libc::c_char;
    } else {
        fclose(testfile);
        return file;
    };
}
pub unsafe extern "C" fn findhome(
    mut errmsg: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    let mut statbuf: stat = stat {
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
        st_atime: 0,
        st_atimensec: 0,
        st_mtime: 0,
        st_mtimensec: 0,
        st_ctime: 0,
        st_ctimensec: 0,
        __glibc_reserved: [0; 3],
    };
    let mut allocated: libc::c_int = 0 as libc::c_int;
    let mut homedir_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut notfound: [libc::c_char; 45] = *::std::mem::transmute::<
        &[u8; 45],
        &mut [libc::c_char; 45],
    >(b"Specified home directory '%s' does not exist\0");
    let mut notadir: [libc::c_char; 49] = *::std::mem::transmute::<
        &[u8; 49],
        &mut [libc::c_char; 49],
    >(b"Specified home directory '%s' is not a directory\0");
    homedir_0 = getenv(b"HOME\0" as *const u8 as *const libc::c_char);
    if !homedir_0.is_null() && *homedir_0 as libc::c_int != 0 {
        if stat(homedir_0, &mut statbuf) != 0 as libc::c_int {
            *errmsg = malloc(
                (strlen(notfound.as_mut_ptr())).wrapping_add(strlen(homedir_0)),
            ) as *mut libc::c_char;
            sprintf(*errmsg, notfound.as_mut_ptr(), homedir_0);
        } else if statbuf.st_mode & 0o40000 as libc::c_int as libc::c_uint == 0 {
            *errmsg = malloc(
                (strlen(notadir.as_mut_ptr())).wrapping_add(strlen(homedir_0)),
            ) as *mut libc::c_char;
            sprintf(*errmsg, notadir.as_mut_ptr(), homedir_0);
        }
        if allocated == 0 {
            homedir_0 = dupstr(homedir_0);
        }
        return homedir_0;
    } else {
        *errmsg = b"no home directory\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        return 0 as *mut libc::c_char;
    };
}
pub unsafe extern "C" fn personalfile(
    mut envname: *const libc::c_char,
    mut basename: *const libc::c_char,
    mut checkonly: libc::c_int,
    mut exists: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut testfile: *mut FILE = 0 as *mut FILE;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    *exists = 0 as libc::c_int;
    if !envname.is_null() {
        filename = getenv(envname);
    }
    if !filename.is_null() && *filename as libc::c_int != 0 {
        testfile = openfile(
            filename,
            b"rt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !testfile.is_null() {
            fclose(testfile);
            *exists = 1 as libc::c_int;
            return filename;
        }
        if checkonly != 0 {
            return filename
        } else {
            fprintf(
                stderr,
                b"%s: cannot open file '%s' specified in %s environment variable: \0"
                    as *const u8 as *const libc::c_char,
                progname,
                filename,
                envname,
            );
            perror(0 as *mut libc::c_void as *mut libc::c_char);
            return 0 as *mut libc::c_char;
        }
    } else if !homedir.is_null() && *homedir as libc::c_int != 0 {
        filename = mymalloc(
            (strlen(homedir))
                .wrapping_add(strlen(basename))
                .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int,
            b"(personalfile)\0" as *const u8 as *const libc::c_char,
        ) as *mut libc::c_char;
        strcpy(filename, homedir);
        if strcmp(homedir, b"/\0" as *const u8 as *const libc::c_char) != 0
            && strcmp(homedir, b"\\\0" as *const u8 as *const libc::c_char) != 0
        {
            strcat(filename, b"/\0" as *const u8 as *const libc::c_char);
        }
        strcat(filename, basename);
        testfile = openfile(
            filename,
            b"rt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !testfile.is_null() {
            fclose(testfile);
            *exists = 1 as libc::c_int;
            return filename;
        }
        if checkonly != 0 {
            return filename
        } else {
            if *__errno_location() == 13 as libc::c_int
                || *__errno_location() == 21 as libc::c_int
            {
                fprintf(
                    stderr,
                    b"%s: cannot open file '%s': \0" as *const u8 as *const libc::c_char,
                    progname,
                    filename,
                );
                perror(0 as *const libc::c_char);
            }
            free(filename as *mut libc::c_void);
            return 0 as *mut libc::c_char;
        }
    } else {
        return 0 as *mut libc::c_char
    };
}
pub unsafe extern "C" fn usage() {
    let mut nlines: libc::c_int = 0;
    let mut unitsfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut msg: *mut libc::c_char = b"\nUsage: %s [options] ['from-unit' 'to-unit']\n\nOptions:\n    -h, --help           show this help and exit\n    -c, --check          check that all units reduce to primitive units\n        --check-verbose  like --check, but lists units as they are checked\n        --verbose-check    so you can find units that cause endless loops\n    -d, --digits         show output to specified number of digits (default: %d)\n    -e, --exponential    exponential format output\n    -f, --file           specify a units data file (-f '' loads default file)\n    -H, --history        specify readline history file (-H '' disables history)\n    -L, --log            specify a file to log conversions\n    -l, --locale         specify a desired locale\n    -m, --minus          make - into a subtraction operator (default)\n        --oldstar        use old '*' precedence, higher than '/'\n        --newstar        use new '*' precedence, equal to '/'\n    -n, --nolists        disable conversion to unit lists\n    -S, --show-factor    show non-unity factor before 1|x in multi-unit output\n        --conformable    in non-interactive mode, show all conformable units\n    -o, --output-format  specify printf numeric output format (default: %%.%d%c)\n    -p, --product        make '-' into a product operator\n    -q, --quiet          suppress prompting\n        --silent         same as --quiet\n    -s, --strict         suppress reciprocal unit conversion (e.g. Hz<->s)\n    -v, --verbose        show slightly more verbose output\n        --compact        suppress printing of tab, '*', and '/' character\n    -1, --one-line       suppress the second line of output\n    -t, --terse          terse output (--strict --compact --quiet --one-line)\n    -r, --round          round last element of unit list output to an integer\n    -U, --unitsfile      show units data filename and exit\n    -u, --units          specify a CGS units system or natural units system:\n                            gauss[ian],esu,emu,hlu,natural,natural-gauss,\n                            hartree,planck,planck-red,si\n    -V, --version        show version, data filenames (with -t: version only)\n    -I, --info           show version, files, and program properties\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    let mut fp: *mut FILE = 0 as *mut FILE;
    unitsfile = findunitsfile(-(2 as libc::c_int));
    nlines = countlines(msg);
    fp = get_output_fp(nlines + 4 as libc::c_int);
    fprintf(fp, msg, progname, 8 as libc::c_int, 8 as libc::c_int, 'g' as i32);
    if unitsfile.is_null() {
        fprintf(
            fp,
            b"Units data file '%s' not found.\n\n\0" as *const u8 as *const libc::c_char,
            b"/usr/local/share/units/definitions.units\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        fprintf(
            fp,
            b"\nTo learn about the available units look in '%s'\n\n\0" as *const u8
                as *const libc::c_char,
            unitsfile,
        );
    }
    fputs(
        b"Report bugs to adrianm@gnu.org.\n\n\0" as *const u8 as *const libc::c_char,
        fp,
    );
    if fp != stdout {
        pclose(fp);
    }
}
pub unsafe extern "C" fn helpmsg() {
    fprintf(
        stderr,
        b"\nTry '%s --help' for more information.\n\0" as *const u8
            as *const libc::c_char,
        progname,
    );
    exit(1 as libc::c_int);
}
pub unsafe extern "C" fn printversion() {
    let mut exists: libc::c_int = 0;
    let mut u_unitsfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut m_unitsfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p_unitsfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut histfile: *mut FILE = 0 as *mut FILE;
    if flags.verbose == 0 as libc::c_int {
        printf(
            b"GNU Units version %s\n\0" as *const u8 as *const libc::c_char,
            b"2.22\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    printf(
        b"GNU Units version %s\n%s, %s, locale %s\n\0" as *const u8
            as *const libc::c_char,
        b"2.22\0" as *const u8 as *const libc::c_char,
        b"with readline\0" as *const u8 as *const libc::c_char,
        b"with utf8\0" as *const u8 as *const libc::c_char,
        mylocale,
    );
    if flags.verbose == 2 as libc::c_int {
        if fullprogname.is_null() {
            getprogdir(progname, &mut fullprogname);
        }
        if !fullprogname.is_null() {
            printf(
                b"\n%s program is %s\n\0" as *const u8 as *const libc::c_char,
                progname,
                fullprogname,
            );
        }
    }
    putchar('\n' as i32);
    if isfullpath(
        b"/usr/local/share/units/definitions.units\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ) != 0
    {
        printf(
            b"Default units data file is '%s'\n\0" as *const u8 as *const libc::c_char,
            b"/usr/local/share/units/definitions.units\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        printf(
            b"Default units data file is '%s';\n  %s will search for this file\n\0"
                as *const u8 as *const libc::c_char,
            b"/usr/local/share/units/definitions.units\0" as *const u8
                as *const libc::c_char,
            progname,
        );
    }
    if flags.verbose < 2 as libc::c_int {
        printf(
            b"Default personal units file: %s\n\0" as *const u8 as *const libc::c_char,
            homeunitsfile,
        );
    }
    if flags.verbose == 2 as libc::c_int {
        u_unitsfile = getenv(b"UNITSFILE\0" as *const u8 as *const libc::c_char);
        if !u_unitsfile.is_null() {
            printf(
                b"Environment variable UNITSFILE set to '%s'\n\0" as *const u8
                    as *const libc::c_char,
                u_unitsfile,
            );
        } else {
            puts(
                b"Environment variable UNITSFILE not set\0" as *const u8
                    as *const libc::c_char,
            );
        }
        unitsfiles[0 as libc::c_int as usize] = findunitsfile(-(4 as libc::c_int));
        if !(unitsfiles[0 as libc::c_int as usize]).is_null() {
            if isfullpath(
                b"/usr/local/share/units/definitions.units\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            ) == 0 && !(!u_unitsfile.is_null() && *u_unitsfile as libc::c_int != 0)
            {
                printf(
                    b"Found data file '%s'\n\0" as *const u8 as *const libc::c_char,
                    unitsfiles[0 as libc::c_int as usize],
                );
            } else {
                printf(
                    b"Units data file is '%s'\n\0" as *const u8 as *const libc::c_char,
                    unitsfiles[0 as libc::c_int as usize],
                );
            }
        } else if *__errno_location() != 0
            && (!u_unitsfile.is_null() && *u_unitsfile as libc::c_int != 0
                || isfullpath(
                    b"/usr/local/share/units/definitions.units\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                ) != 0)
        {
            printf(
                b"*** Units data file invalid: %s ***\n\0" as *const u8
                    as *const libc::c_char,
                strerror(*__errno_location()),
            );
        } else {
            puts(
                b"*** Units data file not found ***\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if !homedir_error.is_null() {
            printf(b"\n%s\n\0" as *const u8 as *const libc::c_char, homedir_error);
        } else {
            printf(
                b"\nHome directory is '%s'\n\0" as *const u8 as *const libc::c_char,
                homedir,
            );
        }
    }
    if flags.verbose == 2 as libc::c_int {
        m_unitsfile = getenv(b"MYUNITSFILE\0" as *const u8 as *const libc::c_char);
        putchar('\n' as i32);
        if !m_unitsfile.is_null() {
            printf(
                b"Environment variable %s set to '%s'\n\0" as *const u8
                    as *const libc::c_char,
                b"MYUNITSFILE\0" as *const u8 as *const libc::c_char,
                m_unitsfile,
            );
        } else {
            printf(
                b"Environment variable %s not set\n\0" as *const u8
                    as *const libc::c_char,
                b"MYUNITSFILE\0" as *const u8 as *const libc::c_char,
            );
        }
        p_unitsfile = personalfile(
            b"MYUNITSFILE\0" as *const u8 as *const libc::c_char,
            homeunitsfile,
            1 as libc::c_int,
            &mut exists,
        );
        if !p_unitsfile.is_null() {
            printf(
                b"Personal units data file is '%s'\n\0" as *const u8
                    as *const libc::c_char,
                p_unitsfile,
            );
            if exists == 0 {
                if !homedir_error.is_null()
                    && !(!m_unitsfile.is_null() && *m_unitsfile as libc::c_int != 0)
                {
                    printf(
                        b"  (File invalid: %s)\n\0" as *const u8 as *const libc::c_char,
                        homedir_error,
                    );
                } else if *__errno_location() == 2 as libc::c_int
                    && !(!m_unitsfile.is_null() && *m_unitsfile as libc::c_int != 0)
                {
                    puts(
                        b"  (File does not exist)\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    printf(
                        b"  (File invalid: %s)\n\0" as *const u8 as *const libc::c_char,
                        strerror(*__errno_location()),
                    );
                }
            }
        } else {
            puts(
                b"Personal units data file not found: no home directory\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    if flags.verbose == 2 as libc::c_int {
        historyfile = personalfile(
            0 as *const libc::c_char,
            b".units_history\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
            &mut exists,
        );
        if !historyfile.is_null() {
            printf(
                b"\nDefault readline history file is '%s'\n\0" as *const u8
                    as *const libc::c_char,
                historyfile,
            );
            histfile = openfile(
                historyfile,
                b"r+\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            if histfile.is_null() {
                printf(
                    b"  (File invalid: %s)\n\0" as *const u8 as *const libc::c_char,
                    if !homedir_error.is_null() {
                        homedir_error
                    } else {
                        strerror(*__errno_location())
                    },
                );
            } else {
                fclose(histfile);
            }
        } else {
            puts(
                b"\nReadline history file unusable: no home directory\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    printf(
        b"\n%s\n\n\0" as *const u8 as *const libc::c_char,
        b"Copyright (C) 2022 Free Software Foundation, Inc.\nGNU Units comes with ABSOLUTELY NO WARRANTY.\nYou may redistribute copies of GNU Units\nunder the terms of the GNU General Public License.\0"
            as *const u8 as *const libc::c_char,
    );
}
pub unsafe extern "C" fn showunitsfile() {
    let mut unitsfile: *mut libc::c_char = 0 as *mut libc::c_char;
    unitsfile = findunitsfile(-(2 as libc::c_int));
    if !unitsfile.is_null() {
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, unitsfile);
    } else {
        puts(b"Units data file not found\0" as *const u8 as *const libc::c_char);
    };
}
pub static mut shortoptions: *mut libc::c_char = b"VIUu:vqechSstf:o:d:mnpr1l:L:H:\0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
pub static mut longoptions: [option; 31] = [option {
    name: 0 as *const libc::c_char,
    has_arg: 0,
    flag: 0 as *mut libc::c_int,
    val: 0,
}; 31];
pub unsafe extern "C" fn processargs(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut from: *mut *mut libc::c_char,
    mut to: *mut *mut libc::c_char,
) -> libc::c_int {
    extern "C" {
        #[link_name = "optarg"]
        static mut optarg_0: *mut libc::c_char;
    }
    extern "C" {
        #[link_name = "optind"]
        static mut optind_0: libc::c_int;
    }
    let mut optchar: libc::c_int = 0;
    let mut optindex: libc::c_int = 0;
    let mut ind: libc::c_int = 0;
    let mut doprintversion: libc::c_int = 0 as libc::c_int;
    let mut unitsys: *mut libc::c_char = 0 as *mut libc::c_char;
    loop {
        optchar = getopt_long(
            argc,
            argv,
            shortoptions,
            longoptions.as_mut_ptr(),
            &mut optindex,
        );
        if !(-(1 as libc::c_int) != optchar) {
            break;
        }
        let mut current_block_49: u64;
        match optchar {
            109 => {
                parserflags.minusminus = 1 as libc::c_int;
                current_block_49 = 5235537862154438448;
            }
            112 => {
                parserflags.minusminus = 0 as libc::c_int;
                current_block_49 = 5235537862154438448;
            }
            116 => {
                flags.oneline = 1 as libc::c_int;
                flags.quiet = 1 as libc::c_int;
                flags.strictconvert = 1 as libc::c_int;
                flags.verbose = 0 as libc::c_int;
                current_block_49 = 5235537862154438448;
            }
            100 => {
                if checksigdigits(optarg) < 0 as libc::c_int {
                    exit(1 as libc::c_int);
                } else {
                    num_format.format = 0 as *mut libc::c_char;
                }
                current_block_49 = 5235537862154438448;
            }
            101 => {
                num_format.format = 0 as *mut libc::c_char;
                num_format.type_0 = 'e' as i32 as libc::c_char;
                current_block_49 = 5235537862154438448;
            }
            111 => {
                num_format.format = optarg;
                current_block_49 = 5235537862154438448;
            }
            99 => {
                flags.unitcheck = 1 as libc::c_int;
                current_block_49 = 5235537862154438448;
            }
            102 => {
                ind = 0 as libc::c_int;
                while !(unitsfiles[ind as usize]).is_null() {
                    ind += 1;
                    ind;
                }
                if ind == 25 as libc::c_int {
                    fprintf(
                        stderr,
                        b"At most %d -f specifications are allowed\n\0" as *const u8
                            as *const libc::c_char,
                        25 as libc::c_int,
                    );
                    exit(1 as libc::c_int);
                }
                if !optarg.is_null() && *optarg as libc::c_int != 0 {
                    unitsfiles[ind as usize] = optarg;
                } else {
                    unitsfiles[ind as usize] = findunitsfile(-(3 as libc::c_int));
                    if (unitsfiles[ind as usize]).is_null() {
                        exit(1 as libc::c_int);
                    }
                }
                unitsfiles[(ind + 1 as libc::c_int) as usize] = 0 as *mut libc::c_char;
                current_block_49 = 5235537862154438448;
            }
            76 => {
                logfilename = optarg;
                current_block_49 = 5235537862154438448;
            }
            108 => {
                mylocale = optarg;
                current_block_49 = 5235537862154438448;
            }
            110 => {
                flags.unitlists = 0 as libc::c_int;
                current_block_49 = 5235537862154438448;
            }
            113 => {
                flags.quiet = 1 as libc::c_int;
                current_block_49 = 5235537862154438448;
            }
            114 => {
                flags.round = 1 as libc::c_int;
                current_block_49 = 5235537862154438448;
            }
            83 => {
                flags.showfactor = 1 as libc::c_int;
                current_block_49 = 5235537862154438448;
            }
            115 => {
                flags.strictconvert = 1 as libc::c_int;
                current_block_49 = 5235537862154438448;
            }
            118 => {
                flags.verbose = 2 as libc::c_int;
                current_block_49 = 5235537862154438448;
            }
            49 => {
                flags.oneline = 1 as libc::c_int;
                current_block_49 = 5235537862154438448;
            }
            73 => {
                flags.verbose = 2 as libc::c_int;
                current_block_49 = 333387252042382477;
            }
            86 => {
                current_block_49 = 333387252042382477;
            }
            85 => {
                showunitsfile();
                exit(0 as libc::c_int);
            }
            117 => {
                unitsys = optarg;
                ind = 0 as libc::c_int;
                while *unitsys.offset(ind as isize) != 0 {
                    *unitsys
                        .offset(
                            ind as isize,
                        ) = ({
                        let mut __res: libc::c_int = 0;
                        if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                            > 1 as libc::c_int as libc::c_ulong
                        {
                            if 0 != 0 {
                                let mut __c: libc::c_int = *unitsys.offset(ind as isize)
                                    as libc::c_int;
                                __res = if __c < -(128 as libc::c_int)
                                    || __c > 255 as libc::c_int
                                {
                                    __c
                                } else {
                                    *(*__ctype_tolower_loc()).offset(__c as isize)
                                };
                            } else {
                                __res = tolower(
                                    *unitsys.offset(ind as isize) as libc::c_int,
                                );
                            }
                        } else {
                            __res = *(*__ctype_tolower_loc())
                                .offset(
                                    *unitsys.offset(ind as isize) as libc::c_int as isize,
                                );
                        }
                        __res
                    }) as libc::c_char;
                    ind += 1;
                    ind;
                }
                current_block_49 = 5235537862154438448;
            }
            104 => {
                usage();
                exit(0 as libc::c_int);
            }
            72 => {
                if *optarg as libc::c_int == 0 as libc::c_int {
                    historyfile = 0 as *mut libc::c_char;
                } else {
                    historyfile = optarg;
                }
                current_block_49 = 5235537862154438448;
            }
            0 => {
                current_block_49 = 5235537862154438448;
            }
            63 | _ => {
                helpmsg();
                current_block_49 = 5235537862154438448;
            }
        }
        match current_block_49 {
            333387252042382477 => {
                doprintversion = 1 as libc::c_int;
            }
            _ => {}
        }
    }
    if doprintversion != 0 {
        printversion();
        exit(0 as libc::c_int);
    }
    if !unitsys.is_null() {
        setenv(
            b"UNITS_SYSTEM\0" as *const u8 as *const libc::c_char,
            unitsys,
            1 as libc::c_int,
        );
    }
    if flags.unitcheck != 0 {
        if optind != argc {
            fprintf(
                stderr,
                b"Too many arguments (arguments are not allowed with -c).\n\0"
                    as *const u8 as *const libc::c_char,
            );
            helpmsg();
        }
    } else {
        if optind == argc - 2 as libc::c_int {
            if flags.showconformable != 0 {
                fprintf(
                    stderr,
                    b"Too many arguments (only one unit expression allowed with '--conformable').\n\0"
                        as *const u8 as *const libc::c_char,
                );
                helpmsg();
            }
            flags.quiet = 1 as libc::c_int;
            *from = *argv.offset(optind as isize);
            *to = dupstr(*argv.offset((optind + 1 as libc::c_int) as isize));
            return 0 as libc::c_int;
        }
        if optind == argc - 1 as libc::c_int {
            flags.quiet = 1 as libc::c_int;
            *from = *argv.offset(optind as isize);
            *to = 0 as *mut libc::c_char;
            return 0 as libc::c_int;
        }
        if optind < argc - 2 as libc::c_int {
            fprintf(
                stderr,
                b"Too many arguments (maybe you need quotes).\n\0" as *const u8
                    as *const libc::c_char,
            );
            helpmsg();
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn showpointer(mut position: libc::c_int) {
    if position >= 0 as libc::c_int {
        loop {
            let fresh22 = position;
            position = position - 1;
            if !(fresh22 != 0) {
                break;
            }
            putchar(' ' as i32);
        }
        puts(b"^\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn processunit(
    mut theunit: *mut unittype,
    mut unitstr: *mut libc::c_char,
    mut promptlen: libc::c_int,
) -> libc::c_int {
    let mut errmsg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut errloc: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut savechar: libc::c_char = 0;
    if flags.unitlists != 0 && !(strchr(unitstr, ';' as i32)).is_null() {
        puts(b"Unit list not allowed\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    err = parseunit(theunit, unitstr, &mut errmsg, &mut errloc);
    if err != 0 {
        if promptlen >= 0 as libc::c_int {
            if err != 7 as libc::c_int || irreducible.is_null() {
                if errloc > 0 as libc::c_int {
                    savechar = *unitstr.offset(errloc as isize);
                    *unitstr.offset(errloc as isize) = 0 as libc::c_int as libc::c_char;
                    showpointer(promptlen + strwidth(unitstr) - 1 as libc::c_int);
                    *unitstr.offset(errloc as isize) = savechar;
                } else {
                    showpointer(promptlen);
                }
            }
        } else {
            printf(b"Error in '%s': \0" as *const u8 as *const libc::c_char, unitstr);
        }
        fputs(errmsg, stdout);
        if err == 7 as libc::c_int && !irreducible.is_null() {
            printf(b" '%s'\0" as *const u8 as *const libc::c_char, irreducible);
        }
        putchar('\n' as i32);
        return 1 as libc::c_int;
    }
    err = completereduce(theunit);
    if err != 0 {
        fputs(errormsg[err as usize], stdout);
        if err == 7 as libc::c_int {
            printf(b" '%s'\0" as *const u8 as *const libc::c_char, irreducible);
        }
        putchar('\n' as i32);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn checkunitlist(
    mut unitstr: *mut libc::c_char,
    mut promptlen: libc::c_int,
) -> libc::c_int {
    let mut unit: [unittype; 2] = [unittype {
        numerator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        denominator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        factor: 0.,
    }; 2];
    let mut one: unittype = unittype {
        numerator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        denominator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        factor: 0.,
    };
    let mut firstunitstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nextunitstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut unitidx: libc::c_int = 0 as libc::c_int;
    let mut printerror: libc::c_int = (promptlen != -(2 as libc::c_int)) as libc::c_int;
    initializeunit(&mut one);
    firstunitstr = unitstr;
    initializeunit(unit.as_mut_ptr());
    initializeunit(unit.as_mut_ptr().offset(1 as libc::c_int as isize));
    while !unitstr.is_null() {
        nextunitstr = strchr(unitstr, ';' as i32);
        if !nextunitstr.is_null() {
            *nextunitstr = '\0' as i32 as libc::c_char;
        }
        if *unitstr
            .offset(strspn(unitstr, b" \0" as *const u8 as *const libc::c_char) as isize)
            == 0
        {
            if nextunitstr.is_null() {
                freeunit(unit.as_mut_ptr());
                return 0 as libc::c_int;
            } else {
                if printerror != 0 {
                    showpointer(promptlen);
                    puts(
                        b"Error: blank unit not allowed\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                freeunit(unit.as_mut_ptr());
                return 1 as libc::c_int;
            }
        }
        if printerror != 0
            && processunit(
                unit.as_mut_ptr().offset(unitidx as isize),
                unitstr,
                promptlen,
            ) != 0
            || printerror == 0
                && (parseunit(
                    unit.as_mut_ptr().offset(unitidx as isize),
                    unitstr,
                    0 as *mut *mut libc::c_char,
                    0 as *mut libc::c_int,
                ) != 0 || completereduce(unit.as_mut_ptr().offset(unitidx as isize)) != 0
                    || compareunits(
                        unit.as_mut_ptr().offset(unitidx as isize),
                        &mut one,
                        Some(
                            ignore_primitive
                                as unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int,
                        ),
                    ) != 0)
        {
            if printerror != 0 {
                printf(
                    b"Error in unit list entry: %s\n\0" as *const u8
                        as *const libc::c_char,
                    unitstr,
                );
            }
            freeunit(unit.as_mut_ptr());
            freeunit(unit.as_mut_ptr().offset(1 as libc::c_int as isize));
            return 1 as libc::c_int;
        }
        if unitidx == 0 as libc::c_int {
            unitidx = 1 as libc::c_int;
        } else {
            if compareunits(
                unit.as_mut_ptr(),
                unit.as_mut_ptr().offset(1 as libc::c_int as isize),
                Some(
                    ignore_dimless
                        as unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int,
                ),
            ) != 0
            {
                if printerror != 0 {
                    let mut wasverbose: libc::c_int = flags.verbose;
                    let mut savelog: *mut FILE = logfile;
                    logfile = 0 as *mut FILE;
                    flags.verbose = 2 as libc::c_int;
                    *strchr(firstunitstr, ';' as i32) = '\0' as i32 as libc::c_char;
                    removespaces(firstunitstr);
                    removespaces(unitstr);
                    showpointer(promptlen);
                    showconformabilityerr(
                        firstunitstr,
                        unit.as_mut_ptr(),
                        unitstr,
                        unit.as_mut_ptr().offset(1 as libc::c_int as isize),
                    );
                    flags.verbose = wasverbose;
                    logfile = savelog;
                }
                freeunit(unit.as_mut_ptr());
                freeunit(unit.as_mut_ptr().offset(1 as libc::c_int as isize));
                return 1 as libc::c_int;
            }
            freeunit(unit.as_mut_ptr().offset(1 as libc::c_int as isize));
        }
        if !nextunitstr.is_null() {
            if promptlen >= 0 as libc::c_int {
                promptlen += strwidth(unitstr) + 1 as libc::c_int;
            }
            let fresh23 = nextunitstr;
            nextunitstr = nextunitstr.offset(1);
            *fresh23 = ';' as i32 as libc::c_char;
        }
        unitstr = nextunitstr;
    }
    freeunit(unit.as_mut_ptr());
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn processwant(
    mut theunit: *mut unittype,
    mut unitstr: *mut libc::c_char,
    mut promptlen: libc::c_int,
) -> libc::c_int {
    if flags.unitlists != 0 && !(strchr(unitstr, ';' as i32)).is_null() {
        return checkunitlist(unitstr, promptlen)
    } else {
        return processunit(theunit, unitstr, promptlen)
    };
}
pub unsafe extern "C" fn checkallaliases(mut verbose: libc::c_int) {
    let mut aliasptr: *mut wantalias = 0 as *mut wantalias;
    aliasptr = firstalias;
    while !aliasptr.is_null() {
        if verbose != 0 {
            printf(
                b"doing unit list '%s'\n\0" as *const u8 as *const libc::c_char,
                (*aliasptr).name,
            );
        }
        if checkunitlist((*aliasptr).definition, -(2 as libc::c_int)) != 0 {
            printf(
                b"Unit list '%s' contains errors\n\0" as *const u8
                    as *const libc::c_char,
                (*aliasptr).name,
            );
        }
        if !(ulookup((*aliasptr).name)).is_null() {
            printf(
                b"Unit list '%s' hides a unit definition.\n\0" as *const u8
                    as *const libc::c_char,
                (*aliasptr).name,
            );
        }
        if !(fnlookup((*aliasptr).name)).is_null() {
            printf(
                b"Unit list '%s' hides a function definition.\n\0" as *const u8
                    as *const libc::c_char,
                (*aliasptr).name,
            );
        }
        aliasptr = (*aliasptr).next;
    }
}
pub unsafe extern "C" fn checkunits(mut verbosecheck: libc::c_int) {
    let mut have: unittype = unittype {
        numerator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        denominator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        factor: 0.,
    };
    let mut second: unittype = unittype {
        numerator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        denominator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        factor: 0.,
    };
    let mut one: unittype = unittype {
        numerator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        denominator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        factor: 0.,
    };
    let mut uptr: *mut unitlist = 0 as *mut unitlist;
    let mut pptr: *mut prefixlist = 0 as *mut prefixlist;
    let mut funcptr: *mut func = 0 as *mut func;
    let mut prefixbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut testunit: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    initializeunit(&mut one);
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        funcptr = ftab[i as usize];
        while !funcptr.is_null() {
            checkfunc(funcptr, verbosecheck);
            funcptr = (*funcptr).next;
        }
        i += 1;
        i;
    }
    checkallaliases(verbosecheck);
    i = 0 as libc::c_int;
    while i < 101 as libc::c_int {
        uptr = utab[i as usize];
        while !uptr.is_null() {
            if verbosecheck != 0 {
                printf(
                    b"doing '%s'\n\0" as *const u8 as *const libc::c_char,
                    (*uptr).name,
                );
            }
            if parseunit(
                &mut have,
                (*uptr).name,
                0 as *mut *mut libc::c_char,
                0 as *mut libc::c_int,
            ) != 0 || completereduce(&mut have) != 0
                || compareunits(
                    &mut have,
                    &mut one,
                    Some(
                        ignore_primitive
                            as unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int,
                    ),
                ) != 0
            {
                if !(fnlookup((*uptr).name)).is_null() {
                    printf(
                        b"Unit '%s' hidden by function '%s'\n\0" as *const u8
                            as *const libc::c_char,
                        (*uptr).name,
                        (*uptr).name,
                    );
                } else {
                    printf(
                        b"'%s' defined as '%s' irreducible\n\0" as *const u8
                            as *const libc::c_char,
                        (*uptr).name,
                        (*uptr).value,
                    );
                }
            } else {
                parserflags.minusminus = (parserflags.minusminus == 0) as libc::c_int;
                parseunit(
                    &mut second,
                    (*uptr).name,
                    0 as *mut *mut libc::c_char,
                    0 as *mut libc::c_int,
                );
                completereduce(&mut second);
                if compareunits(
                    &mut have,
                    &mut second,
                    Some(
                        ignore_nothing
                            as unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int,
                    ),
                ) != 0
                {
                    printf(
                        b"'%s': replace '-' with '+-' for subtraction or '*' to multiply\n\0"
                            as *const u8 as *const libc::c_char,
                        (*uptr).name,
                    );
                }
                freeunit(&mut second);
                parserflags.minusminus = (parserflags.minusminus == 0) as libc::c_int;
            }
            freeunit(&mut have);
            uptr = (*uptr).next;
        }
        i += 1;
        i;
    }
    testunit = b"meter\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        pptr = ptab[i as usize];
        while !pptr.is_null() {
            if verbosecheck != 0 {
                printf(
                    b"doing '%s-'\n\0" as *const u8 as *const libc::c_char,
                    (*pptr).name,
                );
            }
            prefixbuf = mymalloc(
                (strlen((*pptr).name))
                    .wrapping_add(strlen(testunit))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                b"(checkunits)\0" as *const u8 as *const libc::c_char,
            ) as *mut libc::c_char;
            strcpy(prefixbuf, (*pptr).name);
            strcat(prefixbuf, testunit);
            if parseunit(
                &mut have,
                prefixbuf,
                0 as *mut *mut libc::c_char,
                0 as *mut libc::c_int,
            ) != 0 || completereduce(&mut have) != 0
                || compareunits(
                    &mut have,
                    &mut one,
                    Some(
                        ignore_primitive
                            as unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int,
                    ),
                ) != 0
            {
                printf(
                    b"'%s-' defined as '%s' irreducible\n\0" as *const u8
                        as *const libc::c_char,
                    (*pptr).name,
                    (*pptr).value,
                );
            } else {
                let mut plevel: libc::c_int = 0;
                let mut ch: *mut libc::c_char = 0 as *mut libc::c_char;
                plevel = 0 as libc::c_int;
                ch = (*pptr).value;
                while *ch != 0 {
                    if *ch as libc::c_int == ')' as i32 {
                        plevel -= 1;
                        plevel;
                    } else if *ch as libc::c_int == '(' as i32 {
                        plevel += 1;
                        plevel;
                    } else if plevel == 0 as libc::c_int
                        && *ch as libc::c_int == '/' as i32
                    {
                        printf(
                            b"'%s-' defined as '%s' contains a bad '/'. (Add parentheses.)\n\0"
                                as *const u8 as *const libc::c_char,
                            (*pptr).name,
                            (*pptr).value,
                        );
                        break;
                    }
                    ch = ch.offset(1);
                    ch;
                }
            }
            freeunit(&mut have);
            free(prefixbuf as *mut libc::c_void);
            pptr = (*pptr).next;
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn showunitlist(
    mut havestr: *mut libc::c_char,
    mut have: *mut unittype,
    mut wantstr: *mut libc::c_char,
) -> libc::c_int {
    let mut want: unittype = unittype {
        numerator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        denominator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        factor: 0.,
    };
    let mut lastwant: unittype = unittype {
        numerator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        denominator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        factor: 0.,
    };
    let mut lastunitstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nextunitstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lastwantstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut remainder: libc::c_double = 0.;
    let mut round_dir: libc::c_double = 0.;
    let mut value: libc::c_double = 0.;
    let mut firstunit: libc::c_int = 1 as libc::c_int;
    let mut value_shown: libc::c_int = 0 as libc::c_int;
    let mut sigdigits: libc::c_int = 0;
    let mut val_sign: libc::c_char = 0;
    initializeunit(&mut want);
    remainder = fabs((*have).factor);
    val_sign = (if (*have).factor < 0 as libc::c_int as libc::c_double {
        '-' as i32
    } else {
        '+' as i32
    }) as libc::c_char;
    lastunitstr = 0 as *mut libc::c_char;
    nextunitstr = 0 as *mut libc::c_char;
    round_dir = 0 as libc::c_int as libc::c_double;
    if flags.round != 0 {
        if *wantstr.offset(strlen(wantstr) as isize).offset(-(1 as libc::c_int as isize))
            as libc::c_int == ';' as i32
        {
            *wantstr
                .offset(strlen(wantstr) as isize)
                .offset(-(1 as libc::c_int as isize)) = 0 as libc::c_int as libc::c_char;
        }
        lastwantstr = strrchr(wantstr, ';' as i32);
        if !lastwantstr.is_null() {
            lastwantstr = lastwantstr.offset(1);
            lastwantstr;
        }
    }
    while !wantstr.is_null() {
        nextunitstr = strchr(wantstr, ';' as i32);
        if !nextunitstr.is_null() {
            let fresh24 = nextunitstr;
            nextunitstr = nextunitstr.offset(1);
            *fresh24 = '\0' as i32 as libc::c_char;
        }
        removespaces(wantstr);
        if *wantstr as libc::c_int == 0 as libc::c_int {
            wantstr = lastunitstr;
        }
        if processunit(&mut want, wantstr, -(1 as libc::c_int)) != 0 {
            freeunit(&mut want);
            return 1 as libc::c_int;
        }
        if firstunit != 0 {
            if compareunits(
                have,
                &mut want,
                Some(
                    ignore_dimless
                        as unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int,
                ),
            ) != 0
            {
                showconformabilityerr(havestr, have, wantstr, &mut want);
                freeunit(&mut want);
                return 1 as libc::c_int;
            }
            if flags.round != 0 {
                value = remainder;
                if !lastwantstr.is_null() && *lastwantstr as libc::c_int != 0 {
                    removespaces(lastwantstr);
                    initializeunit(&mut lastwant);
                    if processunit(&mut lastwant, lastwantstr, -(1 as libc::c_int)) != 0
                    {
                        freeunit(&mut lastwant);
                        return 1 as libc::c_int;
                    }
                    remainder = round(remainder / lastwant.factor) * lastwant.factor;
                } else {
                    remainder = round(remainder / want.factor) * want.factor;
                }
                round_dir = remainder - value;
            }
            if flags.verbose == 2 as libc::c_int {
                removespaces(havestr);
                logprintf(b"\t%s = \0" as *const u8 as *const libc::c_char, havestr);
            } else if flags.verbose == 1 as libc::c_int {
                logputchar('\t' as i32 as libc::c_char);
            }
        }
        sigdigits = getsigdigits((*have).factor, remainder, 10 as libc::c_int);
        if 0 as libc::c_int == sigdigits {
            break;
        }
        remainder = round_digits(
            remainder,
            getsigdigits((*have).factor, remainder, 2 as libc::c_int),
            2 as libc::c_int,
        );
        if !nextunitstr.is_null() {
            remainder = want.factor * modf(remainder / want.factor, &mut value);
        } else {
            value = remainder / want.factor;
        }
        if !nextunitstr.is_null() {
            let mut rounded_next: libc::c_double = round_digits(
                remainder / want.factor,
                getsigdigits((*have).factor, remainder / want.factor, 10 as libc::c_int),
                10 as libc::c_int,
            );
            if displays_as(
                1 as libc::c_int as libc::c_double,
                rounded_next,
                0 as *mut libc::c_int,
            ) != 0
            {
                value += 1.;
                value;
                remainder = 0 as libc::c_int as libc::c_double;
            }
        }
        value = round_digits(value, sigdigits, 10 as libc::c_int);
        if nextunitstr.is_null()
            && displays_as(
                0 as libc::c_int as libc::c_double,
                value,
                0 as *mut libc::c_int,
            ) != 0
        {
            value = 0 as libc::c_int as libc::c_double;
        }
        if flags.verbose == 0 {
            if firstunit == 0 {
                logputchar(';' as i32 as libc::c_char);
            }
            logprintf(num_format.format, value);
            value_shown = 1 as libc::c_int;
        } else if value != 0 as libc::c_int as libc::c_double {
            if value_shown != 0 {
                logprintf(
                    b" %c \0" as *const u8 as *const libc::c_char,
                    val_sign as libc::c_int,
                );
            } else if val_sign as libc::c_int == '-' as i32 {
                logputs(b"-\0" as *const u8 as *const libc::c_char);
            }
            showunitname(value, wantstr, 1 as libc::c_int);
            if sigdigits as libc::c_double
                <= floor(log10(value)) + 1 as libc::c_int as libc::c_double
            {
                logprintf(
                    b" (at %d-digit precision limit)\0" as *const u8
                        as *const libc::c_char,
                    15 as libc::c_int,
                );
            }
            value_shown = 1 as libc::c_int;
        }
        freeunit(&mut want);
        lastunitstr = wantstr;
        wantstr = nextunitstr;
        firstunit = 0 as libc::c_int;
    }
    if value_shown == 0 {
        logputs(b"0 \0" as *const u8 as *const libc::c_char);
        if isdecimal(*lastunitstr) != 0 {
            logputs(b"* \0" as *const u8 as *const libc::c_char);
        }
        logputs(lastunitstr);
    }
    if round_dir != 0 as libc::c_int as libc::c_double {
        if flags.verbose != 0 {
            if round_dir > 0 as libc::c_int as libc::c_double {
                logprintf(
                    b" (rounded up to nearest %s) \0" as *const u8
                        as *const libc::c_char,
                    lastunitstr,
                );
            } else {
                logprintf(
                    b" (rounded down to nearest %s) \0" as *const u8
                        as *const libc::c_char,
                    lastunitstr,
                );
            }
        } else {
            logprintf(
                b"%c%c\0" as *const u8 as *const libc::c_char,
                ';' as i32,
                if round_dir > 0 as libc::c_int as libc::c_double {
                    '-' as i32
                } else {
                    '+' as i32
                },
            );
        }
    }
    logputchar('\n' as i32 as libc::c_char);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ishelpquery(
    mut str: *mut libc::c_char,
    mut have: *mut unittype,
) -> libc::c_int {
    let mut unit: *mut unitlist = 0 as *mut unitlist;
    let mut function: *mut func = 0 as *mut func;
    let mut alias: *mut wantalias = 0 as *mut wantalias;
    let mut prefix: *mut prefixlist = 0 as *mut prefixlist;
    let mut commandbuf: [libc::c_char; 1000] = [0; 1000];
    let mut unitline: libc::c_int = 0;
    let mut file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut exitptr: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut commandstr: *mut libc::c_char = 0 as *mut libc::c_char;
    if !have.is_null() && strcmp(str, b"?\0" as *const u8 as *const libc::c_char) == 0 {
        tryallunits(have, 0 as *mut libc::c_char);
        return 1 as libc::c_int;
    }
    exitptr = exit_commands.as_mut_ptr();
    while !(*exitptr).is_null() {
        if strcmp(str, *exitptr) == 0 {
            exit(0 as libc::c_int);
        }
        exitptr = exitptr.offset(1);
        exitptr;
    }
    if strncmp(
        str,
        b"search\0" as *const u8 as *const libc::c_char,
        strlen(b"search\0" as *const u8 as *const libc::c_char),
    ) == 0
    {
        str = str
            .offset(strlen(b"search\0" as *const u8 as *const libc::c_char) as isize);
        if !(*str as libc::c_int == 0 as libc::c_int)
            && *str as libc::c_int != ' ' as i32
        {
            return 0 as libc::c_int;
        }
        removespaces(str);
        if *str as libc::c_int == 0 as libc::c_int {
            printf(
                b"\nType 'search text' to see a list of all unit names \ncontaining 'text' as a substring\n\n\0"
                    as *const u8 as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
        tryallunits(0 as *mut unittype, str);
        return 1 as libc::c_int;
    }
    if strncmp(
        str,
        b"help\0" as *const u8 as *const libc::c_char,
        strlen(b"help\0" as *const u8 as *const libc::c_char),
    ) == 0
    {
        str = str.offset(strlen(b"help\0" as *const u8 as *const libc::c_char) as isize);
        if !(*str as libc::c_int == 0 as libc::c_int)
            && *str as libc::c_int != ' ' as i32
        {
            return 0 as libc::c_int;
        }
        removespaces(str);
        if *str as libc::c_int == 0 as libc::c_int {
            let mut nlines: libc::c_int = 0;
            let mut unitsfile: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut msg: *mut libc::c_char = b"\n%s converts between different measuring systems and    %s6 inches\nacts as a units-aware calculator.  At the '%s'    %scm\nprompt, type in the units you want to convert from or             * 15.24\nan expression to evaluate.  At the '%s' prompt,           / 0.065\nenter the units to convert to or press return to see\nthe reduced form or definition.                           %stempF(75)\n                                                          %stempC\nThe first example shows that 6 inches is about 15 cm              23.889\nor (1/0.065) cm.  The second example shows how to\nconvert 75 degrees Fahrenheit to Celsius.  The third      %sbu^(1/3)\nexample converts the cube root of a bushel to a list      %sft;in\nof semicolon-separated units.                                     1 ft + 0.9 in\n\nTo quit from %s type 'quit' or 'exit'.       %s2 btu + 450 ft lbf\n                                                %s(kg^2/s)/(day lb/m^2)\nAt the '%s' prompt type '%s' to get a            * 1.0660684e+08\nlist of conformable units.  At either prompt you        / 9.3802611e-09\ntype 'help myunit' to browse the units database\nand read the comments relating to myunit or see         %s6 tbsp sugar\nother units related to myunit.  Typing 'search          %sg\ntext' will show units whose names contain 'text'.               * 75\n                                                                / 0.013333333\n\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char;
            let mut fmsg: *mut libc::c_char = b"To learn about the available units look in '%s'\n\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char;
            let mut fp: *mut FILE = 0 as *mut FILE;
            unitsfile = findunitsfile(-(2 as libc::c_int));
            nlines = countlines(msg);
            if !unitsfile.is_null() {
                nlines += countlines(fmsg);
            }
            fp = get_output_fp(nlines);
            fprintf(
                fp,
                msg,
                progname,
                b"You have: \0" as *const u8 as *const libc::c_char,
                b"You have: \0" as *const u8 as *const libc::c_char,
                b"You want: \0" as *const u8 as *const libc::c_char,
                b"You want: \0" as *const u8 as *const libc::c_char,
                b"You have: \0" as *const u8 as *const libc::c_char,
                b"You want: \0" as *const u8 as *const libc::c_char,
                b"You have: \0" as *const u8 as *const libc::c_char,
                b"You want: \0" as *const u8 as *const libc::c_char,
                progname,
                b"You have: \0" as *const u8 as *const libc::c_char,
                b"You want: \0" as *const u8 as *const libc::c_char,
                b"You want: \0" as *const u8 as *const libc::c_char,
                b"?\0" as *const u8 as *const libc::c_char,
                b"You have: \0" as *const u8 as *const libc::c_char,
                b"You want: \0" as *const u8 as *const libc::c_char,
            );
            if !unitsfile.is_null() {
                fprintf(fp, fmsg, unitsfile);
            }
            if fp != stdout {
                pclose(fp);
            }
            return 1 as libc::c_int;
        }
        function = fnlookup(str);
        if !function.is_null() {
            file = (*function).file;
            unitline = (*function).linenumber;
        } else {
            unit = ulookup(str);
            if !unit.is_null() {
                unitline = (*unit).linenumber;
                file = (*unit).file;
            } else {
                prefix = plookup(str);
                if !prefix.is_null() && strlen(str) == (*prefix).len as libc::c_ulong {
                    unitline = (*prefix).linenumber;
                    file = (*prefix).file;
                } else {
                    alias = aliaslookup(str);
                    if !alias.is_null() {
                        unitline = (*alias).linenumber;
                        file = (*alias).file;
                    } else {
                        printf(
                            b"Unknown unit '%s'\n\0" as *const u8 as *const libc::c_char,
                            str,
                        );
                        return 1 as libc::c_int;
                    }
                }
            }
        }
        commandstr = b"%s +%d %s\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        sprintf(commandbuf.as_mut_ptr(), commandstr, pager, unitline, file);
        if system(commandbuf.as_mut_ptr()) != 0 {
            fprintf(
                stderr,
                b"%s: cannot invoke pager '%s' to display help\n\0" as *const u8
                    as *const libc::c_char,
                progname,
                pager,
            );
        }
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn checklocale() {
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    temp = setlocale(0 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    utf8mode = (strcmp(
        nl_langinfo(CODESET as libc::c_int),
        b"UTF-8\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int) as libc::c_int;
    if !temp.is_null() {
        mylocale = dupstr(temp);
        temp = strchr(mylocale, '.' as i32);
        if !temp.is_null() {
            *temp = 0 as libc::c_int as libc::c_char;
        }
    } else {
        mylocale = b"en_US\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    };
}
pub unsafe extern "C" fn replacealias(
    mut string: *mut *mut libc::c_char,
    mut buflen: *mut libc::c_int,
) -> libc::c_int {
    let mut usefree: libc::c_int = 1 as libc::c_int;
    let mut aliasptr: *mut wantalias = 0 as *mut wantalias;
    let mut input: *mut libc::c_char = 0 as *mut libc::c_char;
    if flags.readline == 0 && !buflen.is_null() {
        usefree = 0 as libc::c_int;
    }
    if !(*string).is_null() && **string as libc::c_int != 0 {
        input = *string;
        removespaces(input);
        aliasptr = aliaslookup(input);
        if !aliasptr.is_null() {
            if checkunitlist((*aliasptr).definition, -(2 as libc::c_int)) != 0 {
                puts(
                    b"Unit list definition contains errors.\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if usefree != 0 {
                free(*string as *mut libc::c_void);
                *string = dupstr((*aliasptr).definition);
            } else {
                while strlen((*aliasptr).definition) > *buflen as libc::c_ulong {
                    growbuffer(string, buflen);
                }
                strcpy(*string, (*aliasptr).definition);
            }
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn remaplocale(mut filename: *mut libc::c_char) {
    let mut map: *mut FILE = 0 as *mut FILE;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: [libc::c_char; 80] = [0; 80];
    map = openfile(
        filename,
        b"rt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if map.is_null() {
        fprintf(
            stderr,
            b"%s: cannot open locale map '%s'. \0" as *const u8 as *const libc::c_char,
            progname,
            filename,
        );
        perror(0 as *mut libc::c_void as *mut libc::c_char);
    } else {
        while feof(map) == 0 {
            if (fgets(name.as_mut_ptr(), 80 as libc::c_int, map)).is_null() {
                break;
            }
            *name
                .as_mut_ptr()
                .offset(strlen(name.as_mut_ptr()) as isize)
                .offset(-(1 as libc::c_int as isize)) = 0 as libc::c_int as libc::c_char;
            value = strchr(name.as_mut_ptr(), '#' as i32);
            if !value.is_null() {
                *value = 0 as libc::c_int as libc::c_char;
            }
            value = strchr(name.as_mut_ptr(), '\t' as i32);
            if value.is_null() {
                continue;
            }
            let fresh25 = value;
            value = value.offset(1);
            *fresh25 = 0 as libc::c_int as libc::c_char;
            removespaces(value);
            removespaces(name.as_mut_ptr());
            if strcmp(name.as_mut_ptr(), mylocale) == 0 {
                mylocale = dupstr(value);
            }
        }
        fclose(map);
    };
}
pub unsafe extern "C" fn close_logfile() {
    if !logfile.is_null() {
        fputc('\n' as i32, logfile);
        fclose(logfile);
    }
}
pub unsafe extern "C" fn open_logfile() {
    let mut logtime: time_t = 0;
    let mut timestr: *mut libc::c_char = 0 as *mut libc::c_char;
    logfile = openfile(
        logfilename,
        b"at\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if logfile.is_null() {
        fprintf(
            stderr,
            b"%s: cannot write to log file '%s'.  \0" as *const u8
                as *const libc::c_char,
            progname,
            logfilename,
        );
        perror(0 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    time(&mut logtime);
    timestr = ctime(&mut logtime);
    fprintf(
        logfile,
        b"### Log started %s \n\0" as *const u8 as *const libc::c_char,
        timestr,
    );
    atexit(Some(close_logfile as unsafe extern "C" fn() -> ()));
}
pub unsafe extern "C" fn write_files_sig(mut sig: libc::c_int) {
    if !historyfile.is_null() {
        save_history();
    }
    close_logfile();
    signal(sig, None);
    raise(sig);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    static mut have: unittype = unittype {
        numerator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        denominator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        factor: 0.,
    };
    static mut want: unittype = unittype {
        numerator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        denominator: [0 as *const libc::c_char as *mut libc::c_char; 100],
        factor: 0.,
    };
    let mut havestr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut wantstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut funcval: *mut func = 0 as *mut func;
    let mut alias: *mut wantalias = 0 as *mut wantalias;
    let mut havestrsize: libc::c_int = 0 as libc::c_int;
    let mut wantstrsize: libc::c_int = 0 as libc::c_int;
    let mut readerr: libc::c_int = 0;
    let mut unitfileptr: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut unitcount: libc::c_int = 0 as libc::c_int;
    let mut prefixcount: libc::c_int = 0 as libc::c_int;
    let mut funccount: libc::c_int = 0 as libc::c_int;
    let mut queryhave: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut querywant: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut comment: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut queryhavewidth: libc::c_int = 0;
    let mut querywantwidth: libc::c_int = 0;
    num_format.format = 0 as *mut libc::c_char;
    num_format.precision = 8 as libc::c_int;
    num_format.type_0 = 'g' as i32 as libc::c_char;
    flags.quiet = 0 as libc::c_int;
    flags.unitcheck = 0 as libc::c_int;
    flags.verbose = 1 as libc::c_int;
    flags.round = 0 as libc::c_int;
    flags.strictconvert = 0 as libc::c_int;
    flags.unitlists = 1 as libc::c_int;
    flags.oneline = 0 as libc::c_int;
    flags.showconformable = 0 as libc::c_int;
    flags.showfactor = 0 as libc::c_int;
    parserflags.minusminus = 1 as libc::c_int;
    parserflags.oldstar = 0 as libc::c_int;
    progname = getprogramname(*argv.offset(0 as libc::c_int as isize));
    if !(isfullpath(
        b"/usr/local/share/units/definitions.units\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ) != 0
        && isfullpath(
            b"/usr/local/share/units/locale_map.txt\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        ) != 0)
    {
        progdir = getprogdir(*argv.offset(0 as libc::c_int as isize), &mut fullprogname);
    } else {
        progdir = 0 as *mut libc::c_char;
        fullprogname = 0 as *mut libc::c_char;
    }
    datadir = getdatadir();
    checklocale();
    homedir = findhome(&mut homedir_error);
    rl_completion_entry_function = ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(*mut libc::c_char, libc::c_int) -> *mut libc::c_char,
        >,
        Option::<rl_compentry_func_t>,
    >(
        Some(
            completeunits
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    libc::c_int,
                ) -> *mut libc::c_char,
        ),
    );
    rl_basic_word_break_characters = b" \t+-*/()|^;\0" as *const u8
        as *const libc::c_char;
    flags.readline = isatty(0 as libc::c_int);
    if flags.readline != 0 {
        let mut file_exists: libc::c_int = 0;
        historyfile = personalfile(
            0 as *const libc::c_char,
            b".units_history\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
            &mut file_exists,
        );
    }
    unitsfiles[0 as libc::c_int as usize] = 0 as *mut libc::c_char;
    pager = getenv(b"PAGER\0" as *const u8 as *const libc::c_char);
    if pager.is_null() {
        pager = b"/usr/bin/pager\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    flags.interactive = processargs(argc, argv, &mut havestr, &mut wantstr);
    if flags.interactive != 0 && flags.readline != 0 && !historyfile.is_null() {
        rl_initialize();
        read_history(historyfile);
        init_history_length = history_length;
        init_history_base = history_base;
        atexit(Some(save_history as unsafe extern "C" fn() -> ()));
    }
    signal(
        2 as libc::c_int,
        Some(write_files_sig as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    signal(
        15 as libc::c_int,
        Some(write_files_sig as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    if !logfilename.is_null() {
        if flags.interactive == 0 {
            fprintf(
                stderr,
                b"Log file '%s' ignored in non-interactive mode.\n\0" as *const u8
                    as *const libc::c_char,
                logfilename,
            );
        } else {
            open_logfile();
        }
    }
    if !(num_format.format).is_null() {
        if parsenumformat() != 0 {
            exit(1 as libc::c_int);
        }
    } else {
        setnumformat();
    }
    if flags.verbose == 0 as libc::c_int {
        deftext = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (unitsfiles[0 as libc::c_int as usize]).is_null() {
        let mut unitsfile: *mut libc::c_char = 0 as *mut libc::c_char;
        unitsfile = findunitsfile(-(3 as libc::c_int));
        if unitsfile.is_null() {
            exit(1 as libc::c_int);
        } else {
            let mut file_exists_0: libc::c_int = 0;
            unitsfiles[0 as libc::c_int as usize] = unitsfile;
            unitsfiles[1 as libc::c_int
                as usize] = personalfile(
                b"MYUNITSFILE\0" as *const u8 as *const libc::c_char,
                homeunitsfile,
                0 as libc::c_int,
                &mut file_exists_0,
            );
            unitsfiles[2 as libc::c_int as usize] = 0 as *mut libc::c_char;
        }
    }
    unitfileptr = unitsfiles.as_mut_ptr();
    while !(*unitfileptr).is_null() {
        readerr = readunits(
            *unitfileptr,
            stderr,
            &mut unitcount,
            &mut prefixcount,
            &mut funccount,
            0 as libc::c_int,
        );
        if readerr == 18 as libc::c_int || readerr == 16 as libc::c_int {
            exit(1 as libc::c_int);
        }
        unitfileptr = unitfileptr.offset(1);
        unitfileptr;
    }
    if flags.quiet != 0 {
        querywant = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        queryhave = querywant;
    } else {
        if promptprefix.is_null() {
            queryhave = b"You have: \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            querywant = b"You want: \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        } else {
            queryhave = mymalloc(
                (strlen(promptprefix))
                    .wrapping_add(
                        strlen(b"You have: \0" as *const u8 as *const libc::c_char),
                    )
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                b"(main)\0" as *const u8 as *const libc::c_char,
            ) as *mut libc::c_char;
            querywant = mymalloc(
                (strlen(promptprefix))
                    .wrapping_add(
                        strlen(b"You want: \0" as *const u8 as *const libc::c_char),
                    )
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                b"(main)\0" as *const u8 as *const libc::c_char,
            ) as *mut libc::c_char;
            strcpy(queryhave, promptprefix);
            strcat(queryhave, b"You have: \0" as *const u8 as *const libc::c_char);
            memset(querywant as *mut libc::c_void, ' ' as i32, strlen(promptprefix));
            strcpy(
                querywant.offset(strlen(promptprefix) as isize),
                b"You want: \0" as *const u8 as *const libc::c_char,
            );
        }
        printf(
            b"%d units, %d prefixes, %d nonlinear units\n\n\0" as *const u8
                as *const libc::c_char,
            unitcount,
            prefixcount,
            funccount,
        );
    }
    queryhavewidth = strwidth(queryhave);
    querywantwidth = strwidth(querywant);
    if flags.unitcheck != 0 {
        checkunits(
            (flags.unitcheck == 2 as libc::c_int || flags.verbose == 2 as libc::c_int)
                as libc::c_int,
        );
        exit(0 as libc::c_int);
    }
    if flags.interactive == 0 {
        replacectrlchars(havestr);
        if !wantstr.is_null() {
            replacectrlchars(wantstr);
        }
        if strwidth(havestr) < 0 as libc::c_int {
            printf(
                b"Error: %s on input\n\0" as *const u8 as *const libc::c_char,
                invalid_utf8,
            );
            exit(1 as libc::c_int);
        }
        if !wantstr.is_null() && strwidth(wantstr) < 0 as libc::c_int {
            printf(
                b"Error: %s on input\n\0" as *const u8 as *const libc::c_char,
                invalid_utf8,
            );
            exit(1 as libc::c_int);
        }
        replace_minus(havestr);
        removespaces(havestr);
        if !wantstr.is_null() {
            replace_minus(wantstr);
            removespaces(wantstr);
        }
        funcval = fnlookup(havestr);
        if !funcval.is_null() {
            showfuncdefinition(funcval, 0 as libc::c_int);
            exit(0 as libc::c_int);
        }
        funcval = invfnlookup(havestr);
        if !funcval.is_null() {
            showfuncdefinition(funcval, 1 as libc::c_int);
            exit(0 as libc::c_int);
        }
        alias = aliaslookup(havestr);
        if !alias.is_null() {
            showunitlistdef(alias);
            exit(0 as libc::c_int);
        }
        if processunit(&mut have, havestr, -(1 as libc::c_int)) != 0 {
            exit(1 as libc::c_int);
        }
        if flags.showconformable == 1 as libc::c_int {
            tryallunits(&mut have, 0 as *mut libc::c_char);
            exit(0 as libc::c_int);
        }
        if wantstr.is_null() {
            showdefinition(havestr, &mut have);
            exit(0 as libc::c_int);
        }
        if replacealias(&mut wantstr, 0 as *mut libc::c_int) != 0 {
            exit(1 as libc::c_int);
        }
        funcval = fnlookup(wantstr);
        if !funcval.is_null() {
            if showfunc(havestr, &mut have, funcval) != 0 {
                exit(1 as libc::c_int);
            } else {
                exit(0 as libc::c_int);
            }
        }
        if processwant(&mut want, wantstr, -(1 as libc::c_int)) != 0 {
            exit(1 as libc::c_int);
        }
        if !(strchr(wantstr, ';' as i32)).is_null() {
            if showunitlist(havestr, &mut have, wantstr) != 0 {
                exit(1 as libc::c_int);
            } else {
                exit(0 as libc::c_int);
            }
        }
        if showanswer(havestr, &mut have, wantstr, &mut want) != 0 {
            exit(1 as libc::c_int);
        } else {
            exit(0 as libc::c_int);
        }
    } else {
        loop {
            loop {
                fflush(stdout);
                getuser(&mut havestr, &mut havestrsize, queryhave);
                replace_minus(havestr);
                comment = strip_comment(havestr);
                removespaces(havestr);
                if !logfile.is_null() && !comment.is_null()
                    && *havestr as libc::c_int == 0 as libc::c_int
                {
                    fprintf(
                        logfile,
                        b"#%s\n\0" as *const u8 as *const libc::c_char,
                        comment,
                    );
                }
                if !(*havestr as libc::c_int == 0 as libc::c_int
                    || ishelpquery(havestr, 0 as *mut unittype) != 0
                    || (fnlookup(havestr)).is_null() && (invfnlookup(havestr)).is_null()
                        && (aliaslookup(havestr)).is_null()
                        && processunit(&mut have, havestr, queryhavewidth) != 0)
                {
                    break;
                }
            }
            if !logfile.is_null() {
                if !comment.is_null() {
                    fprintf(
                        logfile,
                        b"%s%s\t#%s\n\0" as *const u8 as *const libc::c_char,
                        b"From: \0" as *const u8 as *const libc::c_char,
                        havestr,
                        comment,
                    );
                } else {
                    fprintf(
                        logfile,
                        b"%s%s\n\0" as *const u8 as *const libc::c_char,
                        b"From: \0" as *const u8 as *const libc::c_char,
                        havestr,
                    );
                }
            }
            alias = aliaslookup(havestr);
            if !alias.is_null() {
                showunitlistdef(alias);
            } else {
                funcval = fnlookup(havestr);
                if !funcval.is_null() {
                    showfuncdefinition(funcval, 0 as libc::c_int);
                } else {
                    funcval = invfnlookup(havestr);
                    if !funcval.is_null() {
                        showfuncdefinition(funcval, 1 as libc::c_int);
                    } else {
                        loop {
                            let mut repeat: libc::c_int = 0;
                            loop {
                                repeat = 0 as libc::c_int;
                                fflush(stdout);
                                getuser(&mut wantstr, &mut wantstrsize, querywant);
                                replace_minus(wantstr);
                                comment = strip_comment(wantstr);
                                removespaces(wantstr);
                                if !logfile.is_null() && !comment.is_null()
                                    && *wantstr as libc::c_int == 0 as libc::c_int
                                {
                                    fprintf(
                                        logfile,
                                        b"#%s\n\0" as *const u8 as *const libc::c_char,
                                        comment,
                                    );
                                    repeat = 1 as libc::c_int;
                                }
                                if ishelpquery(wantstr, &mut have) != 0 {
                                    repeat = 1 as libc::c_int;
                                    printf(
                                        b"%s%s\n\0" as *const u8 as *const libc::c_char,
                                        queryhave,
                                        havestr,
                                    );
                                }
                                if !(repeat != 0) {
                                    break;
                                }
                            }
                            if !(replacealias(&mut wantstr, &mut wantstrsize) != 0
                                || (fnlookup(wantstr)).is_null()
                                    && processwant(&mut want, wantstr, querywantwidth) != 0)
                            {
                                break;
                            }
                        }
                        if !logfile.is_null() {
                            fprintf(
                                logfile,
                                b"%s\0" as *const u8 as *const libc::c_char,
                                b"To:   \0" as *const u8 as *const libc::c_char,
                            );
                            tightprint(logfile, wantstr);
                            if !comment.is_null() {
                                fprintf(
                                    logfile,
                                    b"\t#%s\0" as *const u8 as *const libc::c_char,
                                    comment,
                                );
                            }
                            putc('\n' as i32, logfile);
                        }
                        if *wantstr as libc::c_int == 0 as libc::c_int {
                            showdefinition(havestr, &mut have);
                        } else if !(strchr(wantstr, ';' as i32)).is_null() {
                            showunitlist(havestr, &mut have, wantstr);
                        } else {
                            funcval = fnlookup(wantstr);
                            if !funcval.is_null() {
                                showfunc(havestr, &mut have, funcval);
                            } else {
                                showanswer(havestr, &mut have, wantstr, &mut want);
                                freeunit(&mut want);
                            }
                        }
                        unitcopy(&mut lastunit, &mut have);
                        lastunitset = 1 as libc::c_int;
                        freeunit(&mut have);
                    }
                }
            }
        }
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
unsafe extern "C" fn run_static_initializers() {
    longoptions = [
        {
            let mut init = option {
                name: b"check\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut flags.unitcheck,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"check-verbose\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut flags.unitcheck,
                val: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"compact\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut flags.verbose,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"digits\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'd' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"exponential\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'e' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"file\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'f' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"help\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'h' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"history\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'H' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"info\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'I' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"locale\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'l' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"log\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'L' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"minus\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut parserflags.minusminus,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"newstar\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut parserflags.oldstar,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"nolists\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'n' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"oldstar\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut parserflags.oldstar,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"one-line\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut flags.oneline,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"output-format\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'o' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"product\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut parserflags.minusminus,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"quiet\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut flags.quiet,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"round\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'r' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"show-factor\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'S' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"conformable\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut flags.showconformable,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"silent\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut flags.quiet,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"strict\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut flags.strictconvert,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"terse\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 't' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"unitsfile\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'U' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"units\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'u' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"verbose\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut flags.verbose,
                val: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"verbose-check\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut flags.unitcheck,
                val: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"version\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'V' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: 0 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 0 as libc::c_int,
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
