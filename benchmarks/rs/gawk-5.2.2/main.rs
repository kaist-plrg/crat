use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type re_dfa_t;
    pub type dfa;
    pub type break_point;
    pub type instruction_block;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
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
    fn localeconv() -> *mut lconv;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strsignal(__sig: libc::c_int) -> *mut libc::c_char;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    static mut environ: *mut *mut libc::c_char;
    fn getpid() -> __pid_t;
    fn getppid() -> __pid_t;
    fn getpgrp() -> __pid_t;
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn getegid() -> __gid_t;
    fn getgroups(__size: libc::c_int, __list: *mut __gid_t) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    static mut opterr: libc::c_int;
    static mut optopt: libc::c_int;
    static mut sourceline: libc::c_int;
    static mut source: *mut libc::c_char;
    static mut errcount: libc::c_int;
    static mut interpret: Option::<
        unsafe extern "C" fn(*mut INSTRUCTION) -> libc::c_int,
    >;
    static mut make_number: Option::<unsafe extern "C" fn(libc::c_double) -> *mut NODE>;
    static mut str2number: Option::<unsafe extern "C" fn(*mut NODE) -> *mut NODE>;
    static str_array_func: array_funcs_t;
    static mut nextfree: [block_header; 2];
    static quote: libc::c_char;
    static mut defpath: *const libc::c_char;
    static mut deflibpath: *const libc::c_char;
    static awk_namespace: [libc::c_char; 0];
    static mut current_namespace: *const libc::c_char;
    fn r_unref(tmp: *mut NODE);
    fn make_array() -> *mut NODE;
    fn null_array(symbol: *mut NODE);
    fn array_init();
    fn set_SUBSEP();
    fn init_env_array(env_node: *mut NODE);
    fn init_argv_array(argv_node: *mut NODE, shadow_node: *mut NODE);
    fn variable(
        location: libc::c_int,
        name: *mut libc::c_char,
        type_0: NODETYPE,
    ) -> *mut NODE;
    fn parse_program(pcode: *mut *mut INSTRUCTION, from_eval: bool) -> libc::c_int;
    fn dump_funcs();
    fn dump_vars(fname: *const libc::c_char);
    fn shadow_funcs();
    fn check_special(name: *const libc::c_char) -> libc::c_int;
    fn add_srcfile(
        stype: srctype,
        src: *mut libc::c_char,
        curr: *mut SRCFILE,
        already_included: *mut bool,
        errcode: *mut libc::c_int,
    ) -> *mut SRCFILE;
    fn install_builtins();
    fn is_letter(c: libc::c_int) -> bool;
    fn is_identchar(c: libc::c_int) -> bool;
    fn validate_qualified_name(token: *mut libc::c_char) -> bool;
    fn init_debug();
    fn debug_prog(pc: *mut INSTRUCTION) -> libc::c_int;
    fn init_interpret();
    fn set_IGNORECASE();
    fn set_OFS();
    fn set_ORS();
    fn set_OFMT();
    fn set_CONVFMT();
    fn set_BINMODE();
    fn set_LINT();
    fn set_TEXTDOMAIN();
    fn update_NR();
    fn update_NF();
    fn update_FNR();
    fn load_casetable();
    fn r_get_lhs(n: *mut NODE, reference: bool) -> *mut *mut NODE;
    fn load_ext(lib_name: *const libc::c_char);
    fn init_fields();
    fn set_NF();
    fn set_FS();
    fn set_RS();
    fn set_FIELDWIDTHS();
    fn set_FPAT();
    fn update_PROCINFO_str(subscript: *const libc::c_char, str: *const libc::c_char);
    fn update_PROCINFO_num(subscript: *const libc::c_char, val: libc::c_double);
    fn current_field_sep_str() -> *const libc::c_char;
    fn init_ext_api();
    fn print_ext_versions();
    fn gawk_name(filespec: *const libc::c_char) -> *const libc::c_char;
    fn os_arg_fixup(argcp: *mut libc::c_int, argvp: *mut *mut *mut libc::c_char);
    fn os_isatty(fd: libc::c_int) -> libc::c_int;
    fn os_is_setuid() -> libc::c_int;
    fn os_setbinmode(fd: libc::c_int, mode: libc::c_int) -> libc::c_int;
    fn os_maybe_set_errno();
    fn init_io();
    fn set_FNR();
    fn set_NR();
    fn devopen(name: *const libc::c_char, mode: *const libc::c_char) -> libc::c_int;
    fn final_exit(status: libc::c_int) -> !;
    fn r_fatal(mesg: *const libc::c_char, _: ...);
    fn set_loc(file: *const libc::c_char, line: libc::c_int);
    fn make_str_node(
        s: *const libc::c_char,
        len: size_t,
        flags: libc::c_int,
    ) -> *mut NODE;
    fn make_typed_regex(re: *const libc::c_char, len: size_t) -> *mut NODE;
    fn lookup(name: *const libc::c_char) -> *mut NODE;
    fn r_warning(mesg: *const libc::c_char, _: ...);
    fn push_context(ctxt: *mut AWK_CONTEXT);
    fn new_context() -> *mut AWK_CONTEXT;
    fn set_ROUNDMODE();
    fn release_all_vars();
    fn set_PREC();
    fn msg(mesg: *const libc::c_char, _: ...);
    fn init_profiling_signals();
    fn set_prof_file(filename: *const libc::c_char);
    fn dump_prog(code: *mut INSTRUCTION);
    fn more_blocks(id: libc::c_int) -> *mut libc::c_void;
    fn init_btowc_cache();
    fn resetup();
    fn load_symbols();
    fn init_symbol_table();
    fn install_symbol(name: *const libc::c_char, type_0: NODETYPE) -> *mut NODE;
    fn pma_save_free_lists();
    fn pma_mpfr_check();
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn mtrace();
    static mut version_string: *const libc::c_char;
}
pub type size_t = libc::c_ulong;
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
pub type wchar_t = libc::c_int;
pub type gid_t = __gid_t;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const GAWK_API_MINOR_VERSION: C2RustUnnamed_0 = 2;
pub const GAWK_API_MAJOR_VERSION: C2RustUnnamed_0 = 3;
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
pub type INSTRUCTION = exp_instruction;
pub type Func_ptr = Option::<unsafe extern "C" fn() -> ()>;
pub type binmode_values = libc::c_uint;
pub const BINMODE_BOTH: binmode_values = 3;
pub const BINMODE_OUTPUT: binmode_values = 2;
pub const BINMODE_INPUT: binmode_values = 1;
pub const TEXT_TRANSLATE: binmode_values = 0;
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
pub struct instruction_pool {
    pub pool: [instruction_mem_pool; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct instruction_mem_pool {
    pub block_list: *mut instruction_block,
    pub free_space: *mut INSTRUCTION,
    pub free_list: *mut INSTRUCTION,
}
pub type INSTRUCTION_POOL = instruction_pool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct context {
    pub pools: INSTRUCTION_POOL,
    pub symbols: NODE,
    pub rule_list: INSTRUCTION,
    pub srcfiles: SRCFILE,
    pub sourceline: libc::c_int,
    pub source: *mut libc::c_char,
    pub install_func: Option::<unsafe extern "C" fn(*mut NODE) -> ()>,
    pub prev: *mut context,
}
pub type AWK_CONTEXT = context;
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
pub struct varinit {
    pub spec: *mut *mut NODE,
    pub name: *const libc::c_char,
    pub strval: *const libc::c_char,
    pub numval: libc::c_double,
    pub update: Func_ptr,
    pub assign: Func_ptr,
    pub do_assign: bool,
    pub flags: libc::c_int,
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
pub struct pre_assign {
    pub type_0: assign_type,
    pub val: *mut libc::c_char,
}
pub type assign_type = libc::c_uint;
pub const PRE_ASSIGN_FS: assign_type = 2;
pub const PRE_ASSIGN: assign_type = 1;
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
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
static mut varfile: *const libc::c_char = b"awkvars.out\0" as *const u8
    as *const libc::c_char;
pub static mut command_file: *const libc::c_char = 0 as *const libc::c_char;
pub static mut ARGC_node: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut ARGIND_node: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut ARGV_node: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut BINMODE_node: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut CONVFMT_node: *mut NODE = 0 as *const NODE as *mut NODE;
static mut ENVIRON_node: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut ERRNO_node: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut FIELDWIDTHS_node: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut FILENAME_node: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut FNR_node: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut FPAT_node: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut FS_node: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut IGNORECASE_node: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut LINT_node: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut NF_node: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut NR_node: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut OFMT_node: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut OFS_node: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut ORS_node: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut PROCINFO_node: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut RLENGTH_node: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut RSTART_node: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut RS_node: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut RT_node: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut SUBSEP_node: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut PREC_node: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut ROUNDMODE_node: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut TEXTDOMAIN_node: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut NF: libc::c_long = 0;
pub static mut NR: libc::c_long = 0;
pub static mut FNR: libc::c_long = 0;
pub static mut BINMODE: libc::c_int = 0;
pub static mut IGNORECASE: bool = false;
pub static mut OFS: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut ORS: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut OFMT: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut TEXTDOMAIN: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut CONVFMT: *const libc::c_char = b"%.6g\0" as *const u8
    as *const libc::c_char;
pub static mut Nnull_string: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut loc: lconv = lconv {
    decimal_point: 0 as *const libc::c_char as *mut libc::c_char,
    thousands_sep: 0 as *const libc::c_char as *mut libc::c_char,
    grouping: 0 as *const libc::c_char as *mut libc::c_char,
    int_curr_symbol: 0 as *const libc::c_char as *mut libc::c_char,
    currency_symbol: 0 as *const libc::c_char as *mut libc::c_char,
    mon_decimal_point: 0 as *const libc::c_char as *mut libc::c_char,
    mon_thousands_sep: 0 as *const libc::c_char as *mut libc::c_char,
    mon_grouping: 0 as *const libc::c_char as *mut libc::c_char,
    positive_sign: 0 as *const libc::c_char as *mut libc::c_char,
    negative_sign: 0 as *const libc::c_char as *mut libc::c_char,
    int_frac_digits: 0,
    frac_digits: 0,
    p_cs_precedes: 0,
    p_sep_by_space: 0,
    n_cs_precedes: 0,
    n_sep_by_space: 0,
    p_sign_posn: 0,
    n_sign_posn: 0,
    int_p_cs_precedes: 0,
    int_p_sep_by_space: 0,
    int_n_cs_precedes: 0,
    int_n_sep_by_space: 0,
    int_p_sign_posn: 0,
    int_n_sign_posn: 0,
};
pub static mut myname: *const libc::c_char = 0 as *const libc::c_char;
pub static mut code_block: *mut INSTRUCTION = 0 as *const INSTRUCTION
    as *mut INSTRUCTION;
pub static mut d_argv: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
pub static mut rule_list: *mut INSTRUCTION = 0 as *const INSTRUCTION as *mut INSTRUCTION;
pub static mut exit_val: libc::c_int = 0 as libc::c_int;
pub static mut srcfiles: *mut SRCFILE = 0 as *const SRCFILE as *mut SRCFILE;
static mut preassigns: *mut pre_assign = 0 as *const pre_assign as *mut pre_assign;
static mut numassigns: libc::c_long = -(1 as libc::c_int) as libc::c_long;
static mut disallow_var_assigns: bool = 0 as libc::c_int != 0;
static mut stopped_early: bool = 0 as libc::c_int != 0;
pub static mut using_persistent_malloc: bool = 0 as libc::c_int != 0;
pub static mut do_flags: do_flag_values = DO_FLAG_NONE;
pub static mut do_itrace: bool = 0 as libc::c_int != 0;
pub static mut do_optimize: bool = 1 as libc::c_int != 0;
static mut do_nostalgia: libc::c_int = 0 as libc::c_int;
static mut do_binary: libc::c_int = 0 as libc::c_int;
static mut do_version: libc::c_int = 0 as libc::c_int;
static mut locale: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
static mut locale_dir: *const libc::c_char = b"/usr/local/share/locale\0" as *const u8
    as *const libc::c_char;
pub static mut use_lc_numeric: libc::c_int = 0 as libc::c_int;
pub static mut gawk_mb_cur_max: libc::c_int = 0;
pub static mut output_fp: *mut FILE = 0 as *const FILE as *mut FILE;
pub static mut output_is_tty: bool = 0 as libc::c_int != 0;
pub static mut def_strftime_format: [libc::c_char; 24] = unsafe {
    *::std::mem::transmute::<
        &[u8; 24],
        &[libc::c_char; 24],
    >(b"%a %b %e %H:%M:%S %Z %Y\0")
};
pub static mut groupset: *mut gid_t = 0 as *const gid_t as *mut gid_t;
pub static mut ngroups: libc::c_int = 0;
pub static mut lintfunc: Option::<
    unsafe extern "C" fn(*const libc::c_char, ...) -> (),
> = unsafe { Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()) };
static mut optab: [option; 31] = unsafe {
    [
        {
            let mut init = option {
                name: b"assign\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'v' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"bignum\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'M' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"characters-as-bytes\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &do_binary as *const libc::c_int as *mut libc::c_int,
                val: 'b' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"copyright\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'C' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"debug\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'D' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"dump-variables\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'd' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"exec\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'E' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"field-separator\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'F' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"file\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'f' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"gen-pot\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'g' as i32,
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
                name: b"include\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'i' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"lint\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'L' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"lint-old\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 't' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"load\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'l' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"non-decimal-data\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'n' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-optimize\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 's' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"nostalgia\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &do_nostalgia as *const libc::c_int as *mut libc::c_int,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"optimize\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'O' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"persist\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'T' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"posix\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'P' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"pretty-print\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'o' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"profile\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'p' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"re-interval\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'r' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"sandbox\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'S' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"source\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'e' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"trace\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'I' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"traditional\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'c' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"use-lc-numeric\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &use_lc_numeric as *const libc::c_int as *mut libc::c_int,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"version\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &do_version as *const libc::c_int as *mut libc::c_int,
                val: 'V' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: 0 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: '\0' as i32,
            };
            init
        },
    ]
};
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut extra_stack: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut have_srcfile: bool = 0 as libc::c_int != 0;
    let mut s: *mut SRCFILE = 0 as *mut SRCFILE;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut persist_file: *const libc::c_char = getenv(
        b"GAWK_PERSIST_FILE\0" as *const u8 as *const libc::c_char,
    );
    myname = gawk_name(*argv.offset(0 as libc::c_int as isize));
    check_pma_security(persist_file);
    let mut pma_result: libc::c_int = 0 as libc::c_int;
    if pma_result != 0 as libc::c_int {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: fatal: persistent memory allocator failed to initialize: return value %d, pma.c line: %d.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            myname,
            pma_result,
            0 as libc::c_int,
        );
        exit(2 as libc::c_int);
    }
    using_persistent_malloc = !persist_file.is_null();
    if using_persistent_malloc {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"main.c\0" as *const u8 as *const libc::c_char,
            253 as libc::c_int,
        );
        (Some(
            (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"persistent memory is not supported\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if !(getenv(b"TIDYMEM\0" as *const u8 as *const libc::c_char)).is_null() {
        do_flags = ::std::mem::transmute::<
            libc::c_uint,
            do_flag_values,
        >(do_flags as libc::c_uint | DO_TIDY_MEM as libc::c_int as libc::c_uint);
    }
    if !using_persistent_malloc
        && do_flags as libc::c_uint & DO_TIDY_MEM as libc::c_int as libc::c_uint != 0
    {
        mtrace();
    }
    os_arg_fixup(&mut argc, &mut argv);
    if argc < 2 as libc::c_int {
        usage(1 as libc::c_int, stderr);
    }
    cp = getenv(b"GAWK_LOCALE_DIR\0" as *const u8 as *const libc::c_char);
    if !cp.is_null() {
        locale_dir = cp;
    }
    let mut flags: libc::c_int = fcntl(
        fileno(stderr),
        3 as libc::c_int,
        0 as *mut libc::c_void,
    );
    if flags >= 0 as libc::c_int && flags & 0o2000 as libc::c_int == 0 as libc::c_int {
        flags |= 0o2000 as libc::c_int;
        fcntl(fileno(stderr), 4 as libc::c_int, flags);
    }
    set_locale_stuff();
    signal(8 as libc::c_int, Some(catchsig as unsafe extern "C" fn(libc::c_int) -> ()));
    signal(7 as libc::c_int, Some(catchsig as unsafe extern "C" fn(libc::c_int) -> ()));
    signal(
        13 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    signal(11 as libc::c_int, Some(catchsig as unsafe extern "C" fn(libc::c_int) -> ()));
    extra_stack = emalloc_real(
        (16 as libc::c_int * 1024 as libc::c_int) as size_t,
        b"main\0" as *const u8 as *const libc::c_char,
        b"extra_stack\0" as *const u8 as *const libc::c_char,
        b"main.c\0" as *const u8 as *const libc::c_char,
        316 as libc::c_int,
    ) as *mut libc::c_char;
    Nnull_string = make_str_node(
        b"\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as size_t,
        0 as libc::c_int,
    );
    init_fds();
    array_init();
    init_symbol_table();
    output_fp = stdout;
    push_context(new_context());
    parse_args(argc, argv);
    gawk_mb_cur_max = __ctype_get_mb_cur_max() as libc::c_int;
    init_btowc_cache();
    if gawk_mb_cur_max == 1 as libc::c_int {
        load_casetable();
    }
    if do_nostalgia != 0 {
        nostalgia();
    }
    if do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint == 0
        && !(getenv(b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char)).is_null()
    {
        do_flags = ::std::mem::transmute::<
            libc::c_uint,
            do_flag_values,
        >(do_flags as libc::c_uint | DO_POSIX as libc::c_int as libc::c_uint);
        if do_flags as libc::c_uint
            & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                as libc::c_uint != 0
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"main.c\0" as *const u8 as *const libc::c_char,
                365 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"environment variable `POSIXLY_CORRECT' set: turning on `--posix'\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    }
    if do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint != 0 {
        use_lc_numeric = 1 as libc::c_int;
        if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"main.c\0" as *const u8 as *const libc::c_char,
                373 as libc::c_int,
            );
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"`--posix' overrides `--traditional'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        } else {
            do_flags = ::std::mem::transmute::<
                libc::c_uint,
                do_flag_values,
            >(do_flags as libc::c_uint | DO_TRADITIONAL as libc::c_int as libc::c_uint);
        }
    }
    if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0
        && do_flags as libc::c_uint & DO_NON_DEC_DATA as libc::c_int as libc::c_uint != 0
    {
        do_flags = ::std::mem::transmute::<
            libc::c_uint,
            do_flag_values,
        >(do_flags as libc::c_uint & !(DO_NON_DEC_DATA as libc::c_int) as libc::c_uint);
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"main.c\0" as *const u8 as *const libc::c_char,
            384 as libc::c_int,
        );
        (Some(
            (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"`--posix'/`--traditional' overrides `--non-decimal-data'\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if do_binary != 0 {
        if do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint != 0 {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"main.c\0" as *const u8 as *const libc::c_char,
                389 as libc::c_int,
            );
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"`--posix' overrides `--characters-as-bytes'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        } else {
            gawk_mb_cur_max = 1 as libc::c_int;
            setlocale(6 as libc::c_int, b"C\0" as *const u8 as *const libc::c_char);
        }
    }
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
    {
        if os_is_setuid() != 0 {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"main.c\0" as *const u8 as *const libc::c_char,
                400 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"running %s setuid root may be a security problem\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                myname,
            );
        }
        if do_flags as libc::c_uint & DO_INTERVALS as libc::c_int as libc::c_uint != 0 {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"main.c\0" as *const u8 as *const libc::c_char,
                402 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"The -r/--re-interval options no longer have any effect\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    }
    if do_flags as libc::c_uint & DO_DEBUG as libc::c_int as libc::c_uint != 0 {
        init_debug();
    }
    init_groupset();
    (*Nnull_string).sub.val.fltnum = 0.0f64;
    (*Nnull_string)
        .flags = (MALLOC as libc::c_int | STRCUR as libc::c_int | STRING as libc::c_int
        | NUMCUR as libc::c_int | NUMBER as libc::c_int) as flagvals;
    resetup();
    init_vars();
    init_fields();
    let mut dash_v_errs: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i as libc::c_long <= numassigns {
        if (*preassigns.offset(i as isize)).type_0 as libc::c_uint
            == PRE_ASSIGN as libc::c_int as libc::c_uint
        {
            dash_v_errs
                += (arg_assign(
                    (*preassigns.offset(i as isize)).val,
                    1 as libc::c_int != 0,
                ) == 0 as libc::c_int) as libc::c_int;
        } else {
            cmdline_fs((*preassigns.offset(i as isize)).val);
        }
        free((*preassigns.offset(i as isize)).val as *mut libc::c_void);
        i += 1;
        i;
    }
    if !preassigns.is_null() {
        free(preassigns as *mut libc::c_void);
    }
    if BINMODE & BINMODE_INPUT as libc::c_int != 0 as libc::c_int {
        if os_setbinmode(fileno(stdin), 0 as libc::c_int) == -(1 as libc::c_int) {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"main.c\0" as *const u8 as *const libc::c_char,
                456 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot set binary mode on stdin: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                strerror(*__errno_location()),
            );
        }
    }
    if BINMODE & BINMODE_OUTPUT as libc::c_int != 0 as libc::c_int {
        if os_setbinmode(fileno(stdout), 0 as libc::c_int) == -(1 as libc::c_int) {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"main.c\0" as *const u8 as *const libc::c_char,
                459 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot set binary mode on stdout: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                strerror(*__errno_location()),
            );
        }
        if os_setbinmode(fileno(stderr), 0 as libc::c_int) == -(1 as libc::c_int) {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"main.c\0" as *const u8 as *const libc::c_char,
                461 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot set binary mode on stderr: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                strerror(*__errno_location()),
            );
        }
    }
    if os_isatty(fileno(stdout)) != 0 {
        output_is_tty = 1 as libc::c_int != 0;
    }
    atexit(Some(pma_save_free_lists as unsafe extern "C" fn() -> ()));
    init_ext_api();
    s = (*srcfiles).next;
    while s != srcfiles {
        if (*s).stype as libc::c_uint == SRC_EXTLIB as libc::c_int as libc::c_uint {
            load_ext((*s).fullpath);
        } else if (*s).stype as libc::c_uint != SRC_INC as libc::c_int as libc::c_uint {
            have_srcfile = 1 as libc::c_int != 0;
        }
        s = (*s).next;
    }
    if do_version != 0 {
        version();
    }
    if !have_srcfile {
        if optind > argc - 1 as libc::c_int || stopped_early as libc::c_int != 0 {
            usage(1 as libc::c_int, stderr);
        }
        add_srcfile(
            SRC_CMDLINE,
            *argv.offset(optind as isize),
            srcfiles,
            0 as *mut bool,
            0 as *mut libc::c_int,
        );
        optind += 1;
        optind;
    }
    init_interpret();
    init_args(
        optind,
        argc,
        if do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint != 0 {
            *argv.offset(0 as libc::c_int as isize) as *const libc::c_char
        } else {
            myname
        },
        argv,
    );
    setlocale(1 as libc::c_int, b"C\0" as *const u8 as *const libc::c_char);
    if parse_program(&mut code_block, 0 as libc::c_int != 0) != 0 as libc::c_int
        || dash_v_errs > 0 as libc::c_int
    {
        exit(1 as libc::c_int);
    }
    if do_flags as libc::c_uint & DO_INTL as libc::c_int as libc::c_uint != 0 {
        exit(0 as libc::c_int);
    }
    set_current_namespace(awk_namespace.as_ptr());
    install_builtins();
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
    {
        shadow_funcs();
    }
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
        && (*(*code_block).nexti).opcode as libc::c_uint
            == Op_atexit as libc::c_int as libc::c_uint
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"main.c\0" as *const u8 as *const libc::c_char,
            526 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"no program text at all!\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    load_symbols();
    if do_flags as libc::c_uint & DO_PROFILE as libc::c_int as libc::c_uint != 0 {
        init_profiling_signals();
    }
    if use_lc_numeric != 0 {
        setlocale(1 as libc::c_int, locale);
    }
    init_io();
    output_fp = stdout;
    if do_flags as libc::c_uint & DO_DEBUG as libc::c_int as libc::c_uint != 0 {
        debug_prog(code_block);
    } else if !(do_flags as libc::c_uint & DO_PRETTY_PRINT as libc::c_int as libc::c_uint
        != 0
        && do_flags as libc::c_uint & DO_PROFILE as libc::c_int as libc::c_uint == 0)
    {
        interpret.unwrap()(code_block);
    }
    if do_flags as libc::c_uint & DO_PRETTY_PRINT as libc::c_int as libc::c_uint != 0 {
        set_current_namespace(awk_namespace.as_ptr());
        dump_prog(code_block);
        dump_funcs();
    }
    if do_flags as libc::c_uint & DO_DUMP_VARS as libc::c_int as libc::c_uint != 0 {
        dump_vars(varfile);
    }
    if do_flags as libc::c_uint & DO_TIDY_MEM as libc::c_int as libc::c_uint != 0 {
        release_all_vars();
    }
    if !extra_stack.is_null() {
        free(extra_stack as *mut libc::c_void);
    }
    final_exit(exit_val);
}
unsafe extern "C" fn add_preassign(mut type_0: assign_type, mut val: *mut libc::c_char) {
    static mut alloc_assigns: libc::c_long = 0;
    numassigns += 1;
    numassigns;
    if preassigns.is_null() {
        preassigns = emalloc_real(
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<pre_assign>() as libc::c_ulong),
            b"add_preassign\0" as *const u8 as *const libc::c_char,
            b"preassigns\0" as *const u8 as *const libc::c_char,
            b"main.c\0" as *const u8 as *const libc::c_char,
            601 as libc::c_int,
        ) as *mut pre_assign;
        alloc_assigns = 4 as libc::c_int as libc::c_long;
    } else if numassigns >= alloc_assigns {
        alloc_assigns *= 2 as libc::c_int as libc::c_long;
        preassigns = erealloc_real(
            preassigns as *mut libc::c_void,
            (alloc_assigns as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<pre_assign>() as libc::c_ulong),
            b"add_preassigns\0" as *const u8 as *const libc::c_char,
            b"preassigns\0" as *const u8 as *const libc::c_char,
            b"main.c\0" as *const u8 as *const libc::c_char,
            606 as libc::c_int,
        ) as *mut pre_assign;
    }
    (*preassigns.offset(numassigns as isize)).type_0 = type_0;
    let ref mut fresh0 = (*preassigns.offset(numassigns as isize)).val;
    *fresh0 = estrdup(val, strlen(val));
}
unsafe extern "C" fn usage(mut exitval: libc::c_int, mut fp: *mut FILE) -> ! {
    static mut gnu_url: [libc::c_char; 29] = unsafe {
        *::std::mem::transmute::<
            &[u8; 29],
            &[libc::c_char; 29],
        >(b"https://ftp.gnu.org/gnu/gawk\0")
    };
    static mut beta_url: [libc::c_char; 28] = unsafe {
        *::std::mem::transmute::<
            &[u8; 28],
            &[libc::c_char; 28],
        >(b"https://www.skeeve.com/gawk\0")
    };
    let mut url: *const libc::c_char = 0 as *const libc::c_char;
    let mut major_version: libc::c_int = 0;
    let mut minor_version: libc::c_int = 0;
    let mut patchlevel: libc::c_int = 0;
    patchlevel = 0 as libc::c_int;
    minor_version = patchlevel;
    major_version = minor_version;
    sscanf(
        b"5.2.2\0" as *const u8 as *const libc::c_char,
        b"%d.%d.%d\0" as *const u8 as *const libc::c_char,
        &mut major_version as *mut libc::c_int,
        &mut minor_version as *mut libc::c_int,
        &mut patchlevel as *mut libc::c_int,
    );
    fprintf(
        fp,
        dcgettext(
            0 as *const libc::c_char,
            b"Usage: %s [POSIX or GNU style options] -f progfile [--] file ...\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        myname,
    );
    fprintf(
        fp,
        dcgettext(
            0 as *const libc::c_char,
            b"Usage: %s [POSIX or GNU style options] [--] %cprogram%c file ...\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        myname,
        quote as libc::c_int,
        quote as libc::c_int,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"POSIX options:\t\tGNU long options: (standard)\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\t-f progfile\t\t--file=progfile\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\t-F fs\t\t\t--field-separator=fs\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\t-v var=val\t\t--assign=var=val\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"Short options:\t\tGNU long options: (extensions)\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\t-b\t\t\t--characters-as-bytes\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\t-c\t\t\t--traditional\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\t-C\t\t\t--copyright\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\t-d[file]\t\t--dump-variables[=file]\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\t-D[file]\t\t--debug[=file]\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\t-e 'program-text'\t--source='program-text'\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\t-E file\t\t\t--exec=file\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\t-g\t\t\t--gen-pot\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\t-h\t\t\t--help\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\t-i includefile\t\t--include=includefile\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\t-I\t\t\t--trace\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\t-l library\t\t--load=library\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\t-L[fatal|invalid|no-ext]\t--lint[=fatal|invalid|no-ext]\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\t-M\t\t\t--bignum\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\t-N\t\t\t--use-lc-numeric\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\t-n\t\t\t--non-decimal-data\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\t-o[file]\t\t--pretty-print[=file]\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\t-O\t\t\t--optimize\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\t-p[file]\t\t--profile[=file]\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\t-P\t\t\t--posix\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\t-r\t\t\t--re-interval\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\t-s\t\t\t--no-optimize\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\t-S\t\t\t--sandbox\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\t-t\t\t\t--lint-old\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\t-V\t\t\t--version\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\nTo report bugs, use the `gawkbug' program.\nFor full instructions, see the node `Bugs' in `gawk.info'\nwhich is section `Reporting Problems and Bugs' in the\nprinted version.  This same information may be found at\nhttps://www.gnu.org/software/gawk/manual/html_node/Bugs.html.\nPLEASE do NOT try to report bugs by posting in comp.lang.awk,\nor by using a web forum such as Stack Overflow.\n\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    if patchlevel >= 60 as libc::c_int
        || *(*__ctype_b_loc())
            .offset(
                (*::std::mem::transmute::<
                    &[u8; 6],
                    &[libc::c_char; 6],
                >(
                    b"5.2.2\0",
                ))[(strlen(b"5.2.2\0" as *const u8 as *const libc::c_char))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
                    as libc::c_int as isize,
            ) as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        url = beta_url.as_ptr();
    } else {
        url = gnu_url.as_ptr();
    }
    fprintf(
        fp,
        dcgettext(
            0 as *const libc::c_char,
            b"Source code for gawk may be obtained from\n%s/gawk-%s.tar.gz\n\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        url,
        b"5.2.2\0" as *const u8 as *const libc::c_char,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"gawk is a pattern scanning and processing language.\nBy default it reads standard input and writes standard output.\n\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        fp,
    );
    fprintf(
        fp,
        dcgettext(
            0 as *const libc::c_char,
            b"Examples:\n\t%s '{ sum += $1 }; END { print sum }' file\n\t%s -F: '{ print $1 }' /etc/passwd\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        myname,
        myname,
    );
    fflush(fp);
    if ferror(fp) != 0 {
        os_maybe_set_errno();
        if *__errno_location() == 32 as libc::c_int {
            signal(13 as libc::c_int, None);
            kill(getpid(), 13 as libc::c_int);
        }
        if fp == stdout {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"main.c\0" as *const u8 as *const libc::c_char,
                719 as libc::c_int,
            );
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"error writing standard output: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                strerror(*__errno_location()),
            );
        } else if fp == stderr {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"main.c\0" as *const u8 as *const libc::c_char,
                721 as libc::c_int,
            );
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"error writing standard error: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                strerror(*__errno_location()),
            );
        }
        exit(1 as libc::c_int);
    }
    exit(exitval);
}
unsafe extern "C" fn copyleft() -> ! {
    static mut blurb_part1: [libc::c_char; 297] = unsafe {
        *::std::mem::transmute::<
            &[u8; 297],
            &[libc::c_char; 297],
        >(
            b"Copyright (C) 1989, 1991-%d Free Software Foundation.\n\nThis program is free software; you can redistribute it and/or modify\nit under the terms of the GNU General Public License as published by\nthe Free Software Foundation; either version 3 of the License, or\n(at your option) any later version.\n\n\0",
        )
    };
    static mut blurb_part2: [libc::c_char; 236] = unsafe {
        *::std::mem::transmute::<
            &[u8; 236],
            &[libc::c_char; 236],
        >(
            b"This program is distributed in the hope that it will be useful,\nbut WITHOUT ANY WARRANTY; without even the implied warranty of\nMERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\nGNU General Public License for more details.\n\n\0",
        )
    };
    static mut blurb_part3: [libc::c_char; 134] = unsafe {
        *::std::mem::transmute::<
            &[u8; 134],
            &[libc::c_char; 134],
        >(
            b"You should have received a copy of the GNU General Public License\nalong with this program. If not, see http://www.gnu.org/licenses/.\n\0",
        )
    };
    printf(
        dcgettext(0 as *const libc::c_char, blurb_part1.as_ptr(), 5 as libc::c_int),
        2023 as libc::c_int,
    );
    fputs(
        dcgettext(0 as *const libc::c_char, blurb_part2.as_ptr(), 5 as libc::c_int),
        stdout,
    );
    fputs(
        dcgettext(0 as *const libc::c_char, blurb_part3.as_ptr(), 5 as libc::c_int),
        stdout,
    );
    fflush(stdout);
    if ferror(stdout) != 0 {
        os_maybe_set_errno();
        if *__errno_location() != 32 as libc::c_int {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"main.c\0" as *const u8 as *const libc::c_char,
                764 as libc::c_int,
            );
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"error writing standard output: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                strerror(*__errno_location()),
            );
        }
        exit(1 as libc::c_int);
    }
    exit(0 as libc::c_int);
}
unsafe extern "C" fn cmdline_fs(mut str: *mut libc::c_char) {
    let mut tmp: *mut *mut NODE = 0 as *mut *mut NODE;
    tmp = &mut (*FS_node).sub.nodep.l.lptr;
    unref(*tmp);
    if *str.offset(0 as libc::c_int as isize) as libc::c_int == 't' as i32
        && *str.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
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
                b"main.c\0" as *const u8 as *const libc::c_char,
                789 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"-Ft does not set FS to tab in POSIX awk\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0
            && do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint == 0
        {
            *str.offset(0 as libc::c_int as isize) = '\t' as i32 as libc::c_char;
        }
    }
    *tmp = make_str_node(str, strlen(str), 1 as libc::c_int);
    set_FS();
}
unsafe extern "C" fn init_args(
    mut argc0: libc::c_int,
    mut argc: libc::c_int,
    mut argv0: *const libc::c_char,
    mut argv: *mut *mut libc::c_char,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut sub: *mut NODE = 0 as *mut NODE;
    let mut val: *mut NODE = 0 as *mut NODE;
    let mut shadow_node: *mut NODE = 0 as *mut NODE;
    ARGV_node = install_symbol(
        estrdup(
            b"ARGV\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as size_t,
        ),
        Node_var_array,
    );
    sub = make_number.unwrap()(0.0f64);
    val = make_str_node(argv0, strlen(argv0), 0 as libc::c_int);
    (*val)
        .flags = ::std::mem::transmute::<
        libc::c_uint,
        flagvals,
    >((*val).flags as libc::c_uint | USER_INPUT as libc::c_int as libc::c_uint);
    assoc_set(ARGV_node, sub, val);
    if do_flags as libc::c_uint & DO_SANDBOX as libc::c_int as libc::c_uint != 0 {
        shadow_node = make_array();
        sub = make_str_node(argv0, strlen(argv0), 0 as libc::c_int);
        val = make_number.unwrap()(0.0f64);
        assoc_set(shadow_node, sub, val);
    }
    i = argc0;
    j = 1 as libc::c_int;
    while i < argc {
        sub = make_number.unwrap()(j as libc::c_double);
        val = make_str_node(
            *argv.offset(i as isize),
            strlen(*argv.offset(i as isize)),
            0 as libc::c_int,
        );
        (*val)
            .flags = ::std::mem::transmute::<
            libc::c_uint,
            flagvals,
        >((*val).flags as libc::c_uint | USER_INPUT as libc::c_int as libc::c_uint);
        assoc_set(ARGV_node, sub, val);
        if do_flags as libc::c_uint & DO_SANDBOX as libc::c_int as libc::c_uint != 0 {
            sub = make_str_node(
                *argv.offset(i as isize),
                strlen(*argv.offset(i as isize)),
                0 as libc::c_int,
            );
            val = make_number.unwrap()(0.0f64);
            assoc_set(shadow_node, sub, val);
        }
        i += 1;
        i;
        j += 1;
        j;
    }
    ARGC_node = install_symbol(
        estrdup(
            b"ARGC\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as size_t,
        ),
        Node_var,
    );
    (*ARGC_node).sub.nodep.l.lptr = make_number.unwrap()(j as libc::c_double);
    if do_flags as libc::c_uint & DO_SANDBOX as libc::c_int as libc::c_uint != 0 {
        init_argv_array(ARGV_node, shadow_node);
    }
}
static mut varinit: [varinit; 29] = unsafe {
    [
        {
            let mut init = varinit {
                spec: 0 as *const *mut NODE as *mut *mut NODE,
                name: b"ARGC\0" as *const u8 as *const libc::c_char,
                strval: 0 as *const libc::c_char,
                numval: 0 as libc::c_int as libc::c_double,
                update: None,
                assign: None,
                do_assign: 0 as libc::c_int != 0,
                flags: 0x1 as libc::c_int,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &ARGIND_node as *const *mut NODE as *mut *mut NODE,
                name: b"ARGIND\0" as *const u8 as *const libc::c_char,
                strval: 0 as *const libc::c_char,
                numval: 0 as libc::c_int as libc::c_double,
                update: None,
                assign: None,
                do_assign: 0 as libc::c_int != 0,
                flags: 0x2 as libc::c_int,
            };
            init
        },
        {
            let mut init = varinit {
                spec: 0 as *const *mut NODE as *mut *mut NODE,
                name: b"ARGV\0" as *const u8 as *const libc::c_char,
                strval: 0 as *const libc::c_char,
                numval: 0 as libc::c_int as libc::c_double,
                update: None,
                assign: None,
                do_assign: 0 as libc::c_int != 0,
                flags: 0x1 as libc::c_int,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &BINMODE_node as *const *mut NODE as *mut *mut NODE,
                name: b"BINMODE\0" as *const u8 as *const libc::c_char,
                strval: 0 as *const libc::c_char,
                numval: 0 as libc::c_int as libc::c_double,
                update: None,
                assign: Some(set_BINMODE as unsafe extern "C" fn() -> ()),
                do_assign: 0 as libc::c_int != 0,
                flags: 0x2 as libc::c_int,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &CONVFMT_node as *const *mut NODE as *mut *mut NODE,
                name: b"CONVFMT\0" as *const u8 as *const libc::c_char,
                strval: b"%.6g\0" as *const u8 as *const libc::c_char,
                numval: 0 as libc::c_int as libc::c_double,
                update: None,
                assign: Some(set_CONVFMT as unsafe extern "C" fn() -> ()),
                do_assign: 1 as libc::c_int != 0,
                flags: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = varinit {
                spec: 0 as *const *mut NODE as *mut *mut NODE,
                name: b"ENVIRON\0" as *const u8 as *const libc::c_char,
                strval: 0 as *const libc::c_char,
                numval: 0 as libc::c_int as libc::c_double,
                update: None,
                assign: None,
                do_assign: 0 as libc::c_int != 0,
                flags: 0x1 as libc::c_int,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &ERRNO_node as *const *mut NODE as *mut *mut NODE,
                name: b"ERRNO\0" as *const u8 as *const libc::c_char,
                strval: b"\0" as *const u8 as *const libc::c_char,
                numval: 0 as libc::c_int as libc::c_double,
                update: None,
                assign: None,
                do_assign: 0 as libc::c_int != 0,
                flags: 0x2 as libc::c_int,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &FIELDWIDTHS_node as *const *mut NODE as *mut *mut NODE,
                name: b"FIELDWIDTHS\0" as *const u8 as *const libc::c_char,
                strval: b"\0" as *const u8 as *const libc::c_char,
                numval: 0 as libc::c_int as libc::c_double,
                update: None,
                assign: Some(set_FIELDWIDTHS as unsafe extern "C" fn() -> ()),
                do_assign: 0 as libc::c_int != 0,
                flags: 0x2 as libc::c_int,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &FILENAME_node as *const *mut NODE as *mut *mut NODE,
                name: b"FILENAME\0" as *const u8 as *const libc::c_char,
                strval: b"\0" as *const u8 as *const libc::c_char,
                numval: 0 as libc::c_int as libc::c_double,
                update: None,
                assign: None,
                do_assign: 0 as libc::c_int != 0,
                flags: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &FNR_node as *const *mut NODE as *mut *mut NODE,
                name: b"FNR\0" as *const u8 as *const libc::c_char,
                strval: 0 as *const libc::c_char,
                numval: 0 as libc::c_int as libc::c_double,
                update: Some(update_FNR as unsafe extern "C" fn() -> ()),
                assign: Some(set_FNR as unsafe extern "C" fn() -> ()),
                do_assign: 1 as libc::c_int != 0,
                flags: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &FS_node as *const *mut NODE as *mut *mut NODE,
                name: b"FS\0" as *const u8 as *const libc::c_char,
                strval: b" \0" as *const u8 as *const libc::c_char,
                numval: 0 as libc::c_int as libc::c_double,
                update: None,
                assign: Some(set_FS as unsafe extern "C" fn() -> ()),
                do_assign: 0 as libc::c_int != 0,
                flags: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &FPAT_node as *const *mut NODE as *mut *mut NODE,
                name: b"FPAT\0" as *const u8 as *const libc::c_char,
                strval: b"[^[:space:]]+\0" as *const u8 as *const libc::c_char,
                numval: 0 as libc::c_int as libc::c_double,
                update: None,
                assign: Some(set_FPAT as unsafe extern "C" fn() -> ()),
                do_assign: 0 as libc::c_int != 0,
                flags: 0x2 as libc::c_int,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &IGNORECASE_node as *const *mut NODE as *mut *mut NODE,
                name: b"IGNORECASE\0" as *const u8 as *const libc::c_char,
                strval: 0 as *const libc::c_char,
                numval: 0 as libc::c_int as libc::c_double,
                update: None,
                assign: Some(set_IGNORECASE as unsafe extern "C" fn() -> ()),
                do_assign: 0 as libc::c_int != 0,
                flags: 0x2 as libc::c_int,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &LINT_node as *const *mut NODE as *mut *mut NODE,
                name: b"LINT\0" as *const u8 as *const libc::c_char,
                strval: 0 as *const libc::c_char,
                numval: 0 as libc::c_int as libc::c_double,
                update: None,
                assign: Some(set_LINT as unsafe extern "C" fn() -> ()),
                do_assign: 0 as libc::c_int != 0,
                flags: 0x2 as libc::c_int,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &PREC_node as *const *mut NODE as *mut *mut NODE,
                name: b"PREC\0" as *const u8 as *const libc::c_char,
                strval: 0 as *const libc::c_char,
                numval: 53 as libc::c_int as libc::c_double,
                update: None,
                assign: Some(set_PREC as unsafe extern "C" fn() -> ()),
                do_assign: 0 as libc::c_int != 0,
                flags: 0x2 as libc::c_int,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &NF_node as *const *mut NODE as *mut *mut NODE,
                name: b"NF\0" as *const u8 as *const libc::c_char,
                strval: 0 as *const libc::c_char,
                numval: -(1 as libc::c_int) as libc::c_double,
                update: Some(update_NF as unsafe extern "C" fn() -> ()),
                assign: Some(set_NF as unsafe extern "C" fn() -> ()),
                do_assign: 0 as libc::c_int != 0,
                flags: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &NR_node as *const *mut NODE as *mut *mut NODE,
                name: b"NR\0" as *const u8 as *const libc::c_char,
                strval: 0 as *const libc::c_char,
                numval: 0 as libc::c_int as libc::c_double,
                update: Some(update_NR as unsafe extern "C" fn() -> ()),
                assign: Some(set_NR as unsafe extern "C" fn() -> ()),
                do_assign: 1 as libc::c_int != 0,
                flags: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &OFMT_node as *const *mut NODE as *mut *mut NODE,
                name: b"OFMT\0" as *const u8 as *const libc::c_char,
                strval: b"%.6g\0" as *const u8 as *const libc::c_char,
                numval: 0 as libc::c_int as libc::c_double,
                update: None,
                assign: Some(set_OFMT as unsafe extern "C" fn() -> ()),
                do_assign: 1 as libc::c_int != 0,
                flags: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &OFS_node as *const *mut NODE as *mut *mut NODE,
                name: b"OFS\0" as *const u8 as *const libc::c_char,
                strval: b" \0" as *const u8 as *const libc::c_char,
                numval: 0 as libc::c_int as libc::c_double,
                update: None,
                assign: Some(set_OFS as unsafe extern "C" fn() -> ()),
                do_assign: 1 as libc::c_int != 0,
                flags: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &ORS_node as *const *mut NODE as *mut *mut NODE,
                name: b"ORS\0" as *const u8 as *const libc::c_char,
                strval: b"\n\0" as *const u8 as *const libc::c_char,
                numval: 0 as libc::c_int as libc::c_double,
                update: None,
                assign: Some(set_ORS as unsafe extern "C" fn() -> ()),
                do_assign: 1 as libc::c_int != 0,
                flags: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = varinit {
                spec: 0 as *const *mut NODE as *mut *mut NODE,
                name: b"PROCINFO\0" as *const u8 as *const libc::c_char,
                strval: 0 as *const libc::c_char,
                numval: 0 as libc::c_int as libc::c_double,
                update: None,
                assign: None,
                do_assign: 0 as libc::c_int != 0,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &RLENGTH_node as *const *mut NODE as *mut *mut NODE,
                name: b"RLENGTH\0" as *const u8 as *const libc::c_char,
                strval: 0 as *const libc::c_char,
                numval: 0 as libc::c_int as libc::c_double,
                update: None,
                assign: None,
                do_assign: 0 as libc::c_int != 0,
                flags: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &ROUNDMODE_node as *const *mut NODE as *mut *mut NODE,
                name: b"ROUNDMODE\0" as *const u8 as *const libc::c_char,
                strval: b"N\0" as *const u8 as *const libc::c_char,
                numval: 0 as libc::c_int as libc::c_double,
                update: None,
                assign: Some(set_ROUNDMODE as unsafe extern "C" fn() -> ()),
                do_assign: 0 as libc::c_int != 0,
                flags: 0x2 as libc::c_int,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &RS_node as *const *mut NODE as *mut *mut NODE,
                name: b"RS\0" as *const u8 as *const libc::c_char,
                strval: b"\n\0" as *const u8 as *const libc::c_char,
                numval: 0 as libc::c_int as libc::c_double,
                update: None,
                assign: Some(set_RS as unsafe extern "C" fn() -> ()),
                do_assign: 1 as libc::c_int != 0,
                flags: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &RSTART_node as *const *mut NODE as *mut *mut NODE,
                name: b"RSTART\0" as *const u8 as *const libc::c_char,
                strval: 0 as *const libc::c_char,
                numval: 0 as libc::c_int as libc::c_double,
                update: None,
                assign: None,
                do_assign: 0 as libc::c_int != 0,
                flags: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &RT_node as *const *mut NODE as *mut *mut NODE,
                name: b"RT\0" as *const u8 as *const libc::c_char,
                strval: b"\0" as *const u8 as *const libc::c_char,
                numval: 0 as libc::c_int as libc::c_double,
                update: None,
                assign: None,
                do_assign: 0 as libc::c_int != 0,
                flags: 0x2 as libc::c_int,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &SUBSEP_node as *const *mut NODE as *mut *mut NODE,
                name: b"SUBSEP\0" as *const u8 as *const libc::c_char,
                strval: b"\x1C\0" as *const u8 as *const libc::c_char,
                numval: 0 as libc::c_int as libc::c_double,
                update: None,
                assign: Some(set_SUBSEP as unsafe extern "C" fn() -> ()),
                do_assign: 1 as libc::c_int != 0,
                flags: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &TEXTDOMAIN_node as *const *mut NODE as *mut *mut NODE,
                name: b"TEXTDOMAIN\0" as *const u8 as *const libc::c_char,
                strval: b"messages\0" as *const u8 as *const libc::c_char,
                numval: 0 as libc::c_int as libc::c_double,
                update: None,
                assign: Some(set_TEXTDOMAIN as unsafe extern "C" fn() -> ()),
                do_assign: 1 as libc::c_int != 0,
                flags: 0x2 as libc::c_int,
            };
            init
        },
        {
            let mut init = varinit {
                spec: 0 as *const *mut NODE as *mut *mut NODE,
                name: 0 as *const libc::c_char,
                strval: 0 as *const libc::c_char,
                numval: 0 as libc::c_int as libc::c_double,
                update: None,
                assign: None,
                do_assign: 0 as libc::c_int != 0,
                flags: 0 as libc::c_int,
            };
            init
        },
    ]
};
unsafe extern "C" fn init_vars() {
    let mut vp: *const varinit = 0 as *const varinit;
    let mut n: *mut NODE = 0 as *mut NODE;
    vp = varinit.as_ptr();
    while !((*vp).name).is_null() {
        if !((*vp).flags & 0x1 as libc::c_int != 0 as libc::c_int) {
            *(*vp)
                .spec = install_symbol(
                estrdup((*vp).name, strlen((*vp).name)),
                Node_var,
            );
            n = *(*vp).spec;
            if !((*vp).strval).is_null() {
                (*n)
                    .sub
                    .nodep
                    .l
                    .lptr = make_str_node(
                    (*vp).strval,
                    strlen((*vp).strval),
                    0 as libc::c_int,
                );
            } else {
                (*n).sub.nodep.l.lptr = make_number.unwrap()((*vp).numval);
            }
            (*n).sub.nodep.x.aptr = (*vp).assign;
            (*n).sub.nodep.r.uptr = (*vp).update;
            if (*vp).do_assign {
                (Some(((*vp).assign).unwrap())).unwrap()();
            }
        }
        vp = vp.offset(1);
        vp;
    }
    if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint == 0 {
        load_procinfo();
    }
    load_environ();
}
unsafe extern "C" fn path_environ(
    mut pname: *const libc::c_char,
    mut dflt: *const libc::c_char,
) {
    let mut val: *const libc::c_char = 0 as *const libc::c_char;
    let mut aptr: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    tmp = make_str_node(pname, strlen(pname), 0 as libc::c_int);
    val = getenv(pname);
    if val.is_null() || *val as libc::c_int == '\0' as i32 {
        val = dflt;
    }
    aptr = ((*(*ENVIRON_node).sub.nodep.l.lp).lookup).unwrap()(ENVIRON_node, tmp);
    if (**aptr).sub.val.slen == 0 as libc::c_int as libc::c_ulong {
        unref(*aptr);
        *aptr = make_str_node(val, strlen(val), 0 as libc::c_int);
    }
    unref(tmp);
}
unsafe extern "C" fn load_environ() -> *mut NODE {
    extern "C" {
        #[link_name = "environ"]
        static mut environ_0: *mut *mut libc::c_char;
    }
    let mut var: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut sub: *mut NODE = 0 as *mut NODE;
    let mut newval: *mut NODE = 0 as *mut NODE;
    static mut been_here: bool = 0 as libc::c_int != 0;
    if been_here {
        return ENVIRON_node;
    }
    been_here = 1 as libc::c_int != 0;
    ENVIRON_node = install_symbol(
        estrdup(
            b"ENVIRON\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as size_t,
        ),
        Node_var_array,
    );
    (*ENVIRON_node).sub.nodep.l.lp = &str_array_func;
    i = 0 as libc::c_int;
    while !(*environ.offset(i as isize)).is_null() {
        static mut nullstr: [libc::c_char; 1] = unsafe {
            *::std::mem::transmute::<&[u8; 1], &mut [libc::c_char; 1]>(b"\0")
        };
        var = *environ.offset(i as isize);
        val = strchr(var, '=' as i32);
        if !val.is_null() {
            let fresh1 = val;
            val = val.offset(1);
            *fresh1 = '\0' as i32 as libc::c_char;
        } else {
            val = nullstr.as_mut_ptr();
        }
        sub = make_str_node(var, strlen(var), 0 as libc::c_int);
        newval = make_str_node(val, strlen(val), 0 as libc::c_int);
        (*newval)
            .flags = ::std::mem::transmute::<
            libc::c_uint,
            flagvals,
        >((*newval).flags as libc::c_uint | USER_INPUT as libc::c_int as libc::c_uint);
        assoc_set(ENVIRON_node, sub, newval);
        if val != nullstr.as_mut_ptr() {
            val = val.offset(-1);
            *val = '=' as i32 as libc::c_char;
        }
        i += 1;
        i;
    }
    path_environ(b"AWKPATH\0" as *const u8 as *const libc::c_char, defpath);
    path_environ(b"AWKLIBPATH\0" as *const u8 as *const libc::c_char, deflibpath);
    init_env_array(ENVIRON_node);
    return ENVIRON_node;
}
unsafe extern "C" fn load_procinfo_argv() {
    let mut sub: *mut NODE = 0 as *mut NODE;
    let mut val: *mut NODE = 0 as *mut NODE;
    let mut argv_array: *mut NODE = 0 as *mut NODE;
    let mut i: libc::c_int = 0;
    argv_array = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
    if !argv_array.is_null() {
        nextfree[BLOCK_NODE as libc::c_int as usize]
            .freep = (*(argv_array as *mut block_item)).freep;
    } else {
        argv_array = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
    };
    memset(
        argv_array as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<NODE>() as libc::c_ulong,
    );
    null_array(argv_array);
    (*argv_array).sub.nodep.x.extra = PROCINFO_node;
    (*argv_array)
        .sub
        .nodep
        .name = estrdup(
        b"argv\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as size_t,
    );
    i = 0 as libc::c_int;
    while !(*d_argv.offset(i as isize)).is_null() {
        sub = make_number.unwrap()(i as libc::c_double);
        val = make_str_node(
            *d_argv.offset(i as isize),
            strlen(*d_argv.offset(i as isize)),
            0 as libc::c_int,
        );
        assoc_set(argv_array, sub, val);
        i += 1;
        i;
    }
    sub = make_str_node(
        b"argv\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as size_t,
        0 as libc::c_int,
    );
    assoc_set(PROCINFO_node, sub, argv_array);
}
unsafe extern "C" fn load_procinfo() -> *mut NODE {
    let mut i: libc::c_int = 0;
    let mut name: [libc::c_char; 100] = [0; 100];
    let mut value: libc::c_double = 0.;
    static mut been_here: bool = 0 as libc::c_int != 0;
    if been_here {
        return PROCINFO_node;
    }
    been_here = 1 as libc::c_int != 0;
    PROCINFO_node = install_symbol(
        estrdup(
            b"PROCINFO\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as size_t,
        ),
        Node_var_array,
    );
    update_PROCINFO_str(
        b"version\0" as *const u8 as *const libc::c_char,
        b"5.2.2\0" as *const u8 as *const libc::c_char,
    );
    update_PROCINFO_str(
        b"strftime\0" as *const u8 as *const libc::c_char,
        def_strftime_format.as_ptr(),
    );
    update_PROCINFO_str(
        b"platform\0" as *const u8 as *const libc::c_char,
        platform_name(),
    );
    update_PROCINFO_num(
        b"api_major\0" as *const u8 as *const libc::c_char,
        GAWK_API_MAJOR_VERSION as libc::c_int as libc::c_double,
    );
    update_PROCINFO_num(
        b"api_minor\0" as *const u8 as *const libc::c_char,
        GAWK_API_MINOR_VERSION as libc::c_int as libc::c_double,
    );
    value = getpgrp() as libc::c_double;
    update_PROCINFO_num(b"pgrpid\0" as *const u8 as *const libc::c_char, value);
    value = getpid() as libc::c_double;
    update_PROCINFO_num(b"pid\0" as *const u8 as *const libc::c_char, value);
    value = getppid() as libc::c_double;
    update_PROCINFO_num(b"ppid\0" as *const u8 as *const libc::c_char, value);
    value = getuid() as libc::c_double;
    update_PROCINFO_num(b"uid\0" as *const u8 as *const libc::c_char, value);
    value = geteuid() as libc::c_double;
    update_PROCINFO_num(b"euid\0" as *const u8 as *const libc::c_char, value);
    value = getgid() as libc::c_double;
    update_PROCINFO_num(b"gid\0" as *const u8 as *const libc::c_char, value);
    value = getegid() as libc::c_double;
    update_PROCINFO_num(b"egid\0" as *const u8 as *const libc::c_char, value);
    update_PROCINFO_str(
        b"FS\0" as *const u8 as *const libc::c_char,
        current_field_sep_str(),
    );
    i = 0 as libc::c_int;
    while i < ngroups {
        sprintf(
            name.as_mut_ptr(),
            b"group%d\0" as *const u8 as *const libc::c_char,
            i + 1 as libc::c_int,
        );
        value = *groupset.offset(i as isize) as libc::c_double;
        update_PROCINFO_num(name.as_mut_ptr(), value);
        i += 1;
        i;
    }
    if !groupset.is_null() {
        free(groupset as *mut libc::c_void);
        groupset = 0 as *mut gid_t;
    }
    load_procinfo_argv();
    return PROCINFO_node;
}
pub unsafe extern "C" fn is_std_var(mut var: *const libc::c_char) -> libc::c_int {
    let mut vp: *const varinit = 0 as *const varinit;
    vp = varinit.as_ptr();
    while !((*vp).name).is_null() {
        if strcmp((*vp).name, var) == 0 as libc::c_int {
            if (do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint
                != 0
                || do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint
                    != 0) && (*vp).flags & 0x2 as libc::c_int != 0 as libc::c_int
            {
                return 0 as libc::c_int;
            }
            return 1 as libc::c_int;
        }
        vp = vp.offset(1);
        vp;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn is_off_limits_var(mut var: *const libc::c_char) -> libc::c_int {
    let mut vp: *const varinit = 0 as *const varinit;
    vp = varinit.as_ptr();
    while !((*vp).name).is_null() {
        if strcmp((*vp).name, var) == 0 as libc::c_int {
            return ((*vp).flags & 0x4 as libc::c_int == 0 as libc::c_int) as libc::c_int;
        }
        vp = vp.offset(1);
        vp;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn get_spec_varname(mut fptr: Func_ptr) -> *const libc::c_char {
    let mut vp: *const varinit = 0 as *const varinit;
    if fptr.is_none() {
        return 0 as *const libc::c_char;
    }
    vp = varinit.as_ptr();
    while !((*vp).name).is_null() {
        if (*vp).assign == fptr || (*vp).update == fptr {
            return (*vp).name;
        }
        vp = vp.offset(1);
        vp;
    }
    return 0 as *const libc::c_char;
}
pub unsafe extern "C" fn arg_assign(
    mut arg: *mut libc::c_char,
    mut initing: bool,
) -> libc::c_int {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut badvar: bool = false;
    let mut var: *mut NODE = 0 as *mut NODE;
    let mut it: *mut NODE = 0 as *mut NODE;
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut save_FNR: libc::c_long = 0;
    if !initing && disallow_var_assigns as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    cp = strchr(arg, '=' as i32);
    if cp.is_null() {
        if !initing {
            return 0 as libc::c_int;
        }
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: `%s' argument to `-v' not in `var=value' form\n\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            myname,
            arg,
        );
        usage(1 as libc::c_int, stderr);
    }
    let fresh2 = cp;
    cp = cp.offset(1);
    *fresh2 = '\0' as i32 as libc::c_char;
    source = 0 as *mut libc::c_char;
    sourceline = 0 as libc::c_int;
    save_FNR = FNR;
    FNR = 0 as libc::c_int as libc::c_long;
    badvar = 0 as libc::c_int != 0;
    if !is_letter(
        *arg.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int,
    ) {
        badvar = 1 as libc::c_int != 0;
    } else {
        cp2 = arg.offset(1 as libc::c_int as isize);
        while *cp2 != 0 {
            if !is_identchar(*cp2 as libc::c_uchar as libc::c_int)
                && *cp2 as libc::c_int != ':' as i32
            {
                badvar = 1 as libc::c_int != 0;
                break;
            } else {
                cp2 = cp2.offset(1);
                cp2;
            }
        }
    }
    if badvar {
        if initing {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"main.c\0" as *const u8 as *const libc::c_char,
                1240 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"`%s' is not a legal variable name\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                arg,
            );
        }
        if do_flags as libc::c_uint
            & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                as libc::c_uint != 0
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"main.c\0" as *const u8 as *const libc::c_char,
                1243 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"`%s' is not a variable name, looking for file `%s=%s'\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                arg,
                arg,
                cp,
            );
        }
    } else if !validate_qualified_name(arg) {
        badvar = 1 as libc::c_int != 0;
    } else {
        if check_special(arg) >= 0 as libc::c_int {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"main.c\0" as *const u8 as *const libc::c_char,
                1257 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot use gawk builtin `%s' as variable name\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                arg,
            );
        }
        if !initing {
            var = lookup(arg);
            if !var.is_null()
                && (*var).type_0 as libc::c_uint
                    == Node_func as libc::c_int as libc::c_uint
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"main.c\0" as *const u8 as *const libc::c_char,
                    1262 as libc::c_int,
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
                        b"cannot use function `%s' as variable name\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    arg,
                );
            }
        }
        cp2 = cp.offset(strlen(cp) as isize).offset(-(1 as libc::c_int as isize));
        if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint == 0
            && strlen(cp) >= 3 as libc::c_int as libc::c_ulong
            && *cp.offset(0 as libc::c_int as isize) as libc::c_int == '@' as i32
            && *cp.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
            && *cp2 as libc::c_int == '/' as i32
        {
            let mut len: size_t = (strlen(cp))
                .wrapping_sub(3 as libc::c_int as libc::c_ulong);
            cp2 = ezalloc_real(
                len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                b"arg_assign\0" as *const u8 as *const libc::c_char,
                b"cp2\0" as *const u8 as *const libc::c_char,
                b"main.c\0" as *const u8 as *const libc::c_char,
                1272 as libc::c_int,
            ) as *mut libc::c_char;
            memcpy(
                cp2 as *mut libc::c_void,
                cp.offset(2 as libc::c_int as isize) as *const libc::c_void,
                len,
            );
            it = make_typed_regex(cp2, len);
        } else {
            if do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint != 0
                && !(strchr(cp, '\n' as i32)).is_null()
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"main.c\0" as *const u8 as *const libc::c_char,
                    1284 as libc::c_int,
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
                        b"POSIX does not allow physical newlines in string values\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            it = make_str_node(cp, strlen(cp), 1 as libc::c_int | 4 as libc::c_int);
            (*it)
                .flags = ::std::mem::transmute::<
                libc::c_uint,
                flagvals,
            >((*it).flags as libc::c_uint | USER_INPUT as libc::c_int as libc::c_uint);
            if do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint != 0 {
                setlocale(1 as libc::c_int, b"C\0" as *const u8 as *const libc::c_char);
            }
            force_number(it);
            if do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint != 0 {
                setlocale(1 as libc::c_int, locale);
            }
        }
        cp2 = estrdup(arg, cp.offset_from(arg) as libc::c_long as size_t);
        var = variable(0 as libc::c_int, cp2, Node_var);
        if var.is_null() {
            final_exit(2 as libc::c_int);
        }
        if (*var).type_0 as libc::c_uint == Node_var as libc::c_int as libc::c_uint
            && ((*var).sub.nodep.r.uptr).is_some()
        {
            ((*var).sub.nodep.r.uptr).unwrap()();
        }
        lhs = if (*var).type_0 as libc::c_uint == Node_var as libc::c_int as libc::c_uint
            && !((*var).sub.nodep.l.lptr == Nnull_string)
        {
            &mut (*var).sub.nodep.l.lptr
        } else {
            r_get_lhs(var, 0 as libc::c_int != 0)
        };
        unref(*lhs);
        *lhs = it;
        if (*var).type_0 as libc::c_uint == Node_var as libc::c_int as libc::c_uint
            && ((*var).sub.nodep.x.aptr).is_some()
        {
            ((*var).sub.nodep.x.aptr).unwrap()();
        }
    }
    if !initing {
        cp = cp.offset(-1);
        *cp = '=' as i32 as libc::c_char;
    }
    FNR = save_FNR;
    return !badvar as libc::c_int;
}
unsafe extern "C" fn catchsig(mut sig: libc::c_int) {
    if sig == 8 as libc::c_int {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"main.c\0" as *const u8 as *const libc::c_char,
            1341 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"floating point exception\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else if sig == 11 as libc::c_int || sig == 7 as libc::c_int {
        if errcount > 0 as libc::c_int {
            exit(2 as libc::c_int);
        }
        set_loc(b"main.c\0" as *const u8 as *const libc::c_char, 1350 as libc::c_int);
        msg(
            dcgettext(
                0 as *const libc::c_char,
                b"fatal error: internal error\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fflush(0 as *mut FILE);
        abort();
    } else {
        r_fatal(
            b"internal error: file %s, line %d: unexpected signal, number %d (%s)\0"
                as *const u8 as *const libc::c_char,
            b"main.c\0" as *const u8 as *const libc::c_char,
            1357 as libc::c_int,
            sig,
            strsignal(sig),
        );
    };
}
unsafe extern "C" fn nostalgia() -> ! {
    fprintf(
        stderr,
        b"awk: bailing out near line 1\n\0" as *const u8 as *const libc::c_char,
    );
    fflush(stderr);
    abort();
}
unsafe extern "C" fn version() -> ! {
    printf(b"%s\0" as *const u8 as *const libc::c_char, version_string);
    printf(
        b", API %d.%d\0" as *const u8 as *const libc::c_char,
        GAWK_API_MAJOR_VERSION as libc::c_int,
        GAWK_API_MINOR_VERSION as libc::c_int,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    print_ext_versions();
    copyleft();
}
unsafe extern "C" fn init_fds() {
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
    let mut fd: libc::c_int = 0;
    let mut newfd: libc::c_int = 0;
    let opposite_mode: [*const libc::c_char; 3] = [
        b"w\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    ];
    fd = 0 as libc::c_int;
    while fd <= 2 as libc::c_int {
        if fstat(fd, &mut sbuf) < 0 as libc::c_int {
            newfd = devopen(
                b"/dev/null\0" as *const u8 as *const libc::c_char,
                opposite_mode[fd as usize],
            );
            newfd += 0 as libc::c_int;
        }
        fd += 1;
        fd;
    }
}
unsafe extern "C" fn init_groupset() {
    ngroups = getgroups(0 as libc::c_int, 0 as *mut __gid_t);
    if ngroups <= 0 as libc::c_int {
        return;
    }
    groupset = emalloc_real(
        (ngroups as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<gid_t>() as libc::c_ulong),
        b"init_groupset\0" as *const u8 as *const libc::c_char,
        b"groupset\0" as *const u8 as *const libc::c_char,
        b"main.c\0" as *const u8 as *const libc::c_char,
        1517 as libc::c_int,
    ) as *mut gid_t;
    ngroups = getgroups(ngroups, groupset);
    if ngroups == -(1 as libc::c_int) {
        free(groupset as *mut libc::c_void);
        ngroups = 0 as libc::c_int;
        groupset = 0 as *mut gid_t;
    }
}
pub unsafe extern "C" fn estrdup(
    mut str: *const libc::c_char,
    mut len: size_t,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = emalloc_real(
        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        b"estrdup\0" as *const u8 as *const libc::c_char,
        b"s\0" as *const u8 as *const libc::c_char,
        b"main.c\0" as *const u8 as *const libc::c_char,
        1535 as libc::c_int,
    ) as *mut libc::c_char;
    memcpy(s as *mut libc::c_void, str as *const libc::c_void, len);
    *s.offset(len as isize) = '\0' as i32 as libc::c_char;
    return s;
}
unsafe extern "C" fn init_locale(mut l: *mut lconv) {
    let mut t: *mut lconv = 0 as *mut lconv;
    t = localeconv();
    *l = *t;
    (*l).thousands_sep = estrdup((*t).thousands_sep, strlen((*t).thousands_sep));
    (*l).decimal_point = estrdup((*t).decimal_point, strlen((*t).decimal_point));
    (*l).grouping = estrdup((*t).grouping, strlen((*t).grouping));
    (*l).int_curr_symbol = estrdup((*t).int_curr_symbol, strlen((*t).int_curr_symbol));
    (*l).currency_symbol = estrdup((*t).currency_symbol, strlen((*t).currency_symbol));
    (*l)
        .mon_decimal_point = estrdup(
        (*t).mon_decimal_point,
        strlen((*t).mon_decimal_point),
    );
    (*l)
        .mon_thousands_sep = estrdup(
        (*t).mon_thousands_sep,
        strlen((*t).mon_thousands_sep),
    );
    (*l).mon_grouping = estrdup((*t).mon_grouping, strlen((*t).mon_grouping));
    (*l).positive_sign = estrdup((*t).positive_sign, strlen((*t).positive_sign));
    (*l).negative_sign = estrdup((*t).negative_sign, strlen((*t).negative_sign));
}
unsafe extern "C" fn save_argv(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    d_argv = emalloc_real(
        ((argc + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
        b"save_argv\0" as *const u8 as *const libc::c_char,
        b"d_argv\0" as *const u8 as *const libc::c_char,
        b"main.c\0" as *const u8 as *const libc::c_char,
        1580 as libc::c_int,
    ) as *mut *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < argc {
        let ref mut fresh3 = *d_argv.offset(i as isize);
        *fresh3 = estrdup(*argv.offset(i as isize), strlen(*argv.offset(i as isize)));
        i += 1;
        i;
    }
    let ref mut fresh4 = *d_argv.offset(argc as isize);
    *fresh4 = 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn update_global_values() {
    let mut vp: *const varinit = 0 as *const varinit;
    vp = varinit.as_ptr();
    while !((*vp).name).is_null() {
        if ((*vp).update).is_some() {
            ((*vp).update).unwrap()();
        }
        vp = vp.offset(1);
        vp;
    }
}
pub unsafe extern "C" fn getenv_long(mut name: *const libc::c_char) -> libc::c_long {
    let mut val: *const libc::c_char = 0 as *const libc::c_char;
    let mut newval: libc::c_long = 0;
    val = getenv(name);
    if !val.is_null()
        && *(*__ctype_b_loc()).offset(*val as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        newval = 0 as libc::c_int as libc::c_long;
        while *val as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(*val as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            newval = newval * 10 as libc::c_int as libc::c_long + *val as libc::c_long
                - '0' as i32 as libc::c_long;
            val = val.offset(1);
            val;
        }
        return newval;
    }
    return -(1 as libc::c_int) as libc::c_long;
}
unsafe extern "C" fn parse_args(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let mut current_block: u64;
    let mut optlist: *const libc::c_char = b"+F:f:v:W;bcCd::D::e:E:ghi:Il:L::nNo::Op::MPrSstVYZ:\0"
        as *const u8 as *const libc::c_char;
    let mut old_optind: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut scan: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut src: *mut libc::c_char = 0 as *mut libc::c_char;
    opterr = 0 as libc::c_int;
    save_argv(argc, argv);
    optopt = 0 as libc::c_int;
    old_optind = 1 as libc::c_int;
    loop {
        c = getopt_long(argc, argv, optlist, optab.as_ptr(), 0 as *mut libc::c_int);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        if do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint != 0 {
            opterr = 1 as libc::c_int;
        }
        match c {
            70 => {
                add_preassign(PRE_ASSIGN_FS, optarg);
                current_block = 16231175055492490595;
            }
            69 => {
                disallow_var_assigns = 1 as libc::c_int != 0;
                current_block = 9995362966312559327;
            }
            102 => {
                current_block = 9995362966312559327;
            }
            118 => {
                add_preassign(PRE_ASSIGN, optarg);
                current_block = 16231175055492490595;
            }
            98 => {
                do_binary = 1 as libc::c_int;
                current_block = 16231175055492490595;
            }
            99 => {
                do_flags = ::std::mem::transmute::<
                    libc::c_uint,
                    do_flag_values,
                >(
                    do_flags as libc::c_uint
                        | DO_TRADITIONAL as libc::c_int as libc::c_uint,
                );
                current_block = 16231175055492490595;
            }
            67 => {
                copyleft();
            }
            100 => {
                do_flags = ::std::mem::transmute::<
                    libc::c_uint,
                    do_flag_values,
                >(
                    do_flags as libc::c_uint
                        | DO_DUMP_VARS as libc::c_int as libc::c_uint,
                );
                if !optarg.is_null()
                    && *optarg.offset(0 as libc::c_int as isize) as libc::c_int
                        != '\0' as i32
                {
                    varfile = optarg;
                }
                current_block = 16231175055492490595;
            }
            68 => {
                do_flags = ::std::mem::transmute::<
                    libc::c_uint,
                    do_flag_values,
                >(do_flags as libc::c_uint | DO_DEBUG as libc::c_int as libc::c_uint);
                if !optarg.is_null()
                    && *optarg.offset(0 as libc::c_int as isize) as libc::c_int
                        != '\0' as i32
                {
                    command_file = optarg;
                }
                current_block = 16231175055492490595;
            }
            101 => {
                if *optarg.offset(0 as libc::c_int as isize) as libc::c_int
                    == '\0' as i32
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"main.c\0" as *const u8 as *const libc::c_char,
                        1704 as libc::c_int,
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
                            b"empty argument to `-e/--source' ignored\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                } else {
                    add_srcfile(
                        SRC_CMDLINE,
                        optarg,
                        srcfiles,
                        0 as *mut bool,
                        0 as *mut libc::c_int,
                    );
                }
                current_block = 16231175055492490595;
            }
            103 => {
                do_flags = ::std::mem::transmute::<
                    libc::c_uint,
                    do_flag_values,
                >(do_flags as libc::c_uint | DO_INTL as libc::c_int as libc::c_uint);
                current_block = 16231175055492490595;
            }
            104 => {
                usage(0 as libc::c_int, stdout);
            }
            105 => {
                add_srcfile(
                    SRC_INC,
                    optarg,
                    srcfiles,
                    0 as *mut bool,
                    0 as *mut libc::c_int,
                );
                current_block = 16231175055492490595;
            }
            73 => {
                do_itrace = 1 as libc::c_int != 0;
                current_block = 16231175055492490595;
            }
            108 => {
                add_srcfile(
                    SRC_EXTLIB,
                    optarg,
                    srcfiles,
                    0 as *mut bool,
                    0 as *mut libc::c_int,
                );
                current_block = 16231175055492490595;
            }
            76 => {
                do_flags = ::std::mem::transmute::<
                    libc::c_uint,
                    do_flag_values,
                >(
                    do_flags as libc::c_uint
                        | (DO_LINT_ALL as libc::c_int
                            | DO_LINT_EXTENSIONS as libc::c_int) as libc::c_uint,
                );
                if !optarg.is_null() {
                    if strcmp(optarg, b"fatal\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        lintfunc = Some(
                            r_fatal
                                as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                        );
                    } else if strcmp(
                        optarg,
                        b"invalid\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                        do_flags = ::std::mem::transmute::<
                            libc::c_uint,
                            do_flag_values,
                        >(
                            do_flags as libc::c_uint
                                & !(DO_LINT_ALL as libc::c_int) as libc::c_uint,
                        );
                        do_flags = ::std::mem::transmute::<
                            libc::c_uint,
                            do_flag_values,
                        >(
                            do_flags as libc::c_uint
                                | DO_LINT_INVALID as libc::c_int as libc::c_uint,
                        );
                    } else if strcmp(
                        optarg,
                        b"no-ext\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                        do_flags = ::std::mem::transmute::<
                            libc::c_uint,
                            do_flag_values,
                        >(
                            do_flags as libc::c_uint
                                & !(DO_LINT_EXTENSIONS as libc::c_int) as libc::c_uint,
                        );
                    }
                }
                current_block = 16231175055492490595;
            }
            116 => {
                do_flags = ::std::mem::transmute::<
                    libc::c_uint,
                    do_flag_values,
                >(do_flags as libc::c_uint | DO_LINT_OLD as libc::c_int as libc::c_uint);
                current_block = 16231175055492490595;
            }
            110 => {
                do_flags = ::std::mem::transmute::<
                    libc::c_uint,
                    do_flag_values,
                >(
                    do_flags as libc::c_uint
                        | DO_NON_DEC_DATA as libc::c_int as libc::c_uint,
                );
                current_block = 16231175055492490595;
            }
            78 => {
                use_lc_numeric = 1 as libc::c_int;
                current_block = 16231175055492490595;
            }
            79 => {
                do_optimize = 1 as libc::c_int != 0;
                current_block = 16231175055492490595;
            }
            112 => {
                if do_flags as libc::c_uint
                    & DO_PRETTY_PRINT as libc::c_int as libc::c_uint != 0
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"main.c\0" as *const u8 as *const libc::c_char,
                        1769 as libc::c_int,
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
                            b"`--profile' overrides `--pretty-print'\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                do_flags = ::std::mem::transmute::<
                    libc::c_uint,
                    do_flag_values,
                >(do_flags as libc::c_uint | DO_PROFILE as libc::c_int as libc::c_uint);
                current_block = 9548602284111555425;
            }
            111 => {
                current_block = 9548602284111555425;
            }
            77 => {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"main.c\0" as *const u8 as *const libc::c_char,
                    1786 as libc::c_int,
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
                        b"-M ignored: MPFR/GMP support not compiled in\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                current_block = 16231175055492490595;
            }
            80 => {
                do_flags = ::std::mem::transmute::<
                    libc::c_uint,
                    do_flag_values,
                >(do_flags as libc::c_uint | DO_POSIX as libc::c_int as libc::c_uint);
                current_block = 16231175055492490595;
            }
            114 => {
                do_flags = ::std::mem::transmute::<
                    libc::c_uint,
                    do_flag_values,
                >(
                    do_flags as libc::c_uint
                        | DO_INTERVALS as libc::c_int as libc::c_uint,
                );
                current_block = 16231175055492490595;
            }
            115 => {
                do_optimize = 0 as libc::c_int != 0;
                current_block = 16231175055492490595;
            }
            83 => {
                do_flags = ::std::mem::transmute::<
                    libc::c_uint,
                    do_flag_values,
                >(do_flags as libc::c_uint | DO_SANDBOX as libc::c_int as libc::c_uint);
                current_block = 16231175055492490595;
            }
            84 => {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"main.c\0" as *const u8 as *const libc::c_char,
                    1814 as libc::c_int,
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
                        b"Persistent memory is not supported.\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                current_block = 16231175055492490595;
            }
            86 => {
                do_version = 1 as libc::c_int;
                current_block = 16231175055492490595;
            }
            87 => {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: option `-W %s' unrecognized, ignored\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    *argv.offset(0 as libc::c_int as isize),
                    optarg,
                );
                current_block = 16231175055492490595;
            }
            0 => {
                current_block = 16231175055492490595;
            }
            89 | 90 => {
                current_block = 8802492972403975698;
            }
            63 | _ => {
                current_block = 8802492972403975698;
            }
        }
        match current_block {
            8802492972403975698 => {
                if do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint
                    == 0
                    && (optopt == '\0' as i32 || (strchr(optlist, optopt)).is_null())
                {
                    optind = old_optind;
                    stopped_early = 1 as libc::c_int != 0;
                    break;
                } else if optopt != '\0' as i32 {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: option requires an argument -- %c\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        myname,
                        optopt,
                    );
                    usage(1 as libc::c_int, stderr);
                }
            }
            9548602284111555425 => {
                if c == 'o' as i32
                    && do_flags as libc::c_uint
                        & DO_PROFILE as libc::c_int as libc::c_uint != 0
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"main.c\0" as *const u8 as *const libc::c_char,
                        1774 as libc::c_int,
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
                            b"`--profile' overrides `--pretty-print'\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                do_flags = ::std::mem::transmute::<
                    libc::c_uint,
                    do_flag_values,
                >(
                    do_flags as libc::c_uint
                        | DO_PRETTY_PRINT as libc::c_int as libc::c_uint,
                );
                if !optarg.is_null() {
                    set_prof_file(optarg);
                } else {
                    set_prof_file(b"awkprof.out\0" as *const u8 as *const libc::c_char);
                }
            }
            9995362966312559327 => {
                scan = optarg;
                if *argv.offset((optind - 1 as libc::c_int) as isize) != optarg {
                    while *(*__ctype_b_loc())
                        .offset(*scan as libc::c_uchar as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                    {
                        scan = scan.offset(1);
                        scan;
                    }
                }
                src = if *scan as libc::c_int == '\0' as i32 {
                    let fresh5 = optind;
                    optind = optind + 1;
                    *argv.offset(fresh5 as isize)
                } else {
                    optarg
                };
                add_srcfile(
                    (if !src.is_null()
                        && *src.offset(0 as libc::c_int as isize) as libc::c_int
                            == '-' as i32
                        && *src.offset(1 as libc::c_int as isize) as libc::c_int
                            == '\0' as i32
                    {
                        SRC_STDIN as libc::c_int
                    } else {
                        SRC_FILE as libc::c_int
                    }) as srctype,
                    src,
                    srcfiles,
                    0 as *mut bool,
                    0 as *mut libc::c_int,
                );
            }
            _ => {}
        }
        if c == 'E' as i32 {
            break;
        }
        optopt = 0 as libc::c_int;
        old_optind = optind;
    }
    do_optimize = do_optimize as libc::c_int != 0
        && do_flags as libc::c_uint & DO_PRETTY_PRINT as libc::c_int as libc::c_uint
            == 0;
    pma_mpfr_check();
}
unsafe extern "C" fn set_locale_stuff() {
    setlocale(0 as libc::c_int, locale);
    setlocale(3 as libc::c_int, locale);
    setlocale(5 as libc::c_int, locale);
    setlocale(1 as libc::c_int, locale);
    init_locale(&mut loc);
    setlocale(1 as libc::c_int, b"C\0" as *const u8 as *const libc::c_char);
    setlocale(2 as libc::c_int, locale);
    bindtextdomain(b"gawk\0" as *const u8 as *const libc::c_char, locale_dir);
    textdomain(b"gawk\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn platform_name() -> *const libc::c_char {
    return b"posix\0" as *const u8 as *const libc::c_char;
}
pub unsafe extern "C" fn set_current_namespace(mut new_namespace: *const libc::c_char) {
    if current_namespace != awk_namespace.as_ptr() {
        free(current_namespace as *mut libc::c_void);
    }
    current_namespace = new_namespace;
}
unsafe extern "C" fn check_pma_security(mut pma_file: *const libc::c_char) {}
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
