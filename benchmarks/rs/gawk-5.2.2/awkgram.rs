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
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn mbrtowc(
        __pwc: *mut wchar_t,
        __s: *const libc::c_char,
        __n: size_t,
        __p: *mut mbstate_t,
    ) -> size_t;
    fn __mbrlen(__s: *const libc::c_char, __n: size_t, __ps: *mut mbstate_t) -> size_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    static mut CONVFMT: *const libc::c_char;
    static mut CONVFMTidx: libc::c_int;
    static mut FS_node: *mut NODE;
    static mut FPAT_node: *mut NODE;
    static mut Nnull_string: *mut NODE;
    static mut sourceline: libc::c_int;
    static mut source: *mut libc::c_char;
    static mut make_number: Option::<unsafe extern "C" fn(libc::c_double) -> *mut NODE>;
    static mut str2number: Option::<unsafe extern "C" fn(*mut NODE) -> *mut NODE>;
    static mut format_val: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int, *mut NODE) -> *mut NODE,
    >;
    static mut nextfree: [block_header; 2];
    static mut srcfiles: *mut SRCFILE;
    static mut do_flags: do_flag_values;
    static mut do_optimize: bool;
    static mut gawk_mb_cur_max: libc::c_int;
    fn r_unref(tmp: *mut NODE);
    fn do_asort(nargs: libc::c_int) -> *mut NODE;
    fn do_asorti(nargs: libc::c_int) -> *mut NODE;
    static mut hash: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            size_t,
            libc::c_ulong,
            *mut size_t,
        ) -> libc::c_ulong,
    >;
    fn install_symbol(name: *const libc::c_char, type_0: NODETYPE) -> *mut NODE;
    fn err(
        isfatal: bool,
        s: *const libc::c_char,
        emsg: *const libc::c_char,
        argp: ::std::ffi::VaList,
    );
    fn msg(mesg: *const libc::c_char, _: ...);
    fn lookup(name: *const libc::c_char) -> *mut NODE;
    fn r_fatal(mesg: *const libc::c_char, _: ...);
    fn set_loc(file: *const libc::c_char, line: libc::c_int);
    fn check_param_names() -> bool;
    static mut lintfunc: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
    fn in_main_context() -> libc::c_int;
    fn bcfree(_: *mut INSTRUCTION);
    fn bcalloc(op: OPCODE, size: libc::c_int, srcline: libc::c_int) -> *mut INSTRUCTION;
    fn opcode2str(type_0: OPCODE) -> *const libc::c_char;
    fn r_dupnode(n: *mut NODE) -> *mut NODE;
    fn estrdup(str: *const libc::c_char, len: size_t) -> *mut libc::c_char;
    fn is_all_upper(name: *const libc::c_char) -> bool;
    fn is_std_var(var: *const libc::c_char) -> libc::c_int;
    fn do_index(nargs: libc::c_int) -> *mut NODE;
    fn pp_string_fp(
        print_func: Func_print,
        fp_0: *mut FILE,
        str: *const libc::c_char,
        namelen: size_t,
        delim: libc::c_int,
        breaklines: bool,
    );
    fn r_warning(mesg: *const libc::c_char, _: ...);
    fn do_dcngettext(nargs: libc::c_int) -> *mut NODE;
    fn do_dcgettext(nargs: libc::c_int) -> *mut NODE;
    fn do_close(nargs: libc::c_int) -> *mut NODE;
    fn make_regexp(
        s: *const libc::c_char,
        len: size_t,
        ignorecase: bool,
        dfa: bool,
        canfatal: bool,
    ) -> *mut Regexp;
    fn more_blocks(id: libc::c_int) -> *mut libc::c_void;
    fn do_patsplit(nargs: libc::c_int) -> *mut NODE;
    fn do_split(nargs: libc::c_int) -> *mut NODE;
    fn do_match(nargs: libc::c_int) -> *mut NODE;
    fn do_typeof(nargs: libc::c_int) -> *mut NODE;
    fn do_isarray(nargs: libc::c_int) -> *mut NODE;
    fn do_length(nargs: libc::c_int) -> *mut NODE;
    fn do_xor(nargs: libc::c_int) -> *mut NODE;
    fn do_toupper(nargs: libc::c_int) -> *mut NODE;
    fn do_tolower(nargs: libc::c_int) -> *mut NODE;
    fn do_systime(nargs: libc::c_int) -> *mut NODE;
    fn do_system(nargs: libc::c_int) -> *mut NODE;
    fn do_substr(nargs: libc::c_int) -> *mut NODE;
    fn do_strtonum(nargs: libc::c_int) -> *mut NODE;
    fn do_strftime(nargs: libc::c_int) -> *mut NODE;
    fn do_srand(nargs: libc::c_int) -> *mut NODE;
    fn do_sqrt(nargs: libc::c_int) -> *mut NODE;
    fn do_sprintf(nargs: libc::c_int) -> *mut NODE;
    fn do_sin(nargs: libc::c_int) -> *mut NODE;
    fn do_rshift(nargs: libc::c_int) -> *mut NODE;
    fn do_rand(nargs: libc::c_int) -> *mut NODE;
    fn do_or(nargs: libc::c_int) -> *mut NODE;
    fn do_mktime(nargs: libc::c_int) -> *mut NODE;
    fn do_mkbool(nargs: libc::c_int) -> *mut NODE;
    fn do_lshift(nargs: libc::c_int) -> *mut NODE;
    fn do_log(nargs: libc::c_int) -> *mut NODE;
    fn do_int(nargs: libc::c_int) -> *mut NODE;
    fn do_fflush(nargs: libc::c_int) -> *mut NODE;
    fn do_exp(nargs: libc::c_int) -> *mut NODE;
    fn do_cos(nargs: libc::c_int) -> *mut NODE;
    fn do_compl(nargs: libc::c_int) -> *mut NODE;
    fn do_bindtextdomain(nargs: libc::c_int) -> *mut NODE;
    fn do_atan2(nargs: libc::c_int) -> *mut NODE;
    fn do_and(nargs: libc::c_int) -> *mut NODE;
    fn gawk_exit(status: libc::c_int);
    fn calc_exp(x1: libc::c_double, x2: libc::c_double) -> libc::c_double;
    static mut func_table: *mut NODE;
    static mut symbol_table: *mut NODE;
    fn make_typed_regex(re: *const libc::c_char, len: size_t) -> *mut NODE;
    fn make_str_node(
        s: *const libc::c_char,
        len: size_t,
        flags: libc::c_int,
    ) -> *mut NODE;
    fn install_params(func: *mut NODE);
    fn make_params(pnames: *mut *mut libc::c_char, pcount: libc::c_int) -> *mut NODE;
    fn load_ext(lib_name: *const libc::c_char);
    fn files_are_same(path: *mut libc::c_char, src: *mut SRCFILE) -> libc::c_int;
    fn find_source(
        src: *const libc::c_char,
        stb: *mut stat,
        errcode: *mut libc::c_int,
        is_extlib: libc::c_int,
    ) -> *mut libc::c_char;
    fn is_valid_identifier(name: *const libc::c_char) -> bool;
    fn remove_params(func: *mut NODE);
    fn set_current_namespace(new_namespace: *const libc::c_char);
    fn do_sub(nargs: libc::c_int, flags: libc::c_uint) -> *mut NODE;
    fn error(mesg: *const libc::c_char, _: ...);
    fn flags2str(_: libc::c_int) -> *const libc::c_char;
    fn optimal_bufsize(fd: libc::c_int, sbuf: *mut stat) -> size_t;
    fn srcopen(s: *mut SRCFILE) -> libc::c_int;
    fn variable_list() -> *mut *mut NODE;
    fn print_vars(table: *mut *mut NODE, print_func: Func_print, fp_0: *mut FILE);
    fn function_list(sort: bool) -> *mut *mut NODE;
    fn foreach_func(
        table: *mut *mut NODE,
        _: Option::<
            unsafe extern "C" fn(*mut INSTRUCTION, *mut libc::c_void) -> libc::c_int,
        >,
        _: *mut libc::c_void,
    ) -> libc::c_int;
    fn pp_func(pc: *mut INSTRUCTION, _: *mut libc::c_void) -> libc::c_int;
    fn get_numbase(
        str: *const libc::c_char,
        len: size_t,
        use_locale: bool,
    ) -> libc::c_int;
    fn nondec2awknum(
        str: *mut libc::c_char,
        len: size_t,
        endptr: *mut *mut libc::c_char,
    ) -> libc::c_double;
    static mut rule_list: *mut INSTRUCTION;
    static mut max_args: libc::c_int;
    static mut args_array: *mut *mut NODE;
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
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
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
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type time_t = __time_t;
pub type wchar_t = libc::c_int;
pub type int32_t = __int32_t;
pub type mbstate_t = __mbstate_t;
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
pub type defrule = libc::c_uint;
pub const MAXRULE: defrule = 6;
pub const ENDFILE: defrule = 5;
pub const BEGINFILE: defrule = 4;
pub const END: defrule = 3;
pub const Rule: defrule = 2;
pub const BEGIN: defrule = 1;
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
pub type Func_print = Option::<
    unsafe extern "C" fn(*mut FILE, *const libc::c_char, ...) -> libc::c_int,
>;
pub type lintvals = libc::c_uint;
pub const LINT_no_effect: lintvals = 2;
pub const LINT_assign_in_cond: lintvals = 1;
pub const LINT_illegal: lintvals = 0;
pub type LINTTYPE = lintvals;
pub type redirval = libc::c_uint;
pub const redirect_twoway: redirval = 6;
pub const redirect_input: redirval = 5;
pub const redirect_pipein: redirval = 4;
pub const redirect_pipe: redirval = 3;
pub const redirect_append: redirval = 2;
pub const redirect_output: redirval = 1;
pub const redirect_none: redirval = 0;
pub type INSTRUCTION = exp_instruction;
pub type Func_ptr = Option::<unsafe extern "C" fn() -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct srcfile {
    pub next: *mut srcfile,
    pub prev: *mut srcfile,
    pub stype: srctype,
    pub src: *mut libc::c_char,
    pub fullpath: *mut libc::c_char,
    pub mtime: time_t,
    pub sbuf: stat,
    pub srclines: libc::c_int,
    pub bufsize: size_t,
    pub buf: *mut libc::c_char,
    pub line_offset: *mut libc::c_int,
    pub fd: libc::c_int,
    pub maxlen: libc::c_int,
    pub fini_func: Option::<unsafe extern "C" fn() -> ()>,
    pub lexptr: *mut libc::c_char,
    pub lexend: *mut libc::c_char,
    pub lexeme: *mut libc::c_char,
    pub lexptr_begin: *mut libc::c_char,
    pub lasttok: libc::c_int,
    pub comment: *mut INSTRUCTION,
    pub namespace: *const libc::c_char,
}
pub type srctype = libc::c_uint;
pub const SRC_EXTLIB: srctype = 5;
pub const SRC_INC: srctype = 4;
pub const SRC_FILE: srctype = 3;
pub const SRC_STDIN: srctype = 2;
pub const SRC_CMDLINE: srctype = 1;
pub type SRCFILE = srcfile;
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
pub struct fdesc {
    pub name: *mut libc::c_char,
    pub used: libc::c_short,
    pub defined: libc::c_short,
    pub extension: libc::c_short,
    pub next: *mut fdesc,
}
pub type yy_state_t = yytype_int16;
pub type yytype_int16 = libc::c_short;
pub type yysymbol_kind_t = libc::c_int;
pub const YYSYMBOL_comma: yysymbol_kind_t = 148;
pub const YYSYMBOL_colon: yysymbol_kind_t = 147;
pub const YYSYMBOL_semi: yysymbol_kind_t = 146;
pub const YYSYMBOL_opt_semi: yysymbol_kind_t = 145;
pub const YYSYMBOL_r_paren: yysymbol_kind_t = 144;
pub const YYSYMBOL_r_brace: yysymbol_kind_t = 143;
pub const YYSYMBOL_l_brace: yysymbol_kind_t = 142;
pub const YYSYMBOL_opt_incdec: yysymbol_kind_t = 141;
pub const YYSYMBOL_variable: yysymbol_kind_t = 140;
pub const YYSYMBOL_simple_variable: yysymbol_kind_t = 139;
pub const YYSYMBOL_subscript_list: yysymbol_kind_t = 138;
pub const YYSYMBOL_subscript: yysymbol_kind_t = 137;
pub const YYSYMBOL_bracketed_exp_list: yysymbol_kind_t = 136;
pub const YYSYMBOL_delete_exp_list: yysymbol_kind_t = 135;
pub const YYSYMBOL_delete_subscript: yysymbol_kind_t = 134;
pub const YYSYMBOL_delete_subscript_list: yysymbol_kind_t = 133;
pub const YYSYMBOL_opt_variable: yysymbol_kind_t = 132;
pub const YYSYMBOL_direct_func_call: yysymbol_kind_t = 131;
pub const YYSYMBOL_func_call: yysymbol_kind_t = 130;
pub const YYSYMBOL_non_post_simp_exp: yysymbol_kind_t = 129;
pub const YYSYMBOL_simp_exp_nc: yysymbol_kind_t = 128;
pub const YYSYMBOL_simp_exp: yysymbol_kind_t = 127;
pub const YYSYMBOL_common_exp: yysymbol_kind_t = 126;
pub const YYSYMBOL_a_relop: yysymbol_kind_t = 125;
pub const YYSYMBOL_relop_or_less: yysymbol_kind_t = 124;
pub const YYSYMBOL_assign_operator: yysymbol_kind_t = 123;
pub const YYSYMBOL_exp: yysymbol_kind_t = 122;
pub const YYSYMBOL_opt_fcall_exp: yysymbol_kind_t = 121;
pub const YYSYMBOL_fcall_exp: yysymbol_kind_t = 120;
pub const YYSYMBOL_fcall_expression_list: yysymbol_kind_t = 119;
pub const YYSYMBOL_opt_fcall_expression_list: yysymbol_kind_t = 118;
pub const YYSYMBOL_expression_list: yysymbol_kind_t = 117;
pub const YYSYMBOL_opt_expression_list: yysymbol_kind_t = 116;
pub const YYSYMBOL_opt_exp: yysymbol_kind_t = 115;
pub const YYSYMBOL_param_list: yysymbol_kind_t = 114;
pub const YYSYMBOL_opt_param_list: yysymbol_kind_t = 113;
pub const YYSYMBOL_input_redir: yysymbol_kind_t = 112;
pub const YYSYMBOL_opt_nls: yysymbol_kind_t = 111;
pub const YYSYMBOL_nls: yysymbol_kind_t = 110;
pub const YYSYMBOL_if_statement: yysymbol_kind_t = 109;
pub const YYSYMBOL_108_6: yysymbol_kind_t = 108;
pub const YYSYMBOL_output_redir: yysymbol_kind_t = 107;
pub const YYSYMBOL_print_expression_list: yysymbol_kind_t = 106;
pub const YYSYMBOL_print: yysymbol_kind_t = 105;
pub const YYSYMBOL_case_value: yysymbol_kind_t = 104;
pub const YYSYMBOL_case_statement: yysymbol_kind_t = 103;
pub const YYSYMBOL_case_statements: yysymbol_kind_t = 102;
pub const YYSYMBOL_opt_simple_stmt: yysymbol_kind_t = 101;
pub const YYSYMBOL_100_5: yysymbol_kind_t = 100;
pub const YYSYMBOL_99_4: yysymbol_kind_t = 99;
pub const YYSYMBOL_simple_stmt: yysymbol_kind_t = 98;
pub const YYSYMBOL_97_3: yysymbol_kind_t = 97;
pub const YYSYMBOL_non_compound_stmt: yysymbol_kind_t = 96;
pub const YYSYMBOL_statement: yysymbol_kind_t = 95;
pub const YYSYMBOL_statement_term: yysymbol_kind_t = 94;
pub const YYSYMBOL_statements: yysymbol_kind_t = 93;
pub const YYSYMBOL_a_slash: yysymbol_kind_t = 92;
pub const YYSYMBOL_typed_regexp: yysymbol_kind_t = 91;
pub const YYSYMBOL_90_2: yysymbol_kind_t = 90;
pub const YYSYMBOL_regexp: yysymbol_kind_t = 89;
pub const YYSYMBOL_88_1: yysymbol_kind_t = 88;
pub const YYSYMBOL_function_prologue: yysymbol_kind_t = 87;
pub const YYSYMBOL_lex_builtin: yysymbol_kind_t = 86;
pub const YYSYMBOL_func_name: yysymbol_kind_t = 85;
pub const YYSYMBOL_action: yysymbol_kind_t = 84;
pub const YYSYMBOL_pattern: yysymbol_kind_t = 83;
pub const YYSYMBOL_namespace: yysymbol_kind_t = 82;
pub const YYSYMBOL_library: yysymbol_kind_t = 81;
pub const YYSYMBOL_source: yysymbol_kind_t = 80;
pub const YYSYMBOL_rule: yysymbol_kind_t = 79;
pub const YYSYMBOL_program: yysymbol_kind_t = 78;
pub const YYSYMBOL_YYACCEPT: yysymbol_kind_t = 77;
pub const YYSYMBOL_76_: yysymbol_kind_t = 76;
pub const YYSYMBOL_75_: yysymbol_kind_t = 75;
pub const YYSYMBOL_74_: yysymbol_kind_t = 74;
pub const YYSYMBOL_73_: yysymbol_kind_t = 73;
pub const YYSYMBOL_72_: yysymbol_kind_t = 72;
pub const YYSYMBOL_71_: yysymbol_kind_t = 71;
pub const YYSYMBOL_70_: yysymbol_kind_t = 70;
pub const YYSYMBOL_69_: yysymbol_kind_t = 69;
pub const YYSYMBOL_68_: yysymbol_kind_t = 68;
pub const YYSYMBOL_67_: yysymbol_kind_t = 67;
pub const YYSYMBOL_UNARY: yysymbol_kind_t = 66;
pub const YYSYMBOL_65_: yysymbol_kind_t = 65;
pub const YYSYMBOL_64_: yysymbol_kind_t = 64;
pub const YYSYMBOL_63_: yysymbol_kind_t = 63;
pub const YYSYMBOL_62_: yysymbol_kind_t = 62;
pub const YYSYMBOL_61_: yysymbol_kind_t = 61;
pub const YYSYMBOL_60_: yysymbol_kind_t = 60;
pub const YYSYMBOL_59_: yysymbol_kind_t = 59;
pub const YYSYMBOL_58_: yysymbol_kind_t = 58;
pub const YYSYMBOL_57_: yysymbol_kind_t = 57;
pub const YYSYMBOL_56_: yysymbol_kind_t = 56;
pub const YYSYMBOL_55_: yysymbol_kind_t = 55;
pub const YYSYMBOL_SLASH_BEFORE_EQUAL: yysymbol_kind_t = 54;
pub const YYSYMBOL_NEWLINE: yysymbol_kind_t = 53;
pub const YYSYMBOL_LEX_NAMESPACE: yysymbol_kind_t = 52;
pub const YYSYMBOL_LEX_LOAD: yysymbol_kind_t = 51;
pub const YYSYMBOL_LEX_EVAL: yysymbol_kind_t = 50;
pub const YYSYMBOL_LEX_INCLUDE: yysymbol_kind_t = 49;
pub const YYSYMBOL_LEX_EOF: yysymbol_kind_t = 48;
pub const YYSYMBOL_LEX_LENGTH: yysymbol_kind_t = 47;
pub const YYSYMBOL_LEX_BUILTIN: yysymbol_kind_t = 46;
pub const YYSYMBOL_DECREMENT: yysymbol_kind_t = 45;
pub const YYSYMBOL_INCREMENT: yysymbol_kind_t = 44;
pub const YYSYMBOL_LEX_OR: yysymbol_kind_t = 43;
pub const YYSYMBOL_LEX_AND: yysymbol_kind_t = 42;
pub const YYSYMBOL_LEX_IN: yysymbol_kind_t = 41;
pub const YYSYMBOL_LEX_NEXTFILE: yysymbol_kind_t = 40;
pub const YYSYMBOL_LEX_GETLINE: yysymbol_kind_t = 39;
pub const YYSYMBOL_LEX_ENDFILE: yysymbol_kind_t = 38;
pub const YYSYMBOL_LEX_BEGINFILE: yysymbol_kind_t = 37;
pub const YYSYMBOL_LEX_FUNCTION: yysymbol_kind_t = 36;
pub const YYSYMBOL_LEX_EXIT: yysymbol_kind_t = 35;
pub const YYSYMBOL_LEX_NEXT: yysymbol_kind_t = 34;
pub const YYSYMBOL_LEX_PRINTF: yysymbol_kind_t = 33;
pub const YYSYMBOL_LEX_PRINT: yysymbol_kind_t = 32;
pub const YYSYMBOL_LEX_CONTINUE: yysymbol_kind_t = 31;
pub const YYSYMBOL_LEX_BREAK: yysymbol_kind_t = 30;
pub const YYSYMBOL_LEX_FOR: yysymbol_kind_t = 29;
pub const YYSYMBOL_LEX_DO: yysymbol_kind_t = 28;
pub const YYSYMBOL_LEX_WHILE: yysymbol_kind_t = 27;
pub const YYSYMBOL_LEX_DEFAULT: yysymbol_kind_t = 26;
pub const YYSYMBOL_LEX_CASE: yysymbol_kind_t = 25;
pub const YYSYMBOL_LEX_SWITCH: yysymbol_kind_t = 24;
pub const YYSYMBOL_LEX_DELETE: yysymbol_kind_t = 23;
pub const YYSYMBOL_LEX_RETURN: yysymbol_kind_t = 22;
pub const YYSYMBOL_LEX_ELSE: yysymbol_kind_t = 21;
pub const YYSYMBOL_LEX_IF: yysymbol_kind_t = 20;
pub const YYSYMBOL_LEX_END: yysymbol_kind_t = 19;
pub const YYSYMBOL_LEX_BEGIN: yysymbol_kind_t = 18;
pub const YYSYMBOL_SUBSCRIPT: yysymbol_kind_t = 17;
pub const YYSYMBOL_CONCAT_OP: yysymbol_kind_t = 16;
pub const YYSYMBOL_MATCHOP: yysymbol_kind_t = 15;
pub const YYSYMBOL_ASSIGN: yysymbol_kind_t = 14;
pub const YYSYMBOL_ASSIGNOP: yysymbol_kind_t = 13;
pub const YYSYMBOL_IO_IN: yysymbol_kind_t = 12;
pub const YYSYMBOL_IO_OUT: yysymbol_kind_t = 11;
pub const YYSYMBOL_RELOP: yysymbol_kind_t = 10;
pub const YYSYMBOL_TYPED_REGEXP: yysymbol_kind_t = 9;
pub const YYSYMBOL_YSTRING: yysymbol_kind_t = 8;
pub const YYSYMBOL_YNUMBER: yysymbol_kind_t = 7;
pub const YYSYMBOL_FILENAME: yysymbol_kind_t = 6;
pub const YYSYMBOL_REGEXP: yysymbol_kind_t = 5;
pub const YYSYMBOL_NAME: yysymbol_kind_t = 4;
pub const YYSYMBOL_FUNC_CALL: yysymbol_kind_t = 3;
pub const YYSYMBOL_YYUNDEF: yysymbol_kind_t = 2;
pub const YYSYMBOL_YYerror: yysymbol_kind_t = 1;
pub const YYSYMBOL_YYEOF: yysymbol_kind_t = 0;
pub const YYSYMBOL_YYEMPTY: yysymbol_kind_t = -2;
pub type yytype_uint8 = libc::c_uchar;
pub type yytype_int8 = libc::c_schar;
pub type yy_state_fast_t = libc::c_int;
pub type defref = libc::c_uint;
pub const FUNC_EXT: defref = 2;
pub const FUNC_USE: defref = 1;
pub const FUNC_DEFINE: defref = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct token {
    pub operator: *const libc::c_char,
    pub value: OPCODE,
    pub class: libc::c_int,
    pub flags: libc::c_uint,
    pub ptr: Option::<unsafe extern "C" fn(libc::c_int) -> *mut NODE>,
    pub ptr2: Option::<unsafe extern "C" fn(libc::c_int) -> *mut NODE>,
}
pub type C2RustUnnamed_12 = libc::c_uint;
pub const DONT_CHECK: C2RustUnnamed_12 = 2;
pub const FUNC_BODY: C2RustUnnamed_12 = 1;
pub const FUNC_HEADER: C2RustUnnamed_12 = 0;
pub type defline = libc::c_uint;
pub const LAST_LINE: defline = 1;
pub const FIRST_LINE: defline = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union yyalloc {
    pub yyss_alloc: yy_state_t,
    pub yyvs_alloc: *mut INSTRUCTION,
}
pub type builtin_func_t = Option::<unsafe extern "C" fn(libc::c_int) -> *mut NODE>;
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
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
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
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
unsafe extern "C" fn ezalloc_real(
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
            2070 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            b"%s:%d: ezalloc called with zero bytes\0" as *const u8
                as *const libc::c_char,
            file,
            line,
        );
    }
    ret = calloc(1 as libc::c_int as libc::c_ulong, count);
    if ret.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./awk.h\0" as *const u8 as *const libc::c_char,
            2074 as libc::c_int,
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
static mut trailing_comment: *mut INSTRUCTION = 0 as *const INSTRUCTION
    as *mut INSTRUCTION;
static mut outer_comment: *mut INSTRUCTION = 0 as *const INSTRUCTION as *mut INSTRUCTION;
static mut interblock_comment: *mut INSTRUCTION = 0 as *const INSTRUCTION
    as *mut INSTRUCTION;
static mut pending_comment: *mut INSTRUCTION = 0 as *const INSTRUCTION
    as *mut INSTRUCTION;
static mut namespace_chain: *mut INSTRUCTION = 0 as *const INSTRUCTION
    as *mut INSTRUCTION;
static mut at_seen: libc::c_int = 0 as libc::c_int;
static mut want_source: bool = 0 as libc::c_int != 0;
static mut want_namespace: bool = 0 as libc::c_int != 0;
static mut want_regexp: bool = 0 as libc::c_int != 0;
static mut want_param_names: C2RustUnnamed_12 = DONT_CHECK;
static mut in_function: bool = false;
static mut rule: libc::c_int = 0 as libc::c_int;
pub static mut ruletab: [*const libc::c_char; 6] = [
    b"?\0" as *const u8 as *const libc::c_char,
    b"BEGIN\0" as *const u8 as *const libc::c_char,
    b"Rule\0" as *const u8 as *const libc::c_char,
    b"END\0" as *const u8 as *const libc::c_char,
    b"BEGINFILE\0" as *const u8 as *const libc::c_char,
    b"ENDFILE\0" as *const u8 as *const libc::c_char,
];
static mut in_print: bool = 0 as libc::c_int != 0;
static mut in_parens: libc::c_int = 0 as libc::c_int;
static mut sub_counter: libc::c_int = 0 as libc::c_int;
static mut lexptr: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut lexend: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut lexptr_begin: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut lexeme: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut lexeof: bool = false;
static mut thisline: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut in_braces: libc::c_int = 0 as libc::c_int;
static mut lastline: libc::c_int = 0 as libc::c_int;
static mut firstline: libc::c_int = 0 as libc::c_int;
static mut sourcefile: *mut SRCFILE = 0 as *const SRCFILE as *mut SRCFILE;
static mut lasttok: libc::c_int = 0 as libc::c_int;
static mut eof_warned: bool = 0 as libc::c_int != 0;
static mut break_allowed: libc::c_int = 0;
static mut continue_allowed: libc::c_int = 0;
static mut tokstart: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut tok: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut tokend: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut errcount: libc::c_int = 0 as libc::c_int;
pub static mut awk_namespace: [libc::c_char; 4] = unsafe {
    *::std::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"awk\0")
};
pub static mut current_namespace: *const libc::c_char = unsafe {
    awk_namespace.as_ptr()
};
pub static mut namespace_changed: bool = 0 as libc::c_int != 0;
static mut rule_block: [*mut INSTRUCTION; 6] = [0 as *const INSTRUCTION
    as *mut INSTRUCTION; 6];
static mut ip_rec: *mut INSTRUCTION = 0 as *const INSTRUCTION as *mut INSTRUCTION;
static mut ip_newfile: *mut INSTRUCTION = 0 as *const INSTRUCTION as *mut INSTRUCTION;
static mut ip_atexit: *mut INSTRUCTION = 0 as *const INSTRUCTION as *mut INSTRUCTION;
static mut ip_end: *mut INSTRUCTION = 0 as *const INSTRUCTION as *mut INSTRUCTION;
static mut ip_endfile: *mut INSTRUCTION = 0 as *const INSTRUCTION as *mut INSTRUCTION;
static mut ip_beginfile: *mut INSTRUCTION = 0 as *const INSTRUCTION as *mut INSTRUCTION;
pub static mut main_beginfile: *mut INSTRUCTION = 0 as *const INSTRUCTION
    as *mut INSTRUCTION;
static mut called_from_eval: bool = 0 as libc::c_int != 0;
static mut yytranslate: [yytype_int8; 311] = [
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    65 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    68 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    70 as libc::c_int as yytype_int8,
    62 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    57 as libc::c_int as yytype_int8,
    61 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    63 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    76 as libc::c_int as yytype_int8,
    58 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    59 as libc::c_int as yytype_int8,
    55 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    72 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    73 as libc::c_int as yytype_int8,
    67 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    74 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    75 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    48 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
];
static mut yypact: [yytype_int16; 356] = [
    -(276 as libc::c_int) as yytype_int16,
    315 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(45 as libc::c_int) as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    133 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    13 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    -(13 as libc::c_int) as yytype_int16,
    -(7 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    1074 as libc::c_int as yytype_int16,
    817 as libc::c_int as yytype_int16,
    172 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(23 as libc::c_int) as yytype_int16,
    -(10 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    29 as libc::c_int as yytype_int16,
    1112 as libc::c_int as yytype_int16,
    973 as libc::c_int as yytype_int16,
    276 as libc::c_int as yytype_int16,
    310 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    137 as libc::c_int as yytype_int16,
    741 as libc::c_int as yytype_int16,
    817 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    1 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    64 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    98 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    741 as libc::c_int as yytype_int16,
    741 as libc::c_int as yytype_int16,
    173 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    125 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    15 as libc::c_int as yytype_int16,
    1016 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    65 as libc::c_int as yytype_int16,
    151 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    141 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    29 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    141 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    198 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    1001 as libc::c_int as yytype_int16,
    203 as libc::c_int as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    141 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    175 as libc::c_int as yytype_int16,
    276 as libc::c_int as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    209 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    208 as libc::c_int as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    142 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    1141 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    1141 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    142 as libc::c_int as yytype_int16,
    142 as libc::c_int as yytype_int16,
    1016 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    186 as libc::c_int as yytype_int16,
    845 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    8 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    34 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    59 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    541 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    11 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    287 as libc::c_int as yytype_int16,
    130 as libc::c_int as yytype_int16,
    1131 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    139 as libc::c_int as yytype_int16,
    1141 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    123 as libc::c_int as yytype_int16,
    123 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    123 as libc::c_int as yytype_int16,
    123 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    1141 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    1001 as libc::c_int as yytype_int16,
    769 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    45 as libc::c_int as yytype_int16,
    276 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    1141 as libc::c_int as yytype_int16,
    203 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    1141 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    174 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    10 as libc::c_int as yytype_int16,
    179 as libc::c_int as yytype_int16,
    183 as libc::c_int as yytype_int16,
    141 as libc::c_int as yytype_int16,
    185 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    90 as libc::c_int as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    141 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    90 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    1141 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    182 as libc::c_int as yytype_int16,
    141 as libc::c_int as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    142 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    1001 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    251 as libc::c_int as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    659 as libc::c_int as yytype_int16,
    894 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    90 as libc::c_int as yytype_int16,
    1141 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    589 as libc::c_int as yytype_int16,
    541 as libc::c_int as yytype_int16,
    141 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    1141 as libc::c_int as yytype_int16,
    141 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    46 as libc::c_int as yytype_int16,
    1016 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    90 as libc::c_int as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    189 as libc::c_int as yytype_int16,
    1016 as libc::c_int as yytype_int16,
    1016 as libc::c_int as yytype_int16,
    243 as libc::c_int as yytype_int16,
    -(15 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    182 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    817 as libc::c_int as yytype_int16,
    263 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    162 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    141 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    60 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    141 as libc::c_int as yytype_int16,
    141 as libc::c_int as yytype_int16,
    211 as libc::c_int as yytype_int16,
    203 as libc::c_int as yytype_int16,
    141 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    659 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(10 as libc::c_int) as yytype_int16,
    659 as libc::c_int as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    142 as libc::c_int as yytype_int16,
    693 as libc::c_int as yytype_int16,
    186 as libc::c_int as yytype_int16,
    1046 as libc::c_int as yytype_int16,
    256 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    1016 as libc::c_int as yytype_int16,
    141 as libc::c_int as yytype_int16,
    1089 as libc::c_int as yytype_int16,
    141 as libc::c_int as yytype_int16,
    973 as libc::c_int as yytype_int16,
    141 as libc::c_int as yytype_int16,
    260 as libc::c_int as yytype_int16,
    141 as libc::c_int as yytype_int16,
    659 as libc::c_int as yytype_int16,
    141 as libc::c_int as yytype_int16,
    928 as libc::c_int as yytype_int16,
    659 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    340 as libc::c_int as yytype_int16,
    222 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    212 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    928 as libc::c_int as yytype_int16,
    142 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    281 as libc::c_int as yytype_int16,
    282 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    222 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    141 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    142 as libc::c_int as yytype_int16,
    141 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    141 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    141 as libc::c_int as yytype_int16,
    659 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    389 as libc::c_int as yytype_int16,
    659 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    465 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
];
static mut yydefact: [yytype_uint8; 356] = [
    2 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    195 as libc::c_int as yytype_uint8,
    177 as libc::c_int as yytype_uint8,
    178 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    184 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    172 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    94 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    167 as libc::c_int as yytype_uint8,
    38 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    23 as libc::c_int as yytype_uint8,
    138 as libc::c_int as yytype_uint8,
    146 as libc::c_int as yytype_uint8,
    147 as libc::c_int as yytype_uint8,
    149 as libc::c_int as yytype_uint8,
    173 as libc::c_int as yytype_uint8,
    181 as libc::c_int as yytype_uint8,
    197 as libc::c_int as yytype_uint8,
    174 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    192 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    196 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    34 as libc::c_int as yytype_uint8,
    35 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    185 as libc::c_int as yytype_uint8,
    175 as libc::c_int as yytype_uint8,
    176 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    180 as libc::c_int as yytype_uint8,
    174 as libc::c_int as yytype_uint8,
    179 as libc::c_int as yytype_uint8,
    168 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    201 as libc::c_int as yytype_uint8,
    174 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    182 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    207 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    9 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    95 as libc::c_int as yytype_uint8,
    142 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    143 as libc::c_int as yytype_uint8,
    145 as libc::c_int as yytype_uint8,
    144 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    148 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    140 as libc::c_int as yytype_uint8,
    139 as libc::c_int as yytype_uint8,
    157 as libc::c_int as yytype_uint8,
    158 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    121 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
    126 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    119 as libc::c_int as yytype_uint8,
    125 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    194 as libc::c_int as yytype_uint8,
    193 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    156 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    199 as libc::c_int as yytype_uint8,
    200 as libc::c_int as yytype_uint8,
    198 as libc::c_int as yytype_uint8,
    114 as libc::c_int as yytype_uint8,
    204 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    169 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    97 as libc::c_int as yytype_uint8,
    202 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    39 as libc::c_int as yytype_uint8,
    133 as libc::c_int as yytype_uint8,
    134 as libc::c_int as yytype_uint8,
    135 as libc::c_int as yytype_uint8,
    131 as libc::c_int as yytype_uint8,
    132 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    209 as libc::c_int as yytype_uint8,
    136 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    184 as libc::c_int as yytype_uint8,
    154 as libc::c_int as yytype_uint8,
    155 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    152 as libc::c_int as yytype_uint8,
    153 as libc::c_int as yytype_uint8,
    150 as libc::c_int as yytype_uint8,
    165 as libc::c_int as yytype_uint8,
    166 as libc::c_int as yytype_uint8,
    162 as libc::c_int as yytype_uint8,
    163 as libc::c_int as yytype_uint8,
    164 as libc::c_int as yytype_uint8,
    161 as libc::c_int as yytype_uint8,
    130 as libc::c_int as yytype_uint8,
    141 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    183 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    191 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    99 as libc::c_int as yytype_uint8,
    170 as libc::c_int as yytype_uint8,
    171 as libc::c_int as yytype_uint8,
    115 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    116 as libc::c_int as yytype_uint8,
    112 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    85 as libc::c_int as yytype_uint8,
    86 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    107 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    66 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    71 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    205 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    160 as libc::c_int as yytype_uint8,
    123 as libc::c_int as yytype_uint8,
    124 as libc::c_int as yytype_uint8,
    120 as libc::c_int as yytype_uint8,
    104 as libc::c_int as yytype_uint8,
    102 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    159 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    72 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    108 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    203 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    206 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    137 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    105 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    128 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    186 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    195 as libc::c_int as yytype_uint8,
    73 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    62 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    89 as libc::c_int as yytype_uint8,
    87 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    37 as libc::c_int as yytype_uint8,
    106 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    188 as libc::c_int as yytype_uint8,
    190 as libc::c_int as yytype_uint8,
    70 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    90 as libc::c_int as yytype_uint8,
    67 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    187 as libc::c_int as yytype_uint8,
    189 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    88 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    92 as libc::c_int as yytype_uint8,
    74 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    91 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    72 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    76 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    75 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    72 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    93 as libc::c_int as yytype_uint8,
    79 as libc::c_int as yytype_uint8,
    82 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    84 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    208 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    81 as libc::c_int as yytype_uint8,
    80 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
];
static mut yypgoto: [yytype_int16; 72] = [
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    253 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(32 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(77 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(208 as libc::c_int) as yytype_int16,
    -(55 as libc::c_int) as yytype_int16,
    -(68 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(237 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(275 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(48 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(43 as libc::c_int) as yytype_int16,
    157 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(157 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    0 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    268 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    2 as libc::c_int as yytype_int16,
    138 as libc::c_int as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    22 as libc::c_int as yytype_int16,
    -(38 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(81 as libc::c_int) as yytype_int16,
    -(2 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(27 as libc::c_int) as yytype_int16,
    -(213 as libc::c_int) as yytype_int16,
    -(66 as libc::c_int) as yytype_int16,
    -(276 as libc::c_int) as yytype_int16,
    -(22 as libc::c_int) as yytype_int16,
    -(30 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
];
static mut yydefgoto: [yytype_int16; 72] = [
    0 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    146 as libc::c_int as yytype_int16,
    149 as libc::c_int as yytype_int16,
    152 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    187 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    120 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    155 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    217 as libc::c_int as yytype_int16,
    218 as libc::c_int as yytype_int16,
    237 as libc::c_int as yytype_int16,
    219 as libc::c_int as yytype_int16,
    252 as libc::c_int as yytype_int16,
    264 as libc::c_int as yytype_int16,
    271 as libc::c_int as yytype_int16,
    316 as libc::c_int as yytype_int16,
    325 as libc::c_int as yytype_int16,
    338 as libc::c_int as yytype_int16,
    220 as libc::c_int as yytype_int16,
    274 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    306 as libc::c_int as yytype_int16,
    221 as libc::c_int as yytype_int16,
    153 as libc::c_int as yytype_int16,
    154 as libc::c_int as yytype_int16,
    132 as libc::c_int as yytype_int16,
    233 as libc::c_int as yytype_int16,
    234 as libc::c_int as yytype_int16,
    247 as libc::c_int as yytype_int16,
    275 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    121 as libc::c_int as yytype_int16,
    122 as libc::c_int as yytype_int16,
    123 as libc::c_int as yytype_int16,
    263 as libc::c_int as yytype_int16,
    222 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    95 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    284 as libc::c_int as yytype_int16,
    285 as libc::c_int as yytype_int16,
    286 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    138 as libc::c_int as yytype_int16,
    223 as libc::c_int as yytype_int16,
    224 as libc::c_int as yytype_int16,
    143 as libc::c_int as yytype_int16,
    254 as libc::c_int as yytype_int16,
    82 as libc::c_int as yytype_int16,
    340 as libc::c_int as yytype_int16,
    142 as libc::c_int as yytype_int16,
];
static mut yytable: [yytype_int16; 1201] = [
    34 as libc::c_int as yytype_int16,
    125 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    141 as libc::c_int as yytype_int16,
    97 as libc::c_int as yytype_int16,
    270 as libc::c_int as yytype_int16,
    160 as libc::c_int as yytype_int16,
    128 as libc::c_int as yytype_int16,
    195 as libc::c_int as yytype_int16,
    158 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    238 as libc::c_int as yytype_int16,
    253 as libc::c_int as yytype_int16,
    139 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    127 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    292 as libc::c_int as yytype_int16,
    228 as libc::c_int as yytype_int16,
    230 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    144 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    156 as libc::c_int as yytype_int16,
    197 as libc::c_int as yytype_int16,
    180 as libc::c_int as yytype_int16,
    145 as libc::c_int as yytype_int16,
    62 as libc::c_int as yytype_int16,
    64 as libc::c_int as yytype_int16,
    277 as libc::c_int as yytype_int16,
    65 as libc::c_int as yytype_int16,
    124 as libc::c_int as yytype_int16,
    126 as libc::c_int as yytype_int16,
    164 as libc::c_int as yytype_int16,
    330 as libc::c_int as yytype_int16,
    231 as libc::c_int as yytype_int16,
    280 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    232 as libc::c_int as yytype_int16,
    281 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    342 as libc::c_int as yytype_int16,
    183 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    124 as libc::c_int as yytype_int16,
    124 as libc::c_int as yytype_int16,
    199 as libc::c_int as yytype_int16,
    -(13 as libc::c_int) as yytype_int16,
    60 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    135 as libc::c_int as yytype_int16,
    147 as libc::c_int as yytype_int16,
    189 as libc::c_int as yytype_int16,
    190 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    94 as libc::c_int as yytype_int16,
    148 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    139 as libc::c_int as yytype_int16,
    184 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    298 as libc::c_int as yytype_int16,
    259 as libc::c_int as yytype_int16,
    239 as libc::c_int as yytype_int16,
    262 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    270 as libc::c_int as yytype_int16,
    -(13 as libc::c_int) as yytype_int16,
    140 as libc::c_int as yytype_int16,
    159 as libc::c_int as yytype_int16,
    -(16 as libc::c_int) as yytype_int16,
    161 as libc::c_int as yytype_int16,
    162 as libc::c_int as yytype_int16,
    163 as libc::c_int as yytype_int16,
    196 as libc::c_int as yytype_int16,
    270 as libc::c_int as yytype_int16,
    185 as libc::c_int as yytype_int16,
    198 as libc::c_int as yytype_int16,
    165 as libc::c_int as yytype_int16,
    166 as libc::c_int as yytype_int16,
    200 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    -(16 as libc::c_int) as yytype_int16,
    235 as libc::c_int as yytype_int16,
    -(19 as libc::c_int) as yytype_int16,
    341 as libc::c_int as yytype_int16,
    129 as libc::c_int as yytype_int16,
    -(100 as libc::c_int) as yytype_int16,
    182 as libc::c_int as yytype_int16,
    168 as libc::c_int as yytype_int16,
    169 as libc::c_int as yytype_int16,
    170 as libc::c_int as yytype_int16,
    171 as libc::c_int as yytype_int16,
    172 as libc::c_int as yytype_int16,
    173 as libc::c_int as yytype_int16,
    174 as libc::c_int as yytype_int16,
    175 as libc::c_int as yytype_int16,
    176 as libc::c_int as yytype_int16,
    177 as libc::c_int as yytype_int16,
    178 as libc::c_int as yytype_int16,
    179 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    225 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    -(19 as libc::c_int) as yytype_int16,
    48 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    191 as libc::c_int as yytype_int16,
    351 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    194 as libc::c_int as yytype_int16,
    354 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    -(118 as libc::c_int) as yytype_int16,
    87 as libc::c_int as yytype_int16,
    186 as libc::c_int as yytype_int16,
    -(101 as libc::c_int) as yytype_int16,
    188 as libc::c_int as yytype_int16,
    -(119 as libc::c_int) as yytype_int16,
    112 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    150 as libc::c_int as yytype_int16,
    130 as libc::c_int as yytype_int16,
    244 as libc::c_int as yytype_int16,
    245 as libc::c_int as yytype_int16,
    131 as libc::c_int as yytype_int16,
    151 as libc::c_int as yytype_int16,
    246 as libc::c_int as yytype_int16,
    242 as libc::c_int as yytype_int16,
    249 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    139 as libc::c_int as yytype_int16,
    251 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    258 as libc::c_int as yytype_int16,
    250 as libc::c_int as yytype_int16,
    136 as libc::c_int as yytype_int16,
    137 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    -(110 as libc::c_int) as yytype_int16,
    268 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    256 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    124 as libc::c_int as yytype_int16,
    124 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    94 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    272 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    282 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    -(119 as libc::c_int) as yytype_int16,
    -(119 as libc::c_int) as yytype_int16,
    80 as libc::c_int as yytype_int16,
    289 as libc::c_int as yytype_int16,
    290 as libc::c_int as yytype_int16,
    255 as libc::c_int as yytype_int16,
    157 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    260 as libc::c_int as yytype_int16,
    278 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    283 as libc::c_int as yytype_int16,
    276 as libc::c_int as yytype_int16,
    279 as libc::c_int as yytype_int16,
    303 as libc::c_int as yytype_int16,
    140 as libc::c_int as yytype_int16,
    248 as libc::c_int as yytype_int16,
    167 as libc::c_int as yytype_int16,
    -(110 as libc::c_int) as yytype_int16,
    133 as libc::c_int as yytype_int16,
    134 as libc::c_int as yytype_int16,
    119 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    225 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    181 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    257 as libc::c_int as yytype_int16,
    287 as libc::c_int as yytype_int16,
    192 as libc::c_int as yytype_int16,
    305 as libc::c_int as yytype_int16,
    307 as libc::c_int as yytype_int16,
    294 as libc::c_int as yytype_int16,
    225 as libc::c_int as yytype_int16,
    -(110 as libc::c_int) as yytype_int16,
    309 as libc::c_int as yytype_int16,
    297 as libc::c_int as yytype_int16,
    261 as libc::c_int as yytype_int16,
    124 as libc::c_int as yytype_int16,
    311 as libc::c_int as yytype_int16,
    -(110 as libc::c_int) as yytype_int16,
    266 as libc::c_int as yytype_int16,
    267 as libc::c_int as yytype_int16,
    300 as libc::c_int as yytype_int16,
    301 as libc::c_int as yytype_int16,
    236 as libc::c_int as yytype_int16,
    317 as libc::c_int as yytype_int16,
    304 as libc::c_int as yytype_int16,
    337 as libc::c_int as yytype_int16,
    287 as libc::c_int as yytype_int16,
    240 as libc::c_int as yytype_int16,
    293 as libc::c_int as yytype_int16,
    328 as libc::c_int as yytype_int16,
    126 as libc::c_int as yytype_int16,
    241 as libc::c_int as yytype_int16,
    331 as libc::c_int as yytype_int16,
    243 as libc::c_int as yytype_int16,
    265 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    288 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    322 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    318 as libc::c_int as yytype_int16,
    343 as libc::c_int as yytype_int16,
    320 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    321 as libc::c_int as yytype_int16,
    326 as libc::c_int as yytype_int16,
    327 as libc::c_int as yytype_int16,
    291 as libc::c_int as yytype_int16,
    329 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    308 as libc::c_int as yytype_int16,
    295 as libc::c_int as yytype_int16,
    225 as libc::c_int as yytype_int16,
    348 as libc::c_int as yytype_int16,
    315 as libc::c_int as yytype_int16,
    339 as libc::c_int as yytype_int16,
    225 as libc::c_int as yytype_int16,
    302 as libc::c_int as yytype_int16,
    353 as libc::c_int as yytype_int16,
    313 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    355 as libc::c_int as yytype_int16,
    323 as libc::c_int as yytype_int16,
    324 as libc::c_int as yytype_int16,
    216 as libc::c_int as yytype_int16,
    344 as libc::c_int as yytype_int16,
    345 as libc::c_int as yytype_int16,
    319 as libc::c_int as yytype_int16,
    336 as libc::c_int as yytype_int16,
    347 as libc::c_int as yytype_int16,
    67 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    349 as libc::c_int as yytype_int16,
    225 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    350 as libc::c_int as yytype_int16,
    225 as libc::c_int as yytype_int16,
    352 as libc::c_int as yytype_int16,
    310 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    312 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    227 as libc::c_int as yytype_int16,
    314 as libc::c_int as yytype_int16,
    299 as libc::c_int as yytype_int16,
    346 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    2 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    225 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    225 as libc::c_int as yytype_int16,
    225 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    225 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    -(96 as libc::c_int) as yytype_int16,
    100 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    94 as libc::c_int as yytype_int16,
    332 as libc::c_int as yytype_int16,
    333 as libc::c_int as yytype_int16,
    119 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    107 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    109 as libc::c_int as yytype_int16,
    110 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    -(22 as libc::c_int) as yytype_int16,
    201 as libc::c_int as yytype_int16,
    -(22 as libc::c_int) as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    334 as libc::c_int as yytype_int16,
    335 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    202 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    203 as libc::c_int as yytype_int16,
    204 as libc::c_int as yytype_int16,
    205 as libc::c_int as yytype_int16,
    -(78 as libc::c_int) as yytype_int16,
    -(78 as libc::c_int) as yytype_int16,
    206 as libc::c_int as yytype_int16,
    207 as libc::c_int as yytype_int16,
    208 as libc::c_int as yytype_int16,
    209 as libc::c_int as yytype_int16,
    210 as libc::c_int as yytype_int16,
    211 as libc::c_int as yytype_int16,
    212 as libc::c_int as yytype_int16,
    213 as libc::c_int as yytype_int16,
    214 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    215 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    -(78 as libc::c_int) as yytype_int16,
    20 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    -(78 as libc::c_int) as yytype_int16,
    77 as libc::c_int as yytype_int16,
    201 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    202 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    203 as libc::c_int as yytype_int16,
    204 as libc::c_int as yytype_int16,
    205 as libc::c_int as yytype_int16,
    -(77 as libc::c_int) as yytype_int16,
    -(77 as libc::c_int) as yytype_int16,
    206 as libc::c_int as yytype_int16,
    207 as libc::c_int as yytype_int16,
    208 as libc::c_int as yytype_int16,
    209 as libc::c_int as yytype_int16,
    210 as libc::c_int as yytype_int16,
    211 as libc::c_int as yytype_int16,
    212 as libc::c_int as yytype_int16,
    213 as libc::c_int as yytype_int16,
    214 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    215 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    -(77 as libc::c_int) as yytype_int16,
    20 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    -(77 as libc::c_int) as yytype_int16,
    77 as libc::c_int as yytype_int16,
    201 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    202 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    203 as libc::c_int as yytype_int16,
    204 as libc::c_int as yytype_int16,
    205 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    206 as libc::c_int as yytype_int16,
    207 as libc::c_int as yytype_int16,
    208 as libc::c_int as yytype_int16,
    209 as libc::c_int as yytype_int16,
    210 as libc::c_int as yytype_int16,
    211 as libc::c_int as yytype_int16,
    212 as libc::c_int as yytype_int16,
    213 as libc::c_int as yytype_int16,
    214 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    215 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    -(109 as libc::c_int) as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    216 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    -(109 as libc::c_int) as yytype_int16,
    20 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    273 as libc::c_int as yytype_int16,
    -(109 as libc::c_int) as yytype_int16,
    61 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    -(109 as libc::c_int) as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    202 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    203 as libc::c_int as yytype_int16,
    204 as libc::c_int as yytype_int16,
    205 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    206 as libc::c_int as yytype_int16,
    207 as libc::c_int as yytype_int16,
    208 as libc::c_int as yytype_int16,
    209 as libc::c_int as yytype_int16,
    210 as libc::c_int as yytype_int16,
    211 as libc::c_int as yytype_int16,
    212 as libc::c_int as yytype_int16,
    213 as libc::c_int as yytype_int16,
    214 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    215 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    119 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    229 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    119 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    -(117 as libc::c_int) as yytype_int16,
    61 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    193 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    269 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    204 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    211 as libc::c_int as yytype_int16,
    212 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    204 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    211 as libc::c_int as yytype_int16,
    212 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    119 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    91 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    94 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    140 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    66 as libc::c_int as yytype_int16,
    91 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    94 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    91 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    94 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    91 as libc::c_int as yytype_int16,
    226 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    94 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    91 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    94 as libc::c_int as yytype_int16,
];
static mut yycheck: [yytype_int16; 1201] = [
    1 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    243 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    223 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    184 as libc::c_int as yytype_int16,
    185 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    82 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    253 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    320 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    329 as libc::c_int as yytype_int16,
    121 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    60 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    66 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    133 as libc::c_int as yytype_int16,
    134 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    237 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    320 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    91 as libc::c_int as yytype_int16,
    146 as libc::c_int as yytype_int16,
    329 as libc::c_int as yytype_int16,
    122 as libc::c_int as yytype_int16,
    149 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    97 as libc::c_int as yytype_int16,
    152 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    107 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    109 as libc::c_int as yytype_int16,
    110 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    192 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    326 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    107 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    109 as libc::c_int as yytype_int16,
    110 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    131 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    155 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    139 as libc::c_int as yytype_int16,
    347 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    142 as libc::c_int as yytype_int16,
    350 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    131 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    209 as libc::c_int as yytype_int16,
    210 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    213 as libc::c_int as yytype_int16,
    207 as libc::c_int as yytype_int16,
    215 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    219 as libc::c_int as yytype_int16,
    167 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    233 as libc::c_int as yytype_int16,
    216 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    242 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    225 as libc::c_int as yytype_int16,
    67 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    184 as libc::c_int as yytype_int16,
    185 as libc::c_int as yytype_int16,
    62 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    64 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    67 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    247 as libc::c_int as yytype_int16,
    146 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    261 as libc::c_int as yytype_int16,
    149 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    152 as libc::c_int as yytype_int16,
    266 as libc::c_int as yytype_int16,
    267 as libc::c_int as yytype_int16,
    224 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    234 as libc::c_int as yytype_int16,
    254 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    263 as libc::c_int as yytype_int16,
    252 as libc::c_int as yytype_int16,
    258 as libc::c_int as yytype_int16,
    292 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    214 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    60 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    242 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    226 as libc::c_int as yytype_int16,
    264 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    294 as libc::c_int as yytype_int16,
    297 as libc::c_int as yytype_int16,
    273 as libc::c_int as yytype_int16,
    253 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    301 as libc::c_int as yytype_int16,
    282 as libc::c_int as yytype_int16,
    236 as libc::c_int as yytype_int16,
    237 as libc::c_int as yytype_int16,
    303 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    240 as libc::c_int as yytype_int16,
    241 as libc::c_int as yytype_int16,
    289 as libc::c_int as yytype_int16,
    290 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    310 as libc::c_int as yytype_int16,
    293 as libc::c_int as yytype_int16,
    323 as libc::c_int as yytype_int16,
    285 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    271 as libc::c_int as yytype_int16,
    318 as libc::c_int as yytype_int16,
    252 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    321 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    209 as libc::c_int as yytype_int16,
    210 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    213 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    215 as libc::c_int as yytype_int16,
    311 as libc::c_int as yytype_int16,
    330 as libc::c_int as yytype_int16,
    313 as libc::c_int as yytype_int16,
    219 as libc::c_int as yytype_int16,
    315 as libc::c_int as yytype_int16,
    316 as libc::c_int as yytype_int16,
    317 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    319 as libc::c_int as yytype_int16,
    273 as libc::c_int as yytype_int16,
    300 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    297 as libc::c_int as yytype_int16,
    342 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    301 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    349 as libc::c_int as yytype_int16,
    304 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    352 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    312 as libc::c_int as yytype_int16,
    323 as libc::c_int as yytype_int16,
    340 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    247 as libc::c_int as yytype_int16,
    343 as libc::c_int as yytype_int16,
    318 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    346 as libc::c_int as yytype_int16,
    321 as libc::c_int as yytype_int16,
    348 as libc::c_int as yytype_int16,
    302 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    304 as libc::c_int as yytype_int16,
    306 as libc::c_int as yytype_int16,
    167 as libc::c_int as yytype_int16,
    306 as libc::c_int as yytype_int16,
    285 as libc::c_int as yytype_int16,
    338 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    263 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    314 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    0 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    349 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    351 as libc::c_int as yytype_int16,
    352 as libc::c_int as yytype_int16,
    314 as libc::c_int as yytype_int16,
    354 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    60 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    62 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    64 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    67 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    58 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    53 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    60 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    62 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    64 as libc::c_int as yytype_int16,
    60 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    67 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    65 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    68 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    71 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    74 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    60 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    63 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    20 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    53 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    60 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    63 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    65 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    68 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    71 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    20 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    53 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    60 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    63 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    65 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    68 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    71 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    20 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    1 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    54 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    11 as libc::c_int as yytype_int16,
    60 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    63 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    65 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    68 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    71 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    39 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    53 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    60 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    63 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    65 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    68 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    76 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    20 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    54 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    60 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    63 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    65 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    68 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    71 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    39 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    76 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    1 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    54 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    60 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    63 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    65 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    68 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    71 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    76 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    39 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    54 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    60 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    63 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    65 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    39 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    1 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    54 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    60 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    63 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    65 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    68 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    71 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    1 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    39 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    54 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    60 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    63 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    65 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    39 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    71 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    60 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    63 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    65 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    68 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    71 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    54 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    23 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    60 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    63 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    65 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    71 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    39 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    12 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    60 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    63 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    65 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    68 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    71 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    39 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    10 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    15 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    60 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    63 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    65 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    39 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    71 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    60 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    63 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    65 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    68 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    58 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    39 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    10 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    15 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    60 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    63 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    65 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    68 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    71 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    15 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    60 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    63 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    65 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    10 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    10 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    76 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    55 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    57 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    55 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    58 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    55 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    58 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
];
static mut yystos: [yytype_uint8; 356] = [
    0 as libc::c_int as yytype_uint8,
    78 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    37 as libc::c_int as yytype_uint8,
    38 as libc::c_int as yytype_uint8,
    39 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    71 as libc::c_int as yytype_uint8,
    79 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    87 as libc::c_int as yytype_uint8,
    89 as libc::c_int as yytype_uint8,
    92 as libc::c_int as yytype_uint8,
    110 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    126 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    128 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    130 as libc::c_int as yytype_uint8,
    131 as libc::c_int as yytype_uint8,
    139 as libc::c_int as yytype_uint8,
    140 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    72 as libc::c_int as yytype_uint8,
    136 as libc::c_int as yytype_uint8,
    137 as libc::c_int as yytype_uint8,
    138 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    71 as libc::c_int as yytype_uint8,
    85 as libc::c_int as yytype_uint8,
    86 as libc::c_int as yytype_uint8,
    132 as libc::c_int as yytype_uint8,
    140 as libc::c_int as yytype_uint8,
    140 as libc::c_int as yytype_uint8,
    140 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    71 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    140 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    140 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    117 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    131 as libc::c_int as yytype_uint8,
    74 as libc::c_int as yytype_uint8,
    76 as libc::c_int as yytype_uint8,
    84 as libc::c_int as yytype_uint8,
    94 as libc::c_int as yytype_uint8,
    110 as libc::c_int as yytype_uint8,
    142 as libc::c_int as yytype_uint8,
    146 as libc::c_int as yytype_uint8,
    84 as libc::c_int as yytype_uint8,
    90 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    124 as libc::c_int as yytype_uint8,
    125 as libc::c_int as yytype_uint8,
    148 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    62 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    67 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    62 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    67 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    123 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    9 as libc::c_int as yytype_uint8,
    91 as libc::c_int as yytype_uint8,
    118 as libc::c_int as yytype_uint8,
    119 as libc::c_int as yytype_uint8,
    120 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    117 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    136 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    112 as libc::c_int as yytype_uint8,
    118 as libc::c_int as yytype_uint8,
    118 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    141 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    70 as libc::c_int as yytype_uint8,
    144 as libc::c_int as yytype_uint8,
    148 as libc::c_int as yytype_uint8,
    144 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    80 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    81 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    82 as libc::c_int as yytype_uint8,
    110 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    93 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    91 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    139 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    39 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    91 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    144 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    148 as libc::c_int as yytype_uint8,
    73 as libc::c_int as yytype_uint8,
    88 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    144 as libc::c_int as yytype_uint8,
    144 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    94 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    94 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    94 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    22 as libc::c_int as yytype_uint8,
    23 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    34 as libc::c_int as yytype_uint8,
    35 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
    75 as libc::c_int as yytype_uint8,
    95 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    105 as libc::c_int as yytype_uint8,
    109 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    142 as libc::c_int as yytype_uint8,
    143 as libc::c_int as yytype_uint8,
    146 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    132 as libc::c_int as yytype_uint8,
    120 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    120 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    114 as libc::c_int as yytype_uint8,
    139 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    97 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    94 as libc::c_int as yytype_uint8,
    94 as libc::c_int as yytype_uint8,
    94 as libc::c_int as yytype_uint8,
    115 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    94 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    94 as libc::c_int as yytype_uint8,
    99 as libc::c_int as yytype_uint8,
    93 as libc::c_int as yytype_uint8,
    145 as libc::c_int as yytype_uint8,
    146 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    144 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    148 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    120 as libc::c_int as yytype_uint8,
    121 as libc::c_int as yytype_uint8,
    100 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    95 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    101 as libc::c_int as yytype_uint8,
    94 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    106 as libc::c_int as yytype_uint8,
    116 as libc::c_int as yytype_uint8,
    117 as libc::c_int as yytype_uint8,
    143 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    144 as libc::c_int as yytype_uint8,
    94 as libc::c_int as yytype_uint8,
    133 as libc::c_int as yytype_uint8,
    134 as libc::c_int as yytype_uint8,
    135 as libc::c_int as yytype_uint8,
    136 as libc::c_int as yytype_uint8,
    70 as libc::c_int as yytype_uint8,
    144 as libc::c_int as yytype_uint8,
    144 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    146 as libc::c_int as yytype_uint8,
    117 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    107 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    135 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    139 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    144 as libc::c_int as yytype_uint8,
    108 as libc::c_int as yytype_uint8,
    95 as libc::c_int as yytype_uint8,
    142 as libc::c_int as yytype_uint8,
    95 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    144 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    146 as libc::c_int as yytype_uint8,
    126 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    102 as libc::c_int as yytype_uint8,
    144 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    146 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    95 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    101 as libc::c_int as yytype_uint8,
    95 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    89 as libc::c_int as yytype_uint8,
    91 as libc::c_int as yytype_uint8,
    104 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    147 as libc::c_int as yytype_uint8,
    143 as libc::c_int as yytype_uint8,
    101 as libc::c_int as yytype_uint8,
    144 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    147 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    144 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    93 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    95 as libc::c_int as yytype_uint8,
    93 as libc::c_int as yytype_uint8,
    95 as libc::c_int as yytype_uint8,
];
static mut yyr1: [yytype_uint8; 210] = [
    0 as libc::c_int as yytype_uint8,
    77 as libc::c_int as yytype_uint8,
    78 as libc::c_int as yytype_uint8,
    78 as libc::c_int as yytype_uint8,
    78 as libc::c_int as yytype_uint8,
    78 as libc::c_int as yytype_uint8,
    78 as libc::c_int as yytype_uint8,
    79 as libc::c_int as yytype_uint8,
    79 as libc::c_int as yytype_uint8,
    79 as libc::c_int as yytype_uint8,
    79 as libc::c_int as yytype_uint8,
    79 as libc::c_int as yytype_uint8,
    79 as libc::c_int as yytype_uint8,
    80 as libc::c_int as yytype_uint8,
    80 as libc::c_int as yytype_uint8,
    80 as libc::c_int as yytype_uint8,
    81 as libc::c_int as yytype_uint8,
    81 as libc::c_int as yytype_uint8,
    81 as libc::c_int as yytype_uint8,
    82 as libc::c_int as yytype_uint8,
    82 as libc::c_int as yytype_uint8,
    82 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    84 as libc::c_int as yytype_uint8,
    85 as libc::c_int as yytype_uint8,
    85 as libc::c_int as yytype_uint8,
    85 as libc::c_int as yytype_uint8,
    85 as libc::c_int as yytype_uint8,
    86 as libc::c_int as yytype_uint8,
    86 as libc::c_int as yytype_uint8,
    88 as libc::c_int as yytype_uint8,
    87 as libc::c_int as yytype_uint8,
    90 as libc::c_int as yytype_uint8,
    89 as libc::c_int as yytype_uint8,
    91 as libc::c_int as yytype_uint8,
    92 as libc::c_int as yytype_uint8,
    92 as libc::c_int as yytype_uint8,
    93 as libc::c_int as yytype_uint8,
    93 as libc::c_int as yytype_uint8,
    93 as libc::c_int as yytype_uint8,
    94 as libc::c_int as yytype_uint8,
    94 as libc::c_int as yytype_uint8,
    95 as libc::c_int as yytype_uint8,
    95 as libc::c_int as yytype_uint8,
    95 as libc::c_int as yytype_uint8,
    95 as libc::c_int as yytype_uint8,
    95 as libc::c_int as yytype_uint8,
    95 as libc::c_int as yytype_uint8,
    95 as libc::c_int as yytype_uint8,
    95 as libc::c_int as yytype_uint8,
    95 as libc::c_int as yytype_uint8,
    95 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    97 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    99 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    100 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    101 as libc::c_int as yytype_uint8,
    101 as libc::c_int as yytype_uint8,
    102 as libc::c_int as yytype_uint8,
    102 as libc::c_int as yytype_uint8,
    102 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    104 as libc::c_int as yytype_uint8,
    104 as libc::c_int as yytype_uint8,
    104 as libc::c_int as yytype_uint8,
    104 as libc::c_int as yytype_uint8,
    104 as libc::c_int as yytype_uint8,
    104 as libc::c_int as yytype_uint8,
    105 as libc::c_int as yytype_uint8,
    105 as libc::c_int as yytype_uint8,
    106 as libc::c_int as yytype_uint8,
    106 as libc::c_int as yytype_uint8,
    107 as libc::c_int as yytype_uint8,
    108 as libc::c_int as yytype_uint8,
    107 as libc::c_int as yytype_uint8,
    109 as libc::c_int as yytype_uint8,
    109 as libc::c_int as yytype_uint8,
    110 as libc::c_int as yytype_uint8,
    110 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    112 as libc::c_int as yytype_uint8,
    112 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    114 as libc::c_int as yytype_uint8,
    114 as libc::c_int as yytype_uint8,
    114 as libc::c_int as yytype_uint8,
    114 as libc::c_int as yytype_uint8,
    114 as libc::c_int as yytype_uint8,
    115 as libc::c_int as yytype_uint8,
    115 as libc::c_int as yytype_uint8,
    116 as libc::c_int as yytype_uint8,
    116 as libc::c_int as yytype_uint8,
    117 as libc::c_int as yytype_uint8,
    117 as libc::c_int as yytype_uint8,
    117 as libc::c_int as yytype_uint8,
    117 as libc::c_int as yytype_uint8,
    117 as libc::c_int as yytype_uint8,
    117 as libc::c_int as yytype_uint8,
    118 as libc::c_int as yytype_uint8,
    118 as libc::c_int as yytype_uint8,
    119 as libc::c_int as yytype_uint8,
    119 as libc::c_int as yytype_uint8,
    119 as libc::c_int as yytype_uint8,
    119 as libc::c_int as yytype_uint8,
    119 as libc::c_int as yytype_uint8,
    119 as libc::c_int as yytype_uint8,
    120 as libc::c_int as yytype_uint8,
    120 as libc::c_int as yytype_uint8,
    121 as libc::c_int as yytype_uint8,
    121 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    123 as libc::c_int as yytype_uint8,
    123 as libc::c_int as yytype_uint8,
    123 as libc::c_int as yytype_uint8,
    124 as libc::c_int as yytype_uint8,
    124 as libc::c_int as yytype_uint8,
    125 as libc::c_int as yytype_uint8,
    125 as libc::c_int as yytype_uint8,
    126 as libc::c_int as yytype_uint8,
    126 as libc::c_int as yytype_uint8,
    126 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    128 as libc::c_int as yytype_uint8,
    128 as libc::c_int as yytype_uint8,
    128 as libc::c_int as yytype_uint8,
    128 as libc::c_int as yytype_uint8,
    128 as libc::c_int as yytype_uint8,
    128 as libc::c_int as yytype_uint8,
    128 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    130 as libc::c_int as yytype_uint8,
    130 as libc::c_int as yytype_uint8,
    131 as libc::c_int as yytype_uint8,
    132 as libc::c_int as yytype_uint8,
    132 as libc::c_int as yytype_uint8,
    133 as libc::c_int as yytype_uint8,
    133 as libc::c_int as yytype_uint8,
    134 as libc::c_int as yytype_uint8,
    134 as libc::c_int as yytype_uint8,
    135 as libc::c_int as yytype_uint8,
    136 as libc::c_int as yytype_uint8,
    137 as libc::c_int as yytype_uint8,
    137 as libc::c_int as yytype_uint8,
    138 as libc::c_int as yytype_uint8,
    139 as libc::c_int as yytype_uint8,
    139 as libc::c_int as yytype_uint8,
    140 as libc::c_int as yytype_uint8,
    140 as libc::c_int as yytype_uint8,
    141 as libc::c_int as yytype_uint8,
    141 as libc::c_int as yytype_uint8,
    141 as libc::c_int as yytype_uint8,
    142 as libc::c_int as yytype_uint8,
    143 as libc::c_int as yytype_uint8,
    144 as libc::c_int as yytype_uint8,
    145 as libc::c_int as yytype_uint8,
    145 as libc::c_int as yytype_uint8,
    146 as libc::c_int as yytype_uint8,
    147 as libc::c_int as yytype_uint8,
    148 as libc::c_int as yytype_uint8,
];
static mut yyr2: [yytype_int8; 210] = [
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
];
unsafe extern "C" fn yydestruct(
    mut yymsg: *const libc::c_char,
    mut yykind: yysymbol_kind_t,
    mut yyvaluep: *mut *mut INSTRUCTION,
) {
    if yymsg.is_null() {
        yymsg = b"Deleting\0" as *const u8 as *const libc::c_char;
    }
}
pub static mut yychar: libc::c_int = 0;
pub static mut yylval: *mut INSTRUCTION = 0 as *const INSTRUCTION as *mut INSTRUCTION;
pub static mut yynerrs: libc::c_int = 0;
pub unsafe extern "C" fn yyparse() -> libc::c_int {
    let mut current_block: u64;
    let mut yystate: yy_state_fast_t = 0 as libc::c_int;
    let mut yyerrstatus: libc::c_int = 0 as libc::c_int;
    let mut yystacksize: libc::c_long = 200 as libc::c_int as libc::c_long;
    let mut yyssa: [yy_state_t; 200] = [0; 200];
    let mut yyss: *mut yy_state_t = yyssa.as_mut_ptr();
    let mut yyssp: *mut yy_state_t = yyss;
    let mut yyvsa: [*mut INSTRUCTION; 200] = [0 as *mut INSTRUCTION; 200];
    let mut yyvs: *mut *mut INSTRUCTION = yyvsa.as_mut_ptr();
    let mut yyvsp: *mut *mut INSTRUCTION = yyvs;
    let mut yyn: libc::c_int = 0;
    let mut yyresult: libc::c_int = 0;
    let mut yytoken: yysymbol_kind_t = YYSYMBOL_YYEMPTY;
    let mut yyval: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut yylen: libc::c_int = 0 as libc::c_int;
    yychar = -(2 as libc::c_int);
    's_46: loop {
        (0 as libc::c_int != 0
            && (0 as libc::c_int <= yystate && yystate < 356 as libc::c_int))
            as libc::c_int;
        *yyssp = yystate as yy_state_t;
        if yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
            <= yyssp
        {
            let mut yysize: libc::c_long = yyssp.offset_from(yyss) as libc::c_long
                + 1 as libc::c_int as libc::c_long;
            if 10000 as libc::c_int as libc::c_long <= yystacksize {
                current_block = 7639343393969835181;
                break;
            }
            yystacksize *= 2 as libc::c_int as libc::c_long;
            if (10000 as libc::c_int as libc::c_long) < yystacksize {
                yystacksize = 10000 as libc::c_int as libc::c_long;
            }
            let mut yyss1: *mut yy_state_t = yyss;
            let mut yyptr: *mut yyalloc = malloc(
                (yystacksize
                    * (::std::mem::size_of::<yy_state_t>() as libc::c_ulong
                        as libc::c_long
                        + ::std::mem::size_of::<*mut INSTRUCTION>() as libc::c_ulong
                            as libc::c_long)
                    + (::std::mem::size_of::<yyalloc>() as libc::c_ulong as libc::c_long
                        - 1 as libc::c_int as libc::c_long)) as libc::c_ulong,
            ) as *mut yyalloc;
            if yyptr.is_null() {
                current_block = 7639343393969835181;
                break;
            }
            let mut yynewbytes: libc::c_long = 0;
            libc::memcpy(
                &mut (*yyptr).yyss_alloc as *mut yy_state_t as *mut libc::c_void,
                yyss as *const libc::c_void,
                (yysize as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<yy_state_t>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyss = &mut (*yyptr).yyss_alloc;
            yynewbytes = yystacksize
                * ::std::mem::size_of::<yy_state_t>() as libc::c_ulong as libc::c_long
                + (::std::mem::size_of::<yyalloc>() as libc::c_ulong as libc::c_long
                    - 1 as libc::c_int as libc::c_long);
            yyptr = yyptr
                .offset(
                    (yynewbytes
                        / ::std::mem::size_of::<yyalloc>() as libc::c_ulong
                            as libc::c_long) as isize,
                );
            let mut yynewbytes_0: libc::c_long = 0;
            libc::memcpy(
                &mut (*yyptr).yyvs_alloc as *mut *mut INSTRUCTION as *mut libc::c_void,
                yyvs as *const libc::c_void,
                (yysize as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*mut INSTRUCTION>() as libc::c_ulong,
                    ) as libc::size_t,
            );
            yyvs = &mut (*yyptr).yyvs_alloc;
            yynewbytes_0 = yystacksize
                * ::std::mem::size_of::<*mut INSTRUCTION>() as libc::c_ulong
                    as libc::c_long
                + (::std::mem::size_of::<yyalloc>() as libc::c_ulong as libc::c_long
                    - 1 as libc::c_int as libc::c_long);
            yyptr = yyptr
                .offset(
                    (yynewbytes_0
                        / ::std::mem::size_of::<yyalloc>() as libc::c_ulong
                            as libc::c_long) as isize,
                );
            if yyss1 != yyssa.as_mut_ptr() {
                free(yyss1 as *mut libc::c_void);
            }
            yyssp = yyss.offset(yysize as isize).offset(-(1 as libc::c_int as isize));
            yyvsp = yyvs.offset(yysize as isize).offset(-(1 as libc::c_int as isize));
            if yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
                <= yyssp
            {
                current_block = 3397147810542949670;
                break;
            }
        }
        if yystate == 2 as libc::c_int {
            yyresult = 0 as libc::c_int;
            current_block = 9002221925005113408;
            break;
        } else {
            yyn = yypact[yystate as usize] as libc::c_int;
            if yyn == -(276 as libc::c_int) {
                current_block = 12608206817880674234;
            } else {
                if yychar == -(2 as libc::c_int) {
                    yychar = yylex();
                }
                if yychar <= 0 as libc::c_int {
                    yychar = 0 as libc::c_int;
                    yytoken = YYSYMBOL_YYEOF;
                    current_block = 6174974146017752131;
                } else if yychar == 256 as libc::c_int {
                    yychar = 257 as libc::c_int;
                    yytoken = YYSYMBOL_YYerror;
                    current_block = 10380106008157032803;
                } else {
                    yytoken = (if 0 as libc::c_int <= yychar
                        && yychar <= 310 as libc::c_int
                    {
                        yytranslate[yychar as usize] as yysymbol_kind_t as libc::c_int
                    } else {
                        YYSYMBOL_YYUNDEF as libc::c_int
                    }) as yysymbol_kind_t;
                    current_block = 6174974146017752131;
                }
                match current_block {
                    10380106008157032803 => {}
                    _ => {
                        yyn += yytoken as libc::c_int;
                        if yyn < 0 as libc::c_int || (1200 as libc::c_int) < yyn
                            || yycheck[yyn as usize] as libc::c_int
                                != yytoken as libc::c_int
                        {
                            current_block = 12608206817880674234;
                        } else {
                            yyn = yytable[yyn as usize] as libc::c_int;
                            if yyn <= 0 as libc::c_int {
                                if yyn == -(119 as libc::c_int) {
                                    current_block = 2281063462181739393;
                                } else {
                                    yyn = -yyn;
                                    current_block = 3190367813740819885;
                                }
                            } else {
                                if yyerrstatus != 0 {
                                    yyerrstatus -= 1;
                                    yyerrstatus;
                                }
                                yystate = yyn;
                                yyvsp = yyvsp.offset(1);
                                *yyvsp = yylval;
                                yychar = -(2 as libc::c_int);
                                current_block = 12782234849683662922;
                            }
                        }
                    }
                }
            }
            match current_block {
                12608206817880674234 => {
                    yyn = yydefact[yystate as usize] as libc::c_int;
                    if yyn == 0 as libc::c_int {
                        current_block = 2281063462181739393;
                    } else {
                        current_block = 3190367813740819885;
                    }
                }
                _ => {}
            }
            match current_block {
                2281063462181739393 => {
                    yytoken = (if yychar == -(2 as libc::c_int) {
                        YYSYMBOL_YYEMPTY as libc::c_int
                    } else if 0 as libc::c_int <= yychar && yychar <= 310 as libc::c_int
                    {
                        yytranslate[yychar as usize] as yysymbol_kind_t as libc::c_int
                    } else {
                        YYSYMBOL_YYUNDEF as libc::c_int
                    }) as yysymbol_kind_t;
                    if yyerrstatus == 0 {
                        yynerrs += 1;
                        yynerrs;
                        yyerror(b"syntax error\0" as *const u8 as *const libc::c_char);
                    }
                    if yyerrstatus == 3 as libc::c_int {
                        if yychar <= 0 as libc::c_int {
                            if yychar == 0 as libc::c_int {
                                current_block = 3397147810542949670;
                                break;
                            }
                        } else {
                            yydestruct(
                                b"Error: discarding\0" as *const u8 as *const libc::c_char,
                                yytoken,
                                &mut yylval,
                            );
                            yychar = -(2 as libc::c_int);
                        }
                    }
                    current_block = 10380106008157032803;
                }
                3190367813740819885 => {
                    yylen = yyr2[yyn as usize] as libc::c_int;
                    yyval = *yyvsp.offset((1 as libc::c_int - yylen) as isize);
                    match yyn {
                        2 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        3 => {
                            rule = 0 as libc::c_int;
                            yyerrstatus = 0 as libc::c_int;
                        }
                        4 => {
                            if !(*yyvsp.offset(0 as libc::c_int as isize)).is_null() {
                                if (*yyvsp.offset(-(1 as libc::c_int) as isize)).is_null() {
                                    outer_comment = *yyvsp.offset(0 as libc::c_int as isize);
                                } else {
                                    interblock_comment = *yyvsp
                                        .offset(0 as libc::c_int as isize);
                                }
                            }
                            yyval = *yyvsp.offset(-(1 as libc::c_int) as isize);
                        }
                        5 => {
                            next_sourcefile();
                        }
                        6 => {
                            rule = 0 as libc::c_int;
                        }
                        7 => {
                            append_rule(
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                            );
                            if !pending_comment.is_null() {
                                interblock_comment = pending_comment;
                                pending_comment = 0 as *mut INSTRUCTION;
                            }
                        }
                        8 => {
                            if rule != Rule as libc::c_int {
                                msg(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"%s blocks must have an action part\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    ruletab[rule as usize],
                                );
                                errcount += 1;
                                errcount;
                            } else if (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .is_null()
                            {
                                msg(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"each rule must have a pattern or an action part\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                errcount += 1;
                                errcount;
                            } else {
                                if !(*yyvsp.offset(0 as libc::c_int as isize)).is_null() {
                                    list_append(
                                        *yyvsp.offset(-(1 as libc::c_int) as isize),
                                        *yyvsp.offset(0 as libc::c_int as isize),
                                    );
                                }
                                append_rule(
                                    *yyvsp.offset(-(1 as libc::c_int) as isize),
                                    0 as *mut INSTRUCTION,
                                );
                            }
                        }
                        9 => {
                            in_function = 0 as libc::c_int != 0;
                            mk_function(
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                            );
                            want_param_names = DONT_CHECK;
                            if !pending_comment.is_null() {
                                interblock_comment = pending_comment;
                                pending_comment = 0 as *mut INSTRUCTION;
                            }
                            yyerrstatus = 0 as libc::c_int;
                        }
                        10 => {
                            want_source = 0 as libc::c_int != 0;
                            at_seen -= 1;
                            at_seen;
                            if !(*yyvsp.offset(-(1 as libc::c_int) as isize)).is_null()
                                && !(*yyvsp.offset(0 as libc::c_int as isize)).is_null()
                            {
                                let mut s: *mut SRCFILE = *yyvsp
                                    .offset(-(1 as libc::c_int) as isize) as *mut SRCFILE;
                                (*s).comment = *yyvsp.offset(0 as libc::c_int as isize);
                            }
                            yyerrstatus = 0 as libc::c_int;
                        }
                        11 => {
                            want_source = 0 as libc::c_int != 0;
                            at_seen -= 1;
                            at_seen;
                            if !(*yyvsp.offset(-(1 as libc::c_int) as isize)).is_null()
                                && !(*yyvsp.offset(0 as libc::c_int as isize)).is_null()
                            {
                                let mut s_0: *mut SRCFILE = *yyvsp
                                    .offset(-(1 as libc::c_int) as isize) as *mut SRCFILE;
                                (*s_0).comment = *yyvsp.offset(0 as libc::c_int as isize);
                            }
                            yyerrstatus = 0 as libc::c_int;
                        }
                        12 => {
                            want_source = 0 as libc::c_int != 0;
                            want_namespace = 0 as libc::c_int != 0;
                            at_seen -= 1;
                            at_seen;
                            set_namespace(
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                            );
                            yyerrstatus = 0 as libc::c_int;
                        }
                        13 => {
                            let mut srcfile: *mut libc::c_void = 0 as *mut libc::c_void;
                            if !include_source(
                                *yyvsp.offset(0 as libc::c_int as isize),
                                &mut srcfile,
                            ) {
                                current_block = 3397147810542949670;
                                break;
                            }
                            free(
                                (**yyvsp.offset(0 as libc::c_int as isize)).d.name
                                    as *mut libc::c_void,
                            );
                            bcfree(*yyvsp.offset(0 as libc::c_int as isize));
                            yyval = srcfile as *mut INSTRUCTION;
                        }
                        14 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        15 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        16 => {
                            let mut srcfile_0: *mut libc::c_void = 0
                                as *mut libc::c_void;
                            if !load_library(
                                *yyvsp.offset(0 as libc::c_int as isize),
                                &mut srcfile_0,
                            ) {
                                current_block = 3397147810542949670;
                                break;
                            }
                            free(
                                (**yyvsp.offset(0 as libc::c_int as isize)).d.name
                                    as *mut libc::c_void,
                            );
                            bcfree(*yyvsp.offset(0 as libc::c_int as isize));
                            yyval = srcfile_0 as *mut INSTRUCTION;
                        }
                        17 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        18 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        19 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        20 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        21 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        22 => {
                            rule = Rule as libc::c_int;
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        23 => {
                            rule = Rule as libc::c_int;
                        }
                        24 => {
                            let mut tp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            add_lint(
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                                LINT_assign_in_cond,
                            );
                            add_lint(
                                *yyvsp.offset(0 as libc::c_int as isize),
                                LINT_assign_in_cond,
                            );
                            tp = bcalloc(Op_no_op, 1 as libc::c_int, 0 as libc::c_int);
                            list_prepend(
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                                bcalloc(
                                    Op_line_range,
                                    (do_flags as libc::c_uint
                                        & DO_PRETTY_PRINT as libc::c_int as libc::c_uint != 0)
                                        as libc::c_int + 1 as libc::c_int,
                                    0 as libc::c_int,
                                ),
                            );
                            (*(**yyvsp.offset(-(2 as libc::c_int) as isize)).nexti)
                                .x
                                .xl = 0 as libc::c_int as libc::c_long;
                            let ref mut fresh0 = (*(**yyvsp
                                .offset(-(2 as libc::c_int) as isize))
                                .nexti)
                                .d
                                .di;
                            *fresh0 = (**yyvsp.offset(0 as libc::c_int as isize)).nexti;
                            list_append(
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                                bcalloc(Op_cond_pair, 1 as libc::c_int, 0 as libc::c_int),
                            );
                            let ref mut fresh1 = (*(**yyvsp
                                .offset(-(2 as libc::c_int) as isize))
                                .d
                                .di)
                                .x
                                .xi;
                            *fresh1 = (**yyvsp.offset(-(2 as libc::c_int) as isize))
                                .nexti;
                            let ref mut fresh2 = (*(**yyvsp
                                .offset(-(2 as libc::c_int) as isize))
                                .d
                                .di)
                                .d
                                .di;
                            *fresh2 = tp;
                            list_append(
                                *yyvsp.offset(0 as libc::c_int as isize),
                                bcalloc(Op_cond_pair, 1 as libc::c_int, 0 as libc::c_int),
                            );
                            let ref mut fresh3 = (*(**yyvsp
                                .offset(0 as libc::c_int as isize))
                                .d
                                .di)
                                .x
                                .xi;
                            *fresh3 = (**yyvsp.offset(-(2 as libc::c_int) as isize))
                                .nexti;
                            let ref mut fresh4 = (*(**yyvsp
                                .offset(0 as libc::c_int as isize))
                                .d
                                .di)
                                .d
                                .di;
                            *fresh4 = tp;
                            if do_flags as libc::c_uint
                                & DO_PRETTY_PRINT as libc::c_int as libc::c_uint != 0
                            {
                                let ref mut fresh5 = (*((**yyvsp
                                    .offset(-(2 as libc::c_int) as isize))
                                    .nexti)
                                    .offset(1 as libc::c_int as isize))
                                    .d
                                    .di;
                                *fresh5 = (**yyvsp.offset(-(2 as libc::c_int) as isize))
                                    .d
                                    .di;
                                let ref mut fresh6 = (*((**yyvsp
                                    .offset(-(2 as libc::c_int) as isize))
                                    .nexti)
                                    .offset(1 as libc::c_int as isize))
                                    .x
                                    .xi;
                                *fresh6 = (**yyvsp.offset(0 as libc::c_int as isize)).d.di;
                            }
                            if !(*yyvsp.offset(-(1 as libc::c_int) as isize)).is_null() {
                                yyval = list_append(
                                    list_merge(
                                        list_prepend(
                                            *yyvsp.offset(-(2 as libc::c_int) as isize),
                                            *yyvsp.offset(-(1 as libc::c_int) as isize),
                                        ),
                                        *yyvsp.offset(0 as libc::c_int as isize),
                                    ),
                                    tp,
                                );
                            } else {
                                yyval = list_append(
                                    list_merge(
                                        *yyvsp.offset(-(2 as libc::c_int) as isize),
                                        *yyvsp.offset(0 as libc::c_int as isize),
                                    ),
                                    tp,
                                );
                            }
                            rule = Rule as libc::c_int;
                        }
                        25 => {
                            static mut begin_seen: libc::c_int = 0 as libc::c_int;
                            if do_flags as libc::c_uint
                                & DO_LINT_OLD as libc::c_int as libc::c_uint != 0
                                && {
                                    begin_seen += 1;
                                    begin_seen == 2 as libc::c_int
                                }
                            {
                                lintwarn_ln(
                                    (**yyvsp.offset(0 as libc::c_int as isize)).source_line
                                        as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"old awk does not support multiple `BEGIN' or `END' rules\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            rule = BEGIN as libc::c_int;
                            (**yyvsp.offset(0 as libc::c_int as isize))
                                .x
                                .xl = rule as libc::c_long;
                            let ref mut fresh7 = (**yyvsp
                                .offset(0 as libc::c_int as isize))
                                .d
                                .name;
                            *fresh7 = source;
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        26 => {
                            static mut end_seen: libc::c_int = 0 as libc::c_int;
                            if do_flags as libc::c_uint
                                & DO_LINT_OLD as libc::c_int as libc::c_uint != 0
                                && {
                                    end_seen += 1;
                                    end_seen == 2 as libc::c_int
                                }
                            {
                                lintwarn_ln(
                                    (**yyvsp.offset(0 as libc::c_int as isize)).source_line
                                        as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"old awk does not support multiple `BEGIN' or `END' rules\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            rule = END as libc::c_int;
                            (**yyvsp.offset(0 as libc::c_int as isize))
                                .x
                                .xl = rule as libc::c_long;
                            let ref mut fresh8 = (**yyvsp
                                .offset(0 as libc::c_int as isize))
                                .d
                                .name;
                            *fresh8 = source;
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        27 => {
                            rule = BEGINFILE as libc::c_int;
                            (**yyvsp.offset(0 as libc::c_int as isize))
                                .x
                                .xl = rule as libc::c_long;
                            let ref mut fresh9 = (**yyvsp
                                .offset(0 as libc::c_int as isize))
                                .d
                                .name;
                            *fresh9 = source;
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        28 => {
                            rule = ENDFILE as libc::c_int;
                            (**yyvsp.offset(0 as libc::c_int as isize))
                                .x
                                .xl = rule as libc::c_long;
                            let ref mut fresh10 = (**yyvsp
                                .offset(0 as libc::c_int as isize))
                                .d
                                .name;
                            *fresh10 = source;
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        29 => {
                            let mut ip: *mut INSTRUCTION = make_braced_statements(
                                *yyvsp.offset(-(4 as libc::c_int) as isize),
                                *yyvsp.offset(-(3 as libc::c_int) as isize),
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                            );
                            if !(*yyvsp.offset(-(2 as libc::c_int) as isize)).is_null()
                                && !(*yyvsp.offset(0 as libc::c_int as isize)).is_null()
                            {
                                merge_comments(
                                    *yyvsp.offset(-(2 as libc::c_int) as isize),
                                    *yyvsp.offset(0 as libc::c_int as isize),
                                );
                                pending_comment = *yyvsp
                                    .offset(-(2 as libc::c_int) as isize);
                            } else if !(*yyvsp.offset(-(2 as libc::c_int) as isize))
                                .is_null()
                            {
                                pending_comment = *yyvsp
                                    .offset(-(2 as libc::c_int) as isize);
                            } else if !(*yyvsp.offset(0 as libc::c_int as isize))
                                .is_null()
                            {
                                pending_comment = *yyvsp.offset(0 as libc::c_int as isize);
                            }
                            yyval = ip;
                        }
                        31 => {
                            let mut name: *const libc::c_char = (**yyvsp
                                .offset(0 as libc::c_int as isize))
                                .d
                                .name;
                            let mut qname: *mut libc::c_char = qualify_name(
                                name,
                                strlen(name),
                            );
                            if qname != name as *mut libc::c_char {
                                free(name as *mut libc::c_void);
                                let ref mut fresh11 = (**yyvsp
                                    .offset(0 as libc::c_int as isize))
                                    .d
                                    .name;
                                *fresh11 = qname;
                            }
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        32 => {
                            yyerror(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"`%s' is a built-in function, it cannot be redefined\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                tokstart,
                            );
                            current_block = 3397147810542949670;
                            break;
                        }
                        33 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                            at_seen -= 1;
                            at_seen;
                        }
                        36 => {
                            want_param_names = FUNC_HEADER;
                        }
                        37 => {
                            let mut func_comment: *mut INSTRUCTION = 0
                                as *mut INSTRUCTION;
                            if !(*yyvsp.offset(-(2 as libc::c_int) as isize)).is_null()
                                && !((**yyvsp.offset(-(2 as libc::c_int) as isize)).comment)
                                    .is_null()
                            {
                                if !(*yyvsp.offset(0 as libc::c_int as isize)).is_null() {
                                    merge_comments(
                                        (**yyvsp.offset(-(2 as libc::c_int) as isize)).comment,
                                        *yyvsp.offset(0 as libc::c_int as isize),
                                    );
                                }
                                func_comment = (**yyvsp
                                    .offset(-(2 as libc::c_int) as isize))
                                    .comment;
                            } else if !(*yyvsp.offset(0 as libc::c_int as isize))
                                .is_null()
                            {
                                func_comment = *yyvsp.offset(0 as libc::c_int as isize);
                            }
                            let ref mut fresh12 = (**yyvsp
                                .offset(-(6 as libc::c_int) as isize))
                                .d
                                .name;
                            *fresh12 = source;
                            let ref mut fresh13 = (**yyvsp
                                .offset(-(6 as libc::c_int) as isize))
                                .comment;
                            *fresh13 = func_comment;
                            if install_function(
                                (**yyvsp.offset(-(5 as libc::c_int) as isize)).d.name,
                                *yyvsp.offset(-(6 as libc::c_int) as isize),
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                            ) < 0 as libc::c_int
                            {
                                current_block = 3397147810542949670;
                                break;
                            }
                            in_function = 1 as libc::c_int != 0;
                            let ref mut fresh14 = (**yyvsp
                                .offset(-(5 as libc::c_int) as isize))
                                .d
                                .name;
                            *fresh14 = 0 as *mut libc::c_char;
                            bcfree(*yyvsp.offset(-(5 as libc::c_int) as isize));
                            yyval = *yyvsp.offset(-(6 as libc::c_int) as isize);
                            want_param_names = FUNC_BODY;
                        }
                        38 => {
                            want_regexp = 1 as libc::c_int != 0;
                        }
                        39 => {
                            let mut n: *mut NODE = 0 as *mut NODE;
                            let mut exp: *mut NODE = 0 as *mut NODE;
                            let mut re: *mut libc::c_char = 0 as *mut libc::c_char;
                            let mut len: size_t = 0;
                            re = (**yyvsp.offset(0 as libc::c_int as isize)).d.name;
                            let ref mut fresh15 = (**yyvsp
                                .offset(0 as libc::c_int as isize))
                                .d
                                .name;
                            *fresh15 = 0 as *mut libc::c_char;
                            len = strlen(re);
                            if do_flags as libc::c_uint
                                & (DO_LINT_INVALID as libc::c_int
                                    | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                            {
                                if len == 0 as libc::c_int as libc::c_ulong {
                                    lintwarn_ln(
                                        (**yyvsp.offset(0 as libc::c_int as isize)).source_line
                                            as libc::c_int,
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"regexp constant `//' looks like a C++ comment, but is not\0"
                                                as *const u8 as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                } else if *re.offset(0 as libc::c_int as isize)
                                    as libc::c_int == '*' as i32
                                    && *re
                                        .offset(
                                            len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == '*' as i32
                                {
                                    lintwarn_ln(
                                        (**yyvsp.offset(0 as libc::c_int as isize)).source_line
                                            as libc::c_int,
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"regexp constant `/%s/' looks like a C comment, but is not\0"
                                                as *const u8 as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                        re,
                                    );
                                }
                            }
                            exp = make_str_node(re, len, 2 as libc::c_int);
                            n = make_regnode(Node_regex, exp);
                            if n.is_null() {
                                unref(exp);
                                current_block = 3397147810542949670;
                                break;
                            } else {
                                yyval = *yyvsp.offset(0 as libc::c_int as isize);
                                (*yyval).opcode = Op_match_rec;
                                (*yyval).d.dn = n;
                            }
                        }
                        40 => {
                            let mut re_0: *mut libc::c_char = 0 as *mut libc::c_char;
                            let mut len_0: size_t = 0;
                            re_0 = (**yyvsp.offset(0 as libc::c_int as isize)).d.name;
                            let ref mut fresh16 = (**yyvsp
                                .offset(0 as libc::c_int as isize))
                                .d
                                .name;
                            *fresh16 = 0 as *mut libc::c_char;
                            len_0 = strlen(re_0);
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                            (*yyval).opcode = Op_push_re;
                            (*yyval).d.dn = make_typed_regex(re_0, len_0);
                        }
                        41 => {
                            bcfree(*yyvsp.offset(0 as libc::c_int as isize));
                        }
                        43 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        44 => {
                            if (*yyvsp.offset(0 as libc::c_int as isize)).is_null() {
                                yyval = *yyvsp.offset(-(1 as libc::c_int) as isize);
                            } else {
                                add_lint(
                                    *yyvsp.offset(0 as libc::c_int as isize),
                                    LINT_no_effect,
                                );
                                if (*yyvsp.offset(-(1 as libc::c_int) as isize)).is_null() {
                                    yyval = *yyvsp.offset(0 as libc::c_int as isize);
                                } else {
                                    yyval = list_merge(
                                        *yyvsp.offset(-(1 as libc::c_int) as isize),
                                        *yyvsp.offset(0 as libc::c_int as isize),
                                    );
                                }
                            }
                            if !trailing_comment.is_null() {
                                yyval = list_append(yyval, trailing_comment);
                                trailing_comment = 0 as *mut INSTRUCTION;
                            }
                            yyerrstatus = 0 as libc::c_int;
                        }
                        45 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        46 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        47 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        48 => {
                            if !(*yyvsp.offset(0 as libc::c_int as isize)).is_null() {
                                let mut ip_0: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                                merge_comments(
                                    *yyvsp.offset(0 as libc::c_int as isize),
                                    0 as *mut INSTRUCTION,
                                );
                                ip_0 = list_create(
                                    bcalloc(Op_no_op, 1 as libc::c_int, 0 as libc::c_int),
                                );
                                yyval = list_append(
                                    ip_0,
                                    *yyvsp.offset(0 as libc::c_int as isize),
                                );
                            } else {
                                yyval = 0 as *mut INSTRUCTION;
                            }
                        }
                        49 => {
                            trailing_comment = *yyvsp.offset(0 as libc::c_int as isize);
                            yyval = make_braced_statements(
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                            );
                        }
                        50 => {
                            if do_flags as libc::c_uint
                                & DO_PRETTY_PRINT as libc::c_int as libc::c_uint != 0
                            {
                                yyval = list_prepend(
                                    *yyvsp.offset(0 as libc::c_int as isize),
                                    bcalloc(Op_exec_count, 1 as libc::c_int, 0 as libc::c_int),
                                );
                            } else {
                                yyval = *yyvsp.offset(0 as libc::c_int as isize);
                            }
                        }
                        51 => {
                            let mut dflt: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut curr: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut cexp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut cstmt: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut ip_1: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut nextc_0: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut tbreak: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut case_values: *mut *const libc::c_char = 0
                                as *mut *const libc::c_char;
                            let mut maxcount: libc::c_int = 128 as libc::c_int;
                            let mut case_count: libc::c_int = 0 as libc::c_int;
                            let mut i: libc::c_int = 0;
                            tbreak = bcalloc(
                                Op_no_op,
                                1 as libc::c_int,
                                0 as libc::c_int,
                            );
                            cstmt = list_create(tbreak);
                            cexp = list_create(
                                bcalloc(Op_pop, 1 as libc::c_int, 0 as libc::c_int),
                            );
                            dflt = bcalloc(Op_jmp, 1 as libc::c_int, 0 as libc::c_int);
                            (*dflt).d.di = tbreak;
                            if !(*yyvsp.offset(-(2 as libc::c_int) as isize)).is_null() {
                                curr = (**yyvsp.offset(-(2 as libc::c_int) as isize)).nexti;
                                bcfree(*yyvsp.offset(-(2 as libc::c_int) as isize));
                            }
                            while !curr.is_null() {
                                let mut caseexp: *mut INSTRUCTION = (*curr).d.di;
                                let mut casestmt: *mut INSTRUCTION = (*curr).x.xi;
                                nextc_0 = (*curr).nexti;
                                if (*curr).opcode as libc::c_uint
                                    == Op_K_case as libc::c_int as libc::c_uint
                                {
                                    if (*caseexp).opcode as libc::c_uint
                                        == Op_push_i as libc::c_int as libc::c_uint
                                    {
                                        let mut caseval: *mut libc::c_char = 0 as *mut libc::c_char;
                                        caseval = (*force_string_fmt(
                                            (*caseexp).d.dn,
                                            CONVFMT,
                                            CONVFMTidx,
                                        ))
                                            .sub
                                            .val
                                            .sp;
                                        i = 0 as libc::c_int;
                                        while i < case_count {
                                            if strcmp(caseval, *case_values.offset(i as isize))
                                                == 0 as libc::c_int
                                            {
                                                error_ln(
                                                    (*curr).source_line as libc::c_int,
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"duplicate case values in switch body: %s\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    ),
                                                    caseval,
                                                );
                                            }
                                            i += 1;
                                            i;
                                        }
                                        if case_values.is_null() {
                                            case_values = emalloc_real(
                                                (::std::mem::size_of::<*mut libc::c_char>()
                                                    as libc::c_ulong)
                                                    .wrapping_mul(maxcount as libc::c_ulong),
                                                b"statement\0" as *const u8 as *const libc::c_char,
                                                b"case_values\0" as *const u8 as *const libc::c_char,
                                                b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                                699 as libc::c_int,
                                            ) as *mut *const libc::c_char;
                                        } else if case_count >= maxcount {
                                            maxcount += 128 as libc::c_int;
                                            case_values = erealloc_real(
                                                case_values as *mut libc::c_void,
                                                (::std::mem::size_of::<*mut libc::c_char>()
                                                    as libc::c_ulong)
                                                    .wrapping_mul(maxcount as libc::c_ulong),
                                                b"statement\0" as *const u8 as *const libc::c_char,
                                                b"case_values\0" as *const u8 as *const libc::c_char,
                                                b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                                702 as libc::c_int,
                                            ) as *mut *const libc::c_char;
                                        }
                                        let fresh17 = case_count;
                                        case_count = case_count + 1;
                                        let ref mut fresh18 = *case_values.offset(fresh17 as isize);
                                        *fresh18 = caseval;
                                    } else {
                                        (*curr.offset(1 as libc::c_int as isize))
                                            .x
                                            .xl = 1 as libc::c_int as libc::c_long;
                                    }
                                    (*curr).d.di = (*casestmt).nexti;
                                    (*curr).x.xi = (*casestmt).d.di;
                                    list_prepend(cexp, curr);
                                    list_prepend(cexp, caseexp);
                                } else {
                                    if (*dflt).d.di != tbreak {
                                        error_ln(
                                            (*curr).source_line as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"duplicate `default' detected in switch body\0"
                                                    as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                    } else {
                                        (*dflt).d.di = (*casestmt).nexti;
                                    }
                                    if do_flags as libc::c_uint
                                        & DO_PRETTY_PRINT as libc::c_int as libc::c_uint != 0
                                    {
                                        (*curr).d.di = (*casestmt).nexti;
                                        (*curr).x.xi = (*casestmt).d.di;
                                        list_prepend(cexp, curr);
                                    } else {
                                        bcfree(curr);
                                    }
                                }
                                cstmt = list_merge(casestmt, cstmt);
                                curr = nextc_0;
                            }
                            if !case_values.is_null() {
                                free(case_values as *mut libc::c_void);
                            }
                            ip_1 = *yyvsp.offset(-(6 as libc::c_int) as isize);
                            if do_flags as libc::c_uint
                                & DO_PRETTY_PRINT as libc::c_int as libc::c_uint != 0
                            {
                                let mut head_comment: *mut INSTRUCTION = 0
                                    as *mut INSTRUCTION;
                                if !(*yyvsp.offset(-(4 as libc::c_int) as isize)).is_null()
                                    && !(*yyvsp.offset(-(3 as libc::c_int) as isize)).is_null()
                                {
                                    merge_comments(
                                        *yyvsp.offset(-(4 as libc::c_int) as isize),
                                        *yyvsp.offset(-(3 as libc::c_int) as isize),
                                    );
                                    head_comment = *yyvsp.offset(-(4 as libc::c_int) as isize);
                                } else if !(*yyvsp.offset(-(4 as libc::c_int) as isize))
                                    .is_null()
                                {
                                    head_comment = *yyvsp.offset(-(4 as libc::c_int) as isize);
                                } else {
                                    head_comment = *yyvsp.offset(-(3 as libc::c_int) as isize);
                                }
                                let ref mut fresh19 = (**yyvsp
                                    .offset(-(8 as libc::c_int) as isize))
                                    .comment;
                                *fresh19 = head_comment;
                                list_prepend(
                                    ip_1,
                                    *yyvsp.offset(-(8 as libc::c_int) as isize),
                                );
                                list_prepend(
                                    ip_1,
                                    bcalloc(Op_exec_count, 1 as libc::c_int, 0 as libc::c_int),
                                );
                                let ref mut fresh20 = (**yyvsp
                                    .offset(-(8 as libc::c_int) as isize))
                                    .x
                                    .xi;
                                *fresh20 = tbreak;
                                let ref mut fresh21 = (*(*yyvsp
                                    .offset(-(8 as libc::c_int) as isize))
                                    .offset(1 as libc::c_int as isize))
                                    .d
                                    .di;
                                *fresh21 = (*cexp).nexti;
                                let ref mut fresh22 = (*(*yyvsp
                                    .offset(-(8 as libc::c_int) as isize))
                                    .offset(1 as libc::c_int as isize))
                                    .x
                                    .xi;
                                *fresh22 = (*cexp).d.di;
                                let ref mut fresh23 = (*(*(*yyvsp
                                    .offset(-(8 as libc::c_int) as isize))
                                    .offset(1 as libc::c_int as isize))
                                    .x
                                    .xi)
                                    .comment;
                                *fresh23 = *yyvsp.offset(0 as libc::c_int as isize);
                            }
                            list_append(cexp, dflt);
                            list_merge(ip_1, cexp);
                            if !(*yyvsp.offset(-(1 as libc::c_int) as isize)).is_null() {
                                list_append(
                                    cstmt,
                                    *yyvsp.offset(-(1 as libc::c_int) as isize),
                                );
                            }
                            yyval = list_merge(ip_1, cstmt);
                            break_allowed -= 1;
                            break_allowed;
                            fix_break_continue(ip_1, tbreak, 0 as *mut INSTRUCTION);
                        }
                        52 => {
                            let mut ip_2: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut tbreak_0: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut tcont: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            tbreak_0 = bcalloc(
                                Op_no_op,
                                1 as libc::c_int,
                                0 as libc::c_int,
                            );
                            add_lint(
                                *yyvsp.offset(-(3 as libc::c_int) as isize),
                                LINT_assign_in_cond,
                            );
                            tcont = (**yyvsp.offset(-(3 as libc::c_int) as isize)).nexti;
                            ip_2 = list_append(
                                *yyvsp.offset(-(3 as libc::c_int) as isize),
                                bcalloc(Op_jmp_false, 1 as libc::c_int, 0 as libc::c_int),
                            );
                            (*(*ip_2).d.di).d.di = tbreak_0;
                            if do_flags as libc::c_uint
                                & DO_PRETTY_PRINT as libc::c_int as libc::c_uint != 0
                            {
                                list_append(
                                    ip_2,
                                    bcalloc(Op_exec_count, 1 as libc::c_int, 0 as libc::c_int),
                                );
                                let ref mut fresh24 = (**yyvsp
                                    .offset(-(5 as libc::c_int) as isize))
                                    .x
                                    .xi;
                                *fresh24 = tbreak_0;
                                let ref mut fresh25 = (**yyvsp
                                    .offset(-(5 as libc::c_int) as isize))
                                    .d
                                    .di;
                                *fresh25 = tcont;
                                let ref mut fresh26 = (*(*yyvsp
                                    .offset(-(5 as libc::c_int) as isize))
                                    .offset(1 as libc::c_int as isize))
                                    .d
                                    .di;
                                *fresh26 = (*ip_2).d.di;
                                list_prepend(
                                    ip_2,
                                    *yyvsp.offset(-(5 as libc::c_int) as isize),
                                );
                            }
                            if !(*yyvsp.offset(-(1 as libc::c_int) as isize)).is_null() {
                                if (*yyvsp.offset(0 as libc::c_int as isize)).is_null() {
                                    let ref mut fresh27 = *yyvsp
                                        .offset(0 as libc::c_int as isize);
                                    *fresh27 = list_create(
                                        bcalloc(Op_no_op, 1 as libc::c_int, 0 as libc::c_int),
                                    );
                                }
                                (*(**yyvsp.offset(-(1 as libc::c_int) as isize)).d.dn)
                                    .sub
                                    .val
                                    .comtype = BLOCK_COMMENT;
                                let ref mut fresh28 = *yyvsp
                                    .offset(0 as libc::c_int as isize);
                                *fresh28 = list_prepend(
                                    *yyvsp.offset(0 as libc::c_int as isize),
                                    *yyvsp.offset(-(1 as libc::c_int) as isize),
                                );
                            }
                            if !(*yyvsp.offset(0 as libc::c_int as isize)).is_null() {
                                list_merge(ip_2, *yyvsp.offset(0 as libc::c_int as isize));
                            }
                            list_append(
                                ip_2,
                                bcalloc(Op_jmp, 1 as libc::c_int, 0 as libc::c_int),
                            );
                            (*(*ip_2).d.di).d.di = tcont;
                            yyval = list_append(ip_2, tbreak_0);
                            break_allowed -= 1;
                            break_allowed;
                            continue_allowed -= 1;
                            continue_allowed;
                            fix_break_continue(ip_2, tbreak_0, tcont);
                        }
                        53 => {
                            let mut ip_3: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut tbreak_1: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut tcont_0: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            tbreak_1 = bcalloc(
                                Op_no_op,
                                1 as libc::c_int,
                                0 as libc::c_int,
                            );
                            tcont_0 = (**yyvsp.offset(-(2 as libc::c_int) as isize))
                                .nexti;
                            add_lint(
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                                LINT_assign_in_cond,
                            );
                            if !(*yyvsp.offset(-(5 as libc::c_int) as isize)).is_null() {
                                ip_3 = list_merge(
                                    *yyvsp.offset(-(5 as libc::c_int) as isize),
                                    *yyvsp.offset(-(2 as libc::c_int) as isize),
                                );
                            } else {
                                ip_3 = list_prepend(
                                    *yyvsp.offset(-(2 as libc::c_int) as isize),
                                    bcalloc(Op_no_op, 1 as libc::c_int, 0 as libc::c_int),
                                );
                            }
                            if !(*yyvsp.offset(-(6 as libc::c_int) as isize)).is_null() {
                                list_prepend(
                                    ip_3,
                                    *yyvsp.offset(-(6 as libc::c_int) as isize),
                                );
                            }
                            if do_flags as libc::c_uint
                                & DO_PRETTY_PRINT as libc::c_int as libc::c_uint != 0
                            {
                                list_prepend(
                                    ip_3,
                                    bcalloc(Op_exec_count, 1 as libc::c_int, 0 as libc::c_int),
                                );
                            }
                            list_append(
                                ip_3,
                                bcalloc(Op_jmp_true, 1 as libc::c_int, 0 as libc::c_int),
                            );
                            (*(*ip_3).d.di).d.di = (*ip_3).nexti;
                            yyval = list_append(ip_3, tbreak_1);
                            break_allowed -= 1;
                            break_allowed;
                            continue_allowed -= 1;
                            continue_allowed;
                            fix_break_continue(ip_3, tbreak_1, tcont_0);
                            if do_flags as libc::c_uint
                                & DO_PRETTY_PRINT as libc::c_int as libc::c_uint != 0
                            {
                                let ref mut fresh29 = (**yyvsp
                                    .offset(-(7 as libc::c_int) as isize))
                                    .x
                                    .xi;
                                *fresh29 = tbreak_1;
                                let ref mut fresh30 = (**yyvsp
                                    .offset(-(7 as libc::c_int) as isize))
                                    .d
                                    .di;
                                *fresh30 = tcont_0;
                                let ref mut fresh31 = (*(*yyvsp
                                    .offset(-(7 as libc::c_int) as isize))
                                    .offset(1 as libc::c_int as isize))
                                    .d
                                    .di;
                                *fresh31 = tcont_0;
                                yyval = list_prepend(
                                    ip_3,
                                    *yyvsp.offset(-(7 as libc::c_int) as isize),
                                );
                                bcfree(*yyvsp.offset(-(4 as libc::c_int) as isize));
                                if !(*yyvsp.offset(0 as libc::c_int as isize)).is_null() {
                                    let ref mut fresh32 = (**yyvsp
                                        .offset(-(7 as libc::c_int) as isize))
                                        .comment;
                                    *fresh32 = *yyvsp.offset(0 as libc::c_int as isize);
                                }
                            }
                        }
                        54 => {
                            let mut ip_4: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut var_name: *mut libc::c_char = (**yyvsp
                                .offset(-(5 as libc::c_int) as isize))
                                .d
                                .name;
                            let mut tbreak_2: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut tcont_1: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut current_block_474: u64;
                            if !(*yyvsp.offset(0 as libc::c_int as isize)).is_null()
                                && (*(**yyvsp.offset(0 as libc::c_int as isize)).d.di)
                                    .opcode as libc::c_uint
                                    == Op_K_delete as libc::c_int as libc::c_uint
                                && (*(**yyvsp.offset(0 as libc::c_int as isize)).d.di).x.xl
                                    == 1 as libc::c_int as libc::c_long
                                && (*(**yyvsp.offset(0 as libc::c_int as isize)).nexti)
                                    .opcode as libc::c_uint
                                    == Op_push as libc::c_int as libc::c_uint
                                && ((*(*(**yyvsp.offset(0 as libc::c_int as isize)).nexti)
                                    .d
                                    .dn)
                                    .type_0 as libc::c_uint
                                    != Node_var as libc::c_int as libc::c_uint
                                    || ((*(*(**yyvsp.offset(0 as libc::c_int as isize)).nexti)
                                        .d
                                        .dn)
                                        .sub
                                        .nodep
                                        .r
                                        .uptr)
                                        .is_none())
                                && strcmp(
                                    (*(*(**yyvsp.offset(0 as libc::c_int as isize)).nexti).d.dn)
                                        .sub
                                        .nodep
                                        .name,
                                    var_name,
                                ) == 0 as libc::c_int
                            {
                                let mut arr: *mut NODE = 0 as *mut NODE;
                                ip_4 = (*(**yyvsp.offset(0 as libc::c_int as isize)).nexti)
                                    .nexti;
                                if (*(**yyvsp.offset(-(3 as libc::c_int) as isize)).nexti)
                                    .opcode as libc::c_uint
                                    == Op_push as libc::c_int as libc::c_uint
                                    && (**yyvsp.offset(-(3 as libc::c_int) as isize)).d.di
                                        == (**yyvsp.offset(-(3 as libc::c_int) as isize)).nexti
                                {
                                    arr = (*(**yyvsp.offset(-(3 as libc::c_int) as isize))
                                        .nexti)
                                        .d
                                        .dn;
                                }
                                if !arr.is_null()
                                    && (*ip_4).opcode as libc::c_uint
                                        == Op_no_op as libc::c_int as libc::c_uint
                                    && (*(*ip_4).nexti).opcode as libc::c_uint
                                        == Op_push_array as libc::c_int as libc::c_uint
                                    && strcmp(
                                        (*(*(*ip_4).nexti).d.dn).sub.nodep.name,
                                        (*arr).sub.nodep.name,
                                    ) == 0 as libc::c_int
                                    && (*(*ip_4).nexti).nexti
                                        == (**yyvsp.offset(0 as libc::c_int as isize)).d.di
                                {
                                    make_assignable(
                                        (**yyvsp.offset(0 as libc::c_int as isize)).nexti,
                                    );
                                    (*(**yyvsp.offset(0 as libc::c_int as isize)).d.di)
                                        .opcode = Op_K_delete_loop;
                                    (*(**yyvsp.offset(0 as libc::c_int as isize)).d.di)
                                        .x
                                        .xl = 0 as libc::c_int as libc::c_long;
                                    if !(*yyvsp.offset(-(7 as libc::c_int) as isize)).is_null()
                                    {
                                        bcfree(*yyvsp.offset(-(7 as libc::c_int) as isize));
                                    }
                                    free(var_name as *mut libc::c_void);
                                    bcfree(*yyvsp.offset(-(5 as libc::c_int) as isize));
                                    bcfree(*yyvsp.offset(-(4 as libc::c_int) as isize));
                                    bcfree(*yyvsp.offset(-(3 as libc::c_int) as isize));
                                    if !(*yyvsp.offset(-(1 as libc::c_int) as isize)).is_null()
                                    {
                                        merge_comments(
                                            *yyvsp.offset(-(1 as libc::c_int) as isize),
                                            0 as *mut INSTRUCTION,
                                        );
                                        let ref mut fresh33 = *yyvsp
                                            .offset(0 as libc::c_int as isize);
                                        *fresh33 = list_prepend(
                                            *yyvsp.offset(0 as libc::c_int as isize),
                                            *yyvsp.offset(-(1 as libc::c_int) as isize),
                                        );
                                    }
                                    yyval = *yyvsp.offset(0 as libc::c_int as isize);
                                    current_block_474 = 9622788801273990334;
                                } else {
                                    current_block_474 = 4567378207841956082;
                                }
                            } else {
                                tbreak_2 = 0 as *mut INSTRUCTION;
                                tcont_1 = 0 as *mut INSTRUCTION;
                                current_block_474 = 4567378207841956082;
                            }
                            match current_block_474 {
                                4567378207841956082 => {
                                    ip_4 = *yyvsp.offset(-(3 as libc::c_int) as isize);
                                    (*(*ip_4).nexti).opcode = Op_push_array;
                                    tbreak_2 = bcalloc(
                                        Op_arrayfor_final,
                                        1 as libc::c_int,
                                        0 as libc::c_int,
                                    );
                                    (**yyvsp.offset(-(4 as libc::c_int) as isize))
                                        .opcode = Op_arrayfor_incr;
                                    let ref mut fresh34 = (**yyvsp
                                        .offset(-(4 as libc::c_int) as isize))
                                        .x
                                        .xn;
                                    *fresh34 = variable(
                                        (**yyvsp.offset(-(5 as libc::c_int) as isize)).source_line
                                            as libc::c_int,
                                        var_name,
                                        Node_var,
                                    );
                                    let ref mut fresh35 = (**yyvsp
                                        .offset(-(4 as libc::c_int) as isize))
                                        .d
                                        .di;
                                    *fresh35 = tbreak_2;
                                    tcont_1 = *yyvsp.offset(-(4 as libc::c_int) as isize);
                                    (**yyvsp.offset(-(5 as libc::c_int) as isize))
                                        .opcode = Op_arrayfor_init;
                                    let ref mut fresh36 = (**yyvsp
                                        .offset(-(5 as libc::c_int) as isize))
                                        .d
                                        .di;
                                    *fresh36 = tbreak_2;
                                    list_append(
                                        ip_4,
                                        *yyvsp.offset(-(5 as libc::c_int) as isize),
                                    );
                                    if do_flags as libc::c_uint
                                        & DO_PRETTY_PRINT as libc::c_int as libc::c_uint != 0
                                    {
                                        (**yyvsp.offset(-(7 as libc::c_int) as isize))
                                            .opcode = Op_K_arrayfor;
                                        let ref mut fresh37 = (**yyvsp
                                            .offset(-(7 as libc::c_int) as isize))
                                            .d
                                            .di;
                                        *fresh37 = tcont_1;
                                        let ref mut fresh38 = (**yyvsp
                                            .offset(-(7 as libc::c_int) as isize))
                                            .x
                                            .xi;
                                        *fresh38 = tbreak_2;
                                        list_append(
                                            ip_4,
                                            *yyvsp.offset(-(7 as libc::c_int) as isize),
                                        );
                                    }
                                    if (*(**yyvsp.offset(-(4 as libc::c_int) as isize)).x.xn)
                                        .type_0 as libc::c_uint
                                        == Node_var as libc::c_int as libc::c_uint
                                        && ((*(**yyvsp.offset(-(4 as libc::c_int) as isize)).x.xn)
                                            .sub
                                            .nodep
                                            .r
                                            .uptr)
                                            .is_some()
                                    {
                                        list_append(
                                            ip_4,
                                            bcalloc(Op_var_update, 1 as libc::c_int, 0 as libc::c_int),
                                        );
                                        (*(*ip_4).d.di)
                                            .x
                                            .aptr = (*(**yyvsp.offset(-(4 as libc::c_int) as isize))
                                            .x
                                            .xn)
                                            .sub
                                            .nodep
                                            .r
                                            .uptr;
                                    }
                                    list_append(
                                        ip_4,
                                        *yyvsp.offset(-(4 as libc::c_int) as isize),
                                    );
                                    if (*(**yyvsp.offset(-(4 as libc::c_int) as isize)).x.xn)
                                        .type_0 as libc::c_uint
                                        == Node_var as libc::c_int as libc::c_uint
                                        && ((*(**yyvsp.offset(-(4 as libc::c_int) as isize)).x.xn)
                                            .sub
                                            .nodep
                                            .x
                                            .aptr)
                                            .is_some()
                                    {
                                        list_append(
                                            ip_4,
                                            bcalloc(Op_var_assign, 1 as libc::c_int, 0 as libc::c_int),
                                        );
                                        (*(*ip_4).d.di)
                                            .x
                                            .aptr = (*(**yyvsp.offset(-(4 as libc::c_int) as isize))
                                            .x
                                            .xn)
                                            .sub
                                            .nodep
                                            .x
                                            .aptr;
                                    }
                                    if do_flags as libc::c_uint
                                        & DO_PRETTY_PRINT as libc::c_int as libc::c_uint != 0
                                    {
                                        list_append(
                                            ip_4,
                                            bcalloc(Op_exec_count, 1 as libc::c_int, 0 as libc::c_int),
                                        );
                                        let ref mut fresh39 = (*(*yyvsp
                                            .offset(-(7 as libc::c_int) as isize))
                                            .offset(1 as libc::c_int as isize))
                                            .d
                                            .di;
                                        *fresh39 = *yyvsp.offset(-(4 as libc::c_int) as isize);
                                        let ref mut fresh40 = (*(*yyvsp
                                            .offset(-(7 as libc::c_int) as isize))
                                            .offset(1 as libc::c_int as isize))
                                            .x
                                            .xi;
                                        *fresh40 = (*ip_4).d.di;
                                    }
                                    if !(*yyvsp.offset(-(1 as libc::c_int) as isize)).is_null()
                                    {
                                        merge_comments(
                                            *yyvsp.offset(-(1 as libc::c_int) as isize),
                                            0 as *mut INSTRUCTION,
                                        );
                                    }
                                    if !(*yyvsp.offset(0 as libc::c_int as isize)).is_null() {
                                        if !(*yyvsp.offset(-(1 as libc::c_int) as isize)).is_null()
                                        {
                                            let ref mut fresh41 = *yyvsp
                                                .offset(0 as libc::c_int as isize);
                                            *fresh41 = list_prepend(
                                                *yyvsp.offset(0 as libc::c_int as isize),
                                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                                            );
                                        }
                                        list_merge(ip_4, *yyvsp.offset(0 as libc::c_int as isize));
                                    } else if !(*yyvsp.offset(-(1 as libc::c_int) as isize))
                                        .is_null()
                                    {
                                        list_append(
                                            ip_4,
                                            *yyvsp.offset(-(1 as libc::c_int) as isize),
                                        );
                                    }
                                    list_append(
                                        ip_4,
                                        bcalloc(Op_jmp, 1 as libc::c_int, 0 as libc::c_int),
                                    );
                                    (*(*ip_4).d.di)
                                        .d
                                        .di = *yyvsp.offset(-(4 as libc::c_int) as isize);
                                    yyval = list_append(ip_4, tbreak_2);
                                    fix_break_continue(ip_4, tbreak_2, tcont_1);
                                }
                                _ => {}
                            }
                            break_allowed -= 1;
                            break_allowed;
                            continue_allowed -= 1;
                            continue_allowed;
                        }
                        55 => {
                            if !(*yyvsp.offset(-(7 as libc::c_int) as isize)).is_null() {
                                merge_comments(
                                    *yyvsp.offset(-(7 as libc::c_int) as isize),
                                    0 as *mut INSTRUCTION,
                                );
                                let ref mut fresh42 = (**yyvsp
                                    .offset(-(11 as libc::c_int) as isize))
                                    .comment;
                                *fresh42 = *yyvsp.offset(-(7 as libc::c_int) as isize);
                            }
                            if !(*yyvsp.offset(-(4 as libc::c_int) as isize)).is_null() {
                                merge_comments(
                                    *yyvsp.offset(-(4 as libc::c_int) as isize),
                                    0 as *mut INSTRUCTION,
                                );
                                if ((**yyvsp.offset(-(11 as libc::c_int) as isize)).comment)
                                    .is_null()
                                {
                                    (*(**yyvsp.offset(-(4 as libc::c_int) as isize)).d.dn)
                                        .sub
                                        .val
                                        .comtype = FOR_COMMENT;
                                    let ref mut fresh43 = (**yyvsp
                                        .offset(-(11 as libc::c_int) as isize))
                                        .comment;
                                    *fresh43 = *yyvsp.offset(-(4 as libc::c_int) as isize);
                                } else {
                                    let ref mut fresh44 = (*(**yyvsp
                                        .offset(-(11 as libc::c_int) as isize))
                                        .comment)
                                        .comment;
                                    *fresh44 = *yyvsp.offset(-(4 as libc::c_int) as isize);
                                }
                            }
                            if !(*yyvsp.offset(-(1 as libc::c_int) as isize)).is_null() {
                                let ref mut fresh45 = *yyvsp
                                    .offset(0 as libc::c_int as isize);
                                *fresh45 = list_prepend(
                                    *yyvsp.offset(0 as libc::c_int as isize),
                                    *yyvsp.offset(-(1 as libc::c_int) as isize),
                                );
                            }
                            add_lint(
                                *yyvsp.offset(-(6 as libc::c_int) as isize),
                                LINT_assign_in_cond,
                            );
                            yyval = mk_for_loop(
                                *yyvsp.offset(-(11 as libc::c_int) as isize),
                                *yyvsp.offset(-(9 as libc::c_int) as isize),
                                *yyvsp.offset(-(6 as libc::c_int) as isize),
                                *yyvsp.offset(-(3 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                            );
                            break_allowed -= 1;
                            break_allowed;
                            continue_allowed -= 1;
                            continue_allowed;
                        }
                        56 => {
                            if !(*yyvsp.offset(-(6 as libc::c_int) as isize)).is_null() {
                                merge_comments(
                                    *yyvsp.offset(-(6 as libc::c_int) as isize),
                                    0 as *mut INSTRUCTION,
                                );
                                let ref mut fresh46 = (**yyvsp
                                    .offset(-(10 as libc::c_int) as isize))
                                    .comment;
                                *fresh46 = *yyvsp.offset(-(6 as libc::c_int) as isize);
                            }
                            if !(*yyvsp.offset(-(4 as libc::c_int) as isize)).is_null() {
                                merge_comments(
                                    *yyvsp.offset(-(4 as libc::c_int) as isize),
                                    0 as *mut INSTRUCTION,
                                );
                                if ((**yyvsp.offset(-(10 as libc::c_int) as isize)).comment)
                                    .is_null()
                                {
                                    (*(**yyvsp.offset(-(4 as libc::c_int) as isize)).d.dn)
                                        .sub
                                        .val
                                        .comtype = FOR_COMMENT;
                                    let ref mut fresh47 = (**yyvsp
                                        .offset(-(10 as libc::c_int) as isize))
                                        .comment;
                                    *fresh47 = *yyvsp.offset(-(4 as libc::c_int) as isize);
                                } else {
                                    let ref mut fresh48 = (*(**yyvsp
                                        .offset(-(10 as libc::c_int) as isize))
                                        .comment)
                                        .comment;
                                    *fresh48 = *yyvsp.offset(-(4 as libc::c_int) as isize);
                                }
                            }
                            if !(*yyvsp.offset(-(1 as libc::c_int) as isize)).is_null() {
                                let ref mut fresh49 = *yyvsp
                                    .offset(0 as libc::c_int as isize);
                                *fresh49 = list_prepend(
                                    *yyvsp.offset(0 as libc::c_int as isize),
                                    *yyvsp.offset(-(1 as libc::c_int) as isize),
                                );
                            }
                            yyval = mk_for_loop(
                                *yyvsp.offset(-(10 as libc::c_int) as isize),
                                *yyvsp.offset(-(8 as libc::c_int) as isize),
                                0 as *mut libc::c_void as *mut INSTRUCTION,
                                *yyvsp.offset(-(3 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                            );
                            break_allowed -= 1;
                            break_allowed;
                            continue_allowed -= 1;
                            continue_allowed;
                        }
                        57 => {
                            if do_flags as libc::c_uint
                                & DO_PRETTY_PRINT as libc::c_int as libc::c_uint != 0
                            {
                                yyval = list_prepend(
                                    *yyvsp.offset(0 as libc::c_int as isize),
                                    bcalloc(Op_exec_count, 1 as libc::c_int, 0 as libc::c_int),
                                );
                            } else {
                                yyval = *yyvsp.offset(0 as libc::c_int as isize);
                            }
                        }
                        58 => {
                            if break_allowed == 0 {
                                error_ln(
                                    (**yyvsp.offset(-(1 as libc::c_int) as isize)).source_line
                                        as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"`break' is not allowed outside a loop or switch\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let ref mut fresh50 = (**yyvsp
                                .offset(-(1 as libc::c_int) as isize))
                                .d
                                .di;
                            *fresh50 = 0 as *mut exp_instruction;
                            yyval = list_create(
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                            );
                            if !(*yyvsp.offset(0 as libc::c_int as isize)).is_null() {
                                yyval = list_append(
                                    yyval,
                                    *yyvsp.offset(0 as libc::c_int as isize),
                                );
                            }
                        }
                        59 => {
                            if continue_allowed == 0 {
                                error_ln(
                                    (**yyvsp.offset(-(1 as libc::c_int) as isize)).source_line
                                        as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"`continue' is not allowed outside a loop\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let ref mut fresh51 = (**yyvsp
                                .offset(-(1 as libc::c_int) as isize))
                                .d
                                .di;
                            *fresh51 = 0 as *mut exp_instruction;
                            yyval = list_create(
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                            );
                            if !(*yyvsp.offset(0 as libc::c_int as isize)).is_null() {
                                yyval = list_append(
                                    yyval,
                                    *yyvsp.offset(0 as libc::c_int as isize),
                                );
                            }
                        }
                        60 => {
                            if rule != 0 && rule != Rule as libc::c_int {
                                error_ln(
                                    (**yyvsp.offset(-(1 as libc::c_int) as isize)).source_line
                                        as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"`next' used in %s action\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    ruletab[rule as usize],
                                );
                            }
                            let ref mut fresh52 = (**yyvsp
                                .offset(-(1 as libc::c_int) as isize))
                                .d
                                .di;
                            *fresh52 = ip_rec;
                            yyval = list_create(
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                            );
                            if !(*yyvsp.offset(0 as libc::c_int as isize)).is_null() {
                                yyval = list_append(
                                    yyval,
                                    *yyvsp.offset(0 as libc::c_int as isize),
                                );
                            }
                        }
                        61 => {
                            if rule == BEGIN as libc::c_int || rule == END as libc::c_int
                                || rule == ENDFILE as libc::c_int
                            {
                                error_ln(
                                    (**yyvsp.offset(-(1 as libc::c_int) as isize)).source_line
                                        as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"`nextfile' used in %s action\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    ruletab[rule as usize],
                                );
                            }
                            let ref mut fresh53 = (**yyvsp
                                .offset(-(1 as libc::c_int) as isize))
                                .d
                                .di;
                            *fresh53 = ip_newfile;
                            let ref mut fresh54 = (**yyvsp
                                .offset(-(1 as libc::c_int) as isize))
                                .x
                                .xi;
                            *fresh54 = ip_endfile;
                            yyval = list_create(
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                            );
                            if !(*yyvsp.offset(0 as libc::c_int as isize)).is_null() {
                                yyval = list_append(
                                    yyval,
                                    *yyvsp.offset(0 as libc::c_int as isize),
                                );
                            }
                        }
                        62 => {
                            let ref mut fresh55 = (**yyvsp
                                .offset(-(2 as libc::c_int) as isize))
                                .d
                                .di;
                            *fresh55 = ip_end;
                            let ref mut fresh56 = (**yyvsp
                                .offset(-(2 as libc::c_int) as isize))
                                .x
                                .xi;
                            *fresh56 = ip_atexit;
                            if (*yyvsp.offset(-(1 as libc::c_int) as isize)).is_null() {
                                yyval = list_create(
                                    *yyvsp.offset(-(2 as libc::c_int) as isize),
                                );
                                list_prepend(
                                    yyval,
                                    bcalloc(Op_push_i, 1 as libc::c_int, 0 as libc::c_int),
                                );
                                (*(*yyval).nexti).d.dn = dupnode(Nnull_string);
                            } else {
                                yyval = list_append(
                                    *yyvsp.offset(-(1 as libc::c_int) as isize),
                                    *yyvsp.offset(-(2 as libc::c_int) as isize),
                                );
                            }
                            if !(*yyvsp.offset(0 as libc::c_int as isize)).is_null() {
                                yyval = list_append(
                                    yyval,
                                    *yyvsp.offset(0 as libc::c_int as isize),
                                );
                            }
                        }
                        63 => {
                            if !in_function {
                                yyerror(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"`return' used outside function context\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                        }
                        64 => {
                            if called_from_eval {
                                (**yyvsp.offset(-(3 as libc::c_int) as isize))
                                    .opcode = Op_K_return_from_eval;
                            }
                            if (*yyvsp.offset(-(1 as libc::c_int) as isize)).is_null() {
                                yyval = list_create(
                                    *yyvsp.offset(-(3 as libc::c_int) as isize),
                                );
                                list_prepend(
                                    yyval,
                                    bcalloc(Op_push_i, 1 as libc::c_int, 0 as libc::c_int),
                                );
                                (*(*yyval).nexti).d.dn = dupnode(Nnull_string);
                            } else {
                                yyval = list_append(
                                    *yyvsp.offset(-(1 as libc::c_int) as isize),
                                    *yyvsp.offset(-(3 as libc::c_int) as isize),
                                );
                            }
                            if !(*yyvsp.offset(0 as libc::c_int as isize)).is_null() {
                                yyval = list_append(
                                    yyval,
                                    *yyvsp.offset(0 as libc::c_int as isize),
                                );
                            }
                        }
                        65 => {
                            if !(*yyvsp.offset(0 as libc::c_int as isize)).is_null() {
                                yyval = list_append(
                                    *yyvsp.offset(-(1 as libc::c_int) as isize),
                                    *yyvsp.offset(0 as libc::c_int as isize),
                                );
                            } else {
                                yyval = *yyvsp.offset(-(1 as libc::c_int) as isize);
                            }
                        }
                        66 => {
                            in_print = 1 as libc::c_int != 0;
                            in_parens = 0 as libc::c_int;
                        }
                        67 => {
                            let mut current_block_629: u64;
                            if do_optimize as libc::c_int != 0
                                && (**yyvsp.offset(-(3 as libc::c_int) as isize)).opcode
                                    as libc::c_uint == Op_K_print as libc::c_int as libc::c_uint
                                && ((*yyvsp.offset(-(1 as libc::c_int) as isize)).is_null()
                                    || (*(**yyvsp.offset(-(1 as libc::c_int) as isize)).d.di)
                                        .opcode as libc::c_uint
                                        == Op_field_spec as libc::c_int as libc::c_uint
                                        && (*(*(**yyvsp.offset(-(1 as libc::c_int) as isize)).nexti)
                                            .nexti)
                                            .nexti
                                            == (**yyvsp.offset(-(1 as libc::c_int) as isize)).d.di
                                        && (*(*(**yyvsp.offset(-(1 as libc::c_int) as isize)).nexti)
                                            .nexti)
                                            .opcode as libc::c_uint
                                            == Op_push_i as libc::c_int as libc::c_uint
                                        && (*(*(*(**yyvsp.offset(-(1 as libc::c_int) as isize))
                                            .nexti)
                                            .nexti)
                                            .d
                                            .dn)
                                            .type_0 as libc::c_uint
                                            == Node_val as libc::c_int as libc::c_uint)
                            {
                                static mut warned: bool = 0 as libc::c_int != 0;
                                if !(*yyvsp.offset(-(1 as libc::c_int) as isize)).is_null()
                                {
                                    let mut n_0: *mut NODE = (*(*(**yyvsp
                                        .offset(-(1 as libc::c_int) as isize))
                                        .nexti)
                                        .nexti)
                                        .d
                                        .dn;
                                    if (*n_0).flags as libc::c_uint
                                        & (STRING as libc::c_int | STRCUR as libc::c_int)
                                            as libc::c_uint != 0 as libc::c_int as libc::c_uint
                                        || !((*n_0).sub.val.fltnum == 0.0f64)
                                    {
                                        current_block_629 = 17994599192713593059;
                                    } else {
                                        bcfree((**yyvsp.offset(-(1 as libc::c_int) as isize)).d.di);
                                        unref(n_0);
                                        bcfree(
                                            (*(**yyvsp.offset(-(1 as libc::c_int) as isize)).nexti)
                                                .nexti,
                                        );
                                        bcfree(
                                            (**yyvsp.offset(-(1 as libc::c_int) as isize)).nexti,
                                        );
                                        bcfree(*yyvsp.offset(-(1 as libc::c_int) as isize));
                                        current_block_629 = 9828823676310104886;
                                    }
                                } else {
                                    if do_flags as libc::c_uint
                                        & (DO_LINT_INVALID as libc::c_int
                                            | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                        && (rule == BEGIN as libc::c_int
                                            || rule == END as libc::c_int) && !warned
                                    {
                                        warned = 1 as libc::c_int != 0;
                                        lintwarn_ln(
                                            (**yyvsp.offset(-(3 as libc::c_int) as isize)).source_line
                                                as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"plain `print' in BEGIN or END rule should probably be `print \"\"'\0"
                                                    as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                    }
                                    current_block_629 = 9828823676310104886;
                                }
                                match current_block_629 {
                                    17994599192713593059 => {}
                                    _ => {
                                        (**yyvsp.offset(-(3 as libc::c_int) as isize))
                                            .x
                                            .xl = 0 as libc::c_int as libc::c_long;
                                        (**yyvsp.offset(-(3 as libc::c_int) as isize))
                                            .opcode = Op_K_print_rec;
                                        if (*yyvsp.offset(0 as libc::c_int as isize)).is_null() {
                                            (**yyvsp.offset(-(3 as libc::c_int) as isize))
                                                .d
                                                .dl = redirect_none as libc::c_int as libc::c_long;
                                            yyval = list_create(
                                                *yyvsp.offset(-(3 as libc::c_int) as isize),
                                            );
                                        } else {
                                            let mut ip_5: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                                            ip_5 = (**yyvsp.offset(0 as libc::c_int as isize)).nexti;
                                            (**yyvsp.offset(-(3 as libc::c_int) as isize))
                                                .d
                                                .dl = (*ip_5).d.dl;
                                            let ref mut fresh57 = (**yyvsp
                                                .offset(0 as libc::c_int as isize))
                                                .nexti;
                                            *fresh57 = (*ip_5).nexti;
                                            bcfree(ip_5);
                                            yyval = list_append(
                                                *yyvsp.offset(0 as libc::c_int as isize),
                                                *yyvsp.offset(-(3 as libc::c_int) as isize),
                                            );
                                        }
                                        current_block_629 = 279356324420966219;
                                    }
                                }
                            } else {
                                current_block_629 = 17994599192713593059;
                            }
                            match current_block_629 {
                                17994599192713593059 => {
                                    if (*yyvsp.offset(0 as libc::c_int as isize)).is_null() {
                                        if (*yyvsp.offset(-(1 as libc::c_int) as isize)).is_null() {
                                            (**yyvsp.offset(-(3 as libc::c_int) as isize))
                                                .x
                                                .xl = 0 as libc::c_int as libc::c_long;
                                            if (**yyvsp.offset(-(3 as libc::c_int) as isize)).opcode
                                                as libc::c_uint == Op_K_print as libc::c_int as libc::c_uint
                                            {
                                                (**yyvsp.offset(-(3 as libc::c_int) as isize))
                                                    .opcode = Op_K_print_rec;
                                            }
                                            (**yyvsp.offset(-(3 as libc::c_int) as isize))
                                                .d
                                                .dl = redirect_none as libc::c_int as libc::c_long;
                                            yyval = list_create(
                                                *yyvsp.offset(-(3 as libc::c_int) as isize),
                                            );
                                        } else {
                                            let mut t: *mut INSTRUCTION = *yyvsp
                                                .offset(-(1 as libc::c_int) as isize);
                                            (**yyvsp.offset(-(3 as libc::c_int) as isize))
                                                .x
                                                .xl = count_expressions(&mut t, 0 as libc::c_int != 0)
                                                as libc::c_long;
                                            (**yyvsp.offset(-(3 as libc::c_int) as isize))
                                                .d
                                                .dl = redirect_none as libc::c_int as libc::c_long;
                                            yyval = list_append(
                                                t,
                                                *yyvsp.offset(-(3 as libc::c_int) as isize),
                                            );
                                        }
                                    } else {
                                        let mut ip_6: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                                        ip_6 = (**yyvsp.offset(0 as libc::c_int as isize)).nexti;
                                        (**yyvsp.offset(-(3 as libc::c_int) as isize))
                                            .d
                                            .dl = (*ip_6).d.dl;
                                        let ref mut fresh58 = (**yyvsp
                                            .offset(0 as libc::c_int as isize))
                                            .nexti;
                                        *fresh58 = (*ip_6).nexti;
                                        bcfree(ip_6);
                                        if (*yyvsp.offset(-(1 as libc::c_int) as isize)).is_null() {
                                            (**yyvsp.offset(-(3 as libc::c_int) as isize))
                                                .x
                                                .xl = 0 as libc::c_int as libc::c_long;
                                            if (**yyvsp.offset(-(3 as libc::c_int) as isize)).opcode
                                                as libc::c_uint == Op_K_print as libc::c_int as libc::c_uint
                                            {
                                                (**yyvsp.offset(-(3 as libc::c_int) as isize))
                                                    .opcode = Op_K_print_rec;
                                            }
                                            yyval = list_append(
                                                *yyvsp.offset(0 as libc::c_int as isize),
                                                *yyvsp.offset(-(3 as libc::c_int) as isize),
                                            );
                                        } else {
                                            let mut t_0: *mut INSTRUCTION = *yyvsp
                                                .offset(-(1 as libc::c_int) as isize);
                                            (**yyvsp.offset(-(3 as libc::c_int) as isize))
                                                .x
                                                .xl = count_expressions(&mut t_0, 0 as libc::c_int != 0)
                                                as libc::c_long;
                                            yyval = list_append(
                                                list_merge(*yyvsp.offset(0 as libc::c_int as isize), t_0),
                                                *yyvsp.offset(-(3 as libc::c_int) as isize),
                                            );
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        68 => {
                            sub_counter = 0 as libc::c_int;
                        }
                        69 => {
                            let mut arr_0: *mut libc::c_char = (**yyvsp
                                .offset(-(2 as libc::c_int) as isize))
                                .d
                                .name;
                            (**yyvsp.offset(-(2 as libc::c_int) as isize))
                                .opcode = Op_push_array;
                            let ref mut fresh59 = (**yyvsp
                                .offset(-(2 as libc::c_int) as isize))
                                .d
                                .dn;
                            *fresh59 = variable(
                                (**yyvsp.offset(-(2 as libc::c_int) as isize)).source_line
                                    as libc::c_int,
                                arr_0,
                                Node_var_new,
                            );
                            if do_flags as libc::c_uint
                                & DO_POSIX as libc::c_int as libc::c_uint == 0
                                && do_flags as libc::c_uint
                                    & DO_TRADITIONAL as libc::c_int as libc::c_uint == 0
                            {
                                if (**yyvsp.offset(-(2 as libc::c_int) as isize)).d.dn
                                    == symbol_table
                                {
                                    (set_loc
                                        as unsafe extern "C" fn(
                                            *const libc::c_char,
                                            libc::c_int,
                                        ) -> ())(
                                        b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                        1255 as libc::c_int,
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
                                            b"`delete' is not allowed with SYMTAB\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                } else if (**yyvsp.offset(-(2 as libc::c_int) as isize))
                                    .d
                                    .dn == func_table
                                {
                                    (set_loc
                                        as unsafe extern "C" fn(
                                            *const libc::c_char,
                                            libc::c_int,
                                        ) -> ())(
                                        b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                        1257 as libc::c_int,
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
                                            b"`delete' is not allowed with FUNCTAB\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                }
                            }
                            if (*yyvsp.offset(0 as libc::c_int as isize)).is_null() {
                                (**yyvsp.offset(-(3 as libc::c_int) as isize))
                                    .x
                                    .xl = 0 as libc::c_int as libc::c_long;
                                yyval = list_append(
                                    list_create(*yyvsp.offset(-(2 as libc::c_int) as isize)),
                                    *yyvsp.offset(-(3 as libc::c_int) as isize),
                                );
                            } else {
                                (**yyvsp.offset(-(3 as libc::c_int) as isize))
                                    .x
                                    .xl = sub_counter as libc::c_long;
                                yyval = list_append(
                                    list_append(
                                        *yyvsp.offset(0 as libc::c_int as isize),
                                        *yyvsp.offset(-(2 as libc::c_int) as isize),
                                    ),
                                    *yyvsp.offset(-(3 as libc::c_int) as isize),
                                );
                            }
                        }
                        70 => {
                            static mut warned_0: bool = 0 as libc::c_int != 0;
                            let mut arr_1: *mut libc::c_char = (**yyvsp
                                .offset(-(1 as libc::c_int) as isize))
                                .d
                                .name;
                            if do_flags as libc::c_uint
                                & (DO_LINT_INVALID as libc::c_int
                                    | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                && !warned_0
                            {
                                warned_0 = 1 as libc::c_int != 0;
                                lintwarn_ln(
                                    (**yyvsp.offset(-(3 as libc::c_int) as isize)).source_line
                                        as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"`delete(array)' is a non-portable tawk extension\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            if do_flags as libc::c_uint
                                & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0
                            {
                                error_ln(
                                    (**yyvsp.offset(-(3 as libc::c_int) as isize)).source_line
                                        as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"`delete(array)' is a non-portable tawk extension\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let ref mut fresh60 = (**yyvsp
                                .offset(-(1 as libc::c_int) as isize))
                                .d
                                .dn;
                            *fresh60 = variable(
                                (**yyvsp.offset(-(1 as libc::c_int) as isize)).source_line
                                    as libc::c_int,
                                arr_1,
                                Node_var_new,
                            );
                            (**yyvsp.offset(-(1 as libc::c_int) as isize))
                                .opcode = Op_push_array;
                            (**yyvsp.offset(-(3 as libc::c_int) as isize))
                                .x
                                .xl = 0 as libc::c_int as libc::c_long;
                            yyval = list_append(
                                list_create(*yyvsp.offset(-(1 as libc::c_int) as isize)),
                                *yyvsp.offset(-(3 as libc::c_int) as isize),
                            );
                            if do_flags as libc::c_uint
                                & DO_POSIX as libc::c_int as libc::c_uint == 0
                                && do_flags as libc::c_uint
                                    & DO_TRADITIONAL as libc::c_int as libc::c_uint == 0
                            {
                                if (**yyvsp.offset(-(1 as libc::c_int) as isize)).d.dn
                                    == symbol_table
                                {
                                    (set_loc
                                        as unsafe extern "C" fn(
                                            *const libc::c_char,
                                            libc::c_int,
                                        ) -> ())(
                                        b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                        1304 as libc::c_int,
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
                                            b"`delete' is not allowed with SYMTAB\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                } else if (**yyvsp.offset(-(1 as libc::c_int) as isize))
                                    .d
                                    .dn == func_table
                                {
                                    (set_loc
                                        as unsafe extern "C" fn(
                                            *const libc::c_char,
                                            libc::c_int,
                                        ) -> ())(
                                        b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                        1306 as libc::c_int,
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
                                            b"`delete' is not allowed with FUNCTAB\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                }
                            }
                        }
                        71 => {
                            yyval = optimize_assignment(
                                *yyvsp.offset(0 as libc::c_int as isize),
                            );
                        }
                        72 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        73 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        74 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        75 => {
                            if (*yyvsp.offset(-(1 as libc::c_int) as isize)).is_null() {
                                yyval = list_create(
                                    *yyvsp.offset(0 as libc::c_int as isize),
                                );
                            } else {
                                yyval = list_prepend(
                                    *yyvsp.offset(-(1 as libc::c_int) as isize),
                                    *yyvsp.offset(0 as libc::c_int as isize),
                                );
                            }
                        }
                        76 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        77 => {
                            let mut casestmt_0: *mut INSTRUCTION = *yyvsp
                                .offset(0 as libc::c_int as isize);
                            if (*yyvsp.offset(0 as libc::c_int as isize)).is_null() {
                                casestmt_0 = list_create(
                                    bcalloc(Op_no_op, 1 as libc::c_int, 0 as libc::c_int),
                                );
                            }
                            if do_flags as libc::c_uint
                                & DO_PRETTY_PRINT as libc::c_int as libc::c_uint != 0
                            {
                                list_prepend(
                                    casestmt_0,
                                    bcalloc(Op_exec_count, 1 as libc::c_int, 0 as libc::c_int),
                                );
                            }
                            let ref mut fresh61 = (**yyvsp
                                .offset(-(4 as libc::c_int) as isize))
                                .d
                                .di;
                            *fresh61 = *yyvsp.offset(-(3 as libc::c_int) as isize);
                            let ref mut fresh62 = (**yyvsp
                                .offset(-(4 as libc::c_int) as isize))
                                .x
                                .xi;
                            *fresh62 = casestmt_0;
                            let ref mut fresh63 = (**yyvsp
                                .offset(-(4 as libc::c_int) as isize))
                                .comment;
                            *fresh63 = *yyvsp.offset(-(1 as libc::c_int) as isize);
                            bcfree(*yyvsp.offset(-(2 as libc::c_int) as isize));
                            yyval = *yyvsp.offset(-(4 as libc::c_int) as isize);
                        }
                        78 => {
                            let mut casestmt_1: *mut INSTRUCTION = *yyvsp
                                .offset(0 as libc::c_int as isize);
                            if (*yyvsp.offset(0 as libc::c_int as isize)).is_null() {
                                casestmt_1 = list_create(
                                    bcalloc(Op_no_op, 1 as libc::c_int, 0 as libc::c_int),
                                );
                            }
                            if do_flags as libc::c_uint
                                & DO_PRETTY_PRINT as libc::c_int as libc::c_uint != 0
                            {
                                list_prepend(
                                    casestmt_1,
                                    bcalloc(Op_exec_count, 1 as libc::c_int, 0 as libc::c_int),
                                );
                            }
                            bcfree(*yyvsp.offset(-(2 as libc::c_int) as isize));
                            let ref mut fresh64 = (**yyvsp
                                .offset(-(3 as libc::c_int) as isize))
                                .x
                                .xi;
                            *fresh64 = casestmt_1;
                            let ref mut fresh65 = (**yyvsp
                                .offset(-(3 as libc::c_int) as isize))
                                .comment;
                            *fresh65 = *yyvsp.offset(-(1 as libc::c_int) as isize);
                            yyval = *yyvsp.offset(-(3 as libc::c_int) as isize);
                        }
                        79 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        80 => {
                            let mut n_1: *mut NODE = (**yyvsp
                                .offset(0 as libc::c_int as isize))
                                .d
                                .dn;
                            force_number(n_1);
                            negate_num(n_1);
                            bcfree(*yyvsp.offset(-(1 as libc::c_int) as isize));
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        81 => {
                            let mut n_2: *mut NODE = (*(**yyvsp
                                .offset(0 as libc::c_int as isize))
                                .d
                                .di)
                                .d
                                .dn;
                            bcfree(*yyvsp.offset(-(1 as libc::c_int) as isize));
                            add_sign_to_num(n_2, '+' as i32 as libc::c_char);
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        82 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        83 => {
                            if (*(**yyvsp.offset(0 as libc::c_int as isize)).d.dn).type_0
                                as libc::c_uint == Node_regex as libc::c_int as libc::c_uint
                            {
                                (**yyvsp.offset(0 as libc::c_int as isize))
                                    .opcode = Op_push_re;
                            } else {
                                (**yyvsp.offset(0 as libc::c_int as isize))
                                    .opcode = Op_push;
                            }
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        84 => {
                            (**yyvsp.offset(0 as libc::c_int as isize))
                                .opcode = Op_push_re;
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        85 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        86 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        88 => {
                            yyval = *yyvsp.offset(-(1 as libc::c_int) as isize);
                        }
                        89 => {
                            in_print = 0 as libc::c_int != 0;
                            in_parens = 0 as libc::c_int;
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        90 => {
                            in_print = 0 as libc::c_int != 0;
                            in_parens = 0 as libc::c_int;
                        }
                        91 => {
                            if (**yyvsp.offset(-(2 as libc::c_int) as isize)).d.dl
                                == redirect_twoway as libc::c_int as libc::c_long
                                && (*(**yyvsp.offset(0 as libc::c_int as isize)).d.di)
                                    .opcode as libc::c_uint
                                    == Op_K_getline_redir as libc::c_int as libc::c_uint
                                && (*(**yyvsp.offset(0 as libc::c_int as isize)).d.di).d.dl
                                    == redirect_twoway as libc::c_int as libc::c_long
                            {
                                yyerror(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"multistage two-way pipelines don't work\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            if do_flags as libc::c_uint
                                & (DO_LINT_INVALID as libc::c_int
                                    | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                && (**yyvsp.offset(-(2 as libc::c_int) as isize)).d.dl
                                    == redirect_output as libc::c_int as libc::c_long
                                && (*(**yyvsp.offset(0 as libc::c_int as isize)).d.di)
                                    .opcode as libc::c_uint
                                    == Op_concat as libc::c_int as libc::c_uint
                            {
                                (set_loc
                                    as unsafe extern "C" fn(
                                        *const libc::c_char,
                                        libc::c_int,
                                    ) -> ())(
                                    b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                    1433 as libc::c_int,
                                );
                                (Some(lintfunc.unwrap()))
                                    .unwrap()(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"concatenation as I/O `>' redirection target is ambiguous\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            yyval = list_prepend(
                                *yyvsp.offset(0 as libc::c_int as isize),
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                            );
                        }
                        92 => {
                            if !(*yyvsp.offset(-(1 as libc::c_int) as isize)).is_null() {
                                let ref mut fresh66 = (**yyvsp
                                    .offset(-(5 as libc::c_int) as isize))
                                    .comment;
                                *fresh66 = *yyvsp.offset(-(1 as libc::c_int) as isize);
                            }
                            add_lint(
                                *yyvsp.offset(-(3 as libc::c_int) as isize),
                                LINT_assign_in_cond,
                            );
                            yyval = mk_condition(
                                *yyvsp.offset(-(3 as libc::c_int) as isize),
                                *yyvsp.offset(-(5 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                                0 as *mut INSTRUCTION,
                                0 as *mut INSTRUCTION,
                            );
                        }
                        93 => {
                            if !(*yyvsp.offset(-(4 as libc::c_int) as isize)).is_null() {
                                let ref mut fresh67 = (**yyvsp
                                    .offset(-(8 as libc::c_int) as isize))
                                    .comment;
                                *fresh67 = *yyvsp.offset(-(4 as libc::c_int) as isize);
                            }
                            if !(*yyvsp.offset(-(1 as libc::c_int) as isize)).is_null() {
                                let ref mut fresh68 = (**yyvsp
                                    .offset(-(2 as libc::c_int) as isize))
                                    .comment;
                                *fresh68 = *yyvsp.offset(-(1 as libc::c_int) as isize);
                            }
                            add_lint(
                                *yyvsp.offset(-(6 as libc::c_int) as isize),
                                LINT_assign_in_cond,
                            );
                            yyval = mk_condition(
                                *yyvsp.offset(-(6 as libc::c_int) as isize),
                                *yyvsp.offset(-(8 as libc::c_int) as isize),
                                *yyvsp.offset(-(3 as libc::c_int) as isize),
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                            );
                        }
                        94 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        95 => {
                            if !(*yyvsp.offset(-(1 as libc::c_int) as isize)).is_null()
                                && !(*yyvsp.offset(0 as libc::c_int as isize)).is_null()
                            {
                                if (*(**yyvsp.offset(-(1 as libc::c_int) as isize)).d.dn)
                                    .sub
                                    .val
                                    .comtype as libc::c_uint
                                    == EOL_COMMENT as libc::c_int as libc::c_uint
                                {
                                    let ref mut fresh69 = (**yyvsp
                                        .offset(-(1 as libc::c_int) as isize))
                                        .comment;
                                    *fresh69 = *yyvsp.offset(0 as libc::c_int as isize);
                                } else {
                                    merge_comments(
                                        *yyvsp.offset(-(1 as libc::c_int) as isize),
                                        *yyvsp.offset(0 as libc::c_int as isize),
                                    );
                                }
                                yyval = *yyvsp.offset(-(1 as libc::c_int) as isize);
                            } else if !(*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .is_null()
                            {
                                yyval = *yyvsp.offset(-(1 as libc::c_int) as isize);
                            } else if !(*yyvsp.offset(0 as libc::c_int as isize))
                                .is_null()
                            {
                                yyval = *yyvsp.offset(0 as libc::c_int as isize);
                            } else {
                                yyval = 0 as *mut INSTRUCTION;
                            }
                        }
                        96 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        97 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        98 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        99 => {
                            bcfree(*yyvsp.offset(-(1 as libc::c_int) as isize));
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        100 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        101 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        102 => {
                            (**yyvsp.offset(0 as libc::c_int as isize))
                                .x
                                .xl = 0 as libc::c_int as libc::c_long;
                            yyval = list_create(
                                *yyvsp.offset(0 as libc::c_int as isize),
                            );
                        }
                        103 => {
                            if !(*yyvsp.offset(-(2 as libc::c_int) as isize)).is_null()
                                && !(*yyvsp.offset(0 as libc::c_int as isize)).is_null()
                            {
                                (**yyvsp.offset(0 as libc::c_int as isize))
                                    .x
                                    .xl = (*(**yyvsp.offset(-(2 as libc::c_int) as isize)).d.di)
                                    .x
                                    .xl + 1 as libc::c_int as libc::c_long;
                                yyval = list_append(
                                    *yyvsp.offset(-(2 as libc::c_int) as isize),
                                    *yyvsp.offset(0 as libc::c_int as isize),
                                );
                                yyerrstatus = 0 as libc::c_int;
                                if !(*yyvsp.offset(-(1 as libc::c_int) as isize)).is_null()
                                {
                                    if !((**yyvsp.offset(-(2 as libc::c_int) as isize)).comment)
                                        .is_null()
                                    {
                                        merge_comments(
                                            (**yyvsp.offset(-(2 as libc::c_int) as isize)).comment,
                                            *yyvsp.offset(-(1 as libc::c_int) as isize),
                                        );
                                    } else {
                                        let ref mut fresh70 = (**yyvsp
                                            .offset(-(2 as libc::c_int) as isize))
                                            .comment;
                                        *fresh70 = *yyvsp.offset(-(1 as libc::c_int) as isize);
                                    }
                                }
                            } else {
                                yyval = 0 as *mut INSTRUCTION;
                            }
                        }
                        104 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        105 => {
                            yyval = *yyvsp.offset(-(1 as libc::c_int) as isize);
                        }
                        106 => {
                            yyval = *yyvsp.offset(-(2 as libc::c_int) as isize);
                        }
                        107 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        108 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        109 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        110 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        111 => {
                            yyval = mk_expression_list(
                                0 as *mut INSTRUCTION,
                                *yyvsp.offset(0 as libc::c_int as isize),
                            );
                        }
                        112 => {
                            if !(*yyvsp.offset(-(1 as libc::c_int) as isize)).is_null() {
                                let ref mut fresh71 = (*(**yyvsp
                                    .offset(-(2 as libc::c_int) as isize))
                                    .d
                                    .di)
                                    .comment;
                                *fresh71 = *yyvsp.offset(-(1 as libc::c_int) as isize);
                            }
                            yyval = mk_expression_list(
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                            );
                            yyerrstatus = 0 as libc::c_int;
                        }
                        113 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        114 => {
                            yyval = *yyvsp.offset(-(1 as libc::c_int) as isize);
                        }
                        115 => {
                            yyval = mk_expression_list(
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                            );
                        }
                        116 => {
                            if !(*yyvsp.offset(-(1 as libc::c_int) as isize)).is_null() {
                                let ref mut fresh72 = (*(**yyvsp
                                    .offset(-(2 as libc::c_int) as isize))
                                    .d
                                    .di)
                                    .comment;
                                *fresh72 = *yyvsp.offset(-(1 as libc::c_int) as isize);
                            }
                            yyval = *yyvsp.offset(-(2 as libc::c_int) as isize);
                        }
                        117 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        118 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        119 => {
                            yyval = mk_expression_list(
                                0 as *mut INSTRUCTION,
                                *yyvsp.offset(0 as libc::c_int as isize),
                            );
                        }
                        120 => {
                            if !(*yyvsp.offset(-(1 as libc::c_int) as isize)).is_null() {
                                let ref mut fresh73 = (*(**yyvsp
                                    .offset(-(2 as libc::c_int) as isize))
                                    .d
                                    .di)
                                    .comment;
                                *fresh73 = *yyvsp.offset(-(1 as libc::c_int) as isize);
                            }
                            yyval = mk_expression_list(
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                            );
                            yyerrstatus = 0 as libc::c_int;
                        }
                        121 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        122 => {
                            yyval = *yyvsp.offset(-(1 as libc::c_int) as isize);
                        }
                        123 => {
                            yyval = mk_expression_list(
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                            );
                        }
                        124 => {
                            if !(*yyvsp.offset(-(1 as libc::c_int) as isize)).is_null() {
                                let ref mut fresh74 = (**yyvsp
                                    .offset(-(2 as libc::c_int) as isize))
                                    .comment;
                                *fresh74 = *yyvsp.offset(-(1 as libc::c_int) as isize);
                            }
                            yyval = *yyvsp.offset(-(2 as libc::c_int) as isize);
                        }
                        125 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        126 => {
                            yyval = list_create(
                                *yyvsp.offset(0 as libc::c_int as isize),
                            );
                        }
                        127 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        128 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        129 => {
                            if do_flags as libc::c_uint
                                & (DO_LINT_INVALID as libc::c_int
                                    | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                && (*(**yyvsp.offset(0 as libc::c_int as isize)).d.di)
                                    .opcode as libc::c_uint
                                    == Op_match_rec as libc::c_int as libc::c_uint
                            {
                                lintwarn_ln(
                                    (**yyvsp.offset(-(1 as libc::c_int) as isize)).source_line
                                        as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"regular expression on right of assignment\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            yyval = mk_assignment(
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                            );
                        }
                        130 => {
                            yyval = mk_assignment(
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                                list_create(*yyvsp.offset(0 as libc::c_int as isize)),
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                            );
                        }
                        131 => {
                            yyval = mk_boolean(
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                            );
                        }
                        132 => {
                            yyval = mk_boolean(
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                            );
                        }
                        133 => {
                            if (*(**yyvsp.offset(-(2 as libc::c_int) as isize)).d.di)
                                .opcode as libc::c_uint
                                == Op_match_rec as libc::c_int as libc::c_uint
                            {
                                warning_ln(
                                    (**yyvsp.offset(-(1 as libc::c_int) as isize)).source_line
                                        as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"regular expression on left of `~' or `!~' operator\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            let ref mut fresh75 = (**yyvsp
                                .offset(-(1 as libc::c_int) as isize))
                                .d
                                .dn;
                            *fresh75 = (**yyvsp.offset(0 as libc::c_int as isize)).d.dn;
                            bcfree(*yyvsp.offset(0 as libc::c_int as isize));
                            yyval = list_append(
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                            );
                        }
                        134 => {
                            if (*(**yyvsp.offset(-(2 as libc::c_int) as isize)).d.di)
                                .opcode as libc::c_uint
                                == Op_match_rec as libc::c_int as libc::c_uint
                            {
                                warning_ln(
                                    (**yyvsp.offset(-(1 as libc::c_int) as isize)).source_line
                                        as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"regular expression on left of `~' or `!~' operator\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            if (**yyvsp.offset(0 as libc::c_int as isize)).d.di
                                == (**yyvsp.offset(0 as libc::c_int as isize)).nexti
                                && (*(**yyvsp.offset(0 as libc::c_int as isize)).nexti)
                                    .opcode as libc::c_uint
                                    == Op_match_rec as libc::c_int as libc::c_uint
                            {
                                let ref mut fresh76 = (**yyvsp
                                    .offset(-(1 as libc::c_int) as isize))
                                    .d
                                    .dn;
                                *fresh76 = (*(**yyvsp.offset(0 as libc::c_int as isize))
                                    .nexti)
                                    .d
                                    .dn;
                                bcfree((**yyvsp.offset(0 as libc::c_int as isize)).nexti);
                                bcfree(*yyvsp.offset(0 as libc::c_int as isize));
                                yyval = list_append(
                                    *yyvsp.offset(-(2 as libc::c_int) as isize),
                                    *yyvsp.offset(-(1 as libc::c_int) as isize),
                                );
                            } else {
                                let ref mut fresh77 = (**yyvsp
                                    .offset(-(1 as libc::c_int) as isize))
                                    .d
                                    .dn;
                                *fresh77 = make_regnode(Node_dynregex, 0 as *mut NODE);
                                yyval = list_append(
                                    list_merge(
                                        *yyvsp.offset(-(2 as libc::c_int) as isize),
                                        *yyvsp.offset(0 as libc::c_int as isize),
                                    ),
                                    *yyvsp.offset(-(1 as libc::c_int) as isize),
                                );
                            }
                        }
                        135 => {
                            if do_flags as libc::c_uint
                                & DO_LINT_OLD as libc::c_int as libc::c_uint != 0
                            {
                                lintwarn_ln(
                                    (**yyvsp.offset(-(1 as libc::c_int) as isize)).source_line
                                        as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"old awk does not support the keyword `in' except after `for'\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            (*(**yyvsp.offset(0 as libc::c_int as isize)).nexti)
                                .opcode = Op_push_array;
                            (**yyvsp.offset(-(1 as libc::c_int) as isize))
                                .opcode = Op_in_array;
                            (**yyvsp.offset(-(1 as libc::c_int) as isize))
                                .x
                                .xl = 1 as libc::c_int as libc::c_long;
                            yyval = list_append(
                                list_merge(
                                    *yyvsp.offset(-(2 as libc::c_int) as isize),
                                    *yyvsp.offset(0 as libc::c_int as isize),
                                ),
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                            );
                        }
                        136 => {
                            if do_flags as libc::c_uint
                                & (DO_LINT_INVALID as libc::c_int
                                    | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                && (*(**yyvsp.offset(0 as libc::c_int as isize)).d.di)
                                    .opcode as libc::c_uint
                                    == Op_match_rec as libc::c_int as libc::c_uint
                            {
                                lintwarn_ln(
                                    (**yyvsp.offset(-(1 as libc::c_int) as isize)).source_line
                                        as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"regular expression on right of comparison\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            yyval = list_append(
                                list_merge(
                                    *yyvsp.offset(-(2 as libc::c_int) as isize),
                                    *yyvsp.offset(0 as libc::c_int as isize),
                                ),
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                            );
                        }
                        137 => {
                            yyval = mk_condition(
                                *yyvsp.offset(-(4 as libc::c_int) as isize),
                                *yyvsp.offset(-(3 as libc::c_int) as isize),
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                            );
                        }
                        138 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        139 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        140 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        141 => {
                            (**yyvsp.offset(0 as libc::c_int as isize))
                                .opcode = Op_assign_quotient;
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        142 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        143 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        144 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        145 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        146 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        147 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        148 => {
                            let mut count: libc::c_int = 2 as libc::c_int;
                            let mut is_simple_var: bool = 0 as libc::c_int != 0;
                            if (*(**yyvsp.offset(-(1 as libc::c_int) as isize)).d.di)
                                .opcode as libc::c_uint
                                == Op_concat as libc::c_int as libc::c_uint
                            {
                                is_simple_var = (*(**yyvsp
                                    .offset(-(1 as libc::c_int) as isize))
                                    .d
                                    .di)
                                    .d
                                    .dl & 2 as libc::c_int as libc::c_long
                                    != 0 as libc::c_int as libc::c_long;
                                count = ((*(**yyvsp.offset(-(1 as libc::c_int) as isize))
                                    .d
                                    .di)
                                    .x
                                    .xl + 1 as libc::c_int as libc::c_long) as libc::c_int;
                                (*(**yyvsp.offset(-(1 as libc::c_int) as isize)).d.di)
                                    .opcode = Op_no_op;
                            } else {
                                is_simple_var = (*(**yyvsp
                                    .offset(-(1 as libc::c_int) as isize))
                                    .nexti)
                                    .opcode as libc::c_uint
                                    == Op_push as libc::c_int as libc::c_uint
                                    && (**yyvsp.offset(-(1 as libc::c_int) as isize)).d.di
                                        == (**yyvsp.offset(-(1 as libc::c_int) as isize)).nexti;
                            }
                            let mut current_block_948: u64;
                            if do_optimize as libc::c_int != 0
                                && (**yyvsp.offset(-(1 as libc::c_int) as isize)).nexti
                                    == (**yyvsp.offset(-(1 as libc::c_int) as isize)).d.di
                                && (*(**yyvsp.offset(-(1 as libc::c_int) as isize)).nexti)
                                    .opcode as libc::c_uint
                                    == Op_push_i as libc::c_int as libc::c_uint
                                && (**yyvsp.offset(0 as libc::c_int as isize)).nexti
                                    == (**yyvsp.offset(0 as libc::c_int as isize)).d.di
                                && (*(**yyvsp.offset(0 as libc::c_int as isize)).nexti)
                                    .opcode as libc::c_uint
                                    == Op_push_i as libc::c_int as libc::c_uint
                            {
                                let mut n1: *mut NODE = (*(**yyvsp
                                    .offset(-(1 as libc::c_int) as isize))
                                    .nexti)
                                    .d
                                    .dn;
                                let mut n2: *mut NODE = (*(**yyvsp
                                    .offset(0 as libc::c_int as isize))
                                    .nexti)
                                    .d
                                    .dn;
                                let mut nlen: size_t = 0;
                                if (*n1).flags as libc::c_uint
                                    & (NUMBER as libc::c_int | NUMINT as libc::c_int
                                        | INTLSTR as libc::c_int) as libc::c_uint
                                    != 0 as libc::c_int as libc::c_uint
                                    || (*n2).flags as libc::c_uint
                                        & (NUMBER as libc::c_int | NUMINT as libc::c_int
                                            | INTLSTR as libc::c_int) as libc::c_uint
                                        != 0 as libc::c_int as libc::c_uint
                                {
                                    current_block_948 = 11571146449651676854;
                                } else {
                                    n1 = force_string_fmt(n1, CONVFMT, CONVFMTidx);
                                    n2 = force_string_fmt(n2, CONVFMT, CONVFMTidx);
                                    nlen = ((*n1).sub.val.slen)
                                        .wrapping_add((*n2).sub.val.slen);
                                    (*n1)
                                        .sub
                                        .val
                                        .sp = erealloc_real(
                                        (*n1).sub.val.sp as *mut libc::c_void,
                                        nlen.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                        b"constant fold\0" as *const u8 as *const libc::c_char,
                                        b"n1->stptr\0" as *const u8 as *const libc::c_char,
                                        b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                        1775 as libc::c_int,
                                    ) as *mut libc::c_char;
                                    memcpy(
                                        ((*n1).sub.val.sp).offset((*n1).sub.val.slen as isize)
                                            as *mut libc::c_void,
                                        (*n2).sub.val.sp as *const libc::c_void,
                                        (*n2).sub.val.slen,
                                    );
                                    (*n1).sub.val.slen = nlen;
                                    *((*n1).sub.val.sp)
                                        .offset(nlen as isize) = '\0' as i32 as libc::c_char;
                                    (*n1)
                                        .flags = ::std::mem::transmute::<
                                        libc::c_uint,
                                        flagvals,
                                    >(
                                        (*n1).flags as libc::c_uint
                                            & !(NUMCUR as libc::c_int | NUMBER as libc::c_int
                                                | NUMINT as libc::c_int) as libc::c_uint,
                                    );
                                    (*n1)
                                        .flags = ::std::mem::transmute::<
                                        libc::c_uint,
                                        flagvals,
                                    >(
                                        (*n1).flags as libc::c_uint
                                            | (STRING as libc::c_int | STRCUR as libc::c_int)
                                                as libc::c_uint,
                                    );
                                    unref(n2);
                                    bcfree((**yyvsp.offset(0 as libc::c_int as isize)).nexti);
                                    bcfree(*yyvsp.offset(0 as libc::c_int as isize));
                                    yyval = *yyvsp.offset(-(1 as libc::c_int) as isize);
                                    current_block_948 = 2403495654184706625;
                                }
                            } else {
                                current_block_948 = 11571146449651676854;
                            }
                            match current_block_948 {
                                11571146449651676854 => {
                                    yyval = list_append(
                                        list_merge(
                                            *yyvsp.offset(-(1 as libc::c_int) as isize),
                                            *yyvsp.offset(0 as libc::c_int as isize),
                                        ),
                                        bcalloc(Op_concat, 1 as libc::c_int, 0 as libc::c_int),
                                    );
                                    (*(*yyval).d.di)
                                        .d
                                        .dl = (if is_simple_var as libc::c_int != 0 {
                                        2 as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    }) as libc::c_long;
                                    (*(*yyval).d.di).x.xl = count as libc::c_long;
                                    if count > max_args {
                                        max_args = count;
                                    }
                                }
                                _ => {}
                            }
                        }
                        150 => {
                            yyval = mk_binary(
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                            );
                        }
                        151 => {
                            yyval = mk_binary(
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                            );
                        }
                        152 => {
                            yyval = mk_binary(
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                            );
                        }
                        153 => {
                            yyval = mk_binary(
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                            );
                        }
                        154 => {
                            yyval = mk_binary(
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                            );
                        }
                        155 => {
                            yyval = mk_binary(
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                            );
                        }
                        156 => {
                            if (rule == BEGINFILE as libc::c_int
                                || rule == ENDFILE as libc::c_int)
                                && (*yyvsp.offset(0 as libc::c_int as isize)).is_null()
                            {
                                error_ln(
                                    (**yyvsp.offset(-(2 as libc::c_int) as isize)).source_line
                                        as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"non-redirected `getline' invalid inside `%s' rule\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    ruletab[rule as usize],
                                );
                            }
                            if do_flags as libc::c_uint
                                & (DO_LINT_INVALID as libc::c_int
                                    | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                && rule == END as libc::c_int
                                && (*yyvsp.offset(0 as libc::c_int as isize)).is_null()
                            {
                                lintwarn_ln(
                                    (**yyvsp.offset(-(2 as libc::c_int) as isize)).source_line
                                        as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"non-redirected `getline' undefined inside END action\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            yyval = mk_getline(
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                                redirect_input as libc::c_int,
                            );
                        }
                        157 => {
                            (**yyvsp.offset(0 as libc::c_int as isize))
                                .opcode = Op_postincrement;
                            yyval = mk_assignment(
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                                0 as *mut INSTRUCTION,
                                *yyvsp.offset(0 as libc::c_int as isize),
                            );
                        }
                        158 => {
                            (**yyvsp.offset(0 as libc::c_int as isize))
                                .opcode = Op_postdecrement;
                            yyval = mk_assignment(
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                                0 as *mut INSTRUCTION,
                                *yyvsp.offset(0 as libc::c_int as isize),
                            );
                        }
                        159 => {
                            if do_flags as libc::c_uint
                                & DO_LINT_OLD as libc::c_int as libc::c_uint != 0
                            {
                                warning_ln(
                                    (**yyvsp.offset(-(1 as libc::c_int) as isize)).source_line
                                        as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"old awk does not support the keyword `in' except after `for'\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                lintwarn_ln(
                                    (**yyvsp.offset(-(1 as libc::c_int) as isize)).source_line
                                        as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"old awk does not support multidimensional arrays\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            (*(**yyvsp.offset(0 as libc::c_int as isize)).nexti)
                                .opcode = Op_push_array;
                            (**yyvsp.offset(-(1 as libc::c_int) as isize))
                                .opcode = Op_in_array;
                            if (*yyvsp.offset(-(3 as libc::c_int) as isize)).is_null() {
                                errcount += 1;
                                errcount;
                                (**yyvsp.offset(-(1 as libc::c_int) as isize))
                                    .x
                                    .xl = 0 as libc::c_int as libc::c_long;
                                yyval = list_merge(
                                    *yyvsp.offset(0 as libc::c_int as isize),
                                    *yyvsp.offset(-(1 as libc::c_int) as isize),
                                );
                            } else {
                                let mut t_1: *mut INSTRUCTION = *yyvsp
                                    .offset(-(3 as libc::c_int) as isize);
                                (**yyvsp.offset(-(1 as libc::c_int) as isize))
                                    .x
                                    .xl = count_expressions(&mut t_1, 0 as libc::c_int != 0)
                                    as libc::c_long;
                                yyval = list_append(
                                    list_merge(t_1, *yyvsp.offset(0 as libc::c_int as isize)),
                                    *yyvsp.offset(-(1 as libc::c_int) as isize),
                                );
                            }
                        }
                        160 => {
                            yyval = mk_getline(
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                                *yyvsp.offset(-(3 as libc::c_int) as isize),
                                (**yyvsp.offset(-(2 as libc::c_int) as isize)).d.dl
                                    as libc::c_int,
                            );
                            bcfree(*yyvsp.offset(-(2 as libc::c_int) as isize));
                        }
                        161 => {
                            yyval = mk_binary(
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                            );
                        }
                        162 => {
                            yyval = mk_binary(
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                            );
                        }
                        163 => {
                            yyval = mk_binary(
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                            );
                        }
                        164 => {
                            yyval = mk_binary(
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                            );
                        }
                        165 => {
                            yyval = mk_binary(
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                            );
                        }
                        166 => {
                            yyval = mk_binary(
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                            );
                        }
                        167 => {
                            yyval = list_create(
                                *yyvsp.offset(0 as libc::c_int as isize),
                            );
                        }
                        168 => {
                            if (**yyvsp.offset(0 as libc::c_int as isize)).opcode
                                as libc::c_uint
                                == Op_match_rec as libc::c_int as libc::c_uint
                            {
                                (**yyvsp.offset(0 as libc::c_int as isize))
                                    .opcode = Op_nomatch;
                                (**yyvsp.offset(-(1 as libc::c_int) as isize))
                                    .opcode = Op_push_i;
                                let ref mut fresh78 = (**yyvsp
                                    .offset(-(1 as libc::c_int) as isize))
                                    .d
                                    .dn;
                                *fresh78 = set_profile_text(
                                    make_number.unwrap()(0.0f64),
                                    b"0\0" as *const u8 as *const libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                                yyval = list_append(
                                    list_append(
                                        list_create(*yyvsp.offset(-(1 as libc::c_int) as isize)),
                                        bcalloc(Op_field_spec, 1 as libc::c_int, 0 as libc::c_int),
                                    ),
                                    *yyvsp.offset(0 as libc::c_int as isize),
                                );
                            } else if do_optimize as libc::c_int != 0
                                && (**yyvsp.offset(0 as libc::c_int as isize)).nexti
                                    == (**yyvsp.offset(0 as libc::c_int as isize)).d.di
                                && (*(**yyvsp.offset(0 as libc::c_int as isize)).nexti)
                                    .opcode as libc::c_uint
                                    == Op_push_i as libc::c_int as libc::c_uint
                                && (*(*(**yyvsp.offset(0 as libc::c_int as isize)).nexti)
                                    .d
                                    .dn)
                                    .flags as libc::c_uint
                                    & (MPFN as libc::c_int | MPZN as libc::c_int
                                        | INTLSTR as libc::c_int) as libc::c_uint
                                    == 0 as libc::c_int as libc::c_uint
                            {
                                let mut n_3: *mut NODE = (*(**yyvsp
                                    .offset(0 as libc::c_int as isize))
                                    .nexti)
                                    .d
                                    .dn;
                                if (*n_3).flags as libc::c_uint
                                    & STRING as libc::c_int as libc::c_uint
                                    != 0 as libc::c_int as libc::c_uint
                                {
                                    (*n_3)
                                        .sub
                                        .val
                                        .fltnum = ((*n_3).sub.val.slen
                                        == 0 as libc::c_int as libc::c_ulong) as libc::c_int
                                        as libc::c_double;
                                    (*n_3)
                                        .flags = ::std::mem::transmute::<
                                        libc::c_uint,
                                        flagvals,
                                    >(
                                        (*n_3).flags as libc::c_uint
                                            & !(STRCUR as libc::c_int | STRING as libc::c_int)
                                                as libc::c_uint,
                                    );
                                    (*n_3)
                                        .flags = ::std::mem::transmute::<
                                        libc::c_uint,
                                        flagvals,
                                    >(
                                        (*n_3).flags as libc::c_uint
                                            | (NUMCUR as libc::c_int | NUMBER as libc::c_int)
                                                as libc::c_uint,
                                    );
                                    free((*n_3).sub.val.sp as *mut libc::c_void);
                                    (*n_3).sub.val.sp = 0 as *mut libc::c_char;
                                    (*n_3).sub.val.slen = 0 as libc::c_int as size_t;
                                } else {
                                    (*n_3)
                                        .sub
                                        .val
                                        .fltnum = ((*n_3).sub.val.fltnum == 0.0f64) as libc::c_int
                                        as libc::c_double;
                                }
                                bcfree(*yyvsp.offset(-(1 as libc::c_int) as isize));
                                yyval = *yyvsp.offset(0 as libc::c_int as isize);
                            } else {
                                (**yyvsp.offset(-(1 as libc::c_int) as isize))
                                    .opcode = Op_not;
                                add_lint(
                                    *yyvsp.offset(0 as libc::c_int as isize),
                                    LINT_assign_in_cond,
                                );
                                yyval = list_append(
                                    *yyvsp.offset(0 as libc::c_int as isize),
                                    *yyvsp.offset(-(1 as libc::c_int) as isize),
                                );
                            }
                        }
                        169 => {
                            yyval = list_append(
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                                bcalloc(Op_parens, 1 as libc::c_int, sourceline),
                            );
                        }
                        170 => {
                            yyval = snode(
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                                *yyvsp.offset(-(3 as libc::c_int) as isize),
                            );
                            if yyval.is_null() {
                                current_block = 3397147810542949670;
                                break;
                            }
                        }
                        171 => {
                            yyval = snode(
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                                *yyvsp.offset(-(3 as libc::c_int) as isize),
                            );
                            if yyval.is_null() {
                                current_block = 3397147810542949670;
                                break;
                            }
                        }
                        172 => {
                            static mut warned_1: bool = 0 as libc::c_int != 0;
                            if do_flags as libc::c_uint
                                & (DO_LINT_INVALID as libc::c_int
                                    | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                && !warned_1
                            {
                                warned_1 = 1 as libc::c_int != 0;
                                lintwarn_ln(
                                    (**yyvsp.offset(0 as libc::c_int as isize)).source_line
                                        as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"call of `length' without parentheses is not portable\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            yyval = snode(
                                0 as *mut INSTRUCTION,
                                *yyvsp.offset(0 as libc::c_int as isize),
                            );
                            if yyval.is_null() {
                                current_block = 3397147810542949670;
                                break;
                            }
                        }
                        175 => {
                            (**yyvsp.offset(-(1 as libc::c_int) as isize))
                                .opcode = Op_preincrement;
                            yyval = mk_assignment(
                                *yyvsp.offset(0 as libc::c_int as isize),
                                0 as *mut INSTRUCTION,
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                            );
                        }
                        176 => {
                            (**yyvsp.offset(-(1 as libc::c_int) as isize))
                                .opcode = Op_predecrement;
                            yyval = mk_assignment(
                                *yyvsp.offset(0 as libc::c_int as isize),
                                0 as *mut INSTRUCTION,
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                            );
                        }
                        177 => {
                            yyval = list_create(
                                *yyvsp.offset(0 as libc::c_int as isize),
                            );
                        }
                        178 => {
                            yyval = list_create(
                                *yyvsp.offset(0 as libc::c_int as isize),
                            );
                        }
                        179 => {
                            if (*(**yyvsp.offset(0 as libc::c_int as isize)).d.di).opcode
                                as libc::c_uint == Op_push_i as libc::c_int as libc::c_uint
                                && (*(*(**yyvsp.offset(0 as libc::c_int as isize)).d.di)
                                    .d
                                    .dn)
                                    .flags as libc::c_uint
                                    & STRING as libc::c_int as libc::c_uint
                                    == 0 as libc::c_int as libc::c_uint
                            {
                                let mut n_4: *mut NODE = (*(**yyvsp
                                    .offset(0 as libc::c_int as isize))
                                    .d
                                    .di)
                                    .d
                                    .dn;
                                force_number(n_4);
                                negate_num(n_4);
                                yyval = *yyvsp.offset(0 as libc::c_int as isize);
                                bcfree(*yyvsp.offset(-(1 as libc::c_int) as isize));
                            } else {
                                (**yyvsp.offset(-(1 as libc::c_int) as isize))
                                    .opcode = Op_unary_minus;
                                yyval = list_append(
                                    *yyvsp.offset(0 as libc::c_int as isize),
                                    *yyvsp.offset(-(1 as libc::c_int) as isize),
                                );
                            }
                        }
                        180 => {
                            if (*(**yyvsp.offset(0 as libc::c_int as isize)).d.di).opcode
                                as libc::c_uint == Op_push_i as libc::c_int as libc::c_uint
                                && (*(*(**yyvsp.offset(0 as libc::c_int as isize)).d.di)
                                    .d
                                    .dn)
                                    .flags as libc::c_uint
                                    & STRING as libc::c_int as libc::c_uint
                                    == 0 as libc::c_int as libc::c_uint
                                && (*(*(**yyvsp.offset(0 as libc::c_int as isize)).d.di)
                                    .d
                                    .dn)
                                    .flags as libc::c_uint
                                    & NUMCONSTSTR as libc::c_int as libc::c_uint
                                    != 0 as libc::c_int as libc::c_uint
                            {
                                let mut n_5: *mut NODE = (*(**yyvsp
                                    .offset(0 as libc::c_int as isize))
                                    .d
                                    .di)
                                    .d
                                    .dn;
                                add_sign_to_num(n_5, '+' as i32 as libc::c_char);
                                yyval = *yyvsp.offset(0 as libc::c_int as isize);
                                bcfree(*yyvsp.offset(-(1 as libc::c_int) as isize));
                            } else {
                                (**yyvsp.offset(-(1 as libc::c_int) as isize))
                                    .opcode = Op_unary_plus;
                                yyval = list_append(
                                    *yyvsp.offset(0 as libc::c_int as isize),
                                    *yyvsp.offset(-(1 as libc::c_int) as isize),
                                );
                            }
                        }
                        181 => {
                            func_use(
                                (*(**yyvsp.offset(0 as libc::c_int as isize)).d.di).d.name,
                                FUNC_USE,
                            );
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        182 => {
                            let mut f: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut t_2: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut name_0: *mut libc::c_char = 0 as *mut libc::c_char;
                            let mut indirect_var: *mut NODE = 0 as *mut NODE;
                            static mut warned_2: bool = 0 as libc::c_int != 0;
                            let mut msg_0: *const libc::c_char = dcgettext(
                                0 as *const libc::c_char,
                                b"indirect function calls are a gawk extension\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            );
                            if do_flags as libc::c_uint
                                & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0
                                || do_flags as libc::c_uint
                                    & DO_POSIX as libc::c_int as libc::c_uint != 0
                            {
                                yyerror(b"%s\0" as *const u8 as *const libc::c_char, msg_0);
                            } else if do_flags as libc::c_uint
                                & DO_LINT_EXTENSIONS as libc::c_int as libc::c_uint != 0
                                && !warned_2
                            {
                                warned_2 = 1 as libc::c_int != 0;
                                (set_loc
                                    as unsafe extern "C" fn(
                                        *const libc::c_char,
                                        libc::c_int,
                                    ) -> ())(
                                    b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                    2025 as libc::c_int,
                                );
                                (Some(lintfunc.unwrap()))
                                    .unwrap()(
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    msg_0,
                                );
                            }
                            f = (**yyvsp.offset(0 as libc::c_int as isize)).d.di;
                            (*f).opcode = Op_indirect_func_call;
                            name_0 = estrdup((*f).d.name, strlen((*f).d.name));
                            if is_std_var(name_0) != 0 {
                                yyerror(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"cannot use special variable `%s' for indirect function call\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    name_0,
                                );
                            }
                            indirect_var = variable(
                                (*f).source_line as libc::c_int,
                                name_0,
                                Node_var_new,
                            );
                            t_2 = bcalloc(Op_push, 1 as libc::c_int, 0 as libc::c_int);
                            (*t_2).d.dn = indirect_var;
                            yyval = list_prepend(
                                *yyvsp.offset(0 as libc::c_int as isize),
                                t_2,
                            );
                            at_seen -= 1;
                            at_seen;
                        }
                        183 => {
                            let mut n_6: *mut NODE = 0 as *mut NODE;
                            let mut name_1: *mut libc::c_char = (**yyvsp
                                .offset(-(3 as libc::c_int) as isize))
                                .d
                                .name;
                            let mut qname_0: *mut libc::c_char = qualify_name(
                                name_1,
                                strlen(name_1),
                            );
                            if qname_0 != name_1 {
                                free(name_1 as *mut libc::c_void);
                                let ref mut fresh79 = (**yyvsp
                                    .offset(-(3 as libc::c_int) as isize))
                                    .d
                                    .name;
                                *fresh79 = qname_0;
                            }
                            if at_seen == 0 {
                                n_6 = lookup(
                                    (**yyvsp.offset(-(3 as libc::c_int) as isize)).d.name,
                                );
                                if !n_6.is_null()
                                    && (*n_6).type_0 as libc::c_uint
                                        != Node_func as libc::c_int as libc::c_uint
                                    && (*n_6).type_0 as libc::c_uint
                                        != Node_ext_func as libc::c_int as libc::c_uint
                                {
                                    error_ln(
                                        (**yyvsp.offset(-(3 as libc::c_int) as isize)).source_line
                                            as libc::c_int,
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"attempt to use non-function `%s' in function call\0"
                                                as *const u8 as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                        (**yyvsp.offset(-(3 as libc::c_int) as isize)).d.name,
                                    );
                                }
                            }
                            param_sanity(*yyvsp.offset(-(1 as libc::c_int) as isize));
                            (**yyvsp.offset(-(3 as libc::c_int) as isize))
                                .opcode = Op_func_call;
                            let ref mut fresh80 = (**yyvsp
                                .offset(-(3 as libc::c_int) as isize))
                                .x
                                .xn;
                            *fresh80 = 0 as *mut NODE;
                            if (*yyvsp.offset(-(1 as libc::c_int) as isize)).is_null() {
                                (*(*yyvsp.offset(-(3 as libc::c_int) as isize))
                                    .offset(1 as libc::c_int as isize))
                                    .x
                                    .xl = 0 as libc::c_int as libc::c_long;
                                yyval = list_create(
                                    *yyvsp.offset(-(3 as libc::c_int) as isize),
                                );
                            } else {
                                let mut t_3: *mut INSTRUCTION = *yyvsp
                                    .offset(-(1 as libc::c_int) as isize);
                                (*(*yyvsp.offset(-(3 as libc::c_int) as isize))
                                    .offset(1 as libc::c_int as isize))
                                    .x
                                    .xl = count_expressions(&mut t_3, 1 as libc::c_int != 0)
                                    as libc::c_long;
                                yyval = list_append(
                                    t_3,
                                    *yyvsp.offset(-(3 as libc::c_int) as isize),
                                );
                            }
                        }
                        184 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        185 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        186 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        187 => {
                            yyval = *yyvsp.offset(-(1 as libc::c_int) as isize);
                        }
                        188 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        189 => {
                            yyval = list_merge(
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                            );
                        }
                        190 => {
                            let mut ip_7: *mut INSTRUCTION = (**yyvsp
                                .offset(0 as libc::c_int as isize))
                                .d
                                .di;
                            let mut count_0: libc::c_int = (*ip_7).d.dl as libc::c_int;
                            if count_0 > 1 as libc::c_int {
                                (*ip_7).opcode = Op_concat;
                                (*ip_7).d.dl = 1 as libc::c_int as libc::c_long;
                                (*ip_7).x.xl = count_0 as libc::c_long;
                            } else {
                                (*ip_7).opcode = Op_no_op;
                            }
                            sub_counter += 1;
                            sub_counter;
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        191 => {
                            let mut t_4: *mut INSTRUCTION = *yyvsp
                                .offset(-(1 as libc::c_int) as isize);
                            if (*yyvsp.offset(-(1 as libc::c_int) as isize)).is_null() {
                                error_ln(
                                    (**yyvsp.offset(0 as libc::c_int as isize)).source_line
                                        as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"invalid subscript expression\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                t_4 = list_create(
                                    bcalloc(Op_push_i, 1 as libc::c_int, 0 as libc::c_int),
                                );
                                (*(*t_4).nexti).d.dn = dupnode(Nnull_string);
                                (**yyvsp.offset(0 as libc::c_int as isize))
                                    .d
                                    .dl = 1 as libc::c_int as libc::c_long;
                            } else {
                                (**yyvsp.offset(0 as libc::c_int as isize))
                                    .d
                                    .dl = count_expressions(&mut t_4, 0 as libc::c_int != 0)
                                    as libc::c_long;
                            }
                            yyval = list_append(
                                t_4,
                                *yyvsp.offset(0 as libc::c_int as isize),
                            );
                        }
                        192 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        193 => {
                            yyval = list_merge(
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                                *yyvsp.offset(0 as libc::c_int as isize),
                            );
                        }
                        194 => {
                            yyval = *yyvsp.offset(-(1 as libc::c_int) as isize);
                        }
                        195 => {
                            (**yyvsp.offset(0 as libc::c_int as isize)).opcode = Op_push;
                            let ref mut fresh81 = (**yyvsp
                                .offset(0 as libc::c_int as isize))
                                .d
                                .dn;
                            *fresh81 = variable(
                                (**yyvsp.offset(0 as libc::c_int as isize)).source_line
                                    as libc::c_int,
                                (**yyvsp.offset(0 as libc::c_int as isize)).d.name,
                                Node_var_new,
                            );
                            yyval = list_create(
                                *yyvsp.offset(0 as libc::c_int as isize),
                            );
                        }
                        196 => {
                            let mut arr_2: *mut libc::c_char = (**yyvsp
                                .offset(-(1 as libc::c_int) as isize))
                                .d
                                .name;
                            let ref mut fresh82 = (**yyvsp
                                .offset(-(1 as libc::c_int) as isize))
                                .d
                                .dn;
                            *fresh82 = variable(
                                (**yyvsp.offset(-(1 as libc::c_int) as isize)).source_line
                                    as libc::c_int,
                                arr_2,
                                Node_var_new,
                            );
                            (**yyvsp.offset(-(1 as libc::c_int) as isize))
                                .opcode = Op_push_array;
                            yyval = list_prepend(
                                *yyvsp.offset(0 as libc::c_int as isize),
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                            );
                        }
                        197 => {
                            let mut ip_8: *mut INSTRUCTION = (**yyvsp
                                .offset(0 as libc::c_int as isize))
                                .nexti;
                            if (*ip_8).opcode as libc::c_uint
                                == Op_push as libc::c_int as libc::c_uint
                                && (*(*ip_8).d.dn).type_0 as libc::c_uint
                                    == Node_var as libc::c_int as libc::c_uint
                                && ((*(*ip_8).d.dn).sub.nodep.r.uptr).is_some()
                            {
                                yyval = list_prepend(
                                    *yyvsp.offset(0 as libc::c_int as isize),
                                    bcalloc(Op_var_update, 1 as libc::c_int, 0 as libc::c_int),
                                );
                                (*(*yyval).nexti).x.aptr = (*(*ip_8).d.dn).sub.nodep.r.uptr;
                            } else {
                                yyval = *yyvsp.offset(0 as libc::c_int as isize);
                            }
                        }
                        198 => {
                            yyval = list_append(
                                *yyvsp.offset(-(1 as libc::c_int) as isize),
                                *yyvsp.offset(-(2 as libc::c_int) as isize),
                            );
                            if !(*yyvsp.offset(0 as libc::c_int as isize)).is_null() {
                                mk_assignment(
                                    *yyvsp.offset(-(1 as libc::c_int) as isize),
                                    0 as *mut INSTRUCTION,
                                    *yyvsp.offset(0 as libc::c_int as isize),
                                );
                            }
                        }
                        199 => {
                            (**yyvsp.offset(0 as libc::c_int as isize))
                                .opcode = Op_postincrement;
                        }
                        200 => {
                            (**yyvsp.offset(0 as libc::c_int as isize))
                                .opcode = Op_postdecrement;
                        }
                        201 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        202 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        }
                        203 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                            yyerrstatus = 0 as libc::c_int;
                        }
                        204 => {
                            yyerrstatus = 0 as libc::c_int;
                        }
                        205 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        207 => {
                            yyerrstatus = 0 as libc::c_int;
                        }
                        208 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                            yyerrstatus = 0 as libc::c_int;
                        }
                        209 => {
                            yyval = *yyvsp.offset(0 as libc::c_int as isize);
                            yyerrstatus = 0 as libc::c_int;
                        }
                        _ => {}
                    }
                    yyvsp = yyvsp.offset(-(yylen as isize));
                    yyssp = yyssp.offset(-(yylen as isize));
                    yylen = 0 as libc::c_int;
                    yyvsp = yyvsp.offset(1);
                    *yyvsp = yyval;
                    let yylhs: libc::c_int = yyr1[yyn as usize] as libc::c_int
                        - 77 as libc::c_int;
                    let yyi: libc::c_int = yypgoto[yylhs as usize] as libc::c_int
                        + *yyssp as libc::c_int;
                    yystate = if 0 as libc::c_int <= yyi && yyi <= 1200 as libc::c_int
                        && yycheck[yyi as usize] as libc::c_int == *yyssp as libc::c_int
                    {
                        yytable[yyi as usize] as libc::c_int
                    } else {
                        yydefgoto[yylhs as usize] as libc::c_int
                    };
                    current_block = 12782234849683662922;
                }
                _ => {}
            }
            match current_block {
                10380106008157032803 => {
                    yyerrstatus = 3 as libc::c_int;
                    loop {
                        yyn = yypact[yystate as usize] as libc::c_int;
                        if !(yyn == -(276 as libc::c_int)) {
                            yyn += YYSYMBOL_YYerror as libc::c_int;
                            if 0 as libc::c_int <= yyn && yyn <= 1200 as libc::c_int
                                && yycheck[yyn as usize] as libc::c_int
                                    == YYSYMBOL_YYerror as libc::c_int
                            {
                                yyn = yytable[yyn as usize] as libc::c_int;
                                if (0 as libc::c_int) < yyn {
                                    break;
                                }
                            }
                        }
                        if yyssp == yyss {
                            current_block = 3397147810542949670;
                            break 's_46;
                        }
                        yydestruct(
                            b"Error: popping\0" as *const u8 as *const libc::c_char,
                            yystos[yystate as usize] as yysymbol_kind_t,
                            yyvsp,
                        );
                        yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
                        yyssp = yyssp.offset(-(1 as libc::c_int as isize));
                        yystate = *yyssp as yy_state_fast_t;
                    }
                    yyvsp = yyvsp.offset(1);
                    *yyvsp = yylval;
                    yystate = yyn;
                }
                _ => {}
            }
            yyssp = yyssp.offset(1);
            yyssp;
        }
    }
    match current_block {
        7639343393969835181 => {
            yyerror(b"memory exhausted\0" as *const u8 as *const libc::c_char);
            yyresult = 2 as libc::c_int;
        }
        3397147810542949670 => {
            yyresult = 1 as libc::c_int;
        }
        _ => {}
    }
    if yychar != -(2 as libc::c_int) {
        yytoken = (if 0 as libc::c_int <= yychar && yychar <= 310 as libc::c_int {
            yytranslate[yychar as usize] as yysymbol_kind_t as libc::c_int
        } else {
            YYSYMBOL_YYUNDEF as libc::c_int
        }) as yysymbol_kind_t;
        yydestruct(
            b"Cleanup: discarding lookahead\0" as *const u8 as *const libc::c_char,
            yytoken,
            &mut yylval,
        );
    }
    yyvsp = yyvsp.offset(-(yylen as isize));
    yyssp = yyssp.offset(-(yylen as isize));
    while yyssp != yyss {
        yydestruct(
            b"Cleanup: popping\0" as *const u8 as *const libc::c_char,
            yystos[*yyssp as libc::c_int as usize] as yysymbol_kind_t,
            yyvsp,
        );
        yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
        yyssp = yyssp.offset(-(1 as libc::c_int as isize));
    }
    if yyss != yyssa.as_mut_ptr() {
        free(yyss as *mut libc::c_void);
    }
    return yyresult;
}
static mut tokentab: [token; 71] = unsafe {
    [
        {
            let mut init = token {
                operator: b"BEGIN\0" as *const u8 as *const libc::c_char,
                value: Op_rule,
                class: 273 as libc::c_int,
                flags: 0 as libc::c_int as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"BEGINFILE\0" as *const u8 as *const libc::c_char,
                value: Op_rule,
                class: 292 as libc::c_int,
                flags: 0x400 as libc::c_int as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"END\0" as *const u8 as *const libc::c_char,
                value: Op_rule,
                class: 274 as libc::c_int,
                flags: 0 as libc::c_int as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"ENDFILE\0" as *const u8 as *const libc::c_char,
                value: Op_rule,
                class: 293 as libc::c_int,
                flags: 0x400 as libc::c_int as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"and\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: 0x400 as libc::c_int as libc::c_uint,
                ptr: Some(do_and as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"asort\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: (0x400 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint,
                ptr: Some(do_asort as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"asorti\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: (0x400 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint,
                ptr: Some(do_asorti as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"atan2\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: (0x100 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int)
                    as libc::c_uint,
                ptr: Some(do_atan2 as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"bindtextdomain\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: (0x400 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint,
                ptr: Some(
                    do_bindtextdomain as unsafe extern "C" fn(libc::c_int) -> *mut NODE,
                ),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"break\0" as *const u8 as *const libc::c_char,
                value: Op_K_break,
                class: 285 as libc::c_int,
                flags: 0 as libc::c_int as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"case\0" as *const u8 as *const libc::c_char,
                value: Op_K_case,
                class: 280 as libc::c_int,
                flags: 0x400 as libc::c_int as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"close\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: (0x100 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint,
                ptr: Some(do_close as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"compl\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: (0x400 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int)
                    as libc::c_uint,
                ptr: Some(do_compl as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"continue\0" as *const u8 as *const libc::c_char,
                value: Op_K_continue,
                class: 286 as libc::c_int,
                flags: 0 as libc::c_int as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"cos\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: (0x100 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int)
                    as libc::c_uint,
                ptr: Some(do_cos as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"dcgettext\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: (0x400 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint,
                ptr: Some(
                    do_dcgettext as unsafe extern "C" fn(libc::c_int) -> *mut NODE,
                ),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"dcngettext\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: (0x400 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 3 as libc::c_int
                    | (1 as libc::c_int) << 4 as libc::c_int
                    | (1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint,
                ptr: Some(
                    do_dcngettext as unsafe extern "C" fn(libc::c_int) -> *mut NODE,
                ),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"default\0" as *const u8 as *const libc::c_char,
                value: Op_K_default,
                class: 281 as libc::c_int,
                flags: 0x400 as libc::c_int as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"delete\0" as *const u8 as *const libc::c_char,
                value: Op_K_delete,
                class: 278 as libc::c_int,
                flags: 0x100 as libc::c_int as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"do\0" as *const u8 as *const libc::c_char,
                value: Op_K_do,
                class: 283 as libc::c_int,
                flags: (0x100 as libc::c_int | 0x800 as libc::c_int
                    | 0x1000 as libc::c_int) as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"else\0" as *const u8 as *const libc::c_char,
                value: Op_K_else,
                class: 276 as libc::c_int,
                flags: 0 as libc::c_int as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"eval\0" as *const u8 as *const libc::c_char,
                value: Op_symbol,
                class: 305 as libc::c_int,
                flags: 0 as libc::c_int as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"exit\0" as *const u8 as *const libc::c_char,
                value: Op_K_exit,
                class: 290 as libc::c_int,
                flags: 0 as libc::c_int as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"exp\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint,
                ptr: Some(do_exp as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"fflush\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: ((1 as libc::c_int) << 0 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint,
                ptr: Some(do_fflush as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"for\0" as *const u8 as *const libc::c_char,
                value: Op_K_for,
                class: 284 as libc::c_int,
                flags: (0x800 as libc::c_int | 0x1000 as libc::c_int) as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"func\0" as *const u8 as *const libc::c_char,
                value: Op_func,
                class: 291 as libc::c_int,
                flags: (0x200 as libc::c_int | 0x100 as libc::c_int) as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"function\0" as *const u8 as *const libc::c_char,
                value: Op_func,
                class: 291 as libc::c_int,
                flags: 0x100 as libc::c_int as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"gensub\0" as *const u8 as *const libc::c_char,
                value: Op_sub_builtin,
                class: 301 as libc::c_int,
                flags: (0x400 as libc::c_int | (1 as libc::c_int) << 3 as libc::c_int
                    | (1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"getline\0" as *const u8 as *const libc::c_char,
                value: Op_K_getline_redir,
                class: 294 as libc::c_int,
                flags: 0x100 as libc::c_int as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"gsub\0" as *const u8 as *const libc::c_char,
                value: Op_sub_builtin,
                class: 301 as libc::c_int,
                flags: (0x100 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"if\0" as *const u8 as *const libc::c_char,
                value: Op_K_if,
                class: 275 as libc::c_int,
                flags: 0 as libc::c_int as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"in\0" as *const u8 as *const libc::c_char,
                value: Op_symbol,
                class: 296 as libc::c_int,
                flags: 0 as libc::c_int as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"include\0" as *const u8 as *const libc::c_char,
                value: Op_symbol,
                class: 304 as libc::c_int,
                flags: 0x400 as libc::c_int as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"index\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint,
                ptr: Some(do_index as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"int\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint,
                ptr: Some(do_int as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"isarray\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: (0x400 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int)
                    as libc::c_uint,
                ptr: Some(do_isarray as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"length\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 302 as libc::c_int,
                flags: ((1 as libc::c_int) << 0 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint,
                ptr: Some(do_length as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"load\0" as *const u8 as *const libc::c_char,
                value: Op_symbol,
                class: 306 as libc::c_int,
                flags: 0x400 as libc::c_int as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"log\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint,
                ptr: Some(do_log as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"lshift\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: (0x400 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int)
                    as libc::c_uint,
                ptr: Some(do_lshift as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"match\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: (0x100 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint,
                ptr: Some(do_match as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"mkbool\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: (0x400 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int)
                    as libc::c_uint,
                ptr: Some(do_mkbool as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"mktime\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: (0x400 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint,
                ptr: Some(do_mktime as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"namespace\0" as *const u8 as *const libc::c_char,
                value: Op_symbol,
                class: 307 as libc::c_int,
                flags: 0x400 as libc::c_int as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"next\0" as *const u8 as *const libc::c_char,
                value: Op_K_next,
                class: 289 as libc::c_int,
                flags: 0 as libc::c_int as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"nextfile\0" as *const u8 as *const libc::c_char,
                value: Op_K_nextfile,
                class: 295 as libc::c_int,
                flags: 0 as libc::c_int as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"or\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: 0x400 as libc::c_int as libc::c_uint,
                ptr: Some(do_or as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"patsplit\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: (0x400 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 3 as libc::c_int
                    | (1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint,
                ptr: Some(do_patsplit as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"print\0" as *const u8 as *const libc::c_char,
                value: Op_K_print,
                class: 287 as libc::c_int,
                flags: 0 as libc::c_int as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"printf\0" as *const u8 as *const libc::c_char,
                value: Op_K_printf,
                class: 288 as libc::c_int,
                flags: 0 as libc::c_int as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"rand\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: (0x100 as libc::c_int | (1 as libc::c_int) << 0 as libc::c_int)
                    as libc::c_uint,
                ptr: Some(do_rand as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"return\0" as *const u8 as *const libc::c_char,
                value: Op_K_return,
                class: 277 as libc::c_int,
                flags: 0x100 as libc::c_int as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"rshift\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: (0x400 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int)
                    as libc::c_uint,
                ptr: Some(do_rshift as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"sin\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: (0x100 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int)
                    as libc::c_uint,
                ptr: Some(do_sin as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"split\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: ((1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 3 as libc::c_int
                    | (1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint,
                ptr: Some(do_split as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"sprintf\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: 0 as libc::c_int as libc::c_uint,
                ptr: Some(do_sprintf as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"sqrt\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint,
                ptr: Some(do_sqrt as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"srand\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: (0x100 as libc::c_int | (1 as libc::c_int) << 0 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint,
                ptr: Some(do_srand as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"strftime\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: (0x400 as libc::c_int | (1 as libc::c_int) << 0 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint,
                ptr: Some(do_strftime as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"strtonum\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: (0x400 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int)
                    as libc::c_uint,
                ptr: Some(do_strtonum as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"sub\0" as *const u8 as *const libc::c_char,
                value: Op_sub_builtin,
                class: 301 as libc::c_int,
                flags: (0x100 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"substr\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: ((1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint,
                ptr: Some(do_substr as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"switch\0" as *const u8 as *const libc::c_char,
                value: Op_K_switch,
                class: 279 as libc::c_int,
                flags: (0x400 as libc::c_int | 0x800 as libc::c_int) as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"system\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: (0x100 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int)
                    as libc::c_uint,
                ptr: Some(do_system as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"systime\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: (0x400 as libc::c_int | (1 as libc::c_int) << 0 as libc::c_int)
                    as libc::c_uint,
                ptr: Some(do_systime as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"tolower\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: (0x100 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int)
                    as libc::c_uint,
                ptr: Some(do_tolower as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"toupper\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: (0x100 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int)
                    as libc::c_uint,
                ptr: Some(do_toupper as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"typeof\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: (0x400 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint,
                ptr: Some(do_typeof as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"while\0" as *const u8 as *const libc::c_char,
                value: Op_K_while,
                class: 282 as libc::c_int,
                flags: (0x800 as libc::c_int | 0x1000 as libc::c_int) as libc::c_uint,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"xor\0" as *const u8 as *const libc::c_char,
                value: Op_builtin,
                class: 301 as libc::c_int,
                flags: 0x400 as libc::c_int as libc::c_uint,
                ptr: Some(do_xor as unsafe extern "C" fn(libc::c_int) -> *mut NODE),
                ptr2: None,
            };
            init
        },
    ]
};
static mut cur_mbstate: mbstate_t = mbstate_t {
    __count: 0,
    __value: C2RustUnnamed { __wch: 0 },
};
static mut cur_char_ring: [libc::c_char; 128] = [0; 128];
static mut cur_ring_idx: libc::c_int = 0;
pub unsafe extern "C" fn getfname(
    mut fptr: Option::<unsafe extern "C" fn(libc::c_int) -> *mut NODE>,
    mut prepend_awk: bool,
) -> *const libc::c_char {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    static mut buf: [libc::c_char; 100] = [0; 100];
    j = (::std::mem::size_of::<[token; 71]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<token>() as libc::c_ulong) as libc::c_int;
    i = 0 as libc::c_int;
    while i < j {
        if tokentab[i as usize].ptr == fptr || tokentab[i as usize].ptr2 == fptr {
            if prepend_awk as libc::c_int != 0
                && tokentab[i as usize].flags & 0x400 as libc::c_int as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint
            {
                sprintf(
                    buf.as_mut_ptr(),
                    b"awk::%s\0" as *const u8 as *const libc::c_char,
                    tokentab[i as usize].operator,
                );
                return buf.as_mut_ptr();
            }
            return tokentab[i as usize].operator;
        }
        i += 1;
        i;
    }
    return 0 as *const libc::c_char;
}
pub unsafe extern "C" fn negate_num(mut n: *mut NODE) {
    add_sign_to_num(n, '-' as i32 as libc::c_char);
    if 0 as libc::c_int == 0 {
        (*n).sub.val.fltnum = -(*n).sub.val.fltnum;
        return;
    }
}
unsafe extern "C" fn add_sign_to_num(mut n: *mut NODE, mut sign: libc::c_char) {
    if (*n).flags as libc::c_uint & NUMCONSTSTR as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        s = (*n).sub.val.sp;
        memmove(
            &mut *s.offset(1 as libc::c_int as isize) as *mut libc::c_char
                as *mut libc::c_void,
            &mut *s.offset(0 as libc::c_int as isize) as *mut libc::c_char
                as *const libc::c_void,
            ((*n).sub.val.slen).wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        *s.offset(0 as libc::c_int as isize) = sign;
        (*n).sub.val.slen = ((*n).sub.val.slen).wrapping_add(1);
        (*n).sub.val.slen;
    }
}
unsafe extern "C" fn print_included_from() {
    let mut saveline: libc::c_int = 0;
    let mut line: libc::c_int = 0;
    let mut s: *mut SRCFILE = 0 as *mut SRCFILE;
    saveline = sourceline;
    sourceline = 0 as libc::c_int;
    s = sourcefile;
    while !s.is_null()
        && (*s).stype as libc::c_uint == SRC_INC as libc::c_int as libc::c_uint
    {
        s = (*s).next;
        if s.is_null() || (*s).fd <= -(1 as libc::c_int) {
            continue;
        }
        line = (*s).srclines;
        if (*s).lasttok == 308 as libc::c_int {
            line -= 1;
            line;
        }
        msg(
            b"%s %s:%d%c\0" as *const u8 as *const libc::c_char,
            if (*s).prev == sourcefile {
                b"In file included from\0" as *const u8 as *const libc::c_char
            } else {
                b"                 from\0" as *const u8 as *const libc::c_char
            },
            if (*s).stype as libc::c_uint == SRC_INC as libc::c_int as libc::c_uint
                || (*s).stype as libc::c_uint == SRC_FILE as libc::c_int as libc::c_uint
            {
                (*s).src as *const libc::c_char
            } else {
                b"cmd. line\0" as *const u8 as *const libc::c_char
            },
            line,
            if (*s).stype as libc::c_uint == SRC_INC as libc::c_int as libc::c_uint {
                ',' as i32
            } else {
                ':' as i32
            },
        );
    }
    sourceline = saveline;
}
unsafe extern "C" fn warning_ln(
    mut line: libc::c_int,
    mut mesg: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::std::ffi::VaListImpl;
    let mut saveline: libc::c_int = 0;
    saveline = sourceline;
    sourceline = line;
    print_included_from();
    args_0 = args.clone();
    err(
        0 as libc::c_int != 0,
        dcgettext(
            0 as *const libc::c_char,
            b"warning: \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        mesg,
        args_0.as_va_list(),
    );
    sourceline = saveline;
}
unsafe extern "C" fn lintwarn_ln(
    mut line: libc::c_int,
    mut mesg: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::std::ffi::VaListImpl;
    let mut saveline: libc::c_int = 0;
    saveline = sourceline;
    sourceline = line;
    print_included_from();
    args_0 = args.clone();
    if lintfunc == Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ())
    {
        err(
            1 as libc::c_int != 0,
            dcgettext(
                0 as *const libc::c_char,
                b"fatal: \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            mesg,
            args_0.as_va_list(),
        );
    } else {
        err(
            0 as libc::c_int != 0,
            dcgettext(
                0 as *const libc::c_char,
                b"warning: \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            mesg,
            args_0.as_va_list(),
        );
    }
    sourceline = saveline;
    if lintfunc == Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ())
    {
        gawk_exit(2 as libc::c_int);
    }
}
unsafe extern "C" fn error_ln(
    mut line: libc::c_int,
    mut m: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::std::ffi::VaListImpl;
    let mut saveline: libc::c_int = 0;
    saveline = sourceline;
    sourceline = line;
    print_included_from();
    errcount += 1;
    errcount;
    args_0 = args.clone();
    err(
        0 as libc::c_int != 0,
        b"error: \0" as *const u8 as *const libc::c_char,
        m,
        args_0.as_va_list(),
    );
    sourceline = saveline;
}
static mut syn_err_len: size_t = 0;
unsafe extern "C" fn yyerror(mut m: *const libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    let mut mesg: *const libc::c_char = 0 as *const libc::c_char;
    let mut bp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut scan: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut count: libc::c_int = 0;
    static mut end_of_file_line: [libc::c_char; 14] = unsafe {
        *::std::mem::transmute::<&[u8; 14], &mut [libc::c_char; 14]>(b"(END OF FILE)\0")
    };
    static mut syntax_error: [libc::c_char; 13] = unsafe {
        *::std::mem::transmute::<&[u8; 13], &mut [libc::c_char; 13]>(b"syntax error\0")
    };
    let mut generic_error: bool = strncmp(m, syntax_error.as_mut_ptr(), syn_err_len)
        == 0 as libc::c_int;
    print_included_from();
    errcount += 1;
    errcount;
    if !lexptr.is_null() && !lexeme.is_null() {
        if thisline.is_null() {
            cp = lexeme;
            if *cp as libc::c_int == '\n' as i32 {
                if cp > lexptr_begin {
                    cp = cp.offset(-1);
                    cp;
                }
                mesg = dcgettext(
                    0 as *const libc::c_char,
                    b"unexpected newline or end of string\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                );
            }
            while cp != lexptr_begin && *cp as libc::c_int != '\n' as i32 {
                cp = cp.offset(-1);
                cp;
            }
            if *cp as libc::c_int == '\n' as i32 {
                cp = cp.offset(1);
                cp;
            }
            thisline = cp;
        }
        bp = lexeme;
        if bp < thisline {
            bp = thisline.offset(1 as libc::c_int as isize);
        }
        while bp < lexend && *bp as libc::c_int != 0 && *bp as libc::c_int != '\n' as i32
        {
            bp = bp.offset(1);
            bp;
        }
    } else {
        thisline = end_of_file_line.as_mut_ptr();
        bp = thisline.offset(strlen(thisline) as isize);
    }
    if lexeof as libc::c_int != 0 && mesg.is_null() && generic_error as libc::c_int != 0
    {
        msg(b"%s\0" as *const u8 as *const libc::c_char, end_of_file_line.as_mut_ptr());
        mesg = dcgettext(
            0 as *const libc::c_char,
            b"source files / command-line arguments must contain complete functions or rules\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
    } else {
        msg(
            b"%.*s\0" as *const u8 as *const libc::c_char,
            bp.offset_from(thisline) as libc::c_long as libc::c_int,
            thisline,
        );
    }
    args_0 = args.clone();
    if mesg.is_null() {
        mesg = m;
    }
    count = (strlen(mesg)).wrapping_add(1 as libc::c_int as libc::c_ulong)
        as libc::c_int;
    if !lexptr.is_null() {
        count = (count as libc::c_long
            + (lexeme.offset_from(thisline) as libc::c_long
                + 2 as libc::c_int as libc::c_long)) as libc::c_int;
    }
    buf = ezalloc_real(
        (count + 1 as libc::c_int) as size_t,
        b"yyerror\0" as *const u8 as *const libc::c_char,
        b"buf\0" as *const u8 as *const libc::c_char,
        b"awkgram.y\0" as *const u8 as *const libc::c_char,
        2608 as libc::c_int,
    ) as *mut libc::c_char;
    bp = buf;
    if !lexptr.is_null() {
        scan = thisline;
        while scan < lexeme {
            let fresh83 = scan;
            scan = scan.offset(1);
            if *fresh83 as libc::c_int == '\t' as i32 {
                let fresh84 = bp;
                bp = bp.offset(1);
                *fresh84 = '\t' as i32 as libc::c_char;
            } else {
                let fresh85 = bp;
                bp = bp.offset(1);
                *fresh85 = ' ' as i32 as libc::c_char;
            }
        }
        let fresh86 = bp;
        bp = bp.offset(1);
        *fresh86 = '^' as i32 as libc::c_char;
        let fresh87 = bp;
        bp = bp.offset(1);
        *fresh87 = ' ' as i32 as libc::c_char;
    }
    strcpy(bp, mesg);
    err(
        0 as libc::c_int != 0,
        b"\0" as *const u8 as *const libc::c_char,
        buf,
        args_0.as_va_list(),
    );
    free(buf as *mut libc::c_void);
    exit(1 as libc::c_int);
}
unsafe extern "C" fn mk_program() -> *mut INSTRUCTION {
    let mut current_block: u64;
    let mut cp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut tmp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    if (rule_block[END as libc::c_int as usize]).is_null() {
        rule_block[END as libc::c_int as usize] = list_create(ip_end);
    } else {
        list_prepend(rule_block[END as libc::c_int as usize], ip_end);
    }
    if in_main_context() == 0 {
        if !(rule_block[BEGIN as libc::c_int as usize]).is_null()
            && !(rule_block[Rule as libc::c_int as usize]).is_null()
        {
            cp = list_merge(
                rule_block[BEGIN as libc::c_int as usize],
                rule_block[Rule as libc::c_int as usize],
            );
        } else {
            cp = if !(rule_block[BEGIN as libc::c_int as usize]).is_null() {
                rule_block[BEGIN as libc::c_int as usize]
            } else {
                rule_block[Rule as libc::c_int as usize]
            };
        }
        if !cp.is_null() {
            list_merge(cp, rule_block[END as libc::c_int as usize]);
        } else {
            cp = rule_block[END as libc::c_int as usize];
        }
        list_append(cp, bcalloc(Op_stop, 1 as libc::c_int, 0 as libc::c_int));
    } else {
        if (rule_block[ENDFILE as libc::c_int as usize]).is_null() {
            rule_block[ENDFILE as libc::c_int as usize] = list_create(ip_endfile);
        } else {
            (*ip_rec).x.xl = 1 as libc::c_int as libc::c_long;
            list_prepend(rule_block[ENDFILE as libc::c_int as usize], ip_endfile);
        }
        if (rule_block[BEGINFILE as libc::c_int as usize]).is_null() {
            rule_block[BEGINFILE as libc::c_int as usize] = list_create(ip_beginfile);
        } else {
            list_prepend(rule_block[BEGINFILE as libc::c_int as usize], ip_beginfile);
        }
        if (rule_block[Rule as libc::c_int as usize]).is_null() {
            if (*rule_block[END as libc::c_int as usize]).nexti
                == (*rule_block[END as libc::c_int as usize]).d.di
                && (*rule_block[BEGINFILE as libc::c_int as usize]).nexti
                    == (*rule_block[BEGINFILE as libc::c_int as usize]).d.di
                && (*rule_block[ENDFILE as libc::c_int as usize]).nexti
                    == (*rule_block[ENDFILE as libc::c_int as usize]).d.di
            {
                bcfree(ip_rec);
                bcfree(ip_newfile);
                ip_newfile = 0 as *mut INSTRUCTION;
                ip_rec = ip_newfile;
                list_append(
                    rule_block[BEGINFILE as libc::c_int as usize],
                    bcalloc(Op_after_beginfile, 1 as libc::c_int, 0 as libc::c_int),
                );
                list_append(
                    rule_block[ENDFILE as libc::c_int as usize],
                    bcalloc(Op_after_endfile, 1 as libc::c_int, 0 as libc::c_int),
                );
                if (rule_block[BEGIN as libc::c_int as usize]).is_null() {
                    cp = rule_block[END as libc::c_int as usize];
                } else {
                    cp = list_merge(
                        rule_block[BEGIN as libc::c_int as usize],
                        rule_block[END as libc::c_int as usize],
                    );
                }
                if !interblock_comment.is_null() {
                    list_append(cp, interblock_comment);
                    interblock_comment = 0 as *mut INSTRUCTION;
                }
                list_append(cp, ip_atexit);
                list_append(cp, bcalloc(Op_stop, 1 as libc::c_int, 0 as libc::c_int));
                list_merge(cp, rule_block[BEGINFILE as libc::c_int as usize]);
                list_merge(cp, rule_block[ENDFILE as libc::c_int as usize]);
                if !outer_comment.is_null() {
                    cp = list_merge(list_create(outer_comment), cp);
                    outer_comment = 0 as *mut INSTRUCTION;
                }
                if !interblock_comment.is_null() {
                    list_append(cp, interblock_comment);
                    interblock_comment = 0 as *mut INSTRUCTION;
                }
                current_block = 1084786345969644221;
            } else {
                rule_block[Rule as libc::c_int
                    as usize] = list_create(
                    bcalloc(Op_no_op, 1 as libc::c_int, 0 as libc::c_int),
                );
                current_block = 11048769245176032998;
            }
        } else {
            current_block = 11048769245176032998;
        }
        match current_block {
            1084786345969644221 => {}
            _ => {
                list_append(
                    rule_block[ENDFILE as libc::c_int as usize],
                    bcalloc(Op_after_endfile, 1 as libc::c_int, 0 as libc::c_int),
                );
                list_prepend(rule_block[Rule as libc::c_int as usize], ip_rec);
                list_append(
                    rule_block[Rule as libc::c_int as usize],
                    bcalloc(Op_jmp, 1 as libc::c_int, 0 as libc::c_int),
                );
                (*(*rule_block[Rule as libc::c_int as usize]).d.di).d.di = ip_rec;
                list_append(
                    rule_block[BEGINFILE as libc::c_int as usize],
                    bcalloc(Op_after_beginfile, 1 as libc::c_int, 0 as libc::c_int),
                );
                cp = list_merge(
                    rule_block[BEGINFILE as libc::c_int as usize],
                    rule_block[Rule as libc::c_int as usize],
                );
                list_prepend(cp, ip_newfile);
                list_merge(cp, rule_block[ENDFILE as libc::c_int as usize]);
                list_merge(cp, rule_block[END as libc::c_int as usize]);
                if !(rule_block[BEGIN as libc::c_int as usize]).is_null() {
                    cp = list_merge(rule_block[BEGIN as libc::c_int as usize], cp);
                }
                if !outer_comment.is_null() {
                    cp = list_merge(list_create(outer_comment), cp);
                    outer_comment = 0 as *mut INSTRUCTION;
                }
                if !interblock_comment.is_null() {
                    list_append(cp, interblock_comment);
                    interblock_comment = 0 as *mut INSTRUCTION;
                }
                list_append(cp, ip_atexit);
                list_append(cp, bcalloc(Op_stop, 1 as libc::c_int, 0 as libc::c_int));
            }
        }
    }
    tmp = (*cp).nexti;
    bcfree(cp);
    return tmp;
}
pub unsafe extern "C" fn parse_program(
    mut pcode: *mut *mut INSTRUCTION,
    mut from_eval: bool,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    called_from_eval = from_eval;
    ip_end = bcalloc(Op_no_op, 1 as libc::c_int, 0 as libc::c_int);
    if in_main_context() == 0 {
        ip_endfile = 0 as *mut INSTRUCTION;
        ip_beginfile = ip_endfile;
        ip_atexit = ip_beginfile;
        ip_rec = ip_atexit;
        ip_newfile = ip_rec;
    } else {
        ip_endfile = bcalloc(Op_no_op, 1 as libc::c_int, 0 as libc::c_int);
        ip_beginfile = bcalloc(Op_no_op, 1 as libc::c_int, 0 as libc::c_int);
        main_beginfile = ip_beginfile;
        ip_rec = bcalloc(Op_get_record, 1 as libc::c_int, 0 as libc::c_int);
        ip_newfile = bcalloc(Op_newfile, 2 as libc::c_int, 0 as libc::c_int);
        (*ip_newfile).d.di = ip_end;
        (*ip_newfile).x.xi = ip_endfile;
        let ref mut fresh88 = (*ip_newfile.offset(1 as libc::c_int as isize)).x.xi;
        *fresh88 = ip_rec;
        (*ip_rec).d.di = ip_newfile;
        ip_atexit = bcalloc(Op_atexit, 1 as libc::c_int, 0 as libc::c_int);
    }
    sourcefile = (*srcfiles).next;
    while (*sourcefile).stype as libc::c_uint
        == SRC_EXTLIB as libc::c_int as libc::c_uint
    {
        sourcefile = (*sourcefile).next;
    }
    lexeof = 0 as libc::c_int != 0;
    lexptr = 0 as *mut libc::c_char;
    lasttok = 0 as libc::c_int;
    memset(
        rule_block.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[*mut INSTRUCTION; 6]>() as libc::c_ulong,
    );
    errcount = 0 as libc::c_int;
    tok = if !tokstart.is_null() { tokstart } else { tokexpand() };
    ret = yyparse();
    *pcode = mk_program();
    source = 0 as *mut libc::c_char;
    sourceline = 0 as libc::c_int;
    if ret == 0 as libc::c_int {
        check_funcs();
    }
    if do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint != 0
        && !check_param_names()
    {
        errcount += 1;
        errcount;
    }
    if args_array.is_null() {
        args_array = emalloc_real(
            ((max_args + 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong),
            b"parse_program\0" as *const u8 as *const libc::c_char,
            b"args_array\0" as *const u8 as *const libc::c_char,
            b"awkgram.y\0" as *const u8 as *const libc::c_char,
            2819 as libc::c_int,
        ) as *mut *mut NODE;
    } else {
        args_array = erealloc_real(
            args_array as *mut libc::c_void,
            ((max_args + 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong),
            b"parse_program\0" as *const u8 as *const libc::c_char,
            b"args_array\0" as *const u8 as *const libc::c_char,
            b"awkgram.y\0" as *const u8 as *const libc::c_char,
            2821 as libc::c_int,
        ) as *mut *mut NODE;
    }
    return (ret != 0 || errcount != 0) as libc::c_int;
}
pub unsafe extern "C" fn free_srcfile(mut thisfile: *mut SRCFILE) {
    free((*thisfile).src as *mut libc::c_void);
    free(thisfile as *mut libc::c_void);
}
unsafe extern "C" fn do_add_srcfile(
    mut stype: srctype,
    mut src: *mut libc::c_char,
    mut path: *mut libc::c_char,
    mut thisfile: *mut SRCFILE,
) -> *mut SRCFILE {
    let mut s: *mut SRCFILE = 0 as *mut SRCFILE;
    s = ezalloc_real(
        ::std::mem::size_of::<SRCFILE>() as libc::c_ulong,
        b"do_add_srcfile\0" as *const u8 as *const libc::c_char,
        b"s\0" as *const u8 as *const libc::c_char,
        b"awkgram.y\0" as *const u8 as *const libc::c_char,
        2842 as libc::c_int,
    ) as *mut SRCFILE;
    (*s).src = estrdup(src, strlen(src));
    (*s).fullpath = path;
    (*s).stype = stype;
    (*s).fd = -(1 as libc::c_int);
    (*s).next = thisfile;
    (*s).prev = (*thisfile).prev;
    (*(*thisfile).prev).next = s;
    (*thisfile).prev = s;
    return s;
}
pub unsafe extern "C" fn add_srcfile(
    mut stype: srctype,
    mut src: *mut libc::c_char,
    mut thisfile: *mut SRCFILE,
    mut already_included: *mut bool,
    mut errcode: *mut libc::c_int,
) -> *mut SRCFILE {
    let mut s: *mut SRCFILE = 0 as *mut SRCFILE;
    let mut sbuf: stat = stat {
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
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut errno_val: libc::c_int = 0 as libc::c_int;
    if !already_included.is_null() {
        *already_included = 0 as libc::c_int != 0;
    }
    if !errcode.is_null() {
        *errcode = 0 as libc::c_int;
    }
    if stype as libc::c_uint == SRC_CMDLINE as libc::c_int as libc::c_uint
        || stype as libc::c_uint == SRC_STDIN as libc::c_int as libc::c_uint
    {
        return do_add_srcfile(stype, src, 0 as *mut libc::c_char, thisfile);
    }
    path = find_source(
        src,
        &mut sbuf,
        &mut errno_val,
        (stype as libc::c_uint == SRC_EXTLIB as libc::c_int as libc::c_uint)
            as libc::c_int,
    );
    if path.is_null() {
        if !errcode.is_null() {
            *errcode = errno_val;
            return 0 as *mut SRCFILE;
        }
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"awkgram.y\0" as *const u8 as *const libc::c_char,
            2880 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            if stype as libc::c_uint != SRC_EXTLIB as libc::c_int as libc::c_uint {
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot open source file `%s' for reading: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                )
            } else {
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot open shared library `%s' for reading: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                )
            },
            src,
            if errno_val != 0 {
                strerror(errno_val)
            } else {
                dcgettext(
                    0 as *const libc::c_char,
                    b"reason unknown\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            },
        );
    }
    s = (*srcfiles).next;
    while s != srcfiles {
        if ((*s).stype as libc::c_uint == SRC_FILE as libc::c_int as libc::c_uint
            || (*s).stype as libc::c_uint == SRC_INC as libc::c_int as libc::c_uint
            || (*s).stype as libc::c_uint == SRC_EXTLIB as libc::c_int as libc::c_uint)
            && files_are_same(path, s) != 0
        {
            if stype as libc::c_uint == SRC_INC as libc::c_int as libc::c_uint
                || stype as libc::c_uint == SRC_EXTLIB as libc::c_int as libc::c_uint
            {
                if stype as libc::c_uint == SRC_INC as libc::c_int as libc::c_uint
                    && (*s).stype as libc::c_uint
                        == SRC_FILE as libc::c_int as libc::c_uint
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"awkgram.y\0" as *const u8 as *const libc::c_char,
                        2893 as libc::c_int,
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
                            b"cannot include `%s' and use it as a program file\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        src,
                    );
                }
                if do_flags as libc::c_uint
                    & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                        as libc::c_uint != 0
                {
                    let mut line: libc::c_int = sourceline;
                    if sourceline > 1 as libc::c_int && lasttok == 308 as libc::c_int {
                        line -= 1;
                        line;
                    }
                    lintwarn_ln(
                        line,
                        if stype as libc::c_uint
                            != SRC_EXTLIB as libc::c_int as libc::c_uint
                        {
                            dcgettext(
                                0 as *const libc::c_char,
                                b"already included source file `%s'\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            )
                        } else {
                            dcgettext(
                                0 as *const libc::c_char,
                                b"already loaded shared library `%s'\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            )
                        },
                        src,
                    );
                }
                free(path as *mut libc::c_void);
                if !already_included.is_null() {
                    *already_included = 1 as libc::c_int != 0;
                }
                return 0 as *mut SRCFILE;
            } else {
                if (*s).stype as libc::c_uint == SRC_INC as libc::c_int as libc::c_uint {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"awkgram.y\0" as *const u8 as *const libc::c_char,
                        2917 as libc::c_int,
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
                            b"cannot include `%s' and use it as a program file\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        src,
                    );
                }
                break;
            }
        } else {
            s = (*s).next;
        }
    }
    s = do_add_srcfile(stype, src, path, thisfile);
    (*s).sbuf = sbuf;
    (*s).mtime = sbuf.st_mtim.tv_sec;
    return s;
}
unsafe extern "C" fn include_source(
    mut file: *mut INSTRUCTION,
    mut srcfile_p: *mut *mut libc::c_void,
) -> bool {
    let mut s: *mut SRCFILE = 0 as *mut SRCFILE;
    let mut src: *mut libc::c_char = (*file).d.name;
    let mut errcode: libc::c_int = 0;
    let mut already_included: bool = false;
    *srcfile_p = 0 as *mut libc::c_void;
    if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0
        || do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint != 0
    {
        error_ln(
            (*file).source_line as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"@include is a gawk extension\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int != 0;
    }
    if strlen(src) == 0 as libc::c_int as libc::c_ulong {
        if do_flags as libc::c_uint
            & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                as libc::c_uint != 0
        {
            lintwarn_ln(
                (*file).source_line as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"empty filename after @include\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        return 1 as libc::c_int != 0;
    }
    s = add_srcfile(SRC_INC, src, sourcefile, &mut already_included, &mut errcode);
    if s.is_null() {
        if already_included {
            return 1 as libc::c_int != 0;
        }
        error_ln(
            (*file).source_line as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"cannot open source file `%s' for reading: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            src,
            if errcode != 0 {
                strerror(errcode)
            } else {
                dcgettext(
                    0 as *const libc::c_char,
                    b"reason unknown\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            },
        );
        return 0 as libc::c_int != 0;
    }
    (*sourcefile).srclines = sourceline;
    (*sourcefile).lexptr = lexptr;
    (*sourcefile).lexend = lexend;
    (*sourcefile).lexptr_begin = lexptr_begin;
    (*sourcefile).lexeme = lexeme;
    (*sourcefile).lasttok = lasttok;
    (*sourcefile).namespace = current_namespace;
    sourcefile = s;
    lexptr = 0 as *mut libc::c_char;
    sourceline = 0 as libc::c_int;
    source = 0 as *mut libc::c_char;
    lasttok = 0 as libc::c_int;
    lexeof = 0 as libc::c_int != 0;
    eof_warned = 0 as libc::c_int != 0;
    current_namespace = awk_namespace.as_ptr();
    *srcfile_p = s as *mut libc::c_void;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn load_library(
    mut file: *mut INSTRUCTION,
    mut srcfile_p: *mut *mut libc::c_void,
) -> bool {
    let mut s: *mut SRCFILE = 0 as *mut SRCFILE;
    let mut src: *mut libc::c_char = (*file).d.name;
    let mut errcode: libc::c_int = 0;
    let mut already_included: bool = false;
    *srcfile_p = 0 as *mut libc::c_void;
    if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0
        || do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint != 0
    {
        error_ln(
            (*file).source_line as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"@load is a gawk extension\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int != 0;
    }
    if strlen(src) == 0 as libc::c_int as libc::c_ulong {
        if do_flags as libc::c_uint
            & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                as libc::c_uint != 0
        {
            lintwarn_ln(
                (*file).source_line as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"empty filename after @load\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        return 1 as libc::c_int != 0;
    }
    if do_flags as libc::c_uint & DO_PRETTY_PRINT as libc::c_int as libc::c_uint != 0
        && do_flags as libc::c_uint & DO_PROFILE as libc::c_int as libc::c_uint == 0
    {
        s = do_add_srcfile(SRC_EXTLIB, src, src, sourcefile);
    } else {
        s = add_srcfile(
            SRC_EXTLIB,
            src,
            sourcefile,
            &mut already_included,
            &mut errcode,
        );
        if s.is_null() {
            if already_included {
                return 1 as libc::c_int != 0;
            }
            error_ln(
                (*file).source_line as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot open shared library `%s' for reading: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                src,
                if errcode != 0 {
                    strerror(errcode)
                } else {
                    dcgettext(
                        0 as *const libc::c_char,
                        b"reason unknown\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    )
                },
            );
            return 0 as libc::c_int != 0;
        }
        load_ext((*s).fullpath);
    }
    *srcfile_p = s as *mut libc::c_void;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn next_sourcefile() {
    static mut closefunc: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int> = None;
    if closefunc.is_none() {
        let mut cp: *mut libc::c_char = getenv(
            b"AWKREADFUNC\0" as *const u8 as *const libc::c_char,
        );
        if cp.is_null() {
            closefunc = Some(close as unsafe extern "C" fn(libc::c_int) -> libc::c_int);
        } else {
            closefunc = Some(
                one_line_close as unsafe extern "C" fn(libc::c_int) -> libc::c_int,
            );
        }
    }
    lexeof = 0 as libc::c_int != 0;
    eof_warned = 0 as libc::c_int != 0;
    (*sourcefile).srclines = sourceline;
    if (*sourcefile).fd > -(1 as libc::c_int) {
        if (*sourcefile).fd != fileno(stdin) {
            (Some(closefunc.unwrap())).unwrap()((*sourcefile).fd);
        }
        (*sourcefile).fd = -(1 as libc::c_int);
    }
    if !((*sourcefile).buf).is_null() {
        free((*sourcefile).buf as *mut libc::c_void);
        (*sourcefile).buf = 0 as *mut libc::c_char;
        (*sourcefile).lexptr_begin = 0 as *mut libc::c_char;
    }
    loop {
        sourcefile = (*sourcefile).next;
        if sourcefile.is_null() {
            break;
        }
        if sourcefile == srcfiles {
            return;
        }
        if (*sourcefile).stype as libc::c_uint
            != SRC_EXTLIB as libc::c_int as libc::c_uint
        {
            break;
        }
    }
    if !((*sourcefile).lexptr_begin).is_null() {
        lexptr = (*sourcefile).lexptr;
        lexend = (*sourcefile).lexend;
        lasttok = (*sourcefile).lasttok;
        lexptr_begin = (*sourcefile).lexptr_begin;
        lexeme = (*sourcefile).lexeme;
        sourceline = (*sourcefile).srclines;
        source = (*sourcefile).src;
        set_current_namespace((*sourcefile).namespace);
    } else {
        lexptr = 0 as *mut libc::c_char;
        sourceline = 0 as libc::c_int;
        source = 0 as *mut libc::c_char;
        lasttok = 0 as libc::c_int;
        set_current_namespace(awk_namespace.as_ptr());
    };
}
unsafe extern "C" fn get_src_buf() -> *mut libc::c_char {
    let mut n: libc::c_int = 0;
    let mut scan: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newfile: bool = false;
    let mut savelen: libc::c_int = 0;
    let mut sbuf: stat = stat {
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
    static mut readfunc: Option::<
        unsafe extern "C" fn(libc::c_int, *mut libc::c_void, size_t) -> ssize_t,
    > = None;
    if readfunc.is_none() {
        let mut cp: *mut libc::c_char = getenv(
            b"AWKREADFUNC\0" as *const u8 as *const libc::c_char,
        );
        if cp.is_null() {
            readfunc = ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        libc::c_int,
                        *mut libc::c_void,
                        size_t,
                    ) -> ssize_t,
                >,
                Option::<
                    unsafe extern "C" fn(
                        libc::c_int,
                        *mut libc::c_void,
                        size_t,
                    ) -> ssize_t,
                >,
            >(
                Some(
                    read
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut libc::c_void,
                            size_t,
                        ) -> ssize_t,
                ),
            );
        } else {
            readfunc = Some(
                read_one_line
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *mut libc::c_void,
                        size_t,
                    ) -> ssize_t,
            );
        }
    }
    newfile = 0 as libc::c_int != 0;
    if sourcefile == srcfiles {
        return 0 as *mut libc::c_char;
    }
    if (*sourcefile).stype as libc::c_uint == SRC_CMDLINE as libc::c_int as libc::c_uint
    {
        if (*sourcefile).bufsize == 0 as libc::c_int as libc::c_ulong {
            (*sourcefile).bufsize = strlen((*sourcefile).src);
            lexeme = (*sourcefile).src;
            lexptr_begin = lexeme;
            lexptr = lexptr_begin;
            lexend = lexptr.offset((*sourcefile).bufsize as isize);
            sourceline = 1 as libc::c_int;
            if (*sourcefile).bufsize == 0 as libc::c_int as libc::c_ulong {
                static mut warned: bool = 0 as libc::c_int != 0;
                if do_flags as libc::c_uint
                    & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                        as libc::c_uint != 0 && !warned
                {
                    warned = 1 as libc::c_int != 0;
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"awkgram.y\0" as *const u8 as *const libc::c_char,
                        3149 as libc::c_int,
                    );
                    (Some(lintfunc.unwrap()))
                        .unwrap()(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"empty program text on command line\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                lexeof = 1 as libc::c_int != 0;
            }
        } else if ((*sourcefile).buf).is_null()
            && *lexptr.offset(-(1 as libc::c_int as isize)) as libc::c_int != '\n' as i32
        {
            let mut offset: libc::c_int = 0;
            let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
            offset = lexptr.offset_from(lexeme) as libc::c_long as libc::c_int;
            scan = lexeme;
            while scan > lexptr_begin {
                if *scan as libc::c_int == '\n' as i32 {
                    scan = scan.offset(1);
                    scan;
                    break;
                } else {
                    scan = scan.offset(-1);
                    scan;
                }
            }
            savelen = lexptr.offset_from(scan) as libc::c_long as libc::c_int;
            buf = emalloc_real(
                (savelen + 1 as libc::c_int) as size_t,
                b"get_src_buf\0" as *const u8 as *const libc::c_char,
                b"buf\0" as *const u8 as *const libc::c_char,
                b"awkgram.y\0" as *const u8 as *const libc::c_char,
                3169 as libc::c_int,
            ) as *mut libc::c_char;
            memcpy(
                buf as *mut libc::c_void,
                scan as *const libc::c_void,
                savelen as libc::c_ulong,
            );
            thisline = buf;
            lexptr = buf.offset(savelen as isize);
            *lexptr = '\n' as i32 as libc::c_char;
            lexeme = lexptr.offset(-(offset as isize));
            lexptr_begin = buf;
            lexend = lexptr.offset(1 as libc::c_int as isize);
            (*sourcefile).buf = buf;
        } else {
            lexeof = 1 as libc::c_int != 0;
        }
        return lexptr;
    }
    if (*sourcefile).fd <= -(1 as libc::c_int) {
        let mut fd: libc::c_int = 0;
        let mut l: libc::c_int = 0;
        source = (*sourcefile).src;
        if source.is_null() {
            return 0 as *mut libc::c_char;
        }
        fd = srcopen(sourcefile);
        if fd <= -(1 as libc::c_int) {
            let mut in_0: *mut libc::c_char = 0 as *mut libc::c_char;
            in_0 = source;
            source = 0 as *mut libc::c_char;
            error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot open source file `%s' for reading: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                in_0,
                strerror(*__errno_location()),
            );
            errcount += 1;
            errcount;
            lexeof = 1 as libc::c_int != 0;
            return (*sourcefile).src;
        }
        (*sourcefile).fd = fd;
        l = optimal_bufsize(fd, &mut sbuf) as libc::c_int;
        if l < 128 as libc::c_int {
            l = 128 as libc::c_int;
        }
        (*sourcefile).bufsize = l as size_t;
        newfile = 1 as libc::c_int != 0;
        (*sourcefile)
            .buf = emalloc_real(
            (*sourcefile).bufsize,
            b"get_src_buf\0" as *const u8 as *const libc::c_char,
            b"sourcefile->buf\0" as *const u8 as *const libc::c_char,
            b"awkgram.y\0" as *const u8 as *const libc::c_char,
            3217 as libc::c_int,
        ) as *mut libc::c_char;
        memset(
            (*sourcefile).buf as *mut libc::c_void,
            '\0' as i32,
            (*sourcefile).bufsize,
        );
        lexeme = (*sourcefile).buf;
        lexptr_begin = lexeme;
        lexptr = lexptr_begin;
        savelen = 0 as libc::c_int;
        sourceline = 1 as libc::c_int;
        thisline = 0 as *mut libc::c_char;
    } else {
        let mut offset_0: libc::c_int = 0;
        scan = lexeme;
        while scan > lexptr_begin {
            if *scan as libc::c_int == '\n' as i32 {
                scan = scan.offset(1);
                scan;
                break;
            } else {
                scan = scan.offset(-1);
                scan;
            }
        }
        savelen = lexptr.offset_from(scan) as libc::c_long as libc::c_int;
        offset_0 = lexptr.offset_from(lexeme) as libc::c_long as libc::c_int;
        if savelen > 0 as libc::c_int {
            if savelen as libc::c_ulong
                > ((*sourcefile).bufsize).wrapping_div(2 as libc::c_int as libc::c_ulong)
            {
                (*sourcefile)
                    .bufsize = ((*sourcefile).bufsize as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
                (*sourcefile)
                    .buf = erealloc_real(
                    (*sourcefile).buf as *mut libc::c_void,
                    (*sourcefile).bufsize,
                    b"get_src_buf\0" as *const u8 as *const libc::c_char,
                    b"sourcefile->buf\0" as *const u8 as *const libc::c_char,
                    b"awkgram.y\0" as *const u8 as *const libc::c_char,
                    3247 as libc::c_int,
                ) as *mut libc::c_char;
                scan = ((*sourcefile).buf)
                    .offset(scan.offset_from(lexptr_begin) as libc::c_long as isize);
                lexptr_begin = (*sourcefile).buf;
            }
            thisline = lexptr_begin;
            memmove(
                thisline as *mut libc::c_void,
                scan as *const libc::c_void,
                savelen as libc::c_ulong,
            );
            lexptr = thisline.offset(savelen as isize);
            lexeme = lexptr.offset(-(offset_0 as isize));
        } else {
            savelen = 0 as libc::c_int;
            lexeme = lexptr_begin;
            lexptr = lexeme;
            thisline = 0 as *mut libc::c_char;
        }
    }
    n = (Some(readfunc.unwrap()))
        .unwrap()(
        (*sourcefile).fd,
        lexptr as *mut libc::c_void,
        ((*sourcefile).bufsize).wrapping_sub(savelen as libc::c_ulong),
    ) as libc::c_int;
    if n == -(1 as libc::c_int) {
        error(
            dcgettext(
                0 as *const libc::c_char,
                b"cannot read source file `%s': %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            source,
            strerror(*__errno_location()),
        );
        errcount += 1;
        errcount;
        lexeof = 1 as libc::c_int != 0;
    } else {
        lexend = lexptr.offset(n as isize);
        if n == 0 as libc::c_int {
            static mut warned_0: bool = 0 as libc::c_int != 0;
            if do_flags as libc::c_uint
                & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                    as libc::c_uint != 0 && newfile as libc::c_int != 0 && !warned_0
            {
                warned_0 = 1 as libc::c_int != 0;
                sourceline = 0 as libc::c_int;
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"awkgram.y\0" as *const u8 as *const libc::c_char,
                    3276 as libc::c_int,
                );
                (Some(lintfunc.unwrap()))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"source file `%s' is empty\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    source,
                );
            }
            lexeof = 1 as libc::c_int != 0;
        }
    }
    return (*sourcefile).buf;
}
unsafe extern "C" fn tokexpand() -> *mut libc::c_char {
    static mut toksize: size_t = 0;
    let mut tokoffset: libc::c_int = 0;
    if !tokstart.is_null() {
        tokoffset = tok.offset_from(tokstart) as libc::c_long as libc::c_int;
        toksize = (toksize as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        tokstart = erealloc_real(
            tokstart as *mut libc::c_void,
            toksize,
            b"tokexpand\0" as *const u8 as *const libc::c_char,
            b"tokstart\0" as *const u8 as *const libc::c_char,
            b"awkgram.y\0" as *const u8 as *const libc::c_char,
            3299 as libc::c_int,
        ) as *mut libc::c_char;
        tok = tokstart.offset(tokoffset as isize);
    } else {
        toksize = 60 as libc::c_int as size_t;
        tokstart = emalloc_real(
            toksize,
            b"tokexpand\0" as *const u8 as *const libc::c_char,
            b"tokstart\0" as *const u8 as *const libc::c_char,
            b"awkgram.y\0" as *const u8 as *const libc::c_char,
            3303 as libc::c_int,
        ) as *mut libc::c_char;
        tok = tokstart;
    }
    tokend = tokstart.offset(toksize as isize);
    return tok;
}
unsafe extern "C" fn check_bad_char(mut c: libc::c_int) {
    match c {
        7 | 8 | 12 | 10 | 13 | 9 => return,
        _ => {}
    }
    if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
        & _IScntrl as libc::c_int as libc::c_ushort as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"awkgram.y\0" as *const u8 as *const libc::c_char,
            3336 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"error: invalid character '\\%03o' in source code\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            c & 0xff as libc::c_int,
        );
    }
}
unsafe extern "C" fn nextc(mut check_for_bad: bool) -> libc::c_int {
    if gawk_mb_cur_max > 1 as libc::c_int {
        loop {
            if lexeof {
                return -(1000 as libc::c_int);
            }
            if lexptr.is_null() || lexptr >= lexend {
                if !(get_src_buf()).is_null() {
                    continue;
                }
                return -(2000 as libc::c_int);
            } else {
                cur_ring_idx = if cur_ring_idx
                    == 8 as libc::c_int * 16 as libc::c_int - 1 as libc::c_int
                {
                    0 as libc::c_int
                } else {
                    cur_ring_idx + 1 as libc::c_int
                };
                if cur_char_ring[cur_ring_idx as usize] as libc::c_int
                    == 0 as libc::c_int
                {
                    let mut idx: libc::c_int = 0;
                    let mut work_ring_idx: libc::c_int = cur_ring_idx;
                    let mut tmp_state: mbstate_t = mbstate_t {
                        __count: 0,
                        __value: C2RustUnnamed { __wch: 0 },
                    };
                    let mut mbclen: size_t = 0;
                    idx = 0 as libc::c_int;
                    while lexptr.offset(idx as isize) < lexend {
                        tmp_state = cur_mbstate;
                        mbclen = mbrlen(
                            lexptr,
                            (idx + 1 as libc::c_int) as size_t,
                            &mut tmp_state,
                        );
                        if mbclen == 1 as libc::c_int as libc::c_ulong
                            || mbclen == -(1 as libc::c_int) as size_t
                            || mbclen == 0 as libc::c_int as libc::c_ulong
                        {
                            cur_char_ring[work_ring_idx
                                as usize] = 1 as libc::c_int as libc::c_char;
                            break;
                        } else if mbclen == -(2 as libc::c_int) as size_t {
                            cur_char_ring[work_ring_idx
                                as usize] = (idx + 1 as libc::c_int) as libc::c_char;
                            work_ring_idx = if work_ring_idx
                                == 8 as libc::c_int * 16 as libc::c_int - 1 as libc::c_int
                            {
                                0 as libc::c_int
                            } else {
                                work_ring_idx + 1 as libc::c_int
                            };
                            idx += 1;
                            idx;
                        } else {
                            cur_char_ring[work_ring_idx
                                as usize] = mbclen as libc::c_char;
                            break;
                        }
                    }
                    cur_mbstate = tmp_state;
                    work_ring_idx = if work_ring_idx
                        == 8 as libc::c_int * 16 as libc::c_int - 1 as libc::c_int
                    {
                        0 as libc::c_int
                    } else {
                        work_ring_idx + 1 as libc::c_int
                    };
                    cur_char_ring[work_ring_idx
                        as usize] = 0 as libc::c_int as libc::c_char;
                }
                if check_for_bad as libc::c_int != 0
                    || *lexptr as libc::c_int == '\0' as i32
                {
                    check_bad_char(*lexptr as libc::c_int);
                }
                let fresh89 = lexptr;
                lexptr = lexptr.offset(1);
                return *fresh89 as libc::c_uchar as libc::c_int;
            }
        }
    } else {
        loop {
            if lexeof {
                return -(1000 as libc::c_int);
            }
            if !lexptr.is_null() && lexptr < lexend {
                if check_for_bad as libc::c_int != 0
                    || *lexptr as libc::c_int == '\0' as i32
                {
                    check_bad_char(*lexptr as libc::c_int);
                }
                let fresh90 = lexptr;
                lexptr = lexptr.offset(1);
                return *fresh90 as libc::c_uchar as libc::c_int;
            }
            if (get_src_buf()).is_null() {
                break;
            }
        }
        return -(2000 as libc::c_int);
    };
}
#[inline]
unsafe extern "C" fn pushback() {
    if gawk_mb_cur_max > 1 as libc::c_int {
        cur_ring_idx = if cur_ring_idx == 0 as libc::c_int {
            8 as libc::c_int * 16 as libc::c_int - 1 as libc::c_int
        } else {
            cur_ring_idx - 1 as libc::c_int
        };
    }
    if !lexeof && !lexptr.is_null() && lexptr > lexptr_begin {
        lexptr = lexptr.offset(-1);
        lexptr;
    } else {};
}
unsafe extern "C" fn get_comment(
    mut flag: commenttype,
    mut comment_instruction: *mut *mut INSTRUCTION,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut sl: libc::c_int = 0;
    let mut p1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
    tok = tokstart;
    let fresh91 = tok;
    tok = tok.offset(1);
    *fresh91 = '#' as i32 as libc::c_char;
    if tok == tokend {
        tokexpand();
    } else {};
    sl = sourceline;
    loop {
        loop {
            c = nextc(0 as libc::c_int != 0);
            if !(c != '\n' as i32 && c != -(1000 as libc::c_int)) {
                break;
            }
            if c != '\r' as i32 {
                let fresh92 = tok;
                tok = tok.offset(1);
                *fresh92 = c as libc::c_char;
                if tok == tokend {
                    tokexpand();
                } else {};
            }
        }
        if flag as libc::c_uint == EOL_COMMENT as libc::c_int as libc::c_uint {
            if c == '\n' as i32 {
                let fresh93 = tok;
                tok = tok.offset(1);
                *fresh93 = c as libc::c_char;
                if tok == tokend {
                    tokexpand();
                } else {};
            }
            break;
        } else {
            if !(c == '\n' as i32) {
                break;
            }
            let fresh94 = tok;
            tok = tok.offset(1);
            *fresh94 = c as libc::c_char;
            if tok == tokend {
                tokexpand();
            } else {};
            sourceline += 1;
            sourceline;
            loop {
                c = nextc(0 as libc::c_int != 0);
                if c == '\n' as i32 {
                    sourceline += 1;
                    sourceline;
                    let fresh95 = tok;
                    tok = tok.offset(1);
                    *fresh95 = c as libc::c_char;
                    if tok == tokend {
                        tokexpand();
                    } else {};
                }
                if !(c != -(1000 as libc::c_int)
                    && *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0)
                {
                    break;
                }
            }
            if c == -(1000 as libc::c_int) {
                break;
            }
            if c != '#' as i32 {
                pushback();
                sourceline -= 1;
                sourceline;
                break;
            } else {
                let fresh96 = tok;
                tok = tok.offset(1);
                *fresh96 = c as libc::c_char;
                if tok == tokend {
                    tokexpand();
                } else {};
            }
        }
    }
    p1 = tok.offset(-(1 as libc::c_int as isize));
    p2 = tok.offset(-(2 as libc::c_int as isize));
    while *p1 as libc::c_int == '\n' as i32 && *p2 as libc::c_int == '\n' as i32 {
        p1 = p1.offset(-1);
        p1;
        p2 = p2.offset(-1);
        p2;
        tok = tok.offset(-1);
        tok;
    }
    *comment_instruction = bcalloc(Op_comment, 1 as libc::c_int, sl);
    (**comment_instruction).d.name = source;
    (**comment_instruction)
        .d
        .dn = make_str_node(
        tokstart,
        tok.offset_from(tokstart) as libc::c_long as size_t,
        0 as libc::c_int,
    );
    (*(**comment_instruction).d.dn).sub.val.comtype = flag;
    return c;
}
unsafe extern "C" fn allow_newline(mut new_comment: *mut *mut INSTRUCTION) {
    let mut c: libc::c_int = 0;
    loop {
        c = nextc(1 as libc::c_int != 0);
        if c == -(1000 as libc::c_int) {
            pushback();
            break;
        } else {
            if c == '#' as i32 {
                if do_flags as libc::c_uint
                    & DO_PRETTY_PRINT as libc::c_int as libc::c_uint != 0
                    && do_flags as libc::c_uint
                        & DO_PROFILE as libc::c_int as libc::c_uint == 0
                {
                    c = get_comment(EOL_COMMENT, new_comment);
                } else {
                    loop {
                        c = nextc(0 as libc::c_int != 0);
                        if !(c != '\n' as i32 && c != -(1000 as libc::c_int)) {
                            break;
                        }
                    }
                }
                if c == -(1000 as libc::c_int) {
                    pushback();
                    break;
                }
            }
            if c == '\n' as i32 {
                sourceline += 1;
                sourceline;
            }
            if !(*(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0)
            {
                continue;
            }
            pushback();
            break;
        }
    };
}
unsafe extern "C" fn newline_eof() -> libc::c_int {
    if lasttok != 308 as libc::c_int {
        pushback();
        if do_flags as libc::c_uint
            & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                as libc::c_uint != 0 && !eof_warned
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"awkgram.y\0" as *const u8 as *const libc::c_char,
                3563 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"source file does not end in newline\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            eof_warned = 1 as libc::c_int != 0;
        }
        sourceline += 1;
        sourceline;
        return 308 as libc::c_int;
    }
    sourceline -= 1;
    sourceline;
    eof_warned = 0 as libc::c_int != 0;
    return 303 as libc::c_int;
}
unsafe extern "C" fn yylex() -> libc::c_int {
    let mut in_brack: libc::c_int = 0;
    let mut b_index: libc::c_int = 0;
    let mut cur_index: libc::c_int = 0;
    let mut current_block: u64;
    let mut c: libc::c_int = 0;
    let mut seen_e: bool = 0 as libc::c_int != 0;
    let mut seen_point: bool = 0 as libc::c_int != 0;
    let mut esc_seen: bool = false;
    let mut mid: libc::c_int = 0;
    let mut base: libc::c_int = 0;
    static mut did_newline: bool = 0 as libc::c_int != 0;
    let mut tokkey: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut inhex: bool = 0 as libc::c_int != 0;
    let mut intlstr: bool = 0 as libc::c_int != 0;
    let mut d: libc::c_double = 0.;
    let mut collecting_typed_regexp: bool = 0 as libc::c_int != 0;
    static mut qm_col_count: libc::c_int = 0 as libc::c_int;
    yylval = 0 as *mut libc::c_void as *mut INSTRUCTION;
    if lasttok == 272 as libc::c_int {
        lasttok = 0 as libc::c_int;
        return 272 as libc::c_int;
    }
    if lasttok == 303 as libc::c_int {
        return 0 as libc::c_int;
    }
    c = nextc(!want_regexp);
    if c == -(2000 as libc::c_int) {
        return 0 as libc::c_int;
    }
    if c == -(1000 as libc::c_int) {
        lasttok = newline_eof();
        return lasttok;
    }
    pushback();
    lexeme = lexptr;
    thisline = 0 as *mut libc::c_char;
    '_collect_regexp: loop {
        if want_regexp {
            in_brack = 0 as libc::c_int;
            b_index = -(1 as libc::c_int);
            cur_index = 0 as libc::c_int;
            want_regexp = 0 as libc::c_int != 0;
            tok = tokstart;
            current_block = 2370887241019905314;
            break;
        } else {
            loop {
                loop {
                    c = nextc(1 as libc::c_int != 0);
                    if !(c == ' ' as i32 || c == '\t' as i32 || c == '\r' as i32) {
                        break;
                    }
                }
                lexeme = if !lexptr.is_null() {
                    lexptr.offset(-(1 as libc::c_int as isize))
                } else {
                    lexptr
                };
                thisline = 0 as *mut libc::c_char;
                tok = tokstart;
                if !(gawk_mb_cur_max == 1 as libc::c_int
                    || cur_char_ring[cur_ring_idx as usize] as libc::c_int
                        == 1 as libc::c_int)
                {
                    current_block = 13837311639792312879;
                    break '_collect_regexp;
                }
                match c {
                    -2000 => return 0 as libc::c_int,
                    -1000 => {
                        lasttok = newline_eof();
                        return lasttok;
                    }
                    10 => {
                        sourceline += 1;
                        sourceline;
                        lasttok = 308 as libc::c_int;
                        return lasttok;
                    }
                    35 => {
                        yylval = 0 as *mut INSTRUCTION;
                        if do_flags as libc::c_uint
                            & DO_PRETTY_PRINT as libc::c_int as libc::c_uint != 0
                            && do_flags as libc::c_uint
                                & DO_PROFILE as libc::c_int as libc::c_uint == 0
                        {
                            let mut new_comment: *mut INSTRUCTION = 0
                                as *mut INSTRUCTION;
                            if lasttok == 308 as libc::c_int
                                || lasttok == 0 as libc::c_int
                            {
                                c = get_comment(BLOCK_COMMENT, &mut new_comment);
                            } else {
                                c = get_comment(EOL_COMMENT, &mut new_comment);
                            }
                            yylval = new_comment;
                            if c == -(1000 as libc::c_int) {
                                pushback();
                                lasttok = 308 as libc::c_int;
                                return lasttok;
                            }
                        } else {
                            loop {
                                c = nextc(0 as libc::c_int != 0);
                                if !(c != '\n' as i32) {
                                    break;
                                }
                                if c == -(1000 as libc::c_int) {
                                    lasttok = newline_eof();
                                    return lasttok;
                                }
                            }
                        }
                        sourceline += 1;
                        sourceline;
                        lasttok = 308 as libc::c_int;
                        return lasttok;
                    }
                    64 => {
                        c = nextc(1 as libc::c_int != 0);
                        if c == '/' as i32 {
                            current_block = 17688141731389699982;
                            break;
                        } else {
                            current_block = 1852451392920375136;
                            break;
                        }
                    }
                    92 => {
                        c = nextc(1 as libc::c_int != 0);
                        if c == '\r' as i32 {
                            c = nextc(1 as libc::c_int != 0);
                        }
                        if c == '\n' as i32 {
                            sourceline += 1;
                            sourceline;
                        } else {
                            yyerror(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"backslash not last character on line\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                            lasttok = 303 as libc::c_int;
                            return lasttok;
                        }
                    }
                    63 => {
                        qm_col_count += 1;
                        qm_col_count;
                        current_block = 13191286796217823901;
                        break '_collect_regexp;
                    }
                    58 => {
                        current_block = 13191286796217823901;
                        break '_collect_regexp;
                    }
                    41 => {
                        in_parens -= 1;
                        in_parens;
                        lasttok = c;
                        return lasttok;
                    }
                    40 => {
                        in_parens += 1;
                        in_parens;
                        lasttok = c;
                        return lasttok;
                    }
                    36 => {
                        yylval = bcalloc(Op_field_spec, 1 as libc::c_int, sourceline);
                        lasttok = c;
                        return lasttok;
                    }
                    123 => {
                        in_braces += 1;
                        if in_braces == 1 as libc::c_int {
                            firstline = sourceline;
                        }
                        current_block = 6570067857177716340;
                        break '_collect_regexp;
                    }
                    59 | 44 | 91 => {
                        current_block = 6570067857177716340;
                        break '_collect_regexp;
                    }
                    93 => {
                        c = nextc(1 as libc::c_int != 0);
                        pushback();
                        if c == '[' as i32 {
                            if do_flags as libc::c_uint
                                & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0
                            {
                                (set_loc
                                    as unsafe extern "C" fn(
                                        *const libc::c_char,
                                        libc::c_int,
                                    ) -> ())(
                                    b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                    3880 as libc::c_int,
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
                                        b"multidimensional arrays are a gawk extension\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            if do_flags as libc::c_uint
                                & DO_LINT_EXTENSIONS as libc::c_int as libc::c_uint != 0
                            {
                                (set_loc
                                    as unsafe extern "C" fn(
                                        *const libc::c_char,
                                        libc::c_int,
                                    ) -> ())(
                                    b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                    3882 as libc::c_int,
                                );
                                (Some(lintfunc.unwrap()))
                                    .unwrap()(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"multidimensional arrays are a gawk extension\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            yylval = bcalloc(Op_sub_array, 1 as libc::c_int, sourceline);
                            lasttok = ']' as i32;
                        } else {
                            yylval = bcalloc(Op_subscript, 1 as libc::c_int, sourceline);
                            lasttok = 272 as libc::c_int;
                        }
                        return ']' as i32;
                    }
                    42 => {
                        c = nextc(1 as libc::c_int != 0);
                        if c == '=' as i32 {
                            yylval = bcalloc(
                                Op_assign_times,
                                1 as libc::c_int,
                                sourceline,
                            );
                            lasttok = 268 as libc::c_int;
                            return lasttok;
                        } else if do_flags as libc::c_uint
                            & DO_POSIX as libc::c_int as libc::c_uint != 0
                        {
                            pushback();
                            yylval = bcalloc(Op_times, 1 as libc::c_int, sourceline);
                            lasttok = '*' as i32;
                            return lasttok;
                        } else if c == '*' as i32 {
                            static mut did_warn_op: bool = 0 as libc::c_int != 0;
                            static mut did_warn_assgn: bool = 0 as libc::c_int != 0;
                            if nextc(1 as libc::c_int != 0) == '=' as i32 {
                                if !did_warn_assgn {
                                    did_warn_assgn = 1 as libc::c_int != 0;
                                    if do_flags as libc::c_uint
                                        & (DO_LINT_INVALID as libc::c_int
                                            | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                    {
                                        (set_loc
                                            as unsafe extern "C" fn(
                                                *const libc::c_char,
                                                libc::c_int,
                                            ) -> ())(
                                            b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                            3907 as libc::c_int,
                                        );
                                        (Some(lintfunc.unwrap()))
                                            .unwrap()(
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"POSIX does not allow operator `%s'\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            b"**=\0" as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    if do_flags as libc::c_uint
                                        & DO_LINT_OLD as libc::c_int as libc::c_uint != 0
                                    {
                                        (set_loc
                                            as unsafe extern "C" fn(
                                                *const libc::c_char,
                                                libc::c_int,
                                            ) -> ())(
                                            b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                            3909 as libc::c_int,
                                        );
                                        (Some(lintfunc.unwrap()))
                                            .unwrap()(
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"operator `%s' is not supported in old awk\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            b"**=\0" as *const u8 as *const libc::c_char,
                                        );
                                    }
                                }
                                yylval = bcalloc(
                                    Op_assign_exp,
                                    1 as libc::c_int,
                                    sourceline,
                                );
                                return 268 as libc::c_int;
                            } else {
                                pushback();
                                if !did_warn_op {
                                    did_warn_op = 1 as libc::c_int != 0;
                                    if do_flags as libc::c_uint
                                        & (DO_LINT_INVALID as libc::c_int
                                            | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                    {
                                        (set_loc
                                            as unsafe extern "C" fn(
                                                *const libc::c_char,
                                                libc::c_int,
                                            ) -> ())(
                                            b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                            3918 as libc::c_int,
                                        );
                                        (Some(lintfunc.unwrap()))
                                            .unwrap()(
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"POSIX does not allow operator `%s'\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            b"**\0" as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    if do_flags as libc::c_uint
                                        & DO_LINT_OLD as libc::c_int as libc::c_uint != 0
                                    {
                                        (set_loc
                                            as unsafe extern "C" fn(
                                                *const libc::c_char,
                                                libc::c_int,
                                            ) -> ())(
                                            b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                            3920 as libc::c_int,
                                        );
                                        (Some(lintfunc.unwrap()))
                                            .unwrap()(
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"operator `%s' is not supported in old awk\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            b"**\0" as *const u8 as *const libc::c_char,
                                        );
                                    }
                                }
                                yylval = bcalloc(Op_exp, 1 as libc::c_int, sourceline);
                                lasttok = '^' as i32;
                                return lasttok;
                            }
                        }
                        pushback();
                        yylval = bcalloc(Op_times, 1 as libc::c_int, sourceline);
                        lasttok = '*' as i32;
                        return lasttok;
                    }
                    47 => {
                        if nextc(0 as libc::c_int != 0) == '=' as i32 {
                            pushback();
                            lasttok = 309 as libc::c_int;
                            return lasttok;
                        }
                        pushback();
                        yylval = bcalloc(Op_quotient, 1 as libc::c_int, sourceline);
                        lasttok = '/' as i32;
                        return lasttok;
                    }
                    37 => {
                        if nextc(1 as libc::c_int != 0) == '=' as i32 {
                            yylval = bcalloc(
                                Op_assign_mod,
                                1 as libc::c_int,
                                sourceline,
                            );
                            lasttok = 268 as libc::c_int;
                            return lasttok;
                        }
                        pushback();
                        yylval = bcalloc(Op_mod, 1 as libc::c_int, sourceline);
                        lasttok = '%' as i32;
                        return lasttok;
                    }
                    94 => {
                        static mut did_warn_op_0: bool = 0 as libc::c_int != 0;
                        static mut did_warn_assgn_0: bool = 0 as libc::c_int != 0;
                        if nextc(1 as libc::c_int != 0) == '=' as i32 {
                            if do_flags as libc::c_uint
                                & DO_LINT_OLD as libc::c_int as libc::c_uint != 0
                                && !did_warn_assgn_0
                            {
                                did_warn_assgn_0 = 1 as libc::c_int != 0;
                                (set_loc
                                    as unsafe extern "C" fn(
                                        *const libc::c_char,
                                        libc::c_int,
                                    ) -> ())(
                                    b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                    3955 as libc::c_int,
                                );
                                (Some(lintfunc.unwrap()))
                                    .unwrap()(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"operator `%s' is not supported in old awk\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    b"^=\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            yylval = bcalloc(
                                Op_assign_exp,
                                1 as libc::c_int,
                                sourceline,
                            );
                            lasttok = 268 as libc::c_int;
                            return lasttok;
                        }
                        pushback();
                        if do_flags as libc::c_uint
                            & DO_LINT_OLD as libc::c_int as libc::c_uint != 0
                            && !did_warn_op_0
                        {
                            did_warn_op_0 = 1 as libc::c_int != 0;
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    libc::c_int,
                                ) -> ())(
                                b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                3963 as libc::c_int,
                            );
                            (Some(lintfunc.unwrap()))
                                .unwrap()(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"operator `%s' is not supported in old awk\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                b"^\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        yylval = bcalloc(Op_exp, 1 as libc::c_int, sourceline);
                        lasttok = '^' as i32;
                        return lasttok;
                    }
                    43 => {
                        c = nextc(1 as libc::c_int != 0);
                        if c == '=' as i32 {
                            yylval = bcalloc(
                                Op_assign_plus,
                                1 as libc::c_int,
                                sourceline,
                            );
                            lasttok = 268 as libc::c_int;
                            return lasttok;
                        }
                        if c == '+' as i32 {
                            yylval = bcalloc(Op_symbol, 1 as libc::c_int, sourceline);
                            lasttok = 299 as libc::c_int;
                            return lasttok;
                        }
                        pushback();
                        yylval = bcalloc(Op_plus, 1 as libc::c_int, sourceline);
                        lasttok = '+' as i32;
                        return lasttok;
                    }
                    33 => {
                        c = nextc(1 as libc::c_int != 0);
                        if c == '=' as i32 {
                            yylval = bcalloc(Op_notequal, 1 as libc::c_int, sourceline);
                            lasttok = 265 as libc::c_int;
                            return lasttok;
                        }
                        if c == '~' as i32 {
                            yylval = bcalloc(Op_nomatch, 1 as libc::c_int, sourceline);
                            lasttok = 270 as libc::c_int;
                            return lasttok;
                        }
                        pushback();
                        yylval = bcalloc(Op_symbol, 1 as libc::c_int, sourceline);
                        lasttok = '!' as i32;
                        return lasttok;
                    }
                    60 => {
                        if nextc(1 as libc::c_int != 0) == '=' as i32 {
                            yylval = bcalloc(Op_leq, 1 as libc::c_int, sourceline);
                            lasttok = 265 as libc::c_int;
                            return lasttok;
                        }
                        yylval = bcalloc(Op_less, 1 as libc::c_int, sourceline);
                        pushback();
                        lasttok = '<' as i32;
                        return lasttok;
                    }
                    61 => {
                        if nextc(1 as libc::c_int != 0) == '=' as i32 {
                            yylval = bcalloc(Op_equal, 1 as libc::c_int, sourceline);
                            lasttok = 265 as libc::c_int;
                            return lasttok;
                        }
                        yylval = bcalloc(Op_assign, 1 as libc::c_int, sourceline);
                        pushback();
                        lasttok = 269 as libc::c_int;
                        return lasttok;
                    }
                    62 => {
                        c = nextc(1 as libc::c_int != 0);
                        if c == '=' as i32 {
                            yylval = bcalloc(Op_geq, 1 as libc::c_int, sourceline);
                            lasttok = 265 as libc::c_int;
                            return lasttok;
                        } else if c == '>' as i32 {
                            yylval = bcalloc(Op_symbol, 1 as libc::c_int, sourceline);
                            (*yylval)
                                .d
                                .dl = redirect_append as libc::c_int as libc::c_long;
                            lasttok = 266 as libc::c_int;
                            return lasttok;
                        }
                        pushback();
                        if in_print as libc::c_int != 0 && in_parens == 0 as libc::c_int
                        {
                            yylval = bcalloc(Op_symbol, 1 as libc::c_int, sourceline);
                            (*yylval)
                                .d
                                .dl = redirect_output as libc::c_int as libc::c_long;
                            lasttok = 266 as libc::c_int;
                            return lasttok;
                        }
                        yylval = bcalloc(Op_greater, 1 as libc::c_int, sourceline);
                        lasttok = '>' as i32;
                        return lasttok;
                    }
                    126 => {
                        yylval = bcalloc(Op_match, 1 as libc::c_int, sourceline);
                        lasttok = 270 as libc::c_int;
                        return lasttok;
                    }
                    125 => {
                        if did_newline {
                            did_newline = 0 as libc::c_int != 0;
                            in_braces -= 1;
                            if in_braces == 0 as libc::c_int {
                                lastline = sourceline;
                            }
                            lasttok = c;
                            return lasttok;
                        }
                        did_newline = 1 as libc::c_int != 0;
                        lexptr = lexptr.offset(-1);
                        lexptr;
                        lasttok = 308 as libc::c_int;
                        return lasttok;
                    }
                    34 => {
                        current_block = 6256541998046792876;
                        break '_collect_regexp;
                    }
                    45 => {
                        c = nextc(1 as libc::c_int != 0);
                        if c == '=' as i32 {
                            yylval = bcalloc(
                                Op_assign_minus,
                                1 as libc::c_int,
                                sourceline,
                            );
                            lasttok = 268 as libc::c_int;
                            return lasttok;
                        }
                        if c == '-' as i32 {
                            yylval = bcalloc(Op_symbol, 1 as libc::c_int, sourceline);
                            lasttok = 300 as libc::c_int;
                            return lasttok;
                        }
                        pushback();
                        yylval = bcalloc(Op_minus, 1 as libc::c_int, sourceline);
                        lasttok = '-' as i32;
                        return lasttok;
                    }
                    46 => {
                        c = nextc(1 as libc::c_int != 0);
                        pushback();
                        if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                            == 0
                        {
                            lasttok = '.' as i32;
                            return lasttok;
                        } else {
                            c = '.' as i32;
                        }
                        current_block = 11475295656646479480;
                        break '_collect_regexp;
                    }
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                        current_block = 11475295656646479480;
                        break '_collect_regexp;
                    }
                    38 => {
                        c = nextc(1 as libc::c_int != 0);
                        if c == '&' as i32 {
                            yylval = bcalloc(Op_and, 1 as libc::c_int, sourceline);
                            let mut new_comment_1: *mut INSTRUCTION = 0
                                as *mut INSTRUCTION;
                            allow_newline(&mut new_comment_1);
                            (*yylval).comment = new_comment_1;
                            lasttok = 297 as libc::c_int;
                            return lasttok;
                        }
                        pushback();
                        yylval = bcalloc(Op_symbol, 1 as libc::c_int, sourceline);
                        lasttok = '&' as i32;
                        return lasttok;
                    }
                    124 => {
                        c = nextc(1 as libc::c_int != 0);
                        if c == '|' as i32 {
                            yylval = bcalloc(Op_or, 1 as libc::c_int, sourceline);
                            let mut new_comment_2: *mut INSTRUCTION = 0
                                as *mut INSTRUCTION;
                            allow_newline(&mut new_comment_2);
                            (*yylval).comment = new_comment_2;
                            lasttok = 298 as libc::c_int;
                            return lasttok;
                        } else if do_flags as libc::c_uint
                            & DO_TRADITIONAL as libc::c_int as libc::c_uint == 0
                            && c == '&' as i32
                        {
                            yylval = bcalloc(Op_symbol, 1 as libc::c_int, sourceline);
                            (*yylval)
                                .d
                                .dl = redirect_twoway as libc::c_int as libc::c_long;
                            lasttok = if in_print as libc::c_int != 0
                                && in_parens == 0 as libc::c_int
                            {
                                266 as libc::c_int
                            } else {
                                267 as libc::c_int
                            };
                            return lasttok;
                        }
                        pushback();
                        if in_print as libc::c_int != 0 && in_parens == 0 as libc::c_int
                        {
                            yylval = bcalloc(Op_symbol, 1 as libc::c_int, sourceline);
                            (*yylval)
                                .d
                                .dl = redirect_pipe as libc::c_int as libc::c_long;
                            lasttok = 266 as libc::c_int;
                            return lasttok;
                        } else {
                            yylval = bcalloc(Op_symbol, 1 as libc::c_int, sourceline);
                            (*yylval)
                                .d
                                .dl = redirect_pipein as libc::c_int as libc::c_long;
                            lasttok = 267 as libc::c_int;
                            return lasttok;
                        }
                    }
                    _ => {
                        current_block = 13837311639792312879;
                        break '_collect_regexp;
                    }
                }
            }
            match current_block {
                1852451392920375136 => {
                    pushback();
                    at_seen += 1;
                    at_seen;
                    lasttok = '@' as i32;
                    return lasttok;
                }
                _ => {
                    want_regexp = 1 as libc::c_int != 0;
                    collecting_typed_regexp = 1 as libc::c_int != 0;
                }
            }
        }
    }
    match current_block {
        11475295656646479480 => {
            loop {
                let mut gotnumber: bool = 0 as libc::c_int != 0;
                let fresh102 = tok;
                tok = tok.offset(1);
                *fresh102 = c as libc::c_char;
                if tok == tokend {
                    tokexpand();
                } else {};
                match c {
                    120 | 88 => {
                        if do_flags as libc::c_uint
                            & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0
                        {
                            current_block = 11252498884801241259;
                        } else if tok == tokstart.offset(2 as libc::c_int as isize) {
                            let mut peek_0: libc::c_int = nextc(1 as libc::c_int != 0);
                            if *(*__ctype_b_loc()).offset(peek_0 as isize) as libc::c_int
                                & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                            {
                                inhex = 1 as libc::c_int != 0;
                                pushback();
                                current_block = 8145001134476292673;
                            } else {
                                pushback();
                                current_block = 11252498884801241259;
                            }
                        } else {
                            current_block = 8145001134476292673;
                        }
                    }
                    46 => {
                        if seen_point as libc::c_int != 0 || seen_e as libc::c_int != 0 {
                            gotnumber = 1 as libc::c_int != 0;
                        } else {
                            seen_point = 1 as libc::c_int != 0;
                        }
                        current_block = 8145001134476292673;
                    }
                    101 | 69 => {
                        if inhex {
                            current_block = 8145001134476292673;
                        } else {
                            if seen_e {
                                gotnumber = 1 as libc::c_int != 0;
                            } else {
                                seen_e = 1 as libc::c_int != 0;
                                c = nextc(1 as libc::c_int != 0);
                                if c == '-' as i32 || c == '+' as i32 {
                                    let mut c2: libc::c_int = nextc(1 as libc::c_int != 0);
                                    if *(*__ctype_b_loc()).offset(c2 as isize) as libc::c_int
                                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                        != 0
                                    {
                                        let fresh103 = tok;
                                        tok = tok.offset(1);
                                        *fresh103 = c as libc::c_char;
                                        if tok == tokend {
                                            tokexpand();
                                        } else {};
                                        let fresh104 = tok;
                                        tok = tok.offset(1);
                                        *fresh104 = c2 as libc::c_char;
                                        if tok == tokend {
                                            tokexpand();
                                        } else {};
                                    } else {
                                        pushback();
                                        pushback();
                                        pushback();
                                    }
                                } else if *(*__ctype_b_loc()).offset(c as isize)
                                    as libc::c_int
                                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                    == 0
                                {
                                    pushback();
                                    pushback();
                                } else {
                                    pushback();
                                }
                            }
                            current_block = 8145001134476292673;
                        }
                    }
                    97 | 65 | 98 | 66 | 99 | 67 | 68 | 100 | 102 | 70 => {
                        if do_flags as libc::c_uint
                            & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0
                            || !inhex
                        {
                            current_block = 11252498884801241259;
                        } else {
                            current_block = 8145001134476292673;
                        }
                    }
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                        current_block = 8145001134476292673;
                    }
                    _ => {
                        current_block = 11252498884801241259;
                    }
                }
                match current_block {
                    11252498884801241259 => {
                        gotnumber = 1 as libc::c_int != 0;
                    }
                    _ => {}
                }
                if gotnumber {
                    break;
                }
                c = nextc(1 as libc::c_int != 0);
            }
            pushback();
            let fresh105 = tok;
            tok = tok.offset(1);
            *fresh105 = '\0' as i32 as libc::c_char;
            if tok == tokend {
                tokexpand();
            } else {};
            yylval = bcalloc(Op_push_i, 1 as libc::c_int, sourceline);
            base = 10 as libc::c_int;
            if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint
                == 0
            {
                base = get_numbase(
                    tokstart,
                    (strlen(tokstart)).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    0 as libc::c_int != 0,
                );
                if do_flags as libc::c_uint
                    & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                        as libc::c_uint != 0
                {
                    if base == 8 as libc::c_int {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"awkgram.y\0" as *const u8 as *const libc::c_char,
                            4237 as libc::c_int,
                        );
                        (Some(lintfunc.unwrap()))
                            .unwrap()(
                            b"numeric constant `%.*s' treated as octal\0" as *const u8
                                as *const libc::c_char,
                            strlen(tokstart) as libc::c_int - 1 as libc::c_int,
                            tokstart,
                        );
                    } else if base == 16 as libc::c_int {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"awkgram.y\0" as *const u8 as *const libc::c_char,
                            4240 as libc::c_int,
                        );
                        (Some(lintfunc.unwrap()))
                            .unwrap()(
                            b"numeric constant `%.*s' treated as hexadecimal\0"
                                as *const u8 as *const libc::c_char,
                            strlen(tokstart) as libc::c_int - 1 as libc::c_int,
                            tokstart,
                        );
                    }
                }
            }
            if base != 10 as libc::c_int {
                d = nondec2awknum(
                    tokstart,
                    (strlen(tokstart)).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    0 as *mut *mut libc::c_char,
                );
            } else {
                d = atof(tokstart);
            }
            (*yylval)
                .d
                .dn = set_profile_text(
                make_number.unwrap()(d),
                tokstart,
                (strlen(tokstart)).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            if d <= 2147483647 as libc::c_int as libc::c_double
                && d
                    >= (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_double && d == d as int32_t as libc::c_double
            {
                (*(*yylval).d.dn)
                    .flags = ::std::mem::transmute::<
                    libc::c_uint,
                    flagvals,
                >(
                    (*(*yylval).d.dn).flags as libc::c_uint
                        | NUMINT as libc::c_int as libc::c_uint,
                );
            }
            lasttok = 262 as libc::c_int;
            return lasttok;
        }
        13837311639792312879 => {
            if !is_letter(c) {
                yyerror(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid char '%c' in expression\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    c,
                );
                lasttok = 303 as libc::c_int;
                return lasttok;
            }
            if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint
                == 0 && c == '_' as i32 && lasttok != '$' as i32
            {
                c = nextc(1 as libc::c_int != 0);
                if c == '"' as i32 {
                    intlstr = 1 as libc::c_int != 0;
                    current_block = 6256541998046792876;
                } else {
                    pushback();
                    c = '_' as i32;
                    current_block = 15508192053828812388;
                }
            } else {
                current_block = 15508192053828812388;
            }
            match current_block {
                6256541998046792876 => {}
                _ => {
                    tok = tokstart;
                    while c != -(1000 as libc::c_int)
                        && is_identchar(c) as libc::c_int != 0
                    {
                        let fresh106 = tok;
                        tok = tok.offset(1);
                        *fresh106 = c as libc::c_char;
                        if tok == tokend {
                            tokexpand();
                        } else {};
                        c = nextc(1 as libc::c_int != 0);
                        if do_flags as libc::c_uint
                            & DO_TRADITIONAL as libc::c_int as libc::c_uint == 0
                            && c == ':' as i32
                        {
                            let mut peek_1: libc::c_int = nextc(1 as libc::c_int != 0);
                            if peek_1 == ':' as i32 {
                                let fresh107 = tok;
                                tok = tok.offset(1);
                                *fresh107 = c as libc::c_char;
                                if tok == tokend {
                                    tokexpand();
                                } else {};
                                let fresh108 = tok;
                                tok = tok.offset(1);
                                *fresh108 = c as libc::c_char;
                                if tok == tokend {
                                    tokexpand();
                                } else {};
                                c = nextc(1 as libc::c_int != 0);
                            } else {
                                pushback();
                            }
                        }
                    }
                    let fresh109 = tok;
                    tok = tok.offset(1);
                    *fresh109 = '\0' as i32 as libc::c_char;
                    if tok == tokend {
                        tokexpand();
                    } else {};
                    pushback();
                    validate_qualified_name(tokstart);
                    mid = check_qualified_special(tokstart);
                    if mid >= 0 as libc::c_int {
                        static mut warntab: [libc::c_int; 71] = [0; 71];
                        let mut class: libc::c_int = tokentab[mid as usize].class;
                        match class {
                            305 | 304 | 306 | 307 => {
                                if lasttok != '@' as i32 {
                                    current_block = 12777531360084369386;
                                } else {
                                    current_block = 763911284580919563;
                                }
                            }
                            _ => {
                                current_block = 763911284580919563;
                            }
                        }
                        match current_block {
                            12777531360084369386 => {}
                            _ => {
                                if tokentab[mid as usize].flags
                                    & 0x400 as libc::c_int as libc::c_uint
                                    != 0 as libc::c_int as libc::c_uint
                                {
                                    let mut f: *mut NODE = 0 as *mut NODE;
                                    match want_param_names as libc::c_uint {
                                        0 => {
                                            current_block = 12777531360084369386;
                                        }
                                        1 => {
                                            current_block = 10747673418851461431;
                                            match current_block {
                                                10020207588386981035 => {
                                                    r_fatal(
                                                        b"internal error: file %s, line %d: bad value %d for want_param_names\0"
                                                            as *const u8 as *const libc::c_char,
                                                        b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                                        4401 as libc::c_int,
                                                        want_param_names as libc::c_int,
                                                    );
                                                    current_block = 16055061654577082179;
                                                }
                                                _ => {
                                                    f = lookup(tokstart);
                                                    if !f.is_null() {
                                                        if (*f).type_0 as libc::c_uint
                                                            == Node_builtin_func as libc::c_int as libc::c_uint
                                                        {
                                                            current_block = 16055061654577082179;
                                                        } else {
                                                            current_block = 12777531360084369386;
                                                        }
                                                    } else {
                                                        current_block = 16055061654577082179;
                                                    }
                                                }
                                            }
                                        }
                                        2 => {
                                            current_block = 16055061654577082179;
                                        }
                                        _ => {
                                            current_block = 10020207588386981035;
                                            match current_block {
                                                10020207588386981035 => {
                                                    r_fatal(
                                                        b"internal error: file %s, line %d: bad value %d for want_param_names\0"
                                                            as *const u8 as *const libc::c_char,
                                                        b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                                        4401 as libc::c_int,
                                                        want_param_names as libc::c_int,
                                                    );
                                                    current_block = 16055061654577082179;
                                                }
                                                _ => {
                                                    f = lookup(tokstart);
                                                    if !f.is_null() {
                                                        if (*f).type_0 as libc::c_uint
                                                            == Node_builtin_func as libc::c_int as libc::c_uint
                                                        {
                                                            current_block = 16055061654577082179;
                                                        } else {
                                                            current_block = 12777531360084369386;
                                                        }
                                                    } else {
                                                        current_block = 16055061654577082179;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    current_block = 16055061654577082179;
                                }
                                match current_block {
                                    12777531360084369386 => {}
                                    _ => {
                                        if do_flags as libc::c_uint
                                            & (DO_LINT_INVALID as libc::c_int
                                                | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                        {
                                            if do_flags as libc::c_uint
                                                & DO_LINT_EXTENSIONS as libc::c_int as libc::c_uint != 0
                                                && tokentab[mid as usize].flags
                                                    & 0x400 as libc::c_int as libc::c_uint
                                                    != 0 as libc::c_int as libc::c_uint
                                                && warntab[mid as usize] & 0x400 as libc::c_int
                                                    == 0 as libc::c_int
                                            {
                                                (set_loc
                                                    as unsafe extern "C" fn(
                                                        *const libc::c_char,
                                                        libc::c_int,
                                                    ) -> ())(
                                                    b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                                    4408 as libc::c_int,
                                                );
                                                (Some(lintfunc.unwrap()))
                                                    .unwrap()(
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"`%s' is a gawk extension\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    ),
                                                    tokentab[mid as usize].operator,
                                                );
                                                warntab[mid as usize] |= 0x400 as libc::c_int;
                                            }
                                            if tokentab[mid as usize].flags
                                                & 0x200 as libc::c_int as libc::c_uint
                                                != 0 as libc::c_int as libc::c_uint
                                                && warntab[mid as usize] & 0x200 as libc::c_int
                                                    == 0 as libc::c_int
                                            {
                                                (set_loc
                                                    as unsafe extern "C" fn(
                                                        *const libc::c_char,
                                                        libc::c_int,
                                                    ) -> ())(
                                                    b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                                    4413 as libc::c_int,
                                                );
                                                (Some(lintfunc.unwrap()))
                                                    .unwrap()(
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"POSIX does not allow `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    ),
                                                    tokentab[mid as usize].operator,
                                                );
                                                warntab[mid as usize] |= 0x200 as libc::c_int;
                                            }
                                        }
                                        if do_flags as libc::c_uint
                                            & DO_LINT_OLD as libc::c_int as libc::c_uint != 0
                                            && tokentab[mid as usize].flags
                                                & 0x100 as libc::c_int as libc::c_uint
                                                != 0 as libc::c_int as libc::c_uint
                                            && warntab[mid as usize] & 0x100 as libc::c_int
                                                == 0 as libc::c_int
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const libc::c_char,
                                                    libc::c_int,
                                                ) -> ())(
                                                b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                                4421 as libc::c_int,
                                            );
                                            (Some(lintfunc.unwrap()))
                                                .unwrap()(
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"`%s' is not supported in old awk\0" as *const u8
                                                        as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                                tokentab[mid as usize].operator,
                                            );
                                            warntab[mid as usize] |= 0x100 as libc::c_int;
                                        }
                                        if tokentab[mid as usize].flags
                                            & 0x800 as libc::c_int as libc::c_uint
                                            != 0 as libc::c_int as libc::c_uint
                                        {
                                            break_allowed += 1;
                                            break_allowed;
                                        }
                                        if tokentab[mid as usize].flags
                                            & 0x1000 as libc::c_int as libc::c_uint
                                            != 0 as libc::c_int as libc::c_uint
                                        {
                                            continue_allowed += 1;
                                            continue_allowed;
                                        }
                                        match class {
                                            307 => {
                                                want_namespace = 1 as libc::c_int != 0;
                                                current_block = 8523145309031438996;
                                            }
                                            304 | 306 => {
                                                current_block = 8523145309031438996;
                                            }
                                            305 => {
                                                if in_main_context() != 0 {
                                                    current_block = 12777531360084369386;
                                                } else {
                                                    tokkey = emalloc_real(
                                                        (tok.offset_from(tokstart) as libc::c_long
                                                            + 1 as libc::c_int as libc::c_long) as size_t,
                                                        b"yylex\0" as *const u8 as *const libc::c_char,
                                                        b"tokkey\0" as *const u8 as *const libc::c_char,
                                                        b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                                        4442 as libc::c_int,
                                                    ) as *mut libc::c_char;
                                                    *tokkey
                                                        .offset(
                                                            0 as libc::c_int as isize,
                                                        ) = '@' as i32 as libc::c_char;
                                                    memcpy(
                                                        tokkey.offset(1 as libc::c_int as isize)
                                                            as *mut libc::c_void,
                                                        tokstart as *const libc::c_void,
                                                        tok.offset_from(tokstart) as libc::c_long as libc::c_ulong,
                                                    );
                                                    yylval = bcalloc(Op_token, 1 as libc::c_int, sourceline);
                                                    (*yylval).d.name = tokkey;
                                                    current_block = 14170946608255986518;
                                                }
                                            }
                                            291 | 273 | 274 | 292 | 293 => {
                                                yylval = bcalloc(
                                                    tokentab[mid as usize].value,
                                                    4 as libc::c_int,
                                                    sourceline,
                                                );
                                                current_block = 14170946608255986518;
                                            }
                                            284 | 282 | 283 | 279 => {
                                                if do_flags as libc::c_uint
                                                    & DO_PRETTY_PRINT as libc::c_int as libc::c_uint == 0
                                                {
                                                    lasttok = class;
                                                    return lasttok;
                                                }
                                                current_block = 9385870324848442183;
                                            }
                                            280 => {
                                                current_block = 9385870324848442183;
                                            }
                                            286 => {
                                                if continue_allowed == 0 {
                                                    error_ln(
                                                        sourceline,
                                                        dcgettext(
                                                            0 as *const libc::c_char,
                                                            b"`continue' is not allowed outside a loop\0" as *const u8
                                                                as *const libc::c_char,
                                                            5 as libc::c_int,
                                                        ),
                                                    );
                                                    errcount += 1;
                                                    errcount;
                                                }
                                                current_block = 2544673707468588147;
                                            }
                                            285 => {
                                                if break_allowed == 0 {
                                                    error_ln(
                                                        sourceline,
                                                        dcgettext(
                                                            0 as *const libc::c_char,
                                                            b"`break' is not allowed outside a loop or switch\0"
                                                                as *const u8 as *const libc::c_char,
                                                            5 as libc::c_int,
                                                        ),
                                                    );
                                                    errcount += 1;
                                                    errcount;
                                                }
                                                current_block = 2544673707468588147;
                                            }
                                            _ => {
                                                current_block = 2544673707468588147;
                                            }
                                        }
                                        match current_block {
                                            12777531360084369386 => {}
                                            _ => {
                                                match current_block {
                                                    2544673707468588147 => {
                                                        yylval = bcalloc(
                                                            tokentab[mid as usize].value,
                                                            1 as libc::c_int,
                                                            sourceline,
                                                        );
                                                        if class == 301 as libc::c_int
                                                            || class == 302 as libc::c_int
                                                        {
                                                            (*yylval).d.dl = mid as libc::c_long;
                                                        }
                                                    }
                                                    8523145309031438996 => {
                                                        want_source = 1 as libc::c_int != 0;
                                                    }
                                                    9385870324848442183 => {
                                                        yylval = bcalloc(
                                                            tokentab[mid as usize].value,
                                                            2 as libc::c_int,
                                                            sourceline,
                                                        );
                                                    }
                                                    _ => {}
                                                }
                                                lasttok = class;
                                                return lasttok;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if want_param_names as libc::c_uint
                        == FUNC_HEADER as libc::c_int as libc::c_uint
                    {
                        tokkey = estrdup(
                            tokstart,
                            (tok.offset_from(tokstart) as libc::c_long
                                - 1 as libc::c_int as libc::c_long) as size_t,
                        );
                    } else {
                        tokkey = qualify_name(
                            tokstart,
                            (tok.offset_from(tokstart) as libc::c_long
                                - 1 as libc::c_int as libc::c_long) as size_t,
                        );
                    }
                    if *lexptr as libc::c_int == '(' as i32 {
                        yylval = bcalloc(Op_token, 2 as libc::c_int, sourceline);
                        (*yylval).d.name = tokkey;
                        lasttok = 258 as libc::c_int;
                        return lasttok;
                    } else {
                        static mut goto_warned: bool = 0 as libc::c_int != 0;
                        yylval = bcalloc(Op_token, 1 as libc::c_int, sourceline);
                        (*yylval).d.name = tokkey;
                        if 1 as libc::c_int != 0
                            && do_flags as libc::c_uint
                                & (DO_LINT_INVALID as libc::c_int
                                    | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                            && !goto_warned
                            && ({
                                let mut __res: libc::c_int = 0;
                                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                    > 1 as libc::c_int as libc::c_ulong
                                {
                                    if 0 != 0 {
                                        let mut __c: libc::c_int = *tokkey
                                            .offset(0 as libc::c_int as isize) as libc::c_int;
                                        __res = if __c < -(128 as libc::c_int)
                                            || __c > 255 as libc::c_int
                                        {
                                            __c
                                        } else {
                                            *(*__ctype_tolower_loc()).offset(__c as isize)
                                        };
                                    } else {
                                        __res = tolower(
                                            *tokkey.offset(0 as libc::c_int as isize) as libc::c_int,
                                        );
                                    }
                                } else {
                                    __res = *(*__ctype_tolower_loc())
                                        .offset(
                                            *tokkey.offset(0 as libc::c_int as isize) as libc::c_int
                                                as isize,
                                        );
                                }
                                __res
                            }) == 'g' as i32
                            && strcasecmp(
                                tokkey,
                                b"goto\0" as *const u8 as *const libc::c_char,
                            ) == 0 as libc::c_int
                        {
                            goto_warned = 1 as libc::c_int != 0;
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    libc::c_int,
                                ) -> ())(
                                b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                4521 as libc::c_int,
                            );
                            (Some(lintfunc.unwrap()))
                                .unwrap()(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"`goto' considered harmful!\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        }
                        lasttok = 259 as libc::c_int;
                        return lasttok;
                    }
                }
            }
        }
        13191286796217823901 => {
            yylval = bcalloc(Op_cond_exp, 1 as libc::c_int, sourceline);
            if qm_col_count > 0 as libc::c_int {
                if do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint
                    == 0
                {
                    let mut new_comment_0: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                    allow_newline(&mut new_comment_0);
                    (*yylval).comment = new_comment_0;
                }
                if c == ':' as i32 {
                    qm_col_count -= 1;
                    qm_col_count;
                }
            }
            lasttok = c;
            return lasttok;
        }
        2370887241019905314 => {
            loop {
                c = nextc(0 as libc::c_int != 0);
                cur_index = tok.offset_from(tokstart) as libc::c_long as libc::c_int;
                if gawk_mb_cur_max == 1 as libc::c_int
                    || cur_char_ring[cur_ring_idx as usize] as libc::c_int
                        == 1 as libc::c_int
                {
                    match c {
                        91 => {
                            current_block = 8052071319557367686;
                            match current_block {
                                7844809803139404448 => {
                                    if in_brack > 0 as libc::c_int {
                                        current_block = 6545907279487748450;
                                    } else {
                                        current_block = 16639434367485728749;
                                    }
                                }
                                17351657478402406283 => {
                                    pushback();
                                    yyerror(
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"unterminated regexp at end of file\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                    current_block = 16639434367485728749;
                                }
                                10329836446692420713 => {
                                    pushback();
                                    yyerror(
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"unterminated regexp\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                    current_block = 16639434367485728749;
                                }
                                8052071319557367686 => {
                                    if nextc(0 as libc::c_int != 0) == ':' as i32
                                        || in_brack == 0 as libc::c_int
                                    {
                                        in_brack += 1;
                                        in_brack;
                                        if in_brack == 1 as libc::c_int {
                                            b_index = tok.offset_from(tokstart) as libc::c_long
                                                as libc::c_int;
                                        }
                                    }
                                    pushback();
                                    current_block = 6545907279487748450;
                                }
                                14566970060275900876 => {
                                    if !(in_brack > 0 as libc::c_int
                                        && (cur_index == b_index + 1 as libc::c_int
                                            || cur_index == b_index + 2 as libc::c_int
                                                && *tok.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                    == '^' as i32))
                                    {
                                        in_brack -= 1;
                                        in_brack;
                                        if in_brack == 0 as libc::c_int {
                                            b_index = -(1 as libc::c_int);
                                        }
                                    }
                                    current_block = 6545907279487748450;
                                }
                                _ => {
                                    c = nextc(0 as libc::c_int != 0);
                                    if c == -(1000 as libc::c_int) {
                                        pushback();
                                        yyerror(
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"unterminated regexp ends with `\\' at end of file\0"
                                                    as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                    } else {
                                        if c == '\r' as i32 {
                                            c = nextc(1 as libc::c_int != 0);
                                        }
                                        if c == '\n' as i32 {
                                            sourceline += 1;
                                            sourceline;
                                            continue;
                                        } else {
                                            let fresh97 = tok;
                                            tok = tok.offset(1);
                                            *fresh97 = '\\' as i32 as libc::c_char;
                                            if tok == tokend {
                                                tokexpand();
                                            } else {};
                                            let fresh98 = tok;
                                            tok = tok.offset(1);
                                            *fresh98 = c as libc::c_char;
                                            if tok == tokend {
                                                tokexpand();
                                            } else {};
                                            continue;
                                        }
                                    }
                                    current_block = 16639434367485728749;
                                }
                            }
                            match current_block {
                                6545907279487748450 => {}
                                _ => {
                                    yylval = bcalloc(Op_token, 1 as libc::c_int, sourceline);
                                    (*yylval)
                                        .d
                                        .name = estrdup(
                                        tokstart,
                                        tok.offset_from(tokstart) as libc::c_long as size_t,
                                    );
                                    if do_flags as libc::c_uint
                                        & (DO_LINT_INVALID as libc::c_int
                                            | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                    {
                                        let mut peek: libc::c_int = nextc(1 as libc::c_int != 0);
                                        pushback();
                                        if peek == 'i' as i32 || peek == 's' as i32 {
                                            if !source.is_null() {
                                                (set_loc
                                                    as unsafe extern "C" fn(
                                                        *const libc::c_char,
                                                        libc::c_int,
                                                    ) -> ())(
                                                    b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                                    3699 as libc::c_int,
                                                );
                                                (Some(lintfunc.unwrap()))
                                                    .unwrap()(
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"%s: %d: tawk regex modifier `/.../%c' doesn't work in gawk\0"
                                                            as *const u8 as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    ),
                                                    source,
                                                    sourceline,
                                                    peek,
                                                );
                                            } else {
                                                (set_loc
                                                    as unsafe extern "C" fn(
                                                        *const libc::c_char,
                                                        libc::c_int,
                                                    ) -> ())(
                                                    b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                                    3703 as libc::c_int,
                                                );
                                                (Some(lintfunc.unwrap()))
                                                    .unwrap()(
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"tawk regex modifier `/.../%c' doesn't work in gawk\0"
                                                            as *const u8 as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    ),
                                                    peek,
                                                );
                                            }
                                        }
                                    }
                                    if collecting_typed_regexp {
                                        collecting_typed_regexp = 0 as libc::c_int != 0;
                                        lasttok = 264 as libc::c_int;
                                    } else {
                                        lasttok = 260 as libc::c_int;
                                    }
                                    return lasttok;
                                }
                            }
                        }
                        93 => {
                            current_block = 14566970060275900876;
                            match current_block {
                                7844809803139404448 => {
                                    if in_brack > 0 as libc::c_int {
                                        current_block = 6545907279487748450;
                                    } else {
                                        current_block = 16639434367485728749;
                                    }
                                }
                                17351657478402406283 => {
                                    pushback();
                                    yyerror(
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"unterminated regexp at end of file\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                    current_block = 16639434367485728749;
                                }
                                10329836446692420713 => {
                                    pushback();
                                    yyerror(
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"unterminated regexp\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                    current_block = 16639434367485728749;
                                }
                                8052071319557367686 => {
                                    if nextc(0 as libc::c_int != 0) == ':' as i32
                                        || in_brack == 0 as libc::c_int
                                    {
                                        in_brack += 1;
                                        in_brack;
                                        if in_brack == 1 as libc::c_int {
                                            b_index = tok.offset_from(tokstart) as libc::c_long
                                                as libc::c_int;
                                        }
                                    }
                                    pushback();
                                    current_block = 6545907279487748450;
                                }
                                14566970060275900876 => {
                                    if !(in_brack > 0 as libc::c_int
                                        && (cur_index == b_index + 1 as libc::c_int
                                            || cur_index == b_index + 2 as libc::c_int
                                                && *tok.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                    == '^' as i32))
                                    {
                                        in_brack -= 1;
                                        in_brack;
                                        if in_brack == 0 as libc::c_int {
                                            b_index = -(1 as libc::c_int);
                                        }
                                    }
                                    current_block = 6545907279487748450;
                                }
                                _ => {
                                    c = nextc(0 as libc::c_int != 0);
                                    if c == -(1000 as libc::c_int) {
                                        pushback();
                                        yyerror(
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"unterminated regexp ends with `\\' at end of file\0"
                                                    as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                    } else {
                                        if c == '\r' as i32 {
                                            c = nextc(1 as libc::c_int != 0);
                                        }
                                        if c == '\n' as i32 {
                                            sourceline += 1;
                                            sourceline;
                                            continue;
                                        } else {
                                            let fresh97 = tok;
                                            tok = tok.offset(1);
                                            *fresh97 = '\\' as i32 as libc::c_char;
                                            if tok == tokend {
                                                tokexpand();
                                            } else {};
                                            let fresh98 = tok;
                                            tok = tok.offset(1);
                                            *fresh98 = c as libc::c_char;
                                            if tok == tokend {
                                                tokexpand();
                                            } else {};
                                            continue;
                                        }
                                    }
                                    current_block = 16639434367485728749;
                                }
                            }
                            match current_block {
                                6545907279487748450 => {}
                                _ => {
                                    yylval = bcalloc(Op_token, 1 as libc::c_int, sourceline);
                                    (*yylval)
                                        .d
                                        .name = estrdup(
                                        tokstart,
                                        tok.offset_from(tokstart) as libc::c_long as size_t,
                                    );
                                    if do_flags as libc::c_uint
                                        & (DO_LINT_INVALID as libc::c_int
                                            | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                    {
                                        let mut peek: libc::c_int = nextc(1 as libc::c_int != 0);
                                        pushback();
                                        if peek == 'i' as i32 || peek == 's' as i32 {
                                            if !source.is_null() {
                                                (set_loc
                                                    as unsafe extern "C" fn(
                                                        *const libc::c_char,
                                                        libc::c_int,
                                                    ) -> ())(
                                                    b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                                    3699 as libc::c_int,
                                                );
                                                (Some(lintfunc.unwrap()))
                                                    .unwrap()(
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"%s: %d: tawk regex modifier `/.../%c' doesn't work in gawk\0"
                                                            as *const u8 as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    ),
                                                    source,
                                                    sourceline,
                                                    peek,
                                                );
                                            } else {
                                                (set_loc
                                                    as unsafe extern "C" fn(
                                                        *const libc::c_char,
                                                        libc::c_int,
                                                    ) -> ())(
                                                    b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                                    3703 as libc::c_int,
                                                );
                                                (Some(lintfunc.unwrap()))
                                                    .unwrap()(
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"tawk regex modifier `/.../%c' doesn't work in gawk\0"
                                                            as *const u8 as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    ),
                                                    peek,
                                                );
                                            }
                                        }
                                    }
                                    if collecting_typed_regexp {
                                        collecting_typed_regexp = 0 as libc::c_int != 0;
                                        lasttok = 264 as libc::c_int;
                                    } else {
                                        lasttok = 260 as libc::c_int;
                                    }
                                    return lasttok;
                                }
                            }
                        }
                        92 => {
                            current_block = 18145810582175794718;
                            match current_block {
                                7844809803139404448 => {
                                    if in_brack > 0 as libc::c_int {
                                        current_block = 6545907279487748450;
                                    } else {
                                        current_block = 16639434367485728749;
                                    }
                                }
                                17351657478402406283 => {
                                    pushback();
                                    yyerror(
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"unterminated regexp at end of file\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                    current_block = 16639434367485728749;
                                }
                                10329836446692420713 => {
                                    pushback();
                                    yyerror(
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"unterminated regexp\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                    current_block = 16639434367485728749;
                                }
                                8052071319557367686 => {
                                    if nextc(0 as libc::c_int != 0) == ':' as i32
                                        || in_brack == 0 as libc::c_int
                                    {
                                        in_brack += 1;
                                        in_brack;
                                        if in_brack == 1 as libc::c_int {
                                            b_index = tok.offset_from(tokstart) as libc::c_long
                                                as libc::c_int;
                                        }
                                    }
                                    pushback();
                                    current_block = 6545907279487748450;
                                }
                                14566970060275900876 => {
                                    if !(in_brack > 0 as libc::c_int
                                        && (cur_index == b_index + 1 as libc::c_int
                                            || cur_index == b_index + 2 as libc::c_int
                                                && *tok.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                    == '^' as i32))
                                    {
                                        in_brack -= 1;
                                        in_brack;
                                        if in_brack == 0 as libc::c_int {
                                            b_index = -(1 as libc::c_int);
                                        }
                                    }
                                    current_block = 6545907279487748450;
                                }
                                _ => {
                                    c = nextc(0 as libc::c_int != 0);
                                    if c == -(1000 as libc::c_int) {
                                        pushback();
                                        yyerror(
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"unterminated regexp ends with `\\' at end of file\0"
                                                    as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                    } else {
                                        if c == '\r' as i32 {
                                            c = nextc(1 as libc::c_int != 0);
                                        }
                                        if c == '\n' as i32 {
                                            sourceline += 1;
                                            sourceline;
                                            continue;
                                        } else {
                                            let fresh97 = tok;
                                            tok = tok.offset(1);
                                            *fresh97 = '\\' as i32 as libc::c_char;
                                            if tok == tokend {
                                                tokexpand();
                                            } else {};
                                            let fresh98 = tok;
                                            tok = tok.offset(1);
                                            *fresh98 = c as libc::c_char;
                                            if tok == tokend {
                                                tokexpand();
                                            } else {};
                                            continue;
                                        }
                                    }
                                    current_block = 16639434367485728749;
                                }
                            }
                            match current_block {
                                6545907279487748450 => {}
                                _ => {
                                    yylval = bcalloc(Op_token, 1 as libc::c_int, sourceline);
                                    (*yylval)
                                        .d
                                        .name = estrdup(
                                        tokstart,
                                        tok.offset_from(tokstart) as libc::c_long as size_t,
                                    );
                                    if do_flags as libc::c_uint
                                        & (DO_LINT_INVALID as libc::c_int
                                            | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                    {
                                        let mut peek: libc::c_int = nextc(1 as libc::c_int != 0);
                                        pushback();
                                        if peek == 'i' as i32 || peek == 's' as i32 {
                                            if !source.is_null() {
                                                (set_loc
                                                    as unsafe extern "C" fn(
                                                        *const libc::c_char,
                                                        libc::c_int,
                                                    ) -> ())(
                                                    b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                                    3699 as libc::c_int,
                                                );
                                                (Some(lintfunc.unwrap()))
                                                    .unwrap()(
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"%s: %d: tawk regex modifier `/.../%c' doesn't work in gawk\0"
                                                            as *const u8 as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    ),
                                                    source,
                                                    sourceline,
                                                    peek,
                                                );
                                            } else {
                                                (set_loc
                                                    as unsafe extern "C" fn(
                                                        *const libc::c_char,
                                                        libc::c_int,
                                                    ) -> ())(
                                                    b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                                    3703 as libc::c_int,
                                                );
                                                (Some(lintfunc.unwrap()))
                                                    .unwrap()(
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"tawk regex modifier `/.../%c' doesn't work in gawk\0"
                                                            as *const u8 as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    ),
                                                    peek,
                                                );
                                            }
                                        }
                                    }
                                    if collecting_typed_regexp {
                                        collecting_typed_regexp = 0 as libc::c_int != 0;
                                        lasttok = 264 as libc::c_int;
                                    } else {
                                        lasttok = 260 as libc::c_int;
                                    }
                                    return lasttok;
                                }
                            }
                        }
                        47 => {
                            current_block = 7844809803139404448;
                            match current_block {
                                7844809803139404448 => {
                                    if in_brack > 0 as libc::c_int {
                                        current_block = 6545907279487748450;
                                    } else {
                                        current_block = 16639434367485728749;
                                    }
                                }
                                17351657478402406283 => {
                                    pushback();
                                    yyerror(
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"unterminated regexp at end of file\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                    current_block = 16639434367485728749;
                                }
                                10329836446692420713 => {
                                    pushback();
                                    yyerror(
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"unterminated regexp\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                    current_block = 16639434367485728749;
                                }
                                8052071319557367686 => {
                                    if nextc(0 as libc::c_int != 0) == ':' as i32
                                        || in_brack == 0 as libc::c_int
                                    {
                                        in_brack += 1;
                                        in_brack;
                                        if in_brack == 1 as libc::c_int {
                                            b_index = tok.offset_from(tokstart) as libc::c_long
                                                as libc::c_int;
                                        }
                                    }
                                    pushback();
                                    current_block = 6545907279487748450;
                                }
                                14566970060275900876 => {
                                    if !(in_brack > 0 as libc::c_int
                                        && (cur_index == b_index + 1 as libc::c_int
                                            || cur_index == b_index + 2 as libc::c_int
                                                && *tok.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                    == '^' as i32))
                                    {
                                        in_brack -= 1;
                                        in_brack;
                                        if in_brack == 0 as libc::c_int {
                                            b_index = -(1 as libc::c_int);
                                        }
                                    }
                                    current_block = 6545907279487748450;
                                }
                                _ => {
                                    c = nextc(0 as libc::c_int != 0);
                                    if c == -(1000 as libc::c_int) {
                                        pushback();
                                        yyerror(
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"unterminated regexp ends with `\\' at end of file\0"
                                                    as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                    } else {
                                        if c == '\r' as i32 {
                                            c = nextc(1 as libc::c_int != 0);
                                        }
                                        if c == '\n' as i32 {
                                            sourceline += 1;
                                            sourceline;
                                            continue;
                                        } else {
                                            let fresh97 = tok;
                                            tok = tok.offset(1);
                                            *fresh97 = '\\' as i32 as libc::c_char;
                                            if tok == tokend {
                                                tokexpand();
                                            } else {};
                                            let fresh98 = tok;
                                            tok = tok.offset(1);
                                            *fresh98 = c as libc::c_char;
                                            if tok == tokend {
                                                tokexpand();
                                            } else {};
                                            continue;
                                        }
                                    }
                                    current_block = 16639434367485728749;
                                }
                            }
                            match current_block {
                                6545907279487748450 => {}
                                _ => {
                                    yylval = bcalloc(Op_token, 1 as libc::c_int, sourceline);
                                    (*yylval)
                                        .d
                                        .name = estrdup(
                                        tokstart,
                                        tok.offset_from(tokstart) as libc::c_long as size_t,
                                    );
                                    if do_flags as libc::c_uint
                                        & (DO_LINT_INVALID as libc::c_int
                                            | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                    {
                                        let mut peek: libc::c_int = nextc(1 as libc::c_int != 0);
                                        pushback();
                                        if peek == 'i' as i32 || peek == 's' as i32 {
                                            if !source.is_null() {
                                                (set_loc
                                                    as unsafe extern "C" fn(
                                                        *const libc::c_char,
                                                        libc::c_int,
                                                    ) -> ())(
                                                    b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                                    3699 as libc::c_int,
                                                );
                                                (Some(lintfunc.unwrap()))
                                                    .unwrap()(
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"%s: %d: tawk regex modifier `/.../%c' doesn't work in gawk\0"
                                                            as *const u8 as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    ),
                                                    source,
                                                    sourceline,
                                                    peek,
                                                );
                                            } else {
                                                (set_loc
                                                    as unsafe extern "C" fn(
                                                        *const libc::c_char,
                                                        libc::c_int,
                                                    ) -> ())(
                                                    b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                                    3703 as libc::c_int,
                                                );
                                                (Some(lintfunc.unwrap()))
                                                    .unwrap()(
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"tawk regex modifier `/.../%c' doesn't work in gawk\0"
                                                            as *const u8 as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    ),
                                                    peek,
                                                );
                                            }
                                        }
                                    }
                                    if collecting_typed_regexp {
                                        collecting_typed_regexp = 0 as libc::c_int != 0;
                                        lasttok = 264 as libc::c_int;
                                    } else {
                                        lasttok = 260 as libc::c_int;
                                    }
                                    return lasttok;
                                }
                            }
                        }
                        10 => {
                            current_block = 10329836446692420713;
                            match current_block {
                                7844809803139404448 => {
                                    if in_brack > 0 as libc::c_int {
                                        current_block = 6545907279487748450;
                                    } else {
                                        current_block = 16639434367485728749;
                                    }
                                }
                                17351657478402406283 => {
                                    pushback();
                                    yyerror(
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"unterminated regexp at end of file\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                    current_block = 16639434367485728749;
                                }
                                10329836446692420713 => {
                                    pushback();
                                    yyerror(
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"unterminated regexp\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                    current_block = 16639434367485728749;
                                }
                                8052071319557367686 => {
                                    if nextc(0 as libc::c_int != 0) == ':' as i32
                                        || in_brack == 0 as libc::c_int
                                    {
                                        in_brack += 1;
                                        in_brack;
                                        if in_brack == 1 as libc::c_int {
                                            b_index = tok.offset_from(tokstart) as libc::c_long
                                                as libc::c_int;
                                        }
                                    }
                                    pushback();
                                    current_block = 6545907279487748450;
                                }
                                14566970060275900876 => {
                                    if !(in_brack > 0 as libc::c_int
                                        && (cur_index == b_index + 1 as libc::c_int
                                            || cur_index == b_index + 2 as libc::c_int
                                                && *tok.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                    == '^' as i32))
                                    {
                                        in_brack -= 1;
                                        in_brack;
                                        if in_brack == 0 as libc::c_int {
                                            b_index = -(1 as libc::c_int);
                                        }
                                    }
                                    current_block = 6545907279487748450;
                                }
                                _ => {
                                    c = nextc(0 as libc::c_int != 0);
                                    if c == -(1000 as libc::c_int) {
                                        pushback();
                                        yyerror(
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"unterminated regexp ends with `\\' at end of file\0"
                                                    as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                    } else {
                                        if c == '\r' as i32 {
                                            c = nextc(1 as libc::c_int != 0);
                                        }
                                        if c == '\n' as i32 {
                                            sourceline += 1;
                                            sourceline;
                                            continue;
                                        } else {
                                            let fresh97 = tok;
                                            tok = tok.offset(1);
                                            *fresh97 = '\\' as i32 as libc::c_char;
                                            if tok == tokend {
                                                tokexpand();
                                            } else {};
                                            let fresh98 = tok;
                                            tok = tok.offset(1);
                                            *fresh98 = c as libc::c_char;
                                            if tok == tokend {
                                                tokexpand();
                                            } else {};
                                            continue;
                                        }
                                    }
                                    current_block = 16639434367485728749;
                                }
                            }
                            match current_block {
                                6545907279487748450 => {}
                                _ => {
                                    yylval = bcalloc(Op_token, 1 as libc::c_int, sourceline);
                                    (*yylval)
                                        .d
                                        .name = estrdup(
                                        tokstart,
                                        tok.offset_from(tokstart) as libc::c_long as size_t,
                                    );
                                    if do_flags as libc::c_uint
                                        & (DO_LINT_INVALID as libc::c_int
                                            | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                    {
                                        let mut peek: libc::c_int = nextc(1 as libc::c_int != 0);
                                        pushback();
                                        if peek == 'i' as i32 || peek == 's' as i32 {
                                            if !source.is_null() {
                                                (set_loc
                                                    as unsafe extern "C" fn(
                                                        *const libc::c_char,
                                                        libc::c_int,
                                                    ) -> ())(
                                                    b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                                    3699 as libc::c_int,
                                                );
                                                (Some(lintfunc.unwrap()))
                                                    .unwrap()(
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"%s: %d: tawk regex modifier `/.../%c' doesn't work in gawk\0"
                                                            as *const u8 as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    ),
                                                    source,
                                                    sourceline,
                                                    peek,
                                                );
                                            } else {
                                                (set_loc
                                                    as unsafe extern "C" fn(
                                                        *const libc::c_char,
                                                        libc::c_int,
                                                    ) -> ())(
                                                    b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                                    3703 as libc::c_int,
                                                );
                                                (Some(lintfunc.unwrap()))
                                                    .unwrap()(
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"tawk regex modifier `/.../%c' doesn't work in gawk\0"
                                                            as *const u8 as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    ),
                                                    peek,
                                                );
                                            }
                                        }
                                    }
                                    if collecting_typed_regexp {
                                        collecting_typed_regexp = 0 as libc::c_int != 0;
                                        lasttok = 264 as libc::c_int;
                                    } else {
                                        lasttok = 260 as libc::c_int;
                                    }
                                    return lasttok;
                                }
                            }
                        }
                        -1000 => {
                            current_block = 17351657478402406283;
                            match current_block {
                                7844809803139404448 => {
                                    if in_brack > 0 as libc::c_int {
                                        current_block = 6545907279487748450;
                                    } else {
                                        current_block = 16639434367485728749;
                                    }
                                }
                                17351657478402406283 => {
                                    pushback();
                                    yyerror(
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"unterminated regexp at end of file\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                    current_block = 16639434367485728749;
                                }
                                10329836446692420713 => {
                                    pushback();
                                    yyerror(
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"unterminated regexp\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                    current_block = 16639434367485728749;
                                }
                                8052071319557367686 => {
                                    if nextc(0 as libc::c_int != 0) == ':' as i32
                                        || in_brack == 0 as libc::c_int
                                    {
                                        in_brack += 1;
                                        in_brack;
                                        if in_brack == 1 as libc::c_int {
                                            b_index = tok.offset_from(tokstart) as libc::c_long
                                                as libc::c_int;
                                        }
                                    }
                                    pushback();
                                    current_block = 6545907279487748450;
                                }
                                14566970060275900876 => {
                                    if !(in_brack > 0 as libc::c_int
                                        && (cur_index == b_index + 1 as libc::c_int
                                            || cur_index == b_index + 2 as libc::c_int
                                                && *tok.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                    == '^' as i32))
                                    {
                                        in_brack -= 1;
                                        in_brack;
                                        if in_brack == 0 as libc::c_int {
                                            b_index = -(1 as libc::c_int);
                                        }
                                    }
                                    current_block = 6545907279487748450;
                                }
                                _ => {
                                    c = nextc(0 as libc::c_int != 0);
                                    if c == -(1000 as libc::c_int) {
                                        pushback();
                                        yyerror(
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"unterminated regexp ends with `\\' at end of file\0"
                                                    as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                    } else {
                                        if c == '\r' as i32 {
                                            c = nextc(1 as libc::c_int != 0);
                                        }
                                        if c == '\n' as i32 {
                                            sourceline += 1;
                                            sourceline;
                                            continue;
                                        } else {
                                            let fresh97 = tok;
                                            tok = tok.offset(1);
                                            *fresh97 = '\\' as i32 as libc::c_char;
                                            if tok == tokend {
                                                tokexpand();
                                            } else {};
                                            let fresh98 = tok;
                                            tok = tok.offset(1);
                                            *fresh98 = c as libc::c_char;
                                            if tok == tokend {
                                                tokexpand();
                                            } else {};
                                            continue;
                                        }
                                    }
                                    current_block = 16639434367485728749;
                                }
                            }
                            match current_block {
                                6545907279487748450 => {}
                                _ => {
                                    yylval = bcalloc(Op_token, 1 as libc::c_int, sourceline);
                                    (*yylval)
                                        .d
                                        .name = estrdup(
                                        tokstart,
                                        tok.offset_from(tokstart) as libc::c_long as size_t,
                                    );
                                    if do_flags as libc::c_uint
                                        & (DO_LINT_INVALID as libc::c_int
                                            | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                    {
                                        let mut peek: libc::c_int = nextc(1 as libc::c_int != 0);
                                        pushback();
                                        if peek == 'i' as i32 || peek == 's' as i32 {
                                            if !source.is_null() {
                                                (set_loc
                                                    as unsafe extern "C" fn(
                                                        *const libc::c_char,
                                                        libc::c_int,
                                                    ) -> ())(
                                                    b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                                    3699 as libc::c_int,
                                                );
                                                (Some(lintfunc.unwrap()))
                                                    .unwrap()(
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"%s: %d: tawk regex modifier `/.../%c' doesn't work in gawk\0"
                                                            as *const u8 as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    ),
                                                    source,
                                                    sourceline,
                                                    peek,
                                                );
                                            } else {
                                                (set_loc
                                                    as unsafe extern "C" fn(
                                                        *const libc::c_char,
                                                        libc::c_int,
                                                    ) -> ())(
                                                    b"awkgram.y\0" as *const u8 as *const libc::c_char,
                                                    3703 as libc::c_int,
                                                );
                                                (Some(lintfunc.unwrap()))
                                                    .unwrap()(
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"tawk regex modifier `/.../%c' doesn't work in gawk\0"
                                                            as *const u8 as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    ),
                                                    peek,
                                                );
                                            }
                                        }
                                    }
                                    if collecting_typed_regexp {
                                        collecting_typed_regexp = 0 as libc::c_int != 0;
                                        lasttok = 264 as libc::c_int;
                                    } else {
                                        lasttok = 260 as libc::c_int;
                                    }
                                    return lasttok;
                                }
                            }
                        }
                        _ => {}
                    }
                }
                let fresh99 = tok;
                tok = tok.offset(1);
                *fresh99 = c as libc::c_char;
                if tok == tokend {
                    tokexpand();
                } else {};
            }
        }
        6570067857177716340 => {
            lasttok = c;
            return lasttok;
        }
        _ => {}
    }
    esc_seen = 0 as libc::c_int != 0;
    loop {
        c = nextc(0 as libc::c_int != 0);
        if !(c != '"' as i32) {
            break;
        }
        if c == '\n' as i32 {
            pushback();
            yyerror(
                dcgettext(
                    0 as *const libc::c_char,
                    b"unterminated string\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            lasttok = 303 as libc::c_int;
            return lasttok;
        }
        if (gawk_mb_cur_max == 1 as libc::c_int
            || cur_char_ring[cur_ring_idx as usize] as libc::c_int == 1 as libc::c_int)
            && c == '\\' as i32
        {
            c = nextc(1 as libc::c_int != 0);
            if c == '\r' as i32 {
                c = nextc(1 as libc::c_int != 0);
            }
            if c == '\n' as i32 {
                if do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint
                    != 0
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"awkgram.y\0" as *const u8 as *const libc::c_char,
                        4070 as libc::c_int,
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
                            b"POSIX does not allow physical newlines in string values\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                } else if do_flags as libc::c_uint
                    & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                        as libc::c_uint != 0
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"awkgram.y\0" as *const u8 as *const libc::c_char,
                        4072 as libc::c_int,
                    );
                    (Some(lintfunc.unwrap()))
                        .unwrap()(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"backslash string continuation is not portable\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                sourceline += 1;
                sourceline;
                continue;
            } else {
                esc_seen = 1 as libc::c_int != 0;
                if !want_source || c != '"' as i32 {
                    let fresh100 = tok;
                    tok = tok.offset(1);
                    *fresh100 = '\\' as i32 as libc::c_char;
                    if tok == tokend {
                        tokexpand();
                    } else {};
                }
            }
        }
        if c == -(1000 as libc::c_int) {
            pushback();
            yyerror(
                dcgettext(
                    0 as *const libc::c_char,
                    b"unterminated string\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            lasttok = 303 as libc::c_int;
            return lasttok;
        }
        let fresh101 = tok;
        tok = tok.offset(1);
        *fresh101 = c as libc::c_char;
        if tok == tokend {
            tokexpand();
        } else {};
    }
    yylval = bcalloc(Op_token, 1 as libc::c_int, sourceline);
    if want_source {
        (*yylval)
            .d
            .name = estrdup(
            tokstart,
            tok.offset_from(tokstart) as libc::c_long as size_t,
        );
        if want_namespace {
            change_namespace((*yylval).d.name);
        }
        lasttok = 261 as libc::c_int;
        return lasttok;
    }
    (*yylval).opcode = Op_push_i;
    (*yylval)
        .d
        .dn = make_str_node(
        tokstart,
        tok.offset_from(tokstart) as libc::c_long as size_t,
        if esc_seen as libc::c_int != 0 { 1 as libc::c_int } else { 0 as libc::c_int },
    );
    if intlstr {
        (*(*yylval).d.dn)
            .flags = ::std::mem::transmute::<
            libc::c_uint,
            flagvals,
        >(
            (*(*yylval).d.dn).flags as libc::c_uint
                | INTLSTR as libc::c_int as libc::c_uint,
        );
        intlstr = 0 as libc::c_int != 0;
        if do_flags as libc::c_uint & DO_INTL as libc::c_int as libc::c_uint != 0 {
            dumpintlstr((*(*yylval).d.dn).sub.val.sp, (*(*yylval).d.dn).sub.val.slen);
        }
    }
    lasttok = 263 as libc::c_int;
    return lasttok;
}
unsafe extern "C" fn snode(
    mut subn: *mut INSTRUCTION,
    mut r: *mut INSTRUCTION,
) -> *mut INSTRUCTION {
    let mut arg: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut n: *mut NODE = 0 as *mut NODE;
    let mut nexp: libc::c_int = 0 as libc::c_int;
    let mut args_allowed: libc::c_int = 0;
    let mut idx: libc::c_int = (*r).d.dl as libc::c_int;
    if !subn.is_null() {
        let mut tp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
        tp = (*subn).nexti;
        while !tp.is_null() {
            tp = (*tp).d.di;
            nexp += 1;
            nexp;
            tp = (*tp).nexti;
        }
    }
    args_allowed = (tokentab[idx as usize].flags & 0xff as libc::c_int as libc::c_uint)
        as libc::c_int;
    if args_allowed != 0 && args_allowed & (1 as libc::c_int) << nexp == 0 as libc::c_int
    {
        yyerror(
            dcgettext(
                0 as *const libc::c_char,
                b"%d is invalid as number of arguments for %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            nexp,
            tokentab[idx as usize].operator,
        );
        return 0 as *mut INSTRUCTION;
    }
    if tokentab[idx as usize].value as libc::c_uint
        == Op_sub_builtin as libc::c_int as libc::c_uint
    {
        let mut operator: *const libc::c_char = tokentab[idx as usize].operator;
        (*r).d.dl = 0 as libc::c_int as libc::c_long;
        arg = (*subn).nexti;
        mk_rexp(arg);
        if strcmp(operator, b"gensub\0" as *const u8 as *const libc::c_char)
            != 0 as libc::c_int
        {
            if strcmp(operator, b"gsub\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*r).d.dl |= 0x1 as libc::c_int as libc::c_long;
            }
            arg = (*(*arg).d.di).nexti;
            if nexp == 2 as libc::c_int {
                let mut expr: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                expr = list_create(
                    bcalloc(Op_push_i, 1 as libc::c_int, 0 as libc::c_int),
                );
                (*(*expr).nexti)
                    .d
                    .dn = set_profile_text(
                    make_number.unwrap()(0.0f64),
                    b"0\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int as size_t,
                );
                mk_expression_list(
                    subn,
                    list_append(
                        expr,
                        bcalloc(Op_field_spec, 1 as libc::c_int, 0 as libc::c_int),
                    ),
                );
            }
            arg = (*(*arg).d.di).nexti;
            ip = (*arg).d.di;
            if (*ip).opcode as libc::c_uint == Op_push_i as libc::c_int as libc::c_uint {
                if do_flags as libc::c_uint
                    & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                        as libc::c_uint != 0
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"awkgram.y\0" as *const u8 as *const libc::c_char,
                        4625 as libc::c_int,
                    );
                    (Some(lintfunc.unwrap()))
                        .unwrap()(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: string literal as last argument of substitute has no effect\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        operator,
                    );
                }
                (*r).d.dl |= 0x4 as libc::c_int as libc::c_long;
            } else if (make_assignable(ip)).is_null() {
                yyerror(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s third parameter is not a changeable object\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    operator,
                );
            } else {
                (*ip).x.xl = 1 as libc::c_int as libc::c_long;
            }
            (*r)
                .x
                .xl = count_expressions(&mut subn, 0 as libc::c_int != 0)
                as libc::c_long;
            ip = (*subn).d.di;
            list_append(subn, r);
            if (*ip).opcode as libc::c_uint == Op_push_lhs as libc::c_int as libc::c_uint
                && (*(*ip).d.dn).type_0 as libc::c_uint
                    == Node_var as libc::c_int as libc::c_uint
                && ((*(*ip).d.dn).sub.nodep.x.aptr).is_some()
            {
                list_append(
                    subn,
                    bcalloc(Op_var_assign, 1 as libc::c_int, 0 as libc::c_int),
                );
                (*(*subn).d.di).d.dl = Op_sub_builtin as libc::c_int as libc::c_long;
                (*(*subn).d.di).x.aptr = (*(*ip).d.dn).sub.nodep.x.aptr;
            } else if (*ip).opcode as libc::c_uint
                == Op_field_spec_lhs as libc::c_int as libc::c_uint
            {
                list_append(
                    subn,
                    bcalloc(Op_field_assign, 1 as libc::c_int, 0 as libc::c_int),
                );
                (*(*subn).d.di).d.dl = Op_sub_builtin as libc::c_int as libc::c_long;
                (*(*subn).d.di).x.aptr = None;
                (*ip).d.di = (*subn).d.di;
            } else if (*ip).opcode as libc::c_uint
                == Op_subscript_lhs as libc::c_int as libc::c_uint
            {
                list_append(
                    subn,
                    bcalloc(Op_subscript_assign, 1 as libc::c_int, 0 as libc::c_int),
                );
                (*(*subn).d.di).d.dl = Op_sub_builtin as libc::c_int as libc::c_long;
            }
            return subn;
        } else {
            (*r).d.dl |= 0x2 as libc::c_int as libc::c_long;
            if nexp == 3 as libc::c_int {
                ip = bcalloc(Op_push_i, 1 as libc::c_int, 0 as libc::c_int);
                (*ip)
                    .d
                    .dn = set_profile_text(
                    make_number.unwrap()(0.0f64),
                    b"0\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int as size_t,
                );
                mk_expression_list(
                    subn,
                    list_append(
                        list_create(ip),
                        bcalloc(Op_field_spec, 1 as libc::c_int, 0 as libc::c_int),
                    ),
                );
            }
            (*r)
                .x
                .xl = count_expressions(&mut subn, 0 as libc::c_int != 0)
                as libc::c_long;
            return list_append(subn, r);
        }
    }
    (*r).d.fptr = tokentab[idx as usize].ptr;
    if (*r).d.fptr == Some(do_length as unsafe extern "C" fn(libc::c_int) -> *mut NODE) {
        if nexp == 0 as libc::c_int {
            let mut list: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
            (*r).x.xl = 1 as libc::c_int as libc::c_long;
            list = list_create(r);
            list_prepend(
                list,
                bcalloc(Op_field_spec, 1 as libc::c_int, 0 as libc::c_int),
            );
            list_prepend(list, bcalloc(Op_push_i, 1 as libc::c_int, 0 as libc::c_int));
            (*(*list).nexti)
                .d
                .dn = set_profile_text(
                make_number.unwrap()(0.0f64),
                b"0\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
            );
            return list;
        } else {
            arg = (*subn).nexti;
            if (*arg).nexti == (*arg).d.di
                && (*(*arg).nexti).opcode as libc::c_uint
                    == Op_push as libc::c_int as libc::c_uint
            {
                (*(*arg).nexti).opcode = Op_push_arg;
            }
        }
    } else if (*r).d.fptr
        == Some(do_isarray as unsafe extern "C" fn(libc::c_int) -> *mut NODE)
    {
        arg = (*subn).nexti;
        if (*arg).nexti == (*arg).d.di
            && (*(*arg).nexti).opcode as libc::c_uint
                == Op_push as libc::c_int as libc::c_uint
        {
            (*(*arg).nexti).opcode = Op_push_arg_untyped;
        }
    } else if (*r).d.fptr
        == Some(do_typeof as unsafe extern "C" fn(libc::c_int) -> *mut NODE)
    {
        arg = (*subn).nexti;
        if (*arg).nexti == (*arg).d.di
            && (*(*arg).nexti).opcode as libc::c_uint
                == Op_push as libc::c_int as libc::c_uint
        {
            (*(*arg).nexti).opcode = Op_push_arg_untyped;
        }
        if nexp == 2 as libc::c_int {
            arg = (*(*(*subn).nexti).d.di).nexti;
            ip = (*arg).d.di;
            if (*ip).opcode as libc::c_uint == Op_push as libc::c_int as libc::c_uint {
                (*ip).opcode = Op_push_array;
            }
        }
    } else if (*r).d.fptr
        == Some(do_match as unsafe extern "C" fn(libc::c_int) -> *mut NODE)
    {
        static mut warned: bool = 0 as libc::c_int != 0;
        arg = (*(*(*subn).nexti).d.di).nexti;
        mk_rexp(arg);
        if nexp == 3 as libc::c_int {
            if do_flags as libc::c_uint
                & DO_LINT_EXTENSIONS as libc::c_int as libc::c_uint != 0 && !warned
            {
                warned = 1 as libc::c_int != 0;
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"awkgram.y\0" as *const u8 as *const libc::c_char,
                    4734 as libc::c_int,
                );
                (Some(lintfunc.unwrap()))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"match: third argument is a gawk extension\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint
                != 0
            {
                yyerror(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"match: third argument is a gawk extension\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                return 0 as *mut INSTRUCTION;
            }
            arg = (*(*arg).d.di).nexti;
            ip = (*arg).d.di;
            if (*ip).opcode as libc::c_uint == Op_push as libc::c_int as libc::c_uint {
                (*ip).opcode = Op_push_array;
            }
        }
    } else if (*r).d.fptr
        == Some(do_split as unsafe extern "C" fn(libc::c_int) -> *mut NODE)
    {
        arg = (*(*(*subn).nexti).d.di).nexti;
        ip = (*arg).d.di;
        if (*ip).opcode as libc::c_uint == Op_push as libc::c_int as libc::c_uint {
            (*ip).opcode = Op_push_array;
        }
        if nexp == 2 as libc::c_int {
            let mut expr_0: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
            expr_0 = list_create(bcalloc(Op_push, 1 as libc::c_int, 0 as libc::c_int));
            (*(*expr_0).nexti).d.dn = FS_node;
            mk_expression_list(subn, expr_0);
        }
        arg = (*(*arg).d.di).nexti;
        n = mk_rexp(arg);
        if nexp == 2 as libc::c_int {
            (*n)
                .sub
                .nodep
                .reflags = ::std::mem::transmute::<
                libc::c_uint,
                reflagvals,
            >(
                (*n).sub.nodep.reflags as libc::c_uint
                    | FS_DFLT as libc::c_int as libc::c_uint,
            );
        }
        if nexp == 4 as libc::c_int {
            arg = (*(*arg).d.di).nexti;
            ip = (*arg).d.di;
            if (*ip).opcode as libc::c_uint == Op_push as libc::c_int as libc::c_uint {
                (*ip).opcode = Op_push_array;
            }
        }
    } else if (*r).d.fptr
        == Some(do_patsplit as unsafe extern "C" fn(libc::c_int) -> *mut NODE)
    {
        arg = (*(*(*subn).nexti).d.di).nexti;
        ip = (*arg).d.di;
        if (*ip).opcode as libc::c_uint == Op_push as libc::c_int as libc::c_uint {
            (*ip).opcode = Op_push_array;
        }
        if nexp == 2 as libc::c_int {
            let mut expr_1: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
            expr_1 = list_create(bcalloc(Op_push, 1 as libc::c_int, 0 as libc::c_int));
            (*(*expr_1).nexti).d.dn = FPAT_node;
            mk_expression_list(subn, expr_1);
        }
        arg = (*(*arg).d.di).nexti;
        n = mk_rexp(arg);
        if nexp == 4 as libc::c_int {
            arg = (*(*arg).d.di).nexti;
            ip = (*arg).d.di;
            if (*ip).opcode as libc::c_uint == Op_push as libc::c_int as libc::c_uint {
                (*ip).opcode = Op_push_array;
            }
        }
    } else if (*r).d.fptr
        == Some(do_close as unsafe extern "C" fn(libc::c_int) -> *mut NODE)
    {
        static mut warned_0: bool = 0 as libc::c_int != 0;
        if nexp == 2 as libc::c_int {
            if do_flags as libc::c_uint
                & DO_LINT_EXTENSIONS as libc::c_int as libc::c_uint != 0 && !warned_0
            {
                warned_0 = 1 as libc::c_int != 0;
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"awkgram.y\0" as *const u8 as *const libc::c_char,
                    4791 as libc::c_int,
                );
                (Some(lintfunc.unwrap()))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"close: second argument is a gawk extension\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint
                != 0
            {
                yyerror(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"close: second argument is a gawk extension\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                return 0 as *mut INSTRUCTION;
            }
        }
    } else if do_flags as libc::c_uint & DO_INTL as libc::c_int as libc::c_uint != 0
        && (*r).d.fptr
            == Some(do_dcgettext as unsafe extern "C" fn(libc::c_int) -> *mut NODE)
        && (*(*(*subn).nexti).d.di).opcode as libc::c_uint
            == Op_push_i as libc::c_int as libc::c_uint
        && (*(*(*(*subn).nexti).d.di).d.dn).flags as libc::c_uint
            & STRING as libc::c_int as libc::c_uint != 0 as libc::c_int as libc::c_uint
    {
        let mut str: *mut NODE = (*(*(*subn).nexti).d.di).d.dn;
        if (*str).flags as libc::c_uint & INTLSTR as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"awkgram.y\0" as *const u8 as *const libc::c_char,
                4806 as libc::c_int,
            );
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"use of dcgettext(_\"...\") is incorrect: remove leading underscore\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        } else {
            dumpintlstr((*str).sub.val.sp, (*str).sub.val.slen);
        }
    } else if do_flags as libc::c_uint & DO_INTL as libc::c_int as libc::c_uint != 0
        && (*r).d.fptr
            == Some(do_dcngettext as unsafe extern "C" fn(libc::c_int) -> *mut NODE)
        && (*(*(*subn).nexti).d.di).opcode as libc::c_uint
            == Op_push_i as libc::c_int as libc::c_uint
        && (*(*(*(*subn).nexti).d.di).d.dn).flags as libc::c_uint
            & STRING as libc::c_int as libc::c_uint != 0 as libc::c_int as libc::c_uint
        && (*(*(*(*(*subn).nexti).d.di).nexti).d.di).opcode as libc::c_uint
            == Op_push_i as libc::c_int as libc::c_uint
        && (*(*(*(*(*(*subn).nexti).d.di).nexti).d.di).d.dn).flags as libc::c_uint
            & STRING as libc::c_int as libc::c_uint != 0 as libc::c_int as libc::c_uint
    {
        let mut str1: *mut NODE = (*(*(*subn).nexti).d.di).d.dn;
        let mut str2: *mut NODE = (*(*(*(*(*subn).nexti).d.di).nexti).d.di).d.dn;
        if ((*str1).flags as libc::c_uint | (*str2).flags as libc::c_uint)
            & INTLSTR as libc::c_int as libc::c_uint != 0 as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"awkgram.y\0" as *const u8 as *const libc::c_char,
                4821 as libc::c_int,
            );
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"use of dcngettext(_\"...\") is incorrect: remove leading underscore\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        } else {
            dumpintlstr2(
                (*str1).sub.val.sp,
                (*str1).sub.val.slen,
                (*str2).sub.val.sp,
                (*str2).sub.val.slen,
            );
        }
    } else if (*r).d.fptr
        == Some(do_asort as unsafe extern "C" fn(libc::c_int) -> *mut NODE)
        || (*r).d.fptr
            == Some(do_asorti as unsafe extern "C" fn(libc::c_int) -> *mut NODE)
    {
        arg = (*subn).nexti;
        ip = (*arg).d.di;
        if (*ip).opcode as libc::c_uint == Op_push as libc::c_int as libc::c_uint {
            (*ip).opcode = Op_push_array;
        }
        if nexp >= 2 as libc::c_int {
            arg = (*ip).nexti;
            ip = (*arg).d.di;
            if (*ip).opcode as libc::c_uint == Op_push as libc::c_int as libc::c_uint {
                (*ip).opcode = Op_push_array;
            }
        }
    } else if (*r).d.fptr
        == Some(do_index as unsafe extern "C" fn(libc::c_int) -> *mut NODE)
    {
        arg = (*(*(*subn).nexti).d.di).nexti;
        ip = (*arg).d.di;
        if (*ip).opcode as libc::c_uint == Op_match_rec as libc::c_int as libc::c_uint
            || (*ip).opcode as libc::c_uint == Op_push_re as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"awkgram.y\0" as *const u8 as *const libc::c_char,
                4840 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"index: regexp constant as second argument is not allowed\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    }
    if !subn.is_null() {
        (*r).x.xl = count_expressions(&mut subn, 0 as libc::c_int != 0) as libc::c_long;
        return list_append(subn, r);
    }
    (*r).x.xl = 0 as libc::c_int as libc::c_long;
    return list_create(r);
}
unsafe extern "C" fn parms_shadow(
    mut pc: *mut INSTRUCTION,
    mut shadow: *mut bool,
) -> libc::c_int {
    let mut pcount: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: bool = 0 as libc::c_int != 0;
    let mut func: *mut NODE = 0 as *mut NODE;
    let mut fp_0: *mut NODE = 0 as *mut NODE;
    let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
    func = (*pc).x.xn;
    fname = (*func).sub.nodep.name;
    fp_0 = (*func).sub.nodep.rn;
    pcount = (*func).sub.nodep.l.ll as libc::c_int;
    if pcount == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    source = (*pc).d.name;
    sourceline = (*pc).source_line as libc::c_int;
    i = 0 as libc::c_int;
    while i < pcount {
        if !(lookup((*fp_0.offset(i as isize)).sub.nodep.name)).is_null() {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"awkgram.y\0" as *const u8 as *const libc::c_char,
                4892 as libc::c_int,
            );
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"function `%s': parameter `%s' shadows global variable\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                fname,
                (*fp_0.offset(i as isize)).sub.nodep.name,
            );
            ret = 1 as libc::c_int != 0;
        }
        i += 1;
        i;
    }
    *shadow = (*shadow as libc::c_int | ret as libc::c_int) != 0;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn valinfo(
    mut n: *mut NODE,
    mut print_func: Func_print,
    mut fp_0: *mut FILE,
) {
    if n == Nnull_string {
        print_func
            .unwrap()(
            fp_0,
            b"uninitialized scalar\n\0" as *const u8 as *const libc::c_char,
        );
    } else if (*n).flags as libc::c_uint & REGEX as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        print_func
            .unwrap()(
            fp_0,
            b"@/%.*s/\n\0" as *const u8 as *const libc::c_char,
            (*n).sub.val.slen,
            (*n).sub.val.sp,
        );
    } else if (*n).flags as libc::c_uint & STRING as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        pp_string_fp(
            print_func,
            fp_0,
            (*n).sub.val.sp,
            (*n).sub.val.slen,
            '"' as i32,
            0 as libc::c_int != 0,
        );
        print_func.unwrap()(fp_0, b"\n\0" as *const u8 as *const libc::c_char);
    } else if (*n).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        print_func
            .unwrap()(
            fp_0,
            b"%.17g\n\0" as *const u8 as *const libc::c_char,
            (*n).sub.val.fltnum,
        );
    } else {
        print_func
            .unwrap()(
            fp_0,
            b"?? flags %s\n\0" as *const u8 as *const libc::c_char,
            flags2str((*n).flags as libc::c_int),
        );
    };
}
pub unsafe extern "C" fn dump_vars(mut fname: *const libc::c_char) {
    let mut fp_0: *mut FILE = 0 as *mut FILE;
    let mut vars: *mut *mut NODE = 0 as *mut *mut NODE;
    if fname.is_null() {
        fp_0 = stderr;
    } else if strcmp(fname, b"-\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        fp_0 = stdout;
    } else {
        fp_0 = fopen(fname, b"w\0" as *const u8 as *const libc::c_char);
        if fp_0.is_null() {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"awkgram.y\0" as *const u8 as *const libc::c_char,
                4942 as libc::c_int,
            );
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"could not open `%s' for writing: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                fname,
                strerror(*__errno_location()),
            );
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"awkgram.y\0" as *const u8 as *const libc::c_char,
                4943 as libc::c_int,
            );
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"sending variable list to standard error\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            fp_0 = stderr;
        }
    }
    vars = variable_list();
    print_vars(
        vars,
        Some(
            fprintf
                as unsafe extern "C" fn(
                    *mut FILE,
                    *const libc::c_char,
                    ...
                ) -> libc::c_int,
        ),
        fp_0,
    );
    free(vars as *mut libc::c_void);
    if fp_0 != stdout && fp_0 != stderr && fclose(fp_0) != 0 as libc::c_int {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"awkgram.y\0" as *const u8 as *const libc::c_char,
            4951 as libc::c_int,
        );
        (Some(
            (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: close failed: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            fname,
            strerror(*__errno_location()),
        );
    }
}
pub unsafe extern "C" fn dump_funcs() {
    let mut funcs: *mut *mut NODE = 0 as *mut *mut NODE;
    funcs = function_list(1 as libc::c_int != 0);
    foreach_func(
        funcs,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut INSTRUCTION, *mut libc::c_void) -> libc::c_int,
            >,
            Option::<
                unsafe extern "C" fn(*mut INSTRUCTION, *mut libc::c_void) -> libc::c_int,
            >,
        >(
            Some(
                pp_func
                    as unsafe extern "C" fn(
                        *mut INSTRUCTION,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ),
        0 as *mut libc::c_void,
    );
    free(funcs as *mut libc::c_void);
}
pub unsafe extern "C" fn shadow_funcs() {
    static mut calls: libc::c_int = 0 as libc::c_int;
    let mut shadow: bool = 0 as libc::c_int != 0;
    let mut funcs: *mut *mut NODE = 0 as *mut *mut NODE;
    let fresh110 = calls;
    calls = calls + 1;
    if fresh110 != 0 as libc::c_int {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"awkgram.y\0" as *const u8 as *const libc::c_char,
            4976 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"shadow_funcs() called twice!\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    funcs = function_list(1 as libc::c_int != 0);
    foreach_func(
        funcs,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut INSTRUCTION, *mut bool) -> libc::c_int>,
            Option::<
                unsafe extern "C" fn(*mut INSTRUCTION, *mut libc::c_void) -> libc::c_int,
            >,
        >(
            Some(
                parms_shadow
                    as unsafe extern "C" fn(*mut INSTRUCTION, *mut bool) -> libc::c_int,
            ),
        ),
        &mut shadow as *mut bool as *mut libc::c_void,
    );
    free(funcs as *mut libc::c_void);
    if shadow as libc::c_int != 0
        && lintfunc
            == Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ())
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"awkgram.y\0" as *const u8 as *const libc::c_char,
            4984 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"there were shadowed variables\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
}
unsafe extern "C" fn mk_function(
    mut fi: *mut INSTRUCTION,
    mut def: *mut INSTRUCTION,
) -> *mut INSTRUCTION {
    let mut thisfunc: *mut NODE = 0 as *mut NODE;
    thisfunc = (*fi).x.xn;
    if !interblock_comment.is_null() {
        (*interblock_comment).source_line = 0 as libc::c_int as libc::c_short;
        merge_comments(interblock_comment, (*fi).comment);
        (*fi).comment = interblock_comment;
        interblock_comment = 0 as *mut INSTRUCTION;
    }
    list_append(def, bcalloc(Op_push_i, 1 as libc::c_int, 0 as libc::c_int));
    (*(*def).d.di).d.dn = dupnode(Nnull_string);
    list_append(def, bcalloc(Op_K_return, 1 as libc::c_int, 0 as libc::c_int));
    if !trailing_comment.is_null() {
        list_append(def, trailing_comment);
        trailing_comment = 0 as *mut INSTRUCTION;
    }
    if do_flags as libc::c_uint & DO_PRETTY_PRINT as libc::c_int as libc::c_uint != 0 {
        let ref mut fresh111 = (*fi.offset(3 as libc::c_int as isize)).nexti;
        *fresh111 = namespace_chain;
        namespace_chain = 0 as *mut INSTRUCTION;
        list_prepend(def, bcalloc(Op_exec_count, 1 as libc::c_int, 0 as libc::c_int));
    }
    let ref mut fresh112 = (*fi.offset(1 as libc::c_int as isize)).x.xi;
    *fresh112 = (*def).nexti;
    let ref mut fresh113 = (*fi.offset(1 as libc::c_int as isize)).d.di;
    *fresh113 = (*def).d.di;
    (*fi.offset(2 as libc::c_int as isize)).source_line = (*fi).source_line;
    (*fi.offset(2 as libc::c_int as isize)).x.xl = lastline as libc::c_long;
    (*fi).nexti = (*def).nexti;
    bcfree(def);
    list_append(rule_list, fi.offset(1 as libc::c_int as isize));
    func_use((*thisfunc).sub.nodep.name, FUNC_DEFINE);
    remove_params(thisfunc);
    return fi;
}
unsafe extern "C" fn install_function(
    mut fname: *mut libc::c_char,
    mut fi: *mut INSTRUCTION,
    mut plist: *mut INSTRUCTION,
) -> libc::c_int {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut f: *mut NODE = 0 as *mut NODE;
    let mut pcount: libc::c_int = 0 as libc::c_int;
    r = lookup(fname);
    if !r.is_null() {
        error_ln(
            (*fi).source_line as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"function name `%s' previously defined\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            fname,
        );
        return -(1 as libc::c_int);
    }
    if !plist.is_null() {
        pcount = ((*(*plist).d.di).x.xl + 1 as libc::c_int as libc::c_long)
            as libc::c_int;
    }
    f = install_symbol(fname, Node_func);
    if (*f).sub.nodep.name != fname {
        fname = (*f).sub.nodep.name;
    }
    (*fi).x.xn = f;
    (*f).sub.nodep.l.ll = pcount as libc::c_long;
    (*f).sub.nodep.r.iptr = fi;
    (*f).sub.nodep.rn = 0 as *mut exp_node;
    if pcount > 0 as libc::c_int {
        let mut pnames: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        pnames = check_params(fname, pcount, plist);
        (*f).sub.nodep.rn = make_params(pnames, pcount);
        free(pnames as *mut libc::c_void);
        install_params(f);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn check_params(
    mut fname: *mut libc::c_char,
    mut pcount: libc::c_int,
    mut list: *mut INSTRUCTION,
) -> *mut *mut libc::c_char {
    let mut p: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut np: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pnames: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    pnames = emalloc_real(
        (pcount as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
        b"check_params\0" as *const u8 as *const libc::c_char,
        b"pnames\0" as *const u8 as *const libc::c_char,
        b"awkgram.y\0" as *const u8 as *const libc::c_char,
        5102 as libc::c_int,
    ) as *mut *mut libc::c_char;
    i = 0 as libc::c_int;
    p = (*list).nexti;
    while !p.is_null() {
        np = (*p).nexti;
        name = (*p).d.name;
        (*p).d.name = 0 as *mut libc::c_char;
        if strcmp(name, fname) == 0 as libc::c_int {
            error_ln(
                (*p).source_line as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"function `%s': cannot use function name as parameter name\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                fname,
            );
        } else if is_std_var(name) != 0 {
            error_ln(
                (*p).source_line as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"function `%s': cannot use special variable `%s' as a function parameter\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                fname,
                name,
            );
        } else if !(strchr(name, ':' as i32)).is_null() {
            error_ln(
                (*p).source_line as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"function `%s': parameter `%s' cannot contain a namespace\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                fname,
                name,
            );
        }
        j = 0 as libc::c_int;
        while j < i {
            if strcmp(name, *pnames.offset(j as isize)) == 0 as libc::c_int {
                error_ln(
                    (*p).source_line as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"function `%s': parameter #%d, `%s', duplicates parameter #%d\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    fname,
                    i + 1 as libc::c_int,
                    name,
                    j + 1 as libc::c_int,
                );
            }
            j += 1;
            j;
        }
        let ref mut fresh114 = *pnames.offset(i as isize);
        *fresh114 = name;
        bcfree(p);
        i += 1;
        i;
        p = np;
    }
    bcfree(list);
    return pnames;
}
static mut ftable: [*mut fdesc; 1021] = [0 as *const fdesc as *mut fdesc; 1021];
unsafe extern "C" fn func_use(mut name: *const libc::c_char, mut how: defref) {
    let mut current_block: u64;
    let mut fp_0: *mut fdesc = 0 as *mut fdesc;
    let mut len: libc::c_int = 0;
    let mut ind: libc::c_int = 0;
    len = strlen(name) as libc::c_int;
    ind = hash
        .unwrap()(
        name,
        len as size_t,
        1021 as libc::c_int as libc::c_ulong,
        0 as *mut size_t,
    ) as libc::c_int;
    fp_0 = ftable[ind as usize];
    loop {
        if fp_0.is_null() {
            current_block = 7095457783677275021;
            break;
        }
        if strcmp((*fp_0).name, name) == 0 as libc::c_int {
            current_block = 3388965662691613646;
            break;
        }
        fp_0 = (*fp_0).next;
    }
    match current_block {
        7095457783677275021 => {
            fp_0 = ezalloc_real(
                ::std::mem::size_of::<fdesc>() as libc::c_ulong,
                b"func_use\0" as *const u8 as *const libc::c_char,
                b"fp\0" as *const u8 as *const libc::c_char,
                b"awkgram.y\0" as *const u8 as *const libc::c_char,
                5171 as libc::c_int,
            ) as *mut fdesc;
            (*fp_0)
                .name = emalloc_real(
                (len + 1 as libc::c_int) as size_t,
                b"func_use\0" as *const u8 as *const libc::c_char,
                b"fp->name\0" as *const u8 as *const libc::c_char,
                b"awkgram.y\0" as *const u8 as *const libc::c_char,
                5172 as libc::c_int,
            ) as *mut libc::c_char;
            strcpy((*fp_0).name, name);
            (*fp_0).next = ftable[ind as usize];
            ftable[ind as usize] = fp_0;
        }
        _ => {}
    }
    if how as libc::c_uint == FUNC_DEFINE as libc::c_int as libc::c_uint {
        (*fp_0).defined += 1;
        (*fp_0).defined;
    } else if how as libc::c_uint == FUNC_EXT as libc::c_int as libc::c_uint {
        (*fp_0).defined += 1;
        (*fp_0).defined;
        (*fp_0).extension += 1;
        (*fp_0).extension;
    } else {
        (*fp_0).used += 1;
        (*fp_0).used;
    };
}
pub unsafe extern "C" fn track_ext_func(mut name: *const libc::c_char) {
    func_use(name, FUNC_EXT);
}
unsafe extern "C" fn check_funcs() {
    let mut fp_0: *mut fdesc = 0 as *mut fdesc;
    let mut next: *mut fdesc = 0 as *mut fdesc;
    let mut i: libc::c_int = 0;
    if !(in_main_context() == 0) {
        i = 0 as libc::c_int;
        while i < 1021 as libc::c_int {
            fp_0 = ftable[i as usize];
            while !fp_0.is_null() {
                if do_flags as libc::c_uint
                    & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                        as libc::c_uint != 0 && (*fp_0).extension == 0
                {
                    if (*fp_0).defined as libc::c_int == 0 as libc::c_int {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"awkgram.y\0" as *const u8 as *const libc::c_char,
                            5215 as libc::c_int,
                        );
                        (Some(lintfunc.unwrap()))
                            .unwrap()(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"function `%s' called but never defined\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (*fp_0).name,
                        );
                    }
                    if (*fp_0).used as libc::c_int == 0 as libc::c_int {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"awkgram.y\0" as *const u8 as *const libc::c_char,
                            5219 as libc::c_int,
                        );
                        (Some(lintfunc.unwrap()))
                            .unwrap()(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"function `%s' defined but never called directly\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (*fp_0).name,
                        );
                    }
                }
                fp_0 = (*fp_0).next;
            }
            i += 1;
            i;
        }
    }
    i = 0 as libc::c_int;
    while i < 1021 as libc::c_int {
        fp_0 = ftable[i as usize];
        while !fp_0.is_null() {
            next = (*fp_0).next;
            free((*fp_0).name as *mut libc::c_void);
            free(fp_0 as *mut libc::c_void);
            fp_0 = next;
        }
        ftable[i as usize] = 0 as *mut fdesc;
        i += 1;
        i;
    }
}
unsafe extern "C" fn param_sanity(mut arglist: *mut INSTRUCTION) {
    let mut argl: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut arg: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut i: libc::c_int = 1 as libc::c_int;
    if arglist.is_null() {
        return;
    }
    argl = (*arglist).nexti;
    while !argl.is_null() {
        arg = (*argl).d.di;
        if (*arg).opcode as libc::c_uint == Op_match_rec as libc::c_int as libc::c_uint {
            warning_ln(
                (*arg).source_line as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"regexp constant for parameter #%d yields boolean value\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                i,
            );
        }
        argl = (*arg).nexti;
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn variable(
    mut location: libc::c_int,
    mut name: *mut libc::c_char,
    mut type_0: NODETYPE,
) -> *mut NODE {
    let mut r: *mut NODE = 0 as *mut NODE;
    r = lookup(name);
    if !r.is_null() {
        if (*r).type_0 as libc::c_uint == Node_func as libc::c_int as libc::c_uint
            || (*r).type_0 as libc::c_uint
                == Node_ext_func as libc::c_int as libc::c_uint
        {
            error_ln(
                location,
                dcgettext(
                    0 as *const libc::c_char,
                    b"function `%s' called with space between name and `(',\nor used as a variable or an array\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*r).sub.nodep.name,
            );
        }
    } else {
        return install_symbol(name, type_0)
    }
    free(name as *mut libc::c_void);
    return r;
}
pub unsafe extern "C" fn make_regnode(
    mut type_0: NODETYPE,
    mut exp: *mut NODE,
) -> *mut NODE {
    let mut n: *mut NODE = 0 as *mut NODE;
    n = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
    if !n.is_null() {
        nextfree[BLOCK_NODE as libc::c_int as usize]
            .freep = (*(n as *mut block_item)).freep;
    } else {
        n = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
    };
    memset(
        n as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<NODE>() as libc::c_ulong,
    );
    (*n).type_0 = type_0;
    (*n).flags = MALLOC;
    if type_0 as libc::c_uint == Node_regex as libc::c_int as libc::c_uint {
        (*n)
            .sub
            .nodep
            .r
            .preg[0 as libc::c_int
            as usize] = make_regexp(
            (*exp).sub.val.sp,
            (*exp).sub.val.slen,
            0 as libc::c_int != 0,
            1 as libc::c_int != 0,
            0 as libc::c_int != 0,
        );
        if ((*n).sub.nodep.r.preg[0 as libc::c_int as usize]).is_null() {
            let ref mut fresh115 = (*(n as *mut block_item)).freep;
            *fresh115 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
            nextfree[BLOCK_NODE as libc::c_int as usize].freep = n as *mut block_item;
            return 0 as *mut NODE;
        }
        (*n).sub.nodep.x.extra = exp;
        (*n).sub.nodep.reflags = CONSTANT;
    }
    return n;
}
unsafe extern "C" fn mk_rexp(mut list: *mut INSTRUCTION) -> *mut NODE {
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    ip = (*list).nexti;
    if ip == (*list).d.di
        && (*ip).opcode as libc::c_uint == Op_match_rec as libc::c_int as libc::c_uint
    {
        (*ip).opcode = Op_push_re;
    } else if !(ip == (*list).d.di
        && (*ip).opcode as libc::c_uint == Op_push_re as libc::c_int as libc::c_uint)
    {
        ip = bcalloc(Op_push_re, 1 as libc::c_int, 0 as libc::c_int);
        (*ip).d.dn = make_regnode(Node_dynregex, 0 as *mut NODE);
        (*ip).nexti = (*(*list).d.di).nexti;
        (*(*list).d.di).nexti = ip;
        (*list).d.di = ip;
    }
    return (*ip).d.dn;
}
unsafe extern "C" fn isnoeffect(mut type_0: OPCODE) -> libc::c_int {
    match type_0 as libc::c_uint {
        1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 16 | 13 | 11 | 12 | 22 | 24 | 39 | 41
        | 42 | 43 | 44 | 45 | 46 | 47 | 48 | 50 | 49 | 25 | 72 => return 1 as libc::c_int,
        38 | 40 | 75 | 78 | 80 | 86 | 106 => return 1 as libc::c_int,
        _ => {}
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn make_assignable(mut ip: *mut INSTRUCTION) -> *mut INSTRUCTION {
    match (*ip).opcode as libc::c_uint {
        75 => {
            (*ip).opcode = Op_push_lhs;
            return ip;
        }
        24 => {
            (*ip).opcode = Op_field_spec_lhs;
            return ip;
        }
        16 => {
            (*ip).opcode = Op_subscript_lhs;
            return ip;
        }
        97 => return ip,
        _ => {}
    }
    return 0 as *mut INSTRUCTION;
}
pub unsafe extern "C" fn stopme(mut nargs: libc::c_int) -> *mut NODE {
    return make_number.unwrap()(0.0f64);
}
unsafe extern "C" fn dumpintlstr(mut str: *const libc::c_char, mut len: size_t) {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    if !source.is_null() {
        cp = source;
        while *cp.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
            && *cp.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
        {
            cp = cp.offset(2 as libc::c_int as isize);
        }
        printf(b"#: %s:%d\n\0" as *const u8 as *const libc::c_char, cp, sourceline);
    }
    printf(b"msgid \0" as *const u8 as *const libc::c_char);
    pp_string_fp(
        Some(
            fprintf
                as unsafe extern "C" fn(
                    *mut FILE,
                    *const libc::c_char,
                    ...
                ) -> libc::c_int,
        ),
        stdout,
        str,
        len,
        '"' as i32,
        1 as libc::c_int != 0,
    );
    putchar('\n' as i32);
    printf(b"msgstr \"\"\n\n\0" as *const u8 as *const libc::c_char);
    fflush(stdout);
}
unsafe extern "C" fn dumpintlstr2(
    mut str1: *const libc::c_char,
    mut len1: size_t,
    mut str2: *const libc::c_char,
    mut len2: size_t,
) {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    if !source.is_null() {
        cp = source;
        while *cp.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
            && *cp.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
        {
            cp = cp.offset(2 as libc::c_int as isize);
        }
        printf(b"#: %s:%d\n\0" as *const u8 as *const libc::c_char, cp, sourceline);
    }
    printf(b"msgid \0" as *const u8 as *const libc::c_char);
    pp_string_fp(
        Some(
            fprintf
                as unsafe extern "C" fn(
                    *mut FILE,
                    *const libc::c_char,
                    ...
                ) -> libc::c_int,
        ),
        stdout,
        str1,
        len1,
        '"' as i32,
        1 as libc::c_int != 0,
    );
    putchar('\n' as i32);
    printf(b"msgid_plural \0" as *const u8 as *const libc::c_char);
    pp_string_fp(
        Some(
            fprintf
                as unsafe extern "C" fn(
                    *mut FILE,
                    *const libc::c_char,
                    ...
                ) -> libc::c_int,
        ),
        stdout,
        str2,
        len2,
        '"' as i32,
        1 as libc::c_int != 0,
    );
    putchar('\n' as i32);
    printf(b"msgstr[0] \"\"\nmsgstr[1] \"\"\n\n\0" as *const u8 as *const libc::c_char);
    fflush(stdout);
}
unsafe extern "C" fn mk_binary(
    mut s1: *mut INSTRUCTION,
    mut s2: *mut INSTRUCTION,
    mut op: *mut INSTRUCTION,
) -> *mut INSTRUCTION {
    let mut current_block: u64;
    let mut ip1: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut ip2: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut lint_plus: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut res: libc::c_double = 0.;
    ip2 = (*s2).nexti;
    if (*s2).d.di == ip2
        && (*ip2).opcode as libc::c_uint == Op_push_i as libc::c_int as libc::c_uint
    {
        ip1 = (*s1).nexti;
        if do_optimize as libc::c_int != 0 && ip1 == (*s1).d.di
            && (*ip1).opcode as libc::c_uint == Op_push_i as libc::c_int as libc::c_uint
            && (*(*ip1).d.dn).flags as libc::c_uint
                & (MPFN as libc::c_int | MPZN as libc::c_int | STRCUR as libc::c_int
                    | STRING as libc::c_int) as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            && (*(*ip2).d.dn).flags as libc::c_uint
                & (MPFN as libc::c_int | MPZN as libc::c_int | STRCUR as libc::c_int
                    | STRING as libc::c_int) as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
        {
            let mut n1: *mut NODE = (*ip1).d.dn;
            let mut n2: *mut NODE = (*ip2).d.dn;
            res = (*force_number(n1)).sub.val.fltnum;
            force_number(n2);
            match (*op).opcode as libc::c_uint {
                1 => {
                    current_block = 6920085021744161962;
                    match current_block {
                        7162666314531089940 => {
                            res = calc_exp(res, (*n2).sub.val.fltnum);
                            current_block = 6057473163062296781;
                        }
                        12340593168448583670 => {
                            if (*n2).flags as libc::c_uint
                                & NUMBER as libc::c_int as libc::c_uint
                                != 0 as libc::c_int as libc::c_uint
                                && (*n2).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"division by zero attempted\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                current_block = 14358609801444731309;
                            } else {
                                res /= (*n2).sub.val.fltnum;
                                current_block = 6057473163062296781;
                            }
                        }
                        6920085021744161962 => {
                            res *= (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        17200474013191995185 => {
                            res += (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        1257080933869083286 => {
                            res -= (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        _ => {
                            if (*n2).flags as libc::c_uint
                                & NUMBER as libc::c_int as libc::c_uint
                                != 0 as libc::c_int as libc::c_uint
                                && (*n2).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"division by zero attempted in `%%'\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                current_block = 14358609801444731309;
                            } else {
                                res = fmod(res, (*n2).sub.val.fltnum);
                                current_block = 6057473163062296781;
                            }
                        }
                    }
                    match current_block {
                        14358609801444731309 => {}
                        _ => {
                            (*op).opcode = Op_push_i;
                            (*op).d.dn = make_number.unwrap()(res);
                            unref(n1);
                            unref(n2);
                            bcfree(ip1);
                            bcfree(ip2);
                            bcfree(s1);
                            bcfree(s2);
                            return list_create(op);
                        }
                    }
                }
                3 => {
                    current_block = 12340593168448583670;
                    match current_block {
                        7162666314531089940 => {
                            res = calc_exp(res, (*n2).sub.val.fltnum);
                            current_block = 6057473163062296781;
                        }
                        12340593168448583670 => {
                            if (*n2).flags as libc::c_uint
                                & NUMBER as libc::c_int as libc::c_uint
                                != 0 as libc::c_int as libc::c_uint
                                && (*n2).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"division by zero attempted\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                current_block = 14358609801444731309;
                            } else {
                                res /= (*n2).sub.val.fltnum;
                                current_block = 6057473163062296781;
                            }
                        }
                        6920085021744161962 => {
                            res *= (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        17200474013191995185 => {
                            res += (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        1257080933869083286 => {
                            res -= (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        _ => {
                            if (*n2).flags as libc::c_uint
                                & NUMBER as libc::c_int as libc::c_uint
                                != 0 as libc::c_int as libc::c_uint
                                && (*n2).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"division by zero attempted in `%%'\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                current_block = 14358609801444731309;
                            } else {
                                res = fmod(res, (*n2).sub.val.fltnum);
                                current_block = 6057473163062296781;
                            }
                        }
                    }
                    match current_block {
                        14358609801444731309 => {}
                        _ => {
                            (*op).opcode = Op_push_i;
                            (*op).d.dn = make_number.unwrap()(res);
                            unref(n1);
                            unref(n2);
                            bcfree(ip1);
                            bcfree(ip2);
                            bcfree(s1);
                            bcfree(s2);
                            return list_create(op);
                        }
                    }
                }
                5 => {
                    current_block = 13927120366621027390;
                    match current_block {
                        7162666314531089940 => {
                            res = calc_exp(res, (*n2).sub.val.fltnum);
                            current_block = 6057473163062296781;
                        }
                        12340593168448583670 => {
                            if (*n2).flags as libc::c_uint
                                & NUMBER as libc::c_int as libc::c_uint
                                != 0 as libc::c_int as libc::c_uint
                                && (*n2).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"division by zero attempted\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                current_block = 14358609801444731309;
                            } else {
                                res /= (*n2).sub.val.fltnum;
                                current_block = 6057473163062296781;
                            }
                        }
                        6920085021744161962 => {
                            res *= (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        17200474013191995185 => {
                            res += (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        1257080933869083286 => {
                            res -= (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        _ => {
                            if (*n2).flags as libc::c_uint
                                & NUMBER as libc::c_int as libc::c_uint
                                != 0 as libc::c_int as libc::c_uint
                                && (*n2).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"division by zero attempted in `%%'\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                current_block = 14358609801444731309;
                            } else {
                                res = fmod(res, (*n2).sub.val.fltnum);
                                current_block = 6057473163062296781;
                            }
                        }
                    }
                    match current_block {
                        14358609801444731309 => {}
                        _ => {
                            (*op).opcode = Op_push_i;
                            (*op).d.dn = make_number.unwrap()(res);
                            unref(n1);
                            unref(n2);
                            bcfree(ip1);
                            bcfree(ip2);
                            bcfree(s1);
                            bcfree(s2);
                            return list_create(op);
                        }
                    }
                }
                7 => {
                    current_block = 17200474013191995185;
                    match current_block {
                        7162666314531089940 => {
                            res = calc_exp(res, (*n2).sub.val.fltnum);
                            current_block = 6057473163062296781;
                        }
                        12340593168448583670 => {
                            if (*n2).flags as libc::c_uint
                                & NUMBER as libc::c_int as libc::c_uint
                                != 0 as libc::c_int as libc::c_uint
                                && (*n2).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"division by zero attempted\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                current_block = 14358609801444731309;
                            } else {
                                res /= (*n2).sub.val.fltnum;
                                current_block = 6057473163062296781;
                            }
                        }
                        6920085021744161962 => {
                            res *= (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        17200474013191995185 => {
                            res += (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        1257080933869083286 => {
                            res -= (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        _ => {
                            if (*n2).flags as libc::c_uint
                                & NUMBER as libc::c_int as libc::c_uint
                                != 0 as libc::c_int as libc::c_uint
                                && (*n2).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"division by zero attempted in `%%'\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                current_block = 14358609801444731309;
                            } else {
                                res = fmod(res, (*n2).sub.val.fltnum);
                                current_block = 6057473163062296781;
                            }
                        }
                    }
                    match current_block {
                        14358609801444731309 => {}
                        _ => {
                            (*op).opcode = Op_push_i;
                            (*op).d.dn = make_number.unwrap()(res);
                            unref(n1);
                            unref(n2);
                            bcfree(ip1);
                            bcfree(ip2);
                            bcfree(s1);
                            bcfree(s2);
                            return list_create(op);
                        }
                    }
                }
                9 => {
                    current_block = 1257080933869083286;
                    match current_block {
                        7162666314531089940 => {
                            res = calc_exp(res, (*n2).sub.val.fltnum);
                            current_block = 6057473163062296781;
                        }
                        12340593168448583670 => {
                            if (*n2).flags as libc::c_uint
                                & NUMBER as libc::c_int as libc::c_uint
                                != 0 as libc::c_int as libc::c_uint
                                && (*n2).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"division by zero attempted\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                current_block = 14358609801444731309;
                            } else {
                                res /= (*n2).sub.val.fltnum;
                                current_block = 6057473163062296781;
                            }
                        }
                        6920085021744161962 => {
                            res *= (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        17200474013191995185 => {
                            res += (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        1257080933869083286 => {
                            res -= (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        _ => {
                            if (*n2).flags as libc::c_uint
                                & NUMBER as libc::c_int as libc::c_uint
                                != 0 as libc::c_int as libc::c_uint
                                && (*n2).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"division by zero attempted in `%%'\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                current_block = 14358609801444731309;
                            } else {
                                res = fmod(res, (*n2).sub.val.fltnum);
                                current_block = 6057473163062296781;
                            }
                        }
                    }
                    match current_block {
                        14358609801444731309 => {}
                        _ => {
                            (*op).opcode = Op_push_i;
                            (*op).d.dn = make_number.unwrap()(res);
                            unref(n1);
                            unref(n2);
                            bcfree(ip1);
                            bcfree(ip2);
                            bcfree(s1);
                            bcfree(s2);
                            return list_create(op);
                        }
                    }
                }
                11 => {
                    current_block = 7162666314531089940;
                    match current_block {
                        7162666314531089940 => {
                            res = calc_exp(res, (*n2).sub.val.fltnum);
                            current_block = 6057473163062296781;
                        }
                        12340593168448583670 => {
                            if (*n2).flags as libc::c_uint
                                & NUMBER as libc::c_int as libc::c_uint
                                != 0 as libc::c_int as libc::c_uint
                                && (*n2).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"division by zero attempted\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                current_block = 14358609801444731309;
                            } else {
                                res /= (*n2).sub.val.fltnum;
                                current_block = 6057473163062296781;
                            }
                        }
                        6920085021744161962 => {
                            res *= (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        17200474013191995185 => {
                            res += (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        1257080933869083286 => {
                            res -= (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        _ => {
                            if (*n2).flags as libc::c_uint
                                & NUMBER as libc::c_int as libc::c_uint
                                != 0 as libc::c_int as libc::c_uint
                                && (*n2).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"division by zero attempted in `%%'\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                current_block = 14358609801444731309;
                            } else {
                                res = fmod(res, (*n2).sub.val.fltnum);
                                current_block = 6057473163062296781;
                            }
                        }
                    }
                    match current_block {
                        14358609801444731309 => {}
                        _ => {
                            (*op).opcode = Op_push_i;
                            (*op).d.dn = make_number.unwrap()(res);
                            unref(n1);
                            unref(n2);
                            bcfree(ip1);
                            bcfree(ip2);
                            bcfree(s1);
                            bcfree(s2);
                            return list_create(op);
                        }
                    }
                }
                _ => {}
            }
        } else {
            match (*op).opcode as libc::c_uint {
                1 => {
                    current_block = 4787741948862734335;
                    match current_block {
                        13194918759237816078 => {
                            (*op).opcode = Op_exp_i;
                            current_block = 980989089337379490;
                        }
                        2731264325245666651 => {
                            if (*(*ip2).d.dn).flags as libc::c_uint
                                & NUMBER as libc::c_int as libc::c_uint
                                != 0 as libc::c_int as libc::c_uint
                                && (*(*ip2).d.dn).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"division by zero attempted in `%%'\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                current_block = 14358609801444731309;
                            } else {
                                (*op).opcode = Op_mod_i;
                                current_block = 980989089337379490;
                            }
                        }
                        11942264283264667978 => {
                            if (*(*ip2).d.dn).flags as libc::c_uint
                                & NUMBER as libc::c_int as libc::c_uint
                                != 0 as libc::c_int as libc::c_uint
                                && (*(*ip2).d.dn).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"division by zero attempted\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                current_block = 14358609801444731309;
                            } else {
                                (*op).opcode = Op_quotient_i;
                                current_block = 980989089337379490;
                            }
                        }
                        4787741948862734335 => {
                            (*op).opcode = Op_times_i;
                            current_block = 980989089337379490;
                        }
                        15207532774242197982 => {
                            (*op).opcode = Op_minus_i;
                            current_block = 980989089337379490;
                        }
                        _ => {
                            if do_flags as libc::c_uint
                                & (DO_LINT_INVALID as libc::c_int
                                    | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                            {
                                current_block = 14358609801444731309;
                            } else {
                                (*op).opcode = Op_plus_i;
                                current_block = 980989089337379490;
                            }
                        }
                    }
                    match current_block {
                        14358609801444731309 => {}
                        _ => {
                            (*op).d.dn = (*ip2).d.dn;
                            bcfree(ip2);
                            bcfree(s2);
                            return list_append(s1, op);
                        }
                    }
                }
                3 => {
                    current_block = 11942264283264667978;
                    match current_block {
                        13194918759237816078 => {
                            (*op).opcode = Op_exp_i;
                            current_block = 980989089337379490;
                        }
                        2731264325245666651 => {
                            if (*(*ip2).d.dn).flags as libc::c_uint
                                & NUMBER as libc::c_int as libc::c_uint
                                != 0 as libc::c_int as libc::c_uint
                                && (*(*ip2).d.dn).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"division by zero attempted in `%%'\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                current_block = 14358609801444731309;
                            } else {
                                (*op).opcode = Op_mod_i;
                                current_block = 980989089337379490;
                            }
                        }
                        11942264283264667978 => {
                            if (*(*ip2).d.dn).flags as libc::c_uint
                                & NUMBER as libc::c_int as libc::c_uint
                                != 0 as libc::c_int as libc::c_uint
                                && (*(*ip2).d.dn).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"division by zero attempted\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                current_block = 14358609801444731309;
                            } else {
                                (*op).opcode = Op_quotient_i;
                                current_block = 980989089337379490;
                            }
                        }
                        4787741948862734335 => {
                            (*op).opcode = Op_times_i;
                            current_block = 980989089337379490;
                        }
                        15207532774242197982 => {
                            (*op).opcode = Op_minus_i;
                            current_block = 980989089337379490;
                        }
                        _ => {
                            if do_flags as libc::c_uint
                                & (DO_LINT_INVALID as libc::c_int
                                    | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                            {
                                current_block = 14358609801444731309;
                            } else {
                                (*op).opcode = Op_plus_i;
                                current_block = 980989089337379490;
                            }
                        }
                    }
                    match current_block {
                        14358609801444731309 => {}
                        _ => {
                            (*op).d.dn = (*ip2).d.dn;
                            bcfree(ip2);
                            bcfree(s2);
                            return list_append(s1, op);
                        }
                    }
                }
                5 => {
                    current_block = 2731264325245666651;
                    match current_block {
                        13194918759237816078 => {
                            (*op).opcode = Op_exp_i;
                            current_block = 980989089337379490;
                        }
                        2731264325245666651 => {
                            if (*(*ip2).d.dn).flags as libc::c_uint
                                & NUMBER as libc::c_int as libc::c_uint
                                != 0 as libc::c_int as libc::c_uint
                                && (*(*ip2).d.dn).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"division by zero attempted in `%%'\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                current_block = 14358609801444731309;
                            } else {
                                (*op).opcode = Op_mod_i;
                                current_block = 980989089337379490;
                            }
                        }
                        11942264283264667978 => {
                            if (*(*ip2).d.dn).flags as libc::c_uint
                                & NUMBER as libc::c_int as libc::c_uint
                                != 0 as libc::c_int as libc::c_uint
                                && (*(*ip2).d.dn).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"division by zero attempted\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                current_block = 14358609801444731309;
                            } else {
                                (*op).opcode = Op_quotient_i;
                                current_block = 980989089337379490;
                            }
                        }
                        4787741948862734335 => {
                            (*op).opcode = Op_times_i;
                            current_block = 980989089337379490;
                        }
                        15207532774242197982 => {
                            (*op).opcode = Op_minus_i;
                            current_block = 980989089337379490;
                        }
                        _ => {
                            if do_flags as libc::c_uint
                                & (DO_LINT_INVALID as libc::c_int
                                    | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                            {
                                current_block = 14358609801444731309;
                            } else {
                                (*op).opcode = Op_plus_i;
                                current_block = 980989089337379490;
                            }
                        }
                    }
                    match current_block {
                        14358609801444731309 => {}
                        _ => {
                            (*op).d.dn = (*ip2).d.dn;
                            bcfree(ip2);
                            bcfree(s2);
                            return list_append(s1, op);
                        }
                    }
                }
                7 => {
                    current_block = 17359096694954187339;
                    match current_block {
                        13194918759237816078 => {
                            (*op).opcode = Op_exp_i;
                            current_block = 980989089337379490;
                        }
                        2731264325245666651 => {
                            if (*(*ip2).d.dn).flags as libc::c_uint
                                & NUMBER as libc::c_int as libc::c_uint
                                != 0 as libc::c_int as libc::c_uint
                                && (*(*ip2).d.dn).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"division by zero attempted in `%%'\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                current_block = 14358609801444731309;
                            } else {
                                (*op).opcode = Op_mod_i;
                                current_block = 980989089337379490;
                            }
                        }
                        11942264283264667978 => {
                            if (*(*ip2).d.dn).flags as libc::c_uint
                                & NUMBER as libc::c_int as libc::c_uint
                                != 0 as libc::c_int as libc::c_uint
                                && (*(*ip2).d.dn).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"division by zero attempted\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                current_block = 14358609801444731309;
                            } else {
                                (*op).opcode = Op_quotient_i;
                                current_block = 980989089337379490;
                            }
                        }
                        4787741948862734335 => {
                            (*op).opcode = Op_times_i;
                            current_block = 980989089337379490;
                        }
                        15207532774242197982 => {
                            (*op).opcode = Op_minus_i;
                            current_block = 980989089337379490;
                        }
                        _ => {
                            if do_flags as libc::c_uint
                                & (DO_LINT_INVALID as libc::c_int
                                    | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                            {
                                current_block = 14358609801444731309;
                            } else {
                                (*op).opcode = Op_plus_i;
                                current_block = 980989089337379490;
                            }
                        }
                    }
                    match current_block {
                        14358609801444731309 => {}
                        _ => {
                            (*op).d.dn = (*ip2).d.dn;
                            bcfree(ip2);
                            bcfree(s2);
                            return list_append(s1, op);
                        }
                    }
                }
                9 => {
                    current_block = 15207532774242197982;
                    match current_block {
                        13194918759237816078 => {
                            (*op).opcode = Op_exp_i;
                            current_block = 980989089337379490;
                        }
                        2731264325245666651 => {
                            if (*(*ip2).d.dn).flags as libc::c_uint
                                & NUMBER as libc::c_int as libc::c_uint
                                != 0 as libc::c_int as libc::c_uint
                                && (*(*ip2).d.dn).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"division by zero attempted in `%%'\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                current_block = 14358609801444731309;
                            } else {
                                (*op).opcode = Op_mod_i;
                                current_block = 980989089337379490;
                            }
                        }
                        11942264283264667978 => {
                            if (*(*ip2).d.dn).flags as libc::c_uint
                                & NUMBER as libc::c_int as libc::c_uint
                                != 0 as libc::c_int as libc::c_uint
                                && (*(*ip2).d.dn).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"division by zero attempted\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                current_block = 14358609801444731309;
                            } else {
                                (*op).opcode = Op_quotient_i;
                                current_block = 980989089337379490;
                            }
                        }
                        4787741948862734335 => {
                            (*op).opcode = Op_times_i;
                            current_block = 980989089337379490;
                        }
                        15207532774242197982 => {
                            (*op).opcode = Op_minus_i;
                            current_block = 980989089337379490;
                        }
                        _ => {
                            if do_flags as libc::c_uint
                                & (DO_LINT_INVALID as libc::c_int
                                    | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                            {
                                current_block = 14358609801444731309;
                            } else {
                                (*op).opcode = Op_plus_i;
                                current_block = 980989089337379490;
                            }
                        }
                    }
                    match current_block {
                        14358609801444731309 => {}
                        _ => {
                            (*op).d.dn = (*ip2).d.dn;
                            bcfree(ip2);
                            bcfree(s2);
                            return list_append(s1, op);
                        }
                    }
                }
                11 => {
                    current_block = 13194918759237816078;
                    match current_block {
                        13194918759237816078 => {
                            (*op).opcode = Op_exp_i;
                            current_block = 980989089337379490;
                        }
                        2731264325245666651 => {
                            if (*(*ip2).d.dn).flags as libc::c_uint
                                & NUMBER as libc::c_int as libc::c_uint
                                != 0 as libc::c_int as libc::c_uint
                                && (*(*ip2).d.dn).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"division by zero attempted in `%%'\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                current_block = 14358609801444731309;
                            } else {
                                (*op).opcode = Op_mod_i;
                                current_block = 980989089337379490;
                            }
                        }
                        11942264283264667978 => {
                            if (*(*ip2).d.dn).flags as libc::c_uint
                                & NUMBER as libc::c_int as libc::c_uint
                                != 0 as libc::c_int as libc::c_uint
                                && (*(*ip2).d.dn).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"division by zero attempted\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                current_block = 14358609801444731309;
                            } else {
                                (*op).opcode = Op_quotient_i;
                                current_block = 980989089337379490;
                            }
                        }
                        4787741948862734335 => {
                            (*op).opcode = Op_times_i;
                            current_block = 980989089337379490;
                        }
                        15207532774242197982 => {
                            (*op).opcode = Op_minus_i;
                            current_block = 980989089337379490;
                        }
                        _ => {
                            if do_flags as libc::c_uint
                                & (DO_LINT_INVALID as libc::c_int
                                    | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                            {
                                current_block = 14358609801444731309;
                            } else {
                                (*op).opcode = Op_plus_i;
                                current_block = 980989089337379490;
                            }
                        }
                    }
                    match current_block {
                        14358609801444731309 => {}
                        _ => {
                            (*op).d.dn = (*ip2).d.dn;
                            bcfree(ip2);
                            bcfree(s2);
                            return list_append(s1, op);
                        }
                    }
                }
                _ => {}
            }
        }
    }
    list_merge(s1, s2);
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0 && (*op).opcode as libc::c_uint == Op_plus as libc::c_int as libc::c_uint
    {
        lint_plus = bcalloc(Op_lint_plus, 1 as libc::c_int, 0 as libc::c_int);
        list_append(s1, lint_plus);
    }
    return list_append(s1, op);
}
unsafe extern "C" fn mk_boolean(
    mut left: *mut INSTRUCTION,
    mut right: *mut INSTRUCTION,
    mut op: *mut INSTRUCTION,
) -> *mut INSTRUCTION {
    let mut tp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut opc: OPCODE = Op_illegal;
    let mut final_opc: OPCODE = Op_illegal;
    opc = (*op).opcode;
    final_opc = (if opc as libc::c_uint == Op_or as libc::c_int as libc::c_uint {
        Op_or_final as libc::c_int
    } else {
        Op_and_final as libc::c_int
    }) as OPCODE;
    add_lint(right, LINT_assign_in_cond);
    tp = (*left).d.di;
    if (*tp).opcode as libc::c_uint != final_opc as libc::c_uint {
        list_append(right, bcalloc(final_opc, 1 as libc::c_int, 0 as libc::c_int));
        add_lint(left, LINT_assign_in_cond);
        list_append(left, op);
        (*(*left).d.di).d.di = (*right).d.di;
        (*(*left).d.di).x.xi = (*left).d.di;
        (*(*right).d.di).x.xi = (*left).d.di;
    } else {
        let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
        (*op).opcode = final_opc;
        list_append(right, op);
        (*op).x.xi = tp;
        (*tp).opcode = opc;
        (*tp).d.di = op;
        ip = (*tp).x.xi;
        loop {
            (*ip).d.di = op;
            if (*ip).x.xi == ip {
                break;
            }
            ip = (*ip).x.xi;
        }
    }
    return list_merge(left, right);
}
unsafe extern "C" fn mk_condition(
    mut cond: *mut INSTRUCTION,
    mut ifp: *mut INSTRUCTION,
    mut true_branch: *mut INSTRUCTION,
    mut elsep: *mut INSTRUCTION,
    mut false_branch: *mut INSTRUCTION,
) -> *mut INSTRUCTION {
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut setup_else_part: bool = 1 as libc::c_int != 0;
    if false_branch.is_null() {
        false_branch = list_create(
            bcalloc(Op_no_op, 1 as libc::c_int, 0 as libc::c_int),
        );
        if elsep.is_null() {
            setup_else_part = 0 as libc::c_int != 0;
        }
    } else if (*(*false_branch).d.di).opcode as libc::c_uint
        != Op_no_op as libc::c_int as libc::c_uint
    {
        list_append(false_branch, bcalloc(Op_no_op, 1 as libc::c_int, 0 as libc::c_int));
    }
    if setup_else_part {
        if do_flags as libc::c_uint & DO_PRETTY_PRINT as libc::c_int as libc::c_uint != 0
        {
            list_prepend(false_branch, elsep);
            (*(*false_branch).nexti).x.xi = (*false_branch).d.di;
            list_prepend(
                false_branch,
                bcalloc(Op_exec_count, 1 as libc::c_int, 0 as libc::c_int),
            );
        } else {
            bcfree(elsep);
        }
    }
    list_prepend(false_branch, bcalloc(Op_jmp, 1 as libc::c_int, 0 as libc::c_int));
    (*(*false_branch).nexti).d.di = (*false_branch).d.di;
    add_lint(cond, LINT_assign_in_cond);
    ip = list_append(cond, bcalloc(Op_jmp_false, 1 as libc::c_int, 0 as libc::c_int));
    (*(*ip).d.di).d.di = (*(*false_branch).nexti).nexti;
    if do_flags as libc::c_uint & DO_PRETTY_PRINT as libc::c_int as libc::c_uint != 0 {
        list_prepend(ip, ifp);
        list_append(ip, bcalloc(Op_exec_count, 1 as libc::c_int, 0 as libc::c_int));
        (*(*ip).nexti).d.di = (*ip).d.di;
        (*(*ip).nexti).x.xi = (*false_branch).nexti;
    } else {
        bcfree(ifp);
    }
    if !true_branch.is_null() {
        list_merge(ip, true_branch);
    }
    return list_merge(ip, false_branch);
}
unsafe extern "C" fn find_line(
    mut pattern: *mut INSTRUCTION,
    mut what: defline,
) -> libc::c_int {
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut lineno: libc::c_int = 0 as libc::c_int;
    ip = (*pattern).nexti;
    while !ip.is_null() {
        if what as libc::c_uint == LAST_LINE as libc::c_int as libc::c_uint {
            if (*ip).source_line as libc::c_int > lineno {
                lineno = (*ip).source_line as libc::c_int;
            }
        } else if (*ip).source_line as libc::c_int > 0 as libc::c_int
            && (lineno == 0 as libc::c_int
                || ((*ip).source_line as libc::c_int) < lineno)
        {
            lineno = (*ip).source_line as libc::c_int;
        }
        if ip == (*pattern).d.di {
            break;
        }
        ip = (*ip).nexti;
    }
    return lineno;
}
unsafe extern "C" fn append_rule(
    mut pattern: *mut INSTRUCTION,
    mut action: *mut INSTRUCTION,
) -> *mut INSTRUCTION {
    let mut rp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut tp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    if rule != Rule as libc::c_int {
        rp = pattern;
        if do_flags as libc::c_uint & DO_PRETTY_PRINT as libc::c_int as libc::c_uint != 0
        {
            let ref mut fresh116 = (*rp.offset(3 as libc::c_int as isize)).nexti;
            *fresh116 = namespace_chain;
            namespace_chain = 0 as *mut INSTRUCTION;
            list_append(action, bcalloc(Op_no_op, 1 as libc::c_int, 0 as libc::c_int));
        }
        let ref mut fresh117 = (*rp.offset(1 as libc::c_int as isize)).x.xi;
        *fresh117 = (*action).nexti;
        let ref mut fresh118 = (*rp.offset(1 as libc::c_int as isize)).d.di;
        *fresh118 = (*action).d.di;
        (*rp.offset(2 as libc::c_int as isize)).source_line = (*pattern).source_line;
        (*rp.offset(2 as libc::c_int as isize)).x.xl = lastline as libc::c_long;
        ip = list_prepend(action, rp);
        if !interblock_comment.is_null() {
            ip = list_prepend(ip, interblock_comment);
            interblock_comment = 0 as *mut INSTRUCTION;
        }
    } else {
        rp = bcalloc(Op_rule, 4 as libc::c_int, 0 as libc::c_int);
        (*rp).x.xl = Rule as libc::c_int as libc::c_long;
        (*rp).d.name = source;
        tp = bcalloc(Op_no_op, 1 as libc::c_int, 0 as libc::c_int);
        if do_flags as libc::c_uint & DO_PRETTY_PRINT as libc::c_int as libc::c_uint != 0
        {
            let ref mut fresh119 = (*rp.offset(3 as libc::c_int as isize)).nexti;
            *fresh119 = namespace_chain;
            namespace_chain = 0 as *mut INSTRUCTION;
        }
        if pattern.is_null() {
            if do_flags as libc::c_uint & DO_PRETTY_PRINT as libc::c_int as libc::c_uint
                != 0
            {
                list_prepend(
                    action,
                    bcalloc(Op_exec_count, 1 as libc::c_int, 0 as libc::c_int),
                );
            }
            let ref mut fresh120 = (*rp.offset(1 as libc::c_int as isize)).x.xi;
            *fresh120 = (*action).nexti;
            let ref mut fresh121 = (*rp.offset(1 as libc::c_int as isize)).d.di;
            *fresh121 = tp;
            (*rp.offset(2 as libc::c_int as isize))
                .source_line = firstline as libc::c_short;
            (*rp.offset(2 as libc::c_int as isize)).x.xl = lastline as libc::c_long;
            (*rp).source_line = firstline as libc::c_short;
            ip = list_prepend(list_append(action, tp), rp);
        } else {
            list_append(
                pattern,
                bcalloc(Op_jmp_false, 1 as libc::c_int, 0 as libc::c_int),
            );
            (*(*pattern).d.di).d.di = tp;
            (*rp.offset(2 as libc::c_int as isize))
                .source_line = find_line(pattern, FIRST_LINE) as libc::c_short;
            (*rp).source_line = (*rp.offset(2 as libc::c_int as isize)).source_line;
            if action.is_null() {
                (*rp.offset(2 as libc::c_int as isize))
                    .x
                    .xl = find_line(pattern, LAST_LINE) as libc::c_long;
                action = list_create(
                    bcalloc(Op_K_print_rec, 1 as libc::c_int, 0 as libc::c_int),
                );
                if do_flags as libc::c_uint
                    & DO_PRETTY_PRINT as libc::c_int as libc::c_uint != 0
                {
                    action = list_prepend(
                        action,
                        bcalloc(Op_exec_count, 1 as libc::c_int, 0 as libc::c_int),
                    );
                }
            } else {
                (*rp.offset(2 as libc::c_int as isize)).x.xl = lastline as libc::c_long;
            }
            if !interblock_comment.is_null() {
                pattern = list_prepend(pattern, interblock_comment);
                interblock_comment = 0 as *mut INSTRUCTION;
            }
            if do_flags as libc::c_uint & DO_PRETTY_PRINT as libc::c_int as libc::c_uint
                != 0
            {
                pattern = list_prepend(
                    pattern,
                    bcalloc(Op_exec_count, 1 as libc::c_int, 0 as libc::c_int),
                );
                action = list_prepend(
                    action,
                    bcalloc(Op_exec_count, 1 as libc::c_int, 0 as libc::c_int),
                );
            }
            let ref mut fresh122 = (*rp.offset(1 as libc::c_int as isize)).x.xi;
            *fresh122 = (*action).nexti;
            let ref mut fresh123 = (*rp.offset(1 as libc::c_int as isize)).d.di;
            *fresh123 = tp;
            ip = list_append(list_merge(list_prepend(pattern, rp), action), tp);
        }
    }
    list_append(rule_list, rp.offset(1 as libc::c_int as isize));
    if (rule_block[rule as usize]).is_null() {
        rule_block[rule as usize] = ip;
    } else {
        list_merge(rule_block[rule as usize], ip);
    }
    return rule_block[rule as usize];
}
unsafe extern "C" fn mk_assignment(
    mut lhs: *mut INSTRUCTION,
    mut rhs: *mut INSTRUCTION,
    mut op: *mut INSTRUCTION,
) -> *mut INSTRUCTION {
    let mut tp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    tp = (*lhs).d.di;
    match (*tp).opcode as libc::c_uint {
        24 => {
            (*tp).opcode = Op_field_spec_lhs;
        }
        16 => {
            (*tp).opcode = Op_subscript_lhs;
        }
        75 | 80 => {
            (*tp).opcode = Op_push_lhs;
        }
        97 => {
            yyerror(
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot assign a value to the result of a field post-increment expression\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        _ => {
            yyerror(
                dcgettext(
                    0 as *const libc::c_char,
                    b"invalid target of assignment (opcode %s)\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                opcode2str((*tp).opcode),
            );
        }
    }
    (*tp)
        .x
        .xl = ((*op).opcode as libc::c_uint != Op_assign as libc::c_int as libc::c_uint)
        as libc::c_int as libc::c_long;
    if !rhs.is_null() {
        ip = list_merge(rhs, lhs);
    } else {
        ip = lhs;
    }
    if (*tp).opcode as libc::c_uint != Op_field_spec_lhs as libc::c_int as libc::c_uint
        || (*op).opcode as libc::c_uint != Op_assign as libc::c_int as libc::c_uint
    {
        list_append(ip, op);
    }
    if (*tp).opcode as libc::c_uint == Op_push_lhs as libc::c_int as libc::c_uint
        && (*(*tp).d.dn).type_0 as libc::c_uint
            == Node_var as libc::c_int as libc::c_uint
        && ((*(*tp).d.dn).sub.nodep.x.aptr).is_some()
    {
        (*tp).x.xl = 0 as libc::c_int as libc::c_long;
        list_append(ip, bcalloc(Op_var_assign, 1 as libc::c_int, 0 as libc::c_int));
        (*(*ip).d.di).x.aptr = (*(*tp).d.dn).sub.nodep.x.aptr;
    } else if (*tp).opcode as libc::c_uint
        == Op_field_spec_lhs as libc::c_int as libc::c_uint
    {
        if (*op).opcode as libc::c_uint == Op_assign as libc::c_int as libc::c_uint {
            bcfree(op);
            (*tp).opcode = Op_store_field_exp;
        } else {
            list_append(
                ip,
                bcalloc(Op_field_assign, 1 as libc::c_int, 0 as libc::c_int),
            );
            (*(*ip).d.di).x.aptr = None;
            (*tp).d.di = (*ip).d.di;
        }
    } else if (*tp).opcode as libc::c_uint
        == Op_subscript_lhs as libc::c_int as libc::c_uint
    {
        list_append(
            ip,
            bcalloc(Op_subscript_assign, 1 as libc::c_int, 0 as libc::c_int),
        );
    }
    return ip;
}
unsafe extern "C" fn optimize_assignment(mut exp: *mut INSTRUCTION) -> *mut INSTRUCTION {
    let mut i1: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut i2: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut i3: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    i2 = 0 as *mut INSTRUCTION;
    i1 = (*exp).d.di;
    if (*i1).opcode as libc::c_uint == Op_store_field_exp as libc::c_int as libc::c_uint
    {
        (*i1).opcode = Op_store_field;
        return exp;
    }
    if (*i1).opcode as libc::c_uint != Op_assign as libc::c_int as libc::c_uint
        && (*i1).opcode as libc::c_uint != Op_field_assign as libc::c_int as libc::c_uint
    {
        return list_append(exp, bcalloc(Op_pop, 1 as libc::c_int, 0 as libc::c_int));
    }
    i2 = (*exp).nexti;
    while i2 != i1 {
        match (*i2).opcode as libc::c_uint {
            13 => {
                if (*(*i2).nexti).opcode as libc::c_uint
                    == Op_push_lhs as libc::c_int as libc::c_uint
                    && (*i2).d.dl & 2 as libc::c_int as libc::c_long
                        != 0 as libc::c_int as libc::c_long
                    && (*(*i2).nexti).d.dn == (*(*exp).nexti).d.dn
                    && (*(*i2).nexti).nexti == i1
                    && (*i1).opcode as libc::c_uint
                        == Op_assign as libc::c_int as libc::c_uint
                {
                    i3 = (*(*exp).nexti).nexti;
                    while i3 != i2 {
                        if (*i3).opcode as libc::c_uint
                            == Op_push_lhs as libc::c_int as libc::c_uint
                            && (*i3).d.dn == (*(*i2).nexti).d.dn
                            || (*i3).opcode as libc::c_uint
                                == Op_func_call as libc::c_int as libc::c_uint
                        {
                            return list_append(
                                exp,
                                bcalloc(Op_pop, 1 as libc::c_int, 0 as libc::c_int),
                            );
                        }
                        i3 = (*i3).nexti;
                    }
                    i3 = (*exp).nexti;
                    (*exp).nexti = (*i3).nexti;
                    bcfree(i3);
                    (*i2).x.xl -= 1;
                    if (*i2).x.xl == 1 as libc::c_int as libc::c_long {
                        (*i2).opcode = Op_no_op;
                    }
                    i3 = (*i2).nexti;
                    (*i3).opcode = Op_assign_concat;
                    (*i3).nexti = 0 as *mut exp_instruction;
                    bcfree(i1);
                    (*exp).d.di = i3;
                    return exp;
                }
            }
            80 => {
                if (*(*(*i2).nexti).nexti).opcode as libc::c_uint
                    == Op_subscript_lhs as libc::c_int as libc::c_uint
                {
                    i3 = (*(*i2).nexti).nexti;
                    if (*i3).d.dl == 1 as libc::c_int as libc::c_long
                        && (*i3).nexti == i1
                        && (*i1).opcode as libc::c_uint
                            == Op_assign as libc::c_int as libc::c_uint
                    {
                        (*i3).opcode = Op_store_sub;
                        (*i3).d.dn = (*i2).d.dn;
                        (*i3).x.xl = 1 as libc::c_int as libc::c_long;
                        (*i3).nexti = 0 as *mut exp_instruction;
                        (*i2).opcode = Op_no_op;
                        bcfree(i1);
                        (*exp).d.di = i3;
                        return exp;
                    }
                }
            }
            82 => {
                if (*i2).nexti == i1
                    && (*i1).opcode as libc::c_uint
                        == Op_assign as libc::c_int as libc::c_uint
                {
                    (*i2).opcode = Op_store_var;
                    (*i2).nexti = 0 as *mut exp_instruction;
                    bcfree(i1);
                    (*exp).d.di = i2;
                    i3 = (*exp).nexti;
                    if (*i3).opcode as libc::c_uint
                        == Op_push_i as libc::c_int as libc::c_uint
                        && (*(*i3).d.dn).flags as libc::c_uint
                            & INTLSTR as libc::c_int as libc::c_uint
                            == 0 as libc::c_int as libc::c_uint && (*i3).nexti == i2
                    {
                        (*i2).x.xn = (*i3).d.dn;
                        bcfree(i3);
                        (*exp).nexti = i2;
                    } else {
                        (*i2).x.xn = 0 as *mut NODE;
                    }
                    return exp;
                }
            }
            _ => {}
        }
        i2 = (*i2).nexti;
    }
    return list_append(exp, bcalloc(Op_pop, 1 as libc::c_int, 0 as libc::c_int));
}
unsafe extern "C" fn mk_getline(
    mut op: *mut INSTRUCTION,
    mut var: *mut INSTRUCTION,
    mut redir: *mut INSTRUCTION,
    mut redirtype: libc::c_int,
) -> *mut INSTRUCTION {
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut tp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut asgn: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    if redir.is_null() {
        let mut sline: libc::c_int = (*op).source_line as libc::c_int;
        bcfree(op);
        op = bcalloc(Op_K_getline, 2 as libc::c_int, sline);
        let ref mut fresh124 = (*op.offset(1 as libc::c_int as isize)).x.xi;
        *fresh124 = ip_endfile;
        let ref mut fresh125 = (*op.offset(1 as libc::c_int as isize)).d.di;
        *fresh125 = ip_beginfile;
    }
    if !var.is_null() {
        tp = make_assignable((*var).d.di);
        if (*tp).opcode as libc::c_uint == Op_push_lhs as libc::c_int as libc::c_uint
            && (*(*tp).d.dn).type_0 as libc::c_uint
                == Node_var as libc::c_int as libc::c_uint
            && ((*(*tp).d.dn).sub.nodep.x.aptr).is_some()
        {
            asgn = bcalloc(Op_var_assign, 1 as libc::c_int, 0 as libc::c_int);
            (*asgn).d.dl = (*op).opcode as libc::c_long;
            (*asgn).x.aptr = (*(*tp).d.dn).sub.nodep.x.aptr;
        } else if (*tp).opcode as libc::c_uint
            == Op_field_spec_lhs as libc::c_int as libc::c_uint
        {
            asgn = bcalloc(Op_field_assign, 1 as libc::c_int, 0 as libc::c_int);
            (*asgn).d.dl = (*op).opcode as libc::c_long;
            (*asgn).x.aptr = None;
            (*tp).d.di = asgn;
        } else if (*tp).opcode as libc::c_uint
            == Op_subscript_lhs as libc::c_int as libc::c_uint
        {
            asgn = bcalloc(Op_subscript_assign, 1 as libc::c_int, 0 as libc::c_int);
            (*asgn).d.dl = (*op).opcode as libc::c_long;
        }
        if !redir.is_null() {
            ip = list_merge(redir, var);
            list_append(ip, op);
        } else {
            ip = list_append(var, op);
        }
    } else if !redir.is_null() {
        ip = list_append(redir, op);
    } else {
        ip = list_create(op);
    }
    (*op)
        .x
        .xl = (var != 0 as *mut libc::c_void as *mut INSTRUCTION) as libc::c_int
        as libc::c_long;
    (*op)
        .d
        .dl = (if !redir.is_null() { redirtype } else { redirect_none as libc::c_int })
        as libc::c_long;
    return if asgn.is_null() { ip } else { list_append(ip, asgn) };
}
unsafe extern "C" fn mk_for_loop(
    mut forp: *mut INSTRUCTION,
    mut init: *mut INSTRUCTION,
    mut cond: *mut INSTRUCTION,
    mut incr: *mut INSTRUCTION,
    mut body: *mut INSTRUCTION,
) -> *mut INSTRUCTION {
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut tbreak: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut tcont: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut jmp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut pp_cond: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut ret: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    tbreak = bcalloc(Op_no_op, 1 as libc::c_int, 0 as libc::c_int);
    if !cond.is_null() {
        add_lint(cond, LINT_assign_in_cond);
        pp_cond = (*cond).nexti;
        ip = cond;
        list_append(ip, bcalloc(Op_jmp_false, 1 as libc::c_int, 0 as libc::c_int));
        (*(*ip).d.di).d.di = tbreak;
    } else {
        pp_cond = bcalloc(Op_no_op, 1 as libc::c_int, 0 as libc::c_int);
        ip = list_create(pp_cond);
    }
    if !init.is_null() {
        ip = list_merge(init, ip);
    }
    if do_flags as libc::c_uint & DO_PRETTY_PRINT as libc::c_int as libc::c_uint != 0 {
        list_append(ip, bcalloc(Op_exec_count, 1 as libc::c_int, 0 as libc::c_int));
        let ref mut fresh126 = (*forp.offset(1 as libc::c_int as isize)).d.di;
        *fresh126 = pp_cond;
        let ref mut fresh127 = (*forp.offset(1 as libc::c_int as isize)).x.xi;
        *fresh127 = (*ip).d.di;
    }
    if !body.is_null() {
        list_merge(ip, body);
    }
    jmp = bcalloc(Op_jmp, 1 as libc::c_int, 0 as libc::c_int);
    (*jmp).d.di = pp_cond;
    if incr.is_null() {
        tcont = jmp;
    } else {
        tcont = (*incr).nexti;
        list_merge(ip, incr);
    }
    list_append(ip, jmp);
    ret = list_append(ip, tbreak);
    fix_break_continue(ret, tbreak, tcont);
    if do_flags as libc::c_uint & DO_PRETTY_PRINT as libc::c_int as libc::c_uint != 0 {
        (*forp).x.xi = tbreak;
        (*forp).d.di = tcont;
        ret = list_prepend(ret, forp);
    }
    return ret;
}
unsafe extern "C" fn add_lint(mut list: *mut INSTRUCTION, mut linttype: LINTTYPE) {
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut no_effect: bool = 1 as libc::c_int != 0;
    match linttype as libc::c_uint {
        1 => {
            ip = (*list).d.di;
            if (*ip).opcode as libc::c_uint
                == Op_var_assign as libc::c_int as libc::c_uint
                || (*ip).opcode as libc::c_uint
                    == Op_field_assign as libc::c_int as libc::c_uint
            {
                ip = (*list).nexti;
                while (*ip).nexti != (*list).d.di {
                    ip = (*ip).nexti;
                }
            }
            if (*ip).opcode as libc::c_uint == Op_assign as libc::c_int as libc::c_uint
                || (*ip).opcode as libc::c_uint
                    == Op_assign_concat as libc::c_int as libc::c_uint
            {
                list_append(list, bcalloc(Op_lint, 1 as libc::c_int, 0 as libc::c_int));
                (*(*list).d.di).d.dl = linttype as libc::c_long;
            }
        }
        2 => {
            if (*(*list).d.di).opcode as libc::c_uint
                == Op_pop as libc::c_int as libc::c_uint && (*list).nexti != (*list).d.di
            {
                let mut line: libc::c_int = 0 as libc::c_int;
                ip = (*list).nexti;
                while (*ip).nexti != (*list).d.di {
                    if (*ip).source_line as libc::c_int != 0 as libc::c_int {
                        line = (*ip).source_line as libc::c_int;
                    }
                    no_effect = no_effect as libc::c_int != 0
                        && isnoeffect((*ip).opcode) != 0;
                    ip = (*ip).nexti;
                }
                no_effect = no_effect as libc::c_int != 0
                    && isnoeffect((*ip).opcode) != 0;
                if do_flags as libc::c_uint
                    & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                        as libc::c_uint != 0
                {
                    if no_effect {
                        if (*ip).source_line as libc::c_int != 0 as libc::c_int {
                            line = (*ip).source_line as libc::c_int;
                        }
                        lintwarn_ln(
                            line,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"statement has no effect\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                }
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn mk_expression_list(
    mut list: *mut INSTRUCTION,
    mut s1: *mut INSTRUCTION,
) -> *mut INSTRUCTION {
    let mut r: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    if list.is_null() {
        list = bcalloc(Op_list, 1 as libc::c_int, 0 as libc::c_int);
        (*list).nexti = s1;
        (*list).d.di = (*s1).d.di;
        return list;
    }
    r = (*list).d.di;
    (*r).nexti = s1;
    (*list).d.di = (*s1).d.di;
    return list;
}
unsafe extern "C" fn count_expressions(
    mut list: *mut *mut INSTRUCTION,
    mut isarg: bool,
) -> libc::c_int {
    let mut expr: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut r: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut count: libc::c_int = 0 as libc::c_int;
    if (*list).is_null() {
        return 0 as libc::c_int;
    }
    expr = (**list).nexti;
    while !expr.is_null() {
        let mut t1: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
        let mut t2: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
        t1 = (*expr).nexti;
        t2 = (*expr).d.di;
        if isarg as libc::c_int != 0 && t1 == t2
            && (*t1).opcode as libc::c_uint == Op_push as libc::c_int as libc::c_uint
        {
            (*t1).opcode = Op_push_param;
        }
        count += 1;
        if count == 1 as libc::c_int {
            r = expr;
        } else {
            list_merge(r, expr);
        }
        expr = (*t2).nexti;
    }
    if !isarg && count > max_args {
        max_args = count;
    }
    bcfree(*list);
    *list = r;
    return count;
}
unsafe extern "C" fn fix_break_continue(
    mut list: *mut INSTRUCTION,
    mut b_target: *mut INSTRUCTION,
    mut c_target: *mut INSTRUCTION,
) {
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    (*(*list).d.di).nexti = 0 as *mut exp_instruction;
    ip = (*list).nexti;
    while !ip.is_null() {
        match (*ip).opcode as libc::c_uint {
            54 => {
                if ((*ip).d.di).is_null() {
                    (*ip).d.di = b_target;
                }
            }
            55 => {
                if ((*ip).d.di).is_null() {
                    (*ip).d.di = c_target;
                }
            }
            _ => {}
        }
        ip = (*ip).nexti;
    }
}
#[inline]
unsafe extern "C" fn list_create(mut x: *mut INSTRUCTION) -> *mut INSTRUCTION {
    let mut l: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    l = bcalloc(Op_list, 1 as libc::c_int, 0 as libc::c_int);
    (*l).nexti = x;
    (*l).d.di = x;
    return l;
}
#[inline]
unsafe extern "C" fn list_append(
    mut l: *mut INSTRUCTION,
    mut x: *mut INSTRUCTION,
) -> *mut INSTRUCTION {
    (*(*l).d.di).nexti = x;
    (*l).d.di = x;
    return l;
}
#[inline]
unsafe extern "C" fn list_prepend(
    mut l: *mut INSTRUCTION,
    mut x: *mut INSTRUCTION,
) -> *mut INSTRUCTION {
    (*x).nexti = (*l).nexti;
    (*l).nexti = x;
    return l;
}
#[inline]
unsafe extern "C" fn list_merge(
    mut l1: *mut INSTRUCTION,
    mut l2: *mut INSTRUCTION,
) -> *mut INSTRUCTION {
    (*(*l1).d.di).nexti = (*l2).nexti;
    (*l1).d.di = (*l2).d.di;
    bcfree(l2);
    return l1;
}
pub unsafe extern "C" fn check_special(mut name: *const libc::c_char) -> libc::c_int {
    let mut low: libc::c_int = 0;
    let mut high: libc::c_int = 0;
    let mut mid: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut non_standard_flags: libc::c_int = 0 as libc::c_int;
    if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0 {
        non_standard_flags |= 0x400 as libc::c_int;
    }
    if do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint != 0 {
        non_standard_flags |= 0x200 as libc::c_int;
    }
    low = 0 as libc::c_int;
    high = (::std::mem::size_of::<[token; 71]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<token>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    while low <= high {
        mid = (low + high) / 2 as libc::c_int;
        i = *name as libc::c_int
            - *(tokentab[mid as usize].operator).offset(0 as libc::c_int as isize)
                as libc::c_int;
        if i == 0 as libc::c_int {
            i = strcmp(name, tokentab[mid as usize].operator);
        }
        if i < 0 as libc::c_int {
            high = mid - 1 as libc::c_int;
        } else if i > 0 as libc::c_int {
            low = mid + 1 as libc::c_int;
        } else {
            if tokentab[mid as usize].flags & non_standard_flags as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                return -(1 as libc::c_int);
            }
            return mid;
        }
    }
    return -(1 as libc::c_int);
}
static mut fp: *mut FILE = 0 as *const FILE as *mut FILE;
unsafe extern "C" fn read_one_line(
    mut fd: libc::c_int,
    mut buffer: *mut libc::c_void,
    mut count: size_t,
) -> ssize_t {
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    if fp.is_null() {
        fp = fdopen(fd, b"r\0" as *const u8 as *const libc::c_char);
        if fp.is_null() {
            fprintf(
                stderr,
                b"ugh. fdopen: %s\n\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            gawk_exit(1 as libc::c_int);
        }
    }
    if (fgets(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong as libc::c_int,
        fp,
    ))
        .is_null()
    {
        return 0 as libc::c_int as ssize_t;
    }
    memcpy(buffer, buf.as_mut_ptr() as *const libc::c_void, strlen(buf.as_mut_ptr()));
    return strlen(buf.as_mut_ptr()) as ssize_t;
}
unsafe extern "C" fn one_line_close(mut fd: libc::c_int) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if fp.is_null() || fd != fileno(fp) {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"awkgram.y\0" as *const u8 as *const libc::c_char,
            6506 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            b"debugging read/close screwed up!\0" as *const u8 as *const libc::c_char,
        );
    }
    ret = fclose(fp);
    fp = 0 as *mut FILE;
    return ret;
}
pub unsafe extern "C" fn lookup_builtin(
    mut name: *const libc::c_char,
) -> builtin_func_t {
    if strncmp(
        name,
        b"awk::\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        name = name.offset(5 as libc::c_int as isize);
    }
    let mut mid: libc::c_int = check_special(name);
    if mid == -(1 as libc::c_int) {
        return None;
    }
    match tokentab[mid as usize].class {
        301 | 302 => {}
        _ => return None,
    }
    if tokentab[mid as usize].value as libc::c_uint
        == Op_sub_builtin as libc::c_int as libc::c_uint
    {
        return ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int, libc::c_uint) -> *mut NODE>,
            builtin_func_t,
        >(Some(do_sub as unsafe extern "C" fn(libc::c_int, libc::c_uint) -> *mut NODE));
    }
    return tokentab[mid as usize].ptr;
}
pub unsafe extern "C" fn install_builtins() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut flags_that_must_be_clear: libc::c_int = 0x2000 as libc::c_int;
    if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0 {
        flags_that_must_be_clear |= 0x400 as libc::c_int;
    }
    if do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint != 0 {
        flags_that_must_be_clear |= 0x200 as libc::c_int;
    }
    j = (::std::mem::size_of::<[token; 71]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<token>() as libc::c_ulong) as libc::c_int;
    i = 0 as libc::c_int;
    while i < j {
        if (tokentab[i as usize].class == 301 as libc::c_int
            || tokentab[i as usize].class == 302 as libc::c_int)
            && tokentab[i as usize].flags & flags_that_must_be_clear as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
        {
            install_symbol(tokentab[i as usize].operator, Node_builtin_func);
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn is_alpha(mut c: libc::c_int) -> bool {
    match c {
        97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110
        | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 65 | 66
        | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82
        | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => return 1 as libc::c_int != 0,
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn is_alnum(mut c: libc::c_int) -> bool {
    return is_alpha(c) as libc::c_int != 0 || '0' as i32 <= c && c <= '9' as i32;
}
pub unsafe extern "C" fn is_letter(mut c: libc::c_int) -> bool {
    return is_alpha(c) as libc::c_int != 0 || c == '_' as i32;
}
pub unsafe extern "C" fn is_identchar(mut c: libc::c_int) -> bool {
    return is_alnum(c) as libc::c_int != 0 || c == '_' as i32;
}
unsafe extern "C" fn set_profile_text(
    mut n: *mut NODE,
    mut str: *const libc::c_char,
    mut len: size_t,
) -> *mut NODE {
    if do_flags as libc::c_uint & DO_PRETTY_PRINT as libc::c_int as libc::c_uint != 0 {
        (*n)
            .sub
            .val
            .sp = emalloc_real(
            len.wrapping_add(2 as libc::c_int as libc::c_ulong),
            b"set_profile_text\0" as *const u8 as *const libc::c_char,
            b"n->stptr\0" as *const u8 as *const libc::c_char,
            b"awkgram.y\0" as *const u8 as *const libc::c_char,
            6647 as libc::c_int,
        ) as *mut libc::c_char;
        memcpy((*n).sub.val.sp as *mut libc::c_void, str as *const libc::c_void, len);
        *((*n).sub.val.sp).offset(len as isize) = '\0' as i32 as libc::c_char;
        (*n).sub.val.slen = len;
        (*n)
            .flags = ::std::mem::transmute::<
            libc::c_uint,
            flagvals,
        >(
            (*n).flags as libc::c_uint
                | (NUMCONSTSTR as libc::c_int | STRCUR as libc::c_int) as libc::c_uint,
        );
        (*n).sub.val.idx = -(1 as libc::c_int);
    }
    return n;
}
unsafe extern "C" fn merge_comments(mut c1: *mut INSTRUCTION, mut c2: *mut INSTRUCTION) {
    if ((*c1).comment).is_null() && c2.is_null() {
        return;
    }
    let mut total: size_t = (*(*c1).d.dn).sub.val.slen;
    if !((*c1).comment).is_null() {
        total = (total as libc::c_ulong)
            .wrapping_add(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_add((*(*(*c1).comment).d.dn).sub.val.slen),
            ) as size_t as size_t;
    }
    if !c2.is_null() {
        total = (total as libc::c_ulong)
            .wrapping_add(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_add((*(*c2).d.dn).sub.val.slen),
            ) as size_t as size_t;
        if !((*c2).comment).is_null() {
            total = (total as libc::c_ulong)
                .wrapping_add(
                    ((*(*(*c2).comment).d.dn).sub.val.slen)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as size_t as size_t;
        }
    }
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    buffer = emalloc_real(
        total.wrapping_add(1 as libc::c_int as libc::c_ulong),
        b"merge_comments\0" as *const u8 as *const libc::c_char,
        b"buffer\0" as *const u8 as *const libc::c_char,
        b"awkgram.y\0" as *const u8 as *const libc::c_char,
        6691 as libc::c_int,
    ) as *mut libc::c_char;
    strcpy(buffer, (*(*c1).d.dn).sub.val.sp);
    if !((*c1).comment).is_null() {
        strcat(buffer, b"\n\0" as *const u8 as *const libc::c_char);
        strcat(buffer, (*(*(*c1).comment).d.dn).sub.val.sp);
    }
    if !c2.is_null() {
        strcat(buffer, (*(*c2).d.dn).sub.val.sp);
        if !((*c2).comment).is_null() {
            strcat(buffer, b"\n\0" as *const u8 as *const libc::c_char);
            strcat(buffer, (*(*(*c2).comment).d.dn).sub.val.sp);
        }
        unref((*c2).d.dn);
        if !((*c2).comment).is_null() {
            unref((*(*c2).comment).d.dn);
            bcfree((*c2).comment);
            (*c2).comment = 0 as *mut exp_instruction;
        }
        bcfree(c2);
    }
    (*(*c1).d.dn).sub.val.comtype = BLOCK_COMMENT;
    free((*(*c1).d.dn).sub.val.sp as *mut libc::c_void);
    (*(*c1).d.dn).sub.val.sp = buffer;
    (*(*c1).d.dn).sub.val.slen = strlen(buffer);
    if !((*c1).comment).is_null() {
        unref((*(*c1).comment).d.dn);
        bcfree((*c1).comment);
        (*c1).comment = 0 as *mut exp_instruction;
    }
}
unsafe extern "C" fn make_braced_statements(
    mut lbrace: *mut INSTRUCTION,
    mut stmts: *mut INSTRUCTION,
    mut rbrace: *mut INSTRUCTION,
) -> *mut INSTRUCTION {
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    if stmts.is_null() {
        ip = list_create(bcalloc(Op_no_op, 1 as libc::c_int, 0 as libc::c_int));
    } else {
        ip = stmts;
    }
    if !lbrace.is_null() {
        let mut comment2: *mut INSTRUCTION = (*lbrace).comment;
        if !comment2.is_null() {
            ip = list_prepend(ip, comment2);
            (*lbrace).comment = 0 as *mut exp_instruction;
        }
        ip = list_prepend(ip, lbrace);
    }
    return ip;
}
pub unsafe extern "C" fn validate_qualified_name(mut token: *mut libc::c_char) -> bool {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp2: *mut libc::c_char = 0 as *mut libc::c_char;
    cp = strchr(token, ':' as i32);
    if cp.is_null() {
        return 1 as libc::c_int != 0;
    }
    if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0
        || do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint != 0
    {
        error_ln(
            sourceline,
            dcgettext(
                0 as *const libc::c_char,
                b"identifier %s: qualified names not allowed in traditional / POSIX mode\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            token,
        );
        return 0 as libc::c_int != 0;
    }
    if *cp.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32 {
        error_ln(
            sourceline,
            dcgettext(
                0 as *const libc::c_char,
                b"identifier %s: namespace separator is two colons, not one\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            token,
        );
        return 0 as libc::c_int != 0;
    }
    if !is_letter(*cp.offset(2 as libc::c_int as isize) as libc::c_int) {
        error_ln(
            sourceline,
            dcgettext(
                0 as *const libc::c_char,
                b"qualified identifier `%s' is badly formed\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            token,
        );
        return 0 as libc::c_int != 0;
    }
    cp2 = strchr(cp.offset(2 as libc::c_int as isize), ':' as i32);
    if !cp2.is_null() {
        error_ln(
            sourceline,
            dcgettext(
                0 as *const libc::c_char,
                b"identifier `%s': namespace separator can only appear once in a qualified name\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            token,
        );
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn check_qualified_special(
    mut token: *mut libc::c_char,
) -> libc::c_int {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    cp = strchr(token, ':' as i32);
    if cp.is_null() && current_namespace == awk_namespace.as_ptr() {
        return check_special(token);
    }
    let mut tok_0: *const token = 0 as *const token;
    let mut i: libc::c_int = 0;
    if cp.is_null() {
        i = check_special(token);
        if i < 0 as libc::c_int {
            return i;
        }
        tok_0 = &*tokentab.as_ptr().offset(i as isize) as *const token;
        if (*tok_0).flags & 0x400 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint && (*tok_0).class == 301 as libc::c_int
        {
            return -(1 as libc::c_int)
        } else {
            return i
        }
    }
    let mut ns: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut subname: *mut libc::c_char = 0 as *mut libc::c_char;
    ns = token;
    end = cp;
    *end = '\0' as i32 as libc::c_char;
    subname = end.offset(2 as libc::c_int as isize);
    i = check_special(ns);
    if i >= 0 as libc::c_int
        && tokentab[i as usize].flags & 0x400 as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        error_ln(
            sourceline,
            dcgettext(
                0 as *const libc::c_char,
                b"using reserved identifier `%s' as a namespace is not allowed\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            ns,
        );
    } else {
        i = check_special(subname);
        if i >= 0 as libc::c_int
            && tokentab[i as usize].flags & 0x400 as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            && strcmp(ns, awk_namespace.as_ptr()) != 0 as libc::c_int
        {
            error_ln(
                sourceline,
                dcgettext(
                    0 as *const libc::c_char,
                    b"using reserved identifier `%s' as second component of a qualified name is not allowed\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                subname,
            );
        } else if strcmp(ns, awk_namespace.as_ptr()) == 0 as libc::c_int {
            i = check_special(subname);
            if i >= 0 as libc::c_int {
                if !(tokentab[i as usize].flags & 0x400 as libc::c_int as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint
                    && tokentab[i as usize].class == 301 as libc::c_int)
                {
                    error_ln(
                        sourceline,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"using reserved identifier `%s' as second component of a qualified name is not allowed\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        subname,
                    );
                }
            }
        } else {
            i = -(1 as libc::c_int);
        }
    }
    *end = ':' as i32 as libc::c_char;
    return i;
}
unsafe extern "C" fn set_namespace(
    mut ns: *mut INSTRUCTION,
    mut comment: *mut INSTRUCTION,
) {
    if ns.is_null() {
        return;
    }
    if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0
        || do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint != 0
    {
        error_ln(
            (*ns).source_line as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"@namespace is a gawk extension\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        free((*ns).d.name as *mut libc::c_void);
        bcfree(ns);
        return;
    }
    if !is_valid_identifier((*ns).d.name) {
        error_ln(
            (*ns).source_line as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"namespace name `%s' must meet identifier naming rules\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*ns).d.name,
        );
        free((*ns).d.name as *mut libc::c_void);
        bcfree(ns);
        return;
    }
    let mut mid: libc::c_int = check_special((*ns).d.name);
    if mid >= 0 as libc::c_int {
        error_ln(
            (*ns).source_line as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"using reserved identifier `%s' as a namespace is not allowed\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*ns).d.name,
        );
        free((*ns).d.name as *mut libc::c_void);
        bcfree(ns);
        return;
    }
    free((*ns).d.name as *mut libc::c_void);
    let mut new_ns: *mut INSTRUCTION = bcalloc(
        Op_K_namespace,
        1 as libc::c_int,
        0 as libc::c_int,
    );
    (*new_ns).comment = comment;
    (*new_ns).d.name = estrdup(current_namespace, strlen(current_namespace));
    (*new_ns).nexti = namespace_chain;
    namespace_chain = new_ns;
    (*ns).d.name = 0 as *mut libc::c_char;
    bcfree(ns);
}
unsafe extern "C" fn change_namespace(mut new_namespace: *const libc::c_char) {
    if !is_valid_identifier(new_namespace) {
        return;
    }
    let mut mid: libc::c_int = check_special(new_namespace);
    if mid >= 0 as libc::c_int {
        return;
    }
    if !(strcmp(new_namespace, current_namespace) == 0 as libc::c_int) {
        if strcmp(new_namespace, awk_namespace.as_ptr()) == 0 as libc::c_int {
            set_current_namespace(awk_namespace.as_ptr());
        } else {
            set_current_namespace(estrdup(new_namespace, strlen(new_namespace)));
        }
    }
    namespace_changed = 1 as libc::c_int != 0;
}
unsafe extern "C" fn qualify_name(
    mut name: *const libc::c_char,
    mut len: size_t,
) -> *mut libc::c_char {
    if !(strchr(name, ':' as i32)).is_null() {
        return estrdup(name, len);
    }
    let mut p: *mut NODE = lookup(name);
    if !p.is_null()
        && (*p).type_0 as libc::c_uint == Node_param_list as libc::c_int as libc::c_uint
    {
        return estrdup(name, len);
    }
    if current_namespace != awk_namespace.as_ptr() && !is_all_upper(name) {
        let mut length: size_t = (strlen(current_namespace))
            .wrapping_add(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
        buf = emalloc_real(
            length,
            b"qualify_name\0" as *const u8 as *const libc::c_char,
            b"buf\0" as *const u8 as *const libc::c_char,
            b"awkgram.y\0" as *const u8 as *const libc::c_char,
            6953 as libc::c_int,
        ) as *mut libc::c_char;
        sprintf(
            buf,
            b"%s::%s\0" as *const u8 as *const libc::c_char,
            current_namespace,
            name,
        );
        return buf;
    }
    return estrdup(name, len);
}
unsafe extern "C" fn run_static_initializers() {
    syn_err_len = (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
